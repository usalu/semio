//! 🎥️ Camera payload and sparse GIS 3D configuration behavior.
use super::super::{GisTerrainWindowConfig, GisTerrainWindowConfigDelta, GisTerrainWindowConfigDiff, GisTerrainWindowConfigMutation};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-camera")]
pub struct SetCamera {
    pub camera_json: String,
}
//#endregion 🧬️Payload
//#region ⚙️Behavior
impl MutationKind<GisTerrainWindowConfig, GisTerrainWindowConfigMutation> for SetCamera {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "camera", kind: "set-camera", record: "SetCamera" };
    fn diff(&self, base: &GisTerrainWindowConfig) -> MutationOutcome<GisTerrainWindowConfigDiff> {
        if base.camera_json == self.camera_json {
            MutationOutcome::empty().warning("mutation.no-op", "Camera is already at the requested position.")
        } else {
            MutationOutcome::new(GisTerrainWindowConfigDelta { camera_json: Some(self.camera_json.clone()) }.into())
        }
    }
    fn inverse(&self, base: &GisTerrainWindowConfig) -> Result<Vec<GisTerrainWindowConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { camera_json: base.camera_json.clone() }.into()]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set camera", "Kamera setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["cameraJson".into()]
    }
}
//#endregion ⚙️Behavior
//#region 🧪️Contracts
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Contracts
