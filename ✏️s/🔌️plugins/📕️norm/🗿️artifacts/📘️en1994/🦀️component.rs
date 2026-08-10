//! En1994 — document entities (constitutional: general).


pub use crate::artifacts::en1994::schema::snapshot::En1994Snapshot;
pub use crate::artifacts::en1994::schema::mutations::En1994Mutation;
pub use crate::artifacts::en1994::schema::diff::En1994Diff;

use crate::document::AnnexChoice;
use serde::{Deserialize, Serialize};

//#region 🔖️Types


/// 📸️ Persisted snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1994", "EN 1994")
}
//#endregion 🔖️ArtifactKind
