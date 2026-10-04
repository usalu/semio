//! 🎚️ `change-slider-value` payload — the ABSOLUTE value one input slider of the generator graph is set to. A
//! continuous control's intent IS the value (design §13.1), so a scrub commits exactly this leaf and editing it in
//! history replays the value on any base.

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::{generation3d_label_number, Generation3dMutation};
use crate::Generation3dSnapshot;
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
pub fn change_slider_value(id: impl Into<String>, value: f64) -> Generation3dMutation {
    Generation3dMutation::ChangeSliderValue(ChangeSliderValue { id: id.into(), value })
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for ChangeSliderValue {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "slider-value", kind: "change-slider-value", record: "ChangedSliderValue" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (en, de) = generation3d_label_number(self.value);
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set slider \"{}\" to {en}", self.id), &format!("Schieberegler \"{}\" auf {de} setzen", self.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️ChangeSliderValue
