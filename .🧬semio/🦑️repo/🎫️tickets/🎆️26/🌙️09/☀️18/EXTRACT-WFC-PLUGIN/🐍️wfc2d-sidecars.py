#!/usr/bin/env python3
"""🐍️ Emits wfc2d's language sidecars (grammar/protocol/ebnf/g4/ksy/spicy/abnf) under `🚪️io` and the
facet-level 5-language schema leaves (`🔣️.json`/`🔗️.graphql`/`🛰️.proto`/`🟦️.ts`) under `🧬️schema`.
The JSON Schema leaf is normative; the other three mirror it field for field."""
from __future__ import annotations
import json, pathlib

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
SUB = ROOT / "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any"
IO = SUB / "🚪️io"
SCHEMA = SUB / "🧬️schema"

KINDS = [
    "change-seed", "create-slot", "delete-slot", "move-slot", "resize-slot", "connect-slots", "disconnect-slots",
    "pin-slot", "unpin-slot", "create-tile", "delete-tile", "change-tile-weight", "change-tile-media", "create-rule", "delete-rule",
]


def write(path: pathlib.Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def j(value) -> str:
    return json.dumps(value, ensure_ascii=False, indent=2) + "\n"


def protocol(name: str, schema: str) -> str:
    return f"""dialect protocol
protocol {name}
version 1
schema {schema}
start record
framing magic 0x8953f83f7d340d0a
header fixed 32
field format_major u16
field format_minor u16
field flags u32
field domain_tag u32
field header_crc32 u32
segment payload varint bytes
footer fixed 64
field artifact_mark utf8
field body_crc32 u32
"""


def ksy(name: str) -> str:
    return f"""meta:
  id: {name.replace('.', '_')}
  endian: le
seq:
  - id: magic
    contents: [0x0a, 0x0d, 0x34, 0x7d, 0x3f, 0xf8, 0x53, 0x89]
  - id: format_major
    type: u2
  - id: format_minor
    type: u2
  - id: flags
    type: u4
  - id: domain_tag
    type: u4
  - id: header_crc32
    type: u4
  - id: field_count
    type: u4
  - id: payload_length
    type: u4
  - id: payload
    size: payload_length
  - id: body_crc32
    type: u4
"""


def spicy(name: str) -> str:
    ident = name.replace(".", "_").title().replace("_", "")
    return f"""module {name.replace('.', '_')};

public type {ident} = unit {{
    magic: bytes &size=8;
    format_major: uint16;
    format_minor: uint16;
    flags: uint32;
    domain_tag: uint32;
    header_crc32: uint32;
    field_count: uint32;
    field_id: uint32;
    payload_length: uint32;
    payload: bytes &size=self.payload_length;
    body_crc32: uint32;
}};
"""


def abnf(name: str) -> str:
    slug = name.replace(".", "-")
    return f"""; 🔠️ {name} record framing, one record per document.
{slug}-record = {slug}-magic {slug}-header {slug}-payload {slug}-footer
{slug}-magic = %x0A %x0D %x34 %x7D %x3F %xF8 %x53 %x89
{slug}-header = 4OCTET 4OCTET 4OCTET 4OCTET 4OCTET 4OCTET
{slug}-payload = *OCTET
{slug}-footer = 4OCTET
"""


# ── 📖️ grammars ────────────────────────────────────────────────────────────────────────────────
write(
    IO / "📸️snapshot/📝️text/📖️.grammar.semio",
    """dialect grammar
grammar wfc.wfc2d.snapshot
extension wfc2d
start document

document = header body
header = "schema" SP "wfc.wfc2d.snapshot" NL
body = payload NL?
payload = OCTET+
""",
)
write(
    IO / "📸️snapshot/📝️text/🔤️.ebnf",
    """(* 🔤️ derived from 📖️.grammar.semio — kebab rule names rendered in space case. *)
document = header, body ;
header = "schema", " ", "wfc.wfc2d.snapshot", newline ;
body = payload, [ newline ] ;
payload = octet, { octet } ;
""",
)
write(
    IO / "📸️snapshot/📝️text/🅰️.g4",
    """// 🅰️ derived from 📖️.grammar.semio — kebab rule names rendered in camelCase.
grammar WfcWfc2dSnapshot;
document : header body ;
header : 'schema' ' ' 'wfc.wfc2d.snapshot' NL ;
body : payload NL? ;
payload : OCTET+ ;
NL : '\\n' ;
OCTET : . ;
""",
)
write(IO / "📸️snapshot/💾️binary/📡️.protocol.semio", protocol("wfc2d.snapshot", "wfc.wfc2d.snapshot"))
write(IO / "📸️snapshot/💾️binary/🥋️.ksy", ksy("wfc2d.snapshot"))
write(IO / "📸️snapshot/💾️binary/🌶️.spicy", spicy("wfc2d.snapshot"))
write(IO / "📸️snapshot/💾️binary/🔠️.abnf", abnf("wfc2d.snapshot"))

keywords = " / ".join(f'"{kind}"' for kind in KINDS)
write(
    IO / "🧬️mutations/📝️text/📖️.grammar.semio",
    f"""dialect grammar
grammar wfc.wfc2d.mutations
extension wfc2d
start line

line = keyword [SP arguments]
keyword = {keywords}
arguments = OCTET+
""",
)
write(
    IO / "🧬️mutations/📝️text/🔤️.ebnf",
    "(* 🔤️ derived from 📖️.grammar.semio — kebab rule names rendered in space case. *)\nline = keyword, [ \" \", arguments ] ;\nkeyword = "
    + " | ".join(f'"{kind}"' for kind in KINDS)
    + " ;\narguments = octet, { octet } ;\n",
)
write(
    IO / "🧬️mutations/📝️text/🅰️.g4",
    "// 🅰️ derived from 📖️.grammar.semio — kebab rule names rendered in camelCase.\ngrammar WfcWfc2dMutations;\nline : keyword (' ' arguments)? ;\nkeyword : "
    + " | ".join(f"'{kind}'" for kind in KINDS)
    + " ;\narguments : OCTET+ ;\nOCTET : . ;\n",
)
write(IO / "🧬️mutations/💾️binary/📡️.protocol.semio", protocol("wfc2d.mutations", "wfc.wfc2d.mutations"))
write(IO / "🧬️mutations/💾️binary/🥋️.ksy", ksy("wfc2d.mutations"))
write(IO / "🧬️mutations/💾️binary/🌶️.spicy", spicy("wfc2d.mutations"))
write(IO / "🧬️mutations/💾️binary/🔠️.abnf", abnf("wfc2d.mutations"))

write(
    IO / "🔺️diff/📝️text/📖️.grammar.semio",
    """dialect grammar
grammar wfc.wfc2d.diff
extension wfc2d
start diff

artifact-mark = "wfc.wfc2d-diff"
diff = artifact-mark change+
change = scalar-change / collection-change
scalar-change = "schema" "=" TEXT / "seed" "=" INT
collection-change = ("slots" / "edges" / "tiles" / "rules") ("-removed" IDENT / "-upserted" INT IDENT)
""",
)
write(
    IO / "🔺️diff/📝️text/🔤️.ebnf",
    """(* 🔤️ derived from 📖️.grammar.semio — kebab rule names rendered in space case. *)
diff = artifact mark, change, { change } ;
change = scalar change | collection change ;
""",
)
write(
    IO / "🔺️diff/📝️text/🅰️.g4",
    """// 🅰️ derived from 📖️.grammar.semio — kebab rule names rendered in camelCase.
grammar WfcWfc2dDiff;
diff : artifactMark change+ ;
change : scalarChange | collectionChange ;
artifactMark : 'wfc.wfc2d-diff' ;
scalarChange : ('schema' | 'seed') '=' TEXT ;
collectionChange : ('slots' | 'edges' | 'tiles' | 'rules') TEXT ;
TEXT : ~[\\r\\n]+ ;
""",
)
write(IO / "🔺️diff/💾️binary/📡️.protocol.semio", protocol("wfc2d.diff", "wfc.wfc2d.diff"))
write(IO / "🔺️diff/💾️binary/🥋️.ksy", ksy("wfc2d.diff"))
write(IO / "🔺️diff/💾️binary/🌶️.spicy", spicy("wfc2d.diff"))
write(IO / "🔺️diff/💾️binary/🔠️.abnf", abnf("wfc2d.diff"))

write(
    IO / "🔺️diff/📝️text/🦀️.rs",
    """//! 🔺️ WFC 2D diff — grammar spec asset only. `Wfc2dDiff` has no `impl store::ArtifactDsl` anywhere
//! in this plugin: this facet exists purely to register `wfc.wfc2d.diff`'s handcrafted text grammar
//! for LSP/verification tooling through `io()`'s `LanguageSpec`. The real apply/absorb algebra lives
//! in `🧬️schema/🔺️diff/🦀️.rs` (pure snapshot transforms).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar
""",
)
write(
    IO / "🔺️diff/💾️binary/🦀️.rs",
    """//! ⚖️ WFC 2D diff — binary protocol spec asset only; the diff never rides its own pack envelope.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol
""",
)

# ── 🔣️ facet schema leaves ─────────────────────────────────────────────────────────────────────
COLOR = {
    "type": "object",
    "additionalProperties": False,
    "required": ["r", "g", "b", "a"],
    "properties": {key: {"type": "integer", "minimum": 0, "maximum": 255} for key in "rgba"},
}
POINT = {"type": "array", "items": {"type": "number"}, "minItems": 2, "maxItems": 2}
SEGMENT = {
    "description": "Externally tagged path command in tile space 0..1: the bare string \"Close\", or a one-key object Move/Line/Quad/Cubic.",
    "oneOf": [
        {"type": "string", "enum": ["Close"]},
        {"type": "object", "additionalProperties": False, "required": ["Move"], "properties": {"Move": {"type": "object", "additionalProperties": False, "required": ["to"], "properties": {"to": POINT}}}},
        {"type": "object", "additionalProperties": False, "required": ["Line"], "properties": {"Line": {"type": "object", "additionalProperties": False, "required": ["to"], "properties": {"to": POINT}}}},
        {"type": "object", "additionalProperties": False, "required": ["Quad"], "properties": {"Quad": {"type": "object", "additionalProperties": False, "required": ["ctrl", "to"], "properties": {"ctrl": POINT, "to": POINT}}}},
        {
            "type": "object",
            "additionalProperties": False,
            "required": ["Cubic"],
            "properties": {"Cubic": {"type": "object", "additionalProperties": False, "required": ["ctrl1", "ctrl2", "to"], "properties": {"ctrl1": POINT, "ctrl2": POINT, "to": POINT}}},
        },
    ],
}
VECTOR_PATH = {
    "type": "object",
    "additionalProperties": False,
    "required": ["segments", "strokeWidth"],
    "properties": {"segments": {"type": "array", "items": SEGMENT}, "fill": COLOR, "stroke": COLOR, "strokeWidth": {"type": "number", "minimum": 0}},
}
MEDIA = {
    "description": "Externally tagged tile media: the bare string \"Empty\", or a one-key object Bitmap/Vector/Image.",
    "oneOf": [
        {"type": "string", "enum": ["Empty"]},
        {
            "type": "object",
            "additionalProperties": False,
            "required": ["Bitmap"],
            "properties": {
                "Bitmap": {
                    "type": "object",
                    "additionalProperties": False,
                    "required": ["width", "height", "palette", "pixels"],
                    "properties": {
                        "width": {"type": "integer", "minimum": 1},
                        "height": {"type": "integer", "minimum": 1},
                        "palette": {"type": "array", "items": COLOR},
                        "pixels": {"type": "string", "format": "base64"},
                    },
                }
            },
        },
        {
            "type": "object",
            "additionalProperties": False,
            "required": ["Vector"],
            "properties": {"Vector": {"type": "object", "additionalProperties": False, "required": ["paths"], "properties": {"paths": {"type": "array", "items": VECTOR_PATH}}}},
        },
        {
            "type": "object",
            "additionalProperties": False,
            "required": ["Image"],
            "properties": {
                "Image": {
                    "type": "object",
                    "additionalProperties": False,
                    "required": ["child"],
                    "properties": {
                        "child": {
                            "type": "object",
                            "description": "store::ArtifactChild handle into s.stdio.semio@v1/image.",
                            "required": ["childId", "target"],
                            "properties": {"childId": {"type": "string"}, "target": {"type": "object"}},
                        }
                    },
                }
            },
        },
    ],
}
SLOT = {
    "type": "object",
    "additionalProperties": False,
    "required": ["id", "x", "y", "width", "height"],
    "properties": {
        "id": {"type": "string"},
        "x": {"type": "number"},
        "y": {"type": "number"},
        "width": {"type": "number", "exclusiveMinimum": 0},
        "height": {"type": "number", "exclusiveMinimum": 0},
        "pinnedTileId": {"type": "string"},
    },
}
EDGE = {
    "type": "object",
    "additionalProperties": False,
    "required": ["id", "fromSlotId", "toSlotId", "relation"],
    "properties": {"id": {"type": "string"}, "fromSlotId": {"type": "string"}, "toSlotId": {"type": "string"}, "relation": {"type": "string"}},
}
TILE = {
    "type": "object",
    "additionalProperties": False,
    "required": ["id", "weight"],
    "properties": {"id": {"type": "string"}, "label": {"type": "string"}, "weight": {"type": "number", "minimum": 0}, "media": MEDIA},
}
RULE = {
    "type": "object",
    "additionalProperties": False,
    "required": ["id", "tileAId", "tileBId", "allowed"],
    "properties": {"id": {"type": "string"}, "tileAId": {"type": "string"}, "tileBId": {"type": "string"}, "relation": {"type": "string"}, "allowed": {"type": "boolean"}},
}
SNAPSHOT_PROPS = {
    "schema": {"type": "string", "const": "s.wfc.wfc2d"},
    "seed": {"type": "integer", "minimum": 0},
    "slots": {"type": "array", "items": SLOT},
    "edges": {"type": "array", "items": EDGE},
    "tiles": {"type": "array", "items": TILE},
    "rules": {"type": "array", "items": RULE},
}

BASE = "https://json.schemas.assets.semio-tech.com/s/wfc/wfc2d/1/any"

write(
    SCHEMA / "📸️snapshot/🔣️.json",
    j({"$schema": "http://json-schema.org/draft-07/schema#", "$id": f"{BASE}/snapshot/schema.json", "title": "Wfc2dSnapshot", "type": "object", "additionalProperties": False, "required": ["schema", "seed"], "properties": SNAPSHOT_PROPS}),
)

INDEXED = lambda item: {"type": "array", "items": [{"type": "integer", "minimum": 0}, item], "minItems": 2, "maxItems": 2}
write(
    SCHEMA / "🔺️diff/🔣️.json",
    j(
        {
            "$schema": "http://json-schema.org/draft-07/schema#",
            "$id": f"{BASE}/diff/schema.json",
            "title": "Wfc2dDiff",
            "type": "object",
            "additionalProperties": False,
            "required": ["schema", "seed", "slotsRemoved", "slotsUpserted", "edgesRemoved", "edgesUpserted", "tilesRemoved", "tilesUpserted", "rulesRemoved", "rulesUpserted"],
            "properties": {
                "schema": {"type": ["string", "null"]},
                "seed": {"type": ["integer", "null"], "minimum": 0},
                "slotsRemoved": {"type": "array", "items": {"type": "string"}},
                "slotsUpserted": {"type": "array", "items": INDEXED(SLOT)},
                "edgesRemoved": {"type": "array", "items": {"type": "string"}},
                "edgesUpserted": {"type": "array", "items": INDEXED(EDGE)},
                "tilesRemoved": {"type": "array", "items": {"type": "string"}},
                "tilesUpserted": {"type": "array", "items": INDEXED(TILE)},
                "rulesRemoved": {"type": "array", "items": {"type": "string"}},
                "rulesUpserted": {"type": "array", "items": INDEXED(RULE)},
            },
        }
    ),
)

VARIANT_PAYLOADS = {
    "ChangeSeed": {"seed": {"type": "integer", "minimum": 0}},
    "CreateSlot": {"slot": SLOT},
    "DeleteSlot": {"id": {"type": "string"}},
    "MoveSlot": {"id": {"type": "string"}, "x": {"type": "number"}, "y": {"type": "number"}},
    "ResizeSlot": {"id": {"type": "string"}, "width": {"type": "number", "exclusiveMinimum": 0}, "height": {"type": "number", "exclusiveMinimum": 0}},
    "ConnectSlots": {"edge": EDGE},
    "DisconnectSlots": {"id": {"type": "string"}},
    "PinSlot": {"id": {"type": "string"}, "tileId": {"type": "string"}},
    "UnpinSlot": {"id": {"type": "string"}},
    "CreateTile": {"tile": TILE},
    "DeleteTile": {"id": {"type": "string"}},
    "ChangeTileWeight": {"tileId": {"type": "string"}, "weight": {"type": "number", "minimum": 0}},
    "ChangeTileMedia": {"tileId": {"type": "string"}, "media": MEDIA},
    "CreateRule": {"rule": RULE},
    "DeleteRule": {"id": {"type": "string"}},
}
write(
    SCHEMA / "🧬️mutations/🔣️.json",
    j(
        {
            "$schema": "http://json-schema.org/draft-07/schema#",
            "$id": f"{BASE}/mutations/schema.json",
            "title": "Wfc2dMutation",
            "description": "Externally tagged: exactly one variant key, whose value is that kind's payload.",
            "oneOf": [
                {"type": "object", "additionalProperties": False, "required": [variant], "properties": {variant: {"type": "object", "additionalProperties": False, "required": sorted(props), "properties": props}}}
                for variant, props in VARIANT_PAYLOADS.items()
            ],
        }
    ),
)
write(
    SCHEMA / "🔣️.json",
    j(
        {
            "$schema": "http://json-schema.org/draft-07/schema#",
            "$id": f"{BASE}/artifact/schema.json",
            "title": "Wfc2dArtifact",
            "type": "object",
            "additionalProperties": False,
            "required": ["snapshot"],
            "properties": {"snapshot": {"$ref": f"{BASE}/snapshot/schema.json"}},
        }
    ),
)
write(
    SCHEMA / "💡️inferences/🔣️.json",
    j(
        {
            "$schema": "http://json-schema.org/draft-07/schema#",
            "$id": f"{BASE}/inferences/schema.json",
            "title": "Wfc2dInferenceCommit",
            "type": "object",
            "additionalProperties": False,
            "required": ["assignments", "contradiction", "entropy"],
            "properties": {
                "assignments": {"type": "object", "description": "Slot id to solved tile id.", "additionalProperties": {"type": "string"}},
                "contradiction": {"type": "boolean", "description": "True when the spec admits no total assignment."},
                "entropy": {"type": "object", "description": "Slot id to pre-propagation Shannon entropy over the tile weights.", "additionalProperties": {"type": "number"}},
            },
        }
    ),
)

# ── 🔗️ GraphQL mirrors ─────────────────────────────────────────────────────────────────────────
GRAPHQL_TYPES = """\"\"\"🎨 One straight-alpha sRGB colour, 0-255 per channel.\"\"\"
type Wfc2dColor {
  r: Int!
  g: Int!
  b: Int!
  a: Int!
}

\"\"\"✏️ One path command in tile space 0..1.\"\"\"
type Wfc2dPathSegment {
  kind: String!
  to: [Float!]
  ctrl: [Float!]
  ctrl1: [Float!]
  ctrl2: [Float!]
}

\"\"\"🖍️ One filled/stroked subpath of a vector tile.\"\"\"
type Wfc2dVectorPath {
  segments: [Wfc2dPathSegment!]!
  fill: Wfc2dColor
  stroke: Wfc2dColor
  strokeWidth: Float!
}

\"\"\"🖼️ What a tile looks like: exactly one of the four variants is set.\"\"\"
type Wfc2dTileMedia {
  empty: Boolean
  bitmapWidth: Int
  bitmapHeight: Int
  bitmapPalette: [Wfc2dColor!]
  bitmapPixels: String
  vectorPaths: [Wfc2dVectorPath!]
  imageChildId: String
}

\"\"\"🀄️ One placeable tile — the WFC pattern alphabet.\"\"\"
type Wfc2dTile {
  id: String!
  label: String
  weight: Float!
  media: Wfc2dTileMedia!
}

\"\"\"📍 One position the solver must fill.\"\"\"
type Wfc2dSlot {
  id: String!
  x: Float!
  y: Float!
  width: Float!
  height: Float!
  pinnedTileId: String
}

\"\"\"🔗 One adjacency edge between two slots.\"\"\"
type Wfc2dSlotEdge {
  id: String!
  fromSlotId: String!
  toSlotId: String!
  relation: String!
}

\"\"\"⛓️ One adjacency permission between two tile ids.\"\"\"
type Wfc2dRule {
  id: String!
  tileAId: String!
  tileBId: String!
  relation: String
  allowed: Boolean!
}
"""

write(
    SCHEMA / "📸️snapshot/🔗️.graphql",
    GRAPHQL_TYPES
    + """
\"\"\"🌊️ The persisted WFC problem for an arbitrary 2D slot graph.\"\"\"
type Wfc2dSnapshot {
  schema: String!
  seed: Float!
  slots: [Wfc2dSlot!]!
  edges: [Wfc2dSlotEdge!]!
  tiles: [Wfc2dTile!]!
  rules: [Wfc2dRule!]!
}
""",
)
write(
    SCHEMA / "🔗️.graphql",
    """\"\"\"🧬️ The Wfc2d artifact facet — the persisted problem spec is the artifact.\"\"\"
type Wfc2dArtifact {
  snapshot: Wfc2dSnapshot!
}
""",
)
write(
    SCHEMA / "🔺️diff/🔗️.graphql",
    """\"\"\"🔺️ One indexed slot upsert.\"\"\"
type Wfc2dSlotUpsert {
  index: Int!
  value: Wfc2dSlot!
}

\"\"\"🔺️ One indexed edge upsert.\"\"\"
type Wfc2dSlotEdgeUpsert {
  index: Int!
  value: Wfc2dSlotEdge!
}

\"\"\"🔺️ One indexed tile upsert.\"\"\"
type Wfc2dTileUpsert {
  index: Int!
  value: Wfc2dTile!
}

\"\"\"🔺️ One indexed rule upsert.\"\"\"
type Wfc2dRuleUpsert {
  index: Int!
  value: Wfc2dRule!
}

\"\"\"🔺️ A sparse, id-keyed structural delta over Wfc2dSnapshot.\"\"\"
type Wfc2dDiff {
  schema: String
  seed: Float
  slotsRemoved: [String!]!
  slotsUpserted: [Wfc2dSlotUpsert!]!
  edgesRemoved: [String!]!
  edgesUpserted: [Wfc2dSlotEdgeUpsert!]!
  tilesRemoved: [String!]!
  tilesUpserted: [Wfc2dTileUpsert!]!
  rulesRemoved: [String!]!
  rulesUpserted: [Wfc2dRuleUpsert!]!
}
""",
)
mutation_types = "\n\n".join(
    f'"""🧬️ `{variant}` payload."""\ntype Wfc2d{variant} {{\n'
    + "\n".join(
        f"  {name}: " + {"integer": "Int!", "number": "Float!", "string": "String!", "boolean": "Boolean!"}.get(spec.get("type"), f"Wfc2d{name[0].upper()}{name[1:]}!")
        for name, spec in props.items()
    )
    + "\n}"
    for variant, props in VARIANT_PAYLOADS.items()
)
write(
    SCHEMA / "🧬️mutations/🔗️.graphql",
    mutation_types
    + "\n\n\"\"\"🧬️ The mutation union — exactly one member is set.\"\"\"\ntype Wfc2dMutation {\n"
    + "\n".join(f"  {variant[0].lower()}{variant[1:]}: Wfc2d{variant}" for variant in VARIANT_PAYLOADS)
    + "\n}\n",
)
write(
    SCHEMA / "💡️inferences/🔗️.graphql",
    """\"\"\"💡️ One solved slot assignment.\"\"\"
type Wfc2dAssignment {
  slotId: String!
  tileId: String!
}

\"\"\"💡️ One slot's pre-propagation Shannon entropy.\"\"\"
type Wfc2dEntropyEntry {
  slotId: String!
  entropy: Float!
}

\"\"\"💡️ The `s.wfc.wfc2d.solve` commit — never persisted on the snapshot.\"\"\"
type Wfc2dInferenceCommit {
  assignments: [Wfc2dAssignment!]!
  contradiction: Boolean!
  entropy: [Wfc2dEntropyEntry!]!
}
""",
)

# ── 🛰️ Protobuf mirrors ────────────────────────────────────────────────────────────────────────
PROTO_TYPES = """message Wfc2dColor {
  uint32 r = 1;
  uint32 g = 2;
  uint32 b = 3;
  uint32 a = 4;
}

message Wfc2dPathSegment {
  string kind = 1;
  repeated double to = 2;
  repeated double ctrl = 3;
  repeated double ctrl1 = 4;
  repeated double ctrl2 = 5;
}

message Wfc2dVectorPath {
  repeated Wfc2dPathSegment segments = 1;
  Wfc2dColor fill = 2;
  Wfc2dColor stroke = 3;
  double stroke_width = 4;
}

message Wfc2dBitmapMedia {
  uint32 width = 1;
  uint32 height = 2;
  repeated Wfc2dColor palette = 3;
  string pixels = 4;
}

message Wfc2dVectorMedia {
  repeated Wfc2dVectorPath paths = 1;
}

message Wfc2dImageMedia {
  string child_id = 1;
  string artifact_id = 2;
}

message Wfc2dTileMedia {
  oneof media {
    bool empty = 1;
    Wfc2dBitmapMedia bitmap = 2;
    Wfc2dVectorMedia vector = 3;
    Wfc2dImageMedia image = 4;
  }
}

message Wfc2dTile {
  string id = 1;
  string label = 2;
  double weight = 3;
  Wfc2dTileMedia media = 4;
}

message Wfc2dSlot {
  string id = 1;
  double x = 2;
  double y = 3;
  double width = 4;
  double height = 5;
  string pinned_tile_id = 6;
}

message Wfc2dSlotEdge {
  string id = 1;
  string from_slot_id = 2;
  string to_slot_id = 3;
  string relation = 4;
}

message Wfc2dRule {
  string id = 1;
  string tile_a_id = 2;
  string tile_b_id = 3;
  string relation = 4;
  bool allowed = 5;
}
"""
write(
    SCHEMA / "📸️snapshot/🛰️.proto",
    'syntax = "proto3";\n\npackage semio.s.wfc.wfc2d;\n\n'
    + PROTO_TYPES
    + """
message Wfc2dSnapshot {
  string schema = 1;
  uint64 seed = 2;
  repeated Wfc2dSlot slots = 3;
  repeated Wfc2dSlotEdge edges = 4;
  repeated Wfc2dTile tiles = 5;
  repeated Wfc2dRule rules = 6;
}
""",
)
write(
    SCHEMA / "🛰️.proto",
    'syntax = "proto3";\n\npackage semio.s.wfc.wfc2d;\n\nmessage Wfc2dArtifact {\n  Wfc2dSnapshot snapshot = 1;\n}\n',
)
write(
    SCHEMA / "🔺️diff/🛰️.proto",
    """syntax = "proto3";

package semio.s.wfc.wfc2d;

message Wfc2dSlotUpsert {
  uint32 index = 1;
  Wfc2dSlot value = 2;
}

message Wfc2dSlotEdgeUpsert {
  uint32 index = 1;
  Wfc2dSlotEdge value = 2;
}

message Wfc2dTileUpsert {
  uint32 index = 1;
  Wfc2dTile value = 2;
}

message Wfc2dRuleUpsert {
  uint32 index = 1;
  Wfc2dRule value = 2;
}

message Wfc2dDiff {
  optional string schema = 1;
  optional uint64 seed = 2;
  repeated string slots_removed = 3;
  repeated Wfc2dSlotUpsert slots_upserted = 4;
  repeated string edges_removed = 5;
  repeated Wfc2dSlotEdgeUpsert edges_upserted = 6;
  repeated string tiles_removed = 7;
  repeated Wfc2dTileUpsert tiles_upserted = 8;
  repeated string rules_removed = 9;
  repeated Wfc2dRuleUpsert rules_upserted = 10;
}
""",
)
proto_field = {"integer": "uint64", "number": "double", "string": "string", "boolean": "bool"}
mutation_messages = []
for variant, props in VARIANT_PAYLOADS.items():
    lines = []
    for index, (name, spec) in enumerate(props.items(), start=1):
        snake = "".join("_" + ch.lower() if ch.isupper() else ch for ch in name)
        kind = proto_field.get(spec.get("type"))
        if kind is None:
            kind = {"slot": "Wfc2dSlot", "edge": "Wfc2dSlotEdge", "tile": "Wfc2dTile", "rule": "Wfc2dRule", "media": "Wfc2dTileMedia"}[name]
        lines.append(f"  {kind} {snake} = {index};")
    mutation_messages.append(f"message Wfc2d{variant} {{\n" + "\n".join(lines) + "\n}")
write(
    SCHEMA / "🧬️mutations/🛰️.proto",
    'syntax = "proto3";\n\npackage semio.s.wfc.wfc2d;\n\n'
    + "\n\n".join(mutation_messages)
    + "\n\nmessage Wfc2dMutation {\n  oneof mutation {\n"
    + "\n".join(f"    Wfc2d{variant} {''.join('_' + ch.lower() if ch.isupper() else ch for ch in variant).lstrip('_')} = {index};" for index, variant in enumerate(VARIANT_PAYLOADS, start=1))
    + "\n  }\n}\n",
)
write(
    SCHEMA / "💡️inferences/🛰️.proto",
    """syntax = "proto3";

package semio.s.wfc.wfc2d;

message Wfc2dInferenceCommit {
  map<string, string> assignments = 1;
  bool contradiction = 2;
  map<string, double> entropy = 3;
}
""",
)

print("sidecars + facet leaves written")
