//! 🕸️ Sets the flow-graph node-canvas camera (pan/zoom of the widget DAG).

use super::{CameraJson, Generation3dConfig, Generation3dConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
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
        if base.camera == self.camera {
            return protocol::MutationOutcome::new(base.clone()).warning("mutation.no-op", "Flow graph camera is already in the requested state.");
        }
        let mut next = base.clone();
        next.camera = self.camera.clone();
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Generation3dConfig) -> Result<Vec<Generation3dConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { camera: base.camera.clone() }.into()]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Camera", "Kamera setzen")
    }

    fn target(&self) -> Vec<String> {
        vec!["camera".into()]
    }
}
