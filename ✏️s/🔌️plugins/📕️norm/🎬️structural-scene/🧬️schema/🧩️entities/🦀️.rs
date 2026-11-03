//! 🧩️ Entities: the scene root, stable entity identity, and the sum type of everything drawable.

use super::*;

use std::collections::BTreeSet;

pub const SCENE_SCHEMA_ID: &str = "semio.norm.structural-scene/v1";

records! {
    /// 🎬️ A complete, locale-free picture of one artifact at one document and report revision.
    pub struct StructuralScene {
        pub schema: String,
        pub revision: SceneRevision,
        pub bounds: Aabb,
        pub entities: Vec<SceneEntity>,
        pub annotations: Vec<Annotation>,
        pub legends: Vec<Legend>,
        pub layers: Vec<Layer>,
        pub tracks: Vec<ParameterTrack>,
        pub owners: Vec<Ownership>,
        pub description: LocalizedCopy,
    }

    /// 🔗️ The entity an annotation or a parameter track belongs to; subjects without a row are global to the scene.
    pub struct Ownership {
        pub kind: OwnedKind,
        pub id: String,
        pub owner: EntityRef,
    }

    /// 🔢️ Monotonic sequence numbers of the document and of the report a scene was projected from.
    pub struct SceneRevision {
        pub document_seq: u64,
        pub report_seq: u64,
    }

    /// 🆔️ Stable identity equal to document identity so a pick resolves straight to a field-path prefix.
    pub struct EntityRef {
        pub kind: EntityKind,
        pub id: String,
        pub path: String,
    }

    /// 🧱️ One drawable, pickable, accessible scene object.
    pub struct SceneEntity {
        pub entity: EntityRef,
        pub geometry: Geometry,
        pub appearance: Appearance,
        pub utilisation: Option<UtilisationBinding>,
        pub label: Option<LocalizedCopy>,
        pub accessible_name: LocalizedCopy,
        pub accessible_detail: LocalizedCopy,
        pub layer: String,
        pub pick: PickPolicy,
        pub handles: Vec<String>,
        pub content_hash: String,
    }

    /// 📏️ Member axis plus section extrusion.
    pub struct MemberGeometry {
        pub axis: Path3,
        pub frame: Frame,
        pub section: SectionShape,
        pub section_taper: Option<SectionShape>,
        pub start_offset: Vec2,
        pub end_offset: Vec2,
        pub start_release: EndRelease,
        pub end_release: EndRelease,
        pub stations: Vec<f64>,
    }

    /// 🟫️ Flat plate, slab or panel with optional stiffeners.
    pub struct PlateGeometry {
        pub frame: Frame,
        pub outline: Polygon2,
        pub thickness: f64,
        pub offset: f64,
        pub stiffeners: Vec<Stiffener>,
    }

    /// 🦴️ Rib or stiffener running along a polyline in plate coordinates.
    pub struct Stiffener {
        pub path: Polyline2,
        pub section: SectionShape,
    }

    /// 🥣️ Thin shell of revolution or parametric patch.
    pub struct ShellGeometry {
        pub shape: ShellShape,
        pub thickness: f64,
    }

    /// 🕸️ Regular grid of points, row-major.
    pub struct PointGrid {
        pub rows: u32,
        pub columns: u32,
        pub points: Vec<Vec3>,
    }

    /// 🔺️ Index triple of a mesh triangle.
    pub struct Triangle {
        pub a: u32,
        pub b: u32,
        pub c: u32,
    }

    /// 🪨️ One soil or fill layer.
    pub struct Stratum {
        pub thickness: f64,
        pub role: MaterialRole,
        pub label: LocalizedCopy,
    }

    /// 🧱️ One build-up layer of a wall.
    pub struct WallLayer {
        pub thickness: f64,
        pub role: MaterialRole,
        pub label: LocalizedCopy,
    }

    /// 🪟️ Rectangular cut through a wall: `offset` along the wall path from its first point, `sill` above the base.
    pub struct WallCut {
        pub offset: f64,
        pub width: f64,
        pub sill: f64,
        pub height: f64,
    }

    /// 🌲️ One board layer of a laminate; `grain` is the fibre angle in radians from the frame `x` axis.
    pub struct GrainLayer {
        pub thickness: f64,
        pub role: MaterialRole,
        pub grain: f64,
        pub label: LocalizedCopy,
    }

    /// 🔲️ Stirrup spacing zone: `count` loops from `start` metres along the axis, `spacing` apart.
    pub struct LinkZone {
        pub start: f64,
        pub spacing: f64,
        pub count: u32,
    }

    /// ✂️ Intermittent weld: `length` of seam followed by `gap` without.
    pub struct WeldPitch {
        pub length: f64,
        pub gap: f64,
    }

    /// 🧊️ Volumetric body.
    pub struct SolidGeometry {
        pub shape: SolidShape,
    }

    /// 🔩️ A fastener family member, optionally repeated in a pattern.
    pub struct FastenerGeometry {
        pub kind: FastenerKind,
        pub axis: Segment3,
        pub diameter: f64,
        pub head: HeadShape,
        pub pattern: Option<Pattern>,
        pub hole_clearance: Option<f64>,
    }

    /// 🪝️ Bent rebar end geometry; with `both_ends` the same hook is also made at the `from` end, turning to the same side (U-bars, hooked beam bars).
    pub struct BendShape {
        pub bend_angle: f64,
        pub bend_radius: f64,
        pub leg_length: f64,
        pub both_ends: bool,
    }

    /// 🌊️ Soil surface, failure plane or interaction surface.
    pub struct SurfaceGeometry {
        pub shape: SurfaceShape,
    }
}

