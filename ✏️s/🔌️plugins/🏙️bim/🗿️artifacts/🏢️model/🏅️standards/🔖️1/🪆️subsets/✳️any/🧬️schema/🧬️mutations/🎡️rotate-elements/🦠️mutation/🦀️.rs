//! 🎡️ `rotate-elements` payload. Turns placed elements counter-clockwise about a pivot, one sparse patch of exactly the placement fields per element; column rotation, stair direction, slab fall and roof ridge direction turn with them and hosted openings follow by inference.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Point2};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RotateElements {
    pub ids: Vec<String>,
    pub pivot: Point2,
    pub angle: f64,
}

impl MutationKind<ModelSnapshot, ModelMutation> for RotateElements {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rotate", entity: "elements", kind: "rotate-elements", record: "RotateElements" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rotate {} element(s) by {} rad", self.ids.len(), self.angle), &format!("{} Element(e) um {} rad drehen", self.ids.len(), self.angle))
    }
    fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}
