"""W2-A follow-up item 3: the additive `PresencePeer.history_edit` presence field (flag bit 13) and the trailing
`AppFrame::Ephemeral.history_edit` body, applied compile-atomically across every Rust and TypeScript site.

Run from the repo root: `python3 .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️w2-a-presence-history-edit.py`.
Every replacement asserts its anchor count first and nothing is written unless every anchor matched.
"""
import json
import pathlib
import re

WIRE = "🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs"
WIRE_TS = "🧰️framework/🔨️modules/📡️replication/🟦️.ts"
WIRE_SCHEMA = "🧰️framework/🔨️modules/📡️replication/🧬️schema/🔣️.json"
WIRE_FIXTURE = "🧰️framework/🔨️modules/📡️replication/🧫️fixtures/👥️presence-peer-codec-v1/🔣️.json"
WIRE_TESTS = "🧰️framework/🔨️modules/📡️replication/📡️wire/🧪️tests/🔬️presence-codec/🦀️.rs"
KERNEL = "🧰️framework/🔨️modules/🎠️kernel/🦀️.rs"
CHANNEL = "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs"
CHANNEL_TESTS = "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs"
OS_TS = "🧰️framework/🛍️products/💻️os/🟦️.ts"
OS_TS_TESTS = "🧰️framework/🛍️products/💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts"
PLUGIN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
PLUGIN_CONTRACT = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"
PLUGIN_DISPATCH = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs"
BRIDGE = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs"
SHELL = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"
PEER_LITERAL_FILES = [
    WIRE_TESTS,
    "🧰️framework/🔨️modules/📡️replication/👕️peer-overlay/🧪️tests/🔬️unit/🦀️.rs",
    PLUGIN_CONTRACT,
    PLUGIN,
    "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️backbone-parity/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs",
    SHELL,
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-identity-directory-presence/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/👕️canvas-presence/🧪️tests/🔬️wgpu-unit/🦀️.rs",
]

files: dict[str, str] = {}


def text(path: str) -> str:
    if path not in files:
        files[path] = pathlib.Path(path).read_text()
    return files[path]


def rep(path: str, old: str, new: str, count: int = 1) -> None:
    source = text(path)
    found = source.count(old)
    assert found == count, (path, found, old[:160])
    files[path] = source.replace(old, new)


def matching_brace(source: str, open_index: int) -> int:
    depth = 0
    index = open_index
    in_string = False
    while index < len(source):
        char = source[index]
        if in_string:
            if char == "\\":
                index += 2
                continue
            if char == '"':
                in_string = False
        elif char == '"':
            in_string = True
        elif char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return index
        index += 1
    raise AssertionError("unbalanced literal")


def top_level_segments(body: str) -> list[tuple[int, int]]:
    segments = []
    depth = 0
    start = 0
    in_string = False
    index = 0
    while index < len(body):
        char = body[index]
        if in_string:
            if char == "\\":
                index += 2
                continue
            if char == '"':
                in_string = False
        elif char == '"':
            in_string = True
        elif char in "([{":
            depth += 1
        elif char in ")]}":
            depth -= 1
        elif char == "," and depth == 0:
            segments.append((start, index))
            start = index + 1
        index += 1
    segments.append((start, len(body)))
    return segments


def add_history_edit_to_peer_literals(path: str) -> int:
    source = text(path)
    edits = []
    for match in re.finditer(r"PresencePeer \{", source):
        before = source[source.rfind("\n", 0, match.start()) + 1 : match.start()]
        if "struct " in before or "impl " in before or re.search(r"->\s*(?:[A-Za-z_]+::)*$", before):
            continue
        open_index = match.end() - 1
        close_index = matching_brace(source, open_index)
        body = source[open_index + 1 : close_index]
        segments = [body[start:end] for start, end in top_level_segments(body)]
        stripped = [segment.strip() for segment in segments]
        if any(segment.startswith("..") for segment in stripped) or any(segment.startswith("history_edit") for segment in stripped):
            continue
        spans = top_level_segments(body)
        position = next((index for index, segment in enumerate(stripped) if re.match(r"active_tool\b", segment)), None)
        assert position is not None, (path, source[match.start() : close_index + 1][:200])
        start, end = spans[position]
        segment = body[start:end]
        trailing = segment[len(segment.rstrip()) :]
        insert_at = open_index + 1 + start + len(segment.rstrip())
        indent = re.search(r"\n([ \t]*)\S", segment)
        separator = f",\n{indent.group(1)}" if indent else ", "
        edits.append((insert_at, f"{separator}history_edit: None"))
    for insert_at, addition in sorted(edits, reverse=True):
        source = source[:insert_at] + addition + source[insert_at:]
    files[path] = source
    return len(edits)