choices! {
    /// 🔗️ What an [`Ownership`] row points at.
    pub enum OwnedKind {
        Annotation,
        Track,
    }

    /// 🗂️ Entity class used for identity and filtering.
    pub enum EntityKind {
        Member,
        Plate,
        Shell,
        Solid,
        Fastener,
        Support,
        Load,
        Surface,
        Group,
    }

    /// 🖱️ Granularity at which an entity may be picked.
    pub enum PickPolicy {
        Whole,
        Vertex,
        Edge,
        Face,
        None,
    }

    /// 🔗️ Visual end release glyph of a member.
    pub enum EndRelease {
        Rigid,
        Pinned,
        Sliding,
        Free,
    }

    /// 🔘️ Fastener head glyph.
    pub enum HeadShape {
        Hex,
        Countersunk,
        Round,
        Headed,
        None,
    }

    /// 🔶️ Weld seam class.
    pub enum WeldKind {
        Fillet,
        Butt,
    }
}

variants! {
    /// 🎭️ Everything drawable, as one sum type.
    pub enum Geometry {
        Member(MemberGeometry),
        Plate(PlateGeometry),
        Shell(ShellGeometry),
        Solid(SolidGeometry),
        Fastener(FastenerGeometry),
        Support(SupportGeometry),
        Load(LoadGlyph),
        Surface(SurfaceGeometry),
        Group { members: Vec<EntityRef> },
    }

    /// 🌀️ Shell construction.
    pub enum ShellShape {
        Revolved { origin: Vec3, axis: Vec3, generatrix: Polyline2, sweep: f64 },
        Patch { grid: PointGrid },
    }

    /// 🧊️ Solid construction. `Box` and `Cylinder` are anchored at the centre of their base on the frame origin (`Box` spans `±size/2` across and `0..size.z` up) in tessellation and viewport alike; `Prism` places its base polygon in frame coordinates, so its anchor is whatever the polygon says.
    pub enum SolidShape {
        Prism { frame: Frame, base: Polygon2, height: f64 },
        Box { frame: Frame, size: Vec3 },
        Cylinder { frame: Frame, radius: f64, height: f64 },
        Wedge { frame: Frame, base: Vec2, height: f64 },
        Mesh { vertices: Vec<Vec3>, triangles: Vec<Triangle> },
        StrataColumn { frame: Frame, footprint: Polygon2, strata: Vec<Stratum> },
        FootingBody { frame: Frame, footprint: Polygon2, thickness: f64, pedestal: Option<Vec2>, pedestal_height: f64 },
        WallAssembly { path: Polyline3, height: f64, layers: Vec<WallLayer>, openings: Vec<WallCut> },
        Laminate { frame: Frame, footprint: Polygon2, layers: Vec<GrainLayer> },
    }

    /// 🔩️ Fastener family. `WeldRun` follows its own path, continuous or with the given pitch, instead of the straight axis.
    pub enum FastenerKind {
        Bolt { class: String, preload: Option<Quantity> },
        Screw,
        Nail,
        Dowel,
        Stud { head_height: f64, head_diameter: f64 },
        Rebar { bend: Option<BendShape> },
        Anchor,
        Weld { throat: f64, weld: WeldKind },
        WeldRun { throat: f64, weld: WeldKind, path: Path3, pitch: Option<WeldPitch> },
        Tendon { profile: Path3, duct: Option<f64> },
    }

    /// 🔁️ Repetition of a fastener. `Cage` loops are the closed `outline` in section coordinates repeated per spacing zone; outline `x` runs along `reference` projected perpendicular to the axis, outline `y` along axis cross that; `Points` are explicit instance offsets in axis coordinates (`x` along the axis, then `y`, `z`); per-instance colour comes from a `FieldSamples::Instance` field.
    pub enum Pattern {
        Grid { nx: u32, ny: u32, sx: f64, sy: f64 },
        Row { n: u32, s: f64 },
        Along { path: Path3, spacing: f64 },
        Cage { outline: Ring2, zones: Vec<LinkZone>, reference: Vec3 },
        Points { offsets: Vec<Vec3> },
    }

    /// 🌍️ Surface construction.
    pub enum SurfaceShape {
        Polygon { polygon: Polygon3 },
        Arc { arc: Arc, width: f64 },
        Grid { grid: PointGrid },
    }
}

