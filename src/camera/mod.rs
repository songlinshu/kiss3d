//! Camera trait with some common implementations.

pub use camera::arc_ball::ArcBall;
pub use camera::camera::{Camera, PlanarCamera};
pub use camera::first_person::FirstPerson;
pub use camera::first_person_stereo::FirstPersonStereo;
pub use camera::planar_camera::PlanarCamera;
pub use camera::static_camera::StaticCamera;

mod arc_ball;
#[doc(hidden)]
pub mod camera;
mod first_person;
mod first_person_stereo;
mod planar_camera;
mod static_camera;
