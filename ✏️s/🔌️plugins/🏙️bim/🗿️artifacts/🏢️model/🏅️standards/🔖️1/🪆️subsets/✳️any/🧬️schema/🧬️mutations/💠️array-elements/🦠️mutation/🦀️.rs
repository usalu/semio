//! 💠️ `array-elements` payload. Repeats placed elements in a linear array (each copy a further spacing) or a radial array (each copy a further step about a centre), as one set of created records with ids minted from the prefix, the copy number and the position of the source; every copy is computed from the source, so no rounding accumulates.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ArrayElements {
    pub ids: Vec<String>,
    pub prefix: String,
    pub pattern: super::super::modify::ArrayPattern,
}

impl MutationKind<ModelSnapshot, ModelMutation> for ArrayElements {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "duplicate", entity: "elements", kind: "array-elements", record: "ArrayElements" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Array {} element(s) {} times", self.ids.len(), self.pattern.count()), &format!("{} Element(e) {}-mal anordnen", self.ids.len(), self.pattern.count()))
    }
    fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}
