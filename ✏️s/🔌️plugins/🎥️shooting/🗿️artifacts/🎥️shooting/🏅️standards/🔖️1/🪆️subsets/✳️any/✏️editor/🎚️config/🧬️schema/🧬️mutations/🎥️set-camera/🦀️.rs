//! 🧬️ Set Camera in the shooting.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-camera")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: ShootingCamera,
}

impl protocol::MutationKind<ShootingConfig, ShootingConfigMutation> for SetCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "camera", kind: "set-camera", record: "SetCamera" };
    fn diff(&self, base: &ShootingConfig) -> protocol::MutationOutcome<ShootingConfigDiff> {
        match base.camera == self.camera {
            true => protocol::MutationOutcome::empty().warning("mutation.no-op", "Camera is unchanged."),
            false => protocol::MutationOutcome::new(ShootingConfigDiff { camera: Some(self.camera.clone()), ..Default::default() }),
        }
    }
    fn inverse(&self, base: &ShootingConfig) -> Result<Vec<ShootingConfigMutation>, semio_framework_value::ValueError> {
    Ok(vec![ShootingConfigMutation::SetCamera(Self { camera: base.camera.clone() })])
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Camera", "Kamera setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["camera".into()]
    }
}
