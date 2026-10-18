//! 🪜️ `create-storey` payload. Brings a new storey into a building; its level index is unique within the building.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::{ModelSnapshot, Storey}};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateStorey {
    pub id: String,
    pub storey: Storey,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateStorey
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateStorey> + From<crate::standards::v1::subsets::any::schema::mutations::delete_storey::DeleteStorey>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "storey", kind: "create-storey", record: "CreateStorey" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_storey::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_storey::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create storey \"{}\"", self.storey.name), &format!("Geschoss \"{}\" anlegen", self.storey.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
