#!/usr/bin/env python3
"""🧪 Generate procedural2d/procedural3d fifteen-leaf facet schemas (wave 5)."""

from __future__ import annotations

import json
import os
from pathlib import Path

PLUGIN = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural")


def write(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text if text.endswith("\n") else text + "\n", encoding="utf-8")


# ---------------------------------------------------------------------------
# Shared nested type defs (FlowFixture / GenerationPlayState / CameraJson / …)
# ---------------------------------------------------------------------------

CAMERA_JSON_JSON = {
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

WIDGET_LAYOUT_JSON = {
    "title": "WidgetLayout",
    "type": "object",
    "additionalProperties": False,
    "required": ["x", "y"],
    "properties": {
        "x": {"type": "number", "format": "double"},
        "y": {"type": "number", "format": "double"},
    },
}

# Widget / SynapseSpec / FlowFixture are complex; document as opaque JSON blobs inside
# FlowFixture fields that are themselves records. For parity we name the record types and
# use json-blob scalars for deeply polymorphic Widget payloads.

SYNAPSE_SPEC_JSON = {
    "title": "SynapseSpec",
    "type": "object",
    "additionalProperties": False,
    "required": ["id", "from", "to", "fromPort", "toPort"],
    "properties": {
        "id": {"type": "string"},
        "from": {"type": "string"},
        "to": {"type": "string"},
        "fromPort": {"type": "string"},
        "toPort": {"type": "string"},
    },
}

# Widget is a tagged union — represent as a JSON blob string for schema documentation,
# matching §6 json blob scalar. Runtime Rust still uses flow::Widget.
WIDGET_JSON = {
    "title": "Widget",
    "type": "string",
    "contentMediaType": "application/json",
}

FLOW_FIXTURE_JSON = {
    "title": "FlowFixture",
    "type": "object",
    "additionalProperties": False,
    "required": ["schema", "camera", "widgets", "synapses", "layout"],
    "properties": {
        "schema": {"type": "string"},
        "camera": {"$ref": "#/$defs/CameraJson"},
        "widgets": {
            "type": "array",
            "items": {"$ref": "#/$defs/Widget"},
        },
        "synapses": {
            "type": "array",
            "items": {"$ref": "#/$defs/SynapseSpec"},
        },
        "layout": {
            "type": "object",
            "additionalProperties": {"$ref": "#/$defs/WidgetLayout"},
        },
    },
}

FORM_GENERATION_JSON = {
    "title": "FormGeneration",
    "type": "object",
    "additionalProperties": False,
    "required": ["id", "name", "valuesJson"],
    "properties": {
        "id": {"type": "string"},
        "name": {"type": "string"},
        "valuesJson": {"type": "string", "contentMediaType": "application/json"},
    },
}

GENERATION_PLAY_STATE_JSON = {
    "title": "GenerationPlayState",
    "type": "object",
    "additionalProperties": False,
    "required": ["generations"],
    "properties": {
        "generations": {
            "type": "array",
            "items": {"$ref": "#/$defs/FormGeneration"},
        },
        "selectedGenerationId": {"type": "string"},
        "previewText": {"type": "string"},
    },
}

COMMON_DEFS = {
    "CameraJson": CAMERA_JSON_JSON,
    "WidgetLayout": WIDGET_LAYOUT_JSON,
    "SynapseSpec": SYNAPSE_SPEC_JSON,
    "Widget": WIDGET_JSON,
    "FlowFixture": FLOW_FIXTURE_JSON,
    "FormGeneration": FORM_GENERATION_JSON,
    "GenerationPlayState": GENERATION_PLAY_STATE_JSON,
}

PREVIEW_CAMERA_JSON = {
    "title": "Procedural3dPreviewCamera",
    "type": "object",
    "additionalProperties": False,
    "required": [
        "positionX",
        "positionY",
        "positionZ",
        "targetX",
        "targetY",
        "targetZ",
        "fov",
    ],
    "properties": {
        "positionX": {"type": "number", "format": "double"},
        "positionY": {"type": "number", "format": "double"},
        "positionZ": {"type": "number", "format": "double"},
        "targetX": {"type": "number", "format": "double"},
        "targetY": {"type": "number", "format": "double"},
        "targetZ": {"type": "number", "format": "double"},
        "fov": {"type": "number", "format": "double"},
    },
}


def string_list_def(title: str) -> dict:
    return {
        "title": title,
        "type": "object",
        "additionalProperties": False,
        "required": ["values"],
        "properties": {"values": {"type": "array", "items": {"type": "string"}}},
    }


# ---------------------------------------------------------------------------
# Field inventories
# ---------------------------------------------------------------------------

# (camelName, rust_name, state, rust_type, ts_type, gql_type, json_prop, proto_type, required)
# json_prop is a JSON Schema property fragment WITHOUT x-semio-state

P2D_PERSISTENT = [
    (
        "fixture",
        "fixture",
        "persistent",
        "FlowFixture",
        "FlowFixture",
        "FlowFixture!",
        {"$ref": "#/$defs/FlowFixture"},
        "FlowFixture",
        True,
    ),
    (
        "generation",
        "generation",
        "persistent",
        "GenerationPlayState",
        "GenerationPlayState",
        "GenerationPlayState!",
        {"$ref": "#/$defs/GenerationPlayState"},
        "GenerationPlayState",
        True,
    ),
]

P2D_UI = [
    (
        "selectedIds",
        "selected_ids",
        "shared-ui",
        "Vec<String>",
        "string[]",
        "[String!]!",
        {"type": "array", "items": {"type": "string"}},
        "string",  # repeated
        True,
        "list",
    ),
    (
        "graphCamera",
        "graph_camera",
        "local-ui",
        "CameraJson",
        "CameraJson",
        "CameraJson!",
        {"$ref": "#/$defs/CameraJson"},
        "CameraJson",
        True,
        "scalar",
    ),
    (
        "showMode",
        "show_mode",
        "local-ui",
        "String",
        "string",
        "String!",
        {"type": "string"},
        "string",
        True,
        "scalar",
    ),
    (
        "selectedGenerationId",
        "selected_generation_id",
        "shared-ui",
        "Option<String>",
        "string",
        "String",
        {"type": "string"},
        "string",
        False,
        "scalar",
    ),
    (
        "generationPreviewText",
        "generation_preview_text",
        "preview",
        "Option<String>",
        "string",
        "String",
        {"type": "string"},
        "string",
        False,
        "scalar",
    ),
    (
        "locale",
        "locale",
        "local-ui",
        "String",
        "string",
        "String!",
        {"type": "string"},
        "string",
        True,
        "scalar",
    ),
]

P3D_PERSISTENT = list(P2D_PERSISTENT)  # same persistent shape

P3D_UI = [
    (
        "selectedNodeIds",
        "selected_node_ids",
        "shared-ui",
        "Vec<String>",
        "string[]",
        "[String!]!",
        {"type": "array", "items": {"type": "string"}},
        "string",
        True,
        "list",
    ),
    (
        "lodMode",
        "lod_mode",
        "local-ui",
        "String",
        "string",
        "String!",
        {"type": "string"},
        "string",
        True,
        "scalar",
    ),
    (
        "showMode",
        "show_mode",
        "local-ui",
        "String",
        "string",
        "String!",
        {"type": "string"},
        "string",
        True,
        "scalar",
    ),
    (
        "selectionMethod",
        "selection_method",
        "local-ui",
        "String",
        "string",
        "String!",
        {"type": "string"},
        "string",
        True,
        "scalar",
    ),
    (
        "hoveredNodeId",
        "hovered_node_id",
        "preview",
        "Option<String>",
        "string",
        "String",
        {"type": "string"},
        "string",
        False,
        "scalar",
    ),
    (
        "graphCamera",
        "graph_camera",
        "local-ui",
        "CameraJson",
        "CameraJson",
        "CameraJson!",
        {"$ref": "#/$defs/CameraJson"},
        "CameraJson",
        True,
        "scalar",
    ),
    (
        "previewCamera",
        "preview_camera",
        "local-ui",
        "Procedural3dPreviewCamera",
        "Procedural3dPreviewCamera",
        "Procedural3dPreviewCamera!",
        {"$ref": "#/$defs/Procedural3dPreviewCamera"},
        "Procedural3dPreviewCamera",
        True,
        "scalar",
    ),
    (
        "sunJson",
        "sun_json",
        "local-ui",
        "String",
        "string",
        "String!",
        {"type": "string", "contentMediaType": "application/json"},
        "string",
        True,
        "scalar",
    ),
    (
        "selectedGenerationId",
        "selected_generation_id",
        "shared-ui",
        "Option<String>",
        "string",
        "String",
        {"type": "string"},
        "string",
        False,
        "scalar",
    ),
    (
        "generationPreviewText",
        "generation_preview_text",
        "preview",
        "Option<String>",
        "string",
        "String",
        {"type": "string"},
        "string",
        False,
        "scalar",
    ),
    (
        "activeUtilityId",
        "active_utility_id",
        "shared-ui",
        "String",
        "string",
        "String!",
        {"type": "string"},
        "string",
        True,
        "scalar",
    ),
    (
        "locale",
        "locale",
        "local-ui",
        "String",
        "string",
        "String!",
        {"type": "string"},
        "string",
        True,
        "scalar",
    ),
    (
        "contributionsJson",
        "contributions_json",
        "local-ui",
        "String",
        "string",
        "String!",
        {"type": "string", "contentMediaType": "application/json"},
        "string",
        True,
        "scalar",
    ),
]


def state_attr(state: str) -> str:
    return state.replace("-", "_")


def state_gql(state: str) -> str:
    return state.upper().replace("-", "_")


def normalize_fields(rows):
    out = []
    for row in rows:
        if len(row) == 9:
            camel, rust, state, rty, tsty, gql, jprop, proto, required = row
            kind = "list" if rty.startswith("Vec<") else "scalar"
            out.append(
                {
                    "camel": camel,
                    "rust": rust,
                    "state": state,
                    "rust_type": rty,
                    "ts_type": tsty,
                    "gql_type": gql,
                    "json_prop": jprop,
                    "proto_type": proto,
                    "required": required,
                    "kind": kind,
                }
            )
        else:
            camel, rust, state, rty, tsty, gql, jprop, proto, required, kind = row
            out.append(
                {
                    "camel": camel,
                    "rust": rust,
                    "state": state,
                    "rust_type": rty,
                    "ts_type": tsty,
                    "gql_type": gql,
                    "json_prop": jprop,
                    "proto_type": proto,
                    "required": required,
                    "kind": kind,
                }
            )
    return out


# ---------------------------------------------------------------------------
# Emitters
# ---------------------------------------------------------------------------


def emit_json_schema(
    title: str,
    schema_id: str,
    fields: list[dict],
    defs: dict,
    extra_required: list[str] | None = None,
) -> str:
    required = [f["camel"] for f in fields if f["required"]]
    if extra_required:
        required = extra_required + required
    properties = {}
    for f in fields:
        prop = dict(f["json_prop"])
        prop["x-semio-state"] = f["state"]
        properties[f["camel"]] = prop
    doc = {
        "$id": schema_id,
        "title": title,
        "type": "object",
        "additionalProperties": False,
        "required": required,
        "properties": properties,
        "$defs": defs,
    }
    return json.dumps(doc, indent=2, ensure_ascii=False) + "\n"


def emit_rust_struct(
    doc: str,
    title: str,
    schema_id: str,
    fields: list[dict],
    imports: str,
    extra_attrs: str = "",
    first_helpers: str = "",
    after: str = "",
    serde_default: bool = False,
) -> str:
    serde = '#[serde(rename_all = "camelCase"'
    if serde_default:
        serde += ", default"
    serde += ")]"
    lines = [
        doc,
        "",
        imports,
        "",
        first_helpers,
        f"//#region 🔖️{title}",
        f"/// 🧬️ {title} facet type.",
        "#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]",
        serde,
        f'#[artifact_schema(id = "{schema_id}")]',
        extra_attrs,
        f"pub struct {title} {{",
    ]
    for f in fields:
        lines.append(f'    #[state({state_attr(f["state"])})] pub {f["rust"]}: {f["rust_type"]},')
    lines.append("}")
    lines.append(f"//#endregion 🔖️{title}")
    if after:
        lines.append("")
        lines.append(after)
    # filter empty
    text = "\n".join(line for line in lines if line is not None)
    while "\n\n\n" in text:
        text = text.replace("\n\n\n", "\n\n")
    return text.rstrip() + "\n"


def emit_ts_interface(doc: str, title: str, fields: list[dict], helpers: str = "") -> str:
    lines = [doc, "", helpers, f"export interface {title} {{"]
    for f in fields:
        opt = "?" if not f["required"] else ""
        # Option<String> in rust → optional prop; Option types already marked required=False
        lines.append(f'  /** @state {f["state"]} */')
        lines.append(f'  {f["camel"]}{opt}: {f["ts_type"]};')
    lines.append("}")
    text = "\n".join(line for line in lines if line is not None)
    while "\n\n\n" in text:
        text = text.replace("\n\n\n", "\n\n")
    return text.rstrip() + "\n"


def emit_graphql(doc: str, title: str, fields: list[dict], helpers: str = "") -> str:
    lines = [doc, "", helpers, f"type {title} {{"]
    for f in fields:
        lines.append(f'  {f["camel"]}: {f["gql_type"]} @state(class: {state_gql(f["state"])})')
    lines.append("}")
    text = "\n".join(line for line in lines if line is not None)
    while "\n\n\n" in text:
        text = text.replace("\n\n\n", "\n\n")
    return text.rstrip() + "\n"


def emit_proto(doc: str, package: str, title: str, fields: list[dict], helpers: str = "") -> str:
    lines = [
        'syntax = "proto3";',
        f"package {package};",
        "",
        doc,
        "",
        helpers,
        f"message {title} {{",
    ]
    n = 1
    for f in fields:
        lines.append(f'  // @state {f["state"]}')
        if f["kind"] == "list":
            lines.append(f'  repeated {f["proto_type"]} {f["rust"]} = {n};')
        elif not f["required"]:
            lines.append(f'  optional {f["proto_type"]} {f["rust"]} = {n};')
        else:
            lines.append(f'  {f["proto_type"]} {f["rust"]} = {n};')
        n += 1
    lines.append("}")
    text = "\n".join(line for line in lines if line is not None)
    while "\n\n\n" in text:
        text = text.replace("\n\n\n", "\n\n")
    return text.rstrip() + "\n"


def diff_fields_from_artifact(fields: list[dict], prefix: str, artifact_type: str) -> list[dict]:
    """§7.3 sparse entries + artifact replacement."""
    out = [
        {
            "camel": "artifact",
            "rust": "artifact",
            "state": "persistent",
            "rust_type": f"Option<Box<{artifact_type}>>",
            "ts_type": artifact_type,
            "gql_type": artifact_type,
            "json_prop": {"$ref": f"#/$defs/{artifact_type}"},
            "proto_type": artifact_type,
            "required": False,
            "kind": "scalar",
        }
    ]
    for f in fields:
        if f["state"] == "effect":
            continue
        camel = f["camel"]
        rust = f["rust"]
        state = f["state"]
        # optional list → named scalar wrapper
        if f["kind"] == "list":
            wrapper = f"{prefix}StringList"
            out.append(
                {
                    "camel": camel,
                    "rust": rust,
                    "state": state,
                    "rust_type": f"Option<{wrapper}>",
                    "ts_type": wrapper,
                    "gql_type": wrapper,
                    "json_prop": {"$ref": f"#/$defs/{wrapper}"},
                    "proto_type": wrapper,
                    "required": False,
                    "kind": "scalar",
                }
            )
            continue
        # Option<T> artifact field → Option<Option<T>> in diff
        if f["rust_type"].startswith("Option<"):
            inner = f["rust_type"][len("Option<") : -1]
            # JSON oneOf null|T
            inner_json = dict(f["json_prop"])
            out.append(
                {
                    "camel": camel,
                    "rust": rust,
                    "state": state,
                    "rust_type": f"Option<Option<{inner}>>",
                    "ts_type": f"{f['ts_type']} | null",
                    "gql_type": f["gql_type"].rstrip("!"),  # already optional
                    "json_prop": {"oneOf": [{"type": "null"}, inner_json]},
                    "proto_type": f["proto_type"],
                    "required": False,
                    "kind": "scalar",
                }
            )
            continue
        # plain T → Option<T>
        out.append(
            {
                "camel": camel,
                "rust": rust,
                "state": state,
                "rust_type": f"Option<{f['rust_type']}>",
                "ts_type": f["ts_type"],
                "gql_type": f["gql_type"].rstrip("!"),
                "json_prop": dict(f["json_prop"]),
                "proto_type": f["proto_type"],
                "required": False,
                "kind": "scalar",
            }
        )
    return out


HELPER_TS = """
export type CameraJson = { x: number; y: number; zoom: number };
export type WidgetLayout = { x: number; y: number };
export type SynapseSpec = { id: string; from: string; to: string; fromPort: string; toPort: string };
/** @description Polymorphic flow widget — JSON blob. */
export type Widget = string;
export type FlowFixture = {
  schema: string;
  camera: CameraJson;
  widgets: Widget[];
  synapses: SynapseSpec[];
  layout: Record<string, WidgetLayout>;
};
export type FormGeneration = { id: string; name: string; valuesJson: string };
export type GenerationPlayState = {
  generations: FormGeneration[];
  selectedGenerationId?: string;
  previewText?: string;
};
""".strip()

HELPER_GQL = """
type CameraJson {
  x: Float!
  y: Float!
  zoom: Float!
}
type WidgetLayout {
  x: Float!
  y: Float!
}
type SynapseSpec {
  id: String!
  from: String!
  to: String!
  fromPort: String!
  toPort: String!
}
scalar Widget
type FlowFixture {
  schema: String!
  camera: CameraJson!
  widgets: [Widget!]!
  synapses: [SynapseSpec!]!
  layout: [FlowFixtureLayoutEntry!]!
}
type FlowFixtureLayoutEntry {
  key: String!
  value: WidgetLayout!
}
type FormGeneration {
  id: String!
  name: String!
  valuesJson: String!
}
type GenerationPlayState {
  generations: [FormGeneration!]!
  selectedGenerationId: String
  previewText: String
}
""".strip()

HELPER_PROTO = """
message CameraJson {
  double x = 1;
  double y = 2;
  double zoom = 3;
}
message WidgetLayout {
  double x = 1;
  double y = 2;
}
message SynapseSpec {
  string id = 1;
  string from = 2;
  string to = 3;
  string from_port = 4;
  string to_port = 5;
}
message Widget {
  string json = 1;
}
message FlowFixture {
  string schema = 1;
  CameraJson camera = 2;
  repeated Widget widgets = 3;
  repeated SynapseSpec synapses = 4;
  map<string, WidgetLayout> layout = 5;
}
message FormGeneration {
  string id = 1;
  string name = 2;
  string values_json = 3;
}
message GenerationPlayState {
  repeated FormGeneration generations = 1;
  optional string selected_generation_id = 2;
  optional string preview_text = 3;
}
""".strip()

HELPER_GQL_P3D_CAM = """
type Procedural3dPreviewCamera {
  positionX: Float!
  positionY: Float!
  positionZ: Float!
  targetX: Float!
  targetY: Float!
  targetZ: Float!
  fov: Float!
}
""".strip()

HELPER_TS_P3D_CAM = """
export type Procedural3dPreviewCamera = {
  positionX: number;
  positionY: number;
  positionZ: number;
  targetX: number;
  targetY: number;
  targetZ: number;
  fov: number;
};
""".strip()

HELPER_PROTO_P3D_CAM = """
message Procedural3dPreviewCamera {
  double position_x = 1;
  double position_y = 2;
  double position_z = 3;
  double target_x = 4;
  double target_y = 5;
  double target_z = 6;
  double fov = 7;
}
""".strip()


def gen_artifact(
    key: str,
    folder: str,
    prefix: str,
    plugin_key: str,
    persistent: list,
    ui: list,
    extra_defs: dict | None = None,
    extra_ts: str = "",
    extra_gql: str = "",
    extra_proto: str = "",
    rust_preview_type: str | None = None,
) -> None:
    art_dir = PLUGIN / "🗿️artifacts" / folder
    schema_id = f"s.{plugin_key}.{key}"
    artifact_type = f"{prefix}Artifact"
    snapshot_type = f"{prefix}Snapshot"
    diff_type = f"{prefix}Diff"
    string_list = f"{prefix}StringList"

    persist = normalize_fields(persistent)
    ui_fields = normalize_fields(ui)
    artifact_fields = persist + ui_fields

    defs = dict(COMMON_DEFS)
    if extra_defs:
        defs.update(extra_defs)
    defs[string_list] = string_list_def(string_list)
    defs[artifact_type] = {
        "title": artifact_type,
        "type": "object",
        "additionalProperties": False,
        # stub for $ref from diff — full shape is the root schema
        "properties": {},
    }

    # --- snapshot ---
    snap_fields = persist
    snap_json = emit_json_schema(
        snapshot_type,
        f"https://semio.tech/schema/s/{plugin_key}/{key}/snapshot.json",
        snap_fields,
        {k: defs[k] for k in COMMON_DEFS} | ({k: extra_defs[k] for k in (extra_defs or {})}),
    )
    # Fix: ArtifactSchema on Snapshot — need Default etc. in after block later by hand for codecs.
    snap_rust = emit_rust_struct(
        f"//! 🧬️ {prefix} snapshot schema — persistent fields only.",
        snapshot_type,
        schema_id,
        [
            {
                **f,
                # Runtime uses real flow types — Widget is NOT a String.
                "rust_type": (
                    "flow::FlowFixture"
                    if f["rust"] == "fixture"
                    else "flow::playbook::GenerationPlayState"
                    if f["rust"] == "generation"
                    else f["rust_type"]
                ),
            }
            for f in snap_fields
        ],
        "\n".join(
            [
                "use schema::ArtifactSchema;",
                "use serde::{Deserialize, Serialize};",
            ]
        ),
        after=f"""
impl Default for {snapshot_type} {{
    fn default() -> Self {{
        Self {{
            fixture: flow::FlowFixture::default(),
            generation: flow::playbook::GenerationPlayState::default(),
        }}
    }}
}}
""",
    )
    # NOTE: For schema leaf field extractor, Rust type `flow::FlowFixture` becomes scalar
    # `flow::FlowFixture` which WON'T match JSON `FlowFixture`. Must use bare FlowFixture
    # with a use import!
    snap_rust = emit_rust_struct(
        f"//! 🧬️ {prefix} snapshot schema — persistent fields only.",
        snapshot_type,
        schema_id,
        snap_fields,  # FlowFixture / GenerationPlayState bare names
        "\n".join(
            [
                "use flow::FlowFixture;",
                "use flow::playbook::GenerationPlayState;",
                "use schema::ArtifactSchema;",
                "use serde::{Deserialize, Serialize};",
            ]
        ),
        after=f"""
impl Default for {snapshot_type} {{
    fn default() -> Self {{
        Self {{
            fixture: FlowFixture::default(),
            generation: GenerationPlayState::default(),
        }}
    }}
}}
""",
    )

    snap_ts = emit_ts_interface(
        f"/** 🧬️ {prefix} snapshot schema — persistent fields only. */",
        snapshot_type,
        snap_fields,
        HELPER_TS + ("\n" + extra_ts if extra_ts else ""),
    )
    snap_gql = emit_graphql(
        f"# 🧬️ {prefix} snapshot schema — persistent fields only.",
        snapshot_type,
        snap_fields,
        HELPER_GQL + ("\n" + extra_gql if extra_gql else ""),
    )
    snap_proto = emit_proto(
        f"// 🧬️ {prefix} snapshot schema — persistent fields only.",
        f"semio.s.{plugin_key}.{key}.snapshot",
        snapshot_type,
        snap_fields,
        HELPER_PROTO + ("\n" + extra_proto if extra_proto else ""),
    )

    write(art_dir / "📸️snapshot" / "🧬️schema" / "🔣️component.json", snap_json)
    write(art_dir / "📸️snapshot" / "🧬️schema" / "🦀️component.rs", snap_rust)
    write(art_dir / "📸️snapshot" / "🧬️schema" / "🟦️component.ts", snap_ts)
    write(art_dir / "📸️snapshot" / "🧬️schema" / "🔗️component.graphql", snap_gql)
    write(art_dir / "📸️snapshot" / "🧬️schema" / "🛰️component.proto", snap_proto)

    # --- artifact ---
    art_json_defs = dict(defs)
    # don't need self-ref stub in artifact root
    art_json_defs.pop(artifact_type, None)
    art_json = emit_json_schema(
        artifact_type,
        f"https://semio.tech/schema/s/{plugin_key}/{key}/artifact.json",
        artifact_fields,
        art_json_defs,
    )

    rust_ui = []
    for f in artifact_fields:
        rf = dict(f)
        if f["rust"] == "graph_camera":
            rf["rust_type"] = "CameraJson"
        if f["rust"] == "preview_camera" and rust_preview_type:
            rf["rust_type"] = rust_preview_type
        rust_ui.append(rf)

    art_imports = [
        "use crate::artifacts::" + key + f"::snapshot::schema::{snapshot_type};",
        "use flow::CameraJson;",
        "use flow::FlowFixture;",
        "use flow::playbook::GenerationPlayState;",
        "use schema::ArtifactSchema;",
        "use serde::{Deserialize, Serialize};",
    ]
    if rust_preview_type:
        # Procedural3dPreviewCamera lives in apps config today — declare a schema-local twin below
        pass

    preview_helper_rust = ""
    if prefix == "Procedural3d":
        preview_helper_rust = """
//#region 🔖️PreviewCamera
/// 📷️ 3D preview viewport camera (schema twin of the app config record).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural3dPreviewCamera {
    pub position_x: f64,
    pub position_y: f64,
    pub position_z: f64,
    pub target_x: f64,
    pub target_y: f64,
    pub target_z: f64,
    pub fov: f64,
}

impl Default for Procedural3dPreviewCamera {
    fn default() -> Self {
        Self {
            position_x: 4.0,
            position_y: -4.0,
            position_z: 3.0,
            target_x: 0.0,
            target_y: 0.0,
            target_z: 0.0,
            fov: 45.0,
        }
    }
}
//#endregion 🔖️PreviewCamera
"""
        for rf in rust_ui:
            if rf["rust"] == "preview_camera":
                rf["rust_type"] = "Procedural3dPreviewCamera"

    default_body_lines = []
    for f in rust_ui:
        if f["rust"] == "fixture":
            default_body_lines.append("            fixture: FlowFixture::default(),")
        elif f["rust"] == "generation":
            default_body_lines.append("            generation: GenerationPlayState::default(),")
        elif f["rust"] == "selected_ids" or f["rust"] == "selected_node_ids":
            default_body_lines.append(f"            {f['rust']}: Vec::new(),")
        elif f["rust"] == "graph_camera":
            default_body_lines.append("            graph_camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },")
        elif f["rust"] == "preview_camera":
            default_body_lines.append("            preview_camera: Procedural3dPreviewCamera::default(),")
        elif f["rust"] == "show_mode":
            default = '"preview".into()' if prefix == "Procedural2d" else '"shaded".into()'
            default_body_lines.append(f"            show_mode: {default},")
        elif f["rust"] == "lod_mode":
            default_body_lines.append('            lod_mode: String::new(),')
        elif f["rust"] == "selection_method":
            default_body_lines.append('            selection_method: "rectangle".into(),')
        elif f["rust"] == "sun_json":
            default_body_lines.append(
                '            sun_json: serde_json::to_string(&semio_framework_plugin::WorldSunConfig::default()).unwrap_or_default(),'
            )
        elif f["rust"] == "active_utility_id":
            default_body_lines.append('            active_utility_id: "move".into(),')
        elif f["rust"] == "locale":
            default_body_lines.append('            locale: "en-US".into(),')
        elif f["rust"] == "contributions_json":
            default_body_lines.append('            contributions_json: "[]".into(),')
        elif f["rust_type"].startswith("Option<"):
            default_body_lines.append(f"            {f['rust']}: None,")
        elif f["rust_type"] == "String":
            default_body_lines.append(f"            {f['rust']}: String::new(),")
        else:
            default_body_lines.append(f"            {f['rust']}: Default::default(),")

    art_after = preview_helper_rust + f"""
impl Default for {artifact_type} {{
    fn default() -> Self {{
        Self {{
{chr(10).join(default_body_lines)}
        }}
    }}
}}

impl {artifact_type} {{
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> {snapshot_type} {{
        {snapshot_type} {{
            fixture: self.fixture.clone(),
            generation: self.generation.clone(),
        }}
    }}

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: {snapshot_type}) -> Self {{
        Self {{
            fixture: snapshot.fixture,
            generation: snapshot.generation,
            ..Self::default()
        }}
    }}

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: {snapshot_type}) {{
        self.fixture = snapshot.fixture;
        self.generation = snapshot.generation;
    }}
}}

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `{schema_id}` — fifteen handcrafted schema leaves.
pub fn {key}_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {{
    schema::ArtifactSchemaDescriptor {{
        id: "{schema_id}",
        artifact: schema::FacetLeaves {{
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        }},
        snapshot: schema::FacetLeaves {{
            rust: include_str!("../📸️snapshot/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../📸️snapshot/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../📸️snapshot/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../📸️snapshot/🧬️schema/🔣️component.json"),
            proto: include_str!("../📸️snapshot/🧬️schema/🛰️component.proto"),
        }},
        diff: schema::FacetLeaves {{
            rust: include_str!("../🔺️diff/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../🔺️diff/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../🔺️diff/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../🔺️diff/🧬️schema/🔣️component.json"),
            proto: include_str!("../🔺️diff/🧬️schema/🛰️component.proto"),
        }},
    }}
}}
//#endregion 🔖️Descriptor
"""

    art_rust = emit_rust_struct(
        f"//! 🧬️ {prefix} artifact schema — every field of the artifact with its state class.",
        artifact_type,
        schema_id,
        rust_ui,
        "\n".join(art_imports),
        after=art_after,
    )
    # CameraJson import used
    art_ts = emit_ts_interface(
        f"/** 🧬️ {prefix} artifact schema — every field with its state class. */",
        artifact_type,
        artifact_fields,
        HELPER_TS + ("\n" + extra_ts if extra_ts else ""),
    )
    art_gql = emit_graphql(
        f"# 🧬️ {prefix} artifact schema — every field with its state class.",
        artifact_type,
        artifact_fields,
        HELPER_GQL + ("\n" + extra_gql if extra_gql else ""),
    )
    art_proto = emit_proto(
        f"// 🧬️ {prefix} artifact schema — every field with its state class.",
        f"semio.s.{plugin_key}.{key}.artifact",
        artifact_type,
        artifact_fields,
        HELPER_PROTO + ("\n" + extra_proto if extra_proto else ""),
    )

    write(art_dir / "🧬️schema" / "🔣️component.json", art_json)
    write(art_dir / "🧬️schema" / "🦀️component.rs", art_rust)
    write(art_dir / "🧬️schema" / "🟦️component.ts", art_ts)
    write(art_dir / "🧬️schema" / "🔗️component.graphql", art_gql)
    write(art_dir / "🧬️schema" / "🛰️component.proto", art_proto)

    # --- diff ---
    dfields = diff_fields_from_artifact(artifact_fields, prefix, artifact_type)
    # Fix rust types for fixture/generation/cameras
    for df in dfields:
        if df["rust"] == "fixture":
            df["rust_type"] = "Option<FlowFixture>"
        elif df["rust"] == "generation":
            df["rust_type"] = "Option<GenerationPlayState>"
        elif df["rust"] == "graph_camera":
            df["rust_type"] = "Option<CameraJson>"
        elif df["rust"] == "preview_camera":
            df["rust_type"] = "Option<Procedural3dPreviewCamera>"
        elif df["rust"] == "artifact":
            df["rust_type"] = f"Option<Box<crate::artifacts::{key}::schema::{artifact_type}>>"

    diff_defs = dict(defs)
    diff_defs[string_list] = string_list_def(string_list)
    # Full artifact shape for $ref — reuse artifact json properties
    diff_defs[artifact_type] = {
        "title": artifact_type,
        "type": "object",
        "additionalProperties": False,
        "required": [f["camel"] for f in artifact_fields if f["required"]],
        "properties": {
            f["camel"]: dict(f["json_prop"]) for f in artifact_fields
        },
    }

    diff_json = emit_json_schema(
        diff_type,
        f"https://semio.tech/schema/s/{plugin_key}/{key}/diff.json",
        dfields,
        diff_defs,
    )

    string_list_rust = f"""
//#region 🔖️Helpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct {string_list} {{
    pub values: Vec<String>,
}}
//#endregion 🔖️Helpers
"""

    diff_imports = [
        f"use crate::artifacts::{key}::schema::{artifact_type};",
        "use flow::CameraJson;",
        "use flow::FlowFixture;",
        "use flow::playbook::GenerationPlayState;",
        "use schema::ArtifactSchema;",
        "use serde::{Deserialize, Serialize};",
    ]
    if prefix == "Procedural3d":
        diff_imports.insert(
            1, f"use crate::artifacts::{key}::schema::Procedural3dPreviewCamera;"
        )

    # For diff rust, artifact field should use local name with Box — import Artifact
    for df in dfields:
        if df["rust"] == "artifact":
            df["rust_type"] = f"Option<Box<{artifact_type}>>"

    diff_rust = emit_rust_struct(
        f"//! 🧬️ {prefix} diff schema — sparse field delta over the artifact.",
        diff_type,
        schema_id,
        dfields,
        "\n".join(diff_imports),
        serde_default=True,
        first_helpers=string_list_rust,
    )

    diff_ts_helpers = (
        HELPER_TS
        + ("\n" + extra_ts if extra_ts else "")
        + f"\nexport type {string_list} = {{ values: string[] }};\n"
        + f"export interface {artifact_type} {{ /* see artifact facet */ }}\n"
    )
    # Fix ts optional markers — all diff fields optional
    for df in dfields:
        df["required"] = False
        if df["ts_type"].endswith(" | null"):
            pass
        # camel optional via required=False

    diff_ts = emit_ts_interface(
        f"/** 🧬️ {prefix} diff schema — sparse field delta. */",
        diff_type,
        dfields,
        diff_ts_helpers,
    )

    diff_gql_helpers = (
        HELPER_GQL
        + ("\n" + extra_gql if extra_gql else "")
        + f"\ntype {string_list} {{\n  values: [String!]!\n}}\n"
        + f"type {artifact_type} {{\n  fixture: FlowFixture!\n  generation: GenerationPlayState!\n}}\n"
    )
    diff_gql = emit_graphql(
        f"# 🧬️ {prefix} diff schema — sparse field delta.",
        diff_type,
        dfields,
        diff_gql_helpers,
    )

    diff_proto_helpers = (
        HELPER_PROTO
        + ("\n" + extra_proto if extra_proto else "")
        + f"\nmessage {string_list} {{\n  repeated string values = 1;\n}}\n"
        + f"message {artifact_type} {{\n  FlowFixture fixture = 1;\n  GenerationPlayState generation = 2;\n}}\n"
    )
    # Reset required for proto (all optional in diff)
    for df in dfields:
        df["required"] = False
    diff_proto = emit_proto(
        f"// 🧬️ {prefix} diff schema — sparse field delta.",
        f"semio.s.{plugin_key}.{key}.diff",
        diff_type,
        dfields,
        diff_proto_helpers,
    )

    write(art_dir / "🔺️diff" / "🧬️schema" / "🔣️component.json", diff_json)
    write(art_dir / "🔺️diff" / "🧬️schema" / "🦀️component.rs", diff_rust)
    write(art_dir / "🔺️diff" / "🧬️schema" / "🟦️component.ts", diff_ts)
    write(art_dir / "🔺️diff" / "🧬️schema" / "🔗️component.graphql", diff_gql)
    write(art_dir / "🔺️diff" / "🧬️schema" / "🛰️component.proto", diff_proto)

    print(f"generated {key}: {len(artifact_fields)} artifact fields, {len(dfields)} diff fields")


def main() -> None:
    gen_artifact(
        key="procedural2d",
        folder="🌀️procedural2d",
        prefix="Procedural2d",
        plugin_key="procedural",
        persistent=P2D_PERSISTENT,
        ui=P2D_UI,
    )
    gen_artifact(
        key="procedural3d",
        folder="🧊️procedural3d",
        prefix="Procedural3d",
        plugin_key="procedural",
        persistent=P3D_PERSISTENT,
        ui=P3D_UI,
        extra_defs={"Procedural3dPreviewCamera": PREVIEW_CAMERA_JSON},
        extra_ts=HELPER_TS_P3D_CAM,
        extra_gql=HELPER_GQL_P3D_CAM,
        extra_proto=HELPER_PROTO_P3D_CAM,
        rust_preview_type="Procedural3dPreviewCamera",
    )


if __name__ == "__main__":
    main()
