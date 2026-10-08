//! 🔬️ Sets the tessellation level of detail (`""`/`coarse`/`fine`) the read-only preview meshes at.

use super::{Generation3dViewConfigPatch, Generation3dViewConfig, Generation3dViewConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "lod-mode")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetLodMode {
    pub value: String,
}

impl protocol::MutationKind<Generation3dViewConfig, Generation3dViewConfigMutation> for SetLodMode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "lod-mode", kind: "set-lod-mode", record: "SetLodMode" };

    fn diff(&self, base: &Generation3dViewConfig) -> protocol::MutationOutcome<Generation3dViewConfigPatch> {
        protocol::MutationOutcome::new(Generation3dViewConfigPatch { lod_mode: Some(self.value.clone()), ..Default::default() })
    }

    fn inverse(&self, base: &Generation3dViewConfig) -> Result<Vec<Generation3dViewConfigMutation>, semio_framework_value::ValueError> {
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
