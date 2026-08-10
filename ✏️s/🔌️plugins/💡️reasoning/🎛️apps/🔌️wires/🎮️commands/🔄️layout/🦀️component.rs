//! 🔄️ Wires play app commands — force-directed board re-layout (`forceLayout`/`reorganize` share the
//! exact same effect, mirroring the old `WiresCommand::ForceLayout | WiresCommand::Reorganize` match arm).

use crate::apps::wires::config::{WiresConfig, WiresConfigMutation};
use crate::artifacts::wires::engine::{fixture_nodes, force_layout_board, node_position};
use crate::artifacts::wires::op::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// 🕸️ Re-lays out the board and diffs the moved nodes into `PatchNode` operations — shared by both
/// `ForceLayout` and `Reorganize`.
fn force_layout_operations(document: &WiresSnapshot) -> Vec<WiresMutation> {
    let mut board = document.board_fixture.clone();
    force_layout_board(&mut board);
    fixture_nodes(&board)
        .iter()
        .filter_map(|node| {
            let id = node.get("id").and_then(|value| value.as_str())?;
            let (nx, ny) = node_position(node);
            let (ox, oy) = crate::artifacts::wires::engine::find_board_node(document, id).map_or((nx, ny), node_position);
            if nx == ox && ny == oy {
                return None;
            }
            let mut patch = BTreeMap::new();
            patch.insert("x".into(), dsl::to_dsl_value(&nx).unwrap_or(DslValue::Null));
            patch.insert("y".into(), dsl::to_dsl_value(&ny).unwrap_or(DslValue::Null));
            Some(WiresMutation::PatchNode { node_id: id.to_string(), patch })
        })
        .collect()
}

//#region 🔖️ForceLayout
pub mod force_layout {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "force-layout")]
    pub struct ForceLayout {}

    pub fn handle(_payload: &ForceLayout, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
        Ok(Emit::mutations(force_layout_operations(doc.snapshot)))
    }
}
//#endregion 🔖️ForceLayout

//#region 🔖️Reorganize
pub mod reorganize {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "reorganize")]
    pub struct Reorganize {}

    pub fn handle(_payload: &Reorganize, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
        Ok(Emit::mutations(force_layout_operations(doc.snapshot)))
    }
}
//#endregion 🔖️Reorganize

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::wires::testkit::{dispatch, metabolism_app};
    use crate::apps::wires::WiresCommand;

    #[test]
    fn force_layout_action_repositions_metabolism_nodes() {
        let mut app = metabolism_app();
        let before: Vec<(f64, f64)> = fixture_nodes(&app.snapshot().expect("snapshot").board_fixture).iter().map(node_position).collect();
        dispatch(&mut app, WiresCommand::ForceLayout(force_layout::ForceLayout {}));
        let after: Vec<(f64, f64)> = fixture_nodes(&app.snapshot().expect("snapshot").board_fixture).iter().map(node_position).collect();
        assert_eq!(before.len(), after.len());
        assert_ne!(before, after, "force layout should move at least one node");
    }

    #[test]
    fn reorganize_repositions_metabolism_nodes() {
        let mut app = metabolism_app();
        let before: Vec<(f64, f64)> = fixture_nodes(&app.snapshot().expect("snapshot").board_fixture).iter().map(node_position).collect();
        dispatch(&mut app, WiresCommand::Reorganize(reorganize::Reorganize {}));
        let after: Vec<(f64, f64)> = fixture_nodes(&app.snapshot().expect("snapshot").board_fixture).iter().map(node_position).collect();
        assert_eq!(before.len(), after.len());
        assert_ne!(before, after, "reorganize should move at least one node");
    }
}
//#endregion 🧪️Tests
