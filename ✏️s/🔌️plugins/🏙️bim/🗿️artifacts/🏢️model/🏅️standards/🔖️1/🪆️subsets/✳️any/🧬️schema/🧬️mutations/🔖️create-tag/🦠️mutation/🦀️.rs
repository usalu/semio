//! 🔖️ `create-tag` payload. Brings a new tag onto a storey plan: text read from an element (name, type, number or size) and placed beside it, so the tag follows the element.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::{ModelSnapshot, Tag}};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateTag {
    pub id: String,
    pub tag: Tag,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateTag
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateTag> + From<crate::standards::v1::subsets::any::schema::mutations::delete_tag::DeleteTag>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "tag", kind: "create-tag", record: "CreateTag" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_tag::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_tag::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create tag \"{}\"", self.id), &format!("Kennzeichnung \"{}\" anlegen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
