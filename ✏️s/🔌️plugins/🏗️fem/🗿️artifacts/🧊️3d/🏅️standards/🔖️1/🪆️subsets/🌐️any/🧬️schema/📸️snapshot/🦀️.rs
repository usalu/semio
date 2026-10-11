//! 🧬️ Fem3d snapshot schema — artifact-lane fields only.

use crate::{FemAnalysisSettings, FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemSection, FemSolid, FemSupport};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};


//#region 🔖️Snapshot
/// 📸️ Persisted fem3d document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(id = "fem.fem3d", layout = "lines")]
#[artifact_schema(id = "s.fem.fem3d")]
pub struct Fem3dSnapshot {
    #[dsl(table)]
    #[state(artifact)]
    pub nodes: Vec<FemNode>,
    #[dsl(statements, block)]
    #[state(artifact)]
    pub elements: Vec<FemElement>,
    #[dsl(table)]
    #[state(artifact)]
    pub materials: Vec<FemMaterial>,
    #[dsl(table)]
    #[state(artifact)]
    pub sections: Vec<FemSection>,
    #[dsl(table)]
    #[state(artifact)]
    pub solids: Vec<FemSolid>,
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
