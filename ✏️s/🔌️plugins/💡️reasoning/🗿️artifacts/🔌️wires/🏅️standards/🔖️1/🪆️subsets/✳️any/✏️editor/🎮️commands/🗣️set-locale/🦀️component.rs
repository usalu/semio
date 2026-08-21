//! 🗣️ 🗣️ Wires play app commands command — `set-locale`.

use crate::artifacts::wires::op::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;
use crate::editor::wires::config::{WiresConfig, WiresConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "locale")]
pub struct SetLocale {
    pub value: String,
}

pub async fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
    Ok(Emit::config(vec![WiresConfigMutation::SetLocale { value: payload.value.clone() }]))
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
        let mut app = metabolism_app();
        dispatch(&mut app, WiresCommand::SetLocale(SetLocale { value: "de-DE".into() }));
        let json = render(&mut app, WIRES_PLAY_BODY_DOCUMENT);
        assert!(json.contains("Identitäten"));
        assert!(json.contains("Beziehungen"));
    }
}
//#endregion 🧪️Tests
