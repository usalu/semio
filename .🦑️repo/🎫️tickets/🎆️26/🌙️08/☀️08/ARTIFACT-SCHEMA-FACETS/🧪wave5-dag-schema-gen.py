#!/usr/bin/env python3
"""Generate handcrafted schema mirror leaves for dag wave-5."""
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag")

NODE_DEF = """    "DagNodeSpec": {
      "title": "DagNodeSpec",
      "type": "object",
      "additionalProperties": true,
      "required": ["id"],
      "properties": {
        "id": { "type": "string" }
      }
    },
    "DagFixtureEdge": {
      "title": "DagFixtureEdge",
      "type": "object",
      "additionalProperties": false,
      "required": ["id", "source", "target"],
      "properties": {
        "id": { "type": "string" },
        "source": { "type": "string" },
        "target": { "type": "string" }
      }
    }"""

CAMERA_DEF = """    "DagCamera": {
      "title": "DagCamera",
      "type": "object",
      "additionalProperties": false,
      "required": ["x", "y", "zoom"],
      "properties": {
        "x": { "type": "number", "format": "double" },
        "y": { "type": "number", "format": "double" },
        "zoom": { "type": "number", "format": "double" }
      }
    }"""

STRING_LIST = """    "DagStringList": {
      "title": "DagStringList",
      "type": "object",
      "additionalProperties": false,
      "required": ["values"],
      "properties": {
        "values": { "type": "array", "items": { "type": "string" } }
      }
    }"""

NODES_DELTA = """    "DagNodesDelta": {
      "title": "DagNodesDelta",
      "type": "object",
      "additionalProperties": false,
      "required": ["added", "removed", "patched"],
      "properties": {
        "added": { "type": "array", "items": { "$ref": "#/$defs/DagNodeSpec" } },
        "removed": { "type": "array", "items": { "type": "string" } },
        "patched": { "type": "array", "items": { "$ref": "#/$defs/DagNodePatchEntry" } },
        "reordered": { "type": "array", "items": { "type": "string" } }
      }
    },
    "DagEdgesDelta": {
      "title": "DagEdgesDelta",
      "type": "object",
      "additionalProperties": false,
      "required": ["added", "removed", "patched"],
      "properties": {
        "added": { "type": "array", "items": { "$ref": "#/$defs/DagFixtureEdge" } },
        "removed": { "type": "array", "items": { "type": "string" } },
        "patched": { "type": "array", "items": { "$ref": "#/$defs/DagEdgePatchEntry" } },
        "reordered": { "type": "array", "items": { "type": "string" } }
      }
    },
    "DagNodePatchEntry": {
      "title": "DagNodePatchEntry",
      "type": "object",
      "additionalProperties": false,
      "required": ["id", "patch"],
      "properties": {
        "id": { "type": "string" },
        "patch": { "$ref": "#/$defs/DagNodePatch" }
      }
    },
    "DagEdgePatchEntry": {
      "title": "DagEdgePatchEntry",
      "type": "object",
      "additionalProperties": false,
      "required": ["id", "patch"],
      "properties": {
        "id": { "type": "string" },
        "patch": { "$ref": "#/$defs/DagEdgePatch" }
      }
    },
    "DagNodePatch": {
      "title": "DagNodePatch",
      "type": "object",
      "additionalProperties": true
    },
    "DagEdgePatch": {
      "title": "DagEdgePatch",
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "source": { "type": "string" },
        "target": { "type": "string" }
      }
    }"""

ARTIFACT_JSON = f"""{{
  "$id": "https://semio.tech/schema/s/dag/dag/artifact.json",
  "title": "DagArtifact",
  "type": "object",
  "additionalProperties": false,
  "required": ["schema", "nodes", "edges", "selectedNodeIds", "camera", "locale"],
  "properties": {{
    "schema": {{ "type": "string", "x-semio-state": "persistent" }},
    "nodes": {{ "type": "array", "items": {{ "$ref": "#/$defs/DagNodeSpec" }}, "x-semio-state": "persistent" }},
    "edges": {{ "type": "array", "items": {{ "$ref": "#/$defs/DagFixtureEdge" }}, "x-semio-state": "persistent" }},
    "selectedNodeIds": {{ "type": "array", "items": {{ "type": "string" }}, "x-semio-state": "shared-ui" }},
    "camera": {{ "$ref": "#/$defs/DagCamera", "x-semio-state": "local-ui" }},
    "locale": {{ "type": "string", "x-semio-state": "local-ui" }}
  }},
  "$defs": {{
{NODE_DEF},
{CAMERA_DEF}
  }}
}}"""

