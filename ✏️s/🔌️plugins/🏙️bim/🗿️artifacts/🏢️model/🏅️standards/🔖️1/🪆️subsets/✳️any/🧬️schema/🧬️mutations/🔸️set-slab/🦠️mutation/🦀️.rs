//! 🔸️ `set-slab` payload. Sets exactly the provided fields of a slab: type, offset, slope (set or cleared) and name; absent fields stay untouched.

use crate::standards::v1::subsets::any::schema::{diff::{Assigned, ModelDiff}, snapshot::{ModelSnapshot, Slope}};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSlab {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub slab_type: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub slope: Option<Assigned<Option<Slope>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetSlab
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetSlab>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "slab", kind: "set-slab", record: "SetSlab" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_slab::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_slab::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change slab \"{}\"", self.id), &format!("Decke \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
