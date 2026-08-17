#!/usr/bin/env python3
"""Generate procedural A5 config/presence schema leaves + presence runtimes."""

from __future__ import annotations

import json
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
PLUGIN = ROOT / "✏️s/🔌️plugins/🌀️procedural"
APP2 = PLUGIN / "🎛️apps/◻2d"
APP3 = PLUGIN / "🎛️apps/🧊3d"


def write(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text if text.endswith("\n") else text + "\n")
    print("wrote", path.relative_to(ROOT))


# region helpers — CameraJson / PreviewCamera nested defs

CAMERA_JSON_DEF = {
    "title": "CameraJson",
    "type": "object",
    "additionalProperties": False,
    "required": ["x", "y", "zoom"],
    "properties": {
        "x": {"type": "number", "format": "double"},
        "y": {"type": "number", "format": "double"},
        "zoom": {"type": "number", "format": "double"},
    },
}

PREVIEW_CAM_DEF = {
    "title": "Procedural3dPreviewCamera",
    "type": "object",
    "additionalProperties": False,
    "required": ["position", "target", "fov"],
    "properties": {
        "position": {
            "type": "array",
            "items": {"type": "number", "format": "double"},
            "minItems": 3,
            "maxItems": 3,
        },
        "target": {
            "type": "array",
            "items": {"type": "number", "format": "double"},
            "minItems": 3,
            "maxItems": 3,
        },
        "fov": {"type": "number", "format": "double"},
    },
}

CAMERA_TS = "export type CameraJson = { x: number; y: number; zoom: number };\n"
PREVIEW_TS = (
    "export type Procedural3dPreviewCamera = {\n"
    "  position: number[];\n"
    "  target: number[];\n"
    "  fov: number;\n"
    "};\n"
)

CAMERA_GQL = "type CameraJson {\n  x: Float!\n  y: Float!\n  zoom: Float!\n}\n"
PREVIEW_GQL = (
    "type Procedural3dPreviewCamera {\n"
    "  position: [Float!]!\n"
    "  target: [Float!]!\n"
    "  fov: Float!\n"
    "}\n"
)

CAMERA_PROTO = (
    "message CameraJson {\n"
    "  double x = 1;\n"
    "  double y = 2;\n"
    "  double zoom = 3;\n"
    "}\n"
)
PREVIEW_PROTO = (
    "message Procedural3dPreviewCamera {\n"
    "  repeated double position = 1;\n"
    "  repeated double target = 2;\n"
    "  double fov = 3;\n"
    "}\n"
)

# endregion

# region field specs

# (snake, rust_type_for_schema_leaf, json_prop, ts_type, gql_type, proto_type, optional)

CFG2 = [
    ("selected_ids", "Vec<String>", {"type": "array", "items": {"type": "string"}}, "string[]", "[String!]!", "repeated string", False),
    ("camera", "CameraJson", {"$ref": "#/$defs/CameraJson"}, "CameraJson", "CameraJson!", "CameraJson", False),
    ("show_mode", "String", {"type": "string"}, "string", "String!", "string", False),
    ("selected_generation_id", "Option<String>", {"type": "string"}, "string", "String", "optional string", True),
    ("generation_preview_text", "Option<String>", {"type": "string"}, "string", "String", "optional string", True),
    ("locale", "String", {"type": "string"}, "string", "String!", "string", False),
]

PRES2 = [
    ("selected_ids", "Vec<String>", {"type": "array", "items": {"type": "string"}}, "string[]", "[String!]!", "repeated string", False),
    ("camera", "CameraJson", {"$ref": "#/$defs/CameraJson"}, "CameraJson", "CameraJson!", "CameraJson", False),
    ("show_mode", "String", {"type": "string"}, "string", "String!", "string", False),
    ("selected_generation_id", "Option<String>", {"type": "string"}, "string", "String", "optional string", True),
]

