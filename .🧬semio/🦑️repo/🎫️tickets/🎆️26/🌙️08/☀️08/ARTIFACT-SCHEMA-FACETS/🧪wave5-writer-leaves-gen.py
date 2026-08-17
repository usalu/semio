#!/usr/bin/env python3
"""Generate writer schema leaf files (json, graphql, proto, ts)."""
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer")
PKG = "semio.s.writer.writer"

ARTIFACT_FIELDS = [
    ("schema", "persistent", "string", True, "string", "String", "string"),
    ("id", "persistent", "string", True, "string", "String", "string"),
    ("languageId", "persistent", "string", True, "string", "String", "string"),
    ("uri", "persistent", "string", True, "string", "String", "string"),
    ("text", "persistent", "string", True, "string", "String", "string"),
    ("selectedAstIds", "shared-ui", "string[]", True, "array", "[String!]!", "repeated string"),
    ("editorSelection", "shared-ui", "WriterEditorSelection", False, "WriterEditorSelection", "WriterEditorSelection", "WriterEditorSelection"),
    ("editorSettings", "shared-ui", "WriterEditorSettings", True, "WriterEditorSettings!", "WriterEditorSettings!", "WriterEditorSettings"),
    ("formatSignal", "local-ui", "uint32", True, "Int!", "Int", "uint32"),
    ("lintSignal", "local-ui", "uint32", True, "Int!", "Int", "uint32"),
    ("revision", "local-ui", "uint32", True, "Int!", "Int", "uint32"),
    ("engagementInput", "local-ui", "string", True, "String!", "String", "string"),
    ("cameraX", "local-ui", "float64", True, "Float!", "Float", "double"),
    ("cameraY", "local-ui", "float64", True, "Float!", "Float", "double"),
    ("cameraZoom", "local-ui", "float64", True, "Float!", "Float", "double"),
    ("locale", "local-ui", "string", True, "String!", "String", "string"),
    ("treeHoveredAstId", "preview", "string?", False, "String", "String", "optional string"),
    ("editorHoverOffset", "preview", "uint32?", False, "Int", "Int", "optional uint32"),
]

SNAPSHOT_FIELDS = ARTIFACT_FIELDS[:5]

DIFF_FIELDS = [
    ("artifact", "persistent", "WriterArtifact", False, "WriterArtifact", "WriterArtifact", "WriterArtifact"),
    ("schema", "persistent", "string?", False, "String", "String", "optional string"),
    ("id", "persistent", "string?", False, "String", "String", "optional string"),
    ("languageId", "persistent", "string?", False, "String", "String", "optional string"),
    ("uri", "persistent", "string?", False, "String", "String", "optional string"),
    ("text", "persistent", "WriterTextDelta?", False, "WriterTextDelta", "WriterTextDelta", "WriterTextDelta"),
    ("selectedAstIds", "shared-ui", "WriterStringList?", False, "WriterStringList", "WriterStringList", "WriterStringList"),
    ("editorSelection", "shared-ui", "WriterEditorSelection??", False, "WriterEditorSelection", "WriterEditorSelection", "WriterEditorSelection"),
    ("editorSettings", "shared-ui", "WriterEditorSettings?", False, "WriterEditorSettings", "WriterEditorSettings", "WriterEditorSettings"),
    ("formatSignal", "local-ui", "uint32?", False, "Int", "Int", "optional uint32"),
    ("lintSignal", "local-ui", "uint32?", False, "Int", "Int", "optional uint32"),
    ("revision", "local-ui", "uint32?", False, "Int", "Int", "optional uint32"),
    ("engagementInput", "local-ui", "string?", False, "String", "String", "optional string"),
    ("cameraX", "local-ui", "float64?", False, "Float", "Float", "optional double"),
    ("cameraY", "local-ui", "float64?", False, "Float", "Float", "optional double"),
    ("cameraZoom", "local-ui", "float64?", False, "Float", "Float", "optional double"),
    ("locale", "local-ui", "string?", False, "String", "String", "optional string"),
    ("treeHoveredAstId", "preview", "string??", False, "String", "String", "optional string"),
    ("editorHoverOffset", "preview", "uint32??", False, "Int", "Int", "optional uint32"),
]

STATE_JSON = {"persistent": "persistent", "shared-ui": "shared-ui", "local-ui": "local-ui", "preview": "preview"}


