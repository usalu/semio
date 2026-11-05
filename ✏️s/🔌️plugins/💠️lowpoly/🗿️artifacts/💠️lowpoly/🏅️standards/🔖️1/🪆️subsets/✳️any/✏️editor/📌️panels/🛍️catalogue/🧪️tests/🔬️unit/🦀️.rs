extern crate semio_os_kernel as protocol;
#[path = "../../../../🧪️tests/🧰️support/🦀️.rs"]
mod context;
use crate::context::{app, render};

#[semio_framework_async_macros::async_test]
async fn catalogue_lists_primitives() {
    let mut a = app().await;
    let json = render(&mut a, semio_s_artifact_lowpoly_lowpoly_panels::catalogue::LOWPOLY_PLAY_BODY_CATALOGUE).await;
    assert!(json.contains("lowpoly-play-catalogue.box"));
    assert!(json.contains("Cube"));
    assert!(json.contains("Ico Sphere"));
}

fn collect_action_scopes(value: &serde_json::Value, action: &str, scopes: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(fields) => {
            if value["action"]["name"] == action { scopes.push(value["action"]["scope"].as_str().unwrap().into()); }
            for child in fields.values() { collect_action_scopes(child, action, scopes); }
        }
        serde_json::Value::Array(values) => for child in values { collect_action_scopes(child, action, scopes); },
        _ => {}
    }
}

#[semio_framework_async_macros::async_test]
async fn panel_and_window_actions_address_the_registered_editor() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/controller.json")).unwrap();
    let definition = semio_s_artifact_lowpoly_lowpoly_editor::create_lowpoly_app();
    assert_eq!(definition.id, fixture["controller"].as_str().unwrap());
    assert_eq!(definition.controller_id, definition.id);
    let window_action = semio_s_artifact_lowpoly_lowpoly_kit::lowpoly_window_action(fixture["windowAction"].as_str().unwrap(), None);
    assert_eq!(window_action.controller_id, definition.controller_id);
    let mut a = app().await;
    let json: serde_json::Value = serde_json::from_str(&render(&mut a, semio_s_artifact_lowpoly_lowpoly_panels::catalogue::LOWPOLY_PLAY_BODY_CATALOGUE).await).unwrap();
    let mut scopes = Vec::new();
    collect_action_scopes(&json, fixture["action"].as_str().unwrap(), &mut scopes);
    assert_eq!(scopes.len(), fixture["items"].as_u64().unwrap() as usize);
    assert!(scopes.iter().all(|scope| scope == &definition.controller_id));
}
