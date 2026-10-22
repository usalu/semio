//! 📑️ `set-sheet` payload. Sparsely changes a sheet: number, name, paper, orientation and the authored fields of the title block. The title block, the frame and the revision table are inferred and follow.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{Orientation, Paper};
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::documentation::SheetPatch;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSheet {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub paper: Option<Paper>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Orientation>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub drawn_by: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub checked_by: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale_label: Option<String>,
}

impl SetSheet {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> SheetPatch {
        SheetPatch { number: self.number.clone(), name: self.name.clone(), paper: self.paper.clone(), orientation: self.orientation, project: self.project.clone(), drawn_by: self.drawn_by.clone(), checked_by: self.checked_by.clone(), date: self.date.clone(), revision: self.revision.clone(), scale_label: self.scale_label.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: SheetPatch) -> Self {
        Self { id, number: patch.number, name: patch.name, paper: patch.paper, orientation: patch.orientation, project: patch.project, drawn_by: patch.drawn_by, checked_by: patch.checked_by, date: patch.date, revision: patch.revision, scale_label: patch.scale_label }
    }
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetSheet
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetSheet>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "sheet", kind: "set-sheet", record: "SetSheet" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change sheet \"{}\"", self.id), &format!("Blatt \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetSheet
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetSheet>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_sheet::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_sheet::inverse::inverse(self, base))
    }
}
