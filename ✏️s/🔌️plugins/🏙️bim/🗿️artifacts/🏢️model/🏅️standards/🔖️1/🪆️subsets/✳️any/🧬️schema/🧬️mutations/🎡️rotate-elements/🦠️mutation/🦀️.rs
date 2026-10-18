//! 🎡️ `rotate-elements` payload. Turns placed elements counter-clockwise about a pivot, one sparse patch of exactly the placement fields per element; column rotation, stair direction, slab fall and roof ridge direction turn with them and hosted openings follow by inference.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::{ModelSnapshot, Point2}};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RotateElements {
    pub ids: Vec<String>,
    pub pivot: Point2,
    pub angle: f64,
}

impl<Op> MutationKind<ModelSnapshot, Op> for RotateElements
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<RotateElements> + From<crate::standards::v1::subsets::any::schema::mutations::place_elements::PlaceElements>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rotate", entity: "elements", kind: "rotate-elements", record: "RotateElements" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::rotate_elements::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::rotate_elements::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rotate {} element(s) by {} rad", self.ids.len(), self.angle), &format!("{} Element(e) um {} rad drehen", self.ids.len(), self.angle))
    }
    fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}
