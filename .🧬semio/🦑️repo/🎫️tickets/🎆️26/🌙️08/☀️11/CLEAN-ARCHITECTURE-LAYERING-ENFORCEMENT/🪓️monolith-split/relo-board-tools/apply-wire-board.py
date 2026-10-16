"""📮️ Slice D follow-up, board half: the owned wire form of brush placement naming on BoardHost."""
from importlib import util

spec = util.spec_from_file_location("apply_board", __file__.rsplit("/", 1)[0] + "/apply-board.py")
board = util.module_from_spec(spec)
spec.loader.exec_module(board)
B = board.B

WIRE_BLOCK = '''
/// 📮️ The wire form of a domain's brush placement prefixes, JSON `{"brushNodePrefix":…,"brushEdgePrefix":…}`: what a scene
/// carries (`Board2dScene::placement_naming`) to hosts no domain can hand a `&'static` [`BoardNaming`], such as the os renderer's
/// wgpu `Board2d` surface. Installed with [`BoardHost::set_placement_naming_json`], it overrides the installed naming's brush prefixes.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BoardPlacementNaming {
    pub brush_node_prefix: String,
    pub brush_edge_prefix: String,
}

impl BoardPlacementNaming {
    /// 🧾️ Parses the scene wire form; both prefixes must be non-empty.
    pub fn from_json(json: &str) -> Result<Self, NormalPortError> {
        let naming: Self = serde_json::from_str(json).map_err(NormalPortError::PlacementNamingJson)?;
        if naming.brush_node_prefix.is_empty() || naming.brush_edge_prefix.is_empty() {
            return Err(NormalPortError::PlacementNamingEmpty);
        }
        Ok(naming)
    }

    /// 🖨️ The scene wire form of these prefixes.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl BoardNaming {
    /// 📤️ This naming's brush prefixes in wire form, for a domain to publish on its scene.
    pub fn placement_naming(&self) -> BoardPlacementNaming {
        BoardPlacementNaming { brush_node_prefix: self.brush_node_prefix.into(), brush_edge_prefix: self.brush_edge_prefix.into() }
    }
}
'''

