#!/usr/bin/env python3
"""🧩 Generate A5 app-schema facets for puzzle 2d/5d/3d."""
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
PLUGIN = ROOT / "✏️s/🔌️plugins/🧩️puzzle"


def snake_to_camel(s: str) -> str:
    parts = s.split("_")
    return parts[0] + "".join(p.title() for p in parts[1:])


# ---------------------------------------------------------------------------
# Shared emitters
# ---------------------------------------------------------------------------

def emit_json(title: str, id_: str, fields: list[dict], state: str) -> str:
    props = {}
    req = []
    defs = {}
    for f in fields:
        camel = f["camel"]
        prop = dict(f["json"])
        prop["x-semio-state"] = state
        props[camel] = prop
        if not f.get("optional"):
            req.append(camel)
        if "defs" in f:
            defs.update(f["defs"])
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


def emit_ts(title: str, fields: list[dict], state: str, extras: str = "") -> str:
    lines = [f"/** 🧬️ {title} */", f"export interface {title} {{"]
    for f in fields:
        opt = "?" if f.get("optional") else ""
        lines.append(f"  /** @state {state} */")
        lines.append(f"  {f['camel']}{opt}: {f['ts']};")
    lines.append("}")
    if extras:
        lines.append("")
        lines.append(extras.rstrip())
    return "\n".join(lines) + "\n"


def emit_gql(title: str, fields: list[dict], state_enum: str, extras: str = "") -> str:
    lines = [f"type {title} {{"]
    for f in fields:
        bang = "" if f.get("optional") else "!"
        lines.append(f"  {f['camel']}: {f['gql']}{bang} @state(class: {state_enum})")
    lines.append("}")
    if extras:
        lines.append("")
        lines.append(extras.rstrip())
    return "\n".join(lines) + "\n"


def emit_proto(pkg: str, title: str, fields: list[dict], state: str, extras: str = "") -> str:
    lines = ['syntax = "proto3";', f"package {pkg};", f"message {title} {{"]
    for i, f in enumerate(fields, 1):
        lines.append(f"  // @state {state}")
        opt = "optional " if f.get("optional") and f.get("card") != "map" else ""
        lines.append(f"  {opt}{f['proto']} {f['snake']} = {i};")
    lines.append("}")
    if extras:
        lines.append("")
        lines.append(extras.rstrip())
    return "\n".join(lines) + "\n"


def emit_rust_schema(title: str, schema_id: str, fields: list[dict], state_attr: str, preamble: str = "", extras: str = "") -> str:
    lines = [
        "//! 🧬️ schema leaf",
        "use artifact_schema::ArtifactSchema;",
        "use serde::{Deserialize, Serialize};",
    ]
    if preamble:
        lines.append(preamble.rstrip())
    lines += [
        "",
        "#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]",
        '#[serde(rename_all = "camelCase")]',
        f'#[artifact_schema(id = "{schema_id}")]',
        f"pub struct {title} {{",
    ]
    for f in fields:
        lines.append(f"    #[state({state_attr})] pub {f['snake']}: {f['rust']},")
    lines.append("}")
    if extras:
        lines.append("")
        lines.append(extras.rstrip())
    return "\n".join(lines) + "\n"


def write(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content)
    print("wrote", path.relative_to(ROOT))


# ---------------------------------------------------------------------------
# Field helpers
# ---------------------------------------------------------------------------

def f_string(snake: str, optional=False) -> dict:
    return {
        "snake": snake,
        "camel": snake_to_camel(snake),
        "optional": optional,
        "card": "scalar",
        "rust": f"Option<String>" if optional else "String",
        "ts": "string",
        "gql": "String",
        "proto": "string",
        "json": {"type": "string"},
    }


def f_bool(snake: str, optional=False) -> dict:
    return {
        "snake": snake,
        "camel": snake_to_camel(snake),
        "optional": optional,
        "card": "scalar",
        "rust": f"Option<bool>" if optional else "bool",
        "ts": "boolean",
        "gql": "Boolean",
        "proto": "bool",
        "json": {"type": "boolean"},
    }


def f_f64(snake: str, optional=False) -> dict:
    return {
        "snake": snake,
        "camel": snake_to_camel(snake),
        "optional": optional,
        "card": "scalar",
        "rust": f"Option<f64>" if optional else "f64",
        "ts": "number",
        "gql": "Float",
        "proto": "double",
        "json": {"type": "number"},
    }


def f_u32(snake: str, optional=False) -> dict:
    return {
        "snake": snake,
        "camel": snake_to_camel(snake),
        "optional": optional,
        "card": "scalar",
        "rust": f"Option<u32>" if optional else "u32",
        "ts": "number",
        "gql": "Int",
        "proto": "uint32",
        "json": {"type": "integer", "format": "uint32", "minimum": 0},
    }


def f_usize(snake: str, optional=False) -> dict:
    # mirrors writer: usize stays usize in rust schema; JSON integer
    return {
        "snake": snake,
        "camel": snake_to_camel(snake),
        "optional": optional,
        "card": "scalar",
        "rust": f"Option<usize>" if optional else "usize",
        "ts": "number",
        "gql": "Int",
        "proto": "uint64",
        "json": {"type": "integer", "minimum": 0},
    }


def f_str_list(snake: str, optional=False) -> dict:
    return {
        "snake": snake,
        "camel": snake_to_camel(snake),
        "optional": optional,
        "card": "list",
        "rust": f"Option<Vec<String>>" if optional else "Vec<String>",
        "ts": "string[]",
        "gql": "[String!]",
        "proto": "repeated string",
        "json": {"type": "array", "items": {"type": "string"}},
    }


