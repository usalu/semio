//! 🖼️ `create-window-type` payload. Brings a new window type into the library; it needs a free id, an existing material and valid dimensions.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, WindowType};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateWindowType {
    pub id: String,
    pub window_type: WindowType,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateWindowType {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "window-type", kind: "create-window-type", record: "CreateWindowType" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create window type \"{}\"", self.window_type.name), &format!("Fenstertyp \"{}\" anlegen", self.window_type.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
