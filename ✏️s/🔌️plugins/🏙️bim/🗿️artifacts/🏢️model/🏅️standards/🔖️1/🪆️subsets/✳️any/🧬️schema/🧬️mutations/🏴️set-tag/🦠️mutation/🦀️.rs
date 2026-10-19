//! 🏴️ `set-tag` payload. Sets exactly the provided fields of a tag: element, category, offset from the element and style; absent fields stay untouched.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::documentation::TagPatch;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{Point2, TagCategory};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetTag {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub element: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<TagCategory>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<Point2>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

impl SetTag {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> TagPatch {
        TagPatch { element: self.element.clone(), category: self.category, offset: self.offset, style: self.style.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: TagPatch) -> Self {
        Self { id, element: patch.element, category: patch.category, offset: patch.offset, style: patch.style }
    }
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetTag
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetTag>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "tag", kind: "set-tag", record: "SetTag" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_tag::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_tag::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change tag \"{}\"", self.id), &format!("Kennzeichnung \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
