"""Expand 5d artifact schema leaves with normative nested types."""
from __future__ import annotations
import json
from pathlib import Path

puzzle = Path(open("/tmp/puzzle_path.txt").read().strip())
f5 = puzzle / "🗿️artifacts/🖐️5d"

NESTED_TS = r'''
/** ⚓️ Part root plane policy. */
export type Puzzle5dPartAnchor = "fixed" | "derived";

/** 🔗️ Compat row specificity. */
export type Puzzle5dCompatSpecificity = "general" | "part" | "fastener" | "grip" | "rope";

/** 🏷️ Part-kind attribute. */
export interface Puzzle5dAttribute {
  id?: string;
  key?: string;
  value?: string;
  definition?: string;
}

/** ✍️ Part-kind author. */
export interface Puzzle5dAuthor {
  id?: string;
  name?: string;
  email?: string;
  role?: string;
  rank?: number;
}

/** 🖼️ Part-kind representation. */
export interface Puzzle5dRepresentation {
  id?: string;
  name?: string;
  url?: string;
  mime?: string;
  tags?: string[];
  lod?: string;
  description?: string;
}

/** 🌱️ Grip template on a part-kind. */
export interface Puzzle5dGripTemplate {
  id?: string;
  name?: string;
  label?: string;
  description?: string;
  icon?: string;
  gripKind?: string;
  point?: [number, number, number];
  direction?: [number, number, number];
  t?: number;
  mandatory?: boolean;
  radius?: number;
}

/** 🧱️ Part-kind catalog row. */
export interface Puzzle5dCatalogPartKind {
  id: string;
  name?: string;
  label?: string;
  description?: string;
  icon?: string;
  image?: string;
  unit?: string;
  abstract?: boolean;
  baseKinds?: string[];
  representations?: Puzzle5dRepresentation[];
  grips?: Puzzle5dGripTemplate[];
  attributes?: Puzzle5dAttribute[];
  authors?: Puzzle5dAuthor[];
}

/** 🔘️ Grip-kind catalog row. */
export interface Puzzle5dCatalogGripKind {
  id: string;
  code?: string;
  label?: string;
  order?: number;
  compatibleWith?: string[];
  description?: string;
  icon?: string;
  color?: string;
  defaultRopeKind?: string;
}

/** 🔗️ Fastener-kind catalog row. */
export interface Puzzle5dCatalogFastenerKind {
  id: string;
  name?: string;
  label?: string;
}

/** 🧵️ Rope-kind catalog row. */
export interface Puzzle5dCatalogRopeKind {
  id: string;
  name?: string;
  label?: string;
  defaultFastenerKind?: string;
}

/** 🗂️ Kind catalogs bundle. */
export interface Puzzle5dKindCatalogs {
  parts?: Puzzle5dCatalogPartKind[];
  grips?: Puzzle5dCatalogGripKind[];
  fasteners?: Puzzle5dCatalogFastenerKind[];
  ropes?: Puzzle5dCatalogRopeKind[];
}

/** 🔗️ Kind compatibility row. */
export interface Puzzle5dKindCompatibility {
  source: string;
  target: string;
  bidirectional?: boolean;
  important?: boolean;
  specificity?: Puzzle5dCompatSpecificity;
}

/** 📝️ Meta. */
export interface Puzzle5dMeta {
  description?: string;
}

/** 🧱️ Part. */
export interface Puzzle5dPart {
  id: string;
  partKind?: string;
  anchor?: Puzzle5dPartAnchor;
  "2d"?: Record<string, unknown>;
  "3d"?: Record<string, unknown>;
  grips?: Record<string, unknown>[];
}

/** 🔗️ Fastener with eight transform params. */
export interface Puzzle5dFastener {
  id: string;
  source: string;
  target: string;
  fastenerKind?: string;
  gap?: number;
  shift?: number;
  rise?: number;
  rotation?: number;
  turn?: number;
  tilt?: number;
  x?: number;
  y?: number;
}
'''

