#!/usr/bin/env python3
"""🧬 Emits the twenty `s.wfc.grid3d` schema-facet leaves (JSON Schema normative; GraphQL, Protobuf
and TypeScript mirror it field-for-field) plus the inference facet's five. Idempotent."""

import io
import json
import os

ROOT = "/Users/ueli/Documents/semio"
X = os.path.join(ROOT, "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any")
BASE = "https://json.schemas.assets.semio-tech.com/s/wfc/grid3d/1/any"


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    io.open(path, "w", encoding="utf-8").write(text)


def schema(path, body):
    write(path, json.dumps(body, ensure_ascii=False, indent=2) + "\n")


COLOR = {
    "title": "Grid3dColor", "type": "object", "additionalProperties": False,
    "required": ["r", "g", "b", "a"],
    "properties": {k: {"type": "integer", "minimum": 0, "maximum": 255} for k in "rgba"},
}
MESH = {
    "title": "Grid3dMesh", "type": "object", "additionalProperties": False,
    "required": ["positions", "indices"],
    "properties": {
        "positions": {"type": "array", "items": {"type": "number"}, "description": "xyz triples in the tile-space unit box 0..1"},
        "indices": {"type": "array", "items": {"type": "integer", "minimum": 0}},
        "color": COLOR,
    },
}
CHILD = {
    "title": "ArtifactChildHandle", "type": "object", "additionalProperties": False,
    "required": ["childId", "target"],
    "properties": {
        "childId": {"type": "string"},
        "target": {
            "type": "object", "additionalProperties": False, "required": ["artifactId", "dialect"],
            "properties": {
                "artifactId": {"type": "string"},
                "dialect": {
                    "type": "object", "additionalProperties": False, "required": ["artifactKind", "standard", "subset"],
                    "properties": {"artifactKind": {"type": "string"}, "standard": {"type": "string"}, "subset": {"type": "string"}},
                },
            },
        },
    },
}
MEDIA = {
    "title": "Grid3dTileMedia",
    "description": "Tagged by `kind`: inline `mesh` geometry, or a `meshChild` handle into s.stdio.semio@v1/mesh.",
    "oneOf": [
        {"type": "object", "additionalProperties": False, "required": ["kind", "mesh"], "properties": {"kind": {"const": "mesh"}, "mesh": MESH}},
        {"type": "object", "additionalProperties": False, "required": ["kind", "child"], "properties": {"kind": {"const": "meshChild"}, "child": CHILD}},
    ],
}
TILE = {
    "title": "Grid3dTile", "type": "object", "additionalProperties": False,
    "required": ["id", "weight", "media"],
    "properties": {"id": {"type": "string"}, "label": {"type": "string"}, "weight": {"type": "number", "exclusiveMinimum": 0}, "media": MEDIA},
}
DIRECTION = {"type": "string", "enum": ["LEFT", "RIGHT", "FRONT", "BACK", "BOTTOM", "TOP"]}
RULE = {
    "title": "Grid3dRule", "type": "object", "additionalProperties": False,
    "required": ["id", "tileAId", "tileBId", "direction", "allowed"],
    "properties": {"id": {"type": "string"}, "tileAId": {"type": "string"}, "tileBId": {"type": "string"}, "direction": DIRECTION, "allowed": {"type": "boolean"}},
}
PINNED = {
    "title": "Grid3dPinnedCell", "type": "object", "additionalProperties": False,
    "required": ["x", "y", "z", "tileId"],
    "properties": {"x": {"type": "integer", "minimum": 0}, "y": {"type": "integer", "minimum": 0}, "z": {"type": "integer", "minimum": 0}, "tileId": {"type": "string"}},
}
CELL = {
    "title": "Grid3dCell", "type": "object", "additionalProperties": False,
    "required": ["x", "y", "z"],
    "properties": {"x": {"type": "integer", "minimum": 0}, "y": {"type": "integer", "minimum": 0}, "z": {"type": "integer", "minimum": 0}},
}
SIZES = {"type": "array", "items": {"type": "number", "exclusiveMinimum": 0}}

SNAPSHOT = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": f"{BASE}/snapshot/schema.json",
    "title": "Grid3dSnapshot",
    "description": "The persisted WFC problem for a non-uniform box grid. The SOLVE is an inference over this document and is never stored here.",
    "type": "object",
    "additionalProperties": False,
    "required": ["schema", "seed", "width", "height", "depth", "cellSizesX", "cellSizesY", "cellSizesZ", "periodicX", "periodicY", "periodicZ", "tiles", "rules", "pinned", "masked"],
    "properties": {
        "schema": {"const": "s.wfc.grid3d"},
        "seed": {"type": "integer", "minimum": 0},
        "width": {"type": "integer", "minimum": 1},
        "height": {"type": "integer", "minimum": 1},
        "depth": {"type": "integer", "minimum": 1},
        "cellSizesX": SIZES,
        "cellSizesY": SIZES,
        "cellSizesZ": SIZES,
        "periodicX": {"type": "boolean"},
        "periodicY": {"type": "boolean"},
        "periodicZ": {"type": "boolean"},
        "tiles": {"type": "array", "items": TILE},
        "rules": {"type": "array", "items": RULE},
        "pinned": {"type": "array", "items": PINNED},
        "masked": {"type": "array", "items": CELL},
    },
}

