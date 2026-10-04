//! 📷️ Sets the 3d preview viewport camera (orbit/pan/zoom of the evaluated geometry).

use super::{Generation3dConfig, Generation3dConfigMutation, Generation3dPreviewCamera};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
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
        if base.preview_camera == self.camera {
            return protocol::MutationOutcome::new(base.clone()).warning("mutation.no-op", "Preview camera is already in the requested state.");
        }
        let mut next = base.clone();
        next.preview_camera = self.camera.clone();
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Generation3dConfig) -> Result<Vec<Generation3dConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { camera: base.preview_camera.clone() }.into()]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Preview Camera", "Vorschaukamera setzen")
    }

    fn target(&self) -> Vec<String> {
        vec!["previewCamera".into()]
    }
}
