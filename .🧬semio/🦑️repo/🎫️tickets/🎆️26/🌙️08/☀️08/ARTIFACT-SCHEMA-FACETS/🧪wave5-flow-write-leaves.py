#!/usr/bin/env python3
"""Write flow artifact schema leaves + core runtime scaffolding."""
from __future__ import annotations

import json
from pathlib import Path

ROOT = next(
    Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins").glob("🌊️flow")
) / "🗿️artifacts" / "🌊️flow"

ART = ROOT / "🧬️schema"
SNAP = ROOT / "📸️snapshot" / "🧬️schema"
DIFF = ROOT / "🔺️diff" / "🧬️schema"
for d in (ART, SNAP, DIFF):
    d.mkdir(parents=True, exist_ok=True)

# ---------------------------------------------------------------------------
# Shared $defs for snapshot/artifact JSON
# ---------------------------------------------------------------------------
CAMERA = {
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
WIDGET_LAYOUT = {
    "title": "WidgetLayout",
    "type": "object",
    "additionalProperties": False,
    "required": ["x", "y"],
    "properties": {
        "x": {"type": "number", "format": "double"},
        "y": {"type": "number", "format": "double"},
    },
}
SYNAPSE = {
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
WIDGET = {
    "title": "Widget",
    "type": "string",
    "contentMediaType": "application/json",
}

SNAPSHOT_PROPS = {
    "schema": {"type": "string", "x-semio-state": "persistent"},
    "camera": {"$ref": "#/$defs/CameraJson", "x-semio-state": "persistent"},
    "widgets": {
        "type": "array",
        "items": {"$ref": "#/$defs/Widget"},
        "x-semio-state": "persistent",
    },
    "synapses": {
        "type": "array",
        "items": {"$ref": "#/$defs/SynapseSpec"},
        "x-semio-state": "persistent",
    },
    "layout": {
        "type": "object",
        "additionalProperties": {"$ref": "#/$defs/WidgetLayout"},
        "x-semio-state": "persistent",
    },
}
SNAPSHOT_REQUIRED = ["schema", "camera", "widgets", "synapses", "layout"]

ARTIFACT_PROPS = {
    **SNAPSHOT_PROPS,
    "selectedNodeIds": {
        "type": "array",
        "items": {"type": "string"},
        "x-semio-state": "shared-ui",
    },
    "selectedEdgeIds": {
        "type": "array",
        "items": {"type": "string"},
        "x-semio-state": "shared-ui",
    },
    "selectedHandleIds": {
        "type": "array",
        "items": {"type": "string"},
        "x-semio-state": "shared-ui",
    },
    "previewOffNodeIds": {
        "type": "array",
        "items": {"type": "string"},
        "x-semio-state": "shared-ui",
    },
    "lodMode": {"type": "string", "x-semio-state": "local-ui"},
    "proximityDistance": {
        "type": "number",
        "format": "double",
        "x-semio-state": "local-ui",
    },
    "gridVisible": {"type": "boolean", "x-semio-state": "local-ui"},
    "gridSnapEnabled": {"type": "boolean", "x-semio-state": "local-ui"},
    "gridFactor": {"type": "number", "format": "double", "x-semio-state": "local-ui"},
    "catalogueSectionsJson": {
        "type": "string",
        "contentMediaType": "application/json",
        "x-semio-state": "local-ui",
    },
    "automationEnabledJson": {
        "type": "string",
        "contentMediaType": "application/json",
        "x-semio-state": "local-ui",
    },
    "contributionsJson": {
        "type": "string",
        "contentMediaType": "application/json",
        "x-semio-state": "local-ui",
    },
    "generationJson": {
        "type": "string",
        "contentMediaType": "application/json",
        "x-semio-state": "local-ui",
    },
    "locale": {"type": "string", "x-semio-state": "local-ui"},
}
ARTIFACT_REQUIRED = SNAPSHOT_REQUIRED + [
    "selectedNodeIds",
    "selectedEdgeIds",
    "selectedHandleIds",
    "previewOffNodeIds",
    "lodMode",
    "proximityDistance",
    "gridVisible",
    "gridSnapEnabled",
    "gridFactor",
    "catalogueSectionsJson",
    "automationEnabledJson",
    "contributionsJson",
    "generationJson",
    "locale",
]

DEFS = {
    "CameraJson": CAMERA,
    "WidgetLayout": WIDGET_LAYOUT,
    "SynapseSpec": SYNAPSE,
    "Widget": WIDGET,
}

# Diff helpers
STRING_LIST = {
    "title": "FlowStringList",
    "type": "object",
    "additionalProperties": False,
    "required": ["values"],
    "properties": {"values": {"type": "array", "items": {"type": "string"}}},
}
WIDGETS_DELTA = {
    "title": "FlowWidgetsDelta",
    "type": "object",
    "additionalProperties": False,
    "required": ["added", "removed", "patched"],
    "properties": {
        "added": {"type": "array", "items": {"$ref": "#/$defs/Widget"}},
        "removed": {"type": "array", "items": {"type": "string"}},
        "patched": {
            "type": "array",
            "items": {"$ref": "#/$defs/FlowWidgetPatchEntry"},
        },
        "reordered": {"type": "array", "items": {"type": "string"}},
    },
}
WIDGET_PATCH_ENTRY = {
    "title": "FlowWidgetPatchEntry",
    "type": "object",
    "additionalProperties": False,
    "required": ["id", "patch"],
    "properties": {
        "id": {"type": "string"},
        "patch": {"$ref": "#/$defs/Widget"},
    },
}
SYNAPSES_DELTA = {
    "title": "FlowSynapsesDelta",
    "type": "object",
    "additionalProperties": False,
    "required": ["added", "removed", "patched"],
    "properties": {
        "added": {"type": "array", "items": {"$ref": "#/$defs/SynapseSpec"}},
        "removed": {"type": "array", "items": {"type": "string"}},
        "patched": {
            "type": "array",
            "items": {"$ref": "#/$defs/FlowSynapsePatchEntry"},
        },
        "reordered": {"type": "array", "items": {"type": "string"}},
    },
}
SYNAPSE_PATCH_ENTRY = {
    "title": "FlowSynapsePatchEntry",
    "type": "object",
    "additionalProperties": False,
    "required": ["id", "patch"],
    "properties": {
        "id": {"type": "string"},
        "patch": {"$ref": "#/$defs/SynapseSpec"},
    },
}
LAYOUT_DELTA = {
    "title": "FlowLayoutMapDelta",
    "type": "object",
    "additionalProperties": False,
    "required": ["entries"],
    "properties": {
        "entries": {
            "type": "object",
            "additionalProperties": {
                "oneOf": [{"type": "null"}, {"$ref": "#/$defs/WidgetLayout"}]
            },
        }
    },
}

DIFF_PROPS = {
    "artifact": {
        "$ref": "#/$defs/FlowArtifact",
        "x-semio-state": "persistent",
    },
    "schema": {"type": "string", "x-semio-state": "persistent"},
    "camera": {"$ref": "#/$defs/CameraJson", "x-semio-state": "persistent"},
    "widgets": {"$ref": "#/$defs/FlowWidgetsDelta", "x-semio-state": "persistent"},
    "synapses": {"$ref": "#/$defs/FlowSynapsesDelta", "x-semio-state": "persistent"},
    "layout": {"$ref": "#/$defs/FlowLayoutMapDelta", "x-semio-state": "persistent"},
    "selectedNodeIds": {"$ref": "#/$defs/FlowStringList", "x-semio-state": "shared-ui"},
    "selectedEdgeIds": {"$ref": "#/$defs/FlowStringList", "x-semio-state": "shared-ui"},
    "selectedHandleIds": {
        "$ref": "#/$defs/FlowStringList",
        "x-semio-state": "shared-ui",
    },
    "previewOffNodeIds": {
        "$ref": "#/$defs/FlowStringList",
        "x-semio-state": "shared-ui",
    },
    "lodMode": {"type": "string", "x-semio-state": "local-ui"},
    "proximityDistance": {
        "type": "number",
        "format": "double",
        "x-semio-state": "local-ui",
    },
    "gridVisible": {"type": "boolean", "x-semio-state": "local-ui"},
    "gridSnapEnabled": {"type": "boolean", "x-semio-state": "local-ui"},
    "gridFactor": {"type": "number", "format": "double", "x-semio-state": "local-ui"},
    "catalogueSectionsJson": {
        "type": "string",
        "contentMediaType": "application/json",
        "x-semio-state": "local-ui",
    },
    "automationEnabledJson": {
        "type": "string",
        "contentMediaType": "application/json",
        "x-semio-state": "local-ui",
    },
    "contributionsJson": {
        "type": "string",
        "contentMediaType": "application/json",
        "x-semio-state": "local-ui",
    },
    "generationJson": {
        "type": "string",
        "contentMediaType": "application/json",
        "x-semio-state": "local-ui",
    },
    "locale": {"type": "string", "x-semio-state": "local-ui"},
}

FLOW_ARTIFACT_DEF = {
    "title": "FlowArtifact",
    "type": "object",
    "additionalProperties": False,
    "required": ARTIFACT_REQUIRED,
    "properties": {k: {kk: vv for kk, vv in v.items() if kk != "x-semio-state"} for k, v in ARTIFACT_PROPS.items()},
}

DIFF_DEFS = {
    **DEFS,
    "FlowArtifact": FLOW_ARTIFACT_DEF,
    "FlowStringList": STRING_LIST,
    "FlowWidgetsDelta": WIDGETS_DELTA,
    "FlowWidgetPatchEntry": WIDGET_PATCH_ENTRY,
    "FlowSynapsesDelta": SYNAPSES_DELTA,
    "FlowSynapsePatchEntry": SYNAPSE_PATCH_ENTRY,
    "FlowLayoutMapDelta": LAYOUT_DELTA,
}


def write_json(path: Path, doc: dict) -> None:
    path.write_text(json.dumps(doc, indent=2) + "\n")


write_json(
    SNAP / "🔣️component.json",
    {
        "$id": "https://semio.tech/schema/s/flow/flow/snapshot.json",
        "title": "FlowSnapshot",
        "type": "object",
        "additionalProperties": False,
        "required": SNAPSHOT_REQUIRED,
        "properties": SNAPSHOT_PROPS,
        "$defs": DEFS,
    },
)

write_json(
    ART / "🔣️component.json",
    {
        "$id": "https://semio.tech/schema/s/flow/flow/artifact.json",
        "title": "FlowArtifact",
        "type": "object",
        "additionalProperties": False,
        "required": ARTIFACT_REQUIRED,
        "properties": ARTIFACT_PROPS,
        "$defs": DEFS,
    },
)

write_json(
    DIFF / "🔣️component.json",
    {
        "$id": "https://semio.tech/schema/s/flow/flow/diff.json",
        "title": "FlowDiff",
        "type": "object",
        "additionalProperties": False,
        "required": [],
        "properties": DIFF_PROPS,
        "$defs": DIFF_DEFS,
    },
)

# ---------------------------------------------------------------------------
# TypeScript
# ---------------------------------------------------------------------------
(SNAP / "🟦️component.ts").write_text(
    """/** 🧬️ Flow snapshot schema — persistent fields only. */

export interface CameraJson {
  x: number;
  y: number;
  zoom: number;
}

export interface WidgetLayout {
  x: number;
  y: number;
}

export interface SynapseSpec {
  id: string;
  from: string;
  to: string;
  fromPort: string;
  toPort: string;
}

/** Widget payload as JSON text (opaque enum). */
export type Widget = string;

export interface FlowSnapshot {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  camera: CameraJson;
  /** @state persistent */
  widgets: Widget[];
  /** @state persistent */
  synapses: SynapseSpec[];
  /** @state persistent */
  layout: Record<string, WidgetLayout>;
}
"""
)

(ART / "🟦️component.ts").write_text(
    """/** 🧬️ Flow artifact schema — every field with its state class. */

export interface CameraJson {
  x: number;
  y: number;
  zoom: number;
}

export interface WidgetLayout {
  x: number;
  y: number;
}

export interface SynapseSpec {
  id: string;
  from: string;
  to: string;
  fromPort: string;
  toPort: string;
}

/** Widget payload as JSON text (opaque enum). */
export type Widget = string;

export interface FlowArtifact {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  camera: CameraJson;
  /** @state persistent */
  widgets: Widget[];
  /** @state persistent */
  synapses: SynapseSpec[];
  /** @state persistent */
  layout: Record<string, WidgetLayout>;
  /** @state shared-ui */
  selectedNodeIds: string[];
  /** @state shared-ui */
  selectedEdgeIds: string[];
  /** @state shared-ui */
  selectedHandleIds: string[];
  /** @state shared-ui */
  previewOffNodeIds: string[];
  /** @state local-ui */
  lodMode: string;
  /** @state local-ui */
  proximityDistance: number;
  /** @state local-ui */
  gridVisible: boolean;
  /** @state local-ui */
  gridSnapEnabled: boolean;
  /** @state local-ui */
  gridFactor: number;
  /** @state local-ui */
  catalogueSectionsJson: string;
  /** @state local-ui */
  automationEnabledJson: string;
  /** @state local-ui */
  contributionsJson: string;
  /** @state local-ui */
  generationJson: string;
  /** @state local-ui */
  locale: string;
}
"""
)

(DIFF / "🟦️component.ts").write_text(
    """/** 🧬️ Flow diff schema — sparse field delta. */

export interface CameraJson {
  x: number;
  y: number;
  zoom: number;
}

export interface WidgetLayout {
  x: number;
  y: number;
}

export interface SynapseSpec {
  id: string;
  from: string;
  to: string;
  fromPort: string;
  toPort: string;
}

/** Widget payload as JSON text (opaque enum). */
export type Widget = string;

export interface FlowArtifact {
  schema: string;
  camera: CameraJson;
  widgets: Widget[];
  synapses: SynapseSpec[];
  layout: Record<string, WidgetLayout>;
  selectedNodeIds: string[];
  selectedEdgeIds: string[];
  selectedHandleIds: string[];
  previewOffNodeIds: string[];
  lodMode: string;
  proximityDistance: number;
  gridVisible: boolean;
  gridSnapEnabled: boolean;
  gridFactor: number;
  catalogueSectionsJson: string;
  automationEnabledJson: string;
  contributionsJson: string;
  generationJson: string;
  locale: string;
}

export interface FlowStringList {
  values: string[];
}

export interface FlowWidgetPatchEntry {
  id: string;
  patch: Widget;
}

export interface FlowWidgetsDelta {
  added: Widget[];
  removed: string[];
  patched: FlowWidgetPatchEntry[];
  reordered?: string[];
}

export interface FlowSynapsePatchEntry {
  id: string;
  patch: SynapseSpec;
}

export interface FlowSynapsesDelta {
  added: SynapseSpec[];
  removed: string[];
  patched: FlowSynapsePatchEntry[];
  reordered?: string[];
}

export interface FlowLayoutMapDelta {
  entries: Record<string, WidgetLayout | null>;
}

export interface FlowDiff {
  /** @state persistent */
  artifact?: FlowArtifact;
  /** @state persistent */
  schema?: string;
  /** @state persistent */
  camera?: CameraJson;
  /** @state persistent */
  widgets?: FlowWidgetsDelta;
  /** @state persistent */
  synapses?: FlowSynapsesDelta;
  /** @state persistent */
  layout?: FlowLayoutMapDelta;
  /** @state shared-ui */
  selectedNodeIds?: FlowStringList;
  /** @state shared-ui */
  selectedEdgeIds?: FlowStringList;
  /** @state shared-ui */
  selectedHandleIds?: FlowStringList;
  /** @state shared-ui */
  previewOffNodeIds?: FlowStringList;
  /** @state local-ui */
  lodMode?: string;
  /** @state local-ui */
  proximityDistance?: number;
  /** @state local-ui */
  gridVisible?: boolean;
  /** @state local-ui */
  gridSnapEnabled?: boolean;
  /** @state local-ui */
  gridFactor?: number;
  /** @state local-ui */
  catalogueSectionsJson?: string;
  /** @state local-ui */
  automationEnabledJson?: string;
  /** @state local-ui */
  contributionsJson?: string;
  /** @state local-ui */
  generationJson?: string;
  /** @state local-ui */
  locale?: string;
}
"""
)

# ---------------------------------------------------------------------------
# GraphQL
# ---------------------------------------------------------------------------
(SNAP / "🔗️component.graphql").write_text(
    """# 🧬️ Flow snapshot schema — persistent fields only.

type FlowSnapshot {
  schema: String! @state(class: PERSISTENT)
  camera: CameraJson! @state(class: PERSISTENT)
  widgets: [Widget!]! @state(class: PERSISTENT)
  synapses: [SynapseSpec!]! @state(class: PERSISTENT)
  layout: [WidgetLayoutEntry!]! @state(class: PERSISTENT)
}

type CameraJson {
  x: Float!
  y: Float!
  zoom: Float!
}

type WidgetLayout {
  x: Float!
  y: Float!
}

type WidgetLayoutEntry {
  key: String!
  value: WidgetLayout!
}

type SynapseSpec {
  id: String!
  from: String!
  to: String!
  fromPort: String!
  toPort: String!
}

scalar Widget
"""
)

(ART / "🔗️component.graphql").write_text(
    """# 🧬️ Flow artifact schema — every field with its state class.

type FlowArtifact {
  schema: String! @state(class: PERSISTENT)
  camera: CameraJson! @state(class: PERSISTENT)
  widgets: [Widget!]! @state(class: PERSISTENT)
  synapses: [SynapseSpec!]! @state(class: PERSISTENT)
  layout: [WidgetLayoutEntry!]! @state(class: PERSISTENT)
  selectedNodeIds: [String!]! @state(class: SHARED_UI)
  selectedEdgeIds: [String!]! @state(class: SHARED_UI)
  selectedHandleIds: [String!]! @state(class: SHARED_UI)
  previewOffNodeIds: [String!]! @state(class: SHARED_UI)
  lodMode: String! @state(class: LOCAL_UI)
  proximityDistance: Float! @state(class: LOCAL_UI)
  gridVisible: Boolean! @state(class: LOCAL_UI)
  gridSnapEnabled: Boolean! @state(class: LOCAL_UI)
  gridFactor: Float! @state(class: LOCAL_UI)
  catalogueSectionsJson: String! @state(class: LOCAL_UI)
  automationEnabledJson: String! @state(class: LOCAL_UI)
  contributionsJson: String! @state(class: LOCAL_UI)
  generationJson: String! @state(class: LOCAL_UI)
  locale: String! @state(class: LOCAL_UI)
}

type CameraJson {
  x: Float!
  y: Float!
  zoom: Float!
}

type WidgetLayout {
  x: Float!
  y: Float!
}

type WidgetLayoutEntry {
  key: String!
  value: WidgetLayout!
}

type SynapseSpec {
  id: String!
  from: String!
  to: String!
  fromPort: String!
  toPort: String!
}

scalar Widget
"""
)

(DIFF / "🔗️component.graphql").write_text(
    """# 🧬️ Flow diff schema — sparse field delta.

type FlowDiff {
  artifact: FlowArtifact @state(class: PERSISTENT)
  schema: String @state(class: PERSISTENT)
  camera: CameraJson @state(class: PERSISTENT)
  widgets: FlowWidgetsDelta @state(class: PERSISTENT)
  synapses: FlowSynapsesDelta @state(class: PERSISTENT)
  layout: FlowLayoutMapDelta @state(class: PERSISTENT)
  selectedNodeIds: FlowStringList @state(class: SHARED_UI)
  selectedEdgeIds: FlowStringList @state(class: SHARED_UI)
  selectedHandleIds: FlowStringList @state(class: SHARED_UI)
  previewOffNodeIds: FlowStringList @state(class: SHARED_UI)
  lodMode: String @state(class: LOCAL_UI)
  proximityDistance: Float @state(class: LOCAL_UI)
  gridVisible: Boolean @state(class: LOCAL_UI)
  gridSnapEnabled: Boolean @state(class: LOCAL_UI)
  gridFactor: Float @state(class: LOCAL_UI)
  catalogueSectionsJson: String @state(class: LOCAL_UI)
  automationEnabledJson: String @state(class: LOCAL_UI)
  contributionsJson: String @state(class: LOCAL_UI)
  generationJson: String @state(class: LOCAL_UI)
  locale: String @state(class: LOCAL_UI)
}

type FlowArtifact {
  schema: String!
  camera: CameraJson!
  widgets: [Widget!]!
  synapses: [SynapseSpec!]!
  layout: [WidgetLayoutEntry!]!
  selectedNodeIds: [String!]!
  selectedEdgeIds: [String!]!
  selectedHandleIds: [String!]!
  previewOffNodeIds: [String!]!
  lodMode: String!
  proximityDistance: Float!
  gridVisible: Boolean!
  gridSnapEnabled: Boolean!
  gridFactor: Float!
  catalogueSectionsJson: String!
  automationEnabledJson: String!
  contributionsJson: String!
  generationJson: String!
  locale: String!
}

type CameraJson {
  x: Float!
  y: Float!
  zoom: Float!
}

type WidgetLayout {
  x: Float!
  y: Float!
}

type WidgetLayoutEntry {
  key: String!
  value: WidgetLayout!
}

type SynapseSpec {
  id: String!
  from: String!
  to: String!
  fromPort: String!
  toPort: String!
}

scalar Widget

type FlowStringList {
  values: [String!]!
}

type FlowWidgetPatchEntry {
  id: String!
  patch: Widget!
}

type FlowWidgetsDelta {
  added: [Widget!]!
  removed: [String!]!
  patched: [FlowWidgetPatchEntry!]!
  reordered: [String!]
}

type FlowSynapsePatchEntry {
  id: String!
  patch: SynapseSpec!
}

type FlowSynapsesDelta {
  added: [SynapseSpec!]!
  removed: [String!]!
  patched: [FlowSynapsePatchEntry!]!
  reordered: [String!]
}

type FlowLayoutMapDelta {
  entries: [FlowLayoutNullableEntry!]!
}

type FlowLayoutNullableEntry {
  key: String!
  value: WidgetLayout
}
"""
)

# ---------------------------------------------------------------------------
# Proto
# ---------------------------------------------------------------------------
(SNAP / "🛰️component.proto").write_text(
    """syntax = "proto3";
package semio.s.flow.flow.snapshot;

// 🧬️ Flow snapshot schema — persistent fields only.

message FlowSnapshot {
  // @state persistent
  string schema = 1;
  // @state persistent
  CameraJson camera = 2;
  // @state persistent
  repeated Widget widgets = 3;
  // @state persistent
  repeated SynapseSpec synapses = 4;
  // @state persistent
  map<string, WidgetLayout> layout = 5;
}

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
"""
)

(ART / "🛰️component.proto").write_text(
    """syntax = "proto3";
package semio.s.flow.flow.artifact;

// 🧬️ Flow artifact schema — every field with its state class.

message FlowArtifact {
  // @state persistent
  string schema = 1;
  // @state persistent
  CameraJson camera = 2;
  // @state persistent
  repeated Widget widgets = 3;
  // @state persistent
  repeated SynapseSpec synapses = 4;
  // @state persistent
  map<string, WidgetLayout> layout = 5;
  // @state shared-ui
  repeated string selected_node_ids = 6;
  // @state shared-ui
  repeated string selected_edge_ids = 7;
  // @state shared-ui
  repeated string selected_handle_ids = 8;
  // @state shared-ui
  repeated string preview_off_node_ids = 9;
  // @state local-ui
  string lod_mode = 10;
  // @state local-ui
  double proximity_distance = 11;
  // @state local-ui
  bool grid_visible = 12;
  // @state local-ui
  bool grid_snap_enabled = 13;
  // @state local-ui
  double grid_factor = 14;
  // @state local-ui
  string catalogue_sections_json = 15;
  // @state local-ui
  string automation_enabled_json = 16;
  // @state local-ui
  string contributions_json = 17;
  // @state local-ui
  string generation_json = 18;
  // @state local-ui
  string locale = 19;
}

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
"""
)

(DIFF / "🛰️component.proto").write_text(
    """syntax = "proto3";
package semio.s.flow.flow.diff;

// 🧬️ Flow diff schema — sparse field delta.

message FlowDiff {
  // @state persistent
  optional FlowArtifact artifact = 1;
  // @state persistent
  optional string schema = 2;
  // @state persistent
  optional CameraJson camera = 3;
  // @state persistent
  optional FlowWidgetsDelta widgets = 4;
  // @state persistent
  optional FlowSynapsesDelta synapses = 5;
  // @state persistent
  optional FlowLayoutMapDelta layout = 6;
  // @state shared-ui
  optional FlowStringList selected_node_ids = 7;
  // @state shared-ui
  optional FlowStringList selected_edge_ids = 8;
  // @state shared-ui
  optional FlowStringList selected_handle_ids = 9;
  // @state shared-ui
  optional FlowStringList preview_off_node_ids = 10;
  // @state local-ui
  optional string lod_mode = 11;
  // @state local-ui
  optional double proximity_distance = 12;
  // @state local-ui
  optional bool grid_visible = 13;
  // @state local-ui
  optional bool grid_snap_enabled = 14;
  // @state local-ui
  optional double grid_factor = 15;
  // @state local-ui
  optional string catalogue_sections_json = 16;
  // @state local-ui
  optional string automation_enabled_json = 17;
  // @state local-ui
  optional string contributions_json = 18;
  // @state local-ui
  optional string generation_json = 19;
  // @state local-ui
  optional string locale = 20;
}

message FlowArtifact {
  string schema = 1;
  CameraJson camera = 2;
  repeated Widget widgets = 3;
  repeated SynapseSpec synapses = 4;
  map<string, WidgetLayout> layout = 5;
  repeated string selected_node_ids = 6;
  repeated string selected_edge_ids = 7;
  repeated string selected_handle_ids = 8;
  repeated string preview_off_node_ids = 9;
  string lod_mode = 10;
  double proximity_distance = 11;
  bool grid_visible = 12;
  bool grid_snap_enabled = 13;
  double grid_factor = 14;
  string catalogue_sections_json = 15;
  string automation_enabled_json = 16;
  string contributions_json = 17;
  string generation_json = 18;
  string locale = 19;
}

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

message FlowStringList {
  repeated string values = 1;
}

message FlowWidgetPatchEntry {
  string id = 1;
  Widget patch = 2;
}

message FlowWidgetsDelta {
  repeated Widget added = 1;
  repeated string removed = 2;
  repeated FlowWidgetPatchEntry patched = 3;
  repeated string reordered = 4;
}

message FlowSynapsePatchEntry {
  string id = 1;
  SynapseSpec patch = 2;
}

message FlowSynapsesDelta {
  repeated SynapseSpec added = 1;
  repeated string removed = 2;
  repeated FlowSynapsePatchEntry patched = 3;
  repeated string reordered = 4;
}

message FlowLayoutMapDelta {
  map<string, WidgetLayout> entries = 1;
}
"""
)

print("wrote json/ts/graphql/proto leaves to", ROOT)
PY
