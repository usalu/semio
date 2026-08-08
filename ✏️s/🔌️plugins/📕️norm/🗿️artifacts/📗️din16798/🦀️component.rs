//! 🌬️ DIN EN 16798 app — document entities (constitutional: general).

use crate::document::AnnexChoice;
use serde::{Deserialize, Serialize};

// #region 🔖️Types

/// 📸️ Persisted snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
pub use crate::artifacts::din16798::snapshot::schema::Din16798Snapshot;
//#endregion 🔖️Types

// `)` so the
/// artifact node, not the app, owns its own kind declaration.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("din16798", "DIN EN 16798")
}
//#endregion 🔖️ArtifactKind
