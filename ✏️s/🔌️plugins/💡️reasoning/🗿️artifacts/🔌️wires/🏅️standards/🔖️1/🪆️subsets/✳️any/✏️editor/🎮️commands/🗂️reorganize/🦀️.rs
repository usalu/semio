//! 🔄️ 🔄️ Wires play app commands command — `reorganize`.

use crate::op::WiresMutation;
use crate::schema::{fixture_nodes, force_layout_board, node_position};
use crate::WiresSnapshot;
use crate::editor::wires::config::{WiresConfig, WiresConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🕸️ Re-lays out the board and diffs the moved nodes into `move-node` operations — shared by both
/// `ForceLayout` and `Reorganize`.
fn force_layout_operations(document: &WiresSnapshot) -> Vec<WiresMutation> {
    let mut board = crate::wires_working_board(document);
    force_layout_board(&mut board);
    fixture_nodes(&board)
        .iter()
        .filter_map(|node| {
            let id = node.get("id").and_then(|value| value.as_str())?;
            let (nx, ny) = node_position(node);
            let (ox, oy) = crate::standards::v1::subsets::any::schema::inferences::find_board_node(document, id).map_or((nx, ny), |node| node_position(&node));
            if nx == ox && ny == oy {
                return None;
            }
            Some(crate::mutations::move_node(id.to_string(), nx, ny))
        })
        .collect()
}

//#region 🔖️ForceLayout
//#endregion 🔖️ForceLayout

//#region 🔖️Reorganize
//#endregion 🔖️Reorganize

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "reorganize")]
pub struct Reorganize {}

pub fn handle(_payload: &Reorganize, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
    Ok(Emit::mutations(force_layout_operations(doc.snapshot)))
}