impl EntityRef {
    pub fn new(kind: EntityKind, id: impl Into<String>, path: impl Into<String>) -> Self {
        Self { kind, id: id.into(), path: path.into() }
    }
}

impl SceneEntity {
    /// #️⃣ FNV-1a over the canonical JSON of geometry, appearance and utilisation, as 16 lowercase hex digits.
    pub fn compute_hash(&self) -> String {
        let text = format!("{}|{}|{}", canonical_json(&self.geometry), canonical_json(&self.appearance), canonical_json(&self.utilisation));
        format!("{:016x}", text.bytes().fold(0xcbf2_9ce4_8422_2325_u64, |h, b| (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3)))
    }

    pub fn sealed(mut self) -> Self {
        self.content_hash = self.compute_hash();
        self
    }
}

impl Geometry {
    pub fn kind(&self) -> EntityKind {
        match self {
            Self::Member(_) => EntityKind::Member,
            Self::Plate(_) => EntityKind::Plate,
            Self::Shell(_) => EntityKind::Shell,
            Self::Solid(_) => EntityKind::Solid,
            Self::Fastener(_) => EntityKind::Fastener,
            Self::Support(_) => EntityKind::Support,
            Self::Load(_) => EntityKind::Load,
            Self::Surface(_) => EntityKind::Surface,
            Self::Group { .. } => EntityKind::Group,
        }
    }
}

impl Copy for SceneRevision {}

impl Validate for SceneRevision {
    fn check(&self, _path: &str, _out: &mut Vec<Violation>) {}
}

impl Validate for Ownership {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_empty(out, &at(path, "id"), &self.id);
        self.owner.check(&at(path, "owner"), out);
    }
}

impl Validate for EntityRef {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_empty(out, &at(path, "id"), &self.id);
        non_empty(out, &at(path, "path"), &self.path);
    }
}

impl Validate for MemberGeometry {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.axis.check(&at(path, "axis"), out);
        self.frame.check(&at(path, "frame"), out);
        self.section.check(&at(path, "section"), out);
        self.section_taper.check(&at(path, "sectionTaper"), out);
        self.start_offset.check(&at(path, "startOffset"), out);
        self.end_offset.check(&at(path, "endOffset"), out);
        expect(out, &at(path, "stations"), self.stations.iter().all(|s| s.is_finite() && (0.0..=1.0).contains(s)) && self.stations.windows(2).all(|w| w[0] < w[1]), "ascending-unit-interval");
    }
}

impl Validate for Stiffener {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.path.check(&at(path, "path"), out);
        self.section.check(&at(path, "section"), out);
    }
}

impl Validate for PlateGeometry {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.frame.check(&at(path, "frame"), out);
        self.outline.check(&at(path, "outline"), out);
        positive(out, &at(path, "thickness"), self.thickness);
        finite(out, &at(path, "offset"), &[self.offset]);
        each(&self.stiffeners, path, "stiffeners", out);
    }
}

impl Validate for PointGrid {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        expect(out, path, self.rows >= 2 && self.columns >= 2 && self.points.len() == (self.rows as usize) * (self.columns as usize), "rows-times-columns-points");
        each(&self.points, path, "points", out);
    }
}

