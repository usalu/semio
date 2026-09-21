//! ⚙️ Bitmap mutation — `ChangeModel`: the overlapping model's own parameters (pattern size, D4
//! expansion count, periodic input, optional ground colour). Every field is read by the solve and
//! by nothing else, so the whole record moves as one lane.

use crate::diff::BitmapDiff;
use crate::mutations::BitmapMutation;
use crate::schema::snapshot::BitmapSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangeModel
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeModel {
    pub pattern_size: u32,
    pub symmetry: u32,
    pub periodic_input: bool,
    pub ground: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_model(pattern_size: u32, symmetry: u32, periodic_input: bool, ground: Option<u32>) -> BitmapMutation {
    BitmapMutation::ChangeModel(ChangeModel { pattern_size, symmetry, periodic_input, ground })
}

impl MutationKind<BitmapSnapshot, BitmapMutation> for ChangeModel {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "model", kind: "change-model", record: "ChangedModel" };

    fn diff(&self, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &BitmapSnapshot) -> Vec<BitmapMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Change model to N={} symmetry={}", self.pattern_size, self.symmetry), &format!("Modell auf N={} Symmetrie={} ändern", self.pattern_size, self.symmetry))
    }
}
//#endregion 🔖️ChangeModel
