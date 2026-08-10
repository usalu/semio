//! 🌬️ DIN EN 16798 app — document entities (constitutional: general).


pub use crate::artifacts::din16798::schema::snapshot::Din16798Snapshot;
pub use crate::artifacts::din16798::schema::mutations::Din16798Mutation;
pub use crate::artifacts::din16798::schema::diff::Din16798Diff;

use crate::document::AnnexChoice;
use serde::{Deserialize, Serialize};

// #region 🔖️Types

/// 📸️ Persisted snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
//#endregion 🔖️Types

// `)` so the
/// artifact node, not the app, owns its own kind declaration.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("din16798", "DIN EN 16798")
}
//#endregion 🔖️ArtifactKind
