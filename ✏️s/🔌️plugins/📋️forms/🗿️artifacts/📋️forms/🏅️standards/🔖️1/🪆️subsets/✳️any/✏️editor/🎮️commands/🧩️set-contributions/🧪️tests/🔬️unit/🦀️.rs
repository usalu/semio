
use super::*;
use crate::editor::forms::testkit::{building_component_contributions, dispatch, forms_app, render};
use crate::editor::forms::{FORMS_PLAY_BODY_CATALOGUE, FormsCommand};

#[semio_framework_async_macros::async_test]
async fn set_contributions_extends_the_catalogue_with_the_contributed_kind() {
    let mut app = forms_app().await;
    let before = render(&mut app, FORMS_PLAY_BODY_CATALOGUE).await;
    assert!(!before.contains("forms-play-catalogue.buildingComponent"), "catalogue should start without the contributed kind: {before}");
    dispatch(&mut app, FormsCommand::SetContributions(SetContributions { json: dsl::os_pack::json::to_json_string(&building_component_contributions()) })).await;
    let after = render(&mut app, FORMS_PLAY_BODY_CATALOGUE).await;
    assert!(after.contains("forms-play-catalogue.buildingComponent"), "catalogue should list the contributed kind: {after}");
}
