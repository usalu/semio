use crate::modes::edit::windows::inputs::*;
use crate::unit_tests::context;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_INPUTS);
    assert_eq!(definition().id, WINDOW_INPUTS);
}

#[semio_framework_async_macros::async_test]
async fn renders_the_typed_inspector() {
    let mut app = context::app_with_registry().await;
    assert!(context::render(&mut app, BODY_INPUTS).await.contains(':'), "the inputs body renders the typed inspector");
    context::close(&mut app);
}

#[test]
fn immutable_product_identity_is_visible_and_has_no_commit_control_in_either_locale() {
    use semio_ui_locale::Locale;
    use semio_s_plugin_norm_app_surface::{field_controls, field_view, UnitSystem};
    let path = "catalog.products[id=RAD-600].id";
    let value = semio_framework_value::DslValue::String("RAD-600".into());
    for locale in [Locale::En, Locale::De] {
        let view = field_view(path, &value, &value, locale, Some(crate::field_meta::vdi3805_field_meta), UnitSystem::Engineering);
        assert!(view.read_only);
        let mut controls = field_controls(&view, &value, "norm-inputs.catalog.products[id=RAD-600].id", "s.norm.vdi3805@1/*#editor", locale).expect("identity value");
        assert_eq!(controls.len(), 1);
        let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(controls.remove(0))).expect("identity projection");
        assert!(json.contains("RAD-600"), "{json}");
        assert!(json.contains(if locale == Locale::En { "Read only" } else { "Schreibgeschützt" }), "{json}");
        assert!(!json.contains("setField") && !json.contains(".input"), "{json}");
    }
}
