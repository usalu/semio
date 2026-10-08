//! 👁️ Sets the preview shading mode (`shaded`/`shaded+edges`/`wireframe`/`points`).

use super::{Generation3dConfigPatch, Generation3dConfig, Generation3dConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "show-mode")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetShowMode {
    pub value: String,
}

impl protocol::MutationKind<Generation3dConfig, Generation3dConfigMutation> for SetShowMode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "show-mode", kind: "set-show-mode", record: "SetShowMode" };

    fn diff(&self, base: &Generation3dConfig) -> protocol::MutationOutcome<Generation3dConfigPatch> {
        protocol::MutationOutcome::new(Generation3dConfigPatch { show_mode: Some(self.value.clone()), ..Default::default() })
    }

    fn inverse(&self, base: &Generation3dConfig) -> Result<Vec<Generation3dConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { value: base.show_mode.clone() }.into()]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Show Mode", "Anzeigemodus setzen")
    }

    fn target(&self) -> Vec<String> {
        vec!["showMode".into()]
    }
}
