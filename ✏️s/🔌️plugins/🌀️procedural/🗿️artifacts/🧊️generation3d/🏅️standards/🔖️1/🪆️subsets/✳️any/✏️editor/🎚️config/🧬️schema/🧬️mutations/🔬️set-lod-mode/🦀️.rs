//! 🔬️ Sets the tessellation level of detail (`""`/`coarse`/`fine`) the editor previews mesh at.

use super::{Generation3dConfigPatch, Generation3dConfig, Generation3dConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "lod-mode")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetLodMode {
    pub value: String,
}

impl protocol::MutationKind<Generation3dConfig, Generation3dConfigMutation> for SetLodMode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "lod-mode", kind: "set-lod-mode", record: "SetLodMode" };

    fn diff(&self, base: &Generation3dConfig) -> protocol::MutationOutcome<Generation3dConfigPatch> {
        protocol::MutationOutcome::new(Generation3dConfigPatch { lod_mode: Some(self.value.clone()), ..Default::default() })
    }

    fn inverse(&self, base: &Generation3dConfig) -> Result<Vec<Generation3dConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { value: base.lod_mode.clone() }.into()]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Lod Mode", "Detailstufenmodus setzen")
    }

    fn target(&self) -> Vec<String> {
        vec!["lodMode".into()]
    }
}
