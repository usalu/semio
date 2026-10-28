//! 👁️ Exact visibility override assignment or removal for one map layer.

#[cfg(test)]
use crate::editor::gis2d::modes::edit::windows::map::config::component::required_nullable;
use crate::editor::gis2d::modes::edit::windows::map::config::component::{MapWindowConfig, MapWindowConfigDelta, MapWindowConfigDiff};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-layer-visibility")]
pub struct SetLayerVisibility {
    pub layer_id: String,
    #[value(required)]
    #[cfg_attr(test, serde(deserialize_with = "required_nullable"))]
    pub visible: Option<bool>,
}
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl<Op> MutationKind<MapWindowConfig, Op> for SetLayerVisibility
where
    Op: protocol::Mutation<MapWindowConfig, Diff = MapWindowConfigDiff> + From<SetLayerVisibility>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "layer-visibility", kind: "set-layer-visibility", record: "SetLayerVisibility" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set layer visibility {}", self.layer_id), &format!("Ebenensichtbarkeit {} setzen", self.layer_id))
    }
    fn target(&self) -> Vec<String> {
        vec!["layerVisibility".into(), self.layer_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<MapWindowConfig, Op> for SetLayerVisibility
where
    Op: protocol::ColdMutation<MapWindowConfig, ReferenceDiff = MapWindowConfigDiff> + From<SetLayerVisibility>,
{
    fn diff(&self, base: &MapWindowConfig) -> MutationOutcome<MapWindowConfigDiff> {
        if base.layer_visibility.get(&self.layer_id).copied() == self.visible {
            return MutationOutcome::empty().warning("mutation.no-op", "Layer visibility override is already at the requested value.");
        }
        MutationOutcome::new(MapWindowConfigDelta { layer_visibility: [(self.layer_id.clone(), self.visible)].into(), ..Default::default() }.into())
    }
    fn inverse(&self, base: &MapWindowConfig) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { layer_id: self.layer_id.clone(), visible: base.layer_visibility.get(&self.layer_id).copied() }.into()]
    
    })())
}
}
//#endregion ⚙️Behavior

//#region 🧪️Contracts
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Contracts
