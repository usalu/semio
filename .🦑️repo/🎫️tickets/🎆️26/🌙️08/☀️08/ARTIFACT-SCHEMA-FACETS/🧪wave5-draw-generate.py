#!/usr/bin/env python3
"""Generate draw artifact schema facet leaves + structural moves (wave5)."""
from __future__ import annotations

import json
import shutil
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw")
PLUGIN = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw")

HELPERS_JSON = {
    "DrawLayerNode": {
        "title": "DrawLayerNode",
        "type": "object",
        "additionalProperties": True,
        "required": ["kind"],
        "properties": {"kind": {"type": "string"}},
    },
    "DrawImageAsset": {
        "title": "DrawImageAsset",
        "type": "object",
        "additionalProperties": False,
        "required": ["mime", "data"],
        "properties": {
            "mime": {"type": "string"},
            "data": {"type": "string"},
            "width": {"type": "integer", "format": "uint32", "minimum": 0},
            "height": {"type": "integer", "format": "uint32", "minimum": 0},
        },
    },
    "DrawArtboard": {
        "title": "DrawArtboard",
        "type": "object",
        "additionalProperties": False,
        "required": ["width", "height"],
        "properties": {
            "width": {"type": "number", "format": "double"},
            "height": {"type": "number", "format": "double"},
        },
    },
}

ARTIFACT_PROPS = {
    "schema": {"type": "string", "x-semio-state": "persistent"},
    "id": {"type": "string", "x-semio-state": "persistent"},
    "title": {"type": "string", "x-semio-state": "persistent"},
    "layers": {
        "type": "array",
        "items": {"$ref": "#/$defs/DrawLayerNode"},
        "x-semio-state": "persistent",
    },
    "assets": {
        "type": "object",
        "additionalProperties": {"$ref": "#/$defs/DrawImageAsset"},
        "x-semio-state": "persistent",
    },
    "artboard": {"$ref": "#/$defs/DrawArtboard", "x-semio-state": "persistent"},
    "selectedIds": {
        "type": "array",
        "items": {"type": "string"},
        "x-semio-state": "shared-ui",
    },
    "activeUtilityId": {"type": "string", "x-semio-state": "shared-ui"},
    "engagementInput": {"type": "string", "x-semio-state": "local-ui"},
    "cameraX": {"type": "number", "format": "double", "x-semio-state": "local-ui"},
    "cameraY": {"type": "number", "format": "double", "x-semio-state": "local-ui"},
    "cameraZoom": {"type": "number", "format": "double", "x-semio-state": "local-ui"},
    "locale": {"type": "string", "x-semio-state": "local-ui"},
    "hoveredId": {"type": "string", "x-semio-state": "preview"},
}

ARTIFACT_REQUIRED = [
    "schema",
    "id",
    "layers",
    "assets",
    "selectedIds",
    "activeUtilityId",
    "engagementInput",
    "cameraX",
    "cameraY",
    "cameraZoom",
    "locale",
]

SNAPSHOT_PROPS = {
    k: v for k, v in ARTIFACT_PROPS.items() if v.get("x-semio-state") == "persistent"
}
SNAPSHOT_REQUIRED = [k for k in ("schema", "id", "layers", "assets") if k in SNAPSHOT_PROPS]


