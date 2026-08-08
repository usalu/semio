//! 🌋️ EN 1998 artifact root — snapshot re-export and facet modules.


pub use crate::artifacts::en1998::snapshot::schema::En1998Snapshot;

#[path = "./🧬️schema/🦀️component.rs"]
pub mod schema;

pub mod snapshot {
    #[path = "./📸️snapshot/🧬️schema/🦀️component.rs"]
    pub mod schema;
    #[path = "./📸️snapshot/🎒️pack/🦀️component.rs"]
    pub mod pack;
}

pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1998", "EN 1998")
}
