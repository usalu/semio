//! 🔄️ 🔄️ Wires play app commands command — `force-layout`.

use crate::apps::wires::config::{WiresConfig, WiresConfigMutation};
use crate::artifacts::wires::schema::{fixture_nodes, force_layout_board, node_position};
use crate::artifacts::wires::op::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

/// 🕸️ Re-lays out the board and diffs the moved nodes into `move-node` operations — shared by both
/// `ForceLayout` and `Reorganize`.
fn force_layout_operations(document: &WiresSnapshot) -> Vec<WiresMutation> {
    let mut board = crate::artifacts::wires::wires_working_board(document);
    force_layout_board(&mut board);
    fixture_nodes(&board)
        .iter()
        .filter_map(|node| {
            let id = node.get("id").and_then(|value| value.as_str())?;
            let (nx, ny) = node_position(node);
            let (ox, oy) = crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node(document, id).map_or((nx, ny), |node| node_position(&node));
            if nx == ox && ny == oy {
                return None;
            }
            Some(crate::artifacts::wires::mutations::move_node(id.to_string(), nx, ny))
        })
        .collect()
}

//#region 🔖️ForceLayout
//#endregion 🔖️ForceLayout

//#region 🔖️Reorganize
//#endregion 🔖️Reorganize

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "force-layout")]
pub struct ForceLayout {}

pub fn handle(_payload: &ForceLayout, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
    Ok(Emit::mutations(force_layout_operations(doc.snapshot)))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::wires::testkit::{dispatch, metabolism_app};
    use crate::apps::wires::WiresCommand;

    #[test]
    fn force_layout_action_repositions_metabolism_nodes() {
        let mut app = metabolism_app();
        let before: Vec<(f64, f64)> = fixture_nodes(&crate::artifacts::wires::wires_working_board(&app.snapshot().expect("snapshot"))).iter().map(node_position).collect();
        dispatch(&mut app, WiresCommand::ForceLayout(ForceLayout {}));
        let after: Vec<(f64, f64)> = fixture_nodes(&crate::artifacts::wires::wires_working_board(&app.snapshot().expect("snapshot"))).iter().map(node_position).collect();
        assert_eq!(before.len(), after.len());
        assert_ne!(before, after, "force layout should move at least one node");
    }

    #[test]
    fn reorganize_repositions_metabolism_nodes() {
        let mut app = metabolism_app();
        let before: Vec<(f64, f64)> = fixture_nodes(&crate::artifacts::wires::wires_working_board(&app.snapshot().expect("snapshot"))).iter().map(node_position).collect();
        dispatch(&mut app, WiresCommand::Reorganize(reorganize::Reorganize {}));
        let after: Vec<(f64, f64)> = fixture_nodes(&crate::artifacts::wires::wires_working_board(&app.snapshot().expect("snapshot"))).iter().map(node_position).collect();
        assert_eq!(before.len(), after.len());
        assert_ne!(before, after, "reorganize should move at least one node");
    }
}
//#endregion 🧪️Tests
