//! 🏷️ Existing localized property controls remain adjustable after canonical spatial-holder property commands.
use super::{demo, inferred, text, Locale};

#[semio_framework_async_macros::async_test]
async fn spatial_occurrence_properties_keep_existing_adjustable_inputs_and_remove_add_actions_in_both_locales() {
    let (mut snapshot, _) = demo();
    let ids = [snapshot.sites.keys().next().expect("site").clone(), snapshot.buildings.keys().next().expect("building").clone(), snapshot.storeys.keys().next().expect("storey").clone()];
    for id in ids {
        let command: semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation =
            semio_framework_pack_json::from_json_str(&format!(r#"{{"mutation":"setElementProperty","id":"{id}","pset":"Authored","property":"Text","value":{{"Text":{{"value":"Büro"}}}}}}"#), semio_framework_pack_json::JsonMemberPolicy::Reject)
                .expect("canonical typed property command");
        snapshot = semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::apply_model_mutation(&snapshot, &command).expect("canonical edit");
        for locale in [Locale::En, Locale::De] {
            let rendered = text(&snapshot, &inferred(&snapshot), &[id.as_str()], &[], locale);
            for expected in ["bim-properties.property.0", "bim-properties.property.0.remove", "bim-properties.property.add", "Authored · Text", "Büro"] {
                assert!(rendered.contains(expected), "{expected}: {rendered}");
            }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn declared_ifc_width_names_keep_bounded_control_keys_and_exact_source_names() {
    let (snapshot, _) = demo();
    let id = snapshot.storeys.keys().next().expect("storey").clone();
    let set = "S".repeat(255);
    let property = "P".repeat(255);
    let command: semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation = semio_framework_pack_json::from_json_str(
        &format!(r#"{{"mutation":"setElementProperty","id":"{id}","pset":"{set}","property":"{property}","value":{{"Text":{{"value":"exact source"}}}}}}"#),
        semio_framework_pack_json::JsonMemberPolicy::Reject,
    )
    .expect("canonical property command");
    let next = semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::apply_model_mutation(&snapshot, &command).expect("canonical edit");
    for locale in [Locale::En, Locale::De] {
        let rendered = text(&next, &inferred(&next), &[id.as_str()], &[], locale);
        for expected in ["bim-properties.property.0", "bim-properties.property.0.remove", set.as_str(), property.as_str(), "exact source"] {
            assert!(rendered.contains(expected), "{expected}: {rendered}");
        }
        assert!(rendered.contains("bim-properties.effective.0"));
    }
}

#[test]
fn captions_fit_utf8_without_changing_action_text_resource_policy() {
    for value in ["S".repeat(255), "🏗️".repeat(255)] {
        let caption = crate::editor::bim::panels::properties::property_caption(&value);
        assert!(caption.len() <= 160);
        assert!(std::str::from_utf8(caption.as_bytes()).is_ok());
    }
    assert!(semio_s_artifact_bim_model_kit::editor::bim::kit::ui_value_text("🏗".repeat(255)).is_err());
}