NESTED_GQL = r'''
enum Puzzle5dPartAnchor { FIXED DERIVED }
enum Puzzle5dCompatSpecificity { GENERAL PART FASTENER GRIP ROPE }

type Puzzle5dAttribute { id: String key: String value: String definition: String }
type Puzzle5dAuthor { id: String name: String email: String role: String rank: Int }
type Puzzle5dRepresentation { id: String name: String url: String mime: String tags: [String!] lod: String description: String }
type Puzzle5dGripTemplate {
  id: String
  name: String
  label: String
  description: String
  icon: String
  gripKind: String
  point: [Float!]
  direction: [Float!]
  t: Float
  mandatory: Boolean
  radius: Float
}
type Puzzle5dCatalogPartKind {
  id: String!
  name: String
  label: String
  description: String
  icon: String
  image: String
  unit: String
  abstract: Boolean
  baseKinds: [String!]
  representations: [Puzzle5dRepresentation!]
  grips: [Puzzle5dGripTemplate!]
  attributes: [Puzzle5dAttribute!]
  authors: [Puzzle5dAuthor!]
}
type Puzzle5dCatalogGripKind {
  id: String!
  code: String
  label: String
  order: Int
  compatibleWith: [String!]
  description: String
  icon: String
  color: String
  defaultRopeKind: String
}
type Puzzle5dCatalogFastenerKind { id: String! name: String label: String }
type Puzzle5dCatalogRopeKind { id: String! name: String label: String defaultFastenerKind: String }
type Puzzle5dKindCatalogs {
  parts: [Puzzle5dCatalogPartKind!]
  grips: [Puzzle5dCatalogGripKind!]
  fasteners: [Puzzle5dCatalogFastenerKind!]
  ropes: [Puzzle5dCatalogRopeKind!]
}
type Puzzle5dKindCompatibility {
  source: String!
  target: String!
  bidirectional: Boolean
  important: Boolean
  specificity: Puzzle5dCompatSpecificity
}
type Puzzle5dMeta { description: String }
type Puzzle5dPart {
  id: String!
  partKind: String
  anchor: Puzzle5dPartAnchor
}
type Puzzle5dFastener {
  id: String!
  source: String!
  target: String!
  fastenerKind: String
  gap: Float
  shift: Float
  rise: Float
  rotation: Float
  turn: Float
  tilt: Float
  x: Float
  y: Float
}
'''

NESTED_PROTO = r'''
enum Puzzle5dPartAnchor { PUZZLE5D_PART_ANCHOR_FIXED = 0; PUZZLE5D_PART_ANCHOR_DERIVED = 1; }
enum Puzzle5dCompatSpecificity {
  PUZZLE5D_COMPAT_SPECIFICITY_GENERAL = 0;
  PUZZLE5D_COMPAT_SPECIFICITY_PART = 1;
  PUZZLE5D_COMPAT_SPECIFICITY_FASTENER = 2;
  PUZZLE5D_COMPAT_SPECIFICITY_GRIP = 3;
  PUZZLE5D_COMPAT_SPECIFICITY_ROPE = 4;
}
message Puzzle5dAttribute { string id = 1; string key = 2; string value = 3; optional string definition = 4; }
message Puzzle5dAuthor { string id = 1; string name = 2; string email = 3; optional string role = 4; optional int32 rank = 5; }
message Puzzle5dRepresentation {
  string id = 1; string name = 2; string url = 3; string mime = 4;
  repeated string tags = 5; optional string lod = 6; string description = 7;
}
message Puzzle5dGripTemplate {
  string id = 1; string name = 2; string label = 3; string description = 4; string icon = 5;
  optional string grip_kind = 6; repeated double point = 7; repeated double direction = 8;
  optional double t = 9; optional bool mandatory = 10; optional double radius = 11;
}
message Puzzle5dCatalogPartKind {
  string id = 1; string name = 2; string label = 3; string description = 4; string icon = 5;
  string image = 6; string unit = 7; bool abstract = 8; repeated string base_kinds = 9;
  repeated Puzzle5dRepresentation representations = 10; repeated Puzzle5dGripTemplate grips = 11;
  repeated Puzzle5dAttribute attributes = 12; repeated Puzzle5dAuthor authors = 13;
}
message Puzzle5dCatalogGripKind {
  string id = 1; optional string code = 2; optional string label = 3; optional int32 order = 4;
  repeated string compatible_with = 5; string description = 6; string icon = 7; string color = 8;
  string default_rope_kind = 9;
}
message Puzzle5dCatalogFastenerKind { string id = 1; string name = 2; optional string label = 3; }
message Puzzle5dCatalogRopeKind { string id = 1; string name = 2; string label = 3; string default_fastener_kind = 4; }
message Puzzle5dKindCatalogs {
  repeated Puzzle5dCatalogPartKind parts = 1;
  repeated Puzzle5dCatalogGripKind grips = 2;
  repeated Puzzle5dCatalogFastenerKind fasteners = 3;
  repeated Puzzle5dCatalogRopeKind ropes = 4;
}
message Puzzle5dKindCompatibility {
  string source = 1; string target = 2; bool bidirectional = 3; bool important = 4;
  Puzzle5dCompatSpecificity specificity = 5;
}
message Puzzle5dMeta { string description = 1; }
message Puzzle5dPart {
  string id = 1; optional string part_kind = 2; Puzzle5dPartAnchor anchor = 3;
}
message Puzzle5dFastener {
  string id = 1; string source = 2; string target = 3; optional string fastener_kind = 4;
  double gap = 5; double shift = 6; double rise = 7; double rotation = 8; double turn = 9; double tilt = 10;
  double x = 11; double y = 12;
}
'''

