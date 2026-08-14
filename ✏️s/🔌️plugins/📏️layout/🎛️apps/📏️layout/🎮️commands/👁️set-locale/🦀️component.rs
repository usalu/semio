//! 👁️ 👁️ Layout play app commands command — `set-locale`.

use crate::apps::layout::config::{LayoutConfig, LayoutConfigMutation};
use crate::artifacts::layout::{op::LayoutMutation, LayoutSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

/// 🗣️ Not declared as a manifest action (locale is host-pushed, not a user-facing app action needing
/// a palette entry), which is why its wire keyword stays the bare `"locale"` rather than the
/// kebab-cased `"set-locale"` its command id would suggest — see the `as` literal in
/// `crate::apps::layout`'s `app_commands!` invocation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "locale")]
pub struct SetLocale {
    pub value: String,
}

pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    Ok(Emit::config(vec![LayoutConfigMutation::SetLocale { value: payload.value.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::layout::LayoutCommand;

    #[test]
    fn set_locale_is_host_pushed_with_bare_wire_keyword() {
        let command = LayoutCommand::SetLocale(SetLocale { value: "de-DE".into() });
        assert!(protocol::OpText::print_op(&command).starts_with("locale "), "wire keyword must stay bare 'locale'");
    }
}
//#endregion 🧪️Tests
