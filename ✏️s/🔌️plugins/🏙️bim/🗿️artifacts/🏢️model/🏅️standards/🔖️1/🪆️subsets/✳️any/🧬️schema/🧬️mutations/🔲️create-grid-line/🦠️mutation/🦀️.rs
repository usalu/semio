//! 🔲️ `create-grid-line` payload. Brings a new labelled grid line into a building; its label is unique within the building and it has length.

use crate::standards::v1::subsets::any::schema::{snapshot::{GridLine, ModelSnapshot}, diff::ModelDiff};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateGridLine {
    pub id: String,
    pub grid_line: GridLine,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateGridLine
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateGridLine> + From<crate::standards::v1::subsets::any::schema::mutations::delete_grid_line::DeleteGridLine>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "grid-line", kind: "create-grid-line", record: "CreateGridLine" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_grid_line::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_grid_line::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create grid line \"{}\"", self.grid_line.label), &format!("Rasterlinie \"{}\" anlegen", self.grid_line.label))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
