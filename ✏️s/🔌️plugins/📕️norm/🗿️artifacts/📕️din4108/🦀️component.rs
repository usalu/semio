//! 🌡️ DIN 4108 app — document entities (constitutional: general).


pub use crate::artifacts::din4108::schema::snapshot::Din4108Snapshot;
pub use crate::artifacts::din4108::schema::mutations::Din4108Mutation;
pub use crate::artifacts::din4108::schema::diff::Din4108Diff;

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
//#endregion 🔖️Types

// `)` so the
/// artifact node, not the app, owns its own kind declaration.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("din4108", "DIN 4108")
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::din4108::standards::v1::engine::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("Din4108Composer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
