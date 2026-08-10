//! 🗣️ Mathematical play app commands — the host-pushed locale change.
//!
//! Not declared as a manifest action (locale is host-pushed, not a user-facing app action needing a
//! palette entry), which is why its wire keyword stays the bare `"locale"` rather than the kebab-cased
//! `"set-locale"` its command id would suggest — see the `as` literal in `crate::apps::mathematical`'s
//! `app_commands!` invocation.

use crate::apps::mathematical::config::{MathematicalConfig, MathematicalConfigMutation};
use crate::artifacts::mathematical::op::MathematicalMutation;
use crate::artifacts::mathematical::MathematicalSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "locale")]
    pub struct SetLocale {
        pub value: String,
    }

    pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, MathematicalSnapshot>, _cfg: &ConfigView<'_, MathematicalConfig>) -> Result<Emit<MathematicalMutation, MathematicalConfigMutation>, Fault> {
        Ok(Emit::config(vec![MathematicalConfigMutation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::set_locale;
    use crate::apps::mathematical::testkit::math_app;
    use crate::apps::mathematical::MathematicalCommand;

    #[test]
    fn set_locale_writes_config_not_mutations() {
        let mut app = math_app();
        let result = app.dispatch_typed(MathematicalCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }), &semio_framework_plugin::testkit::meta("local")).expect("locale");
        assert!(result.mutations.is_empty(), "setLocale must not emit a VCS operation");
    }
}
//#endregion 🧪️Tests
