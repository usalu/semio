//! 🧬️ GIS map artifact schema — every field of the artifact with its state class.

#[path = "📍️feature/🦀️.rs"]
pub mod feature;

#[cfg(test)]
#[path = "🧪️tests/🪪️document-contract/🦀️.rs"]
mod document_contract_tests;

use crate::document_dsl::REUSE_MAP_EXAMPLE_TEXT;
use crate::mutations::{create_position, create_region, create_route, delete_position, delete_region, delete_route, replace_position_data, replace_region_data, replace_route_data};
use crate::op::GisMapMutation;
use crate::{gis_map_snapshot_with_derived_children, GisMapDrawingChild, GisMapImageChild, GisMapSnapshot, GisMapValueChild, MapFeature};
use ::semio_framework_schema::ArtifactSchema;
use dsl::{FromValue, ToValue};
use semio_framework_plugin::{io_dispatch, resolve_ready, ArtifactSerializer, ErasedComposeSource, IoDirection, IoKey, IoPayload};
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioRgba, SemioTransform};
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::io::export::serializers::artifacts::png::v1_2::any::{circle_normal_form, compose_affine, flatten_segments, semio_transform_affine};
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::io::export::serializers::artifacts::svg::v1_1::any::SemioDrawingToSvg;
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot};
use semio_s_artifact_stdio_svg::SvgSnapshot;
use serde_json::Value;
use std::collections::HashSet;

//#region 🔹Artifact
/// 🧬️ GIS map document artifact state.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact_schema(id = "s.gis.gismap")]
pub struct GisMapArtifact {
    #[state(artifact)]
    pub positions: Vec<MapFeature>,
    #[state(artifact)]
    pub routes: Vec<MapFeature>,
    #[state(artifact)]
    pub regions: Vec<MapFeature>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub drawing: GisMapDrawingChild,
    /// 🕸️ Mirrors `GisMapSnapshot.image` — see that field's own doc comment and
    /// `crate::🦀️.rs`'s `🔖️Composition` region. Carried verbatim (never
    /// derived) since, unlike `drawing`/`value`, nothing in this plugin can rebuild it from
    /// `positions`/`routes`/`regions` — dropping it silently on `from_snapshot`/`to_snapshot` would
    /// be a real, undocumented data loss the moment a future basemap-capture path populates it.
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<GisMapImageChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub value: GisMapValueChild,
}
//#endregion 🔹Artifact

//#region 🔹Conversions


impl Default for GisMapArtifact {
    fn default() -> Self {
        Self::from_snapshot(GisMapSnapshot::default())
    }
}

impl GisMapArtifact {
    /// 📸️ Persisted subset with stable drawing/value member coordinates; their content is emitted
    /// as typed child work while `image` carries straight through.
    pub fn to_snapshot(&self) -> GisMapSnapshot {
        GisMapSnapshot { positions: self.positions.clone(), routes: self.routes.clone(), regions: self.regions.clone(), drawing: self.drawing.clone(), image: self.image.clone(), value: self.value.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: GisMapSnapshot) -> Self {
        Self { positions: snapshot.positions, routes: snapshot.routes, regions: snapshot.regions, drawing: snapshot.drawing, image: snapshot.image, value: snapshot.value }
    }

    /// Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: GisMapSnapshot) {
        self.positions = snapshot.positions;
        self.routes = snapshot.routes;
        self.regions = snapshot.regions;
        self.drawing = snapshot.drawing;
        self.image = snapshot.image;
        self.value = snapshot.value;
    }
}
//#endregion 🔹Conversions

