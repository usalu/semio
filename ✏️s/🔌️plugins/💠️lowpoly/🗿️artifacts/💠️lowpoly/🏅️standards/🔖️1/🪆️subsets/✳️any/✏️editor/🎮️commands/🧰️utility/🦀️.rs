//! 🧰️ Lowpoly play app commands — the active transform/paint utility switch (`setActiveUtility`, which
//! also clears mid-gesture scratch so switching tools never leaves a stale drag behind) and per-utility
//! parameter writes (`setUtilityParam`). Config-only.

use crate::artifacts::lowpoly::op::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolySnapshot;
use crate::editor::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::editor::lowpoly::session::LowpolyScratch;
use crate::editor::lowpoly::view::{is_paint_utility, utility_params_value};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetUtilityParam
pub mod set_utility_param {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "set-utility-param")]
    pub struct SetUtilityParam {
        pub key: String,
        pub value_json: String,
    }

    /// 🌉️ `utility_params_value` (owned outside this ticket slice, `🧭️view/🦀️.rs`) still hands back a
    /// real `serde_json::Value` — bridged into a `DslValue` immediately via the unconditional
    /// `🌱️value/🦀️.rs` conversion, so the merge itself never touches `serde_json`.
    pub fn handle(payload: &SetUtilityParam, _doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let params = utility_params_value(cfg.snapshot);
        let mut entries: Vec<(String, dsl::DslValue)> = dsl::DslValue::from(&params).as_object().map(|entries| entries.to_vec()).unwrap_or_default();
        let value = dsl::json::from_json_str::<dsl::DslValue>(&payload.value_json).unwrap_or(dsl::DslValue::Null);
        match entries.iter_mut().find(|(key, _)| key == &payload.key) {
            Some(entry) => entry.1 = value,
            None => entries.push((payload.key.clone(), value)),
        }
        let json = dsl::json::to_json_string(&dsl::DslValue::object(entries));
        Ok(Emit::config(vec![LowpolyConfigMutation::SetUtilityParams { json }]))
    }
}
//#endregion 🔖️SetUtilityParam

//#region 🔖️SetActiveUtility
pub mod set_active_utility {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "set-active-utility")]
    pub struct SetActiveUtility {
        pub utility_id: String,
    }

    pub fn handle(payload: &SetActiveUtility, _doc: &ArtifactView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        ctx.reset_gestures();
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: hover used to be cleared here
        // (`SetHoveredTarget`/`SetHoveredObject`) — it is framework-owned ephemeral state now, cleared by
        // the surface's own `interactionHover{targets:[]}` on pointer-leave, never app-side.
        let mut config_mutations = vec![LowpolyConfigMutation::SetActiveUtility { utility_id: payload.utility_id.clone() }];
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
    use crate::editor::lowpoly::testkit::{app, dispatch};
    use crate::editor::lowpoly::LowpolyCommand;
    use semio_framework_plugin::{testkit, PluginApp};

    #[semio_framework_async_macros::async_test]
    async fn active_utility_switch_emits_no_ops_and_no_history() {
        // 🧰️ Selecting a host-owned utility must never create an undoable edit.
        let mut a = app().await;
        let result = dispatch(&mut a, LowpolyCommand::SetActiveUtility(super::set_active_utility::SetActiveUtility { utility_id: "rotate".into() })).await;
        assert!(result.mutations.is_empty(), "utility switch must emit no operations");
        let before = a.snapshot().expect("projection");
        a.handle_action("undo", None, &testkit::meta("a")).await.unwrap();
        assert_eq!(a.snapshot().expect("projection"), before, "utility switch left nothing to undo");
    }
}
//#endregion 🧪️Tests
