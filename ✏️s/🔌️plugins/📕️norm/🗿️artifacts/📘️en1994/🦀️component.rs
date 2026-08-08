//! En1994 — document entities (constitutional: general).

use crate::document::AnnexChoice;
use serde::{Deserialize, Serialize};

//#region 🔖️Types


/// 📸️ Persisted snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
pub use crate::artifacts::en1994::snapshot::schema::En1994Snapshot;
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1994", "EN 1994")
}
//#endregion 🔖️ArtifactKind
