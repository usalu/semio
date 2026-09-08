//! 🧬️ Set Camera in the Writer config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-camera")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: WriterCamera,
}

impl protocol::MutationKind<WriterConfig, WriterConfigMutation> for SetCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "camera", kind: "set-camera", record: "SetCamera" };
    fn diff(&self, base: &WriterConfig) -> protocol::MutationOutcome<WriterConfig> {
        let mut next = base.clone();
        next.camera = self.camera.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &WriterConfig) -> Vec<WriterConfigMutation> { vec![WriterConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Camera".into() }
    fn target(&self) -> Vec<String> { vec!["camera".into()] }
}
