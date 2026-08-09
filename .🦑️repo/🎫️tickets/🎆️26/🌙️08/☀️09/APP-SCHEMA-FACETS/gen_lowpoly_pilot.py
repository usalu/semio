#!/usr/bin/env python3
from pathlib import Path
import json, re

lp = next(p for p in Path(".").rglob("*lowpoly*") if p.is_dir() and (p / "🎛️apps").is_dir())
cfg_rs = (lp / "🎛️apps" / "💠️lowpoly" / "🎚️config" / "🦀️component.rs").read_text()
body = re.search(r"pub struct LowpolyConfig \{(.*?)\n\}", cfg_rs, re.S).group(1)
fields = [(m.group(1), m.group(2).strip().rstrip(",")) for m in re.finditer(r"pub ([a-z0-9_]+)\s*:\s*([^,\n]+)", body)]


def snake_to_camel(s):
    parts = s.split("_")
    return parts[0] + "".join(x.title() for x in parts[1:])


def rust_scalar_json(t):
    return {
        "String": {"type": "string"},
        "bool": {"type": "boolean"},
        "u8": {"type": "integer", "minimum": 0, "maximum": 255},
        "u32": {"type": "integer", "minimum": 0},
        "f64": {"type": "number"},
        "i32": {"type": "integer"},
    }.get(t.strip(), {"type": "string"})


def rust_to_json_type(ty):
    optional = ty.startswith("Option<")
    core = re.sub(r"^Option<|>$", "", ty) if optional else ty
    if core.startswith("Vec<"):
        return optional, {"type": "array", "items": rust_scalar_json(core[4:-1])}
    if core.startswith("[") and ";" in core:
        inner = core.split(";")[0].strip("[] ")
        n = int(core.split(";")[1].strip("] "))
        return optional, {"type": "array", "items": rust_scalar_json(inner), "minItems": n, "maxItems": n}
    return optional, rust_scalar_json(core)


def rust_to_ts(core):
    if core.startswith("Vec<"):
        return rust_to_ts(core[4:-1]) + "[]"
    if core.startswith("[") and ";" in core:
        return rust_to_ts(core.split(";")[0].strip("[] ")) + "[]"
    return {"String": "string", "bool": "boolean", "u8": "number", "u32": "number", "f64": "number", "i32": "number"}.get(core, "string")


def rust_to_gql(core):
    if core.startswith("Vec<"):
        return "[" + rust_to_gql(core[4:-1]) + "!]"
    if core.startswith("[") and ";" in core:
        return "[" + rust_to_gql(core.split(";")[0].strip("[] ")) + "!]"
    return {"String": "String", "bool": "Boolean", "u8": "Int", "u32": "Int", "f64": "Float", "i32": "Int"}.get(core, "String")


def emit_json(title, fs, state, id_):
    props = {}
    req = []
    for snake, ty in fs:
        camel = snake_to_camel(snake)
        optional, schema = rust_to_json_type(ty)
        prop = dict(schema)
        prop["x-semio-state"] = state
        props[camel] = prop
        if not optional:
            req.append(camel)
    return {"$id": id_, "title": title, "type": "object", "additionalProperties": False, "required": req, "properties": props}


def rust_fields(fs, state):
    return "\n".join(f"    #[state({state})] pub {snake}: {ty}," for snake, ty in fs)


def emit_ts(title, fs, state):
    lines = [f"/** 🧬️ {title} */", f"export interface {title} {{"]
    for snake, ty in fs:
        camel = snake_to_camel(snake)
        optional = ty.startswith("Option<")
        core = re.sub(r"^Option<|>$", "", ty) if optional else ty
        lines.append(f"  /** @state {state} */")
        lines.append(f"  {camel}{'?' if optional else ''}: {rust_to_ts(core)};")
    lines.append("}")
    return "\n".join(lines) + "\n"


def emit_gql(title, fs, state_enum):
    lines = [f"type {title} {{"]
    for snake, ty in fs:
        camel = snake_to_camel(snake)
        optional = ty.startswith("Option<")
        core = re.sub(r"^Option<|>$", "", ty) if optional else ty
        lines.append(f"  {camel}: {rust_to_gql(core)}{'' if optional else '!'} @state(class: {state_enum})")
    lines.append("}")
    return "\n".join(lines) + "\n"


def emit_proto(title, fs, state):
    lines = ['syntax = "proto3";', "package semio.app.lowpoly.lowpoly;", f"message {title} {{"]
    for i, (snake, ty) in enumerate(fs, 1):
        optional = ty.startswith("Option<")
        core = re.sub(r"^Option<|>$", "", ty) if optional else ty
        repeated = ""
        if core.startswith("Vec<"):
            repeated = "repeated "
            core = core[4:-1]
        if core.startswith("[") and ";" in core:
            repeated = "repeated "
            core = core.split(";")[0].strip("[] ")
        ptype = {"String": "string", "bool": "bool", "u8": "uint32", "u32": "uint32", "f64": "double", "i32": "int32"}.get(core, "string")
        opt = "optional " if optional and not repeated else ""
        lines.append(f"  // @state {state}")
        lines.append(f"  {opt}{repeated}{ptype} {snake} = {i};")
    lines.append("}")
    return "\n".join(lines) + "\n"


