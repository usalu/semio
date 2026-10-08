//! 👁️ Replaces the app-local Generation2d preview output.

use super::{Generation2dTransientPatch, Generation2dTransient, Generation2dTransientMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-generation-preview")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetGenerationPreview {
    #[dsl(block)]
    pub preview_text: Option<String>,
}

impl protocol::MutationKind<Generation2dTransient, Generation2dTransientMutation> for SetGenerationPreview {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "generation-preview", kind: "set-generation-preview", record: "SetGenerationPreview" };
    fn diff(&self, base: &Generation2dTransient) -> protocol::MutationOutcome<Generation2dTransientPatch> {
        protocol::MutationOutcome::new(Generation2dTransientPatch { generation_preview_text: Some(Generation2dPreviewTextChange { text: self.preview_text.clone() }), ..Default::default() })
    }
    fn inverse(&self, base: &Generation2dTransient) -> Result<Vec<Generation2dTransientMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { preview_text: base.generation_preview_text.clone() }.into()]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Generation Preview", "Erzeugungsvorschau setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["generationPreviewText".into()]
    }
}