# --- replication wire (Rust) -------------------------------------------------------------------------------------
rep(WIRE, '''    /// 🛠️ Active editor tool/utility id (ARTIFACT scope). Distinct from `tool_run`.
    pub active_tool: Option<String>,
}
''', '''    /// 🛠️ Active editor tool/utility id (ARTIFACT scope). Distinct from `tool_run`.
    pub active_tool: Option<String>,
    /// ⏪️ Summary of this peer's open history edit (ARTIFACT scope): who edits history, on which mutation, at which stage.
    pub history_edit: Option<PresenceHistoryEdit>,
}
''')
rep(WIRE, '''/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above. `presence_pack` mirrors''', '''/// ⏪️ Stage of a peer's open history edit, spelled like `HistoryTimeTravelStage` on the history wire. The binary tag
/// is the declaration order and the JSON value is the camelCase wire name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresenceHistoryEditStage {
    Editing,
    Replaying,
    Reviewing,
    Choosing,
    Finalizing,
}

impl PresenceHistoryEditStage {
    pub const ALL: [PresenceHistoryEditStage; 5] = [Self::Editing, Self::Replaying, Self::Reviewing, Self::Choosing, Self::Finalizing];

    /// 🔤️ The camelCase wire spelling shared with `HistoryTimeTravelStage`.
    pub fn wire_name(self) -> &'static str {
        match self {
            Self::Editing => "editing",
            Self::Replaying => "replaying",
            Self::Reviewing => "reviewing",
            Self::Choosing => "choosing",
            Self::Finalizing => "finalizing",
        }
    }

    /// 🔡️ Inverse of [`Self::wire_name`].
    pub fn from_wire_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|stage| stage.wire_name() == name)
    }
}

/// 👥️ Ephemeral shared summary of a peer's open history edit ("Alice is editing history: Set count to 2"): the
/// replica-independent id of the mutation whose inputs it drafts — every replica labels it from its own history rows,
/// in its own locale — the session stage and how many drafts it accepted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PresenceHistoryEdit {
    pub mutation_id: String,
    pub stage: PresenceHistoryEditStage,
    pub drafts: u32,
}

impl crate::value::ToValue for PresenceHistoryEdit {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![
            ("mutationId".to_string(), crate::value::ToValue::to_value(&self.mutation_id)),
            ("stage".to_string(), crate::value::DslValue::String(self.stage.wire_name().to_string())),
            ("drafts".to_string(), crate::value::ToValue::to_value(&self.drafts)),
        ])
    }
}

impl crate::value::FromValue for PresenceHistoryEdit {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for PresenceHistoryEdit, found {value:?}")));
        };
        let mut mutation_id = None;
        let mut stage = None;
        let mut drafts = None;
        for (key, entry) in fields {
            match key.as_str() {
                "mutationId" => mutation_id = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("mutationId"))?),
                "stage" => {
                    let name = <String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("stage"))?;
                    stage = Some(PresenceHistoryEditStage::from_wire_name(&name).ok_or_else(|| crate::value::ValueError::new(format!("unknown history edit stage {name}")).under("stage"))?);
                }
                "drafts" => drafts = Some(<u32 as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("drafts"))?),
                _ => {}
            }
        }
        Ok(PresenceHistoryEdit {
            mutation_id: mutation_id.ok_or_else(|| crate::value::ValueError::new("PresenceHistoryEdit missing mutationId"))?,
            stage: stage.ok_or_else(|| crate::value::ValueError::new("PresenceHistoryEdit missing stage"))?,
            drafts: drafts.ok_or_else(|| crate::value::ValueError::new("PresenceHistoryEdit missing drafts"))?,
        })
    }
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above. `presence_pack` mirrors''')
rep(WIRE, '''        if let Some(active_tool) = &self.active_tool {
            entries.push(("activeTool".to_string(), crate::value::ToValue::to_value(active_tool)));
        }
        crate::value::DslValue::object(entries)''', '''        if let Some(active_tool) = &self.active_tool {
            entries.push(("activeTool".to_string(), crate::value::ToValue::to_value(active_tool)));
        }
        if let Some(history_edit) = &self.history_edit {
            entries.push(("historyEdit".to_string(), crate::value::ToValue::to_value(history_edit)));
        }
        crate::value::DslValue::object(entries)''')
rep(WIRE, '''        let mut principal_kind = None;
        let mut active_tool = None;
        for (key, entry) in fields {''', '''        let mut principal_kind = None;
        let mut active_tool = None;
        let mut history_edit = None;
        for (key, entry) in fields {''')
rep(WIRE, '''                "activeTool" => active_tool = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("activeTool"))?,
                _ => {}''', '''                "activeTool" => active_tool = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("activeTool"))?,
                "historyEdit" => history_edit = <Option<PresenceHistoryEdit> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("historyEdit"))?,
                _ => {}''')
