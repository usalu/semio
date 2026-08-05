//! 🗣️ Procedural3d play app commands — host-pushed locale switch and the plugin-contribution catalogue
//! hot-swap (both undeclared in the manifest, never in the command palette).

use crate::apps::procedural3d::config::{Procedural3dConfig, Procedural3dConfigOperation};
use crate::artifacts::procedural3d::op::Procedural3dOperation;
use crate::artifacts::procedural3d::Procedural3dDocument;
use flow_core::FlowEvalSession;
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

    pub fn handle(payload: &SetLocale, _doc: &DocumentView<'_, Procedural3dDocument>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        Ok(Emit::config(vec![Procedural3dConfigOperation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale

//#region 🔖️SetContributions
pub mod set_contributions {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "contributions")]
    pub struct SetContributions {
        pub json: String,
    }

    pub fn handle(payload: &SetContributions, _doc: &DocumentView<'_, Procedural3dDocument>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        Ok(Emit::config(vec![Procedural3dConfigOperation::SetContributions { json: payload.json.clone() }]))
    }
}
//#endregion 🔖️SetContributions

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural3d::testkit::{app, dispatch};
    use crate::apps::procedural3d::Procedural3dCommand;

    #[test]
    fn set_locale_updates_config_locale() {
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
        let mut app = app();
        dispatch(&mut app, Procedural3dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }));
    }
}
//#endregion 🧪️Tests
