//! 👁️ Replaces the viewer's ephemeral local-only evaluated flow output — the render input
//! every preview repaint reads instead of re-evaluating the whole fixture from scratch.

use super::{Generation3dViewTransient, Generation3dViewTransientMutation};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-preview-eval")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPreviewEval {
    pub eval_text: Option<String>,
}

impl protocol::MutationKind<Generation3dViewTransient, Generation3dViewTransientMutation> for SetPreviewEval {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "preview-eval", kind: "set-preview-eval", record: "SetPreviewEval" };

    fn diff(&self, base: &Generation3dViewTransient) -> protocol::MutationOutcome<Generation3dViewTransient> {
        let mut next = base.clone();
        next.preview_eval_text.clone_from(&self.eval_text);
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Generation3dViewTransient) -> Vec<Generation3dViewTransientMutation> {
        vec![Self { eval_text: base.preview_eval_text.clone() }.into()]
    }

    fn label(&self) -> String {
        "Set Preview Eval".into()
    }

    fn target(&self) -> Vec<String> {
        vec!["previewEvalText".into()]
    }
}