ARTIFACT = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": f"{BASE}/artifact/schema.json",
    "title": "Grid3dArtifact",
    "description": "The artifact facet — the persisted problem spec IS the artifact; nothing is derived into it.",
    "type": "object",
    "additionalProperties": False,
    "required": ["snapshot"],
    "properties": {"snapshot": {"$ref": f"{BASE}/snapshot/schema.json"}},
}


def indexed(title, item):
    return {"type": "array", "items": {"title": title, "type": "array", "minItems": 2, "maxItems": 2, "items": [{"type": "integer", "minimum": 0}, item]}}


DIFF = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": f"{BASE}/diff/schema.json",
    "title": "Grid3dDiff",
    "description": "A sparse, key-keyed structural delta over Grid3dSnapshot. Every lane is stated explicitly, an untouched scalar as null and an untouched collection as an empty array.",
    "type": "object",
    "additionalProperties": False,
    "required": ["schema", "seed", "width", "height", "depth", "cellSizesX", "cellSizesY", "cellSizesZ", "periodicX", "periodicY", "periodicZ", "tilesRemoved", "tilesUpserted", "rulesRemoved", "rulesUpserted", "pinnedRemoved", "pinnedUpserted", "maskedRemoved", "maskedUpserted"],
    "properties": {
        "schema": {"type": ["string", "null"]},
        "seed": {"type": ["integer", "null"], "minimum": 0},
        "width": {"type": ["integer", "null"], "minimum": 1},
        "height": {"type": ["integer", "null"], "minimum": 1},
        "depth": {"type": ["integer", "null"], "minimum": 1},
        "cellSizesX": {"type": ["array", "null"], "items": {"type": "number", "exclusiveMinimum": 0}},
        "cellSizesY": {"type": ["array", "null"], "items": {"type": "number", "exclusiveMinimum": 0}},
        "cellSizesZ": {"type": ["array", "null"], "items": {"type": "number", "exclusiveMinimum": 0}},
        "periodicX": {"type": ["boolean", "null"]},
        "periodicY": {"type": ["boolean", "null"]},
        "periodicZ": {"type": ["boolean", "null"]},
        "tilesRemoved": {"type": "array", "items": {"type": "string"}},
        "tilesUpserted": indexed("Grid3dTileUpsert", TILE),
        "rulesRemoved": {"type": "array", "items": {"type": "string"}},
        "rulesUpserted": indexed("Grid3dRuleUpsert", RULE),
        "pinnedRemoved": {"type": "array", "items": {"type": "string"}},
        "pinnedUpserted": indexed("Grid3dPinnedUpsert", PINNED),
        "maskedRemoved": {"type": "array", "items": {"type": "string"}},
        "maskedUpserted": indexed("Grid3dMaskedUpsert", CELL),
    },
}

MUTATION_VARIANTS = [
    ("ChangeSeed", {"seed": {"type": "integer", "minimum": 0}}, ["seed"]),
    ("ResizeGrid", {"width": {"type": "integer", "minimum": 1}, "height": {"type": "integer", "minimum": 1}, "depth": {"type": "integer", "minimum": 1}}, ["width", "height", "depth"]),
    ("ChangeCellSizes", {"axis": {"type": "string", "enum": ["x", "y", "z"]}, "sizes": SIZES}, ["axis", "sizes"]),
    ("ChangePeriodicity", {"periodicX": {"type": "boolean"}, "periodicY": {"type": "boolean"}, "periodicZ": {"type": "boolean"}}, ["periodicX", "periodicY", "periodicZ"]),
    ("CreateTile", {"tile": TILE}, ["tile"]),
    ("DeleteTile", {"id": {"type": "string"}}, ["id"]),
    ("ChangeTileWeight", {"tileId": {"type": "string"}, "weight": {"type": "number", "exclusiveMinimum": 0}}, ["tileId", "weight"]),
    ("ChangeTileMedia", {"tileId": {"type": "string"}, "media": MEDIA}, ["tileId", "media"]),
    ("CreateRule", {"rule": RULE}, ["rule"]),
    ("DeleteRule", {"id": {"type": "string"}}, ["id"]),
    ("PinCell", {"pinned": PINNED}, ["pinned"]),
    ("UnpinCell", {"x": {"type": "integer", "minimum": 0}, "y": {"type": "integer", "minimum": 0}, "z": {"type": "integer", "minimum": 0}}, ["x", "y", "z"]),
    ("MaskCell", {"cell": CELL}, ["cell"]),
    ("UnmaskCell", {"x": {"type": "integer", "minimum": 0}, "y": {"type": "integer", "minimum": 0}, "z": {"type": "integer", "minimum": 0}}, ["x", "y", "z"]),
]