presence_fields = [
    ("selection_mode", "String"),
    ("selection_ids", "Vec<u32>"),
    ("selection_targets_mesh", "bool"),
    ("selection_targets_vertex", "bool"),
    ("selection_targets_edge", "bool"),
    ("selection_targets_face", "bool"),
    ("selected_object_ids", "Vec<String>"),
    ("hovered_object_id", "Option<String>"),
    ("hovered_target_object_id", "Option<String>"),
    ("hovered_target_mode", "Option<String>"),
    ("hovered_target_id", "Option<u32>"),
    ("world_camera_position", "[f64; 3]"),
    ("world_camera_target", "[f64; 3]"),
    ("world_camera_fov", "f64"),
    ("active_utility_id", "String"),
    ("paint_utility", "String"),
]

base = lp / "🎛️apps" / "💠️lowpoly"
config_schema = base / "🎚️config" / "🧬️schema"
presence = base / "👥️presence"
presence_schema = presence / "🧬️schema"
for d in [config_schema, presence, presence_schema]:
    d.mkdir(parents=True, exist_ok=True)

(config_schema / "🔣️component.json").write_text(
    json.dumps(emit_json("LowpolyConfig", fields, "local-ui", "https://semio.tech/schema/app/lowpoly/lowpoly/config.json"), indent=2) + "\n"
)
(presence_schema / "🔣️component.json").write_text(
    json.dumps(emit_json("LowpolyPresence", presence_fields, "shared-ui", "https://semio.tech/schema/app/lowpoly/lowpoly/presence.json"), indent=2) + "\n"
)

(config_schema / "🦀️component.rs").write_text(
    "//! 🧬️ Lowpoly app config schema — every local-ui field of LowpolyConfig.\n\n"
    "use schema::ArtifactSchema;\nuse serde::{Deserialize, Serialize};\n\n"
    "//#region 🔖️Config\n"
    "/// 🎚️ Lowpoly app config — unshared local app state.\n"
    "#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]\n"
    '#[serde(rename_all = "camelCase")]\n'
    '#[artifact_schema(id = "s.lowpoly.lowpoly.config")]\n'
    "pub struct LowpolyConfig {\n"
    + rust_fields(fields, "local_ui")
    + "\n}\n//#endregion 🔖️Config\n"
)

(presence_schema / "🦀️component.rs").write_text(
    "//! 🧬️ Lowpoly app presence schema — shared live ephemeral state.\n\n"
    "use schema::ArtifactSchema;\nuse serde::{Deserialize, Serialize};\n\n"
    "//#region 🔖️Presence\n"
    "/// 👥️ Lowpoly presence — what peers must see live and must not persist.\n"
    "#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]\n"
    '#[serde(rename_all = "camelCase", default)]\n'
    '#[artifact_schema(id = "s.lowpoly.lowpoly.presence")]\n'
    "pub struct LowpolyPresence {\n"
    + rust_fields(presence_fields, "shared_ui")
    + "\n}\n//#endregion 🔖️Presence\n"
)

(presence / "🦀️component.rs").write_text(
    "//! 👥️ Lowpoly presence document + mutation surface.\n\n"
    "pub use crate::apps::lowpoly::presence::schema::LowpolyPresence;\n"
    "use protocol::Mutation;\nuse serde::{Deserialize, Serialize};\n\n"
    "//#region 🔖️PresenceMutation\n"
    "#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]\n"
    '#[serde(rename_all = "camelCase")]\n'
    "pub enum LowpolyPresenceMutation {\n"
    "    Set(LowpolyPresence),\n"
    "}\n\n"
    "impl Mutation<LowpolyPresence> for LowpolyPresenceMutation {\n"
    "    fn apply(&self, target: &mut LowpolyPresence) {\n"
    "        match self {\n"
    "            Self::Set(next) => *target = next.clone(),\n"
    "        }\n"
    "    }\n"
    "    fn backwards(&self, prior: &LowpolyPresence) -> Self {\n"
    "        Self::Set(prior.clone())\n"
    "    }\n"
    "}\n"
    "//#endregion 🔖️PresenceMutation\n"
)

(config_schema / "🟦️component.ts").write_text(emit_ts("LowpolyConfig", fields, "local-ui"))
(config_schema / "🔗️component.graphql").write_text(emit_gql("LowpolyConfig", fields, "LOCAL_UI"))
(config_schema / "🛰️component.proto").write_text(emit_proto("LowpolyConfig", fields, "local-ui"))
(presence_schema / "🟦️component.ts").write_text(emit_ts("LowpolyPresence", presence_fields, "shared-ui"))
(presence_schema / "🔗️component.graphql").write_text(emit_gql("LowpolyPresence", presence_fields, "SHARED_UI"))
(presence_schema / "🛰️component.proto").write_text(emit_proto("LowpolyPresence", presence_fields, "shared-ui"))
print("OK", base)
for p in sorted(base.rglob("*")):
    if p.is_file() and ("🧬️schema" in str(p) or "👥️presence" in str(p)):
        print(" ", p.relative_to(base))
