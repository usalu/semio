//! 👁️ Replaces the app-local computed preview shared by Generation3d generation windows.

use super::{Generation3dTransient, Generation3dTransientMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-generation-preview")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetGenerationPreview {
    pub preview_text: Option<String>,
}

impl protocol::MutationKind<Generation3dTransient, Generation3dTransientMutation> for SetGenerationPreview {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "generation-preview", kind: "set-generation-preview", record: "SetGenerationPreview" };
    fn diff(&self, base: &Generation3dTransient) -> protocol::MutationOutcome<Generation3dTransient> {
        let mut next = base.clone();
        next.generation_preview_text.clone_from(&self.preview_text);
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &Generation3dTransient) -> Result<Vec<Generation3dTransientMutation>, semio_framework_value::ValueError> {
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
