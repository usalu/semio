//! 📋️ `align-elements` payload. Moves placed elements along one axis of the plan so that the lower edge, the middle or the upper edge of the authored extent of each element lies on a target coordinate; arcs count with their extremes, hosted openings follow their host.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct AlignElements {
    pub ids: Vec<String>,
    pub axis: crate::standards::v1::subsets::any::schema::mutations::modify::AlignAxis,
    pub edge: crate::standards::v1::subsets::any::schema::mutations::modify::AlignEdge,
    pub target: f64,
}

impl<Op> MutationKind<ModelSnapshot, Op> for AlignElements
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<AlignElements> + From<crate::standards::v1::subsets::any::schema::mutations::place_elements::PlaceElements>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "elements", kind: "align-elements", record: "AlignElements" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::align_elements::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::align_elements::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Align {} element(s) to {:?} = {} m", self.ids.len(), self.axis, self.target), &format!("{} Element(e) auf {:?} = {} m ausrichten", self.ids.len(), self.axis, self.target))
    }
    fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}
