//! 🏛️ `create-column` payload. Brings a new column onto a storey; its height is never stored, it is inferred from the authored top constraint.

use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{Column, ModelSnapshot};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateColumn {
    pub id: String,
    pub column: Column,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateColumn
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateColumn> + From<semio_s_artifact_bim_model_mut_any_65::standards::v1::subsets::any::schema::mutations::delete_column::DeleteColumn>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "column", kind: "create-column", record: "CreateColumn" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_column::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_column::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create column \"{}\"", self.column.name), &format!("Stütze \"{}\" anlegen", self.column.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