impl Validate for ShellShape {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        match self {
            Self::Revolved { origin, axis, generatrix, sweep } => {
                origin.check(&at(path, "origin"), out);
                axis.check(&at(path, "axis"), out);
                expect(out, &at(path, "axis"), axis.length() > 0.0, "non-zero-direction");
                generatrix.check(&at(path, "generatrix"), out);
                expect(out, &at(path, "sweep"), sweep.is_finite() && *sweep != 0.0 && sweep.abs() <= std::f64::consts::TAU + TOLERANCE, "sweep-within-full-turn");
            }
            Self::Patch { grid } => grid.check(&at(path, "grid"), out),
        }
    }
}

impl Validate for ShellGeometry {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.shape.check(&at(path, "shape"), out);
        positive(out, &at(path, "thickness"), self.thickness);
    }
}

impl Validate for Triangle {
    fn check(&self, _path: &str, _out: &mut Vec<Violation>) {}
}

impl Validate for Stratum {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        positive(out, &at(path, "thickness"), self.thickness);
        self.role.check(&at(path, "role"), out);
        self.label.check(&at(path, "label"), out);
    }
}

impl Validate for WallLayer {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        positive(out, &at(path, "thickness"), self.thickness);
        self.role.check(&at(path, "role"), out);
        self.label.check(&at(path, "label"), out);
    }
}

impl Validate for WallCut {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_negative(out, &at(path, "offset"), self.offset);
        positive(out, &at(path, "width"), self.width);
        non_negative(out, &at(path, "sill"), self.sill);
        positive(out, &at(path, "height"), self.height);
    }
}

impl Validate for GrainLayer {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        positive(out, &at(path, "thickness"), self.thickness);
        finite(out, &at(path, "grain"), &[self.grain]);
        self.role.check(&at(path, "role"), out);
        self.label.check(&at(path, "label"), out);
    }
}

impl Validate for LinkZone {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_negative(out, &at(path, "start"), self.start);
        positive(out, &at(path, "spacing"), self.spacing);
        expect(out, &at(path, "count"), self.count >= 1, "count-at-least-1");
    }
}

impl Validate for WeldPitch {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        positive(out, &at(path, "length"), self.length);
        non_negative(out, &at(path, "gap"), self.gap);
    }
}

impl Validate for SolidShape {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        match self {
            Self::Prism { frame, base, height } => {
                frame.check(&at(path, "frame"), out);
                base.check(&at(path, "base"), out);
                positive(out, &at(path, "height"), *height);
            }
            Self::Box { frame, size } => {
                frame.check(&at(path, "frame"), out);
                expect(out, &at(path, "size"), size.x > 0.0 && size.y > 0.0 && size.z > 0.0 && size.x.is_finite() && size.y.is_finite() && size.z.is_finite(), "positive");
            }
            Self::Cylinder { frame, radius, height } => {
                frame.check(&at(path, "frame"), out);
                positive(out, &at(path, "radius"), *radius);
                positive(out, &at(path, "height"), *height);
            }
            Self::Wedge { frame, base, height } => {
                frame.check(&at(path, "frame"), out);
                expect(out, &at(path, "base"), base.x > 0.0 && base.y > 0.0 && base.x.is_finite() && base.y.is_finite(), "positive");
                positive(out, &at(path, "height"), *height);
            }
            Self::Mesh { vertices, triangles } => {
                each(vertices, path, "vertices", out);
                expect(out, &at(path, "triangles"), !triangles.is_empty(), "non-empty");
                let n = vertices.len() as u32;
                expect(out, &at(path, "triangles"), triangles.iter().all(|t| t.a < n && t.b < n && t.c < n && t.a != t.b && t.b != t.c && t.a != t.c), "valid-vertex-indices");
            }
            Self::StrataColumn { frame, footprint, strata } => {
                frame.check(&at(path, "frame"), out);
                footprint.check(&at(path, "footprint"), out);
                expect(out, &at(path, "strata"), !strata.is_empty(), "non-empty");
                each(strata, path, "strata", out);
            }
            Self::FootingBody { frame, footprint, thickness, pedestal, pedestal_height } => {
                frame.check(&at(path, "frame"), out);
                footprint.check(&at(path, "footprint"), out);
                positive(out, &at(path, "thickness"), *thickness);
                pedestal.check(&at(path, "pedestal"), out);
                non_negative(out, &at(path, "pedestalHeight"), *pedestal_height);
            }
            Self::WallAssembly { path: line, height, layers, openings } => {
                line.check(&at(path, "path"), out);
                positive(out, &at(path, "height"), *height);
                expect(out, &at(path, "layers"), !layers.is_empty(), "non-empty");
                each(layers, path, "layers", out);
                each(openings, path, "openings", out);
                let run: f64 = line.points.windows(2).map(|w| w[0].distance(&w[1])).sum();
                expect(out, &at(path, "openings"), openings.iter().all(|o| o.offset + o.width <= run + TOLERANCE && o.sill + o.height <= *height + TOLERANCE), "inside-wall");
                let mut sorted: Vec<&WallCut> = openings.iter().collect();
                sorted.sort_by(|a, b| a.offset.total_cmp(&b.offset));
                expect(out, &at(path, "openings"), sorted.windows(2).all(|w| w[0].offset + w[0].width <= w[1].offset + TOLERANCE), "disjoint");
            }
            Self::Laminate { frame, footprint, layers } => {
                frame.check(&at(path, "frame"), out);
                footprint.check(&at(path, "footprint"), out);
                expect(out, &at(path, "layers"), !layers.is_empty(), "non-empty");
                each(layers, path, "layers", out);
            }
        }
    }
}

