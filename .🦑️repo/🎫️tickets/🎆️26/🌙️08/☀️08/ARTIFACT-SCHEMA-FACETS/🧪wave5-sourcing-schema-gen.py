#!/usr/bin/env python3
"""🧪 Emit fifteen curate schema leaves (ticket-local generator)."""
from __future__ import annotations
import json
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate")
SCHEMA_ID = "s.sourcing.curate"

GEOMETRY_ONEOF = [
    {
        "type": "object",
        "additionalProperties": False,
        "required": ["kind", "width", "height", "depth"],
        "properties": {
            "kind": {"const": "box"},
            "width": {"type": "number", "format": "double"},
            "height": {"type": "number", "format": "double"},
            "depth": {"type": "number", "format": "double"},
        },
    },
    {
        "type": "object",
        "additionalProperties": False,
        "required": ["kind", "width", "height", "depth", "profile"],
        "properties": {
            "kind": {"const": "frame"},
            "width": {"type": "number", "format": "double"},
            "height": {"type": "number", "format": "double"},
            "depth": {"type": "number", "format": "double"},
            "profile": {"type": "number", "format": "double"},
        },
    },
    {
        "type": "object",
        "additionalProperties": False,
        "required": ["kind", "width", "depth", "thickness"],
        "properties": {
            "kind": {"const": "slab"},
            "width": {"type": "number", "format": "double"},
            "depth": {"type": "number", "format": "double"},
            "thickness": {"type": "number", "format": "double"},
        },
    },
    {
        "type": "object",
        "additionalProperties": False,
        "required": ["kind", "positions", "normals", "indices"],
        "properties": {
            "kind": {"const": "mesh"},
            "positions": {"type": "array", "items": {"type": "number", "format": "float"}},
            "normals": {"type": "array", "items": {"type": "number", "format": "float"}},
            "indices": {"type": "array", "items": {"type": "integer", "format": "uint32"}},
        },
    },
]

SHARED_DEFS = {
    "GeometryRecipe": {
        "title": "GeometryRecipe",
        "oneOf": GEOMETRY_ONEOF,
    },
    "ObjectKind": {
        "title": "ObjectKind",
        "type": "object",
        "additionalProperties": False,
        "required": ["id", "name", "moduleId", "typologyPath", "availability", "geometry"],
        "properties": {
            "id": {"type": "string"},
            "name": {"type": "string"},
            "moduleId": {"type": "string"},
            "typologyPath": {"type": "array", "items": {"type": "string"}},
            "availability": {"type": "integer", "format": "uint32"},
            "geometry": {"$ref": "#/$defs/GeometryRecipe"},
        },
    },
    "CuratedItem": {
        "title": "CuratedItem",
        "type": "object",
        "additionalProperties": False,
        "required": ["objectId", "count"],
        "properties": {
            "objectId": {"type": "string"},
            "count": {"type": "integer", "format": "uint32"},
        },
    },
    "SortDirection": {
        "title": "SortDirection",
        "type": "string",
        "enum": ["asc", "desc"],
    },
    "TableSort": {
        "title": "TableSort",
        "type": "object",
        "additionalProperties": False,
        "required": ["columnId", "direction"],
        "properties": {
            "columnId": {"type": "string"},
            "direction": {"$ref": "#/$defs/SortDirection"},
        },
    },
    "Filters": {
        "title": "Filters",
        "type": "object",
        "additionalProperties": False,
        "required": ["query", "moduleIds", "typologyPath", "minAvailability"],
        "properties": {
            "query": {"type": "string"},
            "moduleIds": {"type": "array", "items": {"type": "string"}},
            "typologyPath": {"type": "array", "items": {"type": "string"}},
            "minAvailability": {"type": "integer", "format": "uint32"},
            "sort": {"$ref": "#/$defs/TableSort"},
        },
    },
}


def write_json(path: Path, doc: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(doc, indent=2) + "\n", encoding="utf-8")


