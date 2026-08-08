#!/usr/bin/env python3
"""🧪 Wave-5 remodel facet leaf generator + structural rename helper."""
from __future__ import annotations

import pathlib
import re
import shutil

ROOT = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel")
ART = ROOT / "🗿️artifacts" / "📸️remodel"
TICKET = pathlib.Path("/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/� combos08/☀️08/ARTIFACT-SCHEMA-FACETS")
# fix emoji path
TICKET = next(
    p.parent
    for p in pathlib.Path("/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets").rglob("📋️fanout-brief.md")
    if "ARTIFACT-SCHEMA" in str(p)
)

SCHEMA_ID = "s.remodel.remodel"
PLUGIN_KEY = "remodel"
ARTIFACT_KEY = "remodel"

# Top-level artifact fields: (camel, snake, state, rust_type, optional, cardinality, scalar)
# cardinality: scalar|list|map
ARTIFACT_FIELDS = [
    ("schema", "schema", "persistent", "String", False, "scalar", "string"),
    ("id", "id", "persistent", "String", False, "scalar", "string"),
    ("streams", "streams", "persistent", "Vec<MediaStream>", False, "list", "MediaStream"),
    ("assets", "assets", "persistent", "BTreeMap<String, ImageAsset>", False, "map", "ImageAsset"),
    ("calibration", "calibration", "persistent", "CalibrationState", False, "scalar", "CalibrationState"),
    ("params", "params", "persistent", "ReconstructionParams", False, "scalar", "ReconstructionParams"),
    ("gcps", "gcps", "persistent", "Vec<GroundControlPoint>", False, "list", "GroundControlPoint"),
    ("job", "job", "persistent", "ReconstructionJob", False, "scalar", "ReconstructionJob"),
    ("results", "results", "persistent", "ReconstructionResults", False, "scalar", "ReconstructionResults"),
    # shared-ui (from RemodelConfig)
    ("selection", "selection", "shared-ui", "RemodelUiSelection", False, "scalar", "RemodelUiSelection"),
    ("activeUtilityId", "active_utility_id", "shared-ui", "String", False, "scalar", "string"),
    ("reportTable", "report_table", "shared-ui", "String", False, "scalar", "string"),
    ("frameCursor", "frame_cursor", "shared-ui", "RemodelUiFrameCursor", False, "scalar", "RemodelUiFrameCursor"),
    # local-ui
    ("camera", "camera", "local-ui", "RemodelUiCamera", False, "scalar", "RemodelUiCamera"),
    ("layers", "layers", "local-ui", "RemodelUiLayers", False, "scalar", "RemodelUiLayers"),
    ("locale", "locale", "local-ui", "String", False, "scalar", "string"),
]

SNAPSHOT_FIELDS = [f for f in ARTIFACT_FIELDS if f[2] == "persistent"]

# Diff fields: artifact whole-replace + one Option entry per non-effect field
# For list fields in diff: named wrapper RemodelXList
# For nullable Option fields: Option<Option<T>> — none here at top level except frameCursor.stream_id nested

DIFF_LIST_WRAPPERS = {
    "streams": ("RemodelMediaStreamList", "MediaStream"),
    "gcps": ("RemodelGcpList", "GroundControlPoint"),
}


def state_rust(s: str) -> str:
    return s.replace("-", "_")


def state_gql(s: str) -> str:
    return s.upper().replace("-", "_")


def write(path: pathlib.Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text if text.endswith("\n") else text + "\n")
    print("wrote", path.relative_to(ROOT) if path.is_relative_to(ROOT) else path)


