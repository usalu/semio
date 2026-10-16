"""🎬️ Slice D follow-up, scene contract: `Board2dScene.placement_naming` in the plane producer, its TS twin and the lane fixture."""
from importlib import util

spec = util.spec_from_file_location("apply_board", __file__.rsplit("/", 1)[0] + "/apply-board.py")
board = util.module_from_spec(spec)
spec.loader.exec_module(board)
S = board.R + "🧰️framework/🔨️modules/🖱️ui/🎬️scene/"

FIELD_DOC = '''    /// 🏷️ The owning domain's brush placement id prefixes in the board's wire form,
    /// `{"brushNodePrefix":…,"brushEdgePrefix":…}`, so a host the domain cannot configure directly (the os wgpu surface)
    /// mints the same ids as the domain's own boards. `None` leaves the host's installed naming.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement_naming: Option<String>,
'''

NAMING_JSON = '"{\\"brushNodePrefix\\":\\"probe.node.\\",\\"brushEdgePrefix\\":\\"probe.edge.\\"}"'

EDITS = {
    S + "🎬️scenes/🖼️plane/🦀️.rs": [
        ("    #[serde(default, skip_serializing_if = \"Option::is_none\")]\n    pub domain_id: Option<String>,\n",
         "    #[serde(default, skip_serializing_if = \"Option::is_none\")]\n    pub domain_id: Option<String>,\n" + FIELD_DOC),
        ("    domain_id: Option<String>,\n    suggestion_menu_json: Option<String>,\n    tool_run_trace: Option<String>,\n    highlighted_ids_json: String,\n",
         "    domain_id: Option<String>,\n    placement_naming: Option<String>,\n    suggestion_menu_json: Option<String>,\n    tool_run_trace: Option<String>,\n    highlighted_ids_json: String,\n"),
        ("            lod_mode: board2d_default_lod_mode(),\n            domain_id: None,\n",
         "            lod_mode: board2d_default_lod_mode(),\n            domain_id: None,\n            placement_naming: None,\n"),
        ('        value_push_option(&mut entries, "domainId", &self.domain_id);\n        value_push_option(&mut entries, "suggestionMenuJson", &self.suggestion_menu_json);\n        value_push_option(&mut entries, "toolRunTrace", &self.tool_run_trace);\n        value_push_if_nonempty(&mut entries, "lanes", &self.lanes);\n',
         '        value_push_option(&mut entries, "domainId", &self.domain_id);\n        value_push_option(&mut entries, "placementNaming", &self.placement_naming);\n        value_push_option(&mut entries, "suggestionMenuJson", &self.suggestion_menu_json);\n        value_push_option(&mut entries, "toolRunTrace", &self.tool_run_trace);\n        value_push_if_nonempty(&mut entries, "lanes", &self.lanes);\n'),
        ('            domain_id: value_decode_option(&entries, "domainId")?,\n            suggestion_menu_json: value_decode_option(&entries, "suggestionMenuJson")?,\n            tool_run_trace: value_decode_option(&entries, "toolRunTrace")?,\n            lanes: value_decode_default(&entries, "lanes", Vec::new)?,\n        })\n    }\n}\n//#endregion 🔖️Board2dScene',
         '            domain_id: value_decode_option(&entries, "domainId")?,\n            placement_naming: value_decode_option(&entries, "placementNaming")?,\n            suggestion_menu_json: value_decode_option(&entries, "suggestionMenuJson")?,\n            tool_run_trace: value_decode_option(&entries, "toolRunTrace")?,\n            lanes: value_decode_default(&entries, "lanes", Vec::new)?,\n        })\n    }\n}\n//#endregion 🔖️Board2dScene'),
    ],
    S + "🟦️.ts": [
        ("  /** 🎯️ The framework interaction domain this board's picks and hovers belong to; absent means the app declares none and the host publishes no `interactionHover`. */\n  readonly domainId?: string;\n  /** 💡️ The one-shot handle-suggestions popup",
         "  /** 🎯️ The framework interaction domain this board's picks and hovers belong to; absent means the app declares none and the host publishes no `interactionHover`. */\n  readonly domainId?: string;\n"
         "  /** 🏷️ The owning domain's brush placement id prefixes in the board's wire form, `{\"brushNodePrefix\":string,\"brushEdgePrefix\":string}`, so every host mints the domain's ids; absent leaves the host's installed naming. */\n"
         "  readonly placementNaming?: string;\n  /** 💡️ The one-shot handle-suggestions popup"),
    ],
    S + "🧪️tests/🔬️plane-unit/🦀️.rs": [
        ("    let mut probe = Board2dScene::base(\"{}\".into(), \"{}\".into(), true);\n    probe.hovered_id = Some(String::new());\n    probe.active_utility = Some(String::new());\n    probe.tool_run_trace = Some(String::new());\n",
         "    let mut probe = Board2dScene::base(\"{}\".into(), \"{}\".into(), true);\n    probe.hovered_id = Some(String::new());\n    probe.active_utility = Some(String::new());\n    probe.placement_naming = Some(String::new());\n    probe.tool_run_trace = Some(String::new());\n"),
    ],
    S + "🧪️tests/🧫️fixtures/🚚️board2d-scene-lanes/🔣️.json": [
        ('    "areaBrushSize",\n    "lanes"\n  ],', '    "areaBrushSize",\n    "placementNaming",\n    "lanes"\n  ],'),
        ('      "lodMode": "automatic",\n      "toolRunTrace": "', '      "lodMode": "automatic",\n      "placementNaming": ' + NAMING_JSON + ',\n      "toolRunTrace": "'),
        ('      "lodMode": "automatic",\n      "lanes": [', '      "lodMode": "automatic",\n      "placementNaming": ' + NAMING_JSON + ',\n      "lanes": ['),
    ],
}

if __name__ == "__main__":
    board.apply(EDITS)
