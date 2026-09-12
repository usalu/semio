//! 🎚️ Forms Try exact-window configuration schema.

use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.forms.forms.try-window-config")]
pub struct FormsTryWindowConfig {
    #[state(window_config)]
    pub current_step_index: u32,
}