def gen_artifact_rust() -> str:
    lines = [
        "//! 🧬️ Remodel artifact schema — every field of the artifact with its state class.",
        "",
        "use crate::artifacts::remodel::{",
        "    CalibrationState, GroundControlPoint, ImageAsset, MediaStream, ReconstructionJob,",
        "    ReconstructionParams, ReconstructionResults, RemodelSnapshot, REMODEL_DOCUMENT_SCHEMA,",
        "};",
        "use schema::ArtifactSchema;",
        "use serde::{Deserialize, Serialize};",
        "use std::collections::BTreeMap;",
        "",
        "//#region 🔖️UiHelpers",
        "/// 🎥️ Artifact-owned orbit camera (mirror of app config camera).",
        "#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]",
        "#[serde(rename_all = \"camelCase\", default)]",
        "pub struct RemodelUiCamera {",
        "    pub position: [f64; 3],",
        "    pub target: [f64; 3],",
        "    pub fov: f64,",
        "}",
        "",
        "impl Default for RemodelUiCamera {",
        "    fn default() -> Self {",
        "        Self { position: [4.0, -4.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 }",
        "    }",
        "}",
        "",
        "/// 🖱️ Artifact-owned selection (mirror of app config selection).",
        "#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]",
        "#[serde(rename_all = \"camelCase\", default)]",
        "pub struct RemodelUiSelection {",
        "    pub mode: String,",
        "    pub ids: Vec<String>,",
        "}",
        "",
        "/// 👁️ Artifact-owned layer visibility (mirror of app config layers).",
        "#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]",
        "#[serde(rename_all = \"camelCase\", default)]",
        "pub struct RemodelUiLayers {",
        "    pub mesh: bool,",
        "    pub dense: bool,",
        "    pub sparse: bool,",
        "    pub cameras: bool,",
        "    pub gcps: bool,",
        "}",
        "",
        "impl Default for RemodelUiLayers {",
        "    fn default() -> Self {",
        "        Self { mesh: true, dense: true, sparse: true, cameras: true, gcps: true }",
        "    }",
        "}",
        "",
        "/// 🎞️ Artifact-owned frame cursor (mirror of app config frame cursor).",
        "#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]",
        "#[serde(rename_all = \"camelCase\", default)]",
        "pub struct RemodelUiFrameCursor {",
        "    pub stream_id: Option<String>,",
        "    pub frame_index: u32,",
        "}",
        "//#endregion 🔖️UiHelpers",
        "",
        "//#region 🔖️Artifact",
        "/// 🧬️ Full remodel artifact state across persistent, shared-ui and local-ui classes.",
        "#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]",
        "#[serde(rename_all = \"camelCase\")]",
        f'#[artifact_schema(id = "{SCHEMA_ID}")]',
        "pub struct RemodelArtifact {",
    ]
    for camel, snake, state, rust_ty, _opt, _card, _scalar in ARTIFACT_FIELDS:
        lines.append(f"    #[state({state_rust(state)})] pub {snake}: {rust_ty},")
    lines += [
        "}",
        "//#endregion 🔖️Artifact",
        "",
        "//#region 🔖️Conversions",
        "impl Default for RemodelArtifact {",
        "    fn default() -> Self {",
        "        Self::from_snapshot(RemodelSnapshot::default())",
        "    }",
        "}",
        "",
        "impl RemodelArtifact {",
        "    /// 📸️ Persisted subset.",
        "    pub fn to_snapshot(&self) -> RemodelSnapshot {",
        "        RemodelSnapshot {",
        "            schema: self.schema.clone(),",
        "            id: self.id.clone(),",
        "            streams: self.streams.clone(),",
        "            assets: self.assets.clone(),",
        "            calibration: self.calibration.clone(),",
        "            params: self.params.clone(),",
        "            gcps: self.gcps.clone(),",
        "            job: self.job.clone(),",
        "            results: self.results.clone(),",
        "        }",
        "    }",
        "",
        "    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.",
        "    pub fn from_snapshot(snapshot: RemodelSnapshot) -> Self {",
        "        Self {",
        "            schema: snapshot.schema,",
        "            id: snapshot.id,",
        "            streams: snapshot.streams,",
        "            assets: snapshot.assets,",
        "            calibration: snapshot.calibration,",
        "            params: snapshot.params,",
        "            gcps: snapshot.gcps,",
        "            job: snapshot.job,",
        "            results: snapshot.results,",
        "            selection: RemodelUiSelection::default(),",
        "            active_utility_id: \"select\".into(),",
        "            report_table: \"frames\".into(),",
        "            frame_cursor: RemodelUiFrameCursor::default(),",
        "            camera: RemodelUiCamera::default(),",
        "            layers: RemodelUiLayers::default(),",
        "            locale: \"en-US\".into(),",
        "        }",
        "    }",
        "",
        "    /// 🔄 Writes persistent fields from a snapshot into this artifact.",
        "    pub fn set_snapshot(&mut self, snapshot: RemodelSnapshot) {",
        "        self.schema = snapshot.schema;",
        "        self.id = snapshot.id;",
        "        self.streams = snapshot.streams;",
        "        self.assets = snapshot.assets;",
        "        self.calibration = snapshot.calibration;",
        "        self.params = snapshot.params;",
        "        self.gcps = snapshot.gcps;",
        "        self.job = snapshot.job;",
        "        self.results = snapshot.results;",
        "    }",
        "}",
        "//#endregion 🔖️Conversions",
        "",
        "//#region 🔖️Descriptor",
        "/// 🧬️ Descriptor for `s.remodel.remodel` — fifteen handcrafted schema leaves.",
        "pub fn remodel_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {",
        "    schema::ArtifactSchemaDescriptor {",
        f'        id: "{SCHEMA_ID}",',
        "        artifact: schema::FacetLeaves {",
        '            rust: include_str!("🦀️component.rs"),',
        '            typescript: include_str!("🟦️component.ts"),',
        '            graphql: include_str!("🔗️component.graphql"),',
        '            json_schema: include_str!("🔣️component.json"),',
        '            proto: include_str!("🛰️component.proto"),',
        "        },",
        "        snapshot: schema::FacetLeaves {",
        '            rust: include_str!("../📸️snapshot/🧬️schema/🦀️component.rs"),',
        '            typescript: include_str!("../📸️snapshot/🧬️schema/🟦️component.ts"),',
        '            graphql: include_str!("../📸️snapshot/🧬️schema/🔗️component.graphql"),',
        '            json_schema: include_str!("../📸️snapshot/🧬️schema/🔣️component.json"),',
        '            proto: include_str!("../📸️snapshot/🧬️schema/🛰️component.proto"),',
        "        },",
        "        diff: schema::FacetLeaves {",
        '            rust: include_str!("../🔺️diff/🧬️schema/🦀️component.rs"),',
        '            typescript: include_str!("../🔺️diff/🧬️schema/🟦️component.ts"),',
        '            graphql: include_str!("../🔺️diff/🧬️schema/🔗️component.graphql"),',
        '            json_schema: include_str!("../🔺️diff/🧬️schema/🔣️component.json"),',
        '            proto: include_str!("../🔺️diff/🧬️schema/🛰️component.proto"),',
        "        },",
        "    }",
        "}",
        "//#endregion 🔖️Descriptor",
    ]
    return "\n".join(lines) + "\n"


