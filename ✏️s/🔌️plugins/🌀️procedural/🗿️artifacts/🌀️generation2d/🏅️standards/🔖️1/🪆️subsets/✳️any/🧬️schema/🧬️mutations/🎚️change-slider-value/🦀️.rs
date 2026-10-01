//! 🎚️ Generation2d mutation — `ChangeSliderValue`: the ABSOLUTE value one input slider of the generator graph is set
//! to. A continuous control's intent IS the value (design §13.1), so a scrub commits exactly this leaf and editing it in
//! history replays the value on any base.

use crate::standards::v1::subsets::any::schema::diff::Generation2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{generation2d_label_number, Generation2dMutation};
use crate::Generation2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️ChangeSliderValue
/// 🎚️ Sets slider `id` to `value`; a value outside the slider's range widens the range the way the canvas knob does.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeSliderValue {
    pub id: String,
    pub value: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_slider_value(id: impl Into<String>, value: f64) -> Generation2dMutation {
    Generation2dMutation::ChangeSliderValue(ChangeSliderValue { id: id.into(), value })
}

impl MutationKind<Generation2dSnapshot, Generation2dMutation> for ChangeSliderValue {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "slider-value", kind: "change-slider-value", record: "ChangedSliderValue" };

    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        let (en, de) = generation2d_label_number(self.value);
        protocol::LocalizedLabel::native(&format!("Set slider \"{}\" to {en}", self.id), &format!("Schieberegler \"{}\" auf {de} setzen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️ChangeSliderValue