PROBE_HELPER = '''
#[cfg(test)]
fn probe_brush_placement_events(host: &mut BoardHost) -> String {
    host.set_size(800, 600, 1.0);
    host.set_active_utility("brush");
    host.set_suggestion_offset(40.0);
    host.set_brush_node_size(40.0);
    let catalogs = serde_json::json!({
        "handleKinds": [{ "id": "port", "name": "Port", "color": "#888" }],
        "nodeKinds": [{ "id": "brush.kind", "name": "Brush Kind", "handles": [{ "handleKind": "port", "angle": 3.141592653589793 }] }]
    });
    host.set_board_kind_catalogs_from_json(&catalogs.to_string()).expect("probe catalogs");
    let scene = SceneDescriptor {
        nodes: vec![NodeDescriptor { id: "a".into(), x: 0.0, y: 0.0, draggable: Some(true), selected: None, style: None, text: None, icon_kind: None, node_kind: Some("a.kind".into()), user_data: None, visible: None, locked: None, root: None, shape: Some("circle".into()), radius: Some(40.0), width: None, height: None, scale: None }].into(),
        handles: vec![HandleDescriptor { id: "a:h0".into(), node_id: "a".into(), angle: 0.0, radius: None, scale: None, selected: None, visible: None, locked: None, style: None, handle_kind: Some("port".into()), color: None, icon_kind: None, user_data: None }].into(),
        edges: vec![].into(),
        wires: vec![].into(),
        regions: vec![],
        selection_exit_highlight_ids: vec![],
    };
    host.sync_descriptor(&scene).expect("probe scene syncs");
    let handle = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
    let slot = host.world_to_screen(handle + (handle - Point::new(0.0, 0.0)));
    host.pointer_move_screen(slot.x, slot.y, false, false, false);
    let _ = host.drain_events_json();
    host.pointer_leave_screen(true);
    host.drain_events_json()
}

#[cfg(test)]
/// 📮️ A scene's wire naming reaches a host no domain configured: a bare host mints the scene's brush prefixes, the wire form
/// round-trips a static naming byte for byte, malformed forms are refused without touching the prefixes in force, clearing
/// falls back to the installed naming, and installing a static naming drops an earlier wire override.
#[test]
fn scene_wire_placement_naming_overrides_brush_prefixes() {
    static WIRE_PROBE: BoardNaming = BoardNaming {
        brush_node_prefix: "wire.node.",
        brush_edge_prefix: "wire.edge.",
        fill_node_prefix: "wire.fill.",
        fill_edge_prefix: "wire.fill.edge.",
        fill_stage_label: BoardFillStage::id,
        fill_stale_operation_fault: "wire-stale",
    };
    let json = WIRE_PROBE.placement_naming().to_json();
    assert_eq!(json, r#"{"brushNodePrefix":"wire.node.","brushEdgePrefix":"wire.edge."}"#);
    assert_eq!(BoardPlacementNaming::from_json(&json).expect("wire form parses"), WIRE_PROBE.placement_naming());
    let mut host = BoardHost::new();
    host.set_placement_naming_json(Some(&json)).expect("wire naming installs");
    assert_eq!(host.brush_placement_naming(), WIRE_PROBE.placement_naming());
    let events = probe_brush_placement_events(&mut host);
    assert!(events.contains("\\"wire.node.1\\"") && events.contains("\\"wire.edge.1\\""), "the wire prefixes mint the placement: {events}");
    assert!(!events.contains("board.brush."), "no neutral id leaks past a wire naming: {events}");
    assert!(host.set_placement_naming_json(Some(r#"{"brushNodePrefix":"","brushEdgePrefix":"x."}"#)).is_err());
    assert!(host.set_placement_naming_json(Some(r#"{"brushNodePrefix":"a.","brushEdgePrefix":"b.","extra":1}"#)).is_err());
    assert!(host.set_placement_naming_json(Some("[]")).is_err());
    assert_eq!(host.brush_placement_naming(), WIRE_PROBE.placement_naming(), "a refused wire form keeps the prefixes in force");
    host.set_placement_naming_json(None).expect("wire naming clears");
    assert_eq!(host.brush_placement_naming(), NEUTRAL_BOARD_NAMING.placement_naming());
    host.set_placement_naming_json(Some(&json)).expect("wire naming installs again");
    host.set_naming(&NEUTRAL_BOARD_NAMING);
    assert_eq!(host.brush_placement_naming(), NEUTRAL_BOARD_NAMING.placement_naming(), "a static naming replaces an earlier wire override");
}
'''

OLD_PLACE = '''    let place = |naming: Option<&'static BoardNaming>| {
        let mut host = BoardHost::new();
        if let Some(naming) = naming {
            host.set_naming(naming);
        }
'''