def gen_snapshot_rust() -> str:
    return "\n".join(
        [
            "//! 🧬️ Remodel snapshot schema — persistent fields only.",
            "",
            "use crate::artifacts::remodel::{",
            "    CalibrationState, GroundControlPoint, ImageAsset, MediaStream, ReconstructionJob,",
            "    ReconstructionParams, ReconstructionResults, REMODEL_DOCUMENT_SCHEMA,",
            "};",
            "use schema::ArtifactSchema;",
            "use serde::{Deserialize, Serialize};",
            "use std::collections::BTreeMap;",
            "",
            "//#region 🔖️Snapshot",
            "/// 📸️ Persisted remodel document snapshot (persistent fields of the artifact).",
            "#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]",
            "#[serde(rename_all = \"camelCase\")]",
            '#[dsl(extension = "remodel")]',
            f'#[artifact_schema(id = "{SCHEMA_ID}")]',
            "pub struct RemodelSnapshot {",
            "    #[state(persistent)]",
            "    pub schema: String,",
            "    #[state(persistent)]",
            "    pub id: String,",
            "    #[serde(default)]",
            "    #[dsl(table)]",
            "    #[state(persistent)]",
            "    pub streams: Vec<MediaStream>,",
            "    #[serde(default)]",
            "    #[state(persistent)]",
            "    pub assets: BTreeMap<String, ImageAsset>,",
            "    #[serde(default)]",
            "    #[dsl(block)]",
            "    #[state(persistent)]",
            "    pub calibration: CalibrationState,",
            "    #[serde(default)]",
            "    #[dsl(block)]",
            "    #[state(persistent)]",
            "    pub params: ReconstructionParams,",
            "    #[serde(default)]",
            "    #[dsl(table)]",
            "    #[state(persistent)]",
            "    pub gcps: Vec<GroundControlPoint>,",
            "    #[serde(default)]",
            "    #[dsl(block)]",
            "    #[state(persistent)]",
            "    pub job: ReconstructionJob,",
            "    #[serde(default)]",
            "    #[dsl(block)]",
            "    #[state(persistent)]",
            "    pub results: ReconstructionResults,",
            "}",
            "//#region 🔖️HandcraftedDocumentCodecs",
            "/// ✉️ P6 handcrafted DocumentDsl/DocumentPack (derive no longer emits these traits).",
            "impl store::DocumentDsl for RemodelSnapshot {",
            '    const EXTENSION: &\'static str = "remodel";',
            '    fn envelope_id() -> &\'static str { "remodel.remodel" }',
            "    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {",
            "        let body = match store::semio_format::split_text_preamble(text) {",
            "            Ok((_, rest)) => rest,",
            "            Err(_) => text,",
            "        };",
            "        let record = dsl::parse(",
            "            body,",
            "            &Self::__dsl_spec(),",
            "            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },",
            "        )?;",
            "        Self::__dsl_from_record(&record)",
            "    }",
            "    fn print_dsl(&self) -> String {",
            "        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);",
            "        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(",
            "            <Self as store::DocumentDsl>::envelope_id(),",
            "            store::semio_format::Component::Dsl,",
            "            1,",
            '        ).expect("valid envelope_id");',
            "        store::semio_format::wrap_text(&envelope, &body)",
            "    }",
            "}",
            "",
            "impl store::DocumentPack for RemodelSnapshot {",
            "    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {",
            "        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;",
            "        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(",
            "            <Self as store::DocumentDsl>::envelope_id(),",
            "            store::semio_format::Component::Pack,",
            "            1,",
            "        ).map_err(|e| store::PackError::Schema(e.to_string()))?;",
            "        Ok(store::semio_format::wrap_binary(&envelope, &inner))",
            "    }",
            "    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {",
            "        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)",
            "            .map_err(|e| store::PackError::Schema(e.to_string()))?;",
            "        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {",
            "            return Err(store::PackError::Schema(format!(",
            '                "pack envelope mismatch: expected {}, got {}",',
            "                <Self as store::DocumentDsl>::envelope_id(),",
            "                envelope.envelope_id()",
            "            )));",
            "        }",
            "        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;",
            "        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)",
            "    }",
            "    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }",
            "}",
            "//#endregion 🔖️HandcraftedDocumentCodecs",
            "",
            "impl Default for RemodelSnapshot {",
            "    fn default() -> Self {",
            "        Self {",
            "            schema: REMODEL_DOCUMENT_SCHEMA.into(),",
            '            id: "remodel".into(),',
            "            streams: Vec::new(),",
            "            assets: BTreeMap::new(),",
            "            calibration: CalibrationState::default(),",
            "            params: ReconstructionParams::default(),",
            "            gcps: Vec::new(),",
            "            job: ReconstructionJob::default(),",
            "            results: ReconstructionResults::default(),",
            "        }",
            "    }",
            "}",
            "//#endregion 🔖️Snapshot",
        ]
    ) + "\n"


