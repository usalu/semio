//! ⤴️ `set-leader` payload. Sets exactly the provided fields of a leader: anchor, offset of the text from the anchor, text and style; absent fields stay untouched.

use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{AnnotationAnchor, Point2};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::documentation::LeaderPatch;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLeader {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<AnnotationAnchor>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<Point2>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

impl SetLeader {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> LeaderPatch {
        LeaderPatch { anchor: self.anchor.clone(), offset: self.offset, text: self.text.clone(), style: self.style.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: LeaderPatch) -> Self {
        Self { id, anchor: patch.anchor, offset: patch.offset, text: patch.text, style: patch.style }
    }
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetLeader
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetLeader>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "leader", kind: "set-leader", record: "SetLeader" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change leader \"{}\"", self.id), &format!("Hinweislinie \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetLeader
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetLeader>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_leader::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_leader::inverse::inverse(self, base))
    }
}