SNAPSHOT_JSON = f"""{{
  "$id": "https://semio.tech/schema/s/dag/dag/snapshot.json",
  "title": "DagSnapshot",
  "type": "object",
  "additionalProperties": false,
  "required": ["schema", "nodes", "edges"],
  "properties": {{
    "schema": {{ "type": "string", "x-semio-state": "persistent" }},
    "nodes": {{ "type": "array", "items": {{ "$ref": "#/$defs/DagNodeSpec" }}, "x-semio-state": "persistent" }},
    "edges": {{ "type": "array", "items": {{ "$ref": "#/$defs/DagFixtureEdge" }}, "x-semio-state": "persistent" }}
  }},
  "$defs": {{
{NODE_DEF}
  }}
}}"""

DIFF_JSON = f"""{{
  "$id": "https://semio.tech/schema/s/dag/dag/diff.json",
  "title": "DagDiff",
  "type": "object",
  "additionalProperties": false,
  "required": [],
  "properties": {{
    "artifact": {{ "title": "DagArtifact", "type": "object", "x-semio-state": "persistent" }},
    "schema": {{ "type": "string", "x-semio-state": "persistent" }},
    "nodes": {{ "$ref": "#/$defs/DagNodesDelta", "x-semio-state": "persistent" }},
    "edges": {{ "$ref": "#/$defs/DagEdgesDelta", "x-semio-state": "persistent" }},
    "setNodes": {{ "type": "array", "items": {{ "$ref": "#/$defs/DagNodeSpec" }}, "x-semio-state": "persistent" }},
    "setEdges": {{ "type": "array", "items": {{ "$ref": "#/$defs/DagFixtureEdge" }}, "x-semio-state": "persistent" }},
    "selectedNodeIds": {{ "$ref": "#/$defs/DagStringList", "x-semio-state": "shared-ui" }},
    "camera": {{ "$ref": "#/$defs/DagCamera", "x-semio-state": "local-ui" }},
    "locale": {{ "type": "string", "x-semio-state": "local-ui" }}
  }},
  "$defs": {{
{STRING_LIST},
{NODES_DELTA},
{NODE_DEF},
{CAMERA_DEF}
  }}
}}"""

ARTIFACT_RS = '''//! 🧬️ DAG artifact schema — every field of the artifact with its state class.

use crate::artifacts::dag::{DagCamera, DagFixtureEdge, DagNodeSpec, DAG_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full DAG artifact state across persistent, shared-ui and local-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.dag.dag")]
pub struct DagArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub nodes: Vec<DagNodeSpec>,
    #[state(persistent)]
    #[serde(default)]
    pub edges: Vec<DagFixtureEdge>,
    #[state(shared_ui)]
    #[serde(default)]
    pub selected_node_ids: Vec<String>,
    #[state(local_ui)]
    pub camera: DagCamera,
    #[state(local_ui)]
    pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for DagArtifact {
    fn default() -> Self {
        Self::from_snapshot(crate::artifacts::dag::default_snapshot())
    }
}

impl DagArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::dag::DagSnapshot {
        crate::artifacts::dag::DagSnapshot {
            schema: self.schema.clone(),
            nodes: self.nodes.clone(),
            edges: self.edges.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::dag::DagSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            nodes: snapshot.nodes,
            edges: snapshot.edges,
            selected_node_ids: Vec::new(),
            camera: DagCamera::default(),
            locale: "en-US".into(),
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::dag::DagSnapshot) {
        self.schema = snapshot.schema;
        self.nodes = snapshot.nodes;
        self.edges = snapshot.edges;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.dag.dag` — fifteen handcrafted schema leaves.
pub fn dag_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.dag.dag",
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
'''

