//! 🎥️ Change Camera in the WFC 3D config facet — one mutation per settled pan/zoom/orbit gesture.

use super::{Wfc3dConfig, Wfc3dConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "change-camera")]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl protocol::MutationKind<Wfc3dConfig, Wfc3dConfigMutation> for ChangeCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "camera", kind: "change-camera", record: "ChangeCamera" };
    fn diff(&self, base: &Wfc3dConfig) -> protocol::MutationOutcome<Wfc3dConfig> {
        protocol::MutationOutcome::new(Wfc3dConfig { camera_x: self.x, camera_y: self.y, camera_zoom: self.zoom, ..base.clone() })
    }
    fn inverse(&self, base: &Wfc3dConfig) -> Vec<Wfc3dConfigMutation> {
        vec![Wfc3dConfigMutation::ChangeCamera(ChangeCamera { x: base.camera_x, y: base.camera_y, zoom: base.camera_zoom })]
    }
    fn label(&self) -> String {
        "Change Camera".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["camera".into()]
    }
}
