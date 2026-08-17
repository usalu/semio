//! 🧬️ GIS map artifact schema — every field of the artifact with its state class.

use crate::artifacts::gismap::dsl::REUSE_MAP_EXAMPLE_TEXT;
use crate::artifacts::gismap::mutations::{create_position, create_region, create_route, delete_position, delete_region, delete_route, replace_position_data, replace_region_data, replace_route_data};
use crate::artifacts::gismap::op::GisMapMutation;
use crate::artifacts::gismap::{gis_map_snapshot_with_derived_children, GisMapImageChild, GisMapSnapshot, MapFeature};
use semio_framework_plugin::{ArtifactSerializer, ErasedComposeSource, IoDirection, IoKey, IoPayload, io_dispatch};
use semio_s_plugin_stdio::artifacts::dwg::{DwgDrawing, DwgGeometry};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint2, SemioRgba, SemioTransform};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::io::export::serializers::artifacts::svg::v1_1::any::SemioDrawingToSvg;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot};
use semio_s_plugin_stdio::artifacts::svg::SvgSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashSet};

//#region 🔹Artifact
/// 🧬️ Full GIS map artifact state across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.gis.gismap")]
pub struct GisMapArtifact {
    #[state(artifact)] pub positions: Vec<MapFeature>,
    #[state(artifact)] pub routes: Vec<MapFeature>,
    #[state(artifact)] pub regions: Vec<MapFeature>,
    /// 🕸️ Mirrors `GisMapSnapshot.image` — see that field's own doc comment and
    /// `crate::artifacts::gismap::🦀️component.rs`'s `🔖️Composition` region. Carried verbatim (never
    /// derived) since, unlike `drawing`/`value`, nothing in this plugin can rebuild it from
    /// `positions`/`routes`/`regions` — dropping it silently on `from_snapshot`/`to_snapshot` would
    /// be a real, undocumented data loss the moment a future basemap-capture path populates it.
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.image")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<GisMapImageChild>,
    #[state(presence)] pub layer_visibility: BTreeMap<String, bool>,
    #[state(presence)] pub layer_stroke_scale: BTreeMap<String, f64>,
    #[state(config)] pub camera_json: String,
    #[state(config)] pub render_mode: String,
    #[state(config)] pub vector_style: String,
    #[state(config)] pub lod_mode: String,
    #[state(config)] pub locale: String,
}
//#endregion 🔹Artifact

//#region 🔹Conversions
impl Default for GisMapArtifact {
    fn default() -> Self {
        Self {
            positions: Vec::new(),
            routes: Vec::new(),
            regions: Vec::new(),
            image: None,
            layer_visibility: BTreeMap::new(),
            layer_stroke_scale: BTreeMap::new(),
            camera_json: r#"{"x":0,"y":0,"zoom":1}"#.into(),
            render_mode: "combined".into(),
            vector_style: "colored".into(),
            lod_mode: "automatic".into(),
            locale: "en-US".into(),
        }
    }
}

impl GisMapArtifact {
    /// 📸️ Persisted subset. `drawing`/`value` are always re-derived (never carried verbatim off
    /// `self`) via `gis_map_snapshot_with_derived_children` so they can never drift from what
    /// `positions`/`routes`/`regions` actually contain; `image` carries straight through (real, but
    /// not derivable from anything this plugin owns — see the field's own doc comment).
    pub fn to_snapshot(&self) -> crate::artifacts::gismap::GisMapSnapshot {
        gis_map_snapshot_with_derived_children(crate::artifacts::gismap::GisMapSnapshot {
            positions: self.positions.clone(),
            routes: self.routes.clone(),
            regions: self.regions.clone(),
            image: self.image.clone(),
            ..Default::default()
        })
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::gismap::GisMapSnapshot) -> Self {
        Self {
            positions: snapshot.positions,
            routes: snapshot.routes,
            regions: snapshot.regions,
            image: snapshot.image,
            ..Self::default()
        }
    }

    /// Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::gismap::GisMapSnapshot) {
        self.positions = snapshot.positions;
        self.routes = snapshot.routes;
        self.regions = snapshot.regions;
        self.image = snapshot.image;
    }
}
//#endregion 🔹Conversions

