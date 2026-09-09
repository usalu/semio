//! 🕸️ Sets the flow-graph node-canvas camera (pan/zoom of the widget DAG).

use super::{CameraJson, Generation3dConfig, Generation3dConfigMutation};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "camera")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: CameraJson,
}

impl protocol::MutationKind<Generation3dConfig, Generation3dConfigMutation> for SetCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "camera", kind: "set-camera", record: "SetCamera" };

    fn diff(&self, base: &Generation3dConfig) -> protocol::MutationOutcome<Generation3dConfig> {
        let mut next = base.clone();
        next.camera = self.camera.clone();
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Generation3dConfig) -> Vec<Generation3dConfigMutation> {
        vec![Self { camera: base.camera.clone() }.into()]
    }

    fn label(&self) -> String {
        "Set Camera".into()
    }

    fn target(&self) -> Vec<String> {
        vec!["camera".into()]
    }
}
