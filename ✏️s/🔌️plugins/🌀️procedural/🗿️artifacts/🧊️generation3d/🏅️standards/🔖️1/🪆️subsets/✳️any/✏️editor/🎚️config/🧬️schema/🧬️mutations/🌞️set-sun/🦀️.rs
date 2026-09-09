//! 🌞️ Sets the JSON-encoded `WorldSunConfig` driving the preview sun rig.

use super::{Generation3dConfig, Generation3dConfigMutation};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "sun")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetSun {
    pub json: String,
}

impl protocol::MutationKind<Generation3dConfig, Generation3dConfigMutation> for SetSun {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "sun", kind: "set-sun", record: "SetSun" };

    fn diff(&self, base: &Generation3dConfig) -> protocol::MutationOutcome<Generation3dConfig> {
        let mut next = base.clone();
        next.sun_json.clone_from(&self.json);
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Generation3dConfig) -> Vec<Generation3dConfigMutation> {
        vec![Self { json: base.sun_json.clone() }.into()]
    }

    fn label(&self) -> String {
        "Set Sun".into()
    }

    fn target(&self) -> Vec<String> {
        vec!["sunJson".into()]
    }
}
