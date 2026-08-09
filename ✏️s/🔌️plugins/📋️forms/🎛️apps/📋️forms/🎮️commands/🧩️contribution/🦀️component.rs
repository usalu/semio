//! 🧩️ Forms play app commands — host-pushed plugin contributions (extension question kinds).
//!
//! Not declared as a manifest action (host-pushed, like `setLocale`) — see `crate::apps::forms`'s
//! `app_commands!` invocation for the shared `as` wire-keyword rationale.

use crate::apps::forms::config::{FormsConfig, FormsConfigMutation};
use crate::artifacts::forms::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetContributions
pub mod set_contributions {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "contributions")]
    pub struct SetContributions {
        pub json: String,
    }

    pub fn handle(payload: &SetContributions, _doc: &DocumentView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
        Ok(Emit::config(vec![FormsConfigMutation::SetContributions { json: payload.json.clone() }]))
    }
}
//#endregion 🔖️SetContributions

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::forms::testkit::{building_component_contributions, dispatch, forms_app, render};
    use crate::apps::forms::{FormsCommand, FORMS_PLAY_BODY_CATALOGUE};

    #[test]
    fn set_contributions_extends_the_catalogue_with_the_contributed_kind() {
        let mut app = forms_app();
        let before = render(&mut app, FORMS_PLAY_BODY_CATALOGUE);
        assert!(!before.contains("forms-play-catalogue.buildingComponent"), "catalogue should start without the contributed kind: {before}");
        dispatch(&mut app, FormsCommand::SetContributions(set_contributions::SetContributions { json: serde_json::to_string(&building_component_contributions()).unwrap() }));
        let after = render(&mut app, FORMS_PLAY_BODY_CATALOGUE);
        assert!(after.contains("forms-play-catalogue.buildingComponent"), "catalogue should list the contributed kind: {after}");
    }
}
//#endregion 🧪️Tests