CFG3 = [
    ("selected_node_ids", "Vec<String>", {"type": "array", "items": {"type": "string"}}, "string[]", "[String!]!", "repeated string", False),
    ("lod_mode", "String", {"type": "string"}, "string", "String!", "string", False),
    ("show_mode", "String", {"type": "string"}, "string", "String!", "string", False),
    ("selection_method", "String", {"type": "string"}, "string", "String!", "string", False),
    ("hovered_node_id", "Option<String>", {"type": "string"}, "string", "String", "optional string", True),
    ("camera", "CameraJson", {"$ref": "#/$defs/CameraJson"}, "CameraJson", "CameraJson!", "CameraJson", False),
    ("preview_camera", "Procedural3dPreviewCamera", {"$ref": "#/$defs/Procedural3dPreviewCamera"}, "Procedural3dPreviewCamera", "Procedural3dPreviewCamera!", "Procedural3dPreviewCamera", False),
    ("sun_json", "String", {"type": "string"}, "string", "String!", "string", False),
    ("selected_generation_id", "Option<String>", {"type": "string"}, "string", "String", "optional string", True),
    ("generation_preview_text", "Option<String>", {"type": "string"}, "string", "String", "optional string", True),
    ("active_utility_id", "String", {"type": "string"}, "string", "String!", "string", False),
    ("locale", "String", {"type": "string"}, "string", "String!", "string", False),
    ("contributions_json", "String", {"type": "string"}, "string", "String!", "string", False),
]

PRES3 = [
    ("selected_node_ids", "Vec<String>", {"type": "array", "items": {"type": "string"}}, "string[]", "[String!]!", "repeated string", False),
    ("hovered_node_id", "Option<String>", {"type": "string"}, "string", "String", "optional string", True),
    ("camera", "CameraJson", {"$ref": "#/$defs/CameraJson"}, "CameraJson", "CameraJson!", "CameraJson", False),
    ("preview_camera", "Procedural3dPreviewCamera", {"$ref": "#/$defs/Procedural3dPreviewCamera"}, "Procedural3dPreviewCamera", "Procedural3dPreviewCamera!", "Procedural3dPreviewCamera", False),
    ("selection_method", "String", {"type": "string"}, "string", "String!", "string", False),
    ("active_utility_id", "String", {"type": "string"}, "string", "String!", "string", False),
    ("show_mode", "String", {"type": "string"}, "string", "String!", "string", False),
]

# endregion


def snake_to_camel(s: str) -> str:
    parts = s.split("_")
    return parts[0] + "".join(p.title() for p in parts[1:])


def emit_json(title: str, fields, state: str, id_: str, defs: dict | None = None) -> str:
    props = {}
    req = []
    for snake, _rust, jprop, _ts, _gql, _proto, optional in fields:
        camel = snake_to_camel(snake)
        prop = dict(jprop)
        prop["x-semio-state"] = state
        props[camel] = prop
        if not optional:
            req.append(camel)
    doc: dict = {
        "$id": id_,
        "title": title,
        "type": "object",
        "additionalProperties": False,
        "required": req,
        "properties": props,
    }
    if defs:
        doc["$defs"] = defs
    return json.dumps(doc, indent=2) + "\n"


def emit_ts(title: str, fields, state: str, extras: str = "") -> str:
    lines = [f"/** 🧬️ {title} */", f"export interface {title} {{"]
    for snake, _rust, _j, ts, _gql, _proto, optional in fields:
        camel = snake_to_camel(snake)
        opt = "?" if optional else ""
        lines.append(f"  /** @state {state} */")
        lines.append(f"  {camel}{opt}: {ts};")
    lines.append("}")
    body = "\n".join(lines) + "\n"
    if extras:
        body += "\n" + extras
    return body


def emit_gql(title: str, fields, state_enum: str, extras: str = "") -> str:
    lines = [f"type {title} {{"]
    for snake, _rust, _j, _ts, gql, _proto, optional in fields:
        camel = snake_to_camel(snake)
        # gql type already includes ! for required scalars; optional ones omit !
        lines.append(f"  {camel}: {gql} @state(class: {state_enum})")
    lines.append("}")
    body = "\n".join(lines) + "\n"
    if extras:
        body += extras
    return body