rep(WIRE, '''            principal_kind,
            active_tool,
        })
    }
}
''', '''            principal_kind,
            active_tool,
            history_edit,
        })
    }
}
''')
rep(WIRE, '''/// Bit 11 carries `principal_kind` as one declaration-order tag byte (`0` human, `1` agent); bit 12 carries `active_tool`; a peer''', '''/// Bit 11 carries `principal_kind` as one declaration-order tag byte (`0` human, `1` agent); bit 12 carries `active_tool`;
/// bit 13 carries `history_edit` as `mutation_id str | stage u8 | drafts varint`; a peer''')
rep(WIRE, '''    if peer.active_tool.is_some() {
        flags |= 1 << 12;
    }
''', '''    if peer.active_tool.is_some() {
        flags |= 1 << 12;
    }
    if peer.history_edit.is_some() {
        flags |= 1 << 13;
    }
''')
rep(WIRE, '''    if let Some(active_tool) = &peer.active_tool {
        crate::write_str(&mut out, active_tool);
    }
    out
}
''', '''    if let Some(active_tool) = &peer.active_tool {
        crate::write_str(&mut out, active_tool);
    }
    if let Some(history_edit) = &peer.history_edit {
        encode_presence_history_edit(history_edit, &mut out);
    }
    out
}
''')
rep(WIRE, '''/// 🛡️ Fixed hostile-input ceilings shared with the TypeScript presence decoder.''', '''/// ⏪️ Appends one `PresenceHistoryEdit` body — the exact bytes a `PresencePeer` carries under flag bit 13, and what a
/// guest's `AppFrame::Ephemeral.history_edit` publishes.
pub fn encode_presence_history_edit(history_edit: &PresenceHistoryEdit, out: &mut Vec<u8>) {
    crate::write_str(out, &history_edit.mutation_id);
    out.push(history_edit.stage as u8);
    crate::wire::write_varint_u64(out, u64::from(history_edit.drafts));
}

/// 🛡️ Fixed hostile-input ceilings shared with the TypeScript presence decoder.''')
rep(WIRE, '''    fn ui(&mut self) -> Result<PresenceUi, crate::ProtocolError> {
        Ok(PresenceUi {''', '''    fn history_edit(&mut self) -> Result<PresenceHistoryEdit, crate::ProtocolError> {
        let mutation_id = self.text("presence history edit mutation")?;
        let tag = self.byte("presence history edit stage")?;
        let stage = *PresenceHistoryEditStage::ALL.get(usize::from(tag)).ok_or_else(|| self.malformed("presence history edit stage", format!("unknown tag {tag:#x}")))?;
        let drafts = u32::try_from(self.varint("presence history edit drafts")?).map_err(|_| crate::ProtocolError::LimitExceeded("presence history edit drafts"))?;
        Ok(PresenceHistoryEdit { mutation_id, stage, drafts })
    }

    fn ui(&mut self) -> Result<PresenceUi, crate::ProtocolError> {
        Ok(PresenceUi {''')
rep(WIRE, '''/// 🎯️ Exact, allocation-bounded inverse of [`encode_presence_peer`]. Unknown flags,''', '''/// 🎞️ Exact inverse of [`encode_presence_history_edit`] over a standalone body: the peer decoder's limits (stage tag,
/// `u32` drafts) and no trailing bytes.
pub fn decode_presence_history_edit(bytes: &[u8]) -> Result<PresenceHistoryEdit, crate::ProtocolError> {
    let limits = PRESENCE_PEER_WIRE_LIMITS_V1;
    if bytes.len() > limits.maximum_entry_bytes { return Err(crate::ProtocolError::LimitExceeded("presence history edit bytes")); }
    let mut reader = PresencePeerReader { bytes, position: 0, limits };
    let history_edit = reader.history_edit()?;
    if reader.position != bytes.len() { return Err(reader.malformed("presence history edit", "trailing bytes")); }
    Ok(history_edit)
}

/// 🎯️ Exact, allocation-bounded inverse of [`encode_presence_peer`]. Unknown flags,''')
rep(WIRE, '''    if flags >> 13 != 0 { return Err(reader.malformed("presence peer flags", format!("unknown flag bits set: {flags:#x}"))); }''', '''    if flags >> 14 != 0 { return Err(reader.malformed("presence peer flags", format!("unknown flag bits set: {flags:#x}"))); }''')
rep(WIRE, '''    let active_tool = if flags & (1 << 12) != 0 { Some(reader.text("presence peer active tool")?) } else { None };
    if reader.position != bytes.len() { return Err(reader.malformed("presence peer", "trailing bytes")); }
    Ok(PresencePeer { actor, connected_at_ms, label, presence_pack, user_id, role, drag_ghost_json, interaction, color, surface, views, ui, tool_run, principal_kind, active_tool })''', '''    let active_tool = if flags & (1 << 12) != 0 { Some(reader.text("presence peer active tool")?) } else { None };
    let history_edit = if flags & (1 << 13) != 0 { Some(reader.history_edit()?) } else { None };
    if reader.position != bytes.len() { return Err(reader.malformed("presence peer", "trailing bytes")); }
    Ok(PresencePeer { actor, connected_at_ms, label, presence_pack, user_id, role, drag_ghost_json, interaction, color, surface, views, ui, tool_run, principal_kind, active_tool, history_edit })''')

# --- replication wire tests: the unknown-flag law moves to bit 14, bit 13 round-trips ---------------------------
rep(WIRE_TESTS, '''    // Hand-built rather than mutating an encode output: flags is a varint_u64. Bit 13 is one past
    // the frozen 0..=12 range (bit 12 = active_tool). The next field to take bit 13 must move this.
    let mut bytes = Vec::new();
    crate::write_str(&mut bytes, "peer-5");
    crate::wire::write_varint_u64(&mut bytes, 1 << 13);''', '''    // Hand-built rather than mutating an encode output: flags is a varint_u64. Bit 14 is one past
    // the frozen 0..=13 range (bit 13 = history_edit). The next field to take bit 14 must move this.
    let mut bytes = Vec::new();
    crate::write_str(&mut bytes, "peer-5");
    crate::wire::write_varint_u64(&mut bytes, 1 << 14);''')