JSON_DEFS = {
  "Puzzle5dPartAnchor": {"title": "Puzzle5dPartAnchor", "type": "string", "enum": ["fixed", "derived"]},
  "Puzzle5dCompatSpecificity": {"title": "Puzzle5dCompatSpecificity", "type": "string", "enum": ["general", "part", "fastener", "grip", "rope"]},
  "Puzzle5dAttribute": {
    "title": "Puzzle5dAttribute", "type": "object", "additionalProperties": False,
    "properties": {"id": {"type": "string"}, "key": {"type": "string"}, "value": {"type": "string"}, "definition": {"type": "string"}},
  },
  "Puzzle5dAuthor": {
    "title": "Puzzle5dAuthor", "type": "object", "additionalProperties": False,
    "properties": {"id": {"type": "string"}, "name": {"type": "string"}, "email": {"type": "string"}, "role": {"type": "string"}, "rank": {"type": "integer"}},
  },
  "Puzzle5dRepresentation": {
    "title": "Puzzle5dRepresentation", "type": "object", "additionalProperties": False,
    "properties": {
      "id": {"type": "string"}, "name": {"type": "string"}, "url": {"type": "string"}, "mime": {"type": "string"},
      "tags": {"type": "array", "items": {"type": "string"}}, "lod": {"type": "string"}, "description": {"type": "string"},
    },
  },
  "Puzzle5dGripTemplate": {
    "title": "Puzzle5dGripTemplate", "type": "object", "additionalProperties": False,
    "properties": {
      "id": {"type": "string"}, "name": {"type": "string"}, "label": {"type": "string"}, "description": {"type": "string"},
      "icon": {"type": "string"}, "gripKind": {"type": "string"},
      "point": {"type": "array", "items": {"type": "number"}, "minItems": 3, "maxItems": 3},
      "direction": {"type": "array", "items": {"type": "number"}, "minItems": 3, "maxItems": 3},
      "t": {"type": "number"}, "mandatory": {"type": "boolean"}, "radius": {"type": "number"},
    },
  },
  "Puzzle5dCatalogPartKind": {
    "title": "Puzzle5dCatalogPartKind", "type": "object", "additionalProperties": False,
    "required": ["id"],
    "properties": {
      "id": {"type": "string"}, "name": {"type": "string"}, "label": {"type": "string"}, "description": {"type": "string"},
      "icon": {"type": "string"}, "image": {"type": "string"}, "unit": {"type": "string"}, "abstract": {"type": "boolean"},
      "baseKinds": {"type": "array", "items": {"type": "string"}},
      "representations": {"type": "array", "items": {"$ref": "#/$defs/Puzzle5dRepresentation"}},
      "grips": {"type": "array", "items": {"$ref": "#/$defs/Puzzle5dGripTemplate"}},
      "attributes": {"type": "array", "items": {"$ref": "#/$defs/Puzzle5dAttribute"}},
      "authors": {"type": "array", "items": {"$ref": "#/$defs/Puzzle5dAuthor"}},
    },
  },
  "Puzzle5dCatalogGripKind": {
    "title": "Puzzle5dCatalogGripKind", "type": "object", "additionalProperties": False, "required": ["id"],
    "properties": {
      "id": {"type": "string"}, "code": {"type": "string"}, "label": {"type": "string"}, "order": {"type": "integer"},
      "compatibleWith": {"type": "array", "items": {"type": "string"}}, "description": {"type": "string"},
      "icon": {"type": "string"}, "color": {"type": "string"}, "defaultRopeKind": {"type": "string"},
    },
  },
  "Puzzle5dCatalogFastenerKind": {
    "title": "Puzzle5dCatalogFastenerKind", "type": "object", "additionalProperties": False, "required": ["id"],
    "properties": {"id": {"type": "string"}, "name": {"type": "string"}, "label": {"type": "string"}},
  },
  "Puzzle5dCatalogRopeKind": {
    "title": "Puzzle5dCatalogRopeKind", "type": "object", "additionalProperties": False, "required": ["id"],
    "properties": {"id": {"type": "string"}, "name": {"type": "string"}, "label": {"type": "string"}, "defaultFastenerKind": {"type": "string"}},
  },
  "Puzzle5dKindCatalogs": {
    "title": "Puzzle5dKindCatalogs", "type": "object", "additionalProperties": False,
    "properties": {
      "parts": {"type": "array", "items": {"$ref": "#/$defs/Puzzle5dCatalogPartKind"}},
      "grips": {"type": "array", "items": {"$ref": "#/$defs/Puzzle5dCatalogGripKind"}},
      "fasteners": {"type": "array", "items": {"$ref": "#/$defs/Puzzle5dCatalogFastenerKind"}},
      "ropes": {"type": "array", "items": {"$ref": "#/$defs/Puzzle5dCatalogRopeKind"}},
    },
  },
  "Puzzle5dKindCompatibility": {
    "title": "Puzzle5dKindCompatibility", "type": "object", "additionalProperties": False,
    "required": ["source", "target"],
    "properties": {
      "source": {"type": "string"}, "target": {"type": "string"}, "bidirectional": {"type": "boolean"},
      "important": {"type": "boolean"}, "specificity": {"$ref": "#/$defs/Puzzle5dCompatSpecificity"},
    },
  },
  "Puzzle5dMeta": {
    "title": "Puzzle5dMeta", "type": "object", "additionalProperties": False,
    "properties": {"description": {"type": "string"}},
  },
  "Puzzle5dPart": {
    "title": "Puzzle5dPart", "type": "object", "additionalProperties": True, "required": ["id"],
    "properties": {
      "id": {"type": "string"}, "partKind": {"type": "string"},
      "anchor": {"$ref": "#/$defs/Puzzle5dPartAnchor"},
      "2d": {"type": "object"}, "3d": {"type": "object"},
      "grips": {"type": "array", "items": {"type": "object"}},
    },
  },
  "Puzzle5dFastener": {
    "title": "Puzzle5dFastener", "type": "object", "additionalProperties": False, "required": ["id", "source", "target"],
    "properties": {
      "id": {"type": "string"}, "source": {"type": "string"}, "target": {"type": "string"}, "fastenerKind": {"type": "string"},
      "gap": {"type": "number"}, "shift": {"type": "number"}, "rise": {"type": "number"},
      "rotation": {"type": "number"}, "turn": {"type": "number"}, "tilt": {"type": "number"},
      "x": {"type": "number"}, "y": {"type": "number"},
    },
  },
}

