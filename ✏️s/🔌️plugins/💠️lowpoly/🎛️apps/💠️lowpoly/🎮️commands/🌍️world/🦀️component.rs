//! 🌍️ Lowpoly play app commands — world-scene selection/hover picking (`worldSelect`/`worldHover`/
//! `setHover`/`worldPick`). All config-only.

use crate::apps::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::apps::lowpoly::session::LowpolyScratch;
use crate::apps::lowpoly::view::{apply_component_selection, selection_keys_for};
use crate::artifacts::lowpoly::op::LowpolyMutation;
use crate::artifacts::lowpoly::LowpolyProjection;
use semio_framework_plugin::{merge_world_selection_ids, ConfigView, DocumentView, Emit, Fault, SelectionSet};
use serde::{Deserialize, Serialize};

//#region 🔖️WorldSelect
pub mod world_select {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "world-select")]
    pub struct WorldSelect {
        pub ids: Vec<String>,
        pub merge: String,
    }

    pub fn handle(payload: &WorldSelect, _doc: &DocumentView<'_, LowpolyProjection>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let config = cfg.projection;
        let current = SelectionSet::from_ids(config.selected_object_ids.clone());
        let merged = merge_world_selection_ids(&current, &payload.ids, &payload.merge).to_vec();
        let mut config_mutations = vec![LowpolyConfigMutation::SetSelectedObjectIds { ids: merged.clone() }];
        if let Some(first) = merged.first() {
            config_mutations.push(LowpolyConfigMutation::SetActiveObject { object_id: first.clone() });
        }
        Ok(Emit::config(config_mutations))
    }
}
//#endregion 🔖️WorldSelect

//#region 🔖️WorldHover
pub mod world_hover {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "world-hover")]
    pub struct WorldHover {
        pub object_id: Option<String>,
    }

    pub fn handle(payload: &WorldHover, _doc: &DocumentView<'_, LowpolyProjection>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let target = payload.object_id.as_ref().map(|id| (id.clone(), "mesh".to_string(), 0u32));
        Ok(Emit::config(vec![
            LowpolyConfigMutation::SetHoveredObject { object_id: payload.object_id.clone() },
            LowpolyConfigMutation::SetHoveredTarget { object_id: target.as_ref().map(|(id, _, _)| id.clone()), mode: target.as_ref().map(|(_, mode, _)| mode.clone()), id: target.as_ref().map(|(_, _, id)| *id) },
        ]))
    }
}
//#endregion 🔖️WorldHover

//#region 🔖️SetHover
pub mod set_hover {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-hover")]
    pub struct SetHover {
        pub object_id: Option<String>,
        pub mode: Option<String>,
        pub id: Option<u32>,
    }

    pub fn handle(payload: &SetHover, _doc: &DocumentView<'_, LowpolyProjection>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(Emit::config(vec![
            LowpolyConfigMutation::SetHoveredObject { object_id: payload.object_id.clone() },
            LowpolyConfigMutation::SetHoveredTarget { object_id: payload.object_id.clone(), mode: payload.mode.clone(), id: payload.id },
        ]))
    }
}
//#endregion 🔖️SetHover

//#region 🔖️WorldPick
pub mod world_pick {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "world-pick")]
    pub struct WorldPick {
        pub granularity: String,
        pub merge: String,
        pub id: Option<u32>,
    }

    pub fn handle(payload: &WorldPick, doc: &DocumentView<'_, LowpolyProjection>, cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let (projection, config) = (doc.projection, cfg.projection);
        match payload.id {
            None => {
                if payload.merge == "replace" {
                    let keys = selection_keys_for(projection, config, &config.selection_mode, &[]);
                    Ok(Emit::config(vec![LowpolyConfigMutation::SetSelection { mode: config.selection_mode.clone(), ids: Vec::new() }, LowpolyConfigMutation::SetSelectionKeys { keys }]))
                } else {
                    Ok(Emit::default())
                }
            }
            Some(id) => {
                let (mode, ids, keys, targets) = apply_component_selection(config, projection, &payload.granularity, &[id], &payload.merge);
                Ok(Emit::config(vec![
                    LowpolyConfigMutation::SetSelectionTargets { mesh: targets.mesh, vertex: targets.vertex, edge: targets.edge, face: targets.face },
                    LowpolyConfigMutation::SetSelection { mode, ids },
                    LowpolyConfigMutation::SetSelectionKeys { keys },
                ]))
            }
        }
    }
}
//#endregion 🔖️WorldPick

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::apps::lowpoly::testkit::{app, dispatch};
    use crate::apps::lowpoly::LowpolyCommand;

    #[test]
    fn world_pick_is_view_state_and_emits_no_operations() {
        let mut a = app();
        let result = dispatch(&mut a, LowpolyCommand::WorldPick(super::world_pick::WorldPick { granularity: "face".into(), merge: "replace".into(), id: Some(0) }));
        assert!(result.mutations.is_empty(), "picking must not create an undoable operation");
    }
}
//#endregion 🧪️Tests