rep(WIRE_TESTS, '''/// ⏯️ The standalone summary body an `AppFrame::Ephemeral` carries is byte-identical to the peer's flag-bit-10''', '''/// ⏪️ The history-edit summary round-trips through both codecs with every stage's wire spelling, its standalone body
/// is the peer's flag-bit-13 suffix, and a bad stage tag or trailing bytes are refused.
#[semio_framework_async_macros::async_test]
async fn presence_peer_history_edit_round_trips_every_stage_with_wire_spelling() {
    let stages = [(PresenceHistoryEditStage::Editing, "editing"), (PresenceHistoryEditStage::Replaying, "replaying"), (PresenceHistoryEditStage::Reviewing, "reviewing"), (PresenceHistoryEditStage::Choosing, "choosing"), (PresenceHistoryEditStage::Finalizing, "finalizing")];
    for (tag, (stage, spelling)) in stages.into_iter().enumerate() {
        assert_eq!((stage.wire_name(), PresenceHistoryEditStage::from_wire_name(spelling), stage as u8), (spelling, Some(stage), tag as u8));
        let history_edit = PresenceHistoryEdit { mutation_id: "e-2#0".into(), stage, drafts: 3 };
        let peer = PresencePeer { actor: "peer-8".into(), connected_at_ms: 1, label: None, presence_pack: None, user_id: None, role: None, drag_ghost_json: None, interaction: None, color: None, surface: None, views: Vec::new(), ui: None, tool_run: None, principal_kind: None, active_tool: None, history_edit: Some(history_edit.clone()) };
        let bytes = encode_presence_peer(&peer).await;
        assert_eq!(decode_presence_peer(&bytes).await.unwrap(), peer);
        let mut body = Vec::new();
        encode_presence_history_edit(&history_edit, &mut body);
        assert!(bytes.ends_with(&body), "the frame body is the peer's own history edit section");
        assert_eq!(decode_presence_history_edit(&body).unwrap(), history_edit);
        let value = crate::value::ToValue::to_value(&peer);
        assert_eq!(serde_json::Value::from(value.clone())["historyEdit"], serde_json::json!({ "mutationId": "e-2#0", "stage": spelling, "drafts": 3 }));
        assert_eq!(<PresencePeer as crate::value::FromValue>::from_value(value).unwrap(), peer);
    }
    assert_eq!(PresenceHistoryEditStage::from_wire_name("inactive"), None, "an inactive session publishes no summary");
    let mut bad_stage = Vec::new();
    crate::write_str(&mut bad_stage, "e-2#0");
    bad_stage.push(5);
    crate::wire::write_varint_u64(&mut bad_stage, 0);
    assert!(decode_presence_history_edit(&bad_stage).is_err(), "an unknown stage tag is refused");
    let mut trailing = Vec::new();
    encode_presence_history_edit(&PresenceHistoryEdit { mutation_id: "e-2#0".into(), stage: PresenceHistoryEditStage::Editing, drafts: 0 }, &mut trailing);
    trailing.push(0);
    assert!(decode_presence_history_edit(&trailing).is_err(), "trailing bytes are refused");
}

/// ⏯️ The standalone summary body an `AppFrame::Ephemeral` carries is byte-identical to the peer's flag-bit-10''')

# --- replication fixture and schema: bit 13 is known now; the unknown-flag vector moves to bit 14 -----------------
fixture = json.loads(text(WIRE_FIXTURE))
unknown = [case for case in fixture["cases"] if case["id"] == "unknown-flag"]
assert len(unknown) == 1 and unknown[0]["prefixHex"] == "0161804001", unknown
unknown[0]["prefixHex"] = "016180800101"
at = next(index for index, case in enumerate(fixture["cases"]) if case["id"] == "unknown-flag")
fixture["cases"][at:at] = [
    {"id": "history-edit-reviewing", "prefixHex": "016180400105652d3223300201", "repeatHex": "00", "repeatCount": 0, "suffixHex": "", "accepted": True, "canonicalHex": "016180400105652d3223300201", "expected": {"actor": "a", "connectedAtMs": 1, "views": [], "historyEdit": {"mutationId": "e-2#0", "stage": "reviewing", "drafts": 1}}},
    {"id": "history-edit-unknown-stage", "prefixHex": "016180400105652d3223300500", "repeatHex": "00", "repeatCount": 0, "suffixHex": "", "accepted": False},
    {"id": "history-edit-drafts-over-u32", "prefixHex": "016180400105652d322330008080808010", "repeatHex": "00", "repeatCount": 0, "suffixHex": "", "accepted": False},
    {"id": "history-edit-truncated", "prefixHex": "016180400105652d3223", "repeatHex": "00", "repeatCount": 0, "suffixHex": "", "accepted": False},
]
files[WIRE_FIXTURE] = json.dumps(fixture, indent=2, ensure_ascii=False) + "\n"
rep(WIRE_SCHEMA, '''        "activeTool": {
          "type": "string"
        },
        "principalKind": {''', '''        "activeTool": {
          "type": "string"
        },
        "historyEdit": {
          "$ref": "#/definitions/historyEdit"
        },
        "principalKind": {''')
