//! 🌍️ EN 1997 artifact root — snapshot re-export and facet modules.


pub use crate::artifacts::en1997::snapshot::schema::En1997Snapshot;


pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1997", "EN 1997")
}