def gen_diff_rust_schema() -> str:
    lines = [
        "//! 🧬️ Remodel diff schema — sparse field delta over the artifact.",
        "",
        "use crate::artifacts::remodel::schema::{",
        "    RemodelArtifact, RemodelUiCamera, RemodelUiFrameCursor, RemodelUiLayers, RemodelUiSelection,",
        "};",
        "use crate::artifacts::remodel::{",
        "    CalibrationState, GroundControlPoint, ImageAsset, MediaStream, ReconstructionJob,",
        "    ReconstructionParams, ReconstructionResults,",
        "};",
        "use schema::ArtifactSchema;",
        "use serde::{Deserialize, Serialize};",
        "use std::collections::BTreeMap;",
        "",
        "//#region 🔖️Diff",
        "/// 🔺️ Sparse field delta for the remodel artifact; persistent entries apply via MutationDiff.",
        "#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]",
        "#[serde(rename_all = \"camelCase\", default)]",
        f'#[artifact_schema(id = "{SCHEMA_ID}")]',
        "pub struct RemodelDiff {",
        "    #[state(persistent)] pub artifact: Option<Box<RemodelArtifact>>,",
        "    #[state(persistent)] pub schema: Option<String>,",
        "    #[state(persistent)] pub id: Option<String>,",
        "    #[state(persistent)] pub streams: Option<RemodelMediaStreamList>,",
        "    #[state(persistent)] pub assets: Option<BTreeMap<String, ImageAsset>>,",
        "    #[state(persistent)] pub calibration: Option<CalibrationState>,",
        "    #[state(persistent)] pub params: Option<ReconstructionParams>,",
        "    #[state(persistent)] pub gcps: Option<RemodelGcpList>,",
        "    #[state(persistent)] pub job: Option<ReconstructionJob>,",
        "    #[state(persistent)] pub results: Option<ReconstructionResults>,",
        "    #[state(shared_ui)] pub selection: Option<RemodelUiSelection>,",
        "    #[state(shared_ui)] pub active_utility_id: Option<String>,",
        "    #[state(shared_ui)] pub report_table: Option<String>,",
        "    #[state(shared_ui)] pub frame_cursor: Option<RemodelUiFrameCursor>,",
        "    #[state(local_ui)] pub camera: Option<RemodelUiCamera>,",
        "    #[state(local_ui)] pub layers: Option<RemodelUiLayers>,",
        "    #[state(local_ui)] pub locale: Option<String>,",
        "}",
        "//#endregion 🔖️Diff",
        "",
        "//#region 🔖️DeltaHelpers",
        "/// 📋 Media-stream list wrapper so optional list diffs stay scalar across formats.",
        "#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]",
        "#[serde(rename_all = \"camelCase\", default)]",
        "pub struct RemodelMediaStreamList {",
        "    pub values: Vec<MediaStream>,",
        "}",
        "",
        "/// 📋 GCP list wrapper so optional list diffs stay scalar across formats.",
        "#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]",
        "#[serde(rename_all = \"camelCase\", default)]",
        "pub struct RemodelGcpList {",
        "    pub values: Vec<GroundControlPoint>,",
        "}",
        "//#endregion 🔖️DeltaHelpers",
    ]
    return "\n".join(lines) + "\n"


def ts_type(rust_ty: str, card: str, scalar: str) -> str:
    if card == "list":
        return f"{scalar}[]"
    if card == "map":
        return f"Record<string, {scalar}>"
    table = {
        "String": "string",
        "string": "string",
        "bool": "boolean",
        "f64": "number",
        "f32": "number",
        "u32": "number",
        "i32": "number",
        "i64": "number",
    }
    return table.get(rust_ty, rust_ty)


def gen_ts(facet: str, type_name: str, fields, extra_ifaces: str = "") -> str:
    lines = [f"/** 🧬️ Remodel {facet} schema — TypeScript mirror of the normative JSON Schema. */", ""]
    if extra_ifaces:
        lines.append(extra_ifaces.rstrip())
        lines.append("")
    lines.append(f"export interface {type_name} {{")
    for camel, _snake, state, rust_ty, optional, card, scalar in fields:
        opt = "?" if optional else ""
        lines.append(f"  /** @state {state} */")
        lines.append(f"  {camel}{opt}: {ts_type(rust_ty, card, scalar)};")
    lines.append("}")
    return "\n".join(lines) + "\n"


UI_IFACES = """\
export interface RemodelUiCamera {
  position: [number, number, number];
  target: [number, number, number];
  fov: number;
}

export interface RemodelUiSelection {
  mode: string;
  ids: string[];
}

export interface RemodelUiLayers {
  mesh: boolean;
  dense: boolean;
  sparse: boolean;
  cameras: boolean;
  gcps: boolean;
}

export interface RemodelUiFrameCursor {
  streamId?: string;
  frameIndex: number;
}

export interface MediaStream { [key: string]: unknown }
export interface ImageAsset { [key: string]: unknown }
export interface CalibrationState { [key: string]: unknown }
export interface ReconstructionParams { [key: string]: unknown }
export interface GroundControlPoint { [key: string]: unknown }
export interface ReconstructionJob { [key: string]: unknown }
export interface ReconstructionResults { [key: string]: unknown }
"""


