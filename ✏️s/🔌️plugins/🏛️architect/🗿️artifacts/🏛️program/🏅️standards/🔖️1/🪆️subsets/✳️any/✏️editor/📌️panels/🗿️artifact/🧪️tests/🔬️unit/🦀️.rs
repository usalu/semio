
use super::*;
use crate::{empty_plugin, sample_plugin};

#[semio_framework_async_macros::async_test]
async fn the_tab_is_the_framework_document_tab_bound_to_this_apps_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key.as_deref(), Some(ARCHITECT_BODY_DOCUMENT));
    assert!(matches!(definition.group, PanelGroup::Workbench));
}

#[semio_framework_async_macros::async_test]
async fn the_tree_lists_program_meta_and_the_elements() {
    let program = sample_plugin();
    let json = crate::editor::architect::testkit::project_render(render(&program, &ArchitectConfig::default()));
    assert!(json.contains("Sample Clinic"));
    assert!(json.contains(&program.elements[0].header.id.to_string()));
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the document panel binds the
/// "program" interaction domain — the framework auto-injects/stamps selection over its bare
/// element rows (see `render`'s own doc comment).
#[semio_framework_async_macros::async_test]
async fn the_tree_binds_the_program_interaction_domain() {
    let json = crate::editor::architect::testkit::project_render(render(&sample_plugin(), &ArchitectConfig::default()));
    assert!(json.contains("\"interactionDomain\":\"program\""));
}

#[semio_framework_async_macros::async_test]
async fn an_empty_program_renders_the_none_placeholder_row() {
    let json = crate::editor::architect::testkit::project_render(render(&empty_plugin(), &ArchitectConfig::default()));
    assert!(json.contains("architect-document.elements.empty"));
}