//#region 🔹Descriptor
/// 🧬️ Descriptor for `s.gis.gismap` — twenty handcrafted schema leaves.
pub fn gismap_artifact_schema_descriptor() -> ::semio_framework_schema::ArtifactSchemaDescriptor {
    ::semio_framework_schema::ArtifactSchemaDescriptor {
        id: "s.gis.gismap",
        artifact: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
        snapshot: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔹Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::{GisMapDiff, GisMapMutation, GisMapSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct GismapBuilderConstruction {
        snapshot: GisMapSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for GismapBuilderConstruction {
        type Snapshot = GisMapSnapshot;
        type Mutation = GisMapMutation;
        type Diff = GisMapDiff;
        fn empty() -> Self {
            Self { snapshot: GisMapSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
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
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <GisMapDiff as protocol::MutationDiff<GisMapSnapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::GisMapSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct GisMapParts {
        pub snapshot: Option<GisMapSnapshot>,
    }

    pub struct GisMapAnalyzerAnalysis;

    impl ArtifactAnalysis for GisMapAnalyzerAnalysis {
        type Parts = GisMapParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.gis.gismap", standard: StandardId("1"), subset: SubsetId("*") };

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
        construction: GismapBuilderConstruction,
        analysis: GisMapAnalyzerAnalysis,
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
pub(crate) fn value_to_dsl(value: &Value) -> dsl::DslValue {
    dsl::DslValue::from(value)
}

pub(crate) fn dsl_to_value(value: &dsl::DslValue) -> Value {
    Value::from(value)
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
    gis_map_snapshot_with_derived_children(GisMapSnapshot { positions: features("positions"), routes: features("routes"), regions: features("regions"), ..Default::default() })
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
/// `crate::GisMapSnapshot`'s derive-generated `.gismap` DSL).
pub fn default_document() -> GisMapSnapshot {
    let parsed = <GisMapSnapshot as store::ArtifactDsl>::parse_dsl(REUSE_MAP_EXAMPLE_TEXT).unwrap_or_else(|_| empty_gis_map_snapshot());
    gis_map_snapshot_with_derived_children(parsed)
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
        |index, item| GisMapMutation::CreatePosition(create_position::CreatePosition { index, item }),
        |id| GisMapMutation::DeletePosition(delete_position::DeletePosition { id }),
        |id, new_data| GisMapMutation::ReplacePositionData(replace_position_data::ReplacePositionData { id, new_data }),
    )
}

pub fn routes_operations(before: &[MapFeature], after: &[MapFeature]) -> Vec<GisMapMutation> {
    feature_collection_operations(
        before,
        after,
        |index, item| GisMapMutation::CreateRoute(create_route::CreateRoute { index, item }),
        |id| GisMapMutation::DeleteRoute(delete_route::DeleteRoute { id }),
        |id, new_data| GisMapMutation::ReplaceRouteData(replace_route_data::ReplaceRouteData { id, new_data }),
    )
}

pub fn regions_operations(before: &[MapFeature], after: &[MapFeature]) -> Vec<GisMapMutation> {
    feature_collection_operations(
        before,
        after,
        |index, item| GisMapMutation::CreateRegion(create_region::CreateRegion { index, item }),
        |id| GisMapMutation::DeleteRegion(delete_region::DeleteRegion { id }),
        |id, new_data| GisMapMutation::ReplaceRegionData(replace_region_data::ReplaceRegionData { id, new_data }),
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

/// 〰️ Reads a `{ points: [[lon, lat], …] }` route or `{ ring: [[lon, lat], …] }` region chain.
fn feature_line(data: &dsl::DslValue) -> Option<Vec<SemioPoint2>> {
    let value = dsl_to_value(data);
    let points = value.get("points").or_else(|| value.get("ring")).and_then(Value::as_array)?;
    let vertices: Vec<SemioPoint2> = points
        .iter()
        .filter_map(|entry| {
            let pair = entry.as_array()?;
            let x = pair.first()?.as_f64()?;
            let y = pair.get(1)?.as_f64()?;
            Some(SemioPoint2 { x, y })
        })
        .collect();
    if vertices.is_empty() {
        None
    } else {
        Some(vertices)
    }
}

/// ✏️ One open (route) or closed (region) polyline lowered to a `DrawNode::Path`, vertices mapped
/// into drawing space by `place`.
fn polyline_draw_node(vertices: &[SemioPoint2], place: impl Fn(&SemioPoint2) -> SemioPoint2, closed: bool) -> DrawNode {
    let mut segments: Vec<PathSegment> = vertices.iter().enumerate().map(|(index, vertex)| if index == 0 { PathSegment::MoveTo { to: place(vertex) } } else { PathSegment::LineTo { to: place(vertex) } }).collect();
    if closed {
        segments.push(PathSegment::Close);
    }
    DrawNode::Path { segments, style: Some(GIS_LINE_STYLE.into()) }
}

/// ⚪️ One position feature lowered to a circular marker in the two-arc circle normal form the
/// drawing↔dxf bridge writes as an exact `CIRCLE`: `[MoveTo(c+r), ArcTo(c−r), ArcTo(c+r), Close]`.
fn point_marker_draw_node(center: &SemioPoint2, radius: f64, place: impl Fn(&SemioPoint2) -> SemioPoint2) -> DrawNode {
    let c = place(center);
    let right = SemioPoint2 { x: c.x + radius, y: c.y };
    let left = SemioPoint2 { x: c.x - radius, y: c.y };
    DrawNode::Path {
        segments: vec![
            PathSegment::MoveTo { to: right },
            PathSegment::ArcTo { rx: radius, ry: radius, x_rotation: 0.0, large_arc: false, sweep: true, to: left },
            PathSegment::ArcTo { rx: radius, ry: radius, x_rotation: 0.0, large_arc: false, sweep: true, to: right },
            PathSegment::Close,
        ],
        style: Some(GIS_POINT_STYLE.into()),
    }
}

/// 🌐️ Radius, in map units (degrees), of the circle a position becomes in a world-coordinate file.
pub const GIS_WORLD_MARKER_RADIUS: f64 = 1e-4;

/// 🗺️ The map's features in world coordinates: position points, route chains, region rings.
struct MapGeometry {
    positions: Vec<SemioPoint2>,
    routes: Vec<Vec<SemioPoint2>>,
    regions: Vec<Vec<SemioPoint2>>,
}

impl MapGeometry {
    fn of(document: &GisMapSnapshot) -> Self {
        Self {
            positions: document.positions.iter().filter_map(|feature| feature_lon_lat(&feature.data)).map(|(lon, lat)| SemioPoint2 { x: lon, y: lat }).collect(),
            routes: document.routes.iter().filter_map(|feature| feature_line(&feature.data)).collect(),
            regions: document.regions.iter().filter_map(|feature| feature_line(&feature.data)).collect(),
        }
    }

    fn bounds(&self) -> (f64, f64, f64, f64) {
        let all = self.positions.iter().chain(self.routes.iter().flatten()).chain(self.regions.iter().flatten());
        let (min_x, min_y, max_x, max_y) = all.fold((f64::MAX, f64::MAX, f64::MIN, f64::MIN), |(min_x, min_y, max_x, max_y), p| (min_x.min(p.x), min_y.min(p.y), max_x.max(p.x), max_y.max(p.y)));
        if min_x.is_finite() { (min_x, min_y, max_x, max_y) } else { (0.0, 0.0, 0.0, 0.0) }
    }

    fn drawing(&self, width: f64, height: f64, marker_radius: f64, stroke_width: f64, place: impl Fn(&SemioPoint2) -> SemioPoint2 + Copy) -> SemioDrawingSnapshot {
        let mut children: Vec<DrawNode> = Vec::with_capacity(self.positions.len() + self.routes.len() + self.regions.len());
        children.extend(self.positions.iter().map(|point| point_marker_draw_node(point, marker_radius, place)));
        children.extend(self.routes.iter().map(|line| polyline_draw_node(line, place, false)));
        children.extend(self.regions.iter().map(|ring| polyline_draw_node(ring, place, true)));
        SemioDrawingSnapshot {
            canvas: DrawCanvas { width, height, background: None },
            styles: vec![
                DrawStyle { name: GIS_POINT_STYLE.into(), fill: Some(SemioRgba { r: 0.145, g: 0.388, b: 0.922, a: 1.0 }), stroke: None, stroke_width: None, opacity: None },
                DrawStyle { name: GIS_LINE_STYLE.into(), fill: None, stroke: Some(SemioRgba { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }), stroke_width: Some(stroke_width), opacity: None },
            ],
            layers: vec![DrawLayer { id: "gis-features".into(), name: "GIS Features".into(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children } }],
            ..SemioDrawingSnapshot::default()
        }
    }
}

/// 🌉️ The map as a PAGE drawing for canvas formats (svg, pdf, png): positions become circular
/// markers, routes/regions open/closed polylines, north up, canvas sized to the feature bounding box
/// (32px pad, 256px floor). Map units become canvas units one to one.
pub fn gis_map_snapshot_to_drawing(document: &GisMapSnapshot) -> SemioDrawingSnapshot {
    let geometry = MapGeometry::of(document);
    let (min_x, min_y, max_x, max_y) = geometry.bounds();
    let pad = 32.0;
    let width = ((max_x - min_x) + pad * 2.0).max(256.0);
    let height = ((max_y - min_y) + pad * 2.0).max(256.0);
    geometry.drawing(width, height, 6.0, 1.0, move |p: &SemioPoint2| SemioPoint2 { x: p.x - min_x + pad, y: max_y - p.y + pad })
}

/// 🌐️ The map as a WORLD drawing for coordinate formats (dxf, dwg): every vertex is its own
/// `(lon, lat)`, positions are circles of [`GIS_WORLD_MARKER_RADIUS`]; [`gis_map_snapshot_from_drawing`]
/// reads it back feature for feature.
pub fn gis_map_snapshot_to_world_drawing(document: &GisMapSnapshot) -> SemioDrawingSnapshot {
    let geometry = MapGeometry::of(document);
    let (min_x, min_y, max_x, max_y) = geometry.bounds();
    geometry.drawing((max_x - min_x).max(GIS_WORLD_MARKER_RADIUS), (max_y - min_y).max(GIS_WORLD_MARKER_RADIUS), GIS_WORLD_MARKER_RADIUS, 0.0, |p: &SemioPoint2| *p)
}

/// 📥️ Reads a WORLD drawing (a dxf or dwg file's geometry, coordinates taken as `lon`/`lat`) into map
/// features: circles become positions at their centres, closed paths regions, open paths routes.
/// Group transforms are applied; text and images carry no map feature and are skipped.
pub fn gis_map_snapshot_from_drawing(drawing: &SemioDrawingSnapshot) -> GisMapSnapshot {
    fn walk(node: &DrawNode, matrix: [f64; 6], document: &mut GisMapSnapshot) {
        let apply = |p: [f64; 2]| [matrix[0] * p[0] + matrix[2] * p[1] + matrix[4], matrix[1] * p[0] + matrix[3] * p[1] + matrix[5]];
        match node {
            DrawNode::Group { transform, children } => {
                let inner = compose_affine(&matrix, &semio_transform_affine(transform));
                children.iter().for_each(|child| walk(child, inner, document));
            }
            DrawNode::Path { segments, .. } => {
                if let Some((centre, _)) = circle_normal_form(segments) {
                    let [lon, lat] = apply(centre);
                    let id = format!("position-{}", document.positions.len());
                    document.positions.push(MapFeature { id: id.clone(), data: value_to_dsl(&serde_json::json!({ "id": id, "lon": lon, "lat": lat })) });
                    return;
                }
                for (points, closed) in flatten_segments(segments, 1.0) {
                    let points: Vec<Value> = points.iter().map(|p| apply(*p)).map(|[x, y]| serde_json::json!([x, y])).collect();
                    let (family, kind) = if closed { (&mut document.regions, "region") } else { (&mut document.routes, "route") };
                    let id = format!("{kind}-{}", family.len());
                    family.push(MapFeature { id: id.clone(), data: value_to_dsl(&serde_json::json!({ "id": id, "points": points })) });
                }
            }
            DrawNode::Text { .. } | DrawNode::Image { .. } => {}
        }
    }
    let mut document = GisMapSnapshot::default();
    for layer in &drawing.layers {
        walk(&layer.root, [1.0, 0.0, 0.0, 1.0, 0.0, 0.0], &mut document);
    }
    gis_map_snapshot_with_derived_children(document)
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
    let composed = resolve_ready(io_dispatch(&drawing_to_svg_io_key(), std::slice::from_ref(&source))).map_err(|error| error.message)?;
    let svg_bytes = match composed.payload {
        IoPayload::Binary(bytes) => bytes,
        IoPayload::Text(_) => return Err("drawing->svg bridge returned Text, expected an ArtifactPack-encoded SvgSnapshot".into()),
    };
    let svg_snapshot = <SvgSnapshot as store::ArtifactPack>::decode_pack(&svg_bytes).map_err(|error| error.to_string())?;
    let svg_text = String::from_utf8(svg_snapshot.export_utf8()?).map_err(|error| error.to_string())?;
    Ok((svg_text, width, height))
}
//#endregion 🔖️DrawingBridge

//#region 🔖️MediaExport
/// 🗺️ Builds a real `SemioDrawingSnapshot` from the map document (positions/routes/regions →
/// markers/polylines, `gis_map_snapshot_to_drawing`) and renders it through stdio's real
/// drawing↔svg bridge (`io_dispatch`) — replaces the old hand-rolled `map_points_svg` delegate.
pub fn gis2d_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    let document = GisMapSnapshot::from_value(value_to_dsl(value)).unwrap_or_default();
    let drawing = gis_map_snapshot_to_drawing(&document);
    render_drawing_to_svg(&drawing)
}
//#endregion 🔖️MediaExport

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️relocated-engine/🦀️.rs"]
mod relocated_engine_tests;
//#endregion 🧪️Tests
