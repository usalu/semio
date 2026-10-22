//! 📇️ `set-project-info` payload. Patches the project metadata (name, description, author, organization, phase names): exactly the provided fields change, an empty patch is a no-op.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::site_and_spaces::ProjectPatch;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetProjectInfo {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub phase_names: Option<Vec<String>>,
}

impl SetProjectInfo {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> ProjectPatch {
        ProjectPatch { name: self.name.clone(), description: self.description.clone(), author: self.author.clone(), organization: self.organization.clone(), phase_names: self.phase_names.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(patch: ProjectPatch) -> Self {
        Self { name: patch.name, description: patch.description, author: patch.author, organization: patch.organization, phase_names: patch.phase_names }
    }
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetProjectInfo
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetProjectInfo>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "project", kind: "set-project-info", record: "SetProjectInfo" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&String::from("Update the project info"), &String::from("Projektinformationen ändern"))
    }
    fn target(&self) -> Vec<String> {
        vec!["project".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetProjectInfo
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetProjectInfo>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_project_info::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_project_info::inverse::inverse(self, base))
    }
}