MUTATIONS = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": f"{BASE}/mutations/schema.json",
    "title": "Grid3dMutation",
    "description": "The semantic mutation vocabulary, internally tagged by its variant name — exactly the shape the committed fixture quintets carry.",
    "oneOf": [
        {"type": "object", "additionalProperties": False, "required": [name],
         "properties": {name: {"title": name, "type": "object", "additionalProperties": False, "required": required, "properties": properties}}}
        for name, properties, required in MUTATION_VARIANTS
    ],
}

INFERENCE = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": f"{BASE}/inferences/schema.json",
    "title": "Grid3dSolve",
    "description": "The s.wfc.grid3d.solve inference: request is the whole problem plus an optional resume checkpoint; the commit is one [x, y, z, tileId] row per solved, unmasked cell.",
    "type": "object",
    "additionalProperties": False,
    "required": ["request", "commit"],
    "properties": {
        "request": {
            "title": "Grid3dInferenceRequest", "type": "object", "additionalProperties": False, "required": ["snapshot"],
            "properties": {"snapshot": {"$ref": f"{BASE}/snapshot/schema.json"}, "checkpoint": {"type": ["array", "null"], "items": {"type": "integer", "minimum": 0, "maximum": 255}}},
        },
        "commit": {
            "title": "Grid3dInferenceCommit", "type": "object", "additionalProperties": False, "required": ["assignments"],
            "properties": {
                "assignments": {
                    "type": "array",
                    "items": {"title": "Grid3dAssignment", "type": "array", "minItems": 4, "maxItems": 4,
                              "items": [{"type": "integer", "minimum": 0}, {"type": "integer", "minimum": 0}, {"type": "integer", "minimum": 0}, {"type": "string"}]},
                }
            },
        },
    },
}

schema(os.path.join(X, "🧬️schema/🔣️.json"), ARTIFACT)
schema(os.path.join(X, "🧬️schema/📸️snapshot/🔣️.json"), SNAPSHOT)
schema(os.path.join(X, "🧬️schema/🔺️diff/🔣️.json"), DIFF)
schema(os.path.join(X, "🧬️schema/🧬️mutations/🔣️.json"), MUTATIONS)
schema(os.path.join(X, "🧬️schema/💡️inferences/🔣️.json"), INFERENCE)

GRAPHQL_COMMON = """# 🧬 s.wfc.grid3d — GraphQL mirror of the normative JSON Schema. Field names and optionality are
# transcribed one-for-one; GraphQL has no untagged union of objects, so `Grid3dTileMedia` is a real
# `union` over the two tagged members the JSON Schema's `oneOf` declares.

type Grid3dColor {
  r: Int!
  g: Int!
  b: Int!
  a: Int!
}

type Grid3dMesh {
  positions: [Float!]!
  indices: [Int!]!
  color: Grid3dColor
}

type ArtifactDialect {
  artifactKind: String!
  standard: String!
  subset: String!
}

type ArtifactRef {
  artifactId: String!
  dialect: ArtifactDialect!
}

type ArtifactChildHandle {
  childId: String!
  target: ArtifactRef!
}

type Grid3dTileMediaMesh {
  mesh: Grid3dMesh!
}

type Grid3dTileMediaMeshChild {
  child: ArtifactChildHandle!
}

union Grid3dTileMedia = Grid3dTileMediaMesh | Grid3dTileMediaMeshChild

type Grid3dTile {
  id: String!
  label: String
  weight: Float!
  media: Grid3dTileMedia!
}

enum Grid3dDirection {
  LEFT
  RIGHT
  FRONT
  BACK
  BOTTOM
  TOP
}

enum Grid3dAxis {
  X
  Y
  Z
}

type Grid3dRule {
  id: String!
  tileAId: String!
  tileBId: String!
  direction: Grid3dDirection!
  allowed: Boolean!
}

type Grid3dPinnedCell {
  x: Int!
  y: Int!
  z: Int!
  tileId: String!
}

type Grid3dCell {
  x: Int!
  y: Int!
  z: Int!
}
"""