def patch_ts(path: Path, artifact_only: bool = False):
    text = path.read_text()
    if "Puzzle5dFastener" in text and "gap?:" in text:
        print("ts already patched", path)
        return
    # Append nested types after artifact interface (or replace loose refs)
    if "export interface Puzzle5dArtifact" in text or "export interface Puzzle5dSnapshot" in text or "export interface Puzzle5dDiff" in text:
        # For diff schema, nested types already have loose interfaces — replace those
        if "export interface Puzzle5dFastener { id: string; [key: string]: unknown; }" in text:
            text = text.replace(
                "export interface Puzzle5dPart { id: string; [key: string]: unknown; }",
                "export interface Puzzle5dPart { id: string; partKind?: string; anchor?: Puzzle5dPartAnchor; [key: string]: unknown; }",
            )
            text = text.replace(
                "export interface Puzzle5dFastener { id: string; [key: string]: unknown; }",
                "export interface Puzzle5dFastener { id: string; source?: string; target?: string; gap?: number; shift?: number; rise?: number; rotation?: number; turn?: number; tilt?: number; x?: number; y?: number; [key: string]: unknown; }",
            )
            text = text.replace(
                "export interface Puzzle5dKindCompatibility { id: string; [key: string]: unknown; }",
                "export interface Puzzle5dKindCompatibility { source?: string; target?: string; bidirectional?: boolean; important?: boolean; specificity?: Puzzle5dCompatSpecificity; [key: string]: unknown; }",
            )
            # prepend type aliases if missing
            if "export type Puzzle5dPartAnchor" not in text:
                text = "/** 🧬️ Puzzle5d nested schema types (design-parity). */\n" + NESTED_TS.split("/** 🏷️")[0] + text
        else:
            if not text.endswith("\n"):
                text += "\n"
            text = text + "\n" + NESTED_TS
        path.write_text(text)
        print("patched ts", path)
    else:
        print("skip ts", path)

