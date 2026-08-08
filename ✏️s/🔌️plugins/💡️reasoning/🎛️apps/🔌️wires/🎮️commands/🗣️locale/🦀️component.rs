//! 🗣️ Wires play app commands — the host-pushed locale change.

use crate::apps::wires::config::{WiresConfig, WiresConfigMutation};
use crate::artifacts::wires::op::MindmapWiresMutation;
use crate::artifacts::wires::MindmapWiresDocument;
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

    pub fn handle(payload: &SetLocale, _doc: &DocumentView<'_, MindmapWiresDocument>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<MindmapWiresMutation, WiresConfigMutation>, Fault> {
        Ok(Emit::config(vec![WiresConfigMutation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::wires::testkit::{dispatch, metabolism_app, render};
    use crate::apps::wires::{WiresCommand, WIRES_PLAY_BODY_DOCUMENT};

    /// 🗣️ B1: locale is now `cfg.locale`, set via the typed `SetLocale` config command — no more
    /// `ViewModel.locale` threaded through `render` (the trait dropped `ViewModel` entirely).
    #[test]
    fn wires_labels_resolve_native_in_german() {
        let mut app = metabolism_app();
        dispatch(&mut app, WiresCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }));
        let json = render(&mut app, WIRES_PLAY_BODY_DOCUMENT);
        assert!(json.contains("Identitäten"));
        assert!(json.contains("Beziehungen"));
    }
}
//#endregion 🧪️Tests
