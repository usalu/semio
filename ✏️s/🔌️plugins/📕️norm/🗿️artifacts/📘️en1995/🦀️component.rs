//! 🪵️ EN 1995 artifact root — snapshot re-export and facet modules.


pub use crate::artifacts::en1995::snapshot::schema::En1995Snapshot;

#[path = "./🧬️schema/🦀️component.rs"]
pub mod schema;

pub mod snapshot {
    #[path = "./📸️snapshot/🧬️schema/🦀️component.rs"]
    pub mod schema;
    #[path = "./📸️snapshot/🎒️pack/🦀️component.rs"]
    pub mod pack;
}

pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1995", "EN 1995")
}
