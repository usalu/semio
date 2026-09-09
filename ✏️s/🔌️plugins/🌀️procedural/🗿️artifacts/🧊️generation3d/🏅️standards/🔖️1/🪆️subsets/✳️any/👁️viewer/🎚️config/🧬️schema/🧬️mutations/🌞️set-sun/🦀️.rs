//! 🌞️ Sets the read-only preview's sun (enabled/azimuth/elevation/intensity), JSON-encoded.

use super::{Generation3dViewConfig, Generation3dViewConfigMutation};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "sun")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetSun {
    pub json: String,
}

impl protocol::MutationKind<Generation3dViewConfig, Generation3dViewConfigMutation> for SetSun {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "sun", kind: "set-sun", record: "SetSun" };

    fn diff(&self, base: &Generation3dViewConfig) -> protocol::MutationOutcome<Generation3dViewConfig> {
        let mut next = base.clone();
        next.sun_json.clone_from(&self.json);
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Generation3dViewConfig) -> Vec<Generation3dViewConfigMutation> {
        vec![Self { json: base.sun_json.clone() }.into()]
    }

    fn label(&self) -> String {
        "Set Sun".into()
    }

    fn target(&self) -> Vec<String> {
        vec!["sunJson".into()]
    }
}
