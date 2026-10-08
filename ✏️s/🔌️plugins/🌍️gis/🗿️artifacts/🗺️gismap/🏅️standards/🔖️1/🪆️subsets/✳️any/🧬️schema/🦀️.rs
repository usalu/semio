//! 🧬️ GIS map artifact schema — every field of the artifact with its state class.

#[path = "📍️feature/🦀️.rs"]
pub mod feature;

#[cfg(test)]
#[path = "🧪️tests/🪪️document/🦀️.rs"]
mod document_contract_tests;


use crate::{gis_map_snapshot_with_derived_children, GisMapDrawingChild, GisMapImageChild, GisMapSnapshot, GisMapValueChild, MapFeature};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value::ToValue;
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioRgba, SemioTransform};
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::geometry::{circle_normal_form, compose_affine, flatten_segments, semio_transform_affine};

use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot};
use semio_framework_value::{DslValue,Number};

//#region 🔹Artifact
/// 🧬️ GIS map document artifact state.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_value::ToValue, semio_framework_value::FromValue)]
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
pub fn gismap_artifact_schema_descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.gis.gismap",
        artifact: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
        snapshot: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: ::semio_framework_schema_registry::FacetLeaves {
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

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers


//#endregion 🔖️DocumentHelpers


//#region 🔖️DrawingBridge
/// 🎨️ The two named styles every gis-built `SemioDrawingSnapshot` references: a filled marker for
/// point features, a stroked line for route/region polylines.
const GIS_POINT_STYLE: &str = "gis-point";
const GIS_LINE_STYLE: &str = "gis-line";

/// 📍️ Reads `{ lon, lat }` off a position feature's opaque payload (the shape both
/// `gis_map_document_from_descriptor_json` and the reuse-map DSL fixture use).
fn feature_lon_lat(data: &semio_framework_value::DslValue) -> Option<(f64, f64)> {
    let value = data;
    let lon = value.get("lon").and_then(DslValue::as_f64)?;
    let lat = value.get("lat").and_then(DslValue::as_f64)?;
    Some((lon, lat))
}

/// 〰️ Reads a `{ points: [[lon, lat], …] }` route or `{ ring: [[lon, lat], …] }` region chain.
fn feature_line(data: &semio_framework_value::DslValue) -> Option<Vec<SemioPoint2>> {
    let value = data;
    let points = value.get("points").or_else(|| value.get("ring")).and_then(DslValue::as_array)?;
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
                    document.positions.push(MapFeature { id: id.clone(), data: DslValue::object([("id".into(),DslValue::String(id)),("lon".into(),DslValue::Number(Number::Float(lon))),("lat".into(),DslValue::Number(Number::Float(lat)))]) });
                    return;
                }
                for (points, closed) in flatten_segments(segments, 1.0) {
                    let points: Vec<DslValue> = points.iter().map(|p| apply(*p)).map(|[x,y]| DslValue::Array(vec![DslValue::Number(Number::Float(x)),DslValue::Number(Number::Float(y))])).collect();
                    let (family, kind) = if closed { (&mut document.regions, "region") } else { (&mut document.routes, "route") };
                    let id = format!("{kind}-{}", family.len());
                    family.push(MapFeature { id: id.clone(), data: DslValue::object([("id".into(),DslValue::String(id)),("points".into(),DslValue::Array(points))]) });
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




//#endregion 🔖️DrawingBridge

//#region 🔖️MediaExport
//#endregion 🔖️MediaExport

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️relocated-engine/🦀️.rs"]
mod relocated_engine_tests;
//#endregion 🧪️Tests

/// 🗺️ Owned descriptor tree for composed value child inference.
pub fn gis_map_descriptor_value(document:&GisMapSnapshot)->DslValue{let payloads=|features:&[MapFeature]|DslValue::Array(features.iter().map(|feature|feature.data.clone()).collect());DslValue::object([("positions".into(),payloads(&document.positions)),("routes".into(),payloads(&document.routes)),("regions".into(),payloads(&document.regions))])}
