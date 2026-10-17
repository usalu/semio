//! 🎥️ Camera payload and sparse camera configuration change.

use crate::editor::gis2d::modes::edit::windows::map::config::component::{MapWindowConfig, MapWindowConfigDelta, MapWindowConfigDiff};
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
impl<Op> MutationKind<MapWindowConfig, Op> for SetCamera
where
    Op: protocol::Mutation<MapWindowConfig, Diff = MapWindowConfigDiff> + From<SetCamera>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "camera", kind: "set-camera", record: "SetCamera" };
    fn diff(&self, base: &MapWindowConfig) -> MutationOutcome<MapWindowConfigDiff> {
        if base.camera_json == self.camera_json {
            return MutationOutcome::empty().warning("mutation.no-op", "Camera is already at the requested position.");
        }
        MutationOutcome::new(MapWindowConfigDelta { camera_json: Some(self.camera_json.clone()), ..Default::default() }.into())
    }
    fn inverse(&self, base: &MapWindowConfig) -> Result<Vec<Op>, semio_framework_value::ValueError> {
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
