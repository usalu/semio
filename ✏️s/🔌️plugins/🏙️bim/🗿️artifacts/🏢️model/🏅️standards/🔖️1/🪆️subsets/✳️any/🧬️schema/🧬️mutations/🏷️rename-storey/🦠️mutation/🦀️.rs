//! 🏷️ `rename-storey` payload. Changes a storey's display name.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RenameStorey {
    pub id: String,
    pub name: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for RenameStorey {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "storey", kind: "rename-storey", record: "RenameStorey" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename storey to \"{}\"", self.name), &format!("Geschoss in \"{}\" umbenennen", self.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