def f_fixed_f64(snake: str, n: int, optional=False) -> dict:
    return {
        "snake": snake,
        "camel": snake_to_camel(snake),
        "optional": optional,
        "card": "fixedList",
        "rust": f"Option<[{'f64; ' + str(n)}]>" if optional else f"[f64; {n}]",
        "ts": "number[]",
        "gql": "[Float!]",
        "proto": "repeated double",
        "json": {"type": "array", "items": {"type": "number"}, "minItems": n, "maxItems": n},
    }


def f_fixed_u32(snake: str, n: int, optional=False) -> dict:
    return {
        "snake": snake,
        "camel": snake_to_camel(snake),
        "optional": optional,
        "card": "fixedList",
        "rust": f"[u32; {n}]",
        "ts": "number[]",
        "gql": "[Int!]",
        "proto": "repeated uint32",
        "json": {
            "type": "array",
            "items": {"type": "integer", "minimum": 0},
            "minItems": n,
            "maxItems": n,
        },
    }


def f_map_string(snake: str) -> dict:
    return {
        "snake": snake,
        "camel": snake_to_camel(snake),
        "optional": False,
        "card": "map",
        "rust": "std::collections::BTreeMap<String, String>",
        "ts": "Record<string, string>",
        "gql": "[StringEntry!]",
        "proto": "map<string, string>",
        "json": {"type": "object", "additionalProperties": {"type": "string"}},
    }


def f_map_f64(snake: str, hash_map=False) -> dict:
    rust = (
        "std::collections::HashMap<String, f64>"
        if hash_map
        else "std::collections::BTreeMap<String, f64>"
    )
    return {
        "snake": snake,
        "camel": snake_to_camel(snake),
        "optional": False,
        "card": "map",
        "rust": rust,
        "ts": "Record<string, number>",
        "gql": "[FloatEntry!]",
        "proto": "map<string, double>",
        "json": {"type": "object", "additionalProperties": {"type": "number"}},
    }


def f_ref(snake: str, type_name: str, optional=False, defs: dict | None = None) -> dict:
    return {
        "snake": snake,
        "camel": snake_to_camel(snake),
        "optional": optional,
        "card": "scalar",
        "rust": f"Option<{type_name}>" if optional else type_name,
        "ts": type_name,
        "gql": type_name,
        "proto": type_name,
        "json": {"$ref": f"#/$defs/{type_name}"},
        "defs": defs or {},
    }


def f_map_ref(snake: str, type_name: str) -> dict:
    return {
        "snake": snake,
        "camel": snake_to_camel(snake),
        "optional": False,
        "card": "map",
        "rust": f"std::collections::BTreeMap<String, {type_name}>",
        "ts": f"Record<string, {type_name}>",
        "gql": f"[{type_name}Entry!]",
        "proto": f"map<string, {type_name}>",
        "json": {"type": "object", "additionalProperties": {"$ref": f"#/$defs/{type_name}"}},
    }


def f_value_list(snake: str) -> dict:
    return {
        "snake": snake,
        "camel": snake_to_camel(snake),
        "optional": False,
        "card": "list",
        "rust": "Vec<serde_json::Value>",
        "ts": "unknown[]",
        "gql": "[JsonValue!]",
        "proto": "repeated string",  # JSON-encoded; scalar name ignored by parity
        "json": {"type": "array", "items": {"title": "Value", "type": "object"}},
    }


MAP_ENTRY_EXTRAS_GQL = """
type StringEntry {
  key: String!
  value: String!
}

type FloatEntry {
  key: String!
  value: Float!
}
""".strip()

JSON_VALUE_GQL = """
scalar JsonValue
""".strip()


# ---------------------------------------------------------------------------
# Nested type defs (JSON / TS / GQL / Proto / Rust extras)
# ---------------------------------------------------------------------------

SELECTION_SET_DEF = {
    "SelectionSet": {
        "title": "SelectionSet",
        "type": "array",
        "items": {"type": "string"},
    }
}

WORLD_SUN_DEF = {
    "WorldSunConfig": {
        "title": "WorldSunConfig",
        "type": "object",
        "additionalProperties": False,
        "required": ["enabled", "azimuth", "elevation", "intensity", "color"],
        "properties": {
            "enabled": {"type": "boolean"},
            "azimuth": {"type": "number"},
            "elevation": {"type": "number"},
            "intensity": {"type": "number"},
            "color": {"type": "string"},
        },
    }
}

WORLD_PROJ_DEF = {
    "WorldProjectionConfig": {
        "title": "WorldProjectionConfig",
        "type": "object",
        "additionalProperties": False,
        "required": [
            "kind",
            "orthographicView",
            "axonometricVariant",
            "axonometricAngleA",
            "axonometricAngleB",
            "axonometricQuadrant",
            "obliqueVariant",
            "obliqueAngle",
            "obliqueDepth",
            "onePointAxis",
            "fov",
            "twoPointShift",
            "curvilinearFov",
            "curvilinearStrength",
            "curvilinearMapping",
        ],
        "properties": {
            "kind": {"type": "string"},
            "orthographicView": {"type": "string"},
            "axonometricVariant": {"type": "string"},
            "axonometricAngleA": {"type": "number"},
            "axonometricAngleB": {"type": "number"},
            "axonometricQuadrant": {"type": "string"},
            "obliqueVariant": {"type": "string"},
            "obliqueAngle": {"type": "number"},
            "obliqueDepth": {"type": "number"},
            "onePointAxis": {"type": "string"},
            "fov": {"type": "number"},
            "twoPointShift": {"type": "number"},
            "curvilinearFov": {"type": "number"},
            "curvilinearStrength": {"type": "number"},
            "curvilinearMapping": {"type": "string"},
        },
    }
}

P5_CAM2D_DEF = {
    "Puzzle5dCamera2d": {
        "title": "Puzzle5dCamera2d",
        "type": "object",
        "additionalProperties": False,
        "required": ["x", "y", "zoom"],
        "properties": {
            "x": {"type": "number"},
            "y": {"type": "number"},
            "zoom": {"type": "number"},
        },
    }
}

