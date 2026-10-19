//! 🧬️ Fem2d snapshot schema — artifact-lane fields only.

use crate::standards::v1::subsets::any::schema::structure::FemCombination;
use crate::standards::v1::subsets::any::schema::structure::FemElement;
use crate::standards::v1::subsets::any::schema::structure::FemLoadCase;
use semio_s_fem_node::planar::FemNodeValue;
use crate::standards::v1::subsets::any::schema::structure::FemRegion;
use crate::standards::v1::subsets::any::schema::structure::FemSection;
use crate::standards::v1::subsets::any::schema::structure::FemSupport;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

#[path="❄️reference/🦀️.rs"]mod reference;
pub use reference::Fem2dSnapshotValue;
//#region 🔖️Snapshot
/// 📸️ Persisted fem2d document snapshot (persistent fields of the artifact).
#[derive(Debug, Default, PartialEq, ToValue, ArtifactSchema, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.fem.fem2d")]
pub struct Fem2dSnapshot {
    #[state(artifact)]
    pub nodes: semio_framework_value::list::persistent::PersistentList<semio_s_fem_node::planar::FemNode>,
    #[state(artifact)]
    pub elements: Vec<FemElement>,
    #[state(artifact)]
    pub regions: Vec<FemRegion>,
    #[state(artifact)]
    pub materials: semio_framework_value::list::persistent::PersistentList<semio_s_fem_material::planar::FemMaterial>,
    #[state(artifact)]
    pub sections: Vec<FemSection>,
    #[state(artifact)]
    pub supports: Vec<FemSupport>,
    #[state(artifact)]
    pub load_cases: Vec<FemLoadCase>,
    #[state(artifact)]
    pub combinations: Vec<FemCombination>,
    #[state(artifact)]
    pub analysis: FemAnalysisSettings,
}

#[path = "🪪️identity/🦀️.rs"]
mod interaction_identity;

impl Fem2dSnapshot {
}
//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs
//#endregion 🔖️Snapshot

//#region 🌉️IdentityBridge

//#endregion 🌉️IdentityBridge

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::standards::v1::subsets::any::schema::structure::FemAnalysisSettings;
//#endregion 🔁️Re-exports