def artifact_json() -> dict:
    return {
        "$id": f"https://semio.tech/schema/s/sourcing/curate/artifact.json",
        "title": "CurateArtifact",
        "type": "object",
        "additionalProperties": False,
        "required": ["stock", "curated", "filters", "locale", "contributionsJson"],
        "properties": {
            "stock": {
                "type": "array",
                "items": {"$ref": "#/$defs/ObjectKind"},
                "x-semio-state": "persistent",
            },
            "curated": {
                "type": "array",
                "items": {"$ref": "#/$defs/CuratedItem"},
                "x-semio-state": "persistent",
            },
            "filters": {"$ref": "#/$defs/Filters", "x-semio-state": "local-ui"},
            "selectedObjectId": {"type": "string", "x-semio-state": "shared-ui"},
            "locale": {"type": "string", "x-semio-state": "local-ui"},
            "contributionsJson": {"type": "string", "x-semio-state": "local-ui"},
        },
        "$defs": SHARED_DEFS,
    }


def snapshot_json() -> dict:
    return {
        "$id": f"https://semio.tech/schema/s/sourcing/curate/snapshot.json",
        "title": "CurateSnapshot",
        "type": "object",
        "additionalProperties": False,
        "required": ["stock", "curated"],
        "properties": {
            "stock": {
                "type": "array",
                "items": {"$ref": "#/$defs/ObjectKind"},
                "x-semio-state": "persistent",
            },
            "curated": {
                "type": "array",
                "items": {"$ref": "#/$defs/CuratedItem"},
                "x-semio-state": "persistent",
            },
        },
        "$defs": {
            k: SHARED_DEFS[k]
            for k in ("GeometryRecipe", "ObjectKind", "CuratedItem")
        },
    }


def diff_json() -> dict:
    return {
        "$id": f"https://semio.tech/schema/s/sourcing/curate/diff.json",
        "title": "CurateDiff",
        "type": "object",
        "additionalProperties": False,
        "required": [],
        "properties": {
            "artifact": {"title": "CurateArtifact", "type": "object", "x-semio-state": "persistent"},
            "stock": {"$ref": "#/$defs/CurateStockDelta", "x-semio-state": "persistent"},
            "curated": {"$ref": "#/$defs/CurateCuratedDelta", "x-semio-state": "persistent"},
            "filters": {"$ref": "#/$defs/Filters", "x-semio-state": "local-ui"},
            "selectedObjectId": {
                "oneOf": [{"type": "null"}, {"type": "string"}],
                "x-semio-state": "shared-ui",
            },
            "locale": {"type": "string", "x-semio-state": "local-ui"},
            "contributionsJson": {"type": "string", "x-semio-state": "local-ui"},
        },
        "$defs": {
            **SHARED_DEFS,
            "CurateStringList": {
                "title": "CurateStringList",
                "type": "object",
                "additionalProperties": False,
                "required": ["values"],
                "properties": {"values": {"type": "array", "items": {"type": "string"}}},
            },
            "CurateStockDelta": {
                "title": "CurateStockDelta",
                "type": "object",
                "additionalProperties": False,
                "required": [],
                "properties": {
                    "added": {"type": "array", "items": {"$ref": "#/$defs/ObjectKind"}},
                    "removed": {"type": "array", "items": {"type": "string"}},
                    "patched": {
                        "type": "array",
                        "items": {"$ref": "#/$defs/CurateObjectKindPatchEntry"},
                    },
                    "reordered": {"type": "array", "items": {"type": "string"}},
                },
            },
            "CurateObjectKindPatchEntry": {
                "title": "CurateObjectKindPatchEntry",
                "type": "object",
                "additionalProperties": False,
                "required": ["id", "kind"],
                "properties": {
                    "id": {"type": "string"},
                    "kind": {"$ref": "#/$defs/ObjectKind"},
                },
            },
            "CurateCuratedDelta": {
                "title": "CurateCuratedDelta",
                "type": "object",
                "additionalProperties": False,
                "required": [],
                "properties": {
                    "added": {"type": "array", "items": {"$ref": "#/$defs/CuratedItem"}},
                    "removed": {"type": "array", "items": {"type": "string"}},
                    "patched": {
                        "type": "array",
                        "items": {"$ref": "#/$defs/CurateCuratedPatchEntry"},
                    },
                    "reordered": {"type": "array", "items": {"type": "string"}},
                },
            },
            "CurateCuratedPatchEntry": {
                "title": "CurateCuratedPatchEntry",
                "type": "object",
                "additionalProperties": False,
                "required": ["objectId"],
                "properties": {
                    "objectId": {"type": "string"},
                    "count": {"type": "integer", "format": "uint32"},
                },
            },
        },
    }