//#region 🔹Descriptor
/// 🧬️ Descriptor for `s.gis.gismap` — twenty handcrafted schema leaves.
pub fn gismap_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.gis.gismap",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔹Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::gismap::{GisMapDiff, GisMapMutation, GisMapSnapshot};

    #[derive(Clone, Debug, Default)]
    pub struct GismapBuilderConstruction {
        snapshot: GisMapSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for GismapBuilderConstruction {
        type Snapshot = GisMapSnapshot;
        type Mutation = GisMapMutation;
        type Diff = GisMapDiff;
        fn empty() -> Self { Self { snapshot: GisMapSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<GisMapSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<GisMapSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error(
                    "mutation.apply",
                    dsl::TextSpan::at(1, 1),
                    error.to_string(),
                )),
            }
            (self, outcome)
        }
        fn absorb(
            mut self,
            diff: Self::Diff,
        ) -> protocol::MutationApplyResult<Self> {
            let snapshot = <GisMapDiff as protocol::MutationDiff<GisMapSnapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::gismap::GisMapSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct GisMapParts {
        pub snapshot: Option<GisMapSnapshot>,
    }

    pub struct GisMapAnalyzerAnalysis;

    impl ArtifactAnalysis for GisMapAnalyzerAnalysis {
        type Parts = GisMapParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.gismap", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = GisMapParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <GisMapSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <GisMapSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec GismapBuilderFacets {
        construction: derived_construction::GismapBuilderConstruction,
        analysis: derived_analysis::GisMapAnalyzerAnalysis,
        composition: super::super::io::derived_composition::GisMapComposerComposition,
    }
    builder: GismapBuilder,
    analyzer: GisMapAnalyzer,
    composer: GisMapComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 🧭️ Relocated from the artifact's `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): pure document helpers over
/// `GisMapSnapshot`/`MapFeature`, no app-state dependency — an artifact must never depend on an app.
fn value_to_dsl(value: &Value) -> dsl::DslValue {
    dsl::to_dsl_value(value).unwrap_or(dsl::DslValue::Null)
}

fn dsl_to_value(value: &dsl::DslValue) -> Value {
    dsl::from_dsl_value(value.clone()).unwrap_or(Value::Null)
}

pub fn empty_gis_map_snapshot() -> GisMapSnapshot {
    GisMapSnapshot::default()
}

/// 📥️ Parses a `{ positions, routes, regions }` map-descriptor JSON into a `GisMapSnapshot` — each
/// array entry becomes a `MapFeature` keyed by its `id`, keeping the full object as the payload.
pub fn gis_map_document_from_descriptor_json(json: &str) -> GisMapSnapshot {
    let value: Value = serde_json::from_str(json).unwrap_or_else(|_| serde_json::json!({}));
    let features = |key: &str| -> Vec<MapFeature> {
        value
            .get(key)
            .and_then(|entry| entry.as_array())
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(|item| {
                        let id = item.get("id").and_then(|value| value.as_str())?.to_string();
                        Some(MapFeature { id, data: value_to_dsl(item) })
                    })
                    .collect()
            })
            .unwrap_or_default()
    };
    crate::artifacts::gismap::gis_map_snapshot_with_derived_children(GisMapSnapshot {
        positions: features("positions"),
        routes: features("routes"),
        regions: features("regions"),
        ..Default::default()
    })
}

/// 📤️ Rebuilds the `{ positions, routes, regions }` map-descriptor JSON the `MapHost`/renderer consume,
/// emitting each feature's opaque payload.
pub fn gis_map_descriptor_json(document: &GisMapSnapshot) -> String {
    let payloads = |features: &[MapFeature]| -> Vec<Value> { features.iter().map(|feature| dsl_to_value(&feature.data)).collect() };
    serde_json::json!({
        "positions": payloads(&document.positions),
        "routes": payloads(&document.routes),
        "regions": payloads(&document.regions),
    })
    .to_string()
}

