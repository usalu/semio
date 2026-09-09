//! 👁️ Sets the preview shading mode (`shaded`/`shaded+edges`/`wireframe`/`points`).

use super::{Generation3dConfig, Generation3dConfigMutation};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "show-mode")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetShowMode {
    pub value: String,
}

impl protocol::MutationKind<Generation3dConfig, Generation3dConfigMutation> for SetShowMode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "show-mode", kind: "set-show-mode", record: "SetShowMode" };

    fn diff(&self, base: &Generation3dConfig) -> protocol::MutationOutcome<Generation3dConfig> {
        let mut next = base.clone();
        next.show_mode.clone_from(&self.value);
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Generation3dConfig) -> Vec<Generation3dConfigMutation> {
        vec![Self { value: base.show_mode.clone() }.into()]
    }

    fn label(&self) -> String {
        "Set Show Mode".into()
    }

    fn target(&self) -> Vec<String> {
        vec!["showMode".into()]
    }
}
