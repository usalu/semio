//! 🪝️ `set-beam-axis` payload. Replaces the axis of a beam, a line or an arc by bulge, exactly like the axis of a wall; this is how a beam is moved, stretched, curved or straightened. The inferred sweep, joins and quantities follow.

use crate::{Axis, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetBeamAxis {
    pub id: String,
    pub axis: Axis,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetBeamAxis {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "beam", kind: "set-beam-axis", record: "SetBeamAxis" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Reshape the axis of beam \"{}\"", self.id), &format!("Achse von Träger \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
