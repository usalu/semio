#!/usr/bin/env python3
"""🧪 Generate reasoning wires fifteen-leaf facet schemas (wave 5)."""

from __future__ import annotations

import json
from pathlib import Path

PLUGIN = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires")
SCHEMA_ID = "s.reasoning.wires"

ARTIFACT_FIELDS = [
    ("wiresFixture", "persistent", "object", True),
    ("boardFixture", "persistent", "object", True),
    ("selectedIds", "shared-ui", "string_list", False),
    ("dragNodeId", "preview", "optional_string", False),
    ("dragLastX", "preview", "number", False),
    ("dragLastY", "preview", "number", False),
    ("locale", "local-ui", "string", False),
]

SNAPSHOT_FIELDS = [
    ("wiresFixture", "persistent", "object", True),
    ("boardFixture", "persistent", "object", True),
]

DIFF_FIELDS = [
    ("artifact", "persistent", "wires_artifact", False),
    ("wiresFixture", "persistent", "optional_object", False),
    ("boardFixture", "persistent", "optional_object", False),
    ("selectedIds", "shared-ui", "optional_string_list", False),
    ("dragNodeId", "preview", "optional_optional_string", False),
    ("dragLastX", "preview", "optional_number", False),
    ("dragLastY", "preview", "optional_number", False),
    ("locale", "local-ui", "optional_string", False),
]


def write(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text if text.endswith("\n") else text + "\n", encoding="utf-8")


def gql_state(s: str) -> str:
    return s.replace("-", "_").upper()


def json_prop(name: str, kind: str, required: bool) -> dict:
    base = {"x-semio-state": kind.split("_")[0] if "optional" not in kind else kind}
    if kind == "object":
        return {**base, "type": "object", "additionalProperties": True}
    if kind == "optional_object":
        return {**base, "type": "object", "additionalProperties": True}
    if kind == "string_list":
        return {**base, "type": "array", "items": {"type": "string"}}
    if kind == "optional_string_list":
        return {**base, "$ref": "#/$defs/WiresStringList"}
    if kind == "optional_string":
        return {**base, "type": "string"}
    if kind == "optional_optional_string":
        return {**base, "oneOf": [{"type": "null"}, {"type": "string"}]}
    if kind == "optional_number":
        return {**base, "type": "number", "format": "double"}
    if kind == "number":
        return {**base, "type": "number", "format": "double"}
    if kind == "optional_string":
        return {**base, "type": "string"}
    if kind == "wires_artifact":
        return {**base, "$ref": "#/$defs/WiresArtifact"}
    return {**base, "type": "string"}


def build_json(facet: str, title: str, fields: list, extra_defs: dict) -> str:
    props = {}
    required = []
    for name, state, kind, req in fields:
        props[name] = json_prop(name, kind, req)
        props[name]["x-semio-state"] = state
        if req or facet == "diff":
            if kind not in ("optional_object", "optional_string", "optional_number", "optional_string_list", "optional_optional_string", "wires_artifact"):
                required.append(name)
            elif facet == "artifact":
                required.append(name)
    if facet == "diff":
        required = [f[0] for f in fields if f[0] != "artifact"]
    defs = {
        "WiresStringList": {
            "title": "WiresStringList",
            "type": "object",
            "additionalProperties": False,
            "required": ["values"],
            "properties": {"values": {"type": "array", "items": {"type": "string"}}},
        },
        **extra_defs,
    }
    doc = {
        "$id": f"https://semio.tech/schema/s/reasoning/wires/{facet}.json",
        "title": title,
        "type": "object",
        "additionalProperties": False,
        "required": required,
        "properties": props,
        "$defs": defs,
    }
    return json.dumps(doc, indent=2) + "\n"


def ts_type(kind: str) -> str:
    m = {
        "object": "Record<string, unknown>",
        "optional_object": "Record<string, unknown>",
        "string_list": "string[]",
        "optional_string_list": "WiresStringList",
        "optional_string": "string",
        "optional_optional_string": "string | null",
        "optional_number": "number",
        "number": "number",
        "wires_artifact": "WiresArtifact",
    }
    return m.get(kind, "string")


