use crate::standards::v1::subsets::any::schema::ARCHITECT_DIALECT;
use semio_framework_plugin::ArtifactViewer;
use semio_framework_plugin::Viewer;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
#[allow(unused_imports)]
use crate::viewer::architect::modes::view;
#[allow(unused_imports)]
use crate::viewer::architect::modes::view::windows::register;
#[allow(unused_imports)]
use crate::{standards::v1::subsets::any::schema::snapshot::sample_plugin, standards::v1::subsets::any::schema::snapshot::ProgramSnapshot, standards::v1::subsets::any::schema::snapshot::ARCHITECT_PROGRAM_SCHEMA};
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactView;
#[allow(unused_imports)]
use semio_framework_plugin::ConfigView;
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
use semio_framework_plugin::{app::Dialect, app::InteractionView};
#[allow(unused_imports)]
use semio_framework_2d::compute::EngineHandles;
use crate::viewer::architect::*;

#[semio_framework_async_macros::async_test]
async fn create_architect_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_architect_viewer();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
    assert_eq!(def.dialect, ARCHITECT_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<ArchitectViewer as ArtifactViewer>::DIALECT, ARCHITECT_DIALECT);
}
