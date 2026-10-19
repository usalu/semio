//! 🗼️ `create-column-type` payload. Brings a new column type into the library; it needs a free id, an existing material and a valid profile.

use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{ColumnType, ModelSnapshot};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateColumnType {
    pub id: String,
    pub column_type: ColumnType,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateColumnType
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateColumnType> + From<crate::standards::v1::subsets::any::schema::mutations::delete_column_type::DeleteColumnType>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "column-type", kind: "create-column-type", record: "CreateColumnType" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_column_type::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_column_type::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create column type \"{}\"", self.column_type.name), &format!("Stützentyp \"{}\" anlegen", self.column_type.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
