#!/usr/bin/env python3
"""One-shot generator for remodel A5 app schema leaves (ticket temp)."""
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🎛️apps/📸️remodel")
CFG_SCHEMA = ROOT / "🎚️config/🧬️schema"
PRS_SCHEMA = ROOT / "👥️presence/🧬️schema"
PRS_COMPONENT = ROOT / "👥️presence/🦀️component.rs"

PLUGIN = "remodel"
OWNER = "remodel"
CONFIG_ID = f"https://semio.tech/schema/app/{PLUGIN}/{OWNER}/config.json"
PRESENCE_ID = f"https://semio.tech/schema/app/{PLUGIN}/{OWNER}/presence.json"
PROTO_PKG = f"semio.app.{PLUGIN}.{OWNER}"


def camel(snake: str) -> str:
    p = snake.split("_")
    return p[0] + "".join(x.title() for x in p[1:])


def json_scalar(ty: str) -> dict:
    ty = ty.strip()
    if ty == "String":
        return {"type": "string"}
    if ty == "bool":
        return {"type": "boolean"}
    if ty == "u32":
        return {"type": "integer", "minimum": 0}
    if ty == "f64":
        return {"type": "number"}
    if ty.startswith("[f64; 3]"):
        return {"type": "array", "items": {"type": "number"}, "minItems": 3, "maxItems": 3}
    if ty == "Vec<String>":
        return {"type": "array", "items": {"type": "string"}}
    if ty == "Option<String>":
        return {"type": "string"}
    raise ValueError(ty)


NESTED = {
    "RemodelWorldCamera": [
        ("position", "[f64; 3]", False),
        ("target", "[f64; 3]", False),
        ("fov", "f64", False),
    ],
    "RemodelSelection": [
        ("mode", "String", False),
        ("ids", "Vec<String>", False),
    ],
    "RemodelLayerVisibility": [
        ("mesh", "bool", False),
        ("dense", "bool", False),
        ("sparse", "bool", False),
        ("cameras", "bool", False),
        ("gcps", "bool", False),
    ],
    "RemodelFrameCursor": [
        ("stream_id", "Option<String>", True),
        ("frame_index", "u32", False),
    ],
}

CONFIG_TOP = [
    ("camera", "RemodelWorldCamera", False),
    ("selection", "RemodelSelection", False),
    ("layers", "RemodelLayerVisibility", False),
    ("frame_cursor", "RemodelFrameCursor", False),
    ("report_table", "String", False),
    ("active_utility_id", "String", False),
    ("locale", "String", False),
]

PRESENCE_FIELDS = [
    ("selection_mode", "String", False),
    ("selection_ids", "Vec<String>", False),
    ("world_camera_position", "[f64; 3]", False),
    ("world_camera_target", "[f64; 3]", False),
    ("world_camera_fov", "f64", False),
    ("frame_stream_id", "Option<String>", True),
    ("frame_index", "u32", False),
    ("active_utility_id", "String", False),
    ("report_table", "String", False),
]


def nested_json_def(name: str) -> dict:
    props = {}
    req = []
    for snake, ty, optional in NESTED[name]:
        c = camel(snake)
        props[c] = json_scalar(ty)
        props[c]["x-semio-state"] = "local-ui"
        if not optional:
            req.append(c)
    return {
        "type": "object",
        "additionalProperties": False,
        "required": req,
        "properties": props,
    }


def emit_config_json() -> dict:
    props = {}
    req = []
    for snake, ty, optional in CONFIG_TOP:
        c = camel(snake)
        if ty in NESTED:
            prop = nested_json_def(ty)
            prop["x-semio-state"] = "local-ui"
            props[c] = prop
        else:
            props[c] = json_scalar(ty)
            props[c]["x-semio-state"] = "local-ui"
        if not optional:
            req.append(c)
    return {
        "$id": CONFIG_ID,
        "title": "RemodelConfig",
        "type": "object",
        "additionalProperties": False,
        "required": req,
        "properties": props,
    }


