//! 📷️ Sets the read-only preview viewport camera — one whole facet per orbit/pan/zoom gesture.

use super::{Generation3dViewConfigPatch, Generation3dViewConfig, Generation3dViewConfigMutation, Generation3dViewCamera};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "preview-camera")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPreviewCamera {
    #[dsl(block)]
    pub camera: Generation3dViewCamera,
}

impl protocol::MutationKind<Generation3dViewConfig, Generation3dViewConfigMutation> for SetPreviewCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "preview-camera", kind: "set-preview-camera", record: "SetPreviewCamera" };

    fn diff(&self, base: &Generation3dViewConfig) -> protocol::MutationOutcome<Generation3dViewConfigPatch> {
        protocol::MutationOutcome::new(Generation3dViewConfigPatch { preview_camera: Some(self.camera.clone()), ..Default::default() })
    }

    fn inverse(&self, base: &Generation3dViewConfig) -> Result<Vec<Generation3dViewConfigMutation>, semio_framework_value::ValueError> {
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
