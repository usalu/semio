//! 🖱️ 🖱️ Wires play app commands command — `canvas-pointer-move`.

use crate::apps::wires::config::{WiresConfig, WiresConfigMutation};
use crate::artifacts::wires::schema::{fixture_camera, node_position};
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::op::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "pointer-move")]
pub struct CanvasPointerMove {
    pub x: f64,
    pub y: f64,
}

pub fn handle(payload: &CanvasPointerMove, doc: &ArtifactView<'_, WiresSnapshot>, cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let Some(drag_node_id) = config.drag_node_id.clone() else { return Ok(Emit::default()) };
    let Some(node) = find_board_node(document, &drag_node_id) else { return Ok(Emit::default()) };
    let zoom = fixture_camera(&crate::artifacts::wires::wires_working_board(document)).2.max(1e-6);
    let (cur_x, cur_y) = node_position(&node);
    let (dx, dy) = ((payload.x - config.drag_last_x) / zoom, (payload.y - config.drag_last_y) / zoom);
    Ok(Emit {
        artifact_mutations: vec![crate::artifacts::wires::mutations::move_node(drag_node_id.clone(), cur_x + dx, cur_y + dy)],
        config_mutations: vec![WiresConfigMutation::SetDrag { node_id: Some(drag_node_id.clone()), last_x: payload.x, last_y: payload.y }],
        coalesce_key: Some(format!("drag:{drag_node_id}")),
        ..Default::default()
    })
}
