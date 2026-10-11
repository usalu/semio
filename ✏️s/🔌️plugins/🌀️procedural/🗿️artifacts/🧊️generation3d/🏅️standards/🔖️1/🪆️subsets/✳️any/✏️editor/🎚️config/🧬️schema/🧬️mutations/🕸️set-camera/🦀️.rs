//! 🕸️ Sets the flow-graph node-canvas camera (pan/zoom of the widget DAG).

use super::{Generation3dConfigPatch, CameraJson, Generation3dConfig, Generation3dConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "camera")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: CameraJson,
}

impl protocol::MutationKind<Generation3dConfig, Generation3dConfigMutation> for SetCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "camera", kind: "set-camera", record: "SetCamera" };

    fn diff(&self, base: &Generation3dConfig) -> protocol::MutationOutcome<Generation3dConfigPatch> {
        if base.camera == self.camera {
            return protocol::MutationOutcome::new(Generation3dConfigPatch::default()).warning("mutation.no-op", "Flow graph camera is already in the requested state.");
        }
        protocol::MutationOutcome::new(Generation3dConfigPatch { camera: Some(self.camera.clone()), ..Default::default() })
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
