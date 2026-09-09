//! 🧬️ Set Preview Camera in the layout.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-preview-camera")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetPreviewCamera {
    #[dsl(block)]
    pub camera: LayoutCamera,
}

impl protocol::MutationKind<LayoutConfig, LayoutConfigMutation> for SetPreviewCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "preview-camera", kind: "set-preview-camera", record: "SetPreviewCamera" };
    fn diff(&self, base: &LayoutConfig) -> protocol::MutationOutcome<LayoutConfig> {
        let mut next = base.clone();
        next.preview_camera = self.camera.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &LayoutConfig) -> Vec<LayoutConfigMutation> {
        vec![LayoutConfigMutation::SetPreviewCamera(Self { camera: base.preview_camera.clone() })]
    }
    fn label(&self) -> String {
        "Set Preview Camera".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["preview_camera".into()]
    }
}
