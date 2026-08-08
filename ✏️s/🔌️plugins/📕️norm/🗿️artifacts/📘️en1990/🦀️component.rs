//! ⚖️ EN 1990 basis of structural design — document entities (constitutional: general).

/// 📸️ Persisted snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
pub use crate::artifacts::en1990::snapshot::schema::En1990Snapshot;
pub use crate::artifacts::en1990::snapshot::schema::En1990QkEntry;

//#region 🔖️Types
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1990", "EN 1990")
}
//#endregion 🔖️ArtifactKind
