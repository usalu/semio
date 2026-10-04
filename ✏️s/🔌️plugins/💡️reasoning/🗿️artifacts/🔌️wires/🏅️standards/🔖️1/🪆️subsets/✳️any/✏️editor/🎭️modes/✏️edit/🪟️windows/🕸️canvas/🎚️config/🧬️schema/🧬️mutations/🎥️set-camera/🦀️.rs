//! 🎥️ Sets the viewport of one addressed Wires canvas.

use super::{WiresCanvasCamera, WiresCanvasWindowConfig, WiresCanvasWindowConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
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
            return protocol::MutationOutcome::new(base.clone()).warning("mutation.no-op", "Configuration field is unchanged.");
        }
        protocol::MutationOutcome::new(WiresCanvasWindowConfig { camera: self.camera.clone() })
    }
    fn inverse(&self, base: &WiresCanvasWindowConfig) -> Result<Vec<WiresCanvasWindowConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        (base.camera != self.camera).then(|| WiresCanvasWindowConfigMutation::SetCamera(SetCamera { camera: base.camera.clone() })).into_iter().collect()
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Camera", "Kamera setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["camera".into()]
    }
}