SNAPSHOT_RS = '''//! 🧬️ DAG snapshot schema — persistent fields only.

use crate::artifacts::dag::{DagFixtureEdge, DagNodeSpec, DAG_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted DAG document snapshot (nodes + edges).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.dag.dag")]
pub struct DagSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub nodes: Vec<DagNodeSpec>,
    #[state(persistent)]
    #[serde(default)]
    pub edges: Vec<DagFixtureEdge>,
}

impl Default for DagSnapshot {
    fn default() -> Self {
        default_snapshot()
    }
}

/// 🌱 Canonical default document used by the play app and examples.
pub fn default_snapshot() -> DagSnapshot {
    let kernel = infinite_board_port_directed_dag::default_dag_document();
    DagSnapshot {
        schema: DAG_DOCUMENT_SCHEMA.into(),
        nodes: kernel.nodes,
        edges: kernel.edges,
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️DocumentCodecs
impl From<DagSnapshot> for infinite_board_port_directed_dag::DagDocument {
    fn from(value: DagSnapshot) -> Self {
        Self { schema: value.schema, nodes: value.nodes, edges: value.edges }
    }
}

impl From<infinite_board_port_directed_dag::DagDocument> for DagSnapshot {
    fn from(value: infinite_board_port_directed_dag::DagDocument) -> Self {
        Self { schema: value.schema, nodes: value.nodes, edges: value.edges }
    }
}

impl From<&DagSnapshot> for infinite_board_port_directed_dag::DagDocument {
    fn from(value: &DagSnapshot) -> Self {
        value.clone().into()
    }
}

impl store::DocumentDsl for DagSnapshot {
    const EXTENSION: &'static str = "dag";
    fn envelope_id() -> &'static str {
        DAG_DOCUMENT_SCHEMA
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        Ok(infinite_board_port_directed_dag::DagDocument::parse_dsl(text)?.into())
    }
    fn print_dsl(&self) -> String {
        infinite_board_port_directed_dag::DagDocument::from(self).print_dsl()
    }
}

impl store::DocumentPack for DagSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        infinite_board_port_directed_dag::DagDocument::from(self).encode_pack_with(options)
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        Ok(infinite_board_port_directed_dag::DagDocument::decode_pack_with(bytes, options)?.into())
    }
}
//#endregion 🔖️DocumentCodecs
'''

DIFF_RS = '''//! 🧬️ DAG diff schema — sparse field delta over the artifact.

use crate::artifacts::dag::{DagCamera, DagFixtureEdge, DagNodePatch, DagNodeSpec};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the DAG artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.dag.dag")]
pub struct DagDiff {
    #[state(persistent)]
    pub artifact: Option<Box<crate::artifacts::dag::schema::DagArtifact>>,
    #[state(persistent)]
    pub schema: Option<String>,
    #[state(persistent)]
    pub nodes: Option<DagNodesDelta>,
    #[state(persistent)]
    pub edges: Option<DagEdgesDelta>,
    #[state(persistent)]
    pub set_nodes: Option<Vec<DagNodeSpec>>,
    #[state(persistent)]
    pub set_edges: Option<Vec<DagFixtureEdge>>,
    #[state(shared_ui)]
    pub selected_node_ids: Option<DagStringList>,
    #[state(local_ui)]
    pub camera: Option<DagCamera>,
    #[state(local_ui)]
    pub locale: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DagStringList {
    pub values: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DagNodesDelta {
    pub added: Vec<DagNodeSpec>,
    pub removed: Vec<String>,
    pub patched: Vec<DagNodePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DagEdgesDelta {
    pub added: Vec<DagFixtureEdge>,
    pub removed: Vec<String>,
    pub patched: Vec<DagEdgePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DagNodePatchEntry {
    pub id: String,
    pub patch: DagNodePatch,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DagEdgePatchEntry {
    pub id: String,
    pub patch: infinite_board_port_directed_dag::DagEdgePatch,
}
//#endregion 🔖️DeltaHelpers
'''

def write_leaf(dir_path: Path, json_text: str, rust_text: str, gql_type: str, ts_iface: str, proto_msg: str):
    dir_path.mkdir(parents=True, exist_ok=True)
    (dir_path / "🔣️component.json").write_text(json_text.strip() + "\n", encoding="utf-8")
    (dir_path / "🦀️component.rs").write_text(rust_text, encoding="utf-8")
    (dir_path / "🔗️component.graphql").write_text(gql_type.strip() + "\n", encoding="utf-8")
    (dir_path / "🟦️component.ts").write_text(ts_iface.strip() + "\n", encoding="utf-8")
    (dir_path / "🛰️component.proto").write_text(proto_msg.strip() + "\n", encoding="utf-8")