/// 🗺️ The default map document, seeded from the bundled reuse example (see
/// `crate::artifacts::gismap::GisMapSnapshot`'s derive-generated `.gismap` DSL).
pub fn default_document() -> GisMapSnapshot {
    <GisMapSnapshot as store::ArtifactDsl>::parse_dsl(REUSE_MAP_EXAMPLE_TEXT).unwrap_or_else(|_| empty_gis_map_snapshot())
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️CollectionDiffing
/// 🌉️ Diffs one feature collection before/after an in-place edit into granular id-keyed
/// create/replace-data/delete operations — used by `patchPositions`, `setActiveExample`, and the
/// `features:in` import (whole-array replacements still converge per-feature). `create`/`delete`/
/// `replace` pick which collection's semantic-mutation triplet (positions/routes/regions) the diff
/// belongs to.
fn feature_collection_operations(
    before: &[MapFeature],
    after: &[MapFeature],
    create: impl Fn(usize, MapFeature) -> GisMapMutation,
    delete: impl Fn(String) -> GisMapMutation,
    replace: impl Fn(String, dsl::DslValue) -> GisMapMutation,
) -> Vec<GisMapMutation> {
    let mut operations = Vec::new();
    let after_ids: HashSet<&str> = after.iter().map(|feature| feature.id.as_str()).collect();
    for feature in before {
        if !after_ids.contains(feature.id.as_str()) {
            operations.push(delete(feature.id.clone()));
        }
    }
    for (index, feature) in after.iter().enumerate() {
        match before.iter().find(|entry| entry.id == feature.id) {
            None => operations.push(create(index, feature.clone())),
            Some(prev) if prev.data != feature.data => operations.push(replace(feature.id.clone(), feature.data.clone())),
            Some(_) => {}
        }
    }
    operations
}

pub fn positions_operations(before: &[MapFeature], after: &[MapFeature]) -> Vec<GisMapMutation> {
    feature_collection_operations(
        before,
        after,
        |index, item| GisMapMutation::CreatePosition(create_position::mutation::CreatePosition { index, item }),
        |id| GisMapMutation::DeletePosition(delete_position::mutation::DeletePosition { id }),
        |id, new_data| GisMapMutation::ReplacePositionData(replace_position_data::mutation::ReplacePositionData { id, new_data }),
    )
}

pub fn routes_operations(before: &[MapFeature], after: &[MapFeature]) -> Vec<GisMapMutation> {
    feature_collection_operations(
        before,
        after,
        |index, item| GisMapMutation::CreateRoute(create_route::mutation::CreateRoute { index, item }),
        |id| GisMapMutation::DeleteRoute(delete_route::mutation::DeleteRoute { id }),
        |id, new_data| GisMapMutation::ReplaceRouteData(replace_route_data::mutation::ReplaceRouteData { id, new_data }),
    )
}

pub fn regions_operations(before: &[MapFeature], after: &[MapFeature]) -> Vec<GisMapMutation> {
    feature_collection_operations(
        before,
        after,
        |index, item| GisMapMutation::CreateRegion(create_region::mutation::CreateRegion { index, item }),
        |id| GisMapMutation::DeleteRegion(delete_region::mutation::DeleteRegion { id }),
        |id, new_data| GisMapMutation::ReplaceRegionData(replace_region_data::mutation::ReplaceRegionData { id, new_data }),
    )
}
//#endregion 🔖️CollectionDiffing

//#region 🔖️DrawingBridge
/// 🎨️ The two named styles every gis-built `SemioDrawingSnapshot` references: a filled marker for
/// point features, a stroked line for route/region polylines.
const GIS_POINT_STYLE: &str = "gis-point";
const GIS_LINE_STYLE: &str = "gis-line";

/// 📍️ Reads `{ lon, lat }` off a position feature's opaque payload (the shape both
/// `gis_map_document_from_descriptor_json` and the reuse-map DSL fixture use).
fn feature_lon_lat(data: &dsl::DslValue) -> Option<(f64, f64)> {
    let value = dsl_to_value(data);
    let lon = value.get("lon").and_then(Value::as_f64)?;
    let lat = value.get("lat").and_then(Value::as_f64)?;
    Some((lon, lat))
}

/// 〰️ Reads a `{ points: [[lon, lat], …] }` vertex chain off a route/region feature's payload.
fn feature_line(data: &dsl::DslValue) -> Option<Vec<SemioPoint2>> {
    let value = dsl_to_value(data);
    let points = value.get("points").and_then(Value::as_array)?;
    let vertices: Vec<SemioPoint2> = points
        .iter()
        .filter_map(|entry| {
            let pair = entry.as_array()?;
            let x = pair.first()?.as_f64()?;
            let y = pair.get(1)?.as_f64()?;
            Some(SemioPoint2 { x, y })
        })
        .collect();
    if vertices.is_empty() { None } else { Some(vertices) }
}

/// ✏️ One open (route) or closed (region) polyline lowered to a `DrawNode::Path`, vertices shifted
/// into canvas space by `shift`.
fn polyline_draw_node(vertices: &[SemioPoint2], shift: impl Fn(&SemioPoint2) -> SemioPoint2, closed: bool) -> DrawNode {
    let mut segments: Vec<PathSegment> = vertices
        .iter()
        .enumerate()
        .map(|(index, vertex)| {
            let to = shift(vertex);
            if index == 0 { PathSegment::MoveTo { to } } else { PathSegment::LineTo { to } }
        })
        .collect();
    if closed {
        segments.push(PathSegment::Close);
    }
    DrawNode::Path { segments, style: Some(GIS_LINE_STYLE.into()) }
}

/// ⚪️ One position feature lowered to a circular marker `DrawNode::Path` (two `ArcTo` halves — the
/// standard SVG two-arc circle recipe), centered at `shift(center)`.
fn point_marker_draw_node(center: &SemioPoint2, radius: f64, shift: impl Fn(&SemioPoint2) -> SemioPoint2) -> DrawNode {
    let c = shift(center);
    let left = SemioPoint2 { x: c.x - radius, y: c.y };
    let right = SemioPoint2 { x: c.x + radius, y: c.y };
    DrawNode::Path {
        segments: vec![
            PathSegment::MoveTo { to: left },
            PathSegment::ArcTo { rx: radius, ry: radius, x_rotation: 0.0, large_arc: true, sweep: false, to: right },
            PathSegment::ArcTo { rx: radius, ry: radius, x_rotation: 0.0, large_arc: true, sweep: false, to: left },
            PathSegment::Close,
        ],
        style: Some(GIS_POINT_STYLE.into()),
    }
}

/// 🌉️ Builds a real `SemioDrawingSnapshot` from the map document: positions become circular
/// markers, routes/regions become open/closed polylines. One layer, one group, canvas sized to the
/// feature bounding box (32px pad, 256px floor) — this is the ONLY place gis turns map features
/// into drawing geometry; both `gis2d_document_json_to_svg` (export, via `io_dispatch`) and any
/// future gis drawing preview reuse it.
pub fn gis_map_snapshot_to_drawing(document: &GisMapSnapshot) -> SemioDrawingSnapshot {
    let position_points: Vec<SemioPoint2> = document
        .positions
        .iter()
        .filter_map(|feature| feature_lon_lat(&feature.data))
        .map(|(lon, lat)| SemioPoint2 { x: lon, y: lat })
        .collect();
    let route_lines: Vec<Vec<SemioPoint2>> = document.routes.iter().filter_map(|feature| feature_line(&feature.data)).collect();
    let region_polys: Vec<Vec<SemioPoint2>> = document.regions.iter().filter_map(|feature| feature_line(&feature.data)).collect();

    let all_points = position_points.iter().chain(route_lines.iter().flatten()).chain(region_polys.iter().flatten());
    let (min_x, min_y, max_x, max_y) = all_points.fold((f64::MAX, f64::MAX, f64::MIN, f64::MIN), |(min_x, min_y, max_x, max_y), p| {
        (min_x.min(p.x), min_y.min(p.y), max_x.max(p.x), max_y.max(p.y))
    });
    let (min_x, min_y, max_x, max_y) = if min_x.is_finite() { (min_x, min_y, max_x, max_y) } else { (0.0, 0.0, 0.0, 0.0) };

    let pad = 32.0;
    let width = ((max_x - min_x) + pad * 2.0).max(256.0);
    let height = ((max_y - min_y) + pad * 2.0).max(256.0);
    let shift = move |p: &SemioPoint2| SemioPoint2 { x: p.x - min_x + pad, y: p.y - min_y + pad };

    let mut children: Vec<DrawNode> = Vec::with_capacity(position_points.len() + route_lines.len() + region_polys.len());
    children.extend(position_points.iter().map(|point| point_marker_draw_node(point, 6.0, shift)));
    children.extend(route_lines.iter().map(|line| polyline_draw_node(line, shift, false)));
    children.extend(region_polys.iter().map(|poly| polyline_draw_node(poly, shift, true)));

    SemioDrawingSnapshot {
        canvas: DrawCanvas { width, height, background: None },
        styles: vec![
            DrawStyle { name: GIS_POINT_STYLE.into(), fill: Some(SemioRgba { r: 0.145, g: 0.388, b: 0.922, a: 1.0 }), stroke: None, stroke_width: None, opacity: None },
            DrawStyle { name: GIS_LINE_STYLE.into(), fill: None, stroke: Some(SemioRgba { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }), stroke_width: Some(1.0), opacity: None },
        ],
        layers: vec![DrawLayer { id: "gis-features".into(), name: "GIS Features".into(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children } }],
        ..SemioDrawingSnapshot::default()
    }
}

/// 🔑️ The `s.stdio.semio/v1/drawing` → `s.stdio.svg/1.1/*` `IoKey`, derived from
/// `SemioDrawingToSvg`'s own `FROM`/`INTO` dialect constants (no hardcoded coordinate strings —
/// stays correct if stdio ever renames the dialect).
fn drawing_to_svg_io_key() -> IoKey {
    let from = SemioDrawingToSvg::FROM;
    let into = SemioDrawingToSvg::INTO;
    IoKey {
        artifact_kind: from.artifact_kind.to_string(),
        standard: from.standard.0.to_string(),
        subset: from.subset.0.to_string(),
        direction: IoDirection::Export,
        format_kind: into.artifact_kind.to_string(),
        format_standard: into.standard.0.to_string(),
        format_subset: into.subset.0.to_string(),
    }
}

/// 🌉️ Renders a `SemioDrawingSnapshot` to real SVG text + dimensions through stdio's registered
/// `s.stdio.semio/v1/drawing` → `s.stdio.svg` bridge — the ONLY svg-producing call in this plugin
/// (no hand-rolled `<svg>` string emission left in gis).
fn render_drawing_to_svg(drawing: &SemioDrawingSnapshot) -> Result<(String, u32, u32), String> {
    let width = drawing.canvas.width.round().max(1.0) as u32;
    let height = drawing.canvas.height.round().max(1.0) as u32;
    let pack_bytes = <SemioDrawingSnapshot as store::ArtifactPack>::encode_pack(drawing);
    let source = ErasedComposeSource { dialect: SemioDrawingToSvg::FROM, payload: IoPayload::Binary(pack_bytes) };
    let composed = io_dispatch(&drawing_to_svg_io_key(), std::slice::from_ref(&source)).map_err(|error| error.message)?;
    let svg_bytes = match composed.payload {
        IoPayload::Binary(bytes) => bytes,
        IoPayload::Text(_) => return Err("drawing->svg bridge returned Text, expected an ArtifactPack-encoded SvgSnapshot".into()),
    };
    let svg_snapshot = <SvgSnapshot as store::ArtifactPack>::decode_pack(&svg_bytes).map_err(|error| error.to_string())?;
    let svg_text = <SvgSnapshot as store::ArtifactDsl>::print_dsl(&svg_snapshot);
    Ok((svg_text, width, height))
}
//#endregion 🔖️DrawingBridge

//#region 🔖️MediaExport
/// 🗺️ Builds a real `SemioDrawingSnapshot` from the map document (positions/routes/regions →
/// markers/polylines, `gis_map_snapshot_to_drawing`) and renders it through stdio's real
/// drawing↔svg bridge (`io_dispatch`) — replaces the old hand-rolled `map_points_svg` delegate.
pub fn gis2d_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    let document: GisMapSnapshot = serde_json::from_value(value.clone()).unwrap_or_default();
    let drawing = gis_map_snapshot_to_drawing(&document);
    render_drawing_to_svg(&drawing)
}
//#endregion 🔖️MediaExport