rep(WIRE_SCHEMA, '''    "toolRun": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "toolId",''', '''    "historyEdit": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "mutationId",
        "stage",
        "drafts"
      ],
      "properties": {
        "mutationId": {
          "type": "string"
        },
        "stage": {
          "enum": [
            "editing",
            "replaying",
            "reviewing",
            "choosing",
            "finalizing"
          ]
        },
        "drafts": {
          "type": "integer",
          "minimum": 0,
          "maximum": 4294967295
        }
      }
    },
    "toolRun": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "toolId",''')

# --- replication TypeScript twin ---------------------------------------------------------------------------------
rep(WIRE_TS, '''  readonly activeTool?: string;
};

/** 🤖️ Twin of Rust `PresencePrincipalKind`, in binary tag order.''', '''  readonly activeTool?: string;
  /** ⏪️ Summary of this peer's open history edit (bit 13, ARTIFACT scope): who edits history, on which mutation, at
   * which stage. */
  readonly historyEdit?: ArtifactPresenceHistoryEdit;
};

/** ⏪️ Twin of Rust `PresenceHistoryEditStage`: the `HistoryTimeTravelStage` wire spelling in binary tag order. */
export const PRESENCE_HISTORY_EDIT_STAGES = Object.freeze(["editing", "replaying", "reviewing", "choosing", "finalizing"] as const);

/** 🔤️ One history-edit stage wire name. */
export type ArtifactPresenceHistoryEditStage = (typeof PRESENCE_HISTORY_EDIT_STAGES)[number];

/** 👥️ Twin of Rust `PresenceHistoryEdit`: the replica-independent id of the mutation a peer's open history edit drafts
 * (every replica labels it from its own history rows), the stage and the accepted draft count. */
export type ArtifactPresenceHistoryEdit = {
  readonly mutationId: string;
  readonly stage: ArtifactPresenceHistoryEditStage;
  readonly drafts: number;
};

/** 🤖️ Twin of Rust `PresencePrincipalKind`, in binary tag order.''')
rep(WIRE_TS, '''  if (presencePresent(peer.activeTool)) flags |= 1 << 12;
  writeVarintU64(out, flags);''', '''  if (presencePresent(peer.activeTool)) flags |= 1 << 12;
  if (presencePresent(peer.historyEdit)) flags |= 1 << 13;
  writeVarintU64(out, flags);''')
rep(WIRE_TS, '''  if (presencePresent(peer.activeTool)) writeStr(out, peer.activeTool);
''', '''  if (presencePresent(peer.activeTool)) writeStr(out, peer.activeTool);
  if (presencePresent(peer.historyEdit)) writePresenceHistoryEdit(out, peer.historyEdit);
''')
rep(WIRE_TS, '''function writePresenceToolRun(out: number[], toolRun: ArtifactPresenceToolRun): void {''', '''function writePresenceHistoryEdit(out: number[], historyEdit: ArtifactPresenceHistoryEdit): void {
  const tag = PRESENCE_HISTORY_EDIT_STAGES.indexOf(historyEdit.stage);
  if (tag < 0) throw new Error(`presence history edit stage: unknown ${historyEdit.stage}`);
  if (!Number.isSafeInteger(historyEdit.drafts) || historyEdit.drafts < 0 || historyEdit.drafts > 0xffffffff) throw new Error("presence history edit drafts: limit exceeded");
  writeStr(out, historyEdit.mutationId);
  out.push(tag);
  writeVarintU64(out, historyEdit.drafts);
}

function writePresenceToolRun(out: number[], toolRun: ArtifactPresenceToolRun): void {''')
rep(WIRE_TS, '''  principalKind(): ArtifactPresencePrincipalKind {''', '''  historyEdit(): ArtifactPresenceHistoryEdit {
    const mutationId = this.text("presence history edit mutation");
    const tag = this.byte("presence history edit stage");
    const stage = PRESENCE_HISTORY_EDIT_STAGES[tag];
    if (stage === undefined) this.fail("presence history edit stage", `unknown tag ${tag}`);
    const drafts = this.varint("presence history edit drafts");
    if (drafts > 0xffffffff) this.fail("presence history edit drafts", "limit exceeded");
    return { mutationId, stage, drafts };
  }

  principalKind(): ArtifactPresencePrincipalKind {''')
rep(WIRE_TS, '''  if (flags > 0x1fff) reader.fail("presence peer flags", `unknown flag bits set: ${flags.toString(16)}`);''', '''  if (flags > 0x3fff) reader.fail("presence peer flags", `unknown flag bits set: ${flags.toString(16)}`);''')
rep(WIRE_TS, '''  const activeTool = flags & (1 << 12) ? reader.text("presence peer active tool") : undefined;
  if (reader.position !== bytes.length) reader.fail("presence peer", "trailing bytes");
  pos[0] = reader.position;
  return { actor, connectedAtMs, label, presencePack, userId, role, dragGhostJson, interaction, color, surface, views, ui, toolRun, principalKind, activeTool };''', '''  const activeTool = flags & (1 << 12) ? reader.text("presence peer active tool") : undefined;
  const historyEdit = flags & (1 << 13) ? reader.historyEdit() : undefined;
  if (reader.position !== bytes.length) reader.fail("presence peer", "trailing bytes");
  pos[0] = reader.position;
  return { actor, connectedAtMs, label, presencePack, userId, role, dragGhostJson, interaction, color, surface, views, ui, toolRun, principalKind, activeTool, historyEdit };''')
