//! 🧬️ Fem3d snapshot schema — artifact-lane fields only.

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemAnalysisSettings;
use crate::standards::v1::subsets::any::schema::structure::FemCombination;
use crate::standards::v1::subsets::any::schema::structure::FemElement;
use crate::standards::v1::subsets::any::schema::structure::FemLoadCase;
use semio_s_fem_node::volumetric::FemNodeValue;
use crate::standards::v1::subsets::any::schema::structure::FemSection;
use crate::standards::v1::subsets::any::schema::structure::FemSolid;
use crate::standards::v1::subsets::any::schema::structure::FemSupport;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
#[path = "🪪️identity/🦀️.rs"]
mod identity_admission;


#[path="❄️reference/🦀️.rs"]mod reference;
pub use reference::Fem3dSnapshotValue;
#[path="native.rs"]mod native;
pub use native::Fem3dReceiving;
#[path="record.rs"]mod record;
#[path="receive-record.rs"]mod receive_record;
#[path="projection.rs"]mod projection;
#[path="write-pack.rs"]mod write_pack;
#[path="identity-index.rs"]mod identity_index;
pub use identity_index::{FemIdentityIndex,FemIdentityKind,FemIdentitySource};
//#region 🔖️Snapshot
/// 📸️ Persisted fem3d document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, Default, PartialEq, ToValue, ArtifactSchema, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.fem.fem3d")]
pub struct Fem3dSnapshot {
    #[state(artifact)]
    pub nodes: semio_framework_value_persistent::list::persistent::PersistentList<semio_s_fem_node::volumetric::FemNode>,
    #[state(artifact)]
    pub elements: Vec<FemElement>,
    #[state(artifact)]
    pub materials: semio_framework_value_persistent::list::persistent::PersistentList<semio_s_fem_material::volumetric::FemMaterial>,
    #[state(artifact)]
    pub sections: Vec<FemSection>,
    #[state(artifact)]
    pub solids: Vec<FemSolid>,
    #[state(artifact)]
    pub supports: Vec<FemSupport>,
    #[state(artifact)]
    pub load_cases: Vec<FemLoadCase>,
    #[state(artifact)]
    pub combinations: Vec<FemCombination>,
    #[state(artifact)]
    pub analysis: FemAnalysisSettings,
}

impl Fem3dSnapshot {

    /// 🛂️ Checks borrowed native record identities before any typed snapshot owner is constructed.
    pub fn validate_interaction_record(record: &semio_framework_dsl_record::RecordValue, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<(), semio_framework_value::ValueError> {
        identity_admission::validate(record, control)
    }

    /// 🪪️ Admits bounded entity addresses and uniqueness inside each semantic owner namespace.
    pub fn validate_interaction_identities(&self) -> Result<(), semio_framework_value::ValueError> {
        let invalid = || semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "FEM3d identity is empty, repeated in its namespace, or exceeds shared scoped identity bounds");
        let rows = self.nodes.iter().map(|row| ("node", row.id.as_str()))
            .chain(self.elements.iter().map(|row| ("element", crate::standards::v1::subsets::any::schema::structure::element_id(row))))
            .chain(self.solids.iter().map(|row| ("solid", row.id.as_str())))
            .chain(self.materials.iter().map(|row| ("material", row.id.as_str())))
            .chain(self.sections.iter().map(|row| ("section", row.id.as_str())))
            .chain(self.supports.iter().map(|row| ("support", row.id.as_str())))
            .chain(self.load_cases.iter().map(|row| ("loadCase", row.id.as_str())))
            .chain(self.combinations.iter().map(|row| ("combination", row.id.as_str())));
        let mut seen = std::collections::HashSet::new();
        for (kind, id) in rows {
            protocol::wire::validate_scoped_identity_segments(&[kind, id]).map_err(|_| invalid())?;
            if !seen.insert((kind, id)) { return Err(invalid()); }
        }
        let mut loads = std::collections::HashSet::new();
        for case in &self.load_cases {
            for load in &case.loads {
                let id = crate::standards::v1::subsets::any::schema::structure::load_id(load);
                protocol::wire::validate_scoped_identity_segments(&["load", &case.id, id]).map_err(|_| invalid())?;
                if !loads.insert((case.id.as_str(), id)) { return Err(invalid()); }
            }
        }
        Ok(())
    }
}
//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs
//#endregion 🔖️Snapshot

//#region 🌉️IdentityBridge

//#endregion 🌉️IdentityBridge
