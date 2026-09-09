//! 👁️ Publishes this viewer's live shading mode alongside its camera.

use super::{Generation3dViewPresence, Generation3dViewPresenceMutation};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "show-mode")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetShowMode {
    pub value: String,
}

impl protocol::MutationKind<Generation3dViewPresence, Generation3dViewPresenceMutation> for SetShowMode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "show-mode", kind: "set-show-mode", record: "SetShowMode" };

    fn diff(&self, base: &Generation3dViewPresence) -> protocol::MutationOutcome<Generation3dViewPresence> {
        let mut next = base.clone();
        next.show_mode.clone_from(&self.value);
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Generation3dViewPresence) -> Vec<Generation3dViewPresenceMutation> {
        vec![Self { value: base.show_mode.clone() }.into()]
    }

    fn label(&self) -> String {
        "Set Show Mode".into()
    }

    fn target(&self) -> Vec<String> {
        vec!["showMode".into()]
    }
}
