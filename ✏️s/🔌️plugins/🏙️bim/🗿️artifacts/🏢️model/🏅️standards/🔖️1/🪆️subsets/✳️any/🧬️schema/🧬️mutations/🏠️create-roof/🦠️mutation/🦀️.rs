//! 🏠️ `create-roof` payload. Brings a new roof onto a storey; footprint is a closed counter-clockwise loop, pitched shapes need pitches in (0, 89 degrees); the solid is inferred.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::{ModelSnapshot, Roof}};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateRoof {
    pub id: String,
    pub roof: Roof,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateRoof
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateRoof> + From<crate::standards::v1::subsets::any::schema::mutations::delete_roof::DeleteRoof>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "roof", kind: "create-roof", record: "CreateRoof" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_roof::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_roof::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create roof \"{}\"", self.roof.name), &format!("Dach \"{}\" anlegen", self.roof.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