def gen_graphql(facet: str, type_name: str, fields, extra_types: str = "") -> str:
    lines = [
        f"# 🧬️ Remodel {facet} schema — every field with its state class.",
        "",
    ]
    if extra_types:
        lines.append(extra_types.rstrip())
        lines.append("")
    lines.append(f"type {type_name} {{")
    for camel, _snake, state, rust_ty, optional, card, scalar in fields:
        bang = "" if optional else "!"
        if card == "list":
            ty = f"[{scalar}!]!"
            if optional:
                ty = f"[{scalar}!]"
        elif card == "map":
            # GraphQL maps as entry list — use generated entry type name
            entry = f"{type_name}{camel[0].upper()+camel[1:]}Entry"
            ty = f"[{entry}!]!"
            if optional:
                ty = f"[{entry}!]"
        else:
            gql = {
                "string": "String",
                "bool": "Boolean",
                "int32": "Int",
                "uint32": "Int",
                "int64": "Int",
                "float32": "Float",
                "float64": "Float",
            }.get(scalar, scalar)
            ty = f"{gql}{bang}"
        lines.append(f"  {camel}: {ty} @state(class: {state_gql(state)})")
    lines.append("}")
    return "\n".join(lines) + "\n"


GQL_HELPERS = """\
scalar RemodelJson

type RemodelUiCamera {
  position: [Float!]!
  target: [Float!]!
  fov: Float!
}

type RemodelUiSelection {
  mode: String!
  ids: [String!]!
}

type RemodelUiLayers {
  mesh: Boolean!
  dense: Boolean!
  sparse: Boolean!
  cameras: Boolean!
  gcps: Boolean!
}

type RemodelUiFrameCursor {
  streamId: String
  frameIndex: Int!
}

type MediaStream { id: String! }
type ImageAsset { mime: String! }
type CalibrationState { _placeholder: Boolean }
type ReconstructionParams { _placeholder: Boolean }
type GroundControlPoint { id: String! }
type ReconstructionJob { _placeholder: Boolean }
type ReconstructionResults { _placeholder: Boolean }

type RemodelArtifactAssetsEntry {
  key: String!
  value: ImageAsset!
}

type RemodelSnapshotAssetsEntry {
  key: String!
  value: ImageAsset!
}
"""


def gen_json(facet: str, type_name: str, fields) -> str:
    import json

    required = [f[0] for f in fields if not f[4]]
    props = {}
    for camel, _snake, state, rust_ty, optional, card, scalar in fields:
        if card == "list":
            prop = {"type": "array", "items": {"$ref": f"#/$defs/{scalar}"}, "x-semio-state": state}
        elif card == "map":
            prop = {
                "type": "object",
                "additionalProperties": {"$ref": f"#/$defs/{scalar}"},
                "x-semio-state": state,
            }
        elif scalar == "string":
            prop = {"type": "string", "x-semio-state": state}
        elif scalar == "bool":
            prop = {"type": "boolean", "x-semio-state": state}
        else:
            prop = {"$ref": f"#/$defs/{scalar}", "x-semio-state": state}
        props[camel] = prop

    defs = {
        "MediaStream": {"type": "object", "title": "MediaStream"},
        "ImageAsset": {"type": "object", "title": "ImageAsset"},
        "CalibrationState": {"type": "object", "title": "CalibrationState"},
        "ReconstructionParams": {"type": "object", "title": "ReconstructionParams"},
        "GroundControlPoint": {"type": "object", "title": "GroundControlPoint"},
        "ReconstructionJob": {"type": "object", "title": "ReconstructionJob"},
        "ReconstructionResults": {"type": "object", "title": "ReconstructionResults"},
        "RemodelUiCamera": {
            "type": "object",
            "title": "RemodelUiCamera",
            "required": ["position", "target", "fov"],
            "properties": {
                "position": {"type": "array", "items": {"type": "number", "format": "double"}, "minItems": 3, "maxItems": 3},
                "target": {"type": "array", "items": {"type": "number", "format": "double"}, "minItems": 3, "maxItems": 3},
                "fov": {"type": "number", "format": "double"},
            },
        },
        "RemodelUiSelection": {
            "type": "object",
            "title": "RemodelUiSelection",
            "required": ["mode", "ids"],
            "properties": {
                "mode": {"type": "string"},
                "ids": {"type": "array", "items": {"type": "string"}},
            },
        },
        "RemodelUiLayers": {
            "type": "object",
            "title": "RemodelUiLayers",
            "required": ["mesh", "dense", "sparse", "cameras", "gcps"],
            "properties": {
                "mesh": {"type": "boolean"},
                "dense": {"type": "boolean"},
                "sparse": {"type": "boolean"},
                "cameras": {"type": "boolean"},
                "gcps": {"type": "boolean"},
            },
        },
        "RemodelUiFrameCursor": {
            "type": "object",
            "title": "RemodelUiFrameCursor",
            "required": ["frameIndex"],
            "properties": {
                "streamId": {"type": "string"},
                "frameIndex": {"type": "integer", "format": "uint32", "minimum": 0},
            },
        },
        "RemodelMediaStreamList": {
            "type": "object",
            "title": "RemodelMediaStreamList",
            "required": ["values"],
            "properties": {"values": {"type": "array", "items": {"$ref": "#/$defs/MediaStream"}}},
        },
        "RemodelGcpList": {
            "type": "object",
            "title": "RemodelGcpList",
            "required": ["values"],
            "properties": {"values": {"type": "array", "items": {"$ref": "#/$defs/GroundControlPoint"}}},
        },
        "RemodelArtifact": {"type": "object", "title": "RemodelArtifact"},
    }

    doc = {
        "$id": f"https://semio.tech/schema/s/{PLUGIN_KEY}/{ARTIFACT_KEY}/{facet}.json",
        "title": type_name,
        "type": "object",
        "additionalProperties": False,
        "required": required,
        "properties": props,
        "$defs": defs,
    }
    return json.dumps(doc, indent=2) + "\n"