impl Validate for SolidGeometry {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.shape.check(&at(path, "shape"), out);
    }
}

impl Validate for BendShape {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        finite(out, path, &[self.bend_angle, self.bend_radius, self.leg_length]);
        expect(out, path, self.bend_radius > 0.0 && self.leg_length > 0.0 && self.bend_angle != 0.0, "positive");
    }
}

impl Validate for FastenerKind {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        match self {
            Self::Bolt { class, preload } => {
                non_empty(out, &at(path, "class"), class);
                preload.check(&at(path, "preload"), out);
            }
            Self::Stud { head_height, head_diameter } => {
                positive(out, &at(path, "headHeight"), *head_height);
                positive(out, &at(path, "headDiameter"), *head_diameter);
            }
            Self::Rebar { bend } => bend.check(&at(path, "bend"), out),
            Self::Weld { throat, .. } => positive(out, &at(path, "throat"), *throat),
            Self::WeldRun { throat, path: line, pitch, .. } => {
                positive(out, &at(path, "throat"), *throat);
                line.check(&at(path, "path"), out);
                pitch.check(&at(path, "pitch"), out);
            }
            Self::Tendon { profile, duct } => {
                profile.check(&at(path, "profile"), out);
                if let Some(d) = duct {
                    positive(out, &at(path, "duct"), *d);
                }
            }
            Self::Screw | Self::Nail | Self::Dowel | Self::Anchor => {}
        }
    }
}

impl Validate for Pattern {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        match self {
            Self::Grid { nx, ny, sx, sy } => {
                expect(out, path, *nx >= 1 && *ny >= 1, "count-at-least-1");
                positive(out, &at(path, "sx"), *sx);
                positive(out, &at(path, "sy"), *sy);
            }
            Self::Row { n, s } => {
                expect(out, &at(path, "n"), *n >= 1, "count-at-least-1");
                positive(out, &at(path, "s"), *s);
            }
            Self::Along { path: line, spacing } => {
                line.check(&at(path, "path"), out);
                positive(out, &at(path, "spacing"), *spacing);
            }
            Self::Cage { outline, zones, reference } => {
                outline.check(&at(path, "outline"), out);
                reference.check(&at(path, "reference"), out);
                expect(out, &at(path, "reference"), reference.length() > 0.0, "non-zero-direction");
                expect(out, &at(path, "zones"), !zones.is_empty(), "non-empty");
                each(zones, path, "zones", out);
                expect(out, &at(path, "zones"), zones.windows(2).all(|w| w[0].start < w[1].start), "ascending-start");
            }
            Self::Points { offsets } => {
                expect(out, &at(path, "offsets"), !offsets.is_empty(), "non-empty");
                each(offsets, path, "offsets", out);
            }
        }
    }
}

impl Validate for FastenerGeometry {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.kind.check(&at(path, "kind"), out);
        self.axis.check(&at(path, "axis"), out);
        positive(out, &at(path, "diameter"), self.diameter);
        self.pattern.check(&at(path, "pattern"), out);
        if let Some(c) = self.hole_clearance {
            non_negative(out, &at(path, "holeClearance"), c);
        }
    }
}

