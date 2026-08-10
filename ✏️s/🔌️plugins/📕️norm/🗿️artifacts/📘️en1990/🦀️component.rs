//! ⚖️ EN 1990 basis of structural design — document entities (constitutional: general).


pub use crate::artifacts::en1990::schema::snapshot::En1990QkEntry;
pub use crate::artifacts::en1990::schema::snapshot::En1990Snapshot;
pub use crate::artifacts::en1990::schema::mutations::En1990Mutation;
pub use crate::artifacts::en1990::schema::diff::En1990Diff;

/// 📸️ Persisted snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.

//#region 🔖️Types
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1990", "EN 1990")
}
//#endregion 🔖️ArtifactKind
