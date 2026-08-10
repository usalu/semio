//! 🪵️ EN 1995 artifact root — snapshot re-export and facet modules.


pub use crate::artifacts::en1995::schema::snapshot::En1995Snapshot;
pub use crate::artifacts::en1995::schema::mutations::En1995Mutation;
pub use crate::artifacts::en1995::schema::diff::En1995Diff;




pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1995", "EN 1995")
}