write(os.path.join(X, "🧬️schema/📸️snapshot/🔗️.graphql"), GRAPHQL_COMMON + """
type Grid3dSnapshot {
  schema: String!
  seed: Int!
  width: Int!
  height: Int!
  depth: Int!
  cellSizesX: [Float!]!
  cellSizesY: [Float!]!
  cellSizesZ: [Float!]!
  periodicX: Boolean!
  periodicY: Boolean!
  periodicZ: Boolean!
  tiles: [Grid3dTile!]!
  rules: [Grid3dRule!]!
  pinned: [Grid3dPinnedCell!]!
  masked: [Grid3dCell!]!
}
""")

write(os.path.join(X, "🧬️schema/🔗️.graphql"), """# 🧬 s.wfc.grid3d artifact facet — the persisted problem spec IS the artifact.

type Grid3dArtifact {
  snapshot: Grid3dSnapshot!
}
""")

write(os.path.join(X, "🧬️schema/🔺️diff/🔗️.graphql"), """# 🔺 s.wfc.grid3d diff facet — a sparse, key-keyed delta. An indexed upsert is a pair of the FINAL
# state index and the member itself, which GraphQL models as a two-field type rather than a tuple.

type Grid3dTileUpsert {
  index: Int!
  tile: Grid3dTile!
}

type Grid3dRuleUpsert {
  index: Int!
  rule: Grid3dRule!
}

type Grid3dPinnedUpsert {
  index: Int!
  pinned: Grid3dPinnedCell!
}

type Grid3dMaskedUpsert {
  index: Int!
  cell: Grid3dCell!
}

type Grid3dDiff {
  schema: String
  seed: Int
  width: Int
  height: Int
  depth: Int
  cellSizesX: [Float!]
  cellSizesY: [Float!]
  cellSizesZ: [Float!]
  periodicX: Boolean
  periodicY: Boolean
  periodicZ: Boolean
  tilesRemoved: [String!]!
  tilesUpserted: [Grid3dTileUpsert!]!
  rulesRemoved: [String!]!
  rulesUpserted: [Grid3dRuleUpsert!]!
  pinnedRemoved: [String!]!
  pinnedUpserted: [Grid3dPinnedUpsert!]!
  maskedRemoved: [String!]!
  maskedUpserted: [Grid3dMaskedUpsert!]!
}
""")

mutation_types = "\n\n".join(
    "type {name} {{\n{fields}\n}}".format(
        name=name,
        fields="\n".join("  {key}: {gql}!".format(key=key, gql=GQL_SCALARS.get(key, "String")) for key in properties),
    )
    for name, properties, _ in []
) if False else ""

write(os.path.join(X, "🧬️schema/🧬️mutations/🔗️.graphql"), """# 🧬 s.wfc.grid3d mutation facet — one type per semantic kind, united by `Grid3dMutation`. The wire
# form is internally tagged by the variant name, exactly as the committed fixtures carry it.

type ChangeSeed {
  seed: Int!
}

type ResizeGrid {
  width: Int!
  height: Int!
  depth: Int!
}

type ChangeCellSizes {
  axis: Grid3dAxis!
  sizes: [Float!]!
}

type ChangePeriodicity {
  periodicX: Boolean!
  periodicY: Boolean!
  periodicZ: Boolean!
}

type CreateTile {
  tile: Grid3dTile!
}

type DeleteTile {
  id: String!
}

type ChangeTileWeight {
  tileId: String!
  weight: Float!
}

type ChangeTileMedia {
  tileId: String!
  media: Grid3dTileMedia!
}

type CreateRule {
  rule: Grid3dRule!
}

type DeleteRule {
  id: String!
}

type PinCell {
  pinned: Grid3dPinnedCell!
}

type UnpinCell {
  x: Int!
  y: Int!
  z: Int!
}

type MaskCell {
  cell: Grid3dCell!
}

type UnmaskCell {
  x: Int!
  y: Int!
  z: Int!
}

union Grid3dMutation =
    ChangeSeed
  | ResizeGrid
  | ChangeCellSizes
  | ChangePeriodicity
  | CreateTile
  | DeleteTile
  | ChangeTileWeight
  | ChangeTileMedia
  | CreateRule
  | DeleteRule
  | PinCell
  | UnpinCell
  | MaskCell
  | UnmaskCell
""")

write(os.path.join(X, "🧬️schema/💡️inferences/🔗️.graphql"), """# 💡 s.wfc.grid3d.solve — the routed solve inference. One assignment row per solved, unmasked cell.

type Grid3dAssignment {
  x: Int!
  y: Int!
  z: Int!
  tileId: String!
}

type Grid3dInferenceRequest {
  snapshot: Grid3dSnapshot!
  checkpoint: [Int!]
}

type Grid3dInferenceCommit {
  assignments: [Grid3dAssignment!]!
}
""")

