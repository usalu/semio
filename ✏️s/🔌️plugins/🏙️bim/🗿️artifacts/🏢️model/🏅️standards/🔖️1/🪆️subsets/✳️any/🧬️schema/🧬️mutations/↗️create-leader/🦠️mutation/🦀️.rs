//! ↗️ `create-leader` payload. Brings a new leader onto a storey plan: a text joined by a line to an anchor (a free point or an element of the model), placed beside the anchor so it follows the element.

use crate::standards::v1::subsets::any::schema::{snapshot::{Leader, ModelSnapshot}, diff::ModelDiff};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateLeader {
    pub id: String,
    pub leader: Leader,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateLeader
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateLeader> + From<crate::standards::v1::subsets::any::schema::mutations::delete_leader::DeleteLeader>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "leader", kind: "create-leader", record: "CreateLeader" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_leader::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_leader::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create leader \"{}\"", self.id), &format!("Hinweislinie \"{}\" anlegen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
