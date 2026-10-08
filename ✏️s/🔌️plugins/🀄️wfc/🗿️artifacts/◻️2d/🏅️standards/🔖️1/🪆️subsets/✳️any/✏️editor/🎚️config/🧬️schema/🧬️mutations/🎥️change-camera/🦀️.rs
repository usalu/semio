//! 🎥️ Change Camera in the WFC 2D config facet — one mutation per settled pan/zoom gesture.

use super::{Wfc2dConfig, Wfc2dConfigDiff, Wfc2dConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
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
    fn diff(&self, base: &Wfc2dConfig) -> protocol::MutationOutcome<Wfc2dConfigDiff> {
        let diff = Wfc2dConfigDiff { camera_x: (base.camera_x != self.x).then_some(self.x), camera_y: (base.camera_y != self.y).then_some(self.y), camera_zoom: (base.camera_zoom != self.zoom).then_some(self.zoom), ..Default::default() };
        if protocol::DiffAlgebra::<Wfc2dConfig>::is_empty(&diff) {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The camera already holds that pose.");
        }
        protocol::MutationOutcome::new(diff)
    }
    fn inverse(&self, base: &Wfc2dConfig) -> Result<Vec<Wfc2dConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Wfc2dConfigMutation::ChangeCamera(ChangeCamera { x: base.camera_x, y: base.camera_y, zoom: base.camera_zoom })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change Camera", "Kamera ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["camera".into()]
    }
}
