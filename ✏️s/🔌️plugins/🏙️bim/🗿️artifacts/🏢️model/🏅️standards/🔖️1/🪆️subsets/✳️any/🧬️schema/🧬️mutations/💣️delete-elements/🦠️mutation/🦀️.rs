//! 💣️ `delete-elements` payload. Removes any set of elements together with everything that depends on them (buildings of a site, storeys and grid lines of a building, the contents of a storey, openings of walls and curtain walls) and with their properties and classifications; refuses while a surviving element's top constraint still points at a removed storey.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteElements {
    pub ids: Vec<String>,
}

impl MutationKind<ModelSnapshot, ModelMutation> for DeleteElements {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "elements", kind: "delete-elements", record: "DeleteElements" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete {} element(s) with their dependants", self.ids.len()), &format!("{} Element(e) samt Abhängigen löschen", self.ids.len()))
    }
    fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}