def emit_presence_json() -> dict:
    props = {}
    req = []
    for snake, ty, optional in PRESENCE_FIELDS:
        c = camel(snake)
        props[c] = json_scalar(ty)
        props[c]["x-semio-state"] = "shared-ui"
        if not optional:
            req.append(c)
    return {
        "$id": PRESENCE_ID,
        "title": "RemodelPresence",
        "type": "object",
        "additionalProperties": False,
        "required": req,
        "properties": props,
    }


def ts_type(ty: str) -> str:
    if ty == "String":
        return "string"
    if ty == "bool":
        return "boolean"
    if ty == "u32":
        return "number"
    if ty == "f64":
        return "number"
    if ty == "[f64; 3]":
        return "number[]"
    if ty == "Vec<String>":
        return "string[]"
    if ty == "Option<String>":
        return "string"
    if ty in NESTED:
        return ty
    raise ValueError(ty)


def emit_ts_interface(title: str, fields, state: str, nested: bool = False) -> str:
    lines = [f"/** 🧬️ {title} */", f"export interface {title} {{"]
    for snake, ty, optional in fields:
        c = camel(snake)
        opt = optional or ty.startswith("Option<")
        lines.append(f"  /** @state {state} */")
        if ty in NESTED:
            lines.append(f"  {c}{'?' if opt else ''}: {ty};")
        else:
            lines.append(f"  {c}{'?' if opt else ''}: {ts_type(ty)};")
    lines.append("}")
    if nested:
        out = []
        for n, fs in NESTED.items():
            out.append(emit_ts_interface(n, fs, state))
        out.append("\n".join(lines))
        return "\n\n".join(out)
    return "\n".join(lines)


def gql_scalar(ty: str) -> str:
    return {
        "String": "String",
        "bool": "Boolean",
        "u32": "Int",
        "f64": "Float",
        "[f64; 3]": "[Float!]",
        "Vec<String>": "[String!]",
        "Option<String>": "String",
    }[ty]


def emit_gql_type(title: str, fields, state: str) -> str:
    enum = "LOCAL_UI" if state == "local-ui" else "SHARED_UI"
    lines = [f"type {title} {{"]
    for snake, ty, optional in fields:
        c = camel(snake)
        if ty in NESTED:
            gql = ty + "!"
        else:
            base = gql_scalar(ty)
            gql = base if optional else f"{base}!"
        lines.append(f"  {c}: {gql} @state(class: {enum})")
    lines.append("}")
    return "\n".join(lines)


def emit_gql_config() -> str:
    parts = []
    for n, fs in NESTED.items():
        parts.append(emit_gql_type(n, fs, "local-ui"))
    parts.append(emit_gql_type("RemodelConfig", CONFIG_TOP, "local-ui"))
    return "\n\n".join(parts)


def emit_gql_presence() -> str:
    return emit_gql_type("RemodelPresence", PRESENCE_FIELDS, "shared-ui")


def rust_ty(ty: str) -> str:
    m = {
        "String": "String",
        "bool": "bool",
        "u32": "u32",
        "f64": "f64",
        "[f64; 3]": "[f64; 3]",
        "Vec<String>": "Vec<String>",
        "Option<String>": "Option<String>",
    }
    if ty in NESTED:
        return ty
    return m[ty]


def emit_rust_schema(title: str, fields, state: str, artifact_id: str, default_derive: bool = False) -> str:
    state_attr = "local_ui" if state == "local-ui" else "shared_ui"
    extra = ", Default" if default_derive else ""
    lines = [
        "//! 🧬️ schema leaf",
        "use schema::ArtifactSchema;",
        "use serde::{Deserialize, Serialize};",
        "",
    ]
    for n, fs in NESTED.items():
        if any(f[1] == n for f in fields):
            lines.append(f"#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]")
            lines.append('#[serde(rename_all = "camelCase")]')
            lines.append(f'#[artifact_schema(id = "s.{PLUGIN}.{OWNER}.{n.lower()}")]')
            lines.append(f"pub struct {n} {{")
            for snake, ty, _ in fs:
                lines.append(f"    #[state({state_attr})] pub {snake}: {rust_ty(ty)},")
            lines.append("}")
            lines.append("")
    lines.append(f"#[derive(Clone, Debug{extra}, PartialEq, Serialize, Deserialize, ArtifactSchema)]")
    lines.append(f'#[serde(rename_all = "camelCase"{", default" if default_derive else ""})]')
    lines.append(f'#[artifact_schema(id = "{artifact_id}")]')
    lines.append(f"pub struct {title} {{")
    for snake, ty, _ in fields:
        lines.append(f"    #[state({state_attr})] pub {snake}: {rust_ty(ty)},")
    lines.append("}")
    return "\n".join(lines)


