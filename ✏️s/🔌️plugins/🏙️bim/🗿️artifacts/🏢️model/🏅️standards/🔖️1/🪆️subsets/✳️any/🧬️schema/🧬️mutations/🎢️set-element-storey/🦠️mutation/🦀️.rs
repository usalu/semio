//! 🎢️ `set-element-storey` payload. Stands a storey-placed element (wall, curtain wall, column, beam, slab, ceiling, roof, stair, railing, ramp, space) on another storey of its building. Hosted openings follow their host by reference; refused while the top constraint would no longer lie above the base or a hosted opening would rise above the new height.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetElementStorey {
    pub id: String,
    pub storey: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetElementStorey {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "element", kind: "set-element-storey", record: "SetElementStorey" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move element \"{}\" to storey \"{}\"", self.id, self.storey), &format!("Element \"{}\" in Geschoss \"{}\" verschieben", self.id, self.storey))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