def write(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if not text.endswith("\n"):
        text += "\n"
    path.write_text(text)
    print("wrote", path)


def main() -> None:
    # Move pack under snapshot if still at root
    pack_src = ROOT / "🎒️pack"
    pack_dst = ROOT / "📸️snapshot" / "🎒️pack"
    if pack_src.is_dir() and not pack_dst.exists():
        pack_dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.move(str(pack_src), str(pack_dst))
        print("moved pack -> snapshot/pack")
    elif pack_src.is_dir() and pack_dst.exists():
        shutil.rmtree(pack_src)
        print("removed leftover root pack")

    # Rename set-document -> set-snapshot
    old_mut = ROOT / "🧬️mutations" / "📄set-document"
    new_mut = ROOT / "🧬️mutations" / "🖼️set-snapshot"
    if old_mut.is_dir() and not new_mut.exists():
        shutil.move(str(old_mut), str(new_mut))
        print("moved set-document -> set-snapshot")

    # JSON leaves
    write(
        ROOT / "🧬️schema" / "🔣️component.json",
        json.dumps(
            {
                "$id": "https://semio.tech/schema/s/draw/draw/artifact.json",
                "title": "DrawArtifact",
                "type": "object",
                "additionalProperties": False,
                "required": ARTIFACT_REQUIRED,
                "properties": ARTIFACT_PROPS,
                "$defs": HELPERS_JSON,
            },
            indent=2,
        ),
    )
    write(
        ROOT / "📸️snapshot" / "🧬️schema" / "🔣️component.json",
        json.dumps(
            {
                "$id": "https://semio.tech/schema/s/draw/draw/snapshot.json",
                "title": "DrawSnapshot",
                "type": "object",
                "additionalProperties": False,
                "required": SNAPSHOT_REQUIRED,
                "properties": SNAPSHOT_PROPS,
                "$defs": HELPERS_JSON,
            },
            indent=2,
        ),
    )
    diff_defs = {
        **HELPERS_JSON,
        "DrawStringList": {
            "title": "DrawStringList",
            "type": "object",
            "additionalProperties": False,
            "required": ["values"],
            "properties": {
                "values": {"type": "array", "items": {"type": "string"}}
            },
        },
        "DrawLayersDelta": {
            "title": "DrawLayersDelta",
            "type": "object",
            "additionalProperties": False,
            "required": ["added", "removed", "patched"],
            "properties": {
                "added": {
                    "type": "array",
                    "items": {"$ref": "#/$defs/DrawLayerNode"},
                },
                "removed": {"type": "array", "items": {"type": "string"}},
                "patched": {
                    "type": "array",
                    "items": {"$ref": "#/$defs/DrawLayerPatchEntry"},
                },
                "reordered": {"type": "array", "items": {"type": "string"}},
            },
        },
        "DrawLayerPatchEntry": {
            "title": "DrawLayerPatchEntry",
            "type": "object",
            "additionalProperties": False,
            "required": ["id", "patch"],
            "properties": {
                "id": {"type": "string"},
                "patch": {"$ref": "#/$defs/DrawLayerPatch"},
            },
        },
        "DrawLayerPatch": {
            "title": "DrawLayerPatch",
            "type": "object",
            "additionalProperties": False,
            "required": [],
            "properties": {
                "visible": {"type": "boolean"},
                "locked": {"type": "boolean"},
                "name": {"type": "string"},
                "opacity": {"type": "number", "format": "double"},
                "blendMode": {"type": "string"},
                "transformJson": {
                    "type": "string",
                    "contentMediaType": "application/json",
                },
                "fillJson": {
                    "type": "string",
                    "contentMediaType": "application/json",
                },
                "strokeJson": {
                    "type": "string",
                    "contentMediaType": "application/json",
                },
                "booleanOperation": {"type": "string"},
                "traceParamsJson": {
                    "type": "string",
                    "contentMediaType": "application/json",
                },
                "layerJson": {
                    "type": "string",
                    "contentMediaType": "application/json",
                },
            },
        },
    }
    diff_props = {
        "artifact": {
            "title": "DrawArtifact",
            "type": "object",
            "x-semio-state": "persistent",
        },
        "schema": {"type": "string", "x-semio-state": "persistent"},
        "id": {"type": "string", "x-semio-state": "persistent"},
        "title": {
            "oneOf": [{"type": "null"}, {"type": "string"}],
            "x-semio-state": "persistent",
        },
        "layers": {
            "$ref": "#/$defs/DrawLayersDelta",
            "x-semio-state": "persistent",
        },
        "assets": {
            "type": "object",
            "additionalProperties": {
                "oneOf": [{"type": "null"}, {"$ref": "#/$defs/DrawImageAsset"}]
            },
            "x-semio-state": "persistent",
        },
        "artboard": {
            "oneOf": [{"type": "null"}, {"$ref": "#/$defs/DrawArtboard"}],
            "x-semio-state": "persistent",
        },
        "selectedIds": {
            "$ref": "#/$defs/DrawStringList",
            "x-semio-state": "shared-ui",
        },
        "activeUtilityId": {"type": "string", "x-semio-state": "shared-ui"},
        "engagementInput": {"type": "string", "x-semio-state": "local-ui"},
        "cameraX": {
            "type": "number",
            "format": "double",
            "x-semio-state": "local-ui",
        },
        "cameraY": {
            "type": "number",
            "format": "double",
            "x-semio-state": "local-ui",
        },
        "cameraZoom": {
            "type": "number",
            "format": "double",
            "x-semio-state": "local-ui",
        },
        "locale": {"type": "string", "x-semio-state": "local-ui"},
        "hoveredId": {
            "oneOf": [{"type": "null"}, {"type": "string"}],
            "x-semio-state": "preview",
        },
    }
    write(
        ROOT / "🔺️diff" / "🧬️schema" / "🔣️component.json",
        json.dumps(
            {
                "$id": "https://semio.tech/schema/s/draw/draw/diff.json",
                "title": "DrawDiff",
                "type": "object",
                "additionalProperties": False,
                "required": [],
                "properties": diff_props,
                "$defs": diff_defs,
            },
            indent=2,
        ),
    )

    # TS
    write(
        ROOT / "🧬️schema" / "🟦️component.ts",
        """/** 🧬️ Draw artifact schema — every field with its state class. */

export interface DrawArtifact {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  id: string;
  /** @state persistent */
  title?: string;
  /** @state persistent */
  layers: DrawLayerNode[];
  /** @state persistent */
  assets: Record<string, DrawImageAsset>;
  /** @state persistent */
  artboard?: DrawArtboard;
  /** @state shared-ui */
  selectedIds: string[];
  /** @state shared-ui */
  activeUtilityId: string;
  /** @state local-ui */
  engagementInput: string;
  /** @state local-ui */
  cameraX: number;
  /** @state local-ui */
  cameraY: number;
  /** @state local-ui */
  cameraZoom: number;
  /** @state local-ui */
  locale: string;
  /** @state preview */
  hoveredId?: string;
}

export interface DrawLayerNode {
  kind: string;
  [key: string]: unknown;
}

export interface DrawImageAsset {
  mime: string;
  data: string;
  width?: number;
  height?: number;
}

export interface DrawArtboard {
  width: number;
  height: number;
}
""",
    )
    write(
        ROOT / "📸️snapshot" / "🧬️schema" / "🟦️component.ts",
        """/** 🧬️ Draw snapshot schema — persistent fields only. */

export interface DrawSnapshot {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  id: string;
  /** @state persistent */
  title?: string;
  /** @state persistent */
  layers: DrawLayerNode[];
  /** @state persistent */
  assets: Record<string, DrawImageAsset>;
  /** @state persistent */
  artboard?: DrawArtboard;
}

export interface DrawLayerNode {
  kind: string;
  [key: string]: unknown;
}

export interface DrawImageAsset {
  mime: string;
  data: string;
  width?: number;
  height?: number;
}

export interface DrawArtboard {
  width: number;
  height: number;
}
""",
    )
    write(
        ROOT / "🔺️diff" / "🧬️schema" / "🟦️component.ts",
        """/** 🧬️ Draw diff schema — sparse field delta. */

export interface DrawDiff {
  /** @state persistent */
  artifact?: DrawArtifact;
  /** @state persistent */
  schema?: string;
  /** @state persistent */
  id?: string;
  /** @state persistent */
  title?: string | null;
  /** @state persistent */
  layers?: DrawLayersDelta;
  /** @state persistent */
  assets?: Record<string, DrawImageAsset | null>;
  /** @state persistent */
  artboard?: DrawArtboard | null;
  /** @state shared-ui */
  selectedIds?: DrawStringList;
  /** @state shared-ui */
  activeUtilityId?: string;
  /** @state local-ui */
  engagementInput?: string;
  /** @state local-ui */
  cameraX?: number;
  /** @state local-ui */
  cameraY?: number;
  /** @state local-ui */
  cameraZoom?: number;
  /** @state local-ui */
  locale?: string;
  /** @state preview */
  hoveredId?: string | null;
}

export interface DrawArtifact {
  schema: string;
  id: string;
  title?: string;
  layers: DrawLayerNode[];
  assets: Record<string, DrawImageAsset>;
  artboard?: DrawArtboard;
  selectedIds: string[];
  activeUtilityId: string;
  engagementInput: string;
  cameraX: number;
  cameraY: number;
  cameraZoom: number;
  locale: string;
  hoveredId?: string;
}

export interface DrawStringList {
  values: string[];
}

export interface DrawLayersDelta {
  added: DrawLayerNode[];
  removed: string[];
  patched: DrawLayerPatchEntry[];
  reordered?: string[];
}

export interface DrawLayerPatchEntry {
  id: string;
  patch: DrawLayerPatch;
}

export interface DrawLayerPatch {
  visible?: boolean;
  locked?: boolean;
  name?: string;
  opacity?: number;
  blendMode?: string;
  transformJson?: string;
  fillJson?: string;
  strokeJson?: string;
  booleanOperation?: string;
  traceParamsJson?: string;
  layerJson?: string;
}

export interface DrawLayerNode {
  kind: string;
  [key: string]: unknown;
}

export interface DrawImageAsset {
  mime: string;
  data: string;
  width?: number;
  height?: number;
}

export interface DrawArtboard {
  width: number;
  height: number;
}
""",
    )

    # GraphQL
    write(
        ROOT / "🧬️schema" / "🔗️component.graphql",
        """# 🧬️ Draw artifact schema — every field with its state class.

type DrawArtifact {
  schema: String! @state(class: PERSISTENT)
  id: String! @state(class: PERSISTENT)
  title: String @state(class: PERSISTENT)
  layers: [DrawLayerNode!]! @state(class: PERSISTENT)
  assets: [DrawImageAssetEntry!]! @state(class: PERSISTENT)
  artboard: DrawArtboard @state(class: PERSISTENT)
  selectedIds: [String!]! @state(class: SHARED_UI)
  activeUtilityId: String! @state(class: SHARED_UI)
  engagementInput: String! @state(class: LOCAL_UI)
  cameraX: Float! @state(class: LOCAL_UI)
  cameraY: Float! @state(class: LOCAL_UI)
  cameraZoom: Float! @state(class: LOCAL_UI)
  locale: String! @state(class: LOCAL_UI)
  hoveredId: String @state(class: PREVIEW)
}

type DrawLayerNode {
  kind: String!
}

type DrawImageAssetEntry {
  key: String!
  value: DrawImageAsset!
}

type DrawImageAsset {
  mime: String!
  data: String!
  width: Int
  height: Int
}

type DrawArtboard {
  width: Float!
  height: Float!
}
""",
    )
    write(
        ROOT / "📸️snapshot" / "🧬️schema" / "🔗️component.graphql",
        """# 🧬️ Draw snapshot schema — persistent fields only.

type DrawSnapshot {
  schema: String! @state(class: PERSISTENT)
  id: String! @state(class: PERSISTENT)
  title: String @state(class: PERSISTENT)
  layers: [DrawLayerNode!]! @state(class: PERSISTENT)
  assets: [DrawImageAssetEntry!]! @state(class: PERSISTENT)
  artboard: DrawArtboard @state(class: PERSISTENT)
}

type DrawLayerNode {
  kind: String!
}

type DrawImageAssetEntry {
  key: String!
  value: DrawImageAsset!
}

type DrawImageAsset {
  mime: String!
  data: String!
  width: Int
  height: Int
}

type DrawArtboard {
  width: Float!
  height: Float!
}
""",
    )
    write(
        ROOT / "🔺️diff" / "🧬️schema" / "🔗️component.graphql",
        """# 🧬️ Draw diff schema — sparse field delta.

type DrawDiff {
  artifact: DrawArtifact @state(class: PERSISTENT)
  schema: String @state(class: PERSISTENT)
  id: String @state(class: PERSISTENT)
  title: String @state(class: PERSISTENT)
  layers: DrawLayersDelta @state(class: PERSISTENT)
  assets: [DrawImageAssetDiffEntry!] @state(class: PERSISTENT)
  artboard: DrawArtboard @state(class: PERSISTENT)
  selectedIds: DrawStringList @state(class: SHARED_UI)
  activeUtilityId: String @state(class: SHARED_UI)
  engagementInput: String @state(class: LOCAL_UI)
  cameraX: Float @state(class: LOCAL_UI)
  cameraY: Float @state(class: LOCAL_UI)
  cameraZoom: Float @state(class: LOCAL_UI)
  locale: String @state(class: LOCAL_UI)
  hoveredId: String @state(class: PREVIEW)
}

type DrawArtifact {
  schema: String!
  id: String!
  title: String
  layers: [DrawLayerNode!]!
  assets: [DrawImageAssetEntry!]!
  artboard: DrawArtboard
  selectedIds: [String!]!
  activeUtilityId: String!
  engagementInput: String!
  cameraX: Float!
  cameraY: Float!
  cameraZoom: Float!
  locale: String!
  hoveredId: String
}

type DrawStringList {
  values: [String!]!
}

type DrawLayersDelta {
  added: [DrawLayerNode!]!
  removed: [String!]!
  patched: [DrawLayerPatchEntry!]!
  reordered: [String!]
}

type DrawLayerPatchEntry {
  id: String!
  patch: DrawLayerPatch!
}

type DrawLayerPatch {
  visible: Boolean
  locked: Boolean
  name: String
  opacity: Float
  blendMode: String
  transformJson: String
  fillJson: String
  strokeJson: String
  booleanOperation: String
  traceParamsJson: String
  layerJson: String
}

type DrawLayerNode {
  kind: String!
}

type DrawImageAssetEntry {
  key: String!
  value: DrawImageAsset!
}

type DrawImageAssetDiffEntry {
  key: String!
  value: DrawImageAsset
}

type DrawImageAsset {
  mime: String!
  data: String!
  width: Int
  height: Int
}

type DrawArtboard {
  width: Float!
  height: Float!
}
""",
    )

    # Proto
    write(
        ROOT / "🧬️schema" / "🛰️component.proto",
        """syntax = "proto3";
package semio.s.draw.draw.artifact;

// 🧬️ Draw artifact schema — every field with its state class.

message DrawArtifact {
  // @state persistent
  string schema = 1;
  // @state persistent
  string id = 2;
  // @state persistent
  optional string title = 3;
  // @state persistent
  repeated DrawLayerNode layers = 4;
  // @state persistent
  map<string, DrawImageAsset> assets = 5;
  // @state persistent
  optional DrawArtboard artboard = 6;
  // @state shared-ui
  repeated string selected_ids = 7;
  // @state shared-ui
  string active_utility_id = 8;
  // @state local-ui
  string engagement_input = 9;
  // @state local-ui
  double camera_x = 10;
  // @state local-ui
  double camera_y = 11;
  // @state local-ui
  double camera_zoom = 12;
  // @state local-ui
  string locale = 13;
  // @state preview
  optional string hovered_id = 14;
}

message DrawLayerNode {
  string kind = 1;
}

message DrawImageAsset {
  string mime = 1;
  string data = 2;
  optional uint32 width = 3;
  optional uint32 height = 4;
}

message DrawArtboard {
  double width = 1;
  double height = 2;
}
""",
    )
    write(
        ROOT / "📸️snapshot" / "🧬️schema" / "🛰️component.proto",
        """syntax = "proto3";
package semio.s.draw.draw.snapshot;

// 🧬️ Draw snapshot schema — persistent fields only.

message DrawSnapshot {
  // @state persistent
  string schema = 1;
  // @state persistent
  string id = 2;
  // @state persistent
  optional string title = 3;
  // @state persistent
  repeated DrawLayerNode layers = 4;
  // @state persistent
  map<string, DrawImageAsset> assets = 5;
  // @state persistent
  optional DrawArtboard artboard = 6;
}

message DrawLayerNode {
  string kind = 1;
}

message DrawImageAsset {
  string mime = 1;
  string data = 2;
  optional uint32 width = 3;
  optional uint32 height = 4;
}

message DrawArtboard {
  double width = 1;
  double height = 2;
}
""",
    )
    write(
        ROOT / "🔺️diff" / "🧬️schema" / "🛰️component.proto",
        """syntax = "proto3";
package semio.s.draw.draw.diff;

// 🧬️ Draw diff schema — sparse field delta.

message DrawDiff {
  // @state persistent
  optional DrawArtifact artifact = 1;
  // @state persistent
  optional string schema = 2;
  // @state persistent
  optional string id = 3;
  // @state persistent
  optional string title = 4;
  // @state persistent
  optional DrawLayersDelta layers = 5;
  // @state persistent
  map<string, DrawImageAsset> assets = 6;
  // @state persistent
  optional DrawArtboard artboard = 7;
  // @state shared-ui
  optional DrawStringList selected_ids = 8;
  // @state shared-ui
  optional string active_utility_id = 9;
  // @state local-ui
  optional string engagement_input = 10;
  // @state local-ui
  optional double camera_x = 11;
  // @state local-ui
  optional double camera_y = 12;
  // @state local-ui
  optional double camera_zoom = 13;
  // @state local-ui
  optional string locale = 14;
  // @state preview
  optional string hovered_id = 15;
}

message DrawArtifact {
  string schema = 1;
  string id = 2;
  optional string title = 3;
  repeated DrawLayerNode layers = 4;
  map<string, DrawImageAsset> assets = 5;
  optional DrawArtboard artboard = 6;
  repeated string selected_ids = 7;
  string active_utility_id = 8;
  string engagement_input = 9;
  double camera_x = 10;
  double camera_y = 11;
  double camera_zoom = 12;
  string locale = 13;
  optional string hovered_id = 14;
}

message DrawStringList {
  repeated string values = 1;
}

message DrawLayersDelta {
  repeated DrawLayerNode added = 1;
  repeated string removed = 2;
  repeated DrawLayerPatchEntry patched = 3;
  repeated string reordered = 4;
}

message DrawLayerPatchEntry {
  string id = 1;
  DrawLayerPatch patch = 2;
}

message DrawLayerPatch {
  optional bool visible = 1;
  optional bool locked = 2;
  optional string name = 3;
  optional double opacity = 4;
  optional string blend_mode = 5;
  optional string transform_json = 6;
  optional string fill_json = 7;
  optional string stroke_json = 8;
  optional string boolean_operation = 9;
  optional string trace_params_json = 10;
  optional string layer_json = 11;
}

message DrawLayerNode {
  string kind = 1;
}

message DrawImageAsset {
  string mime = 1;
  string data = 2;
  optional uint32 width = 3;
  optional uint32 height = 4;
}

message DrawArtboard {
  double width = 1;
  double height = 2;
}
""",
    )

    # Rust artifact schema
    write(
        ROOT / "🧬️schema" / "🦀️component.rs",
        r'''//! 🧬️ Draw artifact schema — every field of the artifact with its state class.

use crate::artifacts::draw::{DrawArtboard, DrawImageAsset, DrawLayerNode, DRAW_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Artifact
/// 🧬️ Full draw artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.draw.draw")]
pub struct DrawArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub id: String,
    #[state(persistent)] pub title: Option<String>,
    #[state(persistent)] pub layers: Vec<DrawLayerNode>,
    #[state(persistent)] pub assets: BTreeMap<String, DrawImageAsset>,
    #[state(persistent)] pub artboard: Option<DrawArtboard>,
    #[state(shared_ui)] pub selected_ids: Vec<String>,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub engagement_input: String,
    #[state(local_ui)] pub camera_x: f64,
    #[state(local_ui)] pub camera_y: f64,
    #[state(local_ui)] pub camera_zoom: f64,
    #[state(local_ui)] pub locale: String,
    #[state(preview)] pub hovered_id: Option<String>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for DrawArtifact {
    fn default() -> Self {
        Self {
            schema: DRAW_DOCUMENT_SCHEMA.into(),
            id: String::new(),
            title: None,
            layers: Vec::new(),
            assets: BTreeMap::new(),
            artboard: Some(DrawArtboard { width: 1024.0, height: 1024.0 }),
            selected_ids: Vec::new(),
            active_utility_id: "selectDirect".into(),
            engagement_input: String::new(),
            camera_x: 512.0,
            camera_y: 512.0,
            camera_zoom: 0.75,
            locale: "en-US".into(),
            hovered_id: None,
        }
    }
}

impl DrawArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::draw::DrawSnapshot {
        crate::artifacts::draw::DrawSnapshot {
            schema: self.schema.clone(),
            id: self.id.clone(),
            title: self.title.clone(),
            layers: self.layers.clone(),
            assets: self.assets.clone(),
            artboard: self.artboard.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::draw::DrawSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            id: snapshot.id,
            title: snapshot.title,
            layers: snapshot.layers,
            assets: snapshot.assets,
            artboard: snapshot.artboard,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::draw::DrawSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.title = snapshot.title;
        self.layers = snapshot.layers;
        self.assets = snapshot.assets;
        self.artboard = snapshot.artboard;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.draw.draw` — fifteen handcrafted schema leaves.
pub fn draw_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.draw.draw",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("../📸️snapshot/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../📸️snapshot/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../📸️snapshot/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../📸️snapshot/🧬️schema/🔣️component.json"),
            proto: include_str!("../📸️snapshot/🧬️schema/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("../🔺️diff/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../🔺️diff/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../🔺️diff/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../🔺️diff/🧬️schema/🔣️component.json"),
            proto: include_str!("../🔺️diff/🧬️schema/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
''',
    )

    # Rust snapshot schema — DrawSnapshot with Document codecs (replaces DrawDocument)
    write(
        ROOT / "📸️snapshot" / "🧬️schema" / "🦀️component.rs",
        r'''//! 🧬️ Draw snapshot schema — persistent fields only.

use crate::artifacts::draw::{DrawArtboard, DrawImageAsset, DrawLayerNode, DRAW_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Snapshot
/// 📸️ Persisted draw document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "draw.draw", layout = "lines")]
#[artifact_schema(id = "s.draw.draw")]
pub struct DrawSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub id: String,
    #[state(persistent)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[state(persistent)]
    #[dsl(statements, block)]
    pub layers: Vec<DrawLayerNode>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub assets: BTreeMap<String, DrawImageAsset>,
    #[state(persistent)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub artboard: Option<DrawArtboard>,
}
//#region 🔖️HandcraftedDocumentCodecs
/// ✉️ P6 handcrafted DocumentDsl/DocumentPack (derive no longer emits these traits).
impl store::DocumentDsl for DrawSnapshot {
    const EXTENSION: &'static str = "draw";
    fn envelope_id() -> &'static str { "draw.draw" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for DrawSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}
//#endregion 🔖️HandcraftedDocumentCodecs

impl Default for DrawSnapshot {
    fn default() -> Self {
        Self {
            schema: DRAW_DOCUMENT_SCHEMA.into(),
            id: String::new(),
            title: None,
            layers: Vec::new(),
            assets: BTreeMap::new(),
            artboard: Some(DrawArtboard { width: 1024.0, height: 1024.0 }),
        }
    }
}
//#endregion 🔖️Snapshot
''',
    )

    # Rust diff schema
    write(
        ROOT / "🔺️diff" / "🧬️schema" / "🦀️component.rs",
        r'''//! 🧬️ Draw diff schema — sparse field delta over the artifact.

use crate::artifacts::draw::{DrawArtboard, DrawImageAsset, DrawLayerNode};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the draw artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.draw.draw")]
pub struct DrawDiff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::draw::schema::DrawArtifact>>,
    #[state(persistent)] pub schema: Option<String>,
    #[state(persistent)] pub id: Option<String>,
    #[state(persistent)] pub title: Option<Option<String>>,
    #[state(persistent)] pub layers: Option<DrawLayersDelta>,
    #[state(persistent)] pub assets: Option<BTreeMap<String, Option<DrawImageAsset>>>,
    #[state(persistent)] pub artboard: Option<Option<DrawArtboard>>,
    #[state(shared_ui)] pub selected_ids: Option<DrawStringList>,
    #[state(shared_ui)] pub active_utility_id: Option<String>,
    #[state(local_ui)] pub engagement_input: Option<String>,
    #[state(local_ui)] pub camera_x: Option<f64>,
    #[state(local_ui)] pub camera_y: Option<f64>,
    #[state(local_ui)] pub camera_zoom: Option<f64>,
    #[state(local_ui)] pub locale: Option<String>,
    #[state(preview)] pub hovered_id: Option<Option<String>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DrawStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `layers`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DrawLayersDelta {
    pub added: Vec<DrawLayerNode>,
    pub removed: Vec<String>,
    pub patched: Vec<DrawLayerPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched layer entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawLayerPatchEntry {
    pub id: String,
    pub patch: DrawLayerPatch,
}

/// 🩹 Sparse layer field patch (JSON blobs for complex nested values).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DrawLayerPatch {
    pub visible: Option<bool>,
    pub locked: Option<bool>,
    pub name: Option<String>,
    pub opacity: Option<f64>,
    pub blend_mode: Option<String>,
    pub transform_json: Option<String>,
    pub fill_json: Option<String>,
    pub stroke_json: Option<String>,
    pub boolean_operation: Option<String>,
    pub trace_params_json: Option<String>,
    pub layer_json: Option<String>,
}
//#endregion 🔖️DeltaHelpers
''',
    )

    # Pack protocol: rename Projection segment if any; draw pack has no Projection segment currently.
    # Add Snapshot segment name comment consistency — update framing comment only when needed.
    protocol = ROOT / "📸️snapshot" / "🎒️pack" / "📡️component.protocol.semio"
    if protocol.exists():
        text = protocol.read_text()
        if "segment Projection" in text:
            text = text.replace("segment Projection", "segment Snapshot")
            protocol.write_text(text)
            print("renamed Projection->Snapshot in pack protocol")

    print("done generate")


if __name__ == "__main__":
    main()