impl Validate for SurfaceShape {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        match self {
            Self::Polygon { polygon } => polygon.check(&at(path, "polygon"), out),
            Self::Arc { arc, width } => {
                arc.check(&at(path, "arc"), out);
                positive(out, &at(path, "width"), *width);
            }
            Self::Grid { grid } => grid.check(&at(path, "grid"), out),
        }
    }
}

impl Validate for SurfaceGeometry {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.shape.check(&at(path, "shape"), out);
    }
}

impl Validate for Geometry {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        match self {
            Self::Member(g) => g.check(path, out),
            Self::Plate(g) => g.check(path, out),
            Self::Shell(g) => g.check(path, out),
            Self::Solid(g) => g.check(path, out),
            Self::Fastener(g) => g.check(path, out),
            Self::Support(g) => g.check(path, out),
            Self::Load(g) => g.check(path, out),
            Self::Surface(g) => g.check(path, out),
            Self::Group { members } => {
                expect(out, &at(path, "members"), !members.is_empty(), "non-empty");
                each(members, path, "members", out);
            }
        }
    }
}

impl Validate for SceneEntity {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.entity.check(&at(path, "entity"), out);
        expect(out, &at(path, "entity.kind"), self.geometry.kind() == self.entity.kind, "kind-matches-geometry");
        self.geometry.check(&at(path, "geometry"), out);
        self.appearance.check(&at(path, "appearance"), out);
        self.utilisation.check(&at(path, "utilisation"), out);
        self.label.check(&at(path, "label"), out);
        self.accessible_name.check(&at(path, "accessibleName"), out);
        self.accessible_detail.check(&at(path, "accessibleDetail"), out);
        non_empty(out, &at(path, "layer"), &self.layer);
        expect(out, &at(path, "handles"), self.handles.iter().all(|h| !h.trim().is_empty()) && self.handles.iter().collect::<BTreeSet<_>>().len() == self.handles.len(), "unique-non-empty");
        expect(out, &at(path, "contentHash"), self.content_hash == self.compute_hash(), "matches-content");
    }
}

impl Validate for StructuralScene {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        expect(out, &at(path, "schema"), self.schema == SCENE_SCHEMA_ID, "schema-id");
        self.bounds.check(&at(path, "bounds"), out);
        each(&self.entities, path, "entities", out);
        each(&self.annotations, path, "annotations", out);
        each(&self.legends, path, "legends", out);
        each(&self.layers, path, "layers", out);
        each(&self.tracks, path, "tracks", out);
        each(&self.owners, path, "owners", out);
        self.description.check(&at(path, "description"), out);
        let unique = |ids: Vec<&str>| ids.iter().collect::<BTreeSet<_>>().len() == ids.len();
        expect(out, &at(path, "entities"), unique(self.entities.iter().map(|e| e.entity.id.as_str()).collect()), "unique-entity-ids");
        expect(out, &at(path, "annotations"), unique(self.annotations.iter().map(Annotation::id).collect()), "unique-annotation-ids");
        expect(out, &at(path, "legends"), unique(self.legends.iter().map(|l| l.id.as_str()).collect()), "unique-legend-ids");
        expect(out, &at(path, "layers"), unique(self.layers.iter().map(|l| l.id.as_str()).collect()), "unique-layer-ids");
        let layers: BTreeSet<_> = self.layers.iter().map(|l| l.id.as_str()).collect();
        expect(out, &at(path, "entities"), self.entities.iter().all(|e| layers.contains(e.layer.as_str())), "layer-exists");
        expect(out, &at(path, "tracks"), unique(self.tracks.iter().map(|t| t.id.as_str()).collect()), "unique-track-ids");
        let owned = |kind: OwnedKind, known: BTreeSet<&str>| self.owners.iter().filter(|o| o.kind == kind).all(|o| known.contains(o.id.as_str()));
        expect(out, &at(path, "owners"), owned(OwnedKind::Annotation, self.annotations.iter().map(Annotation::id).collect()) && owned(OwnedKind::Track, self.tracks.iter().map(|t| t.id.as_str()).collect()), "owner-subject-exists");
        expect(out, &at(path, "owners"), self.owners.iter().map(|o| (o.kind == OwnedKind::Track, o.id.as_str())).collect::<BTreeSet<_>>().len() == self.owners.len(), "unique-owner-subjects");
        expect(out, &at(path, "tracks"), self.tracks.iter().flat_map(|t| &t.keyframes).filter_map(|k| k.layer.as_deref()).all(|l| layers.contains(l)), "track-layer-exists");
    }
}
