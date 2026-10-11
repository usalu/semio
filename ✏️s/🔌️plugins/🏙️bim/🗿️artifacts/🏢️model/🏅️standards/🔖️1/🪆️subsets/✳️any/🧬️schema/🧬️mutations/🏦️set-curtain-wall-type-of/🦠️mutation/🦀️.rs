//! 🏦️ `set-curtain-wall-type-of` payload. Builds a curtain wall as another curtain wall type; its grid, mullions and panels follow by inference.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCurtainWallTypeOf {
    pub id: String,
    pub curtain_wall_type: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetCurtainWallTypeOf {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "curtain-wall", kind: "set-curtain-wall-type-of", record: "SetCurtainWallTypeOf" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set type of curtain wall \"{}\" to \"{}\"", self.id, self.curtain_wall_type), &format!("Typ von Vorhangfassade \"{}\" auf \"{}\" setzen", self.id, self.curtain_wall_type))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
