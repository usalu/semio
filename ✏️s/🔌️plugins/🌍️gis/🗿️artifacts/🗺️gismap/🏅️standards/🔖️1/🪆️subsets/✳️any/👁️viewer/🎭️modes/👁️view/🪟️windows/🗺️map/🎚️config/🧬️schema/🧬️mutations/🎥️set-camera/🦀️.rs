//! 🎥️ Sets the retained camera of ONE addressed `gis2d-view-map` viewer window.

use super::{GisMapViewerCamera, GisMapViewerWindowConfig, GisMapViewerWindowConfigMutation};

#[derive(Clone, Copy, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[dsl(keyword = "set-camera")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: GisMapViewerCamera,
}

impl protocol::MutationKind<GisMapViewerWindowConfig, GisMapViewerWindowConfigMutation> for SetCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "camera", kind: "set-camera", record: "SetCamera" };
    fn diff(&self, base: &GisMapViewerWindowConfig) -> protocol::MutationOutcome<GisMapViewerWindowConfig> {
        if base.camera == self.camera {
            return protocol::MutationOutcome::new(*base).warn("mutation.no-op", "The map window already holds this camera.");
        }
        protocol::MutationOutcome::new(GisMapViewerWindowConfig { camera: self.camera })
    }
    fn inverse(&self, base: &GisMapViewerWindowConfig) -> Vec<GisMapViewerWindowConfigMutation> {
        (base.camera != self.camera).then(|| GisMapViewerWindowConfigMutation::SetCamera(SetCamera { camera: base.camera })).into_iter().collect()
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Set camera", "Kamera setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["camera".into()]
    }
}
