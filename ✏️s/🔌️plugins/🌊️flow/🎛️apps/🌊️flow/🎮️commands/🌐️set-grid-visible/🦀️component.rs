//! 🌐️ 🔳️ Flow play app commands command — `set-grid-visible`.

use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct SetGridVisible {
    pub pressed: Option<bool>,
}

pub fn handle(payload: &SetGridVisible, _doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    Ok(Emit::config(vec![FlowConfigMutation::SetGridVisible { value: payload.pressed.unwrap_or(!cfg.snapshot.grid_visible) }]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::flow::testkit::{dispatch, flow_app, main_window_measures, FlowApp};
    use crate::apps::flow::FlowCommand;
    use semio_framework_plugin::WindowMeasure;

    fn grid_children(app: &mut FlowApp) -> Vec<WindowMeasure> {
        main_window_measures(app)
            .into_iter()
            .find_map(|measure| match measure {
                WindowMeasure::Group { id, children, .. } if id == "flow-play-measures.grid" => Some(children),
                _ => None,
            })
            .expect("grid measure group")
    }

    fn grid_visible(app: &mut FlowApp) -> bool {
        grid_children(app).iter().any(|child| matches!(child, WindowMeasure::Toggle { id, pressed, .. } if id == "flow-play-measures.grid-visible" && *pressed))
    }

    fn grid_factor(app: &mut FlowApp) -> f64 {
        grid_children(app)
            .iter()
            .find_map(|child| match child {
                WindowMeasure::Slider { id, value, .. } if id == "flow-play-measures.grid-factor" => Some(*value),
                _ => None,
            })
            .expect("grid factor slider")
    }

    /// 🔁️ `pressed: None` must flip the live config value, not force it to a constant.
    #[test]
    fn a_bare_toggle_flips_the_current_value() {
        let mut app = flow_app();
        assert!(grid_visible(&mut app), "grid starts visible");
        dispatch(&mut app, FlowCommand::SetGridVisible(SetGridVisible { pressed: None }));
        assert!(!grid_visible(&mut app), "a bare toggle flips it off");
        dispatch(&mut app, FlowCommand::SetGridVisible(SetGridVisible { pressed: None }));
        assert!(grid_visible(&mut app), "and back on");
    }

    #[test]
    fn grid_factor_clamps_to_the_slider_range() {
        let mut app = flow_app();
        dispatch(&mut app, FlowCommand::SetGridFactor(set_grid_factor::SetGridFactor { value: 1000.0 }));
        assert_eq!(grid_factor(&mut app), 50.0);
        dispatch(&mut app, FlowCommand::SetGridFactor(set_grid_factor::SetGridFactor { value: 0.0 }));
        assert_eq!(grid_factor(&mut app), 0.5);
    }
}
//#endregion 🧪️Tests
