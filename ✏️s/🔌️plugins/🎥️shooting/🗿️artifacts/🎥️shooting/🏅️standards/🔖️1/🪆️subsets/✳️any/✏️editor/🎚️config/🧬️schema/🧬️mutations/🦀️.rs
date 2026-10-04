//! 🧬️ Shooting configuration mutation collection.

use super::*;
#[path = "📸️replace-config/🦀️.rs"]
mod replace_config;
pub use replace_config::ReplaceConfig;
#[path = "☑️set-shot-selection/🦀️.rs"]
mod set_shot_selection;
pub use set_shot_selection::SetShotSelection;
#[path = "🎯️set-center-model/🦀️.rs"]
mod set_center_model;
pub use set_center_model::SetCenterModel;
#[path = "🔢️set-fit-revision/🦀️.rs"]
mod set_fit_revision;
pub use set_fit_revision::SetFitRevision;
#[path = "🎥️set-camera/🦀️.rs"]
mod set_camera;
pub use set_camera::SetCamera;
#[path = "🔧️set-defaults/🦀️.rs"]
mod set_defaults;
pub use set_defaults::SetDefaults;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = ShootingConfig, diff = ShootingConfig, schema = "shooting.config")]
pub enum ShootingConfigMutation {
    #[dsl(key = "replace-config")]
    ReplaceConfig(ReplaceConfig),
    #[dsl(key = "set-shot-selection")]
    SetShotSelection(SetShotSelection),
    #[dsl(key = "set-center-model")]
    SetCenterModel(SetCenterModel),
    #[dsl(key = "set-fit-revision")]
    SetFitRevision(SetFitRevision),
    #[dsl(key = "set-camera")]
    SetCamera(SetCamera),
    #[dsl(key = "set-defaults")]
    SetDefaults(SetDefaults),
}

impl protocol::OpText for ShootingConfigMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        for (keyword, spec_fn) in <Self as semio_framework_dsl_record::DslVariants>::variants() {
            if line == keyword || line.starts_with(&format!("{keyword} ")) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(&keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec = (variants.iter().find(|(key, _)| key == &keyword).expect("declared variant").1.ordinary)();
        semio_framework_dsl_record::print(&record, &spec, semio_framework_dsl_record::JoinMode::Inline)
    }
}

impl protocol::OpBinary for ShootingConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

//#region 🌉️TestBridge
/// 🌉️ The committed-vector report of this state lane for the language-neutral case adapter, which links only this
/// crate: production dispatch (`Mutation::diff(..).apply_to`) and the mutation's own inverse over `ShootingConfig`.
///
/// @see store::os_store::test_support::mutation_report_json
pub fn shooting_config_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<ShootingConfig, ShootingConfigMutation>(base_json, mutation_json, after_json)
}
//#endregion 🌉️TestBridge
