//! 🌞️ Sets the JSON-encoded `WorldSunConfig` driving the preview sun rig.

use super::{Generation3dConfigPatch, Generation3dConfig, Generation3dConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "sun")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetSun {
    pub json: String,
}

impl protocol::MutationKind<Generation3dConfig, Generation3dConfigMutation> for SetSun {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "sun", kind: "set-sun", record: "SetSun" };

    fn diff(&self, base: &Generation3dConfig) -> protocol::MutationOutcome<Generation3dConfigPatch> {
        protocol::MutationOutcome::new(Generation3dConfigPatch { sun_json: Some(self.json.clone()), ..Default::default() })
    }

    fn inverse(&self, base: &Generation3dConfig) -> Result<Vec<Generation3dConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { json: base.sun_json.clone() }.into()]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Sun", "Sonne setzen")
    }

    fn target(&self) -> Vec<String> {
        vec!["sunJson".into()]
    }
}
