//! 🎡️ `rotate-elements` payload. Turns placed elements counter-clockwise about a pivot, one sparse patch of exactly the placement fields per element; column rotation, stair direction, slab fall and roof ridge direction turn with them and hosted openings follow by inference.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Point2;
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
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<RotateElements> + From<semio_s_artifact_bim_model_mut_any_84::standards::v1::subsets::any::schema::mutations::place_elements::PlaceElements>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rotate", entity: "elements", kind: "rotate-elements", record: "RotateElements" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rotate {} element(s) by {} rad", self.ids.len(), self.angle), &format!("{} Element(e) um {} rad drehen", self.ids.len(), self.angle))
    }
    fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for RotateElements
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<RotateElements> + From<semio_s_artifact_bim_model_mut_any_84::standards::v1::subsets::any::schema::mutations::place_elements::PlaceElements>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::rotate_elements::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::rotate_elements::inverse::inverse(self, base))
    }
}