//#region 🔖️MediaImport
/// ✏️ Lowers one DWG entity's geometry to a `DrawNode::Path` with the appropriate `PathSegment`
/// sequence (`Point`→single `MoveTo`, `Line`→`MoveTo`+`LineTo`, `LwPolyline`/`Polyline3d`→
/// `MoveTo`+`LineTo`*+optional `Close`) — the semio/drawing shape every other entity-carrying
/// codec in this wave produces, even though the DWG boundary itself stays the hand-rolled
/// `semio_s_plugin_stdio::artifacts::dwg::DwgDrawing` structural codec (ticket 26/08/12/DISSOLVE-
/// KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS G2/G2b — relocated out of `semio_framework`,
/// still fixed by `register_dwg_import_handler`'s `fn(&DwgDrawing)` signature until W6's OS media
/// registry rewrite; see `stdio_gaps` in the wave report — the drawing subset's io tree carries no
/// dwg leaf, only svg/dxf/pdf, so there is no `io_dispatch`-reachable dwg decode to call through to
/// here regardless of the input boundary type).
fn dwg_geometry_to_draw_node(geometry: &DwgGeometry) -> Option<DrawNode> {
    let vertices: Vec<[f64; 2]> = match geometry {
        DwgGeometry::Point { at } => vec![[at[0], at[1]]],
        DwgGeometry::Line { start, end } => vec![[start[0], start[1]], [end[0], end[1]]],
        DwgGeometry::LwPolyline { vertices, .. } => vertices.clone(),
        DwgGeometry::Polyline3d { vertices, .. } => vertices.iter().map(|v| [v[0], v[1]]).collect(),
        _ => return None,
    };
    if vertices.is_empty() {
        return None;
    }
    let closed = matches!(geometry, DwgGeometry::LwPolyline { closed: true, .. } | DwgGeometry::Polyline3d { closed: true, .. });
    let mut segments: Vec<PathSegment> = vertices
        .iter()
        .enumerate()
        .map(|(index, v)| {
            let to = SemioPoint2 { x: v[0], y: v[1] };
            if index == 0 { PathSegment::MoveTo { to } } else { PathSegment::LineTo { to } }
        })
        .collect();
    if closed {
        segments.push(PathSegment::Close);
    }
    Some(DrawNode::Path { segments, style: None })
}

