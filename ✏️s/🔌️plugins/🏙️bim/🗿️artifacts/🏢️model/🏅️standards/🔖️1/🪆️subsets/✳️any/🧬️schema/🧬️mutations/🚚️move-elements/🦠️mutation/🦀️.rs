//! 🚚️ `move-elements` payload. Translates placed elements by a vector: walls, curtain walls, columns, beams, slabs, roofs, stairs, railings, spaces and grid lines each get one sparse patch of exactly their placement fields; hosted openings follow their host by inference.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Point2;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct MoveElements {
    pub ids: Vec<String>,
    pub vector: Point2,
}

impl<Op> MutationKind<ModelSnapshot, Op> for MoveElements
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<MoveElements> + From<semio_s_artifact_bim_model_mut_any_84::standards::v1::subsets::any::schema::mutations::place_elements::PlaceElements>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "elements", kind: "move-elements", record: "MoveElements" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::move_elements::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::move_elements::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move {} element(s) by ({}, {}) m", self.ids.len(), self.vector.x, self.vector.y), &format!("{} Element(e) um ({}, {}) m verschieben", self.ids.len(), self.vector.x, self.vector.y))
    }
    fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}