EDITS = {
    B + "🦀️.rs": [
        ("    InvalidHandleColor(String, String),\n    EventCredits,\n}",
         "    InvalidHandleColor(String, String),\n    EventCredits,\n    PlacementNamingJson(serde_json::Error),\n    PlacementNamingEmpty,\n}"),
        ('            Self::EventCredits => formatter.write_str("board event credits exhausted before descriptor publication"),\n',
         '            Self::EventCredits => formatter.write_str("board event credits exhausted before descriptor publication"),\n'
         '            Self::PlacementNamingJson(error) => write!(formatter, "setPlacementNamingJson: {error}"),\n'
         '            Self::PlacementNamingEmpty => formatter.write_str("setPlacementNamingJson: brushNodePrefix and brushEdgePrefix must be non-empty"),\n'),
        ("            Self::ExternalLinkPreviewJson(error) | Self::BrushSessionJson(error) | Self::DropPreviewJson(error) => Some(error),",
         "            Self::ExternalLinkPreviewJson(error) | Self::BrushSessionJson(error) | Self::DropPreviewJson(error) | Self::PlacementNamingJson(error) => Some(error),"),
        ('    fill_stale_operation_fault: "stale-board-fill-operation",\n};\n',
         '    fill_stale_operation_fault: "stale-board-fill-operation",\n};\n' + WIRE_BLOCK),
        ("    naming: &'static BoardNaming,\n    brush_node_kind_weights",
         "    naming: &'static BoardNaming,\n    /// 📨️ Brush prefixes received as scene data ([`BoardHost::set_placement_naming_json`]); they override `naming`'s brush prefixes.\n    placement_naming: Option<BoardPlacementNaming>,\n    brush_node_kind_weights"),
    ],
    B + "🎨️style/🦀️.rs": [
        ("            naming: &NEUTRAL_BOARD_NAMING,\n", "            naming: &NEUTRAL_BOARD_NAMING,\n            placement_naming: None,\n"),
        ("    pub fn set_naming(&mut self, naming: &'static BoardNaming) {\n        self.naming = naming;\n    }\n",
         "    pub fn set_naming(&mut self, naming: &'static BoardNaming) {\n        self.naming = naming;\n        self.placement_naming = None;\n    }\n"),
        ("    pub fn naming(&self) -> &'static BoardNaming {\n        self.naming\n    }\n",
         "    pub fn naming(&self) -> &'static BoardNaming {\n        self.naming\n    }\n\n"
         "    /// 📮️ Installs (`Some`) or clears (`None`) the brush prefixes a scene carries in wire form ([`BoardPlacementNaming`]);\n"
         "    /// brush placements minted afterwards use them instead of the installed naming's. A refused form changes nothing.\n"
         "    pub fn set_placement_naming_json(&mut self, json: Option<&str>) -> Result<(), NormalPortError> {\n"
         "        self.placement_naming = json.map(BoardPlacementNaming::from_json).transpose()?;\n"
         "        Ok(())\n    }\n\n"
         "    /// 🔍️ The brush prefixes this host mints with, the wire override first, in wire form.\n"
         "    pub fn brush_placement_naming(&self) -> BoardPlacementNaming {\n"
         "        BoardPlacementNaming { brush_node_prefix: self.brush_node_prefix().into(), brush_edge_prefix: self.brush_edge_prefix().into() }\n    }\n\n"
         "    /// 🔤️ The node id prefix the next brush placement is minted with.\n"
         "    pub(super) fn brush_node_prefix(&self) -> &str {\n"
         "        self.placement_naming.as_ref().map_or(self.naming.brush_node_prefix, |naming| naming.brush_node_prefix.as_str())\n    }\n\n"
         "    /// 🔡️ The edge id prefix the next brush placement is minted with.\n"
         "    pub(super) fn brush_edge_prefix(&self) -> &str {\n"
         "        self.placement_naming.as_ref().map_or(self.naming.brush_edge_prefix, |naming| naming.brush_edge_prefix.as_str())\n    }\n"),
        ('        let node_id = format!("{}{serial}", self.naming.brush_node_prefix);\n        let edge_id = format!("{}{serial}", self.naming.brush_edge_prefix);\n',
         '        let node_id = format!("{}{serial}", self.brush_node_prefix());\n        let edge_id = format!("{}{serial}", self.brush_edge_prefix());\n'),
        ("                    self.push_close_string(left);\n                    self.push_close_string(right);\n                } else if let Some((values, mode)) = self.last_select_emit_sig.as_mut() {",
         "                    self.push_close_string(left);\n                    self.push_close_string(right);\n                } else if let Some(naming) = self.placement_naming.take() {\n"
         "                    self.push_close_string(naming.brush_node_prefix);\n                    self.push_close_string(naming.brush_edge_prefix);\n"
         "                } else if let Some((values, mode)) = self.last_select_emit_sig.as_mut() {"),
        ("            && self.close_node_handles.is_none()\n", "            && self.close_node_handles.is_none()\n            && self.placement_naming.is_none()\n"),
    ],
    B + "🧹️scene-delete-links/🦀️.rs": [
        ('        let node_id = format!("{}{serial}", self.naming.brush_node_prefix);\n        let edge_id = format!("{}{serial}", self.naming.brush_edge_prefix);\n',
         '        let node_id = format!("{}{serial}", self.brush_node_prefix());\n        let edge_id = format!("{}{serial}", self.brush_edge_prefix());\n'),
    ],
}


def apply_tests():
    path = B + "🧪️tests/🔬️board-host-standalone/🦀️.rs"
    text = open(path, encoding="utf-8").read()
    start = text.index(OLD_PLACE)
    end = text.index("        let events = host.drain_events_json();\n", start) + len("        let events = host.drain_events_json();\n")
    assert text.count(OLD_PLACE) == 1 and end - start < 4000
    text = text[:start] + OLD_PLACE + "        let events = probe_brush_placement_events(&mut host);\n" + text[end:]
    text = text.rstrip("\n") + "\n" + PROBE_HELPER
    open(path, "w", encoding="utf-8").write(text)
    print("edited", path.replace(board.R, ""))


if __name__ == "__main__":
    board.apply(EDITS)
    apply_tests()