def proto_scalar(ty: str) -> str:
    return {
        "String": "string",
        "bool": "bool",
        "u32": "uint32",
        "f64": "double",
        "[f64; 3]": "repeated double",
        "Vec<String>": "repeated string",
        "Option<String>": "string",
    }[ty]


def emit_proto_message(name: str, fields, state: str, field_start: int = 1) -> tuple[str, int]:
    lines = [f"message {name} {{"]
    n = field_start
    for snake, ty, optional in fields:
        if ty in NESTED:
            continue
        comment = f"  // @state {state}"
        lines.append(comment)
        opt = "optional " if optional and ty == "Option<String>" else ""
        if ty == "[f64; 3]":
            lines.append(f"  repeated double {snake} = {n};")
        else:
            lines.append(f"  {opt}{proto_scalar(ty)} {snake} = {n};")
        n += 1
    lines.append("}")
    return "\n".join(lines), n


def emit_proto_config() -> str:
    parts = [f"syntax = \"proto3\";", f"package {PROTO_PKG};", ""]
    n = 1
    for nest_name, fs in NESTED.items():
        msg, n = emit_proto_message(nest_name, fs, "local-ui", n)
        parts.append(msg)
        parts.append("")
    top_lines = ["message RemodelConfig {"]
    idx = 1
    for snake, ty, _ in CONFIG_TOP:
        top_lines.append("  // @state local-ui")
        if ty in NESTED:
            top_lines.append(f"  {ty} {snake} = {idx};")
        else:
            top_lines.append(f"  {proto_scalar(ty)} {snake} = {idx};")
        idx += 1
    top_lines.append("}")
    parts.append("\n".join(top_lines))
    return "\n".join(parts)


def emit_proto_presence() -> str:
    msg, _ = emit_proto_message("RemodelPresence", PRESENCE_FIELDS, "shared-ui", 1)
    return f"syntax = \"proto3\";\npackage {PROTO_PKG};\n\n{msg}\n"


def write(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text)


def main() -> None:
    write(CFG_SCHEMA / "🔣️component.json", json.dumps(emit_config_json(), indent=2) + "\n")
    write(PRS_SCHEMA / "🔣️component.json", json.dumps(emit_presence_json(), indent=2) + "\n")
    write(
        CFG_SCHEMA / "🦀️component.rs",
        emit_rust_schema("RemodelConfig", CONFIG_TOP, "local-ui", f"s.{PLUGIN}.{OWNER}.config"),
    )
    write(
        PRS_SCHEMA / "🦀️component.rs",
        emit_rust_schema(
            "RemodelPresence",
            PRESENCE_FIELDS,
            "shared-ui",
            f"s.{PLUGIN}.{OWNER}.presence",
            default_derive=True,
        ),
    )
    write(CFG_SCHEMA / "🟦️component.ts", emit_ts_interface("RemodelConfig", CONFIG_TOP, "local-ui", nested=True) + "\n")
    write(PRS_SCHEMA / "🟦️component.ts", emit_ts_interface("RemodelPresence", PRESENCE_FIELDS, "shared-ui") + "\n")
    write(CFG_SCHEMA / "🔗️component.graphql", emit_gql_config() + "\n")
    write(PRS_SCHEMA / "🔗️component.graphql", emit_gql_presence() + "\n")
    write(CFG_SCHEMA / "🛰️component.proto", emit_proto_config())
    write(PRS_SCHEMA / "🛰️component.proto", emit_proto_presence())
    print("wrote config + presence schema leaves")


if __name__ == "__main__":
    main()
