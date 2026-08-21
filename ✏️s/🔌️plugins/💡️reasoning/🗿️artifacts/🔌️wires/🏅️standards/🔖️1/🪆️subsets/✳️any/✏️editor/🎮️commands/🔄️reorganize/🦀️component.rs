//! 🔄️ 🔄️ Wires play app commands command — `reorganize`.

use crate::artifacts::wires::op::WiresMutation;
use crate::artifacts::wires::schema::{fixture_nodes, force_layout_board, node_position};
use crate::artifacts::wires::WiresSnapshot;
use crate::editor::wires::config::{WiresConfig, WiresConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

/// 🕸️ Re-lays out the board and diffs the moved nodes into `move-node` operations — shared by both
/// `ForceLayout` and `Reorganize`.
async fn force_layout_operations(document: &WiresSnapshot) -> Vec<WiresMutation> {
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
#[dsl(keyword = "reorganize")]
pub struct Reorganize {}

pub async fn handle(_payload: &Reorganize, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
    Ok(Emit::mutations(force_layout_operations(doc.snapshot)))
}