def emit_proto(package: str, title: str, fields, state: str, extras: str = "") -> str:
    lines = [
        'syntax = "proto3";',
        f"package {package};",
        f"message {title} {{",
    ]
    n = 1
    for snake, _rust, _j, _ts, _gql, proto, _optional in fields:
        lines.append(f"  // @state {state}")
        lines.append(f"  {proto} {snake} = {n};")
        n += 1
    lines.append("}")
    body = "\n".join(lines) + "\n"
    if extras:
        body += extras
    return body


def emit_schema_rs(
    title: str,
    schema_id: str,
    fields,
    state_attr: str,
    imports: list[str],
    helpers: str = "",
    with_default: bool = False,
) -> str:
    derives = "Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema"
    if with_default:
        derives = "Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema"
    lines = [
        "//! 🧬️ schema leaf",
        "use schema::ArtifactSchema;",
        "use serde::{Deserialize, Serialize};",
    ]
    for imp in imports:
        lines.append(imp)
    lines.append("")
    if helpers:
        lines.append(helpers.rstrip())
        lines.append("")
    serde = '#[serde(rename_all = "camelCase"' + (", default)]" if with_default else ")]")
    lines += [
        f"#[derive({derives})]",
        serde,
        f'#[artifact_schema(id = "{schema_id}")]',
        f"pub struct {title} {{",
    ]
    for snake, rust, _j, _ts, _gql, _proto, _optional in fields:
        lines.append(f"    #[state({state_attr})] pub {snake}: {rust},")
    lines.append("}")
    lines.append("")
    return "\n".join(lines) + "\n"


PRESENCE_RUNTIME_TEMPLATE = '''//! 👥️ {title} — shareable live ephemeral state + mutations.
//!
//! {doc}

use flow::CameraJson;
use protocol::Mutation;
use serde::{{Deserialize, Serialize}};
use store::DocumentPack;
{extra_imports}

//#region 🔖️Presence
/// 👥️ {doc_short}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "{extension}")]
#[dsl(layout = "lines")]
pub struct {title} {{
{fields}
}}

impl Default for {title} {{
    fn default() -> Self {{
        Self {{
{defaults}
        }}
    }}
}}

impl protocol::MutationDiff<{title}> for {title} {{
    fn apply(&self, _base: &{title}) -> {title} {{
        self.clone()
    }}
    fn absorb(&mut self, other: Self) {{
        *self = other;
    }}
}}

impl store::DocumentDsl for {title} {{
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {{
        Self::__DSL_ENVELOPE_ID
    }}
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {{
        let body = match store::semio_format::split_text_preamble(text) {{
            Ok((_, rest)) => rest,
            Err(_) => text,
        }};
        if body.trim().is_empty() {{
            return Ok(Self::default());
        }}
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions {{ limits: dsl::Limits::default(), mode: dsl::SourceMode::Document }},
        )?;
        Self::__dsl_from_record(&record)
    }}
    fn print_dsl(&self) -> String {{
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }}
}}

impl DocumentPack for {title} {{
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {{
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }}
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {{
        if bytes.is_empty() {{
            return Ok(Self::default());
        }}
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {{
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {{}}, got {{}}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }}
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }}
    fn record_spec() -> Option<dsl::RecordSpec> {{
        Some(Self::__dsl_spec())
    }}
}}
//#endregion 🔖️Presence

//#region 🔖️PresenceMutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(rename_all = "camelCase")]
pub enum {mutation} {{
    #[dsl(key = "snapshot")]
    Snapshot {{
        #[dsl(block)]
        presence: {title},
    }},
}}

impl Mutation<{title}> for {mutation} {{
    type Diff = {title};

    fn diff(&self, _base: &{title}) -> {title} {{
        match self {{
            Self::Snapshot {{ presence }} => presence.clone(),
        }}
    }}

    fn inverse(&self, base: &{title}) -> Vec<Self> {{
        vec![Self::Snapshot {{ presence: base.clone() }}]
    }}
}}

impl protocol::OpText for {mutation} {{
    fn parse_op(line: &str) -> Result<Self, store::TextError> {{
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {{
            let probe = format!("{{keyword}} ");
            if line == keyword.as_str() || line.starts_with(&probe) {{
                let body = if line.len() > keyword.len() {{
                    line[keyword.len()..].trim_start()
                }} else {{
                    ""
                }};
                let record = dsl::parse(
                    body,
                    &spec_fn(),
                    &dsl::ParseOptions {{
                        limits: dsl::Limits::default(),
                        mode: dsl::SourceMode::Inline,
                    }},
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }}
        }}
        Err(dsl::__rt::field_error(format!("unknown operation line '{{line}}'")))
    }}
    fn print_op(&self) -> String {{
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants
            .iter()
            .find(|(k, _)| k == &keyword)
            .map(|(_, s)| *s)
            .expect("variant spec must exist for its own keyword");
        let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);
        if body.is_empty() {{
            keyword
        }} else {{
            format!("{{keyword}} {{body}}")
        }}
    }}
}}

impl protocol::OpBinary for {mutation} {{
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {{
        dsl::variants_binary::encode_op(self)
    }}
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {{
        dsl::variants_binary::decode_op(bytes)
    }}
}}
//#endregion 🔖️PresenceMutation
'''


