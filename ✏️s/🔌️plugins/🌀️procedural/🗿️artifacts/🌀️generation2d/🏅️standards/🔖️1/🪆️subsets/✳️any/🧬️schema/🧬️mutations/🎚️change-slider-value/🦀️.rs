//! 🎚️ Generation2d mutation — `ChangeSliderValue`: the ABSOLUTE value one input slider of the generator graph is set
//! to. A continuous control's intent IS the value (design §13.1), so a scrub commits exactly this leaf and editing it in
//! history replays the value on any base.

use semio_s_artifact_procedural_2d_model::diff::Generation2dDiff;
use crate::generation2d_label_number;

use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
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

impl<Op> MutationKind<Generation2dSnapshot, Op> for ChangeSliderValue
where
    Op: protocol::Mutation<Generation2dSnapshot, Diff = Generation2dDiff> + From<crate::replace_widget::ReplaceWidget>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "slider-value", kind: "change-slider-value", record: "ChangedSliderValue" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (en, de) = generation2d_label_number(self.value);
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set slider \"{}\" to {en}", self.id), &format!("Schieberegler \"{}\" auf {de} setzen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Generation2dSnapshot, Op> for ChangeSliderValue
where
    Op: protocol::ColdMutation<Generation2dSnapshot, ReferenceDiff = Generation2dDiff> + From<crate::replace_widget::ReplaceWidget>,
{
    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        crate::change_slider_value::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_slider_value::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️ChangeSliderValue
