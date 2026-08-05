//! 🗣️ Mathematical play app commands — the host-pushed locale change.
//!
//! Not declared as a manifest action (locale is host-pushed, not a user-facing app action needing a
//! palette entry), which is why its wire keyword stays the bare `"locale"` rather than the kebab-cased
//! `"set-locale"` its command id would suggest — see the `as` literal in `crate::apps::mathematical`'s
//! `app_commands!` invocation.

use crate::apps::mathematical::config::{MathConfig, MathConfigOperation};
use crate::artifacts::mathematical::op::MathOperation;
use crate::artifacts::mathematical::MathProjection;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "locale")]
    pub struct SetLocale {
        pub value: String,
    }

    pub fn handle(payload: &SetLocale, _doc: &DocumentView<'_, MathProjection>, _cfg: &ConfigView<'_, MathConfig>) -> Result<Emit<MathOperation, MathConfigOperation>, Fault> {
        Ok(Emit::config(vec![MathConfigOperation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::set_locale;
    use crate::apps::mathematical::testkit::math_app;
    use crate::apps::mathematical::MathCommand;

    #[test]
    fn set_locale_writes_config_not_document_operations() {
        let mut app = math_app();
        let result = app.dispatch_typed(MathCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }), &semio_framework_plugin::testkit::meta("local")).expect("locale");
        assert!(result.operations.is_empty(), "setLocale must not emit a VCS operation");
    }
}
//#endregion 🧪️Tests
