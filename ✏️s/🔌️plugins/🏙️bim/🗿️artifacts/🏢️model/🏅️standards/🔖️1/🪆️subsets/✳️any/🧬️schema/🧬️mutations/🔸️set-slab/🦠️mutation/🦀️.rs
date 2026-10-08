//! 🔸️ `set-slab` payload. Sets exactly the provided fields of a slab: type, offset, slope (set or cleared) and name; absent fields stay untouched.

use crate::{Assigned, ModelDiff, ModelMutation, ModelSnapshot, Slope};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
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

impl MutationKind<ModelSnapshot, ModelMutation> for SetSlab {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "slab", kind: "set-slab", record: "SetSlab" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change slab \"{}\"", self.id), &format!("Decke \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
