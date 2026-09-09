//! 🧬️ Set Camera in the layout.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-camera")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: LayoutCamera,
}

impl protocol::MutationKind<LayoutConfig, LayoutConfigMutation> for SetCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "camera", kind: "set-camera", record: "SetCamera" };
    fn diff(&self, base: &LayoutConfig) -> protocol::MutationOutcome<LayoutConfig> {
        let mut next = base.clone();
        next.camera = self.camera.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &LayoutConfig) -> Vec<LayoutConfigMutation> {
        vec![LayoutConfigMutation::SetCamera(Self { camera: base.camera.clone() })]
    }
    fn label(&self) -> String {
        "Set Camera".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["camera".into()]
    }
}
