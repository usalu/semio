//! 👁️ Replaces the viewer's ephemeral local-only evaluated flow output — the render input
//! every preview repaint reads instead of re-evaluating the whole fixture from scratch.

use super::{Generation3dViewTransientPatch, Generation3dViewTransient, Generation3dViewTransientMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-preview-eval")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPreviewEval {
    pub eval_text: Option<String>,
}

impl protocol::MutationKind<Generation3dViewTransient, Generation3dViewTransientMutation> for SetPreviewEval {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "preview-eval", kind: "set-preview-eval", record: "SetPreviewEval" };

    fn diff(&self, base: &Generation3dViewTransient) -> protocol::MutationOutcome<Generation3dViewTransientPatch> {
        protocol::MutationOutcome::new(Generation3dViewTransientPatch { preview_eval_text: Some(Generation3dPreviewEvalChange { text: self.eval_text.clone() }), ..Default::default() })
    }

    fn inverse(&self, base: &Generation3dViewTransient) -> Result<Vec<Generation3dViewTransientMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { eval_text: base.preview_eval_text.clone() }.into()]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Preview Eval", "Vorschauauswertung setzen")
    }

    fn target(&self) -> Vec<String> {
        vec!["previewEvalText".into()]
    }
}