rep(WIRE_TS, '''/** 🎞️ One raw byte — the TS twin of `protocol_core::read_u8`-shaped inline reads. */''', '''/** ⏪️ Twin of Rust `encode_presence_history_edit`: the standalone history-edit summary body a guest's
 * `AppFrame::Ephemeral.history_edit` carries, byte-identical to a peer's flag-bit-13 section. */
export function encodePresenceHistoryEdit(historyEdit: ArtifactPresenceHistoryEdit): number[] {
  const out: number[] = [];
  writePresenceHistoryEdit(out, historyEdit);
  return out;
}

/** 🎞️ Twin of Rust `decode_presence_history_edit`: the peer decoder's limits over a standalone body, no trailing bytes. */
export function decodePresenceHistoryEdit(bytes: Uint8Array): ArtifactPresenceHistoryEdit {
  if (bytes.length > PRESENCE_PEER_WIRE_LIMITS_V1.maximumEntryBytes) throw new Error("presence history edit bytes: limit exceeded");
  const reader = new PresencePeerReader(bytes, 0);
  const historyEdit = reader.historyEdit();
  if (reader.position !== bytes.length) reader.fail("presence history edit", "trailing bytes");
  return historyEdit;
}

/** 🎞️ One raw byte — the TS twin of `protocol_core::read_u8`-shaped inline reads. */''')

# --- kernel re-export ---------------------------------------------------------------------------------------------
rep(KERNEL, '''pub use semio_framework_os_kernel::{decode_presence_peer, encode_presence_peer, PresencePeer, PresenceToolRun, PresenceToolRunState, PresenceUi, PresenceViewKind, PresenceWindowView};''', '''pub use semio_framework_os_kernel::{decode_presence_history_edit, decode_presence_peer, encode_presence_history_edit, encode_presence_peer, PresenceHistoryEdit, PresenceHistoryEditStage, PresencePeer, PresenceToolRun, PresenceToolRunState, PresenceUi, PresenceViewKind, PresenceWindowView};''')

# --- channel AppFrame::Ephemeral ----------------------------------------------------------------------------------
rep(CHANNEL, '''    /// `tool_run` (trailing field) is `encode_presence_tool_run` of the instance's tool run summary — empty
    /// bytes without a run (`📋️tool-run-contract.md` §3.4).
    Ephemeral {
        presence: Vec<u8>,
        presence_generation: u64,
        transient_generation: u64,
        interaction: Vec<u8>,
        tool_run: Vec<u8>,
    },''', '''    /// `tool_run` (trailing field) is `encode_presence_tool_run` of the instance's tool run summary — empty
    /// bytes without a run (`📋️tool-run-contract.md` §3.4). `history_edit` (trailing field) is
    /// `encode_presence_history_edit` of the instance's open history edit — empty bytes while none is open.
    Ephemeral {
        presence: Vec<u8>,
        presence_generation: u64,
        transient_generation: u64,
        interaction: Vec<u8>,
        tool_run: Vec<u8>,
        history_edit: Vec<u8>,
    },''')
rep(CHANNEL, '''        AppFrame::Ephemeral { presence, presence_generation, transient_generation, interaction, tool_run } => {
            out.push(13);
            crate::os_spr::write_bytes(&mut out, presence);
            crate::os_spr::write_varint_u64(&mut out, *presence_generation);
            crate::os_spr::write_varint_u64(&mut out, *transient_generation);
            crate::os_spr::write_bytes(&mut out, interaction);
            crate::os_spr::write_bytes(&mut out, tool_run);
        }''', '''        AppFrame::Ephemeral { presence, presence_generation, transient_generation, interaction, tool_run, history_edit } => {
            out.push(13);
            crate::os_spr::write_bytes(&mut out, presence);
            crate::os_spr::write_varint_u64(&mut out, *presence_generation);
            crate::os_spr::write_varint_u64(&mut out, *transient_generation);
            crate::os_spr::write_bytes(&mut out, interaction);
            crate::os_spr::write_bytes(&mut out, tool_run);
            crate::os_spr::write_bytes(&mut out, history_edit);
        }''')
rep(CHANNEL, '''            interaction: crate::os_spr::read_bytes(bytes, &mut pos)?,
            tool_run: crate::os_spr::read_bytes(bytes, &mut pos)?,
        },''', '''            interaction: crate::os_spr::read_bytes(bytes, &mut pos)?,
            tool_run: crate::os_spr::read_bytes(bytes, &mut pos)?,
            history_edit: crate::os_spr::read_bytes(bytes, &mut pos)?,
        },''')
