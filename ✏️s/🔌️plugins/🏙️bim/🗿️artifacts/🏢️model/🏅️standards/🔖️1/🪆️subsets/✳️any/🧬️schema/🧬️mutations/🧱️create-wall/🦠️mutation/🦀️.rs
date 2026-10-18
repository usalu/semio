//! 🧱️ `create-wall` payload. Brings a new wall onto a storey; its height is never stored, it is inferred from the top constraint.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::{ModelSnapshot, Wall}};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateWall {
    pub id: String,
    pub wall: Wall,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateWall
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateWall> + From<crate::standards::v1::subsets::any::schema::mutations::delete_wall::DeleteWall>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "wall", kind: "create-wall", record: "CreateWall" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_wall::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_wall::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create wall \"{}\"", self.wall.name), &format!("Wand \"{}\" anlegen", self.wall.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
