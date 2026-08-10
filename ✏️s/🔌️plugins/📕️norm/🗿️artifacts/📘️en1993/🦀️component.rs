//! En1993 — document entities (constitutional: general).


pub use crate::artifacts::en1993::schema::snapshot::En1993Snapshot;
pub use crate::artifacts::en1993::schema::mutations::En1993Mutation;
pub use crate::artifacts::en1993::schema::diff::En1993Diff;

use crate::document::AnnexChoice;
use serde::{Deserialize, Serialize};

//#region 🔖️Types


/// 📸️ Persisted snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1993", "EN 1993")
}
//#endregion 🔖️ArtifactKind
