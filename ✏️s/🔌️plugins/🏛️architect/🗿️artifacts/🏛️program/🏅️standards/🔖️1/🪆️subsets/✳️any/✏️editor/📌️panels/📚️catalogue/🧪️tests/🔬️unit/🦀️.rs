use super::*;

#[semio_framework_async_macros::async_test]
async fn the_tab_is_the_framework_catalogue_tab_bound_to_this_apps_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key.as_deref(), Some(ARCHITECT_BODY_CATALOGUE));
    assert!(matches!(definition.group, PanelGroup::Workbench));
}

#[semio_framework_async_macros::async_test]
async fn every_register_id_gets_a_catalogue_row() {
    let json = crate::editor::architect::unit_tests::context::project_render(render());
    for register in REGISTER_IDS {
        assert!(json.contains(&format!("architect-catalogue.register.{register}")), "missing catalogue row for {register}");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_action_shortcuts_are_present() {
    let json = crate::editor::architect::unit_tests::context::project_render(render());
    for id in ["architect-catalogue.validate", "architect-catalogue.analysis", "architect-catalogue.report", "architect-catalogue.search"] {
        assert!(json.contains(id), "missing shortcut {id}");
    }
}
