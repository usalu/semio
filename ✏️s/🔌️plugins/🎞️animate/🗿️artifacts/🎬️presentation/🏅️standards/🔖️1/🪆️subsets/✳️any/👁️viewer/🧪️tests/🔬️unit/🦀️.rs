use crate::ANIMATE_DIALECT;
use semio_framework_plugin::ArtifactViewer;
use semio_framework_plugin::Viewer;
#[allow(unused_imports)]
use crate::viewer::animate::modes::view;
#[allow(unused_imports)]
use crate::viewer::animate::modes::view::windows::tile_editor;
#[allow(unused_imports)]
use crate::{default_presentation_snapshot, PresentationSnapshot, PRESENTATION_DOCUMENT_SCHEMA};
#[allow(unused_imports)]
use semio_framework_plugin::app::InteractionView;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactView;
#[allow(unused_imports)]
use semio_framework_plugin::ComponentTree;
#[allow(unused_imports)]
use semio_framework_plugin::ConfigView;
#[allow(unused_imports)]
use semio_framework_artifact_reference::Dialect;
#[allow(unused_imports)]
use semio_framework_plugin::Fault;
#[allow(unused_imports)]
use semio_framework_ui_locale::Label;
#[allow(unused_imports)]
use semio_framework_plugin::NoConfig;
#[allow(unused_imports)]
use semio_framework_plugin::NoConfigMutation;
#[allow(unused_imports)]
use semio_framework_plugin::NoPresence;
#[allow(unused_imports)]
use semio_framework_plugin::NoPresenceMutation;
#[allow(unused_imports)]
use semio_framework_plugin::NoTransient;
#[allow(unused_imports)]
use semio_framework_plugin::NoTransientMutation;
#[allow(unused_imports)]
use semio_framework_plugin::ViewEmit;
#[allow(unused_imports)]
use semio_framework_2d::compute::EngineHandles;
use crate::viewer::animate::*;

#[test]
fn create_animate_presentation_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_animate_presentation_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, ANIMATE_DIALECT.into());
}

#[test]
fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<AnimatePresentationViewer as ArtifactViewer>::DIALECT, ANIMATE_DIALECT);
}
