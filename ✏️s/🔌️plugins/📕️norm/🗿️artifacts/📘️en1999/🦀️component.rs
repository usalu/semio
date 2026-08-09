//! ✨️ EN 1999 artifact root — snapshot re-export and facet modules.


pub use crate::artifacts::en1999::snapshot::schema::En1999Snapshot;


pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1999", "EN 1999")
}
