//! 🗣️ 🗣️ Wires play app commands command — `set-locale`.

use crate::op::WiresMutation;
use crate::WiresSnapshot;
use crate::editor::wires::config::{WiresConfig, WiresConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "locale")]
pub struct SetLocale {
    pub value: String,
}

pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
    Ok(Emit::config(vec![WiresConfigMutation::SetLocale(crate::editor::wires::config::SetLocale { value: payload.value.clone() })]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::wires::testkit::{dispatch, metabolism_app, render};
    use crate::editor::wires::{WiresCommand, WIRES_PLAY_BODY_DOCUMENT};

    /// 🗣️ B1: locale is now `cfg.locale`, set via the typed `SetLocale` config command — no more
    /// `ViewModel.locale` threaded through `render` (the trait dropped `ViewModel` entirely).
    #[semio_framework_async_macros::async_test]
    async fn wires_labels_resolve_native_in_german() {
        let mut app = metabolism_app().await;
        dispatch(&mut app, WiresCommand::SetLocale(SetLocale { value: "de-DE".into() })).await;
        let json = render(&mut app, WIRES_PLAY_BODY_DOCUMENT).await;
        assert!(json.contains("Identitäten"));
        assert!(json.contains("Beziehungen"));
    }
}
//#endregion 🧪️Tests
