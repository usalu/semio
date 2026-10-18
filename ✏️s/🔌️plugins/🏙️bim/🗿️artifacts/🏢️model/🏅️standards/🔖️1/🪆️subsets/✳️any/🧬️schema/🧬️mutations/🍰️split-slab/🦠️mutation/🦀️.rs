//! 🍰️ `split-slab` payload. Cuts a slab along the infinite line through two points into two slabs: the piece to the left of the line (seen from its first point to its second) stays, the piece to the right is created with the type, offset, slope and name of the original; holes go with the piece they lie in, split arcs keep their circle. The line must cross the outline exactly twice and no hole.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::{ModelSnapshot, Point2}};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SplitSlab {
    pub id: String,
    pub new_id: String,
    pub line_start: Point2,
    pub line_end: Point2,
}

impl<Op> MutationKind<ModelSnapshot, Op> for SplitSlab
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SplitSlab> + From<crate::standards::v1::subsets::any::schema::mutations::delete_slab::DeleteSlab> + From<crate::standards::v1::subsets::any::schema::mutations::set_slab_boundary::SetSlabBoundary>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "split", entity: "slab", kind: "split-slab", record: "SplitSlab" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::split_slab::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::split_slab::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Split slab \"{}\" along the line through ({}, {}) and ({}, {})", self.id, self.line_start.x, self.line_start.y, self.line_end.x, self.line_end.y), &format!("Decke \"{}\" entlang der Linie durch ({}, {}) und ({}, {}) teilen", self.id, self.line_start.x, self.line_start.y, self.line_end.x, self.line_end.y))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