TS_SHARED = '''
export type SortDirection = "asc" | "desc";

export interface TableSort {
  columnId: string;
  direction: SortDirection;
}

export interface Filters {
  query: string;
  moduleIds: string[];
  typologyPath: string[];
  minAvailability: number;
  sort?: TableSort | null;
}

export type GeometryRecipe =
  | { kind: "box"; width: number; height: number; depth: number }
  | { kind: "frame"; width: number; height: number; depth: number; profile: number }
  | { kind: "slab"; width: number; depth: number; thickness: number }
  | { kind: "mesh"; positions: number[]; normals: number[]; indices: number[] };

export interface ObjectKind {
  id: string;
  name: string;
  moduleId: string;
  typologyPath: string[];
  availability: number;
  geometry: GeometryRecipe;
}

export interface CuratedItem {
  objectId: string;
  count: number;
}
'''

GQL_SHARED = '''
enum SortDirection {
  ASC
  DESC
}

type TableSort {
  columnId: String!
  direction: SortDirection!
}

type Filters {
  query: String!
  moduleIds: [String!]!
  typologyPath: [String!]!
  minAvailability: Int!
  sort: TableSort
}

union GeometryRecipe =
  | GeometryRecipeBox
  | GeometryRecipeFrame
  | GeometryRecipeSlab
  | GeometryRecipeMesh

type GeometryRecipeBox {
  kind: String!
  width: Float!
  height: Float!
  depth: Float!
}

type GeometryRecipeFrame {
  kind: String!
  width: Float!
  height: Float!
  depth: Float!
  profile: Float!
}

type GeometryRecipeSlab {
  kind: String!
  width: Float!
  depth: Float!
  thickness: Float!
}

type GeometryRecipeMesh {
  kind: String!
  positions: [Float!]!
  normals: [Float!]!
  indices: [Int!]!
}

type ObjectKind {
  id: String!
  name: String!
  moduleId: String!
  typologyPath: [String!]!
  availability: Int!
  geometry: GeometryRecipe!
}

type CuratedItem {
  objectId: String!
  count: Int!
}
'''