PROTO_HEAD = """// 🧬 s.wfc.grid3d — Protobuf mirror of the normative JSON Schema. Field numbers are stable and
// follow the declaration order of the Rust records the schema is transcribed from.
syntax = "proto3";

package semio.s.wfc.grid3d.v1;
"""

write(os.path.join(X, "🧬️schema/📸️snapshot/🛰️.proto"), PROTO_HEAD + """
message Grid3dColor {
  uint32 r = 1;
  uint32 g = 2;
  uint32 b = 3;
  uint32 a = 4;
}

message Grid3dMesh {
  repeated double positions = 1;
  repeated uint32 indices = 2;
  optional Grid3dColor color = 3;
}

message ArtifactDialect {
  string artifact_kind = 1;
  string standard = 2;
  string subset = 3;
}

message ArtifactRef {
  string artifact_id = 1;
  ArtifactDialect dialect = 2;
}

message ArtifactChildHandle {
  string child_id = 1;
  ArtifactRef target = 2;
}

message Grid3dTileMedia {
  oneof kind {
    Grid3dMesh mesh = 1;
    ArtifactChildHandle child = 2;
  }
}

message Grid3dTile {
  string id = 1;
  optional string label = 2;
  double weight = 3;
  Grid3dTileMedia media = 4;
}

enum Grid3dDirection {
  GRID3D_DIRECTION_LEFT = 0;
  GRID3D_DIRECTION_RIGHT = 1;
  GRID3D_DIRECTION_FRONT = 2;
  GRID3D_DIRECTION_BACK = 3;
  GRID3D_DIRECTION_BOTTOM = 4;
  GRID3D_DIRECTION_TOP = 5;
}

enum Grid3dAxis {
  GRID3D_AXIS_X = 0;
  GRID3D_AXIS_Y = 1;
  GRID3D_AXIS_Z = 2;
}

message Grid3dRule {
  string id = 1;
  string tile_a_id = 2;
  string tile_b_id = 3;
  Grid3dDirection direction = 4;
  bool allowed = 5;
}

message Grid3dPinnedCell {
  uint32 x = 1;
  uint32 y = 2;
  uint32 z = 3;
  string tile_id = 4;
}

message Grid3dCell {
  uint32 x = 1;
  uint32 y = 2;
  uint32 z = 3;
}

message Grid3dSnapshot {
  string schema = 1;
  uint64 seed = 2;
  uint32 width = 3;
  uint32 height = 4;
  uint32 depth = 5;
  repeated double cell_sizes_x = 6;
  repeated double cell_sizes_y = 7;
  repeated double cell_sizes_z = 8;
  bool periodic_x = 9;
  bool periodic_y = 10;
  bool periodic_z = 11;
  repeated Grid3dTile tiles = 12;
  repeated Grid3dRule rules = 13;
  repeated Grid3dPinnedCell pinned = 14;
  repeated Grid3dCell masked = 15;
}
""")

write(os.path.join(X, "🧬️schema/🛰️.proto"), PROTO_HEAD + """
import "snapshot.proto";

message Grid3dArtifact {
  Grid3dSnapshot snapshot = 1;
}
""")

write(os.path.join(X, "🧬️schema/🔺️diff/🛰️.proto"), PROTO_HEAD + """
import "snapshot.proto";

message Grid3dTileUpsert {
  uint32 index = 1;
  Grid3dTile tile = 2;
}

message Grid3dRuleUpsert {
  uint32 index = 1;
  Grid3dRule rule = 2;
}

message Grid3dPinnedUpsert {
  uint32 index = 1;
  Grid3dPinnedCell pinned = 2;
}

message Grid3dMaskedUpsert {
  uint32 index = 1;
  Grid3dCell cell = 2;
}

message Grid3dDiff {
  optional string schema = 1;
  optional uint64 seed = 2;
  optional uint32 width = 3;
  optional uint32 height = 4;
  optional uint32 depth = 5;
  repeated double cell_sizes_x = 6;
  repeated double cell_sizes_y = 7;
  repeated double cell_sizes_z = 8;
  optional bool periodic_x = 9;
  optional bool periodic_y = 10;
  optional bool periodic_z = 11;
  repeated string tiles_removed = 12;
  repeated Grid3dTileUpsert tiles_upserted = 13;
  repeated string rules_removed = 14;
  repeated Grid3dRuleUpsert rules_upserted = 15;
  repeated string pinned_removed = 16;
  repeated Grid3dPinnedUpsert pinned_upserted = 17;
  repeated string masked_removed = 18;
  repeated Grid3dMaskedUpsert masked_upserted = 19;
}
""")

