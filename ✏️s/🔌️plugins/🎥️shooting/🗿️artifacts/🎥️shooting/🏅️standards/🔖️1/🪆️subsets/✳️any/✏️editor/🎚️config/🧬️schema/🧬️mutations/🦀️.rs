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
#[path = "🏷️set-camera-draft-label/🦀️.rs"]
mod set_camera_draft_label;
pub use set_camera_draft_label::SetCameraDraftLabel;
#[path = "🎥️set-camera/🦀️.rs"]
mod set_camera;
pub use set_camera::SetCamera;
#[path = "🧰️set-active-utility/🦀️.rs"]
mod set_active_utility;
pub use set_active_utility::SetActiveUtility;
#[path = "🗣️set-locale/🦀️.rs"]
mod set_locale;
pub use set_locale::SetLocale;
#[path = "🔧️set-defaults/🦀️.rs"]
mod set_defaults;
pub use set_defaults::SetDefaults;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslOps, dsl::Mutations)]
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
    #[dsl(key = "set-camera-draft-label")]
    SetCameraDraftLabel(SetCameraDraftLabel),
    #[dsl(key = "set-camera")]
    SetCamera(SetCamera),
    #[dsl(key = "set-active-utility")]
    SetActiveUtility(SetActiveUtility),
    #[dsl(key = "set-locale")]
    SetLocale(SetLocale),
    #[dsl(key = "set-defaults")]
    SetDefaults(SetDefaults),
}

impl protocol::OpText for ShootingConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        for (keyword, spec_fn) in <Self as dsl::DslVariants>::variants() {
            if line == keyword || line.starts_with(&format!("{keyword} ")) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(&keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec = variants.iter().find(|(key, _)| key == &keyword).expect("declared variant").1();
        dsl::print(&record, &spec, dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for ShootingConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { dsl::variants_binary::encode_op(self) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { dsl::variants_binary::decode_op(bytes) }
}