def gen_proto(facet: str, type_name: str, fields) -> str:
    lines = [
        'syntax = "proto3";',
        f"package semio.s.{PLUGIN_KEY}.{ARTIFACT_KEY}.{facet};",
        "",
        f"// 🧬️ Remodel {facet} schema.",
        "",
    ]
    # helper messages first (but FIRST type must be XArtifact/XSnapshot/XDiff)
    # So put main message first, helpers after.
    lines.append(f"message {type_name} {{")
    n = 1
    for camel, snake, state, rust_ty, optional, card, scalar in fields:
        lines.append(f"  // @state {state}")
        if card == "list":
            lines.append(f"  repeated {scalar} {snake} = {n};")
        elif card == "map":
            lines.append(f"  map<string, {scalar}> {snake} = {n};")
        else:
            proto_ty = {
                "string": "string",
                "bool": "bool",
                "int32": "int32",
                "uint32": "uint32",
                "int64": "int64",
                "float32": "float",
                "float64": "double",
            }.get(scalar, scalar)
            opt = "optional " if optional else ""
            lines.append(f"  {opt}{proto_ty} {snake} = {n};")
        n += 1
    lines.append("}")
    lines.append("")
    # stubs for nested
    for stub in [
        "MediaStream",
        "ImageAsset",
        "CalibrationState",
        "ReconstructionParams",
        "GroundControlPoint",
        "ReconstructionJob",
        "ReconstructionResults",
        "RemodelUiCamera",
        "RemodelUiSelection",
        "RemodelUiLayers",
        "RemodelUiFrameCursor",
        "RemodelMediaStreamList",
        "RemodelGcpList",
        "RemodelArtifact",
    ]:
        if stub == type_name:
            continue
        lines.append(f"message {stub} {{")
        lines.append("  string _placeholder = 1;")
        lines.append("}")
        lines.append("")
    return "\n".join(lines) + "\n"


def gen_diff_fields_for_leaves():
    """Diff leaf top-level fields for parity scanners."""
    fields = [
        ("artifact", "artifact", "persistent", "Option<Box<RemodelArtifact>>", True, "scalar", "RemodelArtifact"),
        ("schema", "schema", "persistent", "Option<String>", True, "scalar", "string"),
        ("id", "id", "persistent", "Option<String>", True, "scalar", "string"),
        ("streams", "streams", "persistent", "Option<RemodelMediaStreamList>", True, "scalar", "RemodelMediaStreamList"),
        ("assets", "assets", "persistent", "Option<BTreeMap<String, ImageAsset>>", True, "map", "ImageAsset"),
        ("calibration", "calibration", "persistent", "Option<CalibrationState>", True, "scalar", "CalibrationState"),
        ("params", "params", "persistent", "Option<ReconstructionParams>", True, "scalar", "ReconstructionParams"),
        ("gcps", "gcps", "persistent", "Option<RemodelGcpList>", True, "scalar", "RemodelGcpList"),
        ("job", "job", "persistent", "Option<ReconstructionJob>", True, "scalar", "ReconstructionJob"),
        ("results", "results", "persistent", "Option<ReconstructionResults>", True, "scalar", "ReconstructionResults"),
        ("selection", "selection", "shared-ui", "Option<RemodelUiSelection>", True, "scalar", "RemodelUiSelection"),
        ("activeUtilityId", "active_utility_id", "shared-ui", "Option<String>", True, "scalar", "string"),
        ("reportTable", "report_table", "shared-ui", "Option<String>", True, "scalar", "string"),
        ("frameCursor", "frame_cursor", "shared-ui", "Option<RemodelUiFrameCursor>", True, "scalar", "RemodelUiFrameCursor"),
        ("camera", "camera", "local-ui", "Option<RemodelUiCamera>", True, "scalar", "RemodelUiCamera"),
        ("layers", "layers", "local-ui", "Option<RemodelUiLayers>", True, "scalar", "RemodelUiLayers"),
        ("locale", "locale", "local-ui", "Option<String>", True, "scalar", "string"),
    ]
    return fields


