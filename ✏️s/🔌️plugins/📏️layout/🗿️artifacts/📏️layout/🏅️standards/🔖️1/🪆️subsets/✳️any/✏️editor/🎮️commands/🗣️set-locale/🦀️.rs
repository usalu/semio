//! 👁️ 👁️ Layout play app commands command — `set-locale`.

use crate::{op::LayoutMutation, LayoutSnapshot};
use crate::editor::layout::config::{LayoutConfig, LayoutConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🗣️ Not declared as a manifest action (locale is host-pushed, not a user-facing app action needing
/// a palette entry), which is why its wire keyword stays the bare `"locale"` rather than the
/// kebab-cased `"set-locale"` its command id would suggest — see the `as` literal in
/// `crate::editor::layout`'s `app_commands!` invocation.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "locale")]
pub struct SetLocale {
    pub value: String,
}

pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    Ok(Emit::config(vec![LayoutConfigMutation::SetLocale(crate::editor::layout::config::SetLocale { value: payload.value.clone() })]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