/// 🌉️ Builds a `SemioDrawingSnapshot` from a legacy `DwgDrawing`'s entities — one `DrawNode::Path`
/// per real (non-degenerate) entity, all under one layer.
fn dwg_drawing_to_semio_drawing(drawing: &DwgDrawing) -> SemioDrawingSnapshot {
    let children: Vec<DrawNode> = drawing.entities.iter().filter_map(|entity| dwg_geometry_to_draw_node(&entity.geometry)).collect();
    SemioDrawingSnapshot {
        layers: vec![DrawLayer { id: "dwg-import".into(), name: "DWG Import".into(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children } }],
        ..SemioDrawingSnapshot::default()
    }
}

/// 📍️ Walks a `DrawNode` tree collecting every `MoveTo`/`LineTo` endpoint — the vertex set the
/// import path turns into position features (mirrors the old direct `DwgGeometry` vertex walk, now
/// over the semio/drawing shape instead).
fn collect_draw_node_points(node: &DrawNode, out: &mut Vec<SemioPoint2>) {
    match node {
        DrawNode::Path { segments, .. } => {
            for segment in segments {
                match segment {
                    PathSegment::MoveTo { to } | PathSegment::LineTo { to } => out.push(*to),
                    _ => {}
                }
            }
        }
        DrawNode::Group { children, .. } => children.iter().for_each(|child| collect_draw_node_points(child, out)),
        _ => {}
    }
}

