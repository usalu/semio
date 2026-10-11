//! 🗂️ `set-element-classification` payload. Sets the code of an element or type in one classification system; an element without classifications gets its first, another system adds a classification and the same system replaces the code. The code need not be an entry of the system (a diagnostic reports it).

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetElementClassification {
    pub id: String,
    pub system: String,
    pub code: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetElementClassification {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "element-classification", kind: "set-element-classification", record: "SetElementClassification" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Classify \"{}\" as {} in {}", self.id, self.code, self.system), &format!("\"{}\" als {} in {} klassifizieren", self.id, self.code, self.system))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
