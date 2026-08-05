//! 🗣️ Fem2d play app commands — the BCP-47 locale tag. Config-only, host-pushed (no manifest action
//! declaration — see `create_fem2d_app`'s doc for the `setLocale`/`locale` row).

use crate::apps::fem2d::config::{Fem2dConfig, Fem2dConfigOperation};
use crate::artifacts::fem2d::op::Fem2dOperation;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

type Fem2dDocument = crate::artifacts::fem2d::Fem2dDocument;

//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "locale")]
    pub struct SetLocale {
        pub value: String,
    }

    pub fn handle(payload: &SetLocale, _doc: &DocumentView<'_, Fem2dDocument>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dOperation, Fem2dConfigOperation>, Fault> {
        Ok(Emit::config(vec![Fem2dConfigOperation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::fem2d::testkit::{dispatch, fem2d_app};
    use crate::apps::fem2d::Fem2dCommand;

    #[test]
    fn set_locale_action_writes_config_not_document_operations() {
        let mut app = fem2d_app();
        let before = app.projection().expect("projection").clone();
        let result = dispatch(&mut app, Fem2dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }));
        assert!(result.operations.is_empty());
        assert_eq!(app.projection().expect("projection"), &before);
    }
}
//#endregion 🧪️Tests
