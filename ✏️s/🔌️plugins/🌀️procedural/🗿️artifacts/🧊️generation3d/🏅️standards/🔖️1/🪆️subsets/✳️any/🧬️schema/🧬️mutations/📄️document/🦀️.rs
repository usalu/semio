//! 📄️ Generation3d document mutation leaves: pan the camera and restamp the schema. Every leaf is generic over the dispatch enum.
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

#[path = "."]
pub mod update_camera {
    #[path = "../📷️update-camera/🦀️.rs"]
    mod component;
    #[path = "../📷️update-camera/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../📷️update-camera/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
#[path = "."]
pub mod change_schema {
    #[path = "../🔤️change-schema/🦀️.rs"]
    mod component;
    #[path = "../🔤️change-schema/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔤️change-schema/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