def build_ts(facet: str, title: str, fields: list) -> str:
    lines = [f"/** 🧬️ Wires {facet} schema — every field with its state class. */", "", f"export interface {title} {{"]
    for name, state, kind, _ in fields:
        opt = "?" if kind.startswith("optional") and kind != "optional_string_list" else ""
        if kind == "optional_optional_string":
            opt = "?"
        lines.append(f"  /** @state {state} */")
        lines.append(f"  {name}{opt}: {ts_type(kind)};")
    lines.append("}")
    if facet != "artifact" or True:
        lines.extend(["", "export interface WiresStringList {", "  values: string[];", "}"])
    return "\n".join(lines) + "\n"


def build_gql(facet: str, title: str, fields: list) -> str:
    lines = [f"# 🧬️ Wires {facet} schema — every field with its state class.", "", f"type {title} {{"]
    for name, state, kind, req in fields:
        gql_kind = "String!"
        if kind in ("object", "optional_object"):
            gql_kind = "String!" if facet != "artifact" or kind == "object" else "String!"
        if kind == "string_list":
            gql_kind = "[String!]!"
        if kind == "optional_string_list":
            gql_kind = "WiresStringList"
        if kind == "optional_optional_string":
            gql_kind = "String"
        if kind in ("number", "optional_number"):
            gql_kind = "Float!" if not kind.startswith("optional") else "Float"
        if kind == "wires_artifact":
            gql_kind = "WiresArtifact"
        if kind == "optional_string":
            gql_kind = "String"
        null = "" if gql_kind.endswith("!") or "optional" in kind and kind != "optional_string_list" else ""
        if kind == "optional_object":
            gql_kind = "String"
        lines.append(f"  {name}: {gql_kind} @state(class: {gql_state(state)})")
    lines.append("}")
    lines.extend(["", "type WiresStringList {", "  values: [String!]!", "}"])
    return "\n".join(lines) + "\n"


def build_proto(facet: str, title: str, fields: list) -> str:
    pkg = f"semio.s.reasoning.wires.{facet}"
    lines = [
        "syntax = \"proto3\";",
        f"package {pkg};",
        "",
        f"// 🧬️ Wires {facet} schema.",
        "",
        f"message {title} {{",
    ]
    idx = 1
    for name, state, kind, _ in fields:
        snake = "".join("_" + c.lower() if c.isupper() else c for c in name).lstrip("_")
        if kind in ("object", "optional_object"):
            field = f"{snake}_json"
            typ = "string"
        elif kind in ("string_list", "optional_string_list"):
            if kind == "optional_string_list":
                lines.append(f"message WiresStringList {{")
                lines.append(f"  repeated string values = 1;")
                lines.append("}")
                lines.append("")
                field = snake
                typ = "WiresStringList"
            else:
                field = snake
                typ = "repeated string"
        elif kind in ("number", "optional_number"):
            field = snake
            typ = "double"
        elif kind == "wires_artifact":
            field = "artifact"
            typ = "WiresArtifact"
        elif kind == "optional_optional_string":
            field = snake
            typ = "string"
        else:
            field = snake
            typ = "string"
        comment = f"  // @state {state}"
        if typ.startswith("repeated"):
            lines.append(comment)
            lines.append(f"  {typ} {field} = {idx};")
        else:
            lines.append(comment)
            lines.append(f"  {typ} {field} = {idx};")
        idx += 1
    lines.append("}")
    return "\n".join(lines) + "\n"


extra_artifact_defs = {
    "WiresArtifact": {
        "title": "WiresArtifact",
        "type": "object",
        "additionalProperties": False,
        "required": [f[0] for f in ARTIFACT_FIELDS],
        "properties": {f[0]: {**json_prop(f[0], f[2], f[3]), "x-semio-state": f[1]} for f in ARTIFACT_FIELDS},
    }
}

for facet, title, fields, extra in [
    ("artifact", "WiresArtifact", ARTIFACT_FIELDS, extra_artifact_defs),
    ("snapshot", "WiresSnapshot", SNAPSHOT_FIELDS, {}),
    ("diff", "WiresDiff", DIFF_FIELDS, extra_artifact_defs),
]:
    base = PLUGIN / ("🧬️schema" if facet == "artifact" else f"{'📸️snapshot' if facet == 'snapshot' else '🔺️diff'}/🧬️schema")
    write(base / "🔣️component.json", build_json(facet, title, fields, extra))
    write(base / "🟦️component.ts", build_ts(facet, title, fields))
    write(base / "🔗️component.graphql", build_gql(facet, title, fields))
    write(base / "🛰️component.proto", build_proto(facet, title, fields))

print("done")
