//! 💨️ `set-mep-element` payload. Sets exactly the provided fields of an MEP element: storey, system, cross-section, path and name; absent fields stay untouched. A provided path replaces the path as one field.

use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::library::MepElementPatch;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{MepShape, MepSystem, Point3};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetMepElement {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub storey: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<MepSystem>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<MepShape>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<Point3>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl SetMepElement {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> MepElementPatch {
        MepElementPatch { storey: self.storey.clone(), system: self.system, shape: self.shape.clone(), path: self.path.clone(), name: self.name.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: MepElementPatch) -> Self {
        Self { id, storey: patch.storey, system: patch.system, shape: patch.shape, path: patch.path, name: patch.name }
    }
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetMepElement
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetMepElement>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "mep-element", kind: "set-mep-element", record: "SetMepElement" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change MEP element \"{}\"", self.id), &format!("TGA-Element \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetMepElement
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetMepElement>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_mep_element::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_mep_element::inverse::inverse(self, base))
    }
}
