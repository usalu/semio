//! 🧭️ `set-grid-line` payload. Patches a grid line (label, start, end): exactly the provided fields change; the building it belongs to never changes here.

use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::site_and_spaces::GridLinePatch;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Point2;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetGridLine {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<Point2>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<Point2>,
}

impl SetGridLine {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> GridLinePatch {
        GridLinePatch { label: self.label.clone(), start: self.start.clone(), end: self.end.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: GridLinePatch) -> Self {
        Self { id, label: patch.label, start: patch.start, end: patch.end }
    }
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetGridLine
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetGridLine>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "grid-line", kind: "set-grid-line", record: "SetGridLine" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Update grid line \"{}\"", self.id), &format!("Rasterlinie \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetGridLine
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetGridLine>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_grid_line::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_grid_line::inverse::inverse(self, base))
    }
}