def gen_diff_ts() -> str:
    fields = gen_diff_fields_for_leaves()
    extra = UI_IFACES + """
export interface RemodelArtifact { [key: string]: unknown }
export interface RemodelMediaStreamList { values: MediaStream[] }
export interface RemodelGcpList { values: GroundControlPoint[] }
"""
    lines = [
        "/** 🧬️ Remodel diff schema — TypeScript mirror of the normative JSON Schema. */",
        "",
        extra.rstrip(),
        "",
        "export interface RemodelDiff {",
    ]
    for camel, _snake, state, rust_ty, optional, card, scalar in fields:
        if card == "map":
            ty = f"Record<string, {scalar}>"
        else:
            ty = ts_type(rust_ty.replace("Option<", "").replace(">", "").replace("Box<", ""), "scalar", scalar)
            # strip Option already handled by optional mark
            if rust_ty.startswith("Option<"):
                inner = rust_ty[len("Option<") : -1]
                if inner.startswith("Box<"):
                    ty = scalar
                elif card == "map":
                    ty = f"Record<string, {scalar}>"
                else:
                    ty = scalar if scalar not in ("string", "bool") else ts_type(inner, "scalar", scalar)
                    if scalar == "string":
                        ty = "string"
                    elif scalar == "bool":
                        ty = "boolean"
                    else:
                        ty = scalar
        lines.append(f"  /** @state {state} */")
        lines.append(f"  {camel}?: {ty};")
    lines.append("}")
    return "\n".join(lines) + "\n"


def gen_diff_graphql() -> str:
    fields = gen_diff_fields_for_leaves()
    extra = GQL_HELPERS + """
type RemodelMediaStreamList {
  values: [MediaStream!]!
}

type RemodelGcpList {
  values: [GroundControlPoint!]!
}

type RemodelDiffAssetsEntry {
  key: String!
  value: ImageAsset!
}
"""
    lines = [
        "# 🧬️ Remodel diff schema — sparse field delta.",
        "",
        extra.rstrip(),
        "",
        "type RemodelDiff {",
    ]
    for camel, _snake, state, _rust_ty, _optional, card, scalar in fields:
        if card == "map":
            ty = "[RemodelDiffAssetsEntry!]"
        else:
            gql = {
                "string": "String",
                "bool": "Boolean",
            }.get(scalar, scalar)
            ty = gql  # all optional in diff
        lines.append(f"  {camel}: {ty} @state(class: {state_gql(state)})")
    lines.append("}")
    return "\n".join(lines) + "\n"


def gen_diff_json() -> str:
    import json

    fields = gen_diff_fields_for_leaves()
    props = {}
    for camel, _snake, state, _rust_ty, _optional, card, scalar in fields:
        if card == "map":
            prop = {
                "type": "object",
                "additionalProperties": {"$ref": f"#/$defs/{scalar}"},
                "x-semio-state": state,
            }
        elif scalar == "string":
            prop = {"type": "string", "x-semio-state": state}
        else:
            prop = {"$ref": f"#/$defs/{scalar}", "x-semio-state": state}
        props[camel] = prop
    doc = {
        "$id": f"https://semio.tech/schema/s/{PLUGIN_KEY}/{ARTIFACT_KEY}/diff.json",
        "title": "RemodelDiff",
        "type": "object",
        "additionalProperties": False,
        "required": [],
        "properties": props,
        "$defs": {
            "RemodelArtifact": {"type": "object", "title": "RemodelArtifact"},
            "MediaStream": {"type": "object", "title": "MediaStream"},
            "ImageAsset": {"type": "object", "title": "ImageAsset"},
            "CalibrationState": {"type": "object", "title": "CalibrationState"},
            "ReconstructionParams": {"type": "object", "title": "ReconstructionParams"},
            "GroundControlPoint": {"type": "object", "title": "GroundControlPoint"},
            "ReconstructionJob": {"type": "object", "title": "ReconstructionJob"},
            "ReconstructionResults": {"type": "object", "title": "ReconstructionResults"},
            "RemodelMediaStreamList": {
                "type": "object",
                "title": "RemodelMediaStreamList",
                "required": ["values"],
                "properties": {"values": {"type": "array", "items": {"$ref": "#/$defs/MediaStream"}}},
            },
            "RemodelGcpList": {
                "type": "object",
                "title": "RemodelGcpList",
                "required": ["values"],
                "properties": {"values": {"type": "array", "items": {"$ref": "#/$defs/GroundControlPoint"}}},
            },
            "RemodelUiCamera": {"type": "object", "title": "RemodelUiCamera"},
            "RemodelUiSelection": {"type": "object", "title": "RemodelUiSelection"},
            "RemodelUiLayers": {"type": "object", "title": "RemodelUiLayers"},
            "RemodelUiFrameCursor": {"type": "object", "title": "RemodelUiFrameCursor"},
        },
    }
    return json.dumps(doc, indent=2) + "\n"


def gen_diff_proto() -> str:
    fields = gen_diff_fields_for_leaves()
    lines = [
        'syntax = "proto3";',
        f"package semio.s.{PLUGIN_KEY}.{ARTIFACT_KEY}.diff;",
        "",
        "// 🧬️ Remodel diff schema.",
        "",
        "message RemodelDiff {",
    ]
    n = 1
    for camel, snake, state, _rust_ty, _optional, card, scalar in fields:
        lines.append(f"  // @state {state}")
        if card == "map":
            lines.append(f"  map<string, {scalar}> {snake} = {n};")
        else:
            proto_ty = {"string": "string", "bool": "bool"}.get(scalar, scalar)
            lines.append(f"  optional {proto_ty} {snake} = {n};")
        n += 1
    lines.append("}")
    lines.append("")
    for stub in [
        "RemodelArtifact",
        "MediaStream",
        "ImageAsset",
        "CalibrationState",
        "ReconstructionParams",
        "GroundControlPoint",
        "ReconstructionJob",
        "ReconstructionResults",
        "RemodelMediaStreamList",
        "RemodelGcpList",
        "RemodelUiCamera",
        "RemodelUiSelection",
        "RemodelUiLayers",
        "RemodelUiFrameCursor",
    ]:
        lines.append(f"message {stub} {{ string _placeholder = 1; }}")
    return "\n".join(lines) + "\n"


