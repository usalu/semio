//! 🎨️ Names the bundled example document this read-only surface is looking at.
//!
//! 📚️ A viewer opens a document, it never rewrites one: the picked example lives HERE, on the
//! surface's own config lane, and `Generation3dViewer`'s viewed-document resolution reads it. That
//! is the whole difference from the sibling surface's `setActiveExample`, which replaces the
//! artifact's fixture through the document lane (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).

use super::{Generation3dViewConfig, Generation3dViewConfigMutation};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "active-example")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetActiveExample {
    pub value: String,
}

impl protocol::MutationKind<Generation3dViewConfig, Generation3dViewConfigMutation> for SetActiveExample {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "active-example", kind: "set-active-example", record: "SetActiveExample" };

    fn diff(&self, base: &Generation3dViewConfig) -> protocol::MutationOutcome<Generation3dViewConfig> {
        let mut next = base.clone();
        next.active_example_id.clone_from(&self.value);
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Generation3dViewConfig) -> Vec<Generation3dViewConfigMutation> {
        vec![Self { value: base.active_example_id.clone() }.into()]
    }

    fn label(&self) -> String {
        "Set Active Example".into()
    }

    fn target(&self) -> Vec<String> {
        vec!["activeExampleId".into()]
    }
}
