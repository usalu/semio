//! 📷️ Publishes this viewer's live preview camera so a co-viewer can follow the same view.

use super::{Generation3dViewPresence, Generation3dViewPresenceMutation, Generation3dViewCamera};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "preview-camera")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPreviewCamera {
    #[dsl(block)]
    pub camera: Generation3dViewCamera,
}

impl protocol::MutationKind<Generation3dViewPresence, Generation3dViewPresenceMutation> for SetPreviewCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "preview-camera", kind: "set-preview-camera", record: "SetPreviewCamera" };

    fn diff(&self, base: &Generation3dViewPresence) -> protocol::MutationOutcome<Generation3dViewPresence> {
        let mut next = base.clone();
        next.preview_camera.clone_from(&self.camera);
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Generation3dViewPresence) -> Vec<Generation3dViewPresenceMutation> {
        vec![Self { camera: base.preview_camera.clone() }.into()]
    }

    fn label(&self) -> String {
        "Set Preview Camera".into()
    }

    fn target(&self) -> Vec<String> {
        vec!["previewCamera".into()]
    }
}