/// 🗺️ Imports a DWG drawing into a bare gis map document: DWG entities lower to `DrawNode::Path`
/// geometry (`dwg_drawing_to_semio_drawing`), whose vertices become position features. Falls back
/// to the default reuse-map document when the DWG carries no point-like geometry.
pub fn gis2d_document_json_from_dwg(drawing: &DwgDrawing) -> Result<Value, String> {
    let scene = dwg_drawing_to_semio_drawing(drawing);
    let mut points = Vec::new();
    for layer in &scene.layers {
        collect_draw_node_points(&layer.root, &mut points);
    }
    if points.is_empty() {
        return serde_json::to_value(default_document()).map_err(|error| error.to_string());
    }
    let positions: Vec<MapFeature> = points
        .iter()
        .enumerate()
        .map(|(index, point)| {
            let id = format!("dwg-{index}");
            MapFeature { id: id.clone(), data: value_to_dsl(&json!({ "id": id, "lon": point.x, "lat": point.y })) }
        })
        .collect();
    let document = crate::artifacts::gismap::gis_map_snapshot_with_derived_children(GisMapSnapshot { positions, routes: Vec::new(), regions: Vec::new(), ..Default::default() });
    serde_json::to_value(document).map_err(|error| error.to_string())
}
//#endregion 🔖️MediaImport