P5_CAM3D_DEF = {
    "Puzzle5dCamera3d": {
        "title": "Puzzle5dCamera3d",
        "type": "object",
        "additionalProperties": False,
        "required": ["position", "target", "zoom"],
        "properties": {
            "position": {
                "type": "array",
                "items": {"type": "number"},
                "minItems": 3,
                "maxItems": 3,
            },
            "target": {
                "type": "array",
                "items": {"type": "number"},
                "minItems": 3,
                "maxItems": 3,
            },
            "zoom": {"type": "number"},
        },
    }
}

P5_SEL_DEF = {
    "Puzzle5dSelection": {
        "title": "Puzzle5dSelection",
        "type": "object",
        "additionalProperties": False,
        "required": ["partIds", "gripIds", "fastenerIds"],
        "properties": {
            "partIds": {"$ref": "#/$defs/SelectionSet"},
            "gripIds": {"$ref": "#/$defs/SelectionSet"},
            "fastenerIds": {"$ref": "#/$defs/SelectionSet"},
        },
    }
}

P3_CAM_DEF = {
    "Puzzle3dCamera": {
        "title": "Puzzle3dCamera",
        "type": "object",
        "additionalProperties": False,
        "required": ["position", "target", "zoom", "projection"],
        "properties": {
            "position": {
                "type": "array",
                "items": {"type": "number"},
                "minItems": 3,
                "maxItems": 3,
            },
            "target": {
                "type": "array",
                "items": {"type": "number"},
                "minItems": 3,
                "maxItems": 3,
            },
            "zoom": {"type": "number"},
            "up": {
                "type": "array",
                "items": {"type": "number"},
                "minItems": 3,
                "maxItems": 3,
            },
            "projection": {"$ref": "#/$defs/WorldProjectionConfig"},
        },
    }
}

P3_SEL_DEF = {
    "Puzzle3dSelection": {
        "title": "Puzzle3dSelection",
        "type": "object",
        "additionalProperties": False,
        "required": [
            "objectIds",
            "vortexIds",
            "attractionIds",
            "targetVolumeIds",
            "referenceIds",
        ],
        "properties": {
            "objectIds": {"$ref": "#/$defs/SelectionSet"},
            "vortexIds": {"$ref": "#/$defs/SelectionSet"},
            "attractionIds": {"$ref": "#/$defs/SelectionSet"},
            "targetVolumeIds": {"$ref": "#/$defs/SelectionSet"},
            "referenceIds": {"$ref": "#/$defs/SelectionSet"},
        },
    }
}

P3_KINDS_DEF = {
    "Puzzle3dSelectableKinds": {
        "title": "Puzzle3dSelectableKinds",
        "type": "object",
        "additionalProperties": False,
        "required": ["objects", "vortices", "attractions"],
        "properties": {
            "objects": {"type": "boolean"},
            "vortices": {"type": "boolean"},
            "attractions": {"type": "boolean"},
        },
    }
}

P3_SUGGEST_DEF = {
    "Puzzle3dSuggestionMenu": {
        "title": "Puzzle3dSuggestionMenu",
        "type": "object",
        "additionalProperties": False,
        "required": ["x", "y", "windowId"],
        "properties": {
            "x": {"type": "number"},
            "y": {"type": "number"},
            "windowId": {"type": "string"},
        },
    }
}

P3_WIN_DEF = {
    "Puzzle3dWindowOptions": {
        "title": "Puzzle3dWindowOptions",
        "type": "object",
        "additionalProperties": False,
        "required": [
            "selectionMethod",
            "lodAutomatic",
            "lodDepthVariable",
            "gridVisible",
            "lodManual",
            "gridSnapEnabled",
            "gridSpacing",
            "selectableKinds",
            "engagementInput",
            "selectionModeDefault",
            "proximityRadius",
            "chunkSize",
            "voxelDims",
            "transformMove",
            "transformRotate",
            "vortexShow",
            "vortexDirection",
            "sun",
            "camera",
        ],
        "properties": {
            "selectionMethod": {"type": "string"},
            "lodAutomatic": {"type": "boolean"},
            "lodDepthVariable": {"type": "boolean"},
            "gridVisible": {"type": "boolean"},
            "lodManual": {"type": "number"},
            "gridSnapEnabled": {"type": "boolean"},
            "gridSpacing": {"type": "number"},
            "selectableKinds": {"$ref": "#/$defs/Puzzle3dSelectableKinds"},
            "engagementInput": {"type": "string"},
            "selectionModeDefault": {"type": "string"},
            "proximityRadius": {"type": "number"},
            "chunkSize": {"type": "number"},
            "voxelDims": {
                "type": "array",
                "items": {"type": "integer", "minimum": 0},
                "minItems": 3,
                "maxItems": 3,
            },
            "transformMove": {"type": "boolean"},
            "transformRotate": {"type": "boolean"},
            "vortexShow": {"type": "string"},
            "vortexDirection": {"type": "string"},
            "sun": {"$ref": "#/$defs/WorldSunConfig"},
            "camera": {"$ref": "#/$defs/Puzzle3dCamera"},
        },
    }
}


NESTED_TS_COMMON = """
export type SelectionSet = string[];

export interface WorldSunConfig {
  enabled: boolean;
  azimuth: number;
  elevation: number;
  intensity: number;
  color: string;
}

export interface WorldProjectionConfig {
  kind: string;
  orthographicView: string;
  axonometricVariant: string;
  axonometricAngleA: number;
  axonometricAngleB: number;
  axonometricQuadrant: string;
  obliqueVariant: string;
  obliqueAngle: number;
  obliqueDepth: number;
  onePointAxis: string;
  fov: number;
  twoPointShift: number;
  curvilinearFov: number;
  curvilinearStrength: number;
  curvilinearMapping: string;
}
""".strip()