def camel_to_snake(name: str) -> str:
    out = []
    for i, c in enumerate(name):
        if c.isupper() and i:
            out.append("_")
        out.append(c.lower())
    return "".join(out)


def json_scalar(t: str, required: bool):
    if t == "string":
        return {"type": "string"}
    if t == "uint32":
        return {"type": "integer", "format": "uint32", "minimum": 0}
    if t == "float64":
        return {"type": "number", "format": "double"}
    if t.endswith("?"):
        inner = json_scalar(t[:-1], False)
        return {"oneOf": [{"type": "null"}, inner]}
    if t.endswith("??"):
        inner = json_scalar(t[:-2], False)
        return {"oneOf": [{"type": "null"}, inner]}
    if t.endswith("[]"):
        return {"type": "array", "items": {"type": "string"}}
    if t.startswith("Writer"):
        return {"$ref": f"#/$defs/{t.replace('?', '').replace('!', '')}"}
    return {"type": "string"}


def write_json(path: Path, title: str, fields, facet: str):
    props = {}
    required = []
    defs = {
        "WriterEditorSelection": {
            "type": "object",
            "additionalProperties": False,
            "required": ["start", "end"],
            "properties": {
                "start": {"type": "integer", "format": "uint32", "minimum": 0},
                "end": {"type": "integer", "format": "uint32", "minimum": 0},
            },
        },
        "WriterEditorSettings": {
            "type": "object",
            "additionalProperties": False,
            "required": ["showLineNumbers", "fontPx", "lineHeight", "tabSize"],
            "properties": {
                "showLineNumbers": {"type": "boolean"},
                "fontPx": {"type": "integer", "format": "uint32", "minimum": 0},
                "lineHeight": {"type": "integer", "format": "uint32", "minimum": 0},
                "tabSize": {"type": "integer", "format": "uint32", "minimum": 0},
            },
        },
        "WriterStringList": {
            "type": "object",
            "additionalProperties": False,
            "required": ["values"],
            "properties": {"values": {"type": "array", "items": {"type": "string"}}},
        },
        "WriterTextRangeEdit": {
            "type": "object",
            "additionalProperties": False,
            "required": ["start", "end", "insert"],
            "properties": {
                "start": {"type": "integer", "format": "uint32", "minimum": 0},
                "end": {"type": "integer", "format": "uint32", "minimum": 0},
                "insert": {"type": "string"},
            },
        },
        "WriterTextDelta": {
            "type": "object",
            "additionalProperties": False,
            "properties": {
                "replacement": {"type": "string"},
                "edits": {
                    "type": "array",
                    "items": {"$ref": "#/$defs/WriterTextRangeEdit"},
                },
            },
        },
    }
    if facet == "artifact":
        defs["WriterArtifact"] = {
            "type": "object",
            "additionalProperties": False,
            "required": [f[0] for f in fields if f[3]],
            "properties": {},
        }
    for name, state, typ, req, *_ in fields:
        prop = json_scalar(typ, req)
        prop["x-semio-state"] = STATE_JSON[state]
        if facet == "artifact" and name in defs.get("WriterArtifact", {}).get("properties", {}):
            pass
        props[name] = prop
        if facet == "artifact" and req:
            required.append(name)
    obj = {
        "$id": f"https://semio.tech/schema/s/writer/writer/{facet}.json",
        "title": title,
        "type": "object",
        "additionalProperties": False,
        "required": required if facet != "diff" else [],
        "properties": props,
        "$defs": defs,
    }
    import json

    path.write_text(json.dumps(obj, indent=2) + "\n")


def write_graphql(path: Path, type_name: str, fields):
    lines = [f"# 🧬️ Writer {type_name.lower()} schema.", "", f"type {type_name} {{"]
    for name, state, typ, req, gql, *_ in fields:
        lines.append(f"  {name}: {gql}")
    lines.append("}")
    lines += [
        "",
        "type WriterEditorSelection {",
        "  start: Int!",
        "  end: Int!",
        "}",
        "",
        "type WriterEditorSettings {",
        "  showLineNumbers: Boolean!",
        "  fontPx: Int!",
        "  lineHeight: Int!",
        "  tabSize: Int!",
        "}",
        "",
        "type WriterStringList {",
        "  values: [String!]!",
        "}",
        "",
        "type WriterTextRangeEdit {",
        "  start: Int!",
        "  end: Int!",
        "  insert: String!",
        "}",
        "",
        "type WriterTextDelta {",
        "  replacement: String",
        "  edits: [WriterTextRangeEdit!]!",
        "}",
        "",
    ]
    path.write_text("\n".join(lines) + "\n")


