//! 📷️ Sets the 3d preview viewport camera (orbit/pan/zoom of the evaluated geometry).

use super::{Generation3dConfig, Generation3dConfigMutation, Generation3dPreviewCamera};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "preview-camera")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPreviewCamera {
    #[dsl(block)]
    pub camera: Generation3dPreviewCamera,
}

impl protocol::MutationKind<Generation3dConfig, Generation3dConfigMutation> for SetPreviewCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "preview-camera", kind: "set-preview-camera", record: "SetPreviewCamera" };

    fn diff(&self, base: &Generation3dConfig) -> protocol::MutationOutcome<Generation3dConfig> {
        let mut next = base.clone();
        next.preview_camera = self.camera.clone();
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Generation3dConfig) -> Vec<Generation3dConfigMutation> {
        vec![Self { camera: base.preview_camera.clone() }.into()]
    }

    fn label(&self) -> String {
        "Set Preview Camera".into()
    }

    fn target(&self) -> Vec<String> {
        vec!["previewCamera".into()]
    }
}