NESTED_TS_5D = (
    NESTED_TS_COMMON
    + """

export interface Puzzle5dCamera2d {
  x: number;
  y: number;
  zoom: number;
}

export interface Puzzle5dCamera3d {
  position: number[];
  target: number[];
  zoom: number;
}

export interface Puzzle5dSelection {
  partIds: SelectionSet;
  gripIds: SelectionSet;
  fastenerIds: SelectionSet;
}
"""
)

NESTED_TS_3D = (
    NESTED_TS_COMMON
    + """

export interface Puzzle3dCamera {
  position: number[];
  target: number[];
  zoom: number;
  up?: number[];
  projection: WorldProjectionConfig;
}

export interface Puzzle3dSelection {
  objectIds: SelectionSet;
  vortexIds: SelectionSet;
  attractionIds: SelectionSet;
  targetVolumeIds: SelectionSet;
  referenceIds: SelectionSet;
}

export interface Puzzle3dSelectableKinds {
  objects: boolean;
  vortices: boolean;
  attractions: boolean;
}

export interface Puzzle3dSuggestionMenu {
  x: number;
  y: number;
  windowId: string;
}

export interface Puzzle3dWindowOptions {
  selectionMethod: string;
  lodAutomatic: boolean;
  lodDepthVariable: boolean;
  gridVisible: boolean;
  lodManual: number;
  gridSnapEnabled: boolean;
  gridSpacing: number;
  selectableKinds: Puzzle3dSelectableKinds;
  engagementInput: string;
  selectionModeDefault: string;
  proximityRadius: number;
  chunkSize: number;
  voxelDims: number[];
  transformMove: boolean;
  transformRotate: boolean;
  vortexShow: string;
  vortexDirection: string;
  sun: WorldSunConfig;
  camera: Puzzle3dCamera;
}
"""
)

NESTED_GQL_COMMON = """
type SelectionSet {
  ids: [String!]!
}

type WorldSunConfig {
  enabled: Boolean!
  azimuth: Float!
  elevation: Float!
  intensity: Float!
  color: String!
}

type WorldProjectionConfig {
  kind: String!
  orthographicView: String!
  axonometricVariant: String!
  axonometricAngleA: Float!
  axonometricAngleB: Float!
  axonometricQuadrant: String!
  obliqueVariant: String!
  obliqueAngle: Float!
  obliqueDepth: Float!
  onePointAxis: String!
  fov: Float!
  twoPointShift: Float!
  curvilinearFov: Float!
  curvilinearStrength: Float!
  curvilinearMapping: String!
}
""".strip()

NESTED_GQL_5D = (
    NESTED_GQL_COMMON
    + "\n\n"
    + MAP_ENTRY_EXTRAS_GQL
    + """

type Puzzle5dCamera2d {
  x: Float!
  y: Float!
  zoom: Float!
}

type Puzzle5dCamera3d {
  position: [Float!]!
  target: [Float!]!
  zoom: Float!
}

type Puzzle5dSelection {
  partIds: SelectionSet!
  gripIds: SelectionSet!
  fastenerIds: SelectionSet!
}
"""
)

NESTED_GQL_3D = (
    NESTED_GQL_COMMON
    + "\n\n"
    + MAP_ENTRY_EXTRAS_GQL
    + """

type Puzzle3dCamera {
  position: [Float!]!
  target: [Float!]!
  zoom: Float!
  up: [Float!]
  projection: WorldProjectionConfig!
}

type Puzzle3dSelection {
  objectIds: SelectionSet!
  vortexIds: SelectionSet!
  attractionIds: SelectionSet!
  targetVolumeIds: SelectionSet!
  referenceIds: SelectionSet!
}

type Puzzle3dSelectableKinds {
  objects: Boolean!
  vortices: Boolean!
  attractions: Boolean!
}

type Puzzle3dSuggestionMenu {
  x: Float!
  y: Float!
  windowId: String!
}

type Puzzle3dWindowOptions {
  selectionMethod: String!
  lodAutomatic: Boolean!
  lodDepthVariable: Boolean!
  gridVisible: Boolean!
  lodManual: Float!
  gridSnapEnabled: Boolean!
  gridSpacing: Float!
  selectableKinds: Puzzle3dSelectableKinds!
  engagementInput: String!
  selectionModeDefault: String!
  proximityRadius: Float!
  chunkSize: Float!
  voxelDims: [Int!]!
  transformMove: Boolean!
  transformRotate: Boolean!
  vortexShow: String!
  vortexDirection: String!
  sun: WorldSunConfig!
  camera: Puzzle3dCamera!
}

type Puzzle3dWindowOptionsEntry {
  key: String!
  value: Puzzle3dWindowOptions!
}
"""
)

NESTED_PROTO_COMMON = """
message SelectionSet {
  repeated string ids = 1;
}

message WorldSunConfig {
  bool enabled = 1;
  double azimuth = 2;
  double elevation = 3;
  double intensity = 4;
  string color = 5;
}

message WorldProjectionConfig {
  string kind = 1;
  string orthographic_view = 2;
  string axonometric_variant = 3;
  double axonometric_angle_a = 4;
  double axonometric_angle_b = 5;
  string axonometric_quadrant = 6;
  string oblique_variant = 7;
  double oblique_angle = 8;
  double oblique_depth = 9;
  string one_point_axis = 10;
  double fov = 11;
  double two_point_shift = 12;
  double curvilinear_fov = 13;
  double curvilinear_strength = 14;
  string curvilinear_mapping = 15;
}
""".strip()

NESTED_PROTO_5D = (
    NESTED_PROTO_COMMON
    + """

message Puzzle5dCamera2d {
  double x = 1;
  double y = 2;
  double zoom = 3;
}

message Puzzle5dCamera3d {
  repeated double position = 1;
  repeated double target = 2;
  double zoom = 3;
}

message Puzzle5dSelection {
  SelectionSet part_ids = 1;
  SelectionSet grip_ids = 2;
  SelectionSet fastener_ids = 3;
}
"""
)