write(os.path.join(X, "🧬️schema/🧬️mutations/🛰️.proto"), PROTO_HEAD + """
import "snapshot.proto";

message ChangeSeed {
  uint64 seed = 1;
}

message ResizeGrid {
  uint32 width = 1;
  uint32 height = 2;
  uint32 depth = 3;
}

message ChangeCellSizes {
  Grid3dAxis axis = 1;
  repeated double sizes = 2;
}

message ChangePeriodicity {
  bool periodic_x = 1;
  bool periodic_y = 2;
  bool periodic_z = 3;
}

message CreateTile {
  Grid3dTile tile = 1;
}

message DeleteTile {
  string id = 1;
}

message ChangeTileWeight {
  string tile_id = 1;
  double weight = 2;
}

message ChangeTileMedia {
  string tile_id = 1;
  Grid3dTileMedia media = 2;
}

message CreateRule {
  Grid3dRule rule = 1;
}

message DeleteRule {
  string id = 1;
}

message PinCell {
  Grid3dPinnedCell pinned = 1;
}

message UnpinCell {
  uint32 x = 1;
  uint32 y = 2;
  uint32 z = 3;
}

message MaskCell {
  Grid3dCell cell = 1;
}

message UnmaskCell {
  uint32 x = 1;
  uint32 y = 2;
  uint32 z = 3;
}

message Grid3dMutation {
  oneof kind {
    ChangeSeed change_seed = 1;
    ResizeGrid resize_grid = 2;
    ChangeCellSizes change_cell_sizes = 3;
    ChangePeriodicity change_periodicity = 4;
    CreateTile create_tile = 5;
    DeleteTile delete_tile = 6;
    ChangeTileWeight change_tile_weight = 7;
    ChangeTileMedia change_tile_media = 8;
    CreateRule create_rule = 9;
    DeleteRule delete_rule = 10;
    PinCell pin_cell = 11;
    UnpinCell unpin_cell = 12;
    MaskCell mask_cell = 13;
    UnmaskCell unmask_cell = 14;
  }
}
""")

write(os.path.join(X, "🧬️schema/💡️inferences/🛰️.proto"), PROTO_HEAD + """
import "snapshot.proto";

message Grid3dAssignment {
  uint32 x = 1;
  uint32 y = 2;
  uint32 z = 3;
  string tile_id = 4;
}

message Grid3dInferenceRequest {
  Grid3dSnapshot snapshot = 1;
  bytes checkpoint = 2;
}

message Grid3dInferenceCommit {
  repeated Grid3dAssignment assignments = 1;
}
""")

TS_SNAPSHOT = """/** 🧬 s.wfc.grid3d snapshot — the TypeScript twin of the normative JSON Schema, ported field by
 * field (never generated). The wire form is camelCase, exactly as the Rust `#[value(rename_all =
 * "camelCase")]` records emit it. */

export const WFC_GRID3D_DOCUMENT_SCHEMA = "s.wfc.grid3d";

export interface Grid3dColor {
  r: number;
  g: number;
  b: number;
  a: number;
}

export interface Grid3dMesh {
  positions: number[];
  indices: number[];
  color?: Grid3dColor;
}

export interface ArtifactChildHandle {
  childId: string;
  target: { artifactId: string; dialect: { artifactKind: string; standard: string; subset: string } };
}

export type Grid3dTileMedia = { kind: "mesh"; mesh: Grid3dMesh } | { kind: "meshChild"; child: ArtifactChildHandle };

export interface Grid3dTile {
  id: string;
  label?: string;
  weight: number;
  media: Grid3dTileMedia;
}

export type Grid3dDirection = "LEFT" | "RIGHT" | "FRONT" | "BACK" | "BOTTOM" | "TOP";
export type Grid3dAxis = "x" | "y" | "z";

export interface Grid3dRule {
  id: string;
  tileAId: string;
  tileBId: string;
  direction: Grid3dDirection;
  allowed: boolean;
}

export interface Grid3dPinnedCell {
  x: number;
  y: number;
  z: number;
  tileId: string;
}

export interface Grid3dCell {
  x: number;
  y: number;
  z: number;
}

export interface Grid3dSnapshot {
  schema: string;
  seed: number;
  width: number;
  height: number;
  depth: number;
  cellSizesX: number[];
  cellSizesY: number[];
  cellSizesZ: number[];
  periodicX: boolean;
  periodicY: boolean;
  periodicZ: boolean;
  tiles: Grid3dTile[];
  rules: Grid3dRule[];
  pinned: Grid3dPinnedCell[];
  masked: Grid3dCell[];
}

/** 📐 Cell `index`'s lower world coordinate on one axis — the cumulative sum of every size before it.
 * The twin of the Rust `axis_offset`, so a TypeScript consumer places a non-uniform cell identically. */
export function axisOffset(sizes: number[], index: number): number {
  let total = 0;
  for (let cursor = 0; cursor < Math.min(index, sizes.length); cursor += 1) total += sizes[cursor];
  return total;
}

/** 📐 Cell `index`'s own size, defaulting to a unit cell for an index the array does not reach. */
export function axisSize(sizes: number[], index: number): number {
  const size = sizes[index];
  return Number.isFinite(size) && size > 0 ? size : 1;
}

/** 📐 The canonical sort key of one pinned or masked cell. */
export function cellKey(x: number, y: number, z: number): string {
  return `${x}:${y}:${z}`;
}

export function emptyGrid3dSnapshot(): Grid3dSnapshot {
  return {
    schema: WFC_GRID3D_DOCUMENT_SCHEMA,
    seed: 0,
    width: 1,
    height: 1,
    depth: 1,
    cellSizesX: [1],
    cellSizesY: [1],
    cellSizesZ: [1],
    periodicX: false,
    periodicY: false,
    periodicZ: false,
    tiles: [],
    rules: [],
    pinned: [],
    masked: [],
  };
}
"""