rep(CHANNEL_TESTS, '''    assert_frame_round_trips(&AppFrame::Ephemeral { presence: vec![1, 2], presence_generation: 3, transient_generation: 4, interaction: vec![9, 9], tool_run: Vec::new() }).await;
    assert_frame_round_trips(&AppFrame::Ephemeral { presence: vec![1, 2], presence_generation: 3, transient_generation: 4, interaction: Vec::new(), tool_run: vec![5, 6, 7] }).await;''', '''    assert_frame_round_trips(&AppFrame::Ephemeral { presence: vec![1, 2], presence_generation: 3, transient_generation: 4, interaction: vec![9, 9], tool_run: Vec::new(), history_edit: Vec::new() }).await;
    assert_frame_round_trips(&AppFrame::Ephemeral { presence: vec![1, 2], presence_generation: 3, transient_generation: 4, interaction: Vec::new(), tool_run: vec![5, 6, 7], history_edit: Vec::new() }).await;
    assert_frame_round_trips(&AppFrame::Ephemeral { presence: Vec::new(), presence_generation: 0, transient_generation: 0, interaction: Vec::new(), tool_run: Vec::new(), history_edit: vec![5, 101, 45, 50, 35, 48, 2, 1] }).await;''')
rep(CHANNEL_TESTS, '''        ("Ephemeral", AppFrame::Ephemeral { presence: vec![1, 2], presence_generation: 3, transient_generation: 4, interaction: vec![7], tool_run: vec![8] }),''', '''        ("Ephemeral", AppFrame::Ephemeral { presence: vec![1, 2], presence_generation: 3, transient_generation: 4, interaction: vec![7], tool_run: vec![8], history_edit: vec![9] }),''')

# --- os TypeScript AppFrame twin ----------------------------------------------------------------------------------
rep(OS_TS, '''  | { readonly Ephemeral: { readonly presence: readonly number[]; readonly presence_generation: number; readonly transient_generation: number; readonly interaction: readonly number[]; readonly tool_run: readonly number[] } }''', '''  | { readonly Ephemeral: { readonly presence: readonly number[]; readonly presence_generation: number; readonly transient_generation: number; readonly interaction: readonly number[]; readonly tool_run: readonly number[]; readonly history_edit: readonly number[] } }''')
rep(OS_TS, '''    writeBytes(out, frame.Ephemeral.tool_run);
''', '''    writeBytes(out, frame.Ephemeral.tool_run);
    writeBytes(out, frame.Ephemeral.history_edit);
''')
rep(OS_TS, '''interaction: readBytes(bytes, pos), tool_run: readBytes(bytes, pos) },''', '''interaction: readBytes(bytes, pos), tool_run: readBytes(bytes, pos), history_edit: readBytes(bytes, pos) },''')
rep(OS_TS_TESTS, '''      { Ephemeral: { presence: [1, 2], presence_generation: 3, transient_generation: 4, interaction: [7], tool_run: [8] } },
      { Ephemeral: { presence: [1, 2], presence_generation: 3, transient_generation: 4, interaction: [], tool_run: [] } },''', '''      { Ephemeral: { presence: [1, 2], presence_generation: 3, transient_generation: 4, interaction: [7], tool_run: [8], history_edit: [9] } },
      { Ephemeral: { presence: [1, 2], presence_generation: 3, transient_generation: 4, interaction: [], tool_run: [], history_edit: [] } },''')
rep(OS_TS_TESTS, '''      expect(encodeAppFrame({ Ephemeral: { presence: [], presence_generation: 0, transient_generation: 0, interaction: [], tool_run: [] } })[0]).toBe(13);''', '''      expect(encodeAppFrame({ Ephemeral: { presence: [], presence_generation: 0, transient_generation: 0, interaction: [], tool_run: [], history_edit: [] } })[0]).toBe(13);''')

# --- plugin: publish the session summary --------------------------------------------------------------------------
rep(PLUGIN, '''        pub tool_run: Option<protocol::PresenceToolRun>,
    }
''', '''        pub tool_run: Option<protocol::PresenceToolRun>,
        /// ⏪️ This instance's open history edit (`TimeTravelLedger::presence`) — the local `PresencePeer.history_edit`
        /// a host stamps; `None` while no history edit is open.
        pub history_edit: Option<protocol::PresenceHistoryEdit>,
    }
''')
rep(PLUGIN, '''                tool_run: self.tool_runs.presence(),
            }''', '''                tool_run: self.tool_runs.presence(),
                history_edit: self.time_travel.presence(),
            }''')
rep(PLUGIN, '''        if let Ok(EphemeralSnapshot { presence, presence_generation, transient_generation, interaction, tool_run }) = ephemeral.await {
            let tool_run = tool_run.map_or_else(Vec::new, |tool_run| {
                let mut bytes = Vec::new();
                protocol::encode_presence_tool_run(&tool_run, &mut bytes);
                bytes
            });
            frames.push(protocol::AppFrame::Ephemeral { presence, presence_generation, transient_generation, interaction, tool_run });''', '''        if let Ok(EphemeralSnapshot { presence, presence_generation, transient_generation, interaction, tool_run, history_edit }) = ephemeral.await {
            let tool_run = tool_run.map_or_else(Vec::new, |tool_run| {
                let mut bytes = Vec::new();
                protocol::encode_presence_tool_run(&tool_run, &mut bytes);
                bytes
            });
            let history_edit = history_edit.map_or_else(Vec::new, |history_edit| {
                let mut bytes = Vec::new();
                protocol::encode_presence_history_edit(&history_edit, &mut bytes);
                bytes
            });
            frames.push(protocol::AppFrame::Ephemeral { presence, presence_generation, transient_generation, interaction, tool_run, history_edit });''')