NESTED_PROTO_3D = (
    NESTED_PROTO_COMMON
    + """

message Puzzle3dCamera {
  repeated double position = 1;
  repeated double target = 2;
  double zoom = 3;
  repeated double up = 4;
  WorldProjectionConfig projection = 5;
}

message Puzzle3dSelection {
  SelectionSet object_ids = 1;
  SelectionSet vortex_ids = 2;
  SelectionSet attraction_ids = 3;
  SelectionSet target_volume_ids = 4;
  SelectionSet reference_ids = 5;
}

message Puzzle3dSelectableKinds {
  bool objects = 1;
  bool vortices = 2;
  bool attractions = 3;
}

message Puzzle3dSuggestionMenu {
  double x = 1;
  double y = 2;
  string window_id = 3;
}

message Puzzle3dWindowOptions {
  string selection_method = 1;
  bool lod_automatic = 2;
  bool lod_depth_variable = 3;
  bool grid_visible = 4;
  double lod_manual = 5;
  bool grid_snap_enabled = 6;
  double grid_spacing = 7;
  Puzzle3dSelectableKinds selectable_kinds = 8;
  string engagement_input = 9;
  string selection_mode_default = 10;
  double proximity_radius = 11;
  double chunk_size = 12;
  repeated uint32 voxel_dims = 13;
  bool transform_move = 14;
  bool transform_rotate = 15;
  string vortex_show = 16;
  string vortex_direction = 17;
  WorldSunConfig sun = 18;
  Puzzle3dCamera camera = 19;
}
"""
)

RUST_NESTED_HELPERS = """
use std::collections::{BTreeMap, HashMap};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
pub struct SelectionSet {
    pub ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
pub struct WorldSunConfig {
    pub enabled: bool,
    pub azimuth: f64,
    pub elevation: f64,
    pub intensity: f64,
    pub color: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
pub struct WorldProjectionConfig {
    pub kind: String,
    pub orthographic_view: String,
    pub axonometric_variant: String,
    pub axonometric_angle_a: f64,
    pub axonometric_angle_b: f64,
    pub axonometric_quadrant: String,
    pub oblique_variant: String,
    pub oblique_angle: f64,
    pub oblique_depth: f64,
    pub one_point_axis: String,
    pub fov: f64,
    pub two_point_shift: f64,
    pub curvilinear_fov: f64,
    pub curvilinear_strength: f64,
    pub curvilinear_mapping: String,
}
""".strip()

RUST_NESTED_5D = (
    RUST_NESTED_HELPERS
    + """

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dCamera2d {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dCamera3d {
    pub position: [f64; 3],
    pub target: [f64; 3],
    pub zoom: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dSelection {
    pub part_ids: SelectionSet,
    pub grip_ids: SelectionSet,
    pub fastener_ids: SelectionSet,
}
"""
)

RUST_NESTED_3D = (
    RUST_NESTED_HELPERS
    + """

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dCamera {
    pub position: [f64; 3],
    pub target: [f64; 3],
    pub zoom: f64,
    pub up: Option<[f64; 3]>,
    pub projection: WorldProjectionConfig,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dSelection {
    pub object_ids: SelectionSet,
    pub vortex_ids: SelectionSet,
    pub attraction_ids: SelectionSet,
    pub target_volume_ids: SelectionSet,
    pub reference_ids: SelectionSet,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dSelectableKinds {
    pub objects: bool,
    pub vortices: bool,
    pub attractions: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dSuggestionMenu {
    pub x: f64,
    pub y: f64,
    pub window_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dWindowOptions {
    pub selection_method: String,
    pub lod_automatic: bool,
    pub lod_depth_variable: bool,
    pub grid_visible: bool,
    pub lod_manual: f64,
    pub grid_snap_enabled: bool,
    pub grid_spacing: f64,
    pub selectable_kinds: Puzzle3dSelectableKinds,
    pub engagement_input: String,
    pub selection_mode_default: String,
    pub proximity_radius: f64,
    pub chunk_size: f64,
    pub voxel_dims: [u32; 3],
    pub transform_move: bool,
    pub transform_rotate: bool,
    pub vortex_show: String,
    pub vortex_direction: String,
    pub sun: WorldSunConfig,
    pub camera: Puzzle3dCamera,
}
"""
)


def presence_runtime(name: str, extension: str, fields_default_block: str, struct_fields: str) -> str:
    return f'''//! 👥️ {name} — shareable live ephemeral state + mutations.

use protocol::Mutation;
use serde::{{Deserialize, Serialize}};
use store::DocumentPack;

//#region 🔖️Presence
/// 👥️ Shareable live subset of puzzle view state (selection, hover, camera, active utility).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "{extension}")]
#[dsl(layout = "lines")]
pub struct {name} {{
{struct_fields}
}}

impl Default for {name} {{
    fn default() -> Self {{
        Self {{
{fields_default_block}
        }}
    }}
}}

impl protocol::MutationDiff<{name}> for {name} {{
    fn apply(&self, _base: &{name}) -> {name} {{
        self.clone()
    }}
    fn absorb(&mut self, other: Self) {{
        *self = other;
    }}
}}

impl store::DocumentDsl for {name} {{
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

impl DocumentPack for {name} {{
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
pub enum {name}Mutation {{
    #[dsl(key = "snapshot")]
    Snapshot {{
        #[dsl(block)]
        presence: {name},
    }},
}}

impl Mutation<{name}> for {name}Mutation {{
    type Diff = {name};

    fn diff(&self, _base: &{name}) -> {name} {{
        match self {{
            Self::Snapshot {{ presence }} => presence.clone(),
        }}
    }}

    fn inverse(&self, base: &{name}) -> Vec<Self> {{
        vec![Self::Snapshot {{ presence: base.clone() }}]
    }}
}}

impl protocol::OpText for {name}Mutation {{
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

impl protocol::OpBinary for {name}Mutation {{
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {{
        dsl::variants_binary::encode_op(self)
    }}
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {{
        dsl::variants_binary::decode_op(bytes)
    }}
}}
//#endregion 🔖️PresenceMutation
'''


