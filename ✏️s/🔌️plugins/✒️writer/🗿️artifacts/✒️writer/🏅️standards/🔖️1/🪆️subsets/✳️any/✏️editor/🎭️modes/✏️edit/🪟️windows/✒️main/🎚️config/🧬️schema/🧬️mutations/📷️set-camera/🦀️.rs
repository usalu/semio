use super::{WriterMainWindowConfig, WriterMainWindowConfigDiff, WriterMainWindowConfigMutation};
use crate::WriterCamera;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-camera")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: WriterCamera,
}

impl protocol::MutationKind<WriterMainWindowConfig, WriterMainWindowConfigMutation> for SetCamera {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "writer-window-camera", kind: "set-camera", record: "SetCamera" };

    fn diff(&self, base: &WriterMainWindowConfig) -> protocol::MutationOutcome<WriterMainWindowConfigDiff> {
        protocol::MutationOutcome::new(WriterMainWindowConfigDiff { camera: (base.camera != self.camera).then(|| self.camera.clone()), ..Default::default() })
    }

    fn inverse(&self, base: &WriterMainWindowConfig) -> Result<Vec<WriterMainWindowConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { camera: base.camera.clone() }.into()]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Writer Window Camera", "Kamera des Schreibfensters setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["camera".into()]
    }
}

#[cfg(test)]
mod law_tests {
    use super::*;

    /// ⚖️ The inverse diffs sum to the negative of the forward diff (L3).
    #[semio_framework_async_macros::async_test]
    async fn inverse_diffs_sum_to_the_negative_diff() {
        let base = WriterMainWindowConfig::default();
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&WriterMainWindowConfigMutation::from(SetCamera { camera: crate::WriterCamera { x: 3.0, y: -1.5, zoom: 2.0 } }), &base).await;
    }
}
