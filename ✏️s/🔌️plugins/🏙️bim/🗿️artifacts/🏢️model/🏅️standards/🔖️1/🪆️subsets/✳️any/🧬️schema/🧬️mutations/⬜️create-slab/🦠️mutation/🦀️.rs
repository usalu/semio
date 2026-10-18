//! ⬜️ `create-slab` payload. Brings a new slab onto a storey; its boundary and holes are closed counter-clockwise loops, area and solid are inferred.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::{ModelSnapshot, Slab}};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateSlab {
    pub id: String,
    pub slab: Slab,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateSlab
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateSlab> + From<crate::standards::v1::subsets::any::schema::mutations::delete_slab::DeleteSlab>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "slab", kind: "create-slab", record: "CreateSlab" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_slab::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_slab::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create slab \"{}\"", self.slab.name), &format!("Decke \"{}\" anlegen", self.slab.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