def write_facet(owner_app: Path, slug: str, config_type: str, presence_type: str, config_fields, presence_fields, nested):
    cfg_schema = owner_app / "🎚️config" / "🧬️schema"
    presence_dir = owner_app / "👥️presence"
    presence_schema = presence_dir / "🧬️schema"
    pkg = f"semio.app.puzzle.{slug}"
    cfg_id = f"https://semio.tech/schema/app/puzzle/{slug}/config.json"
    pres_id = f"https://semio.tech/schema/app/puzzle/{slug}/presence.json"
    rust_cfg_id = f"s.puzzle.{slug}.config"
    rust_pres_id = f"s.puzzle.{slug}.presence"

    # merge defs from fields + nested
    for f in config_fields:
        if "defs" in f and f["defs"]:
            nested.setdefault("json_defs", {}).update(f["defs"])

    write(cfg_schema / "🔣️component.json", emit_json(config_type, cfg_id, config_fields, "local-ui").replace(
        '"properties":',
        # inject $defs by rebuilding
        '"properties":',
    ))
    # rebuild json properly with defs
    write(
        cfg_schema / "🔣️component.json",
        emit_json(
            config_type,
            cfg_id,
            [{**f, "defs": {}} for f in config_fields],
            "local-ui",
        ).rstrip()[:-1]
        + (
            (",\n  \"$defs\": " + json.dumps(nested.get("json_defs", {}), indent=2).replace("\n", "\n  ") + "\n}\n")
            if nested.get("json_defs")
            else "\n}\n"
        )
        if nested.get("json_defs")
        else emit_json(config_type, cfg_id, config_fields, "local-ui"),
    )
    # Fix botched emit — always use clean path
    props = {}
    req = []
    for f in config_fields:
        prop = dict(f["json"])
        prop["x-semio-state"] = "local-ui"
        props[f["camel"]] = prop
        if not f.get("optional"):
            req.append(f["camel"])
    doc = {
        "$id": cfg_id,
        "title": config_type,
        "type": "object",
        "additionalProperties": False,
        "required": req,
        "properties": props,
    }
    if nested.get("json_defs"):
        doc["$defs"] = nested["json_defs"]
    write(cfg_schema / "🔣️component.json", json.dumps(doc, indent=2) + "\n")

    write(cfg_schema / "🟦️component.ts", emit_ts(config_type, config_fields, "local-ui", nested.get("ts", "")))
    write(cfg_schema / "🔗️component.graphql", emit_gql(config_type, config_fields, "LOCAL_UI", nested.get("gql", "")))
    write(cfg_schema / "🛰️component.proto", emit_proto(pkg, config_type, config_fields, "local-ui", nested.get("proto", "")))

    rust_preamble = "use serde_json::Value;\n" if any("serde_json::Value" in f["rust"] for f in config_fields) else ""
    # nested types go AFTER the main struct as extras — but main struct references them, so put nested BEFORE via preamble
    rust_extras = ""
    rust_pre = rust_preamble
    if nested.get("rust"):
        # put nested types before struct by using a custom emitter
        content = (
            "//! 🧬️ schema leaf\n"
            "use artifact_schema::ArtifactSchema;\n"
            "use serde::{Deserialize, Serialize};\n"
        )
        if rust_preamble:
            content += rust_preamble
        content += "\n" + nested["rust"].rstrip() + "\n\n"
        content += (
            f"#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]\n"
            f'#[serde(rename_all = "camelCase")]\n'
            f'#[artifact_schema(id = "{rust_cfg_id}")]\n'
            f"pub struct {config_type} {{\n"
        )
        for f in config_fields:
            # fix rust types that used fully-qualified maps — nested already imported BTreeMap/HashMap
            rust_ty = (
                f["rust"]
                .replace("std::collections::BTreeMap", "BTreeMap")
                .replace("std::collections::HashMap", "HashMap")
                .replace("serde_json::Value", "Value")
            )
            content += f"    #[state(local_ui)] pub {f['snake']}: {rust_ty},\n"
        content += "}\n"
        write(cfg_schema / "🦀️component.rs", content)
    else:
        # 2d — may still need BTreeMap
        needs_btree = any("BTreeMap" in f["rust"] for f in config_fields)
        content = (
            "//! 🧬️ schema leaf\n"
            "use artifact_schema::ArtifactSchema;\n"
            "use serde::{Deserialize, Serialize};\n"
        )
        if needs_btree:
            content += "use std::collections::BTreeMap;\n"
        if rust_preamble:
            content += rust_preamble
        content += (
            f"\n#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]\n"
            f'#[serde(rename_all = "camelCase")]\n'
            f'#[artifact_schema(id = "{rust_cfg_id}")]\n'
            f"pub struct {config_type} {{\n"
        )
        for f in config_fields:
            rust_ty = f["rust"].replace("std::collections::BTreeMap", "BTreeMap").replace("serde_json::Value", "Value")
            content += f"    #[state(local_ui)] pub {f['snake']}: {rust_ty},\n"
        content += "}\n"
        write(cfg_schema / "🦀️component.rs", content)

    # presence schema
    props = {}
    req = []
    for f in presence_fields:
        prop = dict(f["json"])
        prop["x-semio-state"] = "shared-ui"
        props[f["camel"]] = prop
        if not f.get("optional"):
            req.append(f["camel"])
    write(
        presence_schema / "🔣️component.json",
        json.dumps(
            {
                "$id": pres_id,
                "title": presence_type,
                "type": "object",
                "additionalProperties": False,
                "required": req,
                "properties": props,
            },
            indent=2,
        )
        + "\n",
    )
    write(presence_schema / "🟦️component.ts", emit_ts(presence_type, presence_fields, "shared-ui"))
    write(presence_schema / "🔗️component.graphql", emit_gql(presence_type, presence_fields, "SHARED_UI"))
    write(presence_schema / "🛰️component.proto", emit_proto(pkg, presence_type, presence_fields, "shared-ui"))
    content = (
        "//! 🧬️ schema leaf\n"
        "use artifact_schema::ArtifactSchema;\n"
        "use serde::{Deserialize, Serialize};\n"
        f"\n#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]\n"
        f'#[serde(rename_all = "camelCase", default)]\n'
        f'#[artifact_schema(id = "{rust_pres_id}")]\n'
        f"pub struct {presence_type} {{\n"
    )
    for f in presence_fields:
        content += f"    #[state(shared_ui)] pub {f['snake']}: {f['rust']},\n"
    content += "}\n"
    write(presence_schema / "🦀️component.rs", content)


