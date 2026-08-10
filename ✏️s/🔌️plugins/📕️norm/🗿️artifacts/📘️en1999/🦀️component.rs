//! ✨️ EN 1999 artifact root — snapshot re-export and facet modules.


pub use crate::artifacts::en1999::schema::snapshot::En1999Snapshot;
pub use crate::artifacts::en1999::schema::mutations::En1999Mutation;
pub use crate::artifacts::en1999::schema::diff::En1999Diff;




pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1999", "EN 1999")
}