def emit_presence_runtime_2d() -> str:
    fields = """    /// 👁️ Selected widget ids.
    pub selected_ids: Vec<String>,
    /// 🗺️ The node-graph camera.
    #[dsl(block)]
    pub camera: CameraJson,
    /// 👁️ Display mode (`"preview"`/`"generate"`/`"wire"`).
    pub show_mode: String,
    /// 👁️ Active generation selection.
    pub selected_generation_id: Option<String>,"""
    defaults = """            selected_ids: Vec::new(),
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            show_mode: "preview".into(),
            selected_generation_id: None,"""
    return PRESENCE_RUNTIME_TEMPLATE.format(
        title="Procedural2dPresence",
        mutation="Procedural2dPresenceMutation",
        extension="procedural2d.presence",
        doc="Shareable live subset of the 2d procedural surface: selection, graph camera, show-mode, generation pick.",
        doc_short="Shareable live subset of procedural 2d view state (selection, camera, show-mode, generation).",
        extra_imports="",
        fields=fields,
        defaults=defaults,
    )


def emit_presence_runtime_3d() -> str:
    fields = """    /// 👁️ Selected flow-graph widget ids.
    pub selected_node_ids: Vec<String>,
    /// 👁️ Hovered flow-graph widget id.
    pub hovered_node_id: Option<String>,
    /// 📷️ The flow-graph node canvas camera.
    #[dsl(block)]
    pub camera: CameraJson,
    /// 📷️ The 3D preview viewport camera.
    #[dsl(block)]
    pub preview_camera: Procedural3dPreviewCamera,
    /// 🖱️ Marquee selection method.
    pub selection_method: String,
    /// 🧰 Active utility id.
    pub active_utility_id: String,
    /// 👁️ Preview shading mode.
    pub show_mode: String,"""
    defaults = """            selected_node_ids: Vec::new(),
            hovered_node_id: None,
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            preview_camera: Procedural3dPreviewCamera::default(),
            selection_method: "rectangle".into(),
            active_utility_id: String::new(),
            show_mode: "shaded".into(),"""
    return PRESENCE_RUNTIME_TEMPLATE.format(
        title="Procedural3dPresence",
        mutation="Procedural3dPresenceMutation",
        extension="procedural3d.presence",
        doc="Shareable live subset of the 3d procedural surface: selection, hover, cameras, utility, show-mode.",
        doc_short="Shareable live subset of procedural 3d view state (selection, hover, cameras, utility).",
        extra_imports="use crate::apps::procedural3d::config::Procedural3dPreviewCamera;\n",
        fields=fields,
        defaults=defaults,
    )


