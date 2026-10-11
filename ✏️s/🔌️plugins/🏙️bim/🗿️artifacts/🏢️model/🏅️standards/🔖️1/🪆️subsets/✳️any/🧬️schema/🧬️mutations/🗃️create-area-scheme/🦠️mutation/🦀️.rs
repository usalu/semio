//! 🗃️ `create-area-scheme` payload. Brings a new area scheme into the model: the authored rule that decides which spaces a gross, net or rentable area adds up, by usage and by zone.

use crate::{AreaScheme, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateAreaScheme {
    pub id: String,
    pub area_scheme: AreaScheme,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateAreaScheme {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "area-scheme", kind: "create-area-scheme", record: "CreateAreaScheme" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create area scheme \"{}\"", self.area_scheme.name), &format!("Flächenschema \"{}\" anlegen", self.area_scheme.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
