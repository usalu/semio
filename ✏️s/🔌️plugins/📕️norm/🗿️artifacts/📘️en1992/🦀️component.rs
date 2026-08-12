//! En1992 — document entities (constitutional: general).


pub use crate::artifacts::en1992::schema::snapshot::En1992Snapshot;
pub use crate::artifacts::en1992::schema::mutations::En1992Mutation;
pub use crate::artifacts::en1992::schema::diff::En1992Diff;

use crate::document::AnnexChoice;
use serde::{Deserialize, Serialize};

//#region 🔖️Types
pub mod part_1_2 {
    use super::*;

    /// 🏗️ Fire resistance rating.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
    pub enum FireRating {
        R30,
        R60,
        R90,
        R120,
    }
}
pub mod part_3 {
    use super::*;

    /// 💧️ Tightness class per EN 1992-3 Table 7.105: required degree of protection against leakage.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
    pub enum TightnessClass {
        Tc0,
        Tc1,
        Tc2,
    }
}

/// 📸️ Persisted snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1992", "EN 1992")
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::en1992::standards::v1::engine::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("En1992Composer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