rep(PLUGIN_CONTRACT, '''interaction: Vec::new(), tool_run: None },''', '''interaction: Vec::new(), tool_run: None, history_edit: None },''')
rep(PLUGIN_DISPATCH, '''                    protocol::AppFrame::Ephemeral { presence, presence_generation, transient_generation, interaction, tool_run } => {
                        assert!(presence.is_empty()); assert!(interaction.is_empty()); assert!(tool_run.is_empty());''', '''                    protocol::AppFrame::Ephemeral { presence, presence_generation, transient_generation, interaction, tool_run, history_edit } => {
                        assert!(presence.is_empty()); assert!(interaction.is_empty()); assert!(tool_run.is_empty()); assert!(history_edit.is_empty());''')

# --- native host: decode and stamp --------------------------------------------------------------------------------
rep(BRIDGE, '''        let Some(AppFrame::Ephemeral { presence, presence_generation, interaction, tool_run, .. }) = frames.iter().rev().find(|frame| matches!(frame, AppFrame::Ephemeral { .. })) else { return };''', '''        let Some(AppFrame::Ephemeral { presence, presence_generation, interaction, tool_run, history_edit, .. }) = frames.iter().rev().find(|frame| matches!(frame, AppFrame::Ephemeral { .. })) else { return };''')
rep(BRIDGE, '''        let snapshot = ProgramEphemeralSnapshot { presence: (*presence_generation > 0).then(|| presence.clone()), interaction, tool_run };''', '''        let history_edit = if history_edit.is_empty() { None } else { protocol::decode_presence_history_edit(history_edit).ok() };
        let snapshot = ProgramEphemeralSnapshot { presence: (*presence_generation > 0).then(|| presence.clone()), interaction, tool_run, history_edit };''')
rep(BRIDGE, '''        let snapshot = ProgramEphemeralSnapshot { presence: (generation > 0.0).then_some(presence), interaction, tool_run: None };''', '''        let snapshot = ProgramEphemeralSnapshot { presence: (generation > 0.0).then_some(presence), interaction, tool_run: None, history_edit: None };''')
rep(BRIDGE, '''    pub tool_run: Option<protocol::PresenceToolRun>,
}
''', '''    pub tool_run: Option<protocol::PresenceToolRun>,
    pub history_edit: Option<protocol::PresenceHistoryEdit>,
}
''')

# --- plugin time-travel ledger: the summary it publishes -----------------------------------------------------------
TIME_TRAVEL = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs"
rep(TIME_TRAVEL, '''    /// 🪧️ The plain-data view the history panel and the history wire read; `None` while inactive.
    pub fn panel(&self) -> Option<TimeTravelPanel> {''', '''    /// 👥️ The ephemeral shared summary of the open session a host stamps on `PresencePeer.history_edit`: the mutation
    /// being edited (else the newest accepted draft's), the stage and the accepted draft count; `None` while inactive.
    pub fn presence(&self) -> Option<protocol::PresenceHistoryEdit> {
        let session = &self.session;
        let stage = match session.stage {
            TimeTravelStage::Inactive => return None,
            TimeTravelStage::Editing => protocol::PresenceHistoryEditStage::Editing,
            TimeTravelStage::Replaying => protocol::PresenceHistoryEditStage::Replaying,
            TimeTravelStage::Reviewing => protocol::PresenceHistoryEditStage::Reviewing,
            TimeTravelStage::Choosing => protocol::PresenceHistoryEditStage::Choosing,
            TimeTravelStage::Finalizing => protocol::PresenceHistoryEditStage::Finalizing,
        };
        let mutation_id = self.editor.as_ref().map(|editor| editor.target.0.clone()).or_else(|| session.accepted.last().map(|draft| draft.target.mutation.0.clone()))?;
        Some(protocol::PresenceHistoryEdit { mutation_id, stage, drafts: u32::try_from(session.accepted.len()).unwrap_or(u32::MAX) })
    }

    /// 🪧️ The plain-data view the history panel and the history wire read; `None` while inactive.
    pub fn panel(&self) -> Option<TimeTravelPanel> {''')

# --- every PresencePeer literal -----------------------------------------------------------------------------------
counts = {path: add_history_edit_to_peer_literals(path) for path in PEER_LITERAL_FILES}
rep(SHELL, '''tool_run: None, principal_kind: None, active_tool, history_edit: None };
        self.document_host.presence_heartbeat_key(''', '''tool_run: None, principal_kind: None, active_tool, history_edit: ephemeral.history_edit };
        self.document_host.presence_heartbeat_key(''')

for path, source in files.items():
    pathlib.Path(path).write_text(source)
print(counts)
