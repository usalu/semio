//! 📄️ `create-sheet` payload. Brings a new sheet into the drawing set: number, name, paper size, orientation and the authored fields of the title block. The frame, the title block and the revision table it prints are inferred.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{ModelSnapshot, Sheet};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateSheet {
    pub id: String,
    pub sheet: Sheet,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateSheet
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateSheet> + From<semio_s_artifact_bim_model_mut_any_104::standards::v1::subsets::any::schema::mutations::delete_sheet::DeleteSheet>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "sheet", kind: "create-sheet", record: "CreateSheet" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create sheet \"{}\"", self.sheet.number), &format!("Blatt \"{}\" anlegen", self.sheet.number))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for CreateSheet
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<CreateSheet> + From<semio_s_artifact_bim_model_mut_any_104::standards::v1::subsets::any::schema::mutations::delete_sheet::DeleteSheet>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_sheet::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_sheet::inverse::inverse(self, base))
    }
}