ARTIFACT_GQL = """# 🧬️ DAG artifact schema.
type DagArtifact {
  schema: String! @state(class: PERSISTENT)
  nodes: [DagNodeSpec!]! @state(class: PERSISTENT)
  edges: [DagFixtureEdge!]! @state(class: PERSISTENT)
  selectedNodeIds: [String!]! @state(class: SHARED_UI)
  camera: DagCamera! @state(class: LOCAL_UI)
  locale: String! @state(class: LOCAL_UI)
}
type DagNodeSpec { id: String! }
type DagFixtureEdge { id: String! source: String! target: String! }
type DagCamera { x: Float! y: Float! zoom: Float! }
"""

ARTIFACT_TS = """export interface DagCamera { x: number; y: number; zoom: number }
export interface DagFixtureEdge { id: string; source: string; target: string }
export interface DagNodeSpec { id: string; [key: string]: unknown }
export interface DagArtifact {
  schema: string
  nodes: DagNodeSpec[]
  edges: DagFixtureEdge[]
  selectedNodeIds: string[]
  camera: DagCamera
  locale: string
}
"""

ARTIFACT_PROTO = """syntax = "proto3";
package semio.s.dag.dag;
message DagCamera { double x = 1; double y = 2; double zoom = 3; }
message DagFixtureEdge { string id = 1; string source = 2; string target = 3; }
message DagNodeSpec { string id = 1; }
message DagArtifact {
  string schema = 1;
  repeated DagNodeSpec nodes = 2;
  repeated DagFixtureEdge edges = 3;
  repeated string selected_node_ids = 4;
  DagCamera camera = 5;
  string locale = 6;
}
"""

SNAPSHOT_GQL = """# 📸️ DAG snapshot schema.
type DagSnapshot {
  schema: String! @state(class: PERSISTENT)
  nodes: [DagNodeSpec!]! @state(class: PERSISTENT)
  edges: [DagFixtureEdge!]! @state(class: PERSISTENT)
}
type DagNodeSpec { id: String! }
type DagFixtureEdge { id: String! source: String! target: String! }
"""

SNAPSHOT_TS = """export interface DagFixtureEdge { id: string; source: string; target: string }
export interface DagNodeSpec { id: string; [key: string]: unknown }
export interface DagSnapshot {
  schema: string
  nodes: DagNodeSpec[]
  edges: DagFixtureEdge[]
}
"""

SNAPSHOT_PROTO = """syntax = "proto3";
package semio.s.dag.dag.snapshot;
message DagFixtureEdge { string id = 1; string source = 2; string target = 3; }
message DagNodeSpec { string id = 1; }
message DagSnapshot {
  string schema = 1;
  repeated DagNodeSpec nodes = 2;
  repeated DagFixtureEdge edges = 3;
}
"""

DIFF_GQL = """# 🔺️ DAG diff schema.
type DagDiff {
  artifact: DagArtifact @state(class: PERSISTENT)
  schema: String @state(class: PERSISTENT)
  nodes: DagNodesDelta @state(class: PERSISTENT)
  edges: DagEdgesDelta @state(class: PERSISTENT)
  setNodes: [DagNodeSpec!] @state(class: PERSISTENT)
  setEdges: [DagFixtureEdge!] @state(class: PERSISTENT)
  selectedNodeIds: DagStringList @state(class: SHARED_UI)
  camera: DagCamera @state(class: LOCAL_UI)
  locale: String @state(class: LOCAL_UI)
}
type DagStringList { values: [String!]! }
type DagNodesDelta { added: [DagNodeSpec!]! removed: [String!]! patched: [DagNodePatchEntry!]! reordered: [String!] }
type DagEdgesDelta { added: [DagFixtureEdge!]! removed: [String!]! patched: [DagEdgePatchEntry!]! reordered: [String!] }
type DagNodePatchEntry { id: String! patch: DagNodePatch! }
type DagEdgePatchEntry { id: String! patch: DagEdgePatch! }
type DagNodePatch { name: String x: Float y: Float }
type DagEdgePatch { source: String target: String }
type DagNodeSpec { id: String! }
type DagFixtureEdge { id: String! source: String! target: String! }
type DagCamera { x: Float! y: Float! zoom: Float! }
type DagArtifact { schema: String! nodes: [DagNodeSpec!]! edges: [DagFixtureEdge!]! selectedNodeIds: [String!]! camera: DagCamera! locale: String! }
"""