//#region 🧪️Tests
#[cfg(test)]
mod relocated_engine_tests {
    use super::*;
    use semio_s_plugin_stdio::artifacts::dwg::{DwgColor, DwgEntity};

    #[test]
    fn dwg_import_collects_point_and_line_vertices() {
        let mut drawing = DwgDrawing::default();
        let layer = drawing.ensure_layer("0");
        drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::Point { at: [1.0, 2.0, 0.0] } });
        drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::Line { start: [0.0, 0.0, 0.0], end: [3.0, 4.0, 0.0] } });
        let value = gis2d_document_json_from_dwg(&drawing).expect("import dwg");
        let positions = value.get("positions").and_then(|v| v.as_array()).expect("positions array");
        assert_eq!(positions.len(), 3);
    }

    #[test]
    fn dwg_import_falls_back_to_default_document_when_empty() {
        let drawing = DwgDrawing::default();
        let value = gis2d_document_json_from_dwg(&drawing).expect("import empty dwg");
        let snapshot: GisMapSnapshot = serde_json::from_value(value).expect("document");
        assert!(!snapshot.positions.is_empty(), "fallback seeds the reuse-map document");
    }

    #[test]
    fn dwg_import_lowers_a_closed_polyline_through_a_draw_node_and_carries_the_close_segment() {
        let mut drawing = DwgDrawing::default();
        let layer = drawing.ensure_layer("0");
        drawing.entities.push(DwgEntity {
            layer,
            color: DwgColor::ByLayer,
            geometry: DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]], bulges: vec![0.0, 0.0, 0.0] },
        });
        let scene = dwg_drawing_to_semio_drawing(&drawing);
        let DrawNode::Group { children, .. } = &scene.layers[0].root else { panic!("expected a group root") };
        let DrawNode::Path { segments, .. } = &children[0] else { panic!("expected a path node") };
        assert!(matches!(segments.first(), Some(PathSegment::MoveTo { .. })));
        assert!(matches!(segments.last(), Some(PathSegment::Close)));
        assert_eq!(segments.len(), 4, "3 vertices + Close");

        let value = gis2d_document_json_from_dwg(&drawing).expect("import dwg");
        let positions = value.get("positions").and_then(|v| v.as_array()).expect("positions array");
        assert_eq!(positions.len(), 3, "one position feature per polyline vertex");
    }

    /// 🌉️ Once-guarded stdio registration so `render_drawing_to_svg`'s `io_dispatch` call can
    /// resolve the `s.stdio.semio/v1/drawing` → `s.stdio.svg` bridge in a bare `cargo test`
    /// process (production boots this via stdio's own plugin `setup()`, never gis).
    fn ensure_stdio_semio_registered_for_tests() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| {
            semio_s_plugin_stdio::artifacts::semio::register();
        });
    }

    #[test]
    fn gis_map_snapshot_to_drawing_builds_markers_and_polylines() {
        let mut document = GisMapSnapshot::default();
        document.positions.push(MapFeature { id: "p0".into(), data: value_to_dsl(&json!({ "id": "p0", "lon": 5.5818, "lat": 50.603 })) });
        document.routes.push(MapFeature { id: "r0".into(), data: value_to_dsl(&json!({ "id": "r0", "points": [[5.5818, 50.603], [5.5825, 50.6035]] })) });
        document.regions.push(MapFeature { id: "g0".into(), data: value_to_dsl(&json!({ "id": "g0", "points": [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]] })) });

        let drawing = gis_map_snapshot_to_drawing(&document);
        assert!(drawing.canvas.width >= 256.0 && drawing.canvas.height >= 256.0);
        assert_eq!(drawing.styles.len(), 2);
        let DrawNode::Group { children, .. } = &drawing.layers[0].root else { panic!("expected a group root") };
        assert_eq!(children.len(), 3, "1 marker + 1 route path + 1 region path");

        let DrawNode::Path { segments: marker_segments, style: marker_style } = &children[0] else { panic!("expected the marker path first") };
        assert_eq!(marker_style.as_deref(), Some(GIS_POINT_STYLE));
        assert_eq!(marker_segments.len(), 4, "MoveTo + 2×ArcTo + Close");

        let DrawNode::Path { segments: route_segments, style: route_style } = &children[1] else { panic!("expected the route path second") };
        assert_eq!(route_style.as_deref(), Some(GIS_LINE_STYLE));
        assert!(matches!(route_segments.last(), Some(PathSegment::LineTo { .. })), "routes stay open");

        let DrawNode::Path { segments: region_segments, .. } = &children[2] else { panic!("expected the region path third") };
        assert!(matches!(region_segments.last(), Some(PathSegment::Close)), "regions close");
    }

    #[test]
    fn svg_export_renders_real_svg_text_through_the_stdio_drawing_bridge() {
        ensure_stdio_semio_registered_for_tests();
        let document = default_document();
        let value = serde_json::to_value(&document).expect("document json");
        let (svg, width, height) = gis2d_document_json_to_svg(&value).expect("svg export");
        assert!(svg.contains("<svg"), "real svg text: {svg}");
        assert!(svg.contains("<path"), "at least one path node rendered: {svg}");
        assert!(width > 0 && height > 0);
    }

    #[test]
    fn svg_export_of_an_empty_document_still_renders_a_bare_canvas() {
        ensure_stdio_semio_registered_for_tests();
        let value = serde_json::to_value(GisMapSnapshot::default()).expect("empty document json");
        let (svg, width, height) = gis2d_document_json_to_svg(&value).expect("svg export");
        assert!(svg.contains("<svg"), "{svg}");
        assert_eq!(width, 256);
        assert_eq!(height, 256);
    }

    #[test]
    fn feature_collection_diffing_emits_create_replace_and_delete() {
        let feature = |id: &str, label: &str| MapFeature { id: id.into(), data: value_to_dsl(&json!({ "id": id, "label": label })) };
        let before = vec![feature("keep", "a"), feature("gone", "b")];
        let after = vec![feature("keep", "changed"), feature("new", "c")];
        let operations = positions_operations(&before, &after);
        assert!(operations.iter().any(|operation| matches!(operation, GisMapMutation::DeletePosition(payload) if payload.id == "gone")));
        assert!(operations.iter().any(|operation| matches!(operation, GisMapMutation::ReplacePositionData(payload) if payload.id == "keep")));
        assert!(operations.iter().any(|operation| matches!(operation, GisMapMutation::CreatePosition(payload) if payload.item.id == "new")));
        assert!(routes_operations(&before, &before).is_empty(), "an unchanged collection produces no operations");
        assert!(regions_operations(&before, &before).is_empty());
    }
}
//#endregion 🧪️Tests