def write_proto(path: Path, msg: str, fields):
    lines = [
        "syntax = \"proto3\";",
        f"package {PKG};",
        "",
        f"message {msg} {{",
    ]
    n = 1
    for name, _, typ, req, _, _, proto in fields:
        opt = "optional " if "optional" in proto else ""
        if proto.startswith("repeated"):
            lines.append(f"  {proto} {camel_to_snake(name)} = {n};")
        elif "Writer" in proto:
            lines.append(f"  {opt}{proto} {camel_to_snake(name)} = {n};")
        else:
            lines.append(f"  {opt}{proto} {camel_to_snake(name)} = {n};")
        n += 1
    lines.append("}")
    lines += [
        "",
        "message WriterEditorSelection {",
        "  uint32 start = 1;",
        "  uint32 end = 2;",
        "}",
        "",
        "message WriterEditorSettings {",
        "  bool show_line_numbers = 1;",
        "  uint32 font_px = 2;",
        "  uint32 line_height = 3;",
        "  uint32 tab_size = 4;",
        "}",
        "",
        "message WriterStringList {",
        "  repeated string values = 1;",
        "}",
        "",
        "message WriterTextRangeEdit {",
        "  uint32 start = 1;",
        "  uint32 end = 2;",
        "  string insert = 3;",
        "}",
        "",
        "message WriterTextDelta {",
        "  optional string replacement = 1;",
        "  repeated WriterTextRangeEdit edits = 2;",
        "}",
        "",
    ]
    path.write_text("\n".join(lines) + "\n")


def write_ts(path: Path, iface: str, fields, facet: str):
    lines = [f"/** 🧬️ Writer {facet} schema. */", "", f"export interface {iface} {{"]
    for name, state, typ, req, *_ in fields:
        ts = typ.replace("uint32", "number").replace("float64", "number")
        if ts.endswith("[]"):
            ts = "string[]"
        if ts.endswith("??"):
            ts = ts[:-2] + " | null"
        if ts.endswith("?") and not ts.endswith("| null"):
            ts = ts[:-1] + "?"
        if "Writer" in ts:
            ts = ts.replace("!", "")
        lines.append(f"  /** @state {state} */")
        lines.append(f"  {name}: {ts};")
    lines.append("}")
    lines += [
        "",
        "export interface WriterEditorSelection {",
        "  start: number;",
        "  end: number;",
        "}",
        "",
        "export interface WriterEditorSettings {",
        "  showLineNumbers: boolean;",
        "  fontPx: number;",
        "  lineHeight: number;",
        "  tabSize: number;",
        "}",
        "",
        "export interface WriterStringList {",
        "  values: string[];",
        "}",
        "",
        "export interface WriterTextRangeEdit {",
        "  start: number;",
        "  end: number;",
        "  insert: string;",
        "}",
        "",
        "export interface WriterTextDelta {",
        "  replacement?: string;",
        "  edits: WriterTextRangeEdit[];",
        "}",
        "",
    ]
    path.write_text("\n".join(lines) + "\n")


for facet, fields, title, iface in [
    ("artifact", ARTIFACT_FIELDS, "WriterArtifact", "WriterArtifact"),
    ("snapshot", SNAPSHOT_FIELDS, "WriterSnapshot", "WriterSnapshot"),
    ("diff", DIFF_FIELDS, "WriterDiff", "WriterDiff"),
]:
    if facet == "artifact":
        base = ROOT / "🧬️schema"
    elif facet == "snapshot":
        base = ROOT / "📸️snapshot/🧬️schema"
    else:
        base = ROOT / "🔺️diff/🧬️schema"
    write_json(base / "🔣️component.json", title, fields, facet)
    write_graphql(base / "🔗️component.graphql", title, fields)
    write_proto(base / "🛰️component.proto", title, fields)
    write_ts(base / "🟦️component.ts", iface, fields, facet)

print("done")