def main():
    # ---- 2d ----
    p2_cfg = [
        f_str_list("selected_ids"),
        f_f64("camera_x"),
        f_f64("camera_y"),
        f_f64("camera_zoom"),
        f_map_string("lod_mode_by_pane"),
        f_map_string("engagement_input_by_pane"),
        f_usize("brush_candidate_index"),
        f_value_list("brush_candidates"),
        f_string("brush_candidate_source_handle_id"),
        f_u32("fill_count"),
        f_string("selection_method"),
        f_bool("grid_snap_enabled"),
        f_f64("grid_factor"),
        f_f64("suggestion_offset"),
        f_map_f64("node_kind_weights"),
        f_map_f64("handle_kind_weights"),
        f_map_string("active_utility_by_window_id"),
        f_string("locale"),
        f_string("terminology"),
    ]
    # fix map_f64 to use BTreeMap for 2d
    for f in p2_cfg:
        if f["snake"] in ("node_kind_weights", "handle_kind_weights"):
            f["rust"] = "std::collections::BTreeMap<String, f64>"

    p2_pres = [
        f_str_list("selected_ids"),
        f_f64("camera_x"),
        f_f64("camera_y"),
        f_f64("camera_zoom"),
        f_string("selection_method"),
        f_string("active_utility_id"),
    ]

    write_facet(
        PLUGIN / "🎛️apps/◻2d",
        "puzzle2d",
        "Puzzle2dConfig",
        "Puzzle2dPresence",
        p2_cfg,
        p2_pres,
        {
            "gql": MAP_ENTRY_EXTRAS_GQL + "\n\n" + JSON_VALUE_GQL,
            "ts": "",
            "proto": "",
            "json_defs": {},
        },
    )
    write(
        PLUGIN / "🎛️apps/◻2d/👥️presence/🦀️component.rs",
        presence_runtime(
            "Puzzle2dPresence",
            "puzzle2d.presence",
            """            selected_ids: Vec::new(),
            camera_x: 0.0,
            camera_y: 0.0,
            camera_zoom: 1.0,
            selection_method: "rectangle".into(),
            active_utility_id: String::new(),""",
            """    pub selected_ids: Vec<String>,
    pub camera_x: f64,
    pub camera_y: f64,
    pub camera_zoom: f64,
    pub selection_method: String,
    pub active_utility_id: String,""",
        ),
    )

    # ---- 5d ----
    p5_defs = {}
    p5_defs.update(SELECTION_SET_DEF)
    p5_defs.update(WORLD_SUN_DEF)
    p5_defs.update(P5_CAM2D_DEF)
    p5_defs.update(P5_CAM3D_DEF)
    p5_defs.update(P5_SEL_DEF)

    p5_cfg = [
        f_ref("camera2d", "Puzzle5dCamera2d", defs=P5_CAM2D_DEF),
        f_ref("camera3d", "Puzzle5dCamera3d", defs=P5_CAM3D_DEF),
        f_ref("selection", "Puzzle5dSelection", defs={**SELECTION_SET_DEF, **P5_SEL_DEF}),
        f_string("selection_method"),
        f_string("hovered_part_id", optional=True),
        f_u32("fill_count"),
        f_usize("brush_candidate_index"),
        f_f64("overlap_budget"),
        f_string("lod_mode"),
        f_f64("suggestion_offset"),
        f_bool("grid_snap_enabled"),
        f_f64("grid_factor"),
        f_map_string("engagement_input_by_window"),
        f_map_f64("object_kind_weights", hash_map=True),
        f_map_f64("vortex_kind_weights", hash_map=True),
        f_ref("sun", "WorldSunConfig", defs=WORLD_SUN_DEF),
        f_map_string("active_utility_by_window_id"),
        f_string("terminology"),
        f_string("locale"),
    ]

    p5_pres = [
        f_str_list("selected_part_ids"),
        f_str_list("selected_grip_ids"),
        f_str_list("selected_fastener_ids"),
        f_string("hovered_part_id", optional=True),
        f_f64("camera2d_x"),
        f_f64("camera2d_y"),
        f_f64("camera2d_zoom"),
        f_fixed_f64("camera3d_position", 3),
        f_fixed_f64("camera3d_target", 3),
        f_f64("camera3d_zoom"),
        f_string("active_utility_id"),
    ]

    write_facet(
        PLUGIN / "🎛️apps/🖐️5d",
        "puzzle5d",
        "Puzzle5dConfig",
        "Puzzle5dPresence",
        p5_cfg,
        p5_pres,
        {
            "json_defs": p5_defs,
            "ts": NESTED_TS_5D,
            "gql": NESTED_GQL_5D,
            "proto": NESTED_PROTO_5D,
            "rust": RUST_NESTED_5D,
        },
    )
    write(
        PLUGIN / "🎛️apps/🖐️5d/👥️presence/🦀️component.rs",
        presence_runtime(
            "Puzzle5dPresence",
            "puzzle5d.presence",
            """            selected_part_ids: Vec::new(),
            selected_grip_ids: Vec::new(),
            selected_fastener_ids: Vec::new(),
            hovered_part_id: None,
            camera2d_x: 0.0,
            camera2d_y: 0.0,
            camera2d_zoom: 1.0,
            camera3d_position: [8.0, -8.0, 8.0],
            camera3d_target: [0.0, 0.0, 0.0],
            camera3d_zoom: 1.0,
            active_utility_id: String::new(),""",
            """    pub selected_part_ids: Vec<String>,
    pub selected_grip_ids: Vec<String>,
    pub selected_fastener_ids: Vec<String>,
    pub hovered_part_id: Option<String>,
    pub camera2d_x: f64,
    pub camera2d_y: f64,
    pub camera2d_zoom: f64,
    pub camera3d_position: [f64; 3],
    pub camera3d_target: [f64; 3],
    pub camera3d_zoom: f64,
    pub active_utility_id: String,""",
        ),
    )

    # ---- 3d ----
    p3_defs = {}
    p3_defs.update(SELECTION_SET_DEF)
    p3_defs.update(WORLD_SUN_DEF)
    p3_defs.update(WORLD_PROJ_DEF)
    p3_defs.update(P3_CAM_DEF)
    p3_defs.update(P3_SEL_DEF)
    p3_defs.update(P3_KINDS_DEF)
    p3_defs.update(P3_SUGGEST_DEF)
    p3_defs.update(P3_WIN_DEF)

    p3_cfg = [
        f_ref("selection", "Puzzle3dSelection", defs={**SELECTION_SET_DEF, **P3_SEL_DEF}),
        f_string("selection_method"),
        f_string("hovered_object_id", optional=True),
        f_string("hovered_vortex_full_id", optional=True),
        f_ref("suggestion_menu", "Puzzle3dSuggestionMenu", optional=True, defs=P3_SUGGEST_DEF),
        f_f64("overlap_budget"),
        f_u32("fill_count"),
        f_usize("brush_candidate_index"),
        f_map_f64("object_kind_weights", hash_map=True),
        f_map_f64("vortex_kind_weights", hash_map=True),
        f_bool("lod_automatic"),
        f_bool("lod_depth_variable"),
        f_bool("grid_visible"),
        f_f64("lod_manual"),
        f_bool("grid_snap_enabled"),
        f_f64("grid_spacing"),
        f_ref("selectable_kinds", "Puzzle3dSelectableKinds", defs=P3_KINDS_DEF),
        f_string("hovered_kind_id", optional=True),
        f_string("engagement_input"),
        f_string("selection_mode_default"),
        f_f64("proximity_radius"),
        f_f64("chunk_size"),
        f_fixed_u32("voxel_dims", 3),
        f_bool("transform_move"),
        f_bool("transform_rotate"),
        f_string("vortex_show"),
        f_string("vortex_direction"),
        f_ref("sun", "WorldSunConfig", defs=WORLD_SUN_DEF),
        f_ref("camera", "Puzzle3dCamera", defs={**WORLD_PROJ_DEF, **P3_CAM_DEF}),
        f_map_ref("window_options", "Puzzle3dWindowOptions"),
        f_map_string("active_utility_by_window_id"),
        f_string("active_tool_id", optional=True),
        f_string("terminology"),
        f_string("locale"),
        f_str_list("window_ids"),
    ]

    p3_pres = [
        f_str_list("selected_object_ids"),
        f_str_list("selected_vortex_ids"),
        f_str_list("selected_attraction_ids"),
        f_str_list("selected_target_volume_ids"),
        f_str_list("selected_reference_ids"),
        f_string("hovered_object_id", optional=True),
        f_string("hovered_vortex_full_id", optional=True),
        f_fixed_f64("camera_position", 3),
        f_fixed_f64("camera_target", 3),
        f_f64("camera_zoom"),
        f_string("active_utility_id"),
        f_string("active_tool_id", optional=True),
    ]

    write_facet(
        PLUGIN / "🎛️apps/🧊️3d",
        "puzzle3d",
        "Puzzle3dConfig",
        "Puzzle3dPresence",
        p3_cfg,
        p3_pres,
        {
            "json_defs": p3_defs,
            "ts": NESTED_TS_3D,
            "gql": NESTED_GQL_3D,
            "proto": NESTED_PROTO_3D,
            "rust": RUST_NESTED_3D,
        },
    )
    write(
        PLUGIN / "🎛️apps/🧊️3d/👥️presence/🦀️component.rs",
        presence_runtime(
            "Puzzle3dPresence",
            "puzzle3d.presence",
            """            selected_object_ids: Vec::new(),
            selected_vortex_ids: Vec::new(),
            selected_attraction_ids: Vec::new(),
            selected_target_volume_ids: Vec::new(),
            selected_reference_ids: Vec::new(),
            hovered_object_id: None,
            hovered_vortex_full_id: None,
            camera_position: [0.0, 0.0, 0.0],
            camera_target: [0.0, 0.0, 0.0],
            camera_zoom: 1.0,
            active_utility_id: String::new(),
            active_tool_id: None,""",
            """    pub selected_object_ids: Vec<String>,
    pub selected_vortex_ids: Vec<String>,
    pub selected_attraction_ids: Vec<String>,
    pub selected_target_volume_ids: Vec<String>,
    pub selected_reference_ids: Vec<String>,
    pub hovered_object_id: Option<String>,
    pub hovered_vortex_full_id: Option<String>,
    pub camera_position: [f64; 3],
    pub camera_target: [f64; 3],
    pub camera_zoom: f64,
    pub active_utility_id: String,
    pub active_tool_id: Option<String>,""",
        ),
    )

    print("done")


if __name__ == "__main__":
    main()