def write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def main() -> None:
    write_json(ROOT / "🧬️schema/🔣️component.json", artifact_json())
    write_json(ROOT / "📸️snapshot/🧬️schema/🔣️component.json", snapshot_json())
    write_json(ROOT / "🔺️diff/🧬️schema/🔣️component.json", diff_json())

    artifact_ts = (
        "/** 🧬️ Curate artifact schema — every field with its state class. */\n"
        + TS_SHARED
        + """
export interface CurateArtifact {
  /** @state persistent */
  stock: ObjectKind[];
  /** @state persistent */
  curated: CuratedItem[];
  /** @state local-ui */
  filters: Filters;
  /** @state shared-ui */
  selectedObjectId?: string | null;
  /** @state local-ui */
  locale: string;
  /** @state local-ui */
  contributionsJson: string;
}
"""
    )
    snapshot_ts = (
        "/** 🧬️ Curate snapshot schema — persistent fields only. */\n"
        + TS_SHARED
        + """
export interface CurateSnapshot {
  /** @state persistent */
  stock: ObjectKind[];
  /** @state persistent */
  curated: CuratedItem[];
}
"""
    )
    diff_ts = (
        "/** 🧬️ Curate diff schema — sparse field delta. */\n"
        + TS_SHARED
        + """
export interface CurateStringList {
  values: string[];
}

export interface CurateObjectKindPatchEntry {
  id: string;
  kind: ObjectKind;
}

export interface CurateStockDelta {
  added?: ObjectKind[];
  removed?: string[];
  patched?: CurateObjectKindPatchEntry[];
  reordered?: string[];
}

export interface CurateCuratedPatchEntry {
  objectId: string;
  count?: number;
}

export interface CurateCuratedDelta {
  added?: CuratedItem[];
  removed?: string[];
  patched?: CurateCuratedPatchEntry[];
  reordered?: string[];
}

export interface CurateDiff {
  /** @state persistent */
  artifact?: CurateArtifact | null;
  /** @state persistent */
  stock?: CurateStockDelta | null;
  /** @state persistent */
  curated?: CurateCuratedDelta | null;
  /** @state local-ui */
  filters?: Filters | null;
  /** @state shared-ui */
  selectedObjectId?: string | null;
  /** @state local-ui */
  locale?: string | null;
  /** @state local-ui */
  contributionsJson?: string | null;
}
"""
    )
    write_text(ROOT / "🧬️schema/🟦️component.ts", artifact_ts)
    write_text(ROOT / "📸️snapshot/🧬️schema/🟦️component.ts", snapshot_ts)
    write_text(ROOT / "🔺️diff/🧬️schema/🟦️component.ts", diff_ts)

    artifact_gql = (
        "# 🧬️ Curate artifact schema — every field with its state class.\n"
        + GQL_SHARED
        + """
type CurateArtifact {
  stock: [ObjectKind!]! @state(class: PERSISTENT)
  curated: [CuratedItem!]! @state(class: PERSISTENT)
  filters: Filters! @state(class: LOCAL_UI)
  selectedObjectId: String @state(class: SHARED_UI)
  locale: String! @state(class: LOCAL_UI)
  contributionsJson: String! @state(class: LOCAL_UI)
}
"""
    )
    snapshot_gql = (
        "# 🧬️ Curate snapshot schema — persistent fields only.\n"
        + GQL_SHARED
        + """
type CurateSnapshot {
  stock: [ObjectKind!]! @state(class: PERSISTENT)
  curated: [CuratedItem!]! @state(class: PERSISTENT)
}
"""
    )
    diff_gql = (
        "# 🧬️ Curate diff schema — sparse field delta.\n"
        + GQL_SHARED
        + """
type CurateStringList {
  values: [String!]!
}

type CurateObjectKindPatchEntry {
  id: String!
  kind: ObjectKind!
}

type CurateStockDelta {
  added: [ObjectKind!]
  removed: [String!]
  patched: [CurateObjectKindPatchEntry!]
  reordered: [String!]
}

type CurateCuratedPatchEntry {
  objectId: String!
  count: Int
}

type CurateCuratedDelta {
  added: [CuratedItem!]
  removed: [String!]
  patched: [CurateCuratedPatchEntry!]
  reordered: [String!]
}

type CurateDiff {
  artifact: CurateArtifact @state(class: PERSISTENT)
  stock: CurateStockDelta @state(class: PERSISTENT)
  curated: CurateCuratedDelta @state(class: PERSISTENT)
  filters: Filters @state(class: LOCAL_UI)
  selectedObjectId: String @state(class: SHARED_UI)
  locale: String @state(class: LOCAL_UI)
  contributionsJson: String @state(class: LOCAL_UI)
}
"""
    )
    write_text(ROOT / "🧬️schema/🔗️component.graphql", artifact_gql)
    write_text(ROOT / "📸️snapshot/🧬️schema/🔗️component.graphql", snapshot_gql)
    write_text(ROOT / "🔺️diff/🧬️schema/🔗️component.graphql", diff_gql)

    proto_shared = '''
message TableSort {
  string column_id = 1;
  string direction = 2;
}

message Filters {
  string query = 1;
  repeated string module_ids = 2;
  repeated string typology_path = 3;
  uint32 min_availability = 4;
  optional TableSort sort = 5;
}

message GeometryRecipeBox {
  string kind = 1;
  double width = 2;
  double height = 3;
  double depth = 4;
}

message GeometryRecipeFrame {
  string kind = 1;
  double width = 2;
  double height = 3;
  double depth = 4;
  double profile = 5;
}

message GeometryRecipeSlab {
  string kind = 1;
  double width = 2;
  double depth = 3;
  double thickness = 4;
}

message GeometryRecipeMesh {
  string kind = 1;
  repeated float positions = 2;
  repeated float normals = 3;
  repeated uint32 indices = 4;
}

message ObjectKind {
  string id = 1;
  string name = 2;
  string module_id = 3;
  repeated string typology_path = 4;
  uint32 availability = 5;
  string geometry_json = 6;
}

message CuratedItem {
  string object_id = 1;
  uint32 count = 2;
}
'''
    # proto uses geometry_json as scalar stand-in for tagged union parity — rust leaf uses real enum
    artifact_proto = (
        f"syntax = \"proto3\";\npackage semio.s.sourcing.curate.artifact;\n\n"
        f"// 🧬️ Curate artifact schema — every field with its state class.\n"
        + proto_shared
        + """
message CurateArtifact {
  // @state persistent
  repeated ObjectKind stock = 1;
  // @state persistent
  repeated CuratedItem curated = 2;
  // @state local-ui
  Filters filters = 3;
  // @state shared-ui
  optional string selected_object_id = 4;
  // @state local-ui
  string locale = 5;
  // @state local-ui
  string contributions_json = 6;
}
"""
    )
    snapshot_proto = (
        f"syntax = \"proto3\";\npackage semio.s.sourcing.curate.snapshot;\n\n"
        f"// 🧬️ Curate snapshot schema — persistent fields only.\n"
        + proto_shared
        + """
message CurateSnapshot {
  // @state persistent
  repeated ObjectKind stock = 1;
  // @state persistent
  repeated CuratedItem curated = 2;
}
"""
    )
    diff_proto = (
        f"syntax = \"proto3\";\npackage semio.s.sourcing.curate.diff;\n\n"
        f"// 🧬️ Curate diff schema — sparse field delta.\n"
        + proto_shared
        + """
message CurateStringList {
  repeated string values = 1;
}

message CurateObjectKindPatchEntry {
  string id = 1;
  ObjectKind kind = 2;
}

message CurateStockDelta {
  repeated ObjectKind added = 1;
  repeated string removed = 2;
  repeated CurateObjectKindPatchEntry patched = 3;
  repeated string reordered = 4;
}

message CurateCuratedPatchEntry {
  string object_id = 1;
  optional uint32 count = 2;
}

message CurateCuratedDelta {
  repeated CuratedItem added = 1;
  repeated string removed = 2;
  repeated CurateCuratedPatchEntry patched = 3;
  repeated string reordered = 4;
}

message CurateDiff {
  // @state persistent
  optional CurateArtifact artifact = 1;
  // @state persistent
  optional CurateStockDelta stock = 2;
  // @state persistent
  optional CurateCuratedDelta curated = 3;
  // @state local-ui
  optional Filters filters = 4;
  // @state shared-ui
  optional string selected_object_id = 5;
  // @state local-ui
  optional string locale = 6;
  // @state local-ui
  optional string contributions_json = 7;
}
"""
    )
    write_text(ROOT / "🧬️schema/🛰️component.proto", artifact_proto)
    write_text(ROOT / "📸️snapshot/🧬️schema/🛰️component.proto", snapshot_proto)
    write_text(ROOT / "🔺️diff/🧬️schema/🛰️component.proto", diff_proto)
    print("wrote json/ts/graphql/proto leaves (rust leaves are handcrafted separately)")


if __name__ == "__main__":
    main()
