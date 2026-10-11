//! 🎥️ Change Camera in the WFC 3D config facet — one mutation per settled pan/zoom/orbit gesture.

use super::{Wfc3dConfig, Wfc3dConfigDiff, Wfc3dConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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
    fn diff(&self, base: &Wfc3dConfig) -> protocol::MutationOutcome<Wfc3dConfigDiff> {
        let diff = Wfc3dConfigDiff { camera_x: (base.camera_x != self.x).then_some(self.x), camera_y: (base.camera_y != self.y).then_some(self.y), camera_zoom: (base.camera_zoom != self.zoom).then_some(self.zoom), ..Default::default() };
        if protocol::DiffAlgebra::<Wfc3dConfig>::is_empty(&diff) {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The camera already holds that pose.");
        }
        protocol::MutationOutcome::new(diff)
    }
    fn inverse(&self, base: &Wfc3dConfig) -> Result<Vec<Wfc3dConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Wfc3dConfigMutation::ChangeCamera(ChangeCamera { x: base.camera_x, y: base.camera_y, zoom: base.camera_zoom })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change Camera", "Kamera ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["camera".into()]
    }
}