def write_leaves() -> None:
    # artifact
    write(ART / "🧬️schema" / "🦀️component.rs", gen_artifact_rust())
    write(
        ART / "🧬️schema" / "🟦️component.ts",
        gen_ts("artifact", "RemodelArtifact", ARTIFACT_FIELDS, UI_IFACES),
    )
    write(
        ART / "🧬️schema" / "🔗️component.graphql",
        gen_graphql(
            "artifact",
            "RemodelArtifact",
            ARTIFACT_FIELDS,
            GQL_HELPERS,
        ),
    )
    write(ART / "🧬️schema" / "🔣️component.json", gen_json("artifact", "RemodelArtifact", ARTIFACT_FIELDS))
    write(ART / "🧬️schema" / "🛰️component.proto", gen_proto("artifact", "RemodelArtifact", ARTIFACT_FIELDS))

    # snapshot
    write(ART / "📸️snapshot" / "🧬️schema" / "🦀️component.rs", gen_snapshot_rust())
    write(
        ART / "📸️snapshot" / "🧬️schema" / "🟦️component.ts",
        gen_ts("snapshot", "RemodelSnapshot", SNAPSHOT_FIELDS, UI_IFACES),
    )
    write(
        ART / "📸️snapshot" / "🧬️schema" / "🔗️component.graphql",
        gen_graphql("snapshot", "RemodelSnapshot", SNAPSHOT_FIELDS, GQL_HELPERS),
    )
    write(
        ART / "📸️snapshot" / "🧬️schema" / "🔣️component.json",
        gen_json("snapshot", "RemodelSnapshot", SNAPSHOT_FIELDS),
    )
    write(
        ART / "📸️snapshot" / "🧬️schema" / "🛰️component.proto",
        gen_proto("snapshot", "RemodelSnapshot", SNAPSHOT_FIELDS),
    )

    # diff schema
    write(ART / "🔺️diff" / "🧬️schema" / "🦀️component.rs", gen_diff_rust_schema())
    write(ART / "🔺️diff" / "🧬️schema" / "🟦️component.ts", gen_diff_ts())
    write(ART / "🔺️diff" / "🧬️schema" / "🔗️component.graphql", gen_diff_graphql())
    write(ART / "🔺️diff" / "🧬️schema" / "🔣️component.json", gen_diff_json())
    write(ART / "🔺️diff" / "🧬️schema" / "🛰️component.proto", gen_diff_proto())


def patch_root_component() -> None:
    path = ART / "🦀️component.rs"
    text = path.read_text()
    # Remove RemodelProjection struct + Document codecs; re-export RemodelSnapshot
    # Find the RemodelProjection struct block through HandcraftedDocumentCodecs end
    pattern = re.compile(
        r"/// 🗂️ Top-level remodel project document.*?//#endregion 🔖️HandcraftedDocumentCodecs\n+",
        re.S,
    )
    replacement = (
        "/// 📸️ Persisted remodel snapshot — re-exported from `📸️snapshot/🧬️schema`.\n"
        "pub use crate::artifacts::remodel::snapshot::schema::RemodelSnapshot;\n\n"
    )
    new, n = pattern.subn(replacement, text, count=1)
    if n != 1:
        raise SystemExit(f"failed to replace RemodelProjection block (n={n})")
    new = new.replace("RemodelProjection", "RemodelSnapshot")
    # Fix references in docs that said RemodelProjection::assets etc - already replaced
    path.write_text(new)
    print("patched root component.rs")


def bulk_rename_projection() -> None:
    for path in ROOT.rglob("*"):
        if not path.is_file():
            continue
        if path.suffix not in {".rs", ".ts", ".semio", ".json", ".graphql", ".proto", ".md"}:
            continue
        # skip our generator
        text = path.read_text(errors="ignore")
        orig = text
        text = text.replace("RemodelProjection", "RemodelSnapshot")
        text = text.replace("type Projection =", "type Snapshot =")
        text = text.replace("initial_projection", "initial_snapshot")
        text = text.replace("fn projection(", "fn snapshot(")
        text = text.replace("into_projection", "into_snapshot")
        # DocumentView / ConfigView field access — careful not to rename local vars named projection
        text = text.replace("doc.projection", "doc.snapshot")
        text = text.replace("cfg.projection", "cfg.snapshot")
        text = text.replace("store.projection()", "store.snapshot()")
        text = text.replace("store::test_support::", "store::os_store::test_support::")
        if text != orig:
            path.write_text(text)
            print("renamed in", path.relative_to(ROOT))


def main() -> None:
    write_leaves()
    patch_root_component()
    bulk_rename_projection()
    print("TICKET", TICKET)
    print("done gen")


if __name__ == "__main__":
    main()