write(os.path.join(X, "🧬️schema/📸️snapshot/🟦️.ts"), TS_SNAPSHOT)

write(os.path.join(X, "🧬️schema/🟦️.ts"), """/** 🧬 s.wfc.grid3d artifact facet — the persisted problem spec IS the artifact. */

import type { Grid3dSnapshot } from "./📸️snapshot/🟦️.ts";

export interface Grid3dArtifact {
  snapshot: Grid3dSnapshot;
}
""")

write(os.path.join(X, "🧬️schema/🔺️diff/🟦️.ts"), """/** 🔺 s.wfc.grid3d diff — a sparse, key-keyed delta. An indexed upsert rides as a `[index, member]`
 * pair, exactly as the Rust `Vec<(usize, T)>` lanes encode it. */

import type { Grid3dCell, Grid3dPinnedCell, Grid3dRule, Grid3dSnapshot, Grid3dTile } from "../📸️snapshot/🟦️.ts";

export type Indexed<T> = [number, T];

export interface Grid3dDiff {
  schema: string | null;
  seed: number | null;
  width: number | null;
  height: number | null;
  depth: number | null;
  cellSizesX: number[] | null;
  cellSizesY: number[] | null;
  cellSizesZ: number[] | null;
  periodicX: boolean | null;
  periodicY: boolean | null;
  periodicZ: boolean | null;
  tilesRemoved: string[];
  tilesUpserted: Indexed<Grid3dTile>[];
  rulesRemoved: string[];
  rulesUpserted: Indexed<Grid3dRule>[];
  pinnedRemoved: string[];
  pinnedUpserted: Indexed<Grid3dPinnedCell>[];
  maskedRemoved: string[];
  maskedUpserted: Indexed<Grid3dCell>[];
}

export function emptyGrid3dDiff(): Grid3dDiff {
  return {
    schema: null,
    seed: null,
    width: null,
    height: null,
    depth: null,
    cellSizesX: null,
    cellSizesY: null,
    cellSizesZ: null,
    periodicX: null,
    periodicY: null,
    periodicZ: null,
    tilesRemoved: [],
    tilesUpserted: [],
    rulesRemoved: [],
    rulesUpserted: [],
    pinnedRemoved: [],
    pinnedUpserted: [],
    maskedRemoved: [],
    maskedUpserted: [],
  };
}

/** 🧬 Applies one key-keyed collection delta — the twin of the Rust `apply_collection`, so the
 * cross-language fixture oracle compares two real implementations rather than one and a stub. */
export function applyCollection<T>(base: T[], removed: string[], upserted: Indexed<T>[], key: (item: T) => string): T[] {
  const items = base.filter((item) => !removed.includes(key(item)));
  for (const [index, value] of upserted) {
    const at = items.findIndex((item) => key(item) === key(value));
    if (at >= 0) items[at] = value;
    else items.splice(index, 0, value);
  }
  return items;
}

/** 🔺 Applies a whole diff to a snapshot. */
export function applyGrid3dDiff(base: Grid3dSnapshot, diff: Grid3dDiff): Grid3dSnapshot {
  const cell = (item: { x: number; y: number; z: number }) => `${item.x}:${item.y}:${item.z}`;
  return {
    ...base,
    schema: diff.schema ?? base.schema,
    seed: diff.seed ?? base.seed,
    width: diff.width ?? base.width,
    height: diff.height ?? base.height,
    depth: diff.depth ?? base.depth,
    cellSizesX: diff.cellSizesX ?? base.cellSizesX,
    cellSizesY: diff.cellSizesY ?? base.cellSizesY,
    cellSizesZ: diff.cellSizesZ ?? base.cellSizesZ,
    periodicX: diff.periodicX ?? base.periodicX,
    periodicY: diff.periodicY ?? base.periodicY,
    periodicZ: diff.periodicZ ?? base.periodicZ,
    tiles: applyCollection(base.tiles, diff.tilesRemoved, diff.tilesUpserted, (tile) => tile.id),
    rules: applyCollection(base.rules, diff.rulesRemoved, diff.rulesUpserted, (rule) => rule.id),
    pinned: applyCollection(base.pinned, diff.pinnedRemoved, diff.pinnedUpserted, cell),
    masked: applyCollection(base.masked, diff.maskedRemoved, diff.maskedUpserted, cell),
  };
}
""")

