//! 🔄️ Sequence play app commands — auto-layout (`reorganize`) and its flow-direction setting.

use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::SequenceSnapshot;
use crate::editor::sequence::config::{SequenceConfig, SequenceConfigMutation};
use crate::editor::sequence::ops_from_host_mutation;
use infinite_board_port_directed_dag::{DagLayoutOptions, DagLayoutOrientation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Orientation
/// 🌳️ `SequenceConfig::orientation`'s string encoding <-> the DAG kernel's real
/// `DagLayoutOrientation` — see `SequenceConfig::orientation`'s doc comment for why the config field
/// itself stays a string. Single consumer (`reorganize`), so it lives here rather than the artifact
/// engine.
pub async fn orientation_from_config(value: &str) -> DagLayoutOrientation {
    match value {
        "topBottom" => DagLayoutOrientation::TopBottom,
        _ => DagLayoutOrientation::LeftRight,
    }
}
//#endregion 🔖️Orientation

//#region 🔖️Reorganize
pub mod reorganize {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "reorganize")]
    pub struct Reorganize {}

    pub async fn handle(_payload: &Reorganize, doc: &ArtifactView<'_, SequenceSnapshot>, cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        let orientation = orientation_from_config(&cfg.snapshot.orientation);
        Ok(Emit::mutations(ops_from_host_mutation(doc.snapshot, |host| {
            let opts = DagLayoutOptions { orientation, ..DagLayoutOptions::default() };
            let _ = host.reorganize(&opts);
        })))
    }
}
//#endregion 🔖️Reorganize

//#region 🔖️SetOrientation
pub mod set_orientation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "set-orientation")]
    pub struct SetOrientation {
        pub value: String,
    }

    pub async fn handle(payload: &SetOrientation, _doc: &ArtifactView<'_, SequenceSnapshot>, _cfg: &ConfigView<'_, SequenceConfig>) -> Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault> {
        Ok(Emit::config(vec![SequenceConfigMutation::SetOrientation { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetOrientation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::editor::sequence::testkit::{dispatch, new_app};
    use crate::editor::sequence::SequenceCommand;

    use super::reorganize::Reorganize;
    use super::set_orientation::SetOrientation;
    use move_step_helper::move_all_steps_to_origin;

    mod move_step_helper {
        use crate::editor::sequence::commands::step::move_step::MoveStep;
        use crate::editor::sequence::testkit::dispatch;
        use crate::editor::sequence::SequenceCommand;
        use semio_framework_plugin::{EditorApp, VcsArtifactApp};

        pub async fn move_all_steps_to_origin(app: &mut VcsArtifactApp<EditorApp<crate::editor::sequence::SequencePlayApp>>) {
            let ids: Vec<String> = app.snapshot().expect("projection").to_fixture().steps.iter().map(|step| step.id.clone()).collect();
            for id in &ids {
                dispatch(app, SequenceCommand::MoveStep(MoveStep { node_id: id.clone(), x: 0.0, y: 0.0 }));
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn set_orientation_command_changes_reorganize_layout_axis() {
        let mut app = new_app();
        dispatch(&mut app, SequenceCommand::SetOrientation(SetOrientation { value: "topBottom".into() }));
        move_all_steps_to_origin(&mut app);
        dispatch(&mut app, SequenceCommand::Reorganize(Reorganize {}));
        let ys: Vec<f64> = app.snapshot().expect("projection").to_fixture().steps.iter().map(|step| step.y).collect();
        assert!(ys.iter().any(|y| *y != 0.0), "topBottom orientation should spread steps vertically, got {ys:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn reorganize_command_spreads_step_positions_apart() {
        let mut app = new_app();
        move_all_steps_to_origin(&mut app);
        dispatch(&mut app, SequenceCommand::Reorganize(Reorganize {}));
        let xs: Vec<f64> = app.snapshot().expect("projection").to_fixture().steps.iter().map(|step| step.x).collect();
        assert!(xs.iter().any(|x| *x != 0.0), "reorganize should spread steps apart, got {xs:?}");
    }
}
//#endregion 🧪️Tests
