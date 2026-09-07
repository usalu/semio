//! 🎥️ SetCamera changes only the addressed configuration field.

use super::{EquationConfig, EquationConfigMutation, EquationCamera};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[dsl(keyword = "camera")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: EquationCamera,
}

impl protocol::MutationKind<EquationConfig, EquationConfigMutation> for SetCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "camera", kind: "set-camera", record: "SetCamera" };
    fn diff(&self, base: &EquationConfig) -> protocol::MutationOutcome<EquationConfig> {
        if base.camera == self.camera { return protocol::MutationOutcome::new(base.clone()).warn("mutation.no-op", "Configuration field is unchanged."); }
        protocol::MutationOutcome::new(EquationConfig { camera: self.camera.clone(), ..base.clone() })
    }
    fn inverse(&self, base: &EquationConfig) -> Vec<EquationConfigMutation> {
        if base.camera == self.camera { Vec::new() } else { vec![EquationConfigMutation::SetCamera(SetCamera { camera: base.camera.clone() })] }
    }
    fn label(&self) -> String { "Set Camera".into() }
    fn target(&self) -> Vec<String> { vec!["camera".into()] }
}
