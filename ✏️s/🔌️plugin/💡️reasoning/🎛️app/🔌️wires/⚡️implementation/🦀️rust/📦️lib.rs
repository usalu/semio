//! 🧠️ Reasoning wires app — document entities (constitutional: general).

use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️Constants
pub const MINDMAP_WIRES_SCHEMA: &str = "reasoning.wires.fixture";
/// 🕸️ Mindmap's own board fixture schema — recognized by the neutral force-graph-layout crate
/// (`infinite_board_normal_undirected`) as an undirected graph, distinct from puzzle's directed
/// `puzzle.2d.fixture` board.
pub const MINDMAP_BOARD_SCHEMA: &str = "reasoning.mindmap.fixture";
//#endregion 🔖️Constants

//#region 🔖️Types
/// 🧠️ The mindmap-wires document: the semantic wires fixture (identities/relationships/kind catalogs)
/// paired with its own `reasoning.mindmap.fixture` board fixture (nodes/edges/camera). Both are kept
/// as opaque JSON so this crate stays free of any board-engine schema types, while operations still address
/// board nodes/edges and wires relationships by id for mergeable, granular edits.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "wires", layout = "lines")]
pub struct MindmapWiresDocument {
    #[dsl(key = "wires")]
    pub wires_fixture: Value,
    #[dsl(key = "board")]
    pub board_fixture: Value,
}
//#endregion 🔖️Types
