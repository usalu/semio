//! 👁️ Replaces the app-local Generation2d preview output.

use super::{Generation2dTransient, Generation2dTransientMutation};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-generation-preview")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetGenerationPreview {
    #[dsl(block)]
    pub preview_text: Option<String>,
}

impl protocol::MutationKind<Generation2dTransient, Generation2dTransientMutation> for SetGenerationPreview {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "generation-preview", kind: "set-generation-preview", record: "SetGenerationPreview" };
    fn diff(&self, base: &Generation2dTransient) -> protocol::MutationOutcome<Generation2dTransient> {
        let mut next = base.clone();
        next.generation_preview_text.clone_from(&self.preview_text);
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &Generation2dTransient) -> Vec<Generation2dTransientMutation> {
        vec![Self { preview_text: base.generation_preview_text.clone() }.into()]
    }
    fn label(&self) -> String {
        "Set Generation Preview".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["generationPreviewText".into()]
    }
}
