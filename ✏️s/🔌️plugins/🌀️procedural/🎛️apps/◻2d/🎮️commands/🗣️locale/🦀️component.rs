//! 🗣️ Procedural2d play app commands — host-pushed locale switch (undeclared in the manifest, never
//! in the command palette).

use crate::apps::procedural2d::config::{Procedural2dConfig, Procedural2dConfigOperation};
use crate::artifacts::procedural2d::op::Procedural2dOperation;
use crate::artifacts::procedural2d::Procedural2dDocument;
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

    pub fn handle(payload: &SetLocale, _doc: &DocumentView<'_, Procedural2dDocument>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dOperation, Procedural2dConfigOperation>, Fault> {
        Ok(Emit::config(vec![Procedural2dConfigOperation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural2d::testkit::{app, dispatch};
    use crate::apps::procedural2d::Procedural2dCommand;

    #[test]
    fn set_locale_updates_config_locale() {
        let mut app = app();
        dispatch(&mut app, Procedural2dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }));
    }
}
//#endregion 🧪️Tests
