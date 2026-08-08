//! 🧰️ Lowpoly play app commands — the active transform/paint utility switch (`setActiveUtility`, which
//! also clears mid-gesture scratch so switching tools never leaves a stale drag behind) and per-utility
//! parameter writes (`setUtilityParam`). Config-only.

use crate::apps::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::apps::lowpoly::session::LowpolyScratch;
use crate::apps::lowpoly::view::{is_paint_utility, utility_params_value};
use crate::artifacts::lowpoly::op::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolyProjection;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

//#region 🔖️SetUtilityParam
pub mod set_utility_param {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-utility-param")]
    pub struct SetUtilityParam {
        pub key: String,
        pub value_json: String,
    }

    pub fn handle(payload: &SetUtilityParam, _doc: &DocumentView<'_, LowpolyProjection>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let mut params = utility_params_value(cfg.projection);
        let value: Value = serde_json::from_str(&payload.value_json).unwrap_or(Value::Null);
        if let Some(map) = params.as_object_mut() {
            map.insert(payload.key.clone(), value);
        } else {
            let mut map = Map::new();
            map.insert(payload.key.clone(), value);
            params = Value::Object(map);
        }
        Ok(Emit::config(vec![LowpolyConfigMutation::SetUtilityParams { json: params.to_string() }]))
    }
}
//#endregion 🔖️SetUtilityParam

//#region 🔖️SetActiveUtility
pub mod set_active_utility {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-active-utility")]
    pub struct SetActiveUtility {
        pub utility_id: String,
    }

    pub fn handle(payload: &SetActiveUtility, _doc: &DocumentView<'_, LowpolyProjection>, _cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        ctx.reset_gestures();
        let mut config_mutations = vec![
            LowpolyConfigMutation::SetActiveUtility { utility_id: payload.utility_id.clone() },
            LowpolyConfigMutation::SetHoveredTarget { object_id: None, mode: None, id: None },
            LowpolyConfigMutation::SetHoveredObject { object_id: None },
        ];
        if is_paint_utility(&payload.utility_id) {
            config_mutations.push(LowpolyConfigMutation::SetPaintUtility { value: payload.utility_id.clone() });
        }
        Ok(Emit::config(config_mutations))
    }
}
//#endregion 🔖️SetActiveUtility

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::apps::lowpoly::testkit::{app, dispatch};
    use crate::apps::lowpoly::LowpolyCommand;
    use semio_framework_plugin::{testkit, PluginApp};

    #[test]
    fn active_utility_switch_emits_no_ops_and_no_history() {
        // 🧰️ Selecting a host-owned utility must never create an undoable edit.
        let mut a = app();
        let result = dispatch(&mut a, LowpolyCommand::SetActiveUtility(super::set_active_utility::SetActiveUtility { utility_id: "rotate".into() }));
        assert!(result.mutations.is_empty(), "utility switch must emit no operations");
        let before = a.projection().expect("projection");
        a.handle_action("undo", None, &testkit::meta("a")).unwrap();
        assert_eq!(a.projection().expect("projection"), before, "utility switch left nothing to undo");
    }
}
//#endregion 🧪️Tests
