//! 🏢️ `create-building` payload. Brings a new building onto an existing site.

use crate::standards::v1::subsets::any::schema::{snapshot::{Building, ModelSnapshot}, diff::ModelDiff};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateBuilding {
    pub id: String,
    pub building: Building,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateBuilding
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateBuilding> + From<crate::standards::v1::subsets::any::schema::mutations::delete_building::DeleteBuilding>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "building", kind: "create-building", record: "CreateBuilding" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_building::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_building::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create building \"{}\"", self.building.name), &format!("Gebäude \"{}\" anlegen", self.building.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
