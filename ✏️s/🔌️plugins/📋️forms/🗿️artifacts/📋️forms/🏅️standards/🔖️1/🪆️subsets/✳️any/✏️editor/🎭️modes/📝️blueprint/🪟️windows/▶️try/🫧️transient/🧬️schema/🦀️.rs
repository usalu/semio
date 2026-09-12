//! 🫧️ Forms Try exact-window transient schema.

use framework_schema::ArtifactSchema;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.forms.forms.try-window-transient")]
pub struct FormsTryWindowTransient {
    #[state(window_transient)]
    pub try_values: BTreeMap<String, Vec<String>>,
}
