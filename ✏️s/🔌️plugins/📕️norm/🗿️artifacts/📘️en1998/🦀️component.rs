//! 🌋️ EN 1998 artifact root — snapshot re-export and facet modules.


pub use crate::artifacts::en1998::schema::snapshot::En1998Snapshot;
pub use crate::artifacts::en1998::schema::mutations::En1998Mutation;
pub use crate::artifacts::en1998::schema::diff::En1998Diff;




pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1998", "EN 1998")
}
