//! 🌡️ DIN 4108 app — document entities (constitutional: general).

use crate::document::ClimateZoneDe;
use serde::{Deserialize, Serialize};

// #region 🔖️Types
// No `#[dsl(keyword = ...)]`: reached only through the plain, un-tagged `Vec<LayerDocument>`
// list on `Document::layers` — same reasoning as `draw`'s `GradientStop`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct LayerDocument {
    #[dsl(positional, unit = "m")]
    pub thickness_m: f64,
    #[dsl(positional)]
    pub lambda_w_mk: f64,
}


/// 📸️ Persisted snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
pub use crate::artifacts::din4108::snapshot::schema::Din4108Snapshot;
//#endregion 🔖️Types

// `)` so the
/// artifact node, not the app, owns its own kind declaration.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("din4108", "DIN 4108")
}
//#endregion 🔖️ArtifactKind
