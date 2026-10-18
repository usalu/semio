//! 🏬️ `create-curtain-wall` payload. Brings a new curtain wall onto a storey; its height is never stored, it is inferred from the top constraint, its grid, mullions and panels from its type.

use crate::standards::v1::subsets::any::schema::{snapshot::{CurtainWall, ModelSnapshot}, diff::ModelDiff};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateCurtainWall {
    pub id: String,
    pub curtain_wall: CurtainWall,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateCurtainWall
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateCurtainWall> + From<crate::standards::v1::subsets::any::schema::mutations::delete_curtain_wall::DeleteCurtainWall>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "curtain-wall", kind: "create-curtain-wall", record: "CreateCurtainWall" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_curtain_wall::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_curtain_wall::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create curtain wall \"{}\"", self.curtain_wall.name), &format!("Vorhangfassade \"{}\" anlegen", self.curtain_wall.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
