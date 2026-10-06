//! 🧬️ Fem2d snapshot schema — artifact-lane fields only.

use crate::{FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Snapshot
/// 📸️ Persisted fem2d document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(id = "fem.fem2d", layout = "lines")]
#[artifact_schema(id = "s.fem.fem2d")]
pub struct Fem2dSnapshot {
    #[dsl(table)]
    #[state(artifact)]
    pub nodes: Vec<FemNode>,
    #[dsl(statements, block)]
    #[state(artifact)]
    pub elements: Vec<FemElement>,
    #[dsl(table)]
    #[state(artifact)]
    pub regions: Vec<FemRegion>,
    #[dsl(table)]
    #[state(artifact)]
    pub materials: Vec<FemMaterial>,
    #[dsl(table)]
    #[state(artifact)]
    pub sections: Vec<FemSection>,
    #[dsl(table)]
    #[state(artifact)]
    pub supports: Vec<FemSupport>,
    #[dsl(table)]
    #[state(artifact)]
    pub load_cases: Vec<FemLoadCase>,
    #[dsl(table)]
    #[state(artifact)]
    pub combinations: Vec<FemCombination>,
    #[dsl(block)]
    #[state(artifact)]
    pub analysis: FemAnalysisSettings,
}
//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs
//#endregion 🔖️Snapshot

//#region 🌉️IdentityBridge

//#endregion 🌉️IdentityBridge

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::FemAnalysisSettings;
//#endregion 🔁️Re-exports
