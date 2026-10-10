//! 🪛️ `create-wall-sweep` payload. Brings a new sweep onto one face of a wall: a profile run along the face (a baseboard, a cornice, a drip rail) at a height above the wall base, optionally set into the wall; its solid, length and areas are inferred.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, WallSweep};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateWallSweep {
    pub id: String,
    pub wall_sweep: WallSweep,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateWallSweep {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "wall-sweep", kind: "create-wall-sweep", record: "CreateWallSweep" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create wall sweep \"{}\"", self.wall_sweep.name), &format!("Wandprofil \"{}\" anlegen", self.wall_sweep.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
