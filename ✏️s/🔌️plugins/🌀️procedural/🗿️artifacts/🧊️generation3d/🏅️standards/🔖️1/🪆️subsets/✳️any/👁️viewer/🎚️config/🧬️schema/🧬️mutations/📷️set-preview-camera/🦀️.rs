//! 📷️ Sets the read-only preview viewport camera — one whole facet per orbit/pan/zoom gesture.

use super::{Generation3dViewConfig, Generation3dViewConfigMutation, Generation3dViewCamera};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "preview-camera")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPreviewCamera {
    #[dsl(block)]
    pub camera: Generation3dViewCamera,
}

impl protocol::MutationKind<Generation3dViewConfig, Generation3dViewConfigMutation> for SetPreviewCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "preview-camera", kind: "set-preview-camera", record: "SetPreviewCamera" };

    fn diff(&self, base: &Generation3dViewConfig) -> protocol::MutationOutcome<Generation3dViewConfig> {
        let mut next = base.clone();
        next.preview_camera.clone_from(&self.camera);
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Generation3dViewConfig) -> Vec<Generation3dViewConfigMutation> {
        vec![Self { camera: base.preview_camera.clone() }.into()]
    }

    fn label(&self) -> String {
        "Set Preview Camera".into()
    }

    fn target(&self) -> Vec<String> {
        vec!["previewCamera".into()]
    }
}
