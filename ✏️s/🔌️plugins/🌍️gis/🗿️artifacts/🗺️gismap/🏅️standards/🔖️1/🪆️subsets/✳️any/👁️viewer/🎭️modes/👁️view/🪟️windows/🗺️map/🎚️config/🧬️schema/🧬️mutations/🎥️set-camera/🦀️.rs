//! 🎥️ Sets the retained camera of ONE addressed `gis2d-view-map` viewer window.

use super::{GisMapViewerCamera, GisMapViewerWindowConfig, GisMapViewerWindowConfigDiff, GisMapViewerWindowConfigMutation};

#[derive(Clone, Copy, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[dsl(keyword = "set-camera")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: GisMapViewerCamera,
}

impl protocol::MutationKind<GisMapViewerWindowConfig, GisMapViewerWindowConfigMutation> for SetCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "camera", kind: "set-camera", record: "SetCamera" };
    fn diff(&self, base: &GisMapViewerWindowConfig) -> protocol::MutationOutcome<GisMapViewerWindowConfigDiff> {
        if base.camera == self.camera {
            return protocol::MutationOutcome::new(GisMapViewerWindowConfigDiff::default()).warning("mutation.no-op", "The map window already holds this camera.");
        }
        protocol::MutationOutcome::new(GisMapViewerWindowConfigDiff { camera: Some(self.camera) })
    }
    fn inverse(&self, base: &GisMapViewerWindowConfig) -> Result<Vec<GisMapViewerWindowConfigMutation>, semio_framework_value::ValueError> {
        Ok((base.camera != self.camera).then(|| GisMapViewerWindowConfigMutation::SetCamera(SetCamera { camera: base.camera })).into_iter().collect())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set camera", "Kamera setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["camera".into()]
    }
}
