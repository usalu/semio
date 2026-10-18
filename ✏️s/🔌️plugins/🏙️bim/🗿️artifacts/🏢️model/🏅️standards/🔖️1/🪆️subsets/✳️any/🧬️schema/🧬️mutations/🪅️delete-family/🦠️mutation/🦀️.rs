//! 🪅️ `delete-family` payload. Removes a family with its parameters and solids; refused while a column type, beam type, curtain wall or railing uses it as a profile.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteFamily {
    pub id: String,
}

impl<Op> MutationKind<ModelSnapshot, Op> for DeleteFamily
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<DeleteFamily> + From<crate::standards::v1::subsets::any::schema::mutations::create_family::CreateFamily> + From<crate::standards::v1::subsets::any::schema::mutations::create_family_solid::CreateFamilySolid> + From<crate::standards::v1::subsets::any::schema::mutations::set_family_parameter::SetFamilyParameter> + crate::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "family", kind: "delete-family", record: "DeleteFamily" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::delete_family::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::delete_family::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete family \"{}\" with its parts", self.id), &format!("Familie \"{}\" mit ihren Teilen löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
