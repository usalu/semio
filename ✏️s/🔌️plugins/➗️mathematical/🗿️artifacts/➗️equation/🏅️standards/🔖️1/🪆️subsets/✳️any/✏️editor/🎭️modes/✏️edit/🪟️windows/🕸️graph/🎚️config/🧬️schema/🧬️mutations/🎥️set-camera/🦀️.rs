//! 🎥️ SetCamera changes only the addressed configuration field.

use super::{EquationCamera, EquationGraphWindowConfig, EquationGraphWindowConfigDiff, EquationGraphWindowConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[dsl(keyword = "set-camera")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: EquationCamera,
}

impl protocol::MutationKind<EquationGraphWindowConfig, EquationGraphWindowConfigMutation> for SetCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "camera", kind: "set-camera", record: "SetCamera" };
    fn diff(&self, base: &EquationGraphWindowConfig) -> protocol::MutationOutcome<EquationGraphWindowConfigDiff> {
        if base.camera == self.camera {
            return protocol::MutationOutcome::new(EquationGraphWindowConfigDiff::default()).warning("mutation.no-op", "Configuration field is unchanged.");
        }
        protocol::MutationOutcome::new(EquationGraphWindowConfigDiff { camera: Some(self.camera.clone()) })
    }
    fn inverse(&self, base: &EquationGraphWindowConfig) -> Result<Vec<EquationGraphWindowConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        if base.camera == self.camera {
            Vec::new()
        } else {
            vec![EquationGraphWindowConfigMutation::SetCamera(SetCamera { camera: base.camera.clone() })]
        }
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Camera", "Kamera setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["camera".into()]
    }
}
