//! 📖️ `create-sheet-revision` payload. Adds a row to the revision table of a sheet: the mark, its date, what changed and who changed it. The table is printed in the corner of the sheet by the inferred sheet layout.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{ModelSnapshot, SheetRevision};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateSheetRevision {
    pub id: String,
    pub sheet_revision: SheetRevision,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateSheetRevision
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateSheetRevision> + From<semio_s_artifact_bim_model_mut_any_106::standards::v1::subsets::any::schema::mutations::delete_sheet_revision::DeleteSheetRevision>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "sheet-revision", kind: "create-sheet-revision", record: "CreateSheetRevision" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add revision \"{}\" to sheet \"{}\"", self.sheet_revision.number, self.sheet_revision.sheet), &format!("Änderung \"{}\" zu Blatt \"{}\" hinzufügen", self.sheet_revision.number, self.sheet_revision.sheet))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for CreateSheetRevision
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<CreateSheetRevision> + From<semio_s_artifact_bim_model_mut_any_106::standards::v1::subsets::any::schema::mutations::delete_sheet_revision::DeleteSheetRevision>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_sheet_revision::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_sheet_revision::inverse::inverse(self, base))
    }
}