write(os.path.join(X, "🧬️schema/🧬️mutations/🟦️.ts"), """/** 🧬 s.wfc.grid3d mutations — the TypeScript twin of the semantic vocabulary. The wire form is
 * internally tagged by the variant name, exactly as the committed fixture quintets carry it. */

import type { Grid3dAxis, Grid3dCell, Grid3dPinnedCell, Grid3dRule, Grid3dTile, Grid3dTileMedia } from "../📸️snapshot/🟦️.ts";

export type Grid3dMutation =
  | { ChangeSeed: { seed: number } }
  | { ResizeGrid: { width: number; height: number; depth: number } }
  | { ChangeCellSizes: { axis: Grid3dAxis; sizes: number[] } }
  | { ChangePeriodicity: { periodicX: boolean; periodicY: boolean; periodicZ: boolean } }
  | { CreateTile: { tile: Grid3dTile } }
  | { DeleteTile: { id: string } }
  | { ChangeTileWeight: { tileId: string; weight: number } }
  | { ChangeTileMedia: { tileId: string; media: Grid3dTileMedia } }
  | { CreateRule: { rule: Grid3dRule } }
  | { DeleteRule: { id: string } }
  | { PinCell: { pinned: Grid3dPinnedCell } }
  | { UnpinCell: { x: number; y: number; z: number } }
  | { MaskCell: { cell: Grid3dCell } }
  | { UnmaskCell: { x: number; y: number; z: number } };

/** 🏷️ The kebab-case spelling of every variant, in declaration order — the same roster the Rust
 * `KINDS` const and the oracle catalog declare. */
export const GRID3D_MUTATION_KINDS = [
  "change-seed",
  "resize-grid",
  "change-cell-sizes",
  "change-periodicity",
  "create-tile",
  "delete-tile",
  "change-tile-weight",
  "change-tile-media",
  "create-rule",
  "delete-rule",
  "pin-cell",
  "unpin-cell",
  "mask-cell",
  "unmask-cell",
] as const;

export type Grid3dMutationKind = (typeof GRID3D_MUTATION_KINDS)[number];

/** 🏷️ The semantic kind of one wire mutation. */
export function grid3dMutationKind(mutation: Grid3dMutation): Grid3dMutationKind {
  const variant = Object.keys(mutation)[0];
  const kebab = variant.replace(/([a-z0-9])([A-Z])/g, "$1-$2").toLowerCase();
  return kebab as Grid3dMutationKind;
}

/** 📐 Where a key belongs in an already-sorted collection — the twin of the Rust `ordered_index`. */
export function orderedIndex<T>(items: T[], key: string, itemKey: (item: T) => string): number {
  const at = items.findIndex((item) => itemKey(item) >= key);
  return at < 0 ? items.length : at;
}
""")

write(os.path.join(X, "🧬️schema/💡️inferences/🟦️.ts"), """/** 💡 s.wfc.grid3d.solve — the routed solve inference's wire twin. The commit is one
 * `[x, y, z, tileId]` row per solved, unmasked cell; masked cells never appear. */

import type { Grid3dSnapshot } from "../📸️snapshot/🟦️.ts";

export const GRID3D_INFERENCE_TOOL_ID = "s.wfc.grid3d.solve";
export const GRID3D_INFERENCE_PAYLOAD_SCHEMA = "s.wfc.grid3d.inference.request.v1";

export type Grid3dAssignment = [number, number, number, string];

export interface Grid3dInferenceRequest {
  snapshot: Grid3dSnapshot;
  checkpoint?: number[] | null;
}

export interface Grid3dInferenceCommit {
  assignments: Grid3dAssignment[];
}

/** 🩺 Whether a commit covers exactly the cells the grid has to fill. */
export function coversEveryUnmaskedCell(snapshot: Grid3dSnapshot, commit: Grid3dInferenceCommit): boolean {
  const cells = snapshot.width * snapshot.height * snapshot.depth - snapshot.masked.length;
  return commit.assignments.length === cells;
}
""")

print("wrote the schema leaves under", X)