DIFF_TS = """export interface DagStringList { values: string[] }
export interface DagNodePatch { name?: string; x?: number; y?: number }
export interface DagEdgePatch { source?: string; target?: string }
export interface DagNodePatchEntry { id: string; patch: DagNodePatch }
export interface DagEdgePatchEntry { id: string; patch: DagEdgePatch }
export interface DagNodesDelta { added: DagNodeSpec[]; removed: string[]; patched: DagNodePatchEntry[]; reordered?: string[] }
export interface DagEdgesDelta { added: DagFixtureEdge[]; removed: string[]; patched: DagEdgePatchEntry[]; reordered?: string[] }
export interface DagCamera { x: number; y: number; zoom: number }
export interface DagFixtureEdge { id: string; source: string; target: string }
export interface DagNodeSpec { id: string; [key: string]: unknown }
export interface DagArtifact { schema: string; nodes: DagNodeSpec[]; edges: DagFixtureEdge[]; selectedNodeIds: string[]; camera: DagCamera; locale: string }
export interface DagDiff {
  artifact?: DagArtifact
  schema?: string
  nodes?: DagNodesDelta
  edges?: DagEdgesDelta
  setNodes?: DagNodeSpec[]
  setEdges?: DagFixtureEdge[]
  selectedNodeIds?: DagStringList
  camera?: DagCamera
  locale?: string
}
"""

DIFF_PROTO = """syntax = "proto3";
package semio.s.dag.dag.diff;
message DagStringList { repeated string values = 1; }
message DagNodePatch { optional string name = 1; optional double x = 2; optional double y = 3; }
message DagEdgePatch { optional string source = 1; optional string target = 2; }
message DagNodePatchEntry { string id = 1; DagNodePatch patch = 2; }
message DagEdgePatchEntry { string id = 1; DagEdgePatch patch = 2; }
message DagNodesDelta { repeated DagNodeSpec added = 1; repeated string removed = 2; repeated DagNodePatchEntry patched = 3; repeated string reordered = 4; }
message DagEdgesDelta { repeated DagFixtureEdge added = 1; repeated string removed = 2; repeated DagEdgePatchEntry patched = 3; repeated string reordered = 4; }
message DagCamera { double x = 1; double y = 2; double zoom = 3; }
message DagFixtureEdge { string id = 1; string source = 2; string target = 3; }
message DagNodeSpec { string id = 1; }
message DagArtifact { string schema = 1; repeated DagNodeSpec nodes = 2; repeated DagFixtureEdge edges = 3; repeated string selected_node_ids = 4; DagCamera camera = 5; string locale = 6; }
message DagDiff {
  optional DagArtifact artifact = 1;
  optional string schema = 2;
  optional DagNodesDelta nodes = 3;
  optional DagEdgesDelta edges = 4;
  repeated DagNodeSpec set_nodes = 5;
  repeated DagFixtureEdge set_edges = 6;
  optional DagStringList selected_node_ids = 7;
  optional DagCamera camera = 8;
  optional string locale = 9;
}
"""

write_leaf(ROOT / "🧬️schema", ARTIFACT_JSON, ARTIFACT_RS, ARTIFACT_GQL, ARTIFACT_TS, ARTIFACT_PROTO)
write_leaf(ROOT / "📸️snapshot/🧬️schema", SNAPSHOT_JSON, SNAPSHOT_RS, SNAPSHOT_GQL, SNAPSHOT_TS, SNAPSHOT_PROTO)
write_leaf(ROOT / "🔺️diff/🧬️schema", DIFF_JSON, DIFF_RS, DIFF_GQL, DIFF_TS, DIFF_PROTO)
print("wrote 15 leaves")