def write_facet(
    base: Path,
    title: str,
    slug: str,
    facet: str,
    fields,
    state: str,
    state_attr: str,
    state_enum: str,
    schema_id: str,
    imports: list[str],
    defs: dict | None,
    extras_ts: str,
    extras_gql: str,
    extras_proto: str,
    helpers_rs: str = "",
    with_default: bool = False,
) -> None:
    schema = base / "🧬️schema"
    package = f"semio.app.procedural.{slug}"
    id_ = f"https://semio.tech/schema/app/procedural/{slug}/{facet}.json"
    write(schema / "🔣️component.json", emit_json(title, fields, state, id_, defs))
    write(schema / "🟦️component.ts", emit_ts(title, fields, state, extras_ts))
    write(schema / "🔗️component.graphql", emit_gql(title, fields, state_enum, extras_gql))
    write(schema / "🛰️component.proto", emit_proto(package, title, fields, state, extras_proto))
    write(
        schema / "🦀️component.rs",
        emit_schema_rs(title, schema_id, fields, state_attr, imports, helpers_rs, with_default),
    )


def main() -> None:
    # Fix app3 path (emoji ice)
    global APP3
    apps = PLUGIN / "🎛️apps"
    APP3 = next(p for p in apps.iterdir() if p.name.endswith("3d"))
    print("APP3", APP3)

    cam_defs = {"CameraJson": CAMERA_JSON_DEF}
    cam_preview_defs = {**cam_defs, "Procedural3dPreviewCamera": PREVIEW_CAM_DEF}

    # --- 2d config ---
    write_facet(
        APP2 / "🎚️config",
        "Procedural2dConfig",
        "2d",
        "config",
        CFG2,
        "local-ui",
        "local_ui",
        "LOCAL_UI",
        "s.procedural.2d.config",
        ["use flow::CameraJson;"],
        cam_defs,
        CAMERA_TS,
        CAMERA_GQL,
        CAMERA_PROTO,
    )

    # --- 2d presence schema ---
    write_facet(
        APP2 / "👥️presence",
        "Procedural2dPresence",
        "2d",
        "presence",
        PRES2,
        "shared-ui",
        "shared_ui",
        "SHARED_UI",
        "s.procedural.2d.presence",
        ["use flow::CameraJson;"],
        cam_defs,
        CAMERA_TS,
        CAMERA_GQL,
        CAMERA_PROTO,
        with_default=True,
    )
    write(APP2 / "👥️presence" / "🦀️component.rs", emit_presence_runtime_2d())

    # --- 3d config ---
    preview_helper = ""  # import from super
    write_facet(
        APP3 / "🎚️config",
        "Procedural3dConfig",
        "3d",
        "config",
        CFG3,
        "local-ui",
        "local_ui",
        "LOCAL_UI",
        "s.procedural.3d.config",
        ["use flow::CameraJson;", "use super::Procedural3dPreviewCamera;"],
        cam_preview_defs,
        CAMERA_TS + PREVIEW_TS,
        CAMERA_GQL + PREVIEW_GQL,
        CAMERA_PROTO + PREVIEW_PROTO,
    )

    # --- 3d presence ---
    write_facet(
        APP3 / "👥️presence",
        "Procedural3dPresence",
        "3d",
        "presence",
        PRES3,
        "shared-ui",
        "shared_ui",
        "SHARED_UI",
        "s.procedural.3d.presence",
        [
            "use flow::CameraJson;",
            "use crate::apps::procedural3d::config::Procedural3dPreviewCamera;",
        ],
        cam_preview_defs,
        CAMERA_TS + PREVIEW_TS,
        CAMERA_GQL + PREVIEW_GQL,
        CAMERA_PROTO + PREVIEW_PROTO,
        with_default=True,
    )
    write(APP3 / "👥️presence" / "🦀️component.rs", emit_presence_runtime_3d())


if __name__ == "__main__":
    main()
