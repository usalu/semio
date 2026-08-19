//! 🧩️ 🧩️ Forms play app commands command — `set-contributions`.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::artifacts::forms::{op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "contributions")]
pub struct SetContributions {
    pub json: String,
}

pub async fn handle(payload: &SetContributions, _doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    Ok(Emit::config(vec![FormsConfigMutation::SetContributions { json: payload.json.clone() }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::forms::testkit::{building_component_contributions, dispatch, forms_app, render};
    use crate::editor::forms::{FormsCommand, FORMS_PLAY_BODY_CATALOGUE};

    #[test]
    async fn set_contributions_extends_the_catalogue_with_the_contributed_kind() {
        let mut app = forms_app();
        let before = render(&mut app, FORMS_PLAY_BODY_CATALOGUE);
        assert!(!before.contains("forms-play-catalogue.buildingComponent"), "catalogue should start without the contributed kind: {before}");
        dispatch(&mut app, FormsCommand::SetContributions(SetContributions { json: serde_json::to_string(&building_component_contributions()).unwrap() }));
        let after = render(&mut app, FORMS_PLAY_BODY_CATALOGUE);
        assert!(after.contains("forms-play-catalogue.buildingComponent"), "catalogue should list the contributed kind: {after}");
    }
}
//#endregion 🧪️Tests
