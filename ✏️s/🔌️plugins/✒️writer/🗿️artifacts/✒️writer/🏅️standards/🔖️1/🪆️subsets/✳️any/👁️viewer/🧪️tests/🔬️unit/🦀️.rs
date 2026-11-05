use crate::{schema, WRITER_DIALECT};
use semio_plugin::app::{ArtifactViewer, Viewer};
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use semio_plugin::NoConfig;
#[allow(unused_imports)]
use crate::viewer::writer::modes::view;
#[allow(unused_imports)]
use crate::viewer::writer::modes::view::windows::main;
#[allow(unused_imports)]
use crate::{WriterSnapshot, WRITER_DOCUMENT_SCHEMA};
#[allow(unused_imports)]
use semio_plugin::app::ViewEmit;
#[allow(unused_imports)]
use semio_framework_artifact_reference::Dialect;
#[allow(unused_imports)]
use semio_plugin::Fault;
#[allow(unused_imports)]
use semio_ui_locale::Label;
#[allow(unused_imports)]
use semio_plugin::NoConfigMutation;
#[allow(unused_imports)]
use semio_plugin::NoPresence;
#[allow(unused_imports)]
use semio_plugin::NoPresenceMutation;
#[allow(unused_imports)]
use semio_plugin::NoTransient;
#[allow(unused_imports)]
use semio_plugin::NoTransientMutation;
#[allow(unused_imports)]
use semio_framework_2d_compute::EngineHandles;
use crate::viewer::writer::*;

#[semio_framework_async_macros::async_test]
async fn create_writer_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_writer_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, WRITER_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<WriterViewer as ArtifactViewer>::DIALECT, WRITER_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    let document = schema::empty_writer_snapshot();
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let node = WriterViewer::render("writer.view.nope", &doc, &cfg, &semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native));
    assert!(semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(node.expect("viewer render")).unwrap().contains("Unknown body"));
}
