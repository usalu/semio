//! 🎚️ `change-slider-value` payload — the ABSOLUTE value one input slider of the generator graph is set to. A
//! continuous control's intent IS the value (design §13.1), so a scrub commits exactly this leaf and editing it in
//! history replays the value on any base.

use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use crate::generation3d_label_number;

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

//#region 🔖️ChangeSliderValue
/// 🎚️ Sets slider `id` to `value`; a value outside the slider's range widens the range the way the canvas knob does.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeSliderValue {
    pub id: String,
    pub value: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_slider_value<Op: From<ChangeSliderValue>>(id: impl Into<String>, value: f64) -> Op {
    Op::from(ChangeSliderValue { id: id.into(), value })
}

impl<Op> protocol::MutationKind<Generation3dSnapshot, Op> for ChangeSliderValue
where
    Op: protocol::Mutation<Generation3dSnapshot, Diff = Generation3dDiff> + From<crate::update_widget::UpdateWidget>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "slider-value", kind: "change-slider-value", record: "ChangedSliderValue" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (en, de) = generation3d_label_number(self.value);
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set slider \"{}\" to {en}", self.id), &format!("Schieberegler \"{}\" auf {de} setzen", self.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Generation3dSnapshot, Op> for ChangeSliderValue
where
    Op: protocol::ColdMutation<Generation3dSnapshot, ReferenceDiff = Generation3dDiff> + From<crate::update_widget::UpdateWidget>,
{
    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::change_slider_value::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_slider_value::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️ChangeSliderValue
