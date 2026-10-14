extern crate semio_framework_os_kernel as protocol;
#[path = "../../../../🧪️tests/🧰️support/🦀️.rs"]
mod context;
use semio_s_artifact_lowpoly_lowpoly_commands::patch_object::*;
use semio_s_artifact_lowpoly_lowpoly_config::{LowpolyConfig, LowpolyConfigMutation};
use semio_s_artifact_lowpoly_lowpoly_view::editor::lowpoly::session::LowpolyScratch;
use semio_s_artifact_lowpoly_lowpoly_mut_objects::change_object_smooth_shading::ChangeObjectSmoothShading;
use semio_s_artifact_lowpoly_lowpoly_mut_objects::rename_object::RenameObject;
use semio_s_artifact_lowpoly_lowpoly_mutations::LowpolyMutation;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};
use crate::context::{app, dispatch};
use semio_s_artifact_lowpoly_lowpoly_editor::LowpolyCommand;

#[semio_framework_async_macros::async_test]
async fn patch_object_name_emits_operation() {
    let mut a = app().await;
    let object_id = a.snapshot().expect("projection").objects[0].id.clone();
    dispatch(&mut a, LowpolyCommand::PatchObject(PatchObject { object_id, field: "name".into(), value_json: Some(serde_json::to_string("Renamed").unwrap()) })).await;
    assert_eq!(a.snapshot().expect("projection").objects[0].name, "Renamed");
}
