//! 🌍️ EN 1997 artifact root — snapshot re-export and facet modules.


pub use crate::artifacts::en1997::schema::snapshot::En1997Snapshot;
pub use crate::artifacts::en1997::schema::mutations::En1997Mutation;
pub use crate::artifacts::en1997::schema::diff::En1997Diff;




pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1997", "EN 1997")
}