def patch_gql(path: Path):
    text = path.read_text()
    if "type Puzzle5dFastener" in text and "x: Float" in text:
        print("gql already", path)
        return
    # For diff, replace stub types
    if "type Puzzle5dFastener { id: String! }" in text:
        text = text.replace("type Puzzle5dPart { id: String! }", "type Puzzle5dPart { id: String! partKind: String anchor: Puzzle5dPartAnchor }")
        text = text.replace(
            "type Puzzle5dFastener { id: String! }",
            "type Puzzle5dFastener { id: String! source: String target: String gap: Float shift: Float rise: Float rotation: Float turn: Float tilt: Float x: Float y: Float }",
        )
        text = text.replace(
            "type Puzzle5dKindCompatibility { id: String! }",
            "type Puzzle5dKindCompatibility { source: String target: String bidirectional: Boolean important: Boolean specificity: Puzzle5dCompatSpecificity }",
        )
        if "enum Puzzle5dPartAnchor" not in text:
            text += "\n" + "\n".join(NESTED_GQL.strip().splitlines()[:2]) + "\n"
        path.write_text(text)
        print("patched gql diff-ish", path)
        return
    if not text.endswith("\n"):
        text += "\n"
    text += "\n" + NESTED_GQL
    path.write_text(text)
    print("patched gql", path)

def patch_proto(path: Path):
    text = path.read_text()
    if "message Puzzle5dFastener" in text and "double x = 11" in text:
        print("proto already", path)
        return
    if "message Puzzle5dFastener { string id = 1; }" in text:
        text = text.replace(
            "message Puzzle5dPart { string id = 1; }",
            "message Puzzle5dPart { string id = 1; optional string part_kind = 2; Puzzle5dPartAnchor anchor = 3; }",
        )
        text = text.replace(
            "message Puzzle5dFastener { string id = 1; }",
            "message Puzzle5dFastener { string id = 1; string source = 2; string target = 3; double gap = 4; double shift = 5; double rise = 6; double rotation = 7; double turn = 8; double tilt = 9; double x = 10; double y = 11; }",
        )
        text = text.replace(
            "message Puzzle5dKindCompatibility { string id = 1; }",
            "message Puzzle5dKindCompatibility { string source = 1; string target = 2; bool bidirectional = 3; bool important = 4; Puzzle5dCompatSpecificity specificity = 5; }",
        )
        if "enum Puzzle5dPartAnchor" not in text:
            # insert enums before messages that need them — append at end
            text += "\n" + "\n".join(NESTED_PROTO.strip().splitlines()[:8]) + "\n"
        path.write_text(text)
        print("patched proto diff-ish", path)
        return
    if not text.endswith("\n"):
        text += "\n"
    text += "\n" + NESTED_PROTO
    path.write_text(text)
    print("patched proto", path)

def patch_json(path: Path):
    data = json.loads(path.read_text())
    defs = data.setdefault("$defs", {})
    # Keep delta wrappers; upgrade domain defs
    for k, v in JSON_DEFS.items():
        defs[k] = v
    # Also alias old names if referenced
    path.write_text(json.dumps(data, indent=2) + "\n")
    print("patched json", path)

for rel in [
    "🧬️schema",
    "📸️snapshot/🧬️schema",
    "🔺️diff/🧬️schema",
]:
    base = f5 / rel
    patch_ts(base / "🟦️component.ts")
    patch_gql(base / "🔗️component.graphql")
    patch_proto(base / "🛰️component.proto")
    patch_json(base / "🔣️component.json")
    print("---", rel)

print("schema leaves done")
