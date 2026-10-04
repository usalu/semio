//! ⚙️ Replaces the whole editor config in one settled step — the inverse every non-invertible bulk
//! change (example load, config restore) hands back.

use super::{Generation3dConfig, Generation3dConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "snapshot")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetSnapshot {
    #[dsl(block)]
    pub config: Generation3dConfig,
}

impl protocol::MutationKind<Generation3dConfig, Generation3dConfigMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };

    fn diff(&self, _base: &Generation3dConfig) -> protocol::MutationOutcome<Generation3dConfig> {
        protocol::MutationOutcome::new(self.config.clone())
    }

    fn inverse(&self, base: &Generation3dConfig) -> Result<Vec<Generation3dConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { config: base.clone() }.into()]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Snapshot", "Momentaufnahme setzen")
    }

    fn target(&self) -> Vec<String> {
        vec!["config".into()]
    }
}
