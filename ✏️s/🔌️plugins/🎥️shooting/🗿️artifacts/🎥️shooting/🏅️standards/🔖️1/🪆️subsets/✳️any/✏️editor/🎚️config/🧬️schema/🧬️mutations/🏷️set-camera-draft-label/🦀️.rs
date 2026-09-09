//! 🧬️ Set Camera Draft Label in the shooting.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-camera-draft-label")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCameraDraftLabel {
    pub value: String,
}

impl protocol::MutationKind<ShootingConfig, ShootingConfigMutation> for SetCameraDraftLabel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "camera-draft-label", kind: "set-camera-draft-label", record: "SetCameraDraftLabel" };
    fn diff(&self, base: &ShootingConfig) -> protocol::MutationOutcome<ShootingConfig> {
        let mut next = base.clone();
        next.camera_draft_label = self.value.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &ShootingConfig) -> Vec<ShootingConfigMutation> {
        vec![ShootingConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })]
    }
    fn label(&self) -> String {
        "Set Camera Draft Label".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["camera_draft_label".into()]
    }
}
