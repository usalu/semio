//! 🎥️ Sets the viewport of one addressed Wires canvas.

use super::{WiresCanvasCamera, WiresCanvasWindowConfig, WiresCanvasWindowConfigMutation};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[dsl(keyword = "set-camera")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: WiresCanvasCamera,
}

impl protocol::MutationKind<WiresCanvasWindowConfig, WiresCanvasWindowConfigMutation> for SetCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "camera", kind: "set-camera", record: "SetCamera" };
    fn diff(&self, base: &WiresCanvasWindowConfig) -> protocol::MutationOutcome<WiresCanvasWindowConfig> {
        if base.camera == self.camera {
            return protocol::MutationOutcome::new(base.clone()).warn("mutation.no-op", "Configuration field is unchanged.");
        }
        protocol::MutationOutcome::new(WiresCanvasWindowConfig { camera: self.camera.clone() })
    }
    fn inverse(&self, base: &WiresCanvasWindowConfig) -> Vec<WiresCanvasWindowConfigMutation> {
        (base.camera != self.camera).then(|| WiresCanvasWindowConfigMutation::SetCamera(SetCamera { camera: base.camera.clone() })).into_iter().collect()
    }
    fn label(&self) -> String {
        "Set Camera".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["camera".into()]
    }
}
