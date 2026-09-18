//! 🎥️ Change Camera in the WFC 2D config facet — one mutation per settled pan/zoom gesture.

use super::{Wfc2dConfig, Wfc2dConfigMutation};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[dsl(keyword = "change-camera")]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl protocol::MutationKind<Wfc2dConfig, Wfc2dConfigMutation> for ChangeCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "camera", kind: "change-camera", record: "ChangeCamera" };
    fn diff(&self, base: &Wfc2dConfig) -> protocol::MutationOutcome<Wfc2dConfig> {
        protocol::MutationOutcome::new(Wfc2dConfig { camera_x: self.x, camera_y: self.y, camera_zoom: self.zoom, ..base.clone() })
    }
    fn inverse(&self, base: &Wfc2dConfig) -> Vec<Wfc2dConfigMutation> {
        vec![Wfc2dConfigMutation::ChangeCamera(ChangeCamera { x: base.camera_x, y: base.camera_y, zoom: base.camera_zoom })]
    }
    fn label(&self) -> String {
        "Change Camera".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["camera".into()]
    }
}
