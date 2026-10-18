//! 🪵️ `create-wall-type` payload. Brings a new layered wall type into the library; every layer names an existing material and has a positive thickness.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::{ModelSnapshot, WallType}};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateWallType {
    pub id: String,
    pub wall_type: WallType,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateWallType
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateWallType> + From<crate::standards::v1::subsets::any::schema::mutations::delete_wall_type::DeleteWallType>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "wall-type", kind: "create-wall-type", record: "CreateWallType" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_wall_type::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_wall_type::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create wall type \"{}\"", self.wall_type.name), &format!("Wandtyp \"{}\" anlegen", self.wall_type.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
