//! 🎞️ Animate app engine facet: 📷️camera (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES:
//! relocated verbatim from the deleted artifact-tree `⚙️engine/📷️camera`).

#![allow(clippy::too_many_arguments, clippy::type_complexity)]

#[allow(clippy::module_inception)]
pub mod camera {
    //! 📷️ Scene cameras: static, moving, 3D, and zoomed views.

    use crate::editor::animate::engine::text::color::Color;
    use geometry::{Affine, Point, Vec2};

    /// 📸️ Base camera framing the scene.
    #[derive(Clone, Debug)]
    pub struct Camera {
        pub frame_center: Point,
        pub frame_width: f64,
        pub frame_height: f64,
        pub background: Color,
        pub transform: Affine,
    }

    impl Default for Camera {
        fn default() -> Self {
            Self { frame_center: Point::ZERO, frame_width: 14.0, frame_height: 8.0, background: Color::BLACK, transform: Affine::IDENTITY }
        }
    }

    impl Camera {
        pub fn new(frame_width: f64, frame_height: f64) -> Self {
            Self { frame_width, frame_height, ..Self::default() }
        }

        pub fn pixel_coords_to_scene(&self, px: f64, py: f64, pixel_width: u32, pixel_height: u32) -> Point {
            let u = px / pixel_width as f64;
            let v = 1.0 - py / pixel_height as f64;
            let x = self.frame_center.x() + (u - 0.5) * self.frame_width;
            let y = self.frame_center.y() + (v - 0.5) * self.frame_height;
            self.transform * Point::new(x, y)
        }

        pub fn scene_to_pixel(&self, p: Point, pixel_width: u32, pixel_height: u32) -> (f64, f64) {
            let p = self.transform * p;
            let u = (p.x() - self.frame_center.x()) / self.frame_width + 0.5;
            let v = (p.y() - self.frame_center.y()) / self.frame_height + 0.5;
            (u * pixel_width as f64, (1.0 - v) * pixel_height as f64)
        }
    }

    /// 🎥️ Camera that can pan and zoom over time.
    #[derive(Clone, Debug)]
    pub struct MovingCamera {
        pub camera: Camera,
        pub target_center: Point,
        pub target_width: f64,
    }

    impl MovingCamera {
        pub fn new(camera: Camera) -> Self {
            let target_center = camera.frame_center;
            let target_width = camera.frame_width;
            Self { camera, target_center, target_width }
        }

        pub fn interpolate(&mut self, alpha: f64) {
            let a = alpha.clamp(0.0, 1.0);
            let c0 = self.camera.frame_center;
            let c1 = self.target_center;
            self.camera.frame_center = Point::new(c0.x() + (c1.x() - c0.x()) * a, c0.y() + (c1.y() - c0.y()) * a);
            self.camera.frame_width = self.camera.frame_width + (self.target_width - self.camera.frame_width) * a;
            self.camera.frame_height = self.camera.frame_width * self.camera.frame_height / self.camera.frame_width.max(1e-9);
        }

        pub fn set_target(&mut self, center: Point, width: f64) {
            self.target_center = center;
            self.target_width = width;
        }
    }

    /// 🧊️ Perspective camera for 3D scenes.
    #[derive(Clone, Debug)]
    pub struct ThreeDCamera {
        pub camera: Camera,
        pub phi: f64,
        pub theta: f64,
        pub distance: f64,
        pub gamma: f64,
    }

    impl ThreeDCamera {
        pub fn new(camera: Camera) -> Self {
            Self { camera, phi: 0.0, theta: -std::f64::consts::FRAC_PI_2, distance: 10.0, gamma: 0.0 }
        }

        pub fn project(&self, x: f64, y: f64, z: f64) -> Point {
            let cy = self.phi.cos();
            let sy = self.phi.sin();
            let ct = self.theta.cos();
            let st = self.theta.sin();
            let x1 = x * cy - z * sy;
            let z1 = x * sy + z * cy;
            let y1 = y * ct - z1 * st;
            let z2 = y * st + z1 * ct + self.distance;
            let scale = 1.0 / z2.max(0.1);
            Point::new(x1 * scale, y1 * scale)
        }
    }

    /// 🔍️ Picture-in-picture zoomed camera region.
    #[derive(Clone, Debug)]
    pub struct ZoomedCamera {
        pub camera: Camera,
        pub zoom_factor: f64,
        pub display_corner: Vec2,
        pub display_size: (f64, f64),
    }

    impl ZoomedCamera {
        pub fn new(camera: Camera, zoom_factor: f64) -> Self {
            Self { camera, zoom_factor, display_corner: Vec2::new(1.0, 1.0), display_size: (3.0, 2.0) }
        }

        pub fn effective_frame_width(&self) -> f64 {
            self.camera.frame_width / self.zoom_factor.max(1e-9)
        }
    }

    #[cfg(test)]
    include!("🧪️tests/🔬️camera-unit/🦀️.rs");
}

pub mod matrix {
    //! 🔢️ Matrix types as gridded Sobject groups.

    use crate::editor::animate::engine::geometry::geometry::rectangle;
    use crate::editor::animate::engine::scene::sobject::{arrange, Group, Sobject, Sobjects};
    use crate::editor::animate::engine::text::color::Color;
    use crate::editor::animate::engine::text::text::{MathText, Text};
    use geometry::{Point, Vec2};

    fn arrange_grid(group: &mut Group, rows: usize, cols: usize, cell_size: (f64, f64)) {
        if group.children.is_empty() || rows == 0 || cols == 0 {
            return;
        }
        let origin = group.children[0].center();
        for (idx, child) in group.children.iter_mut().enumerate() {
            let row = idx / cols;
            let col = idx % cols;
            let x = origin.x() + col as f64 * cell_size.0;
            let y = origin.y() - row as f64 * cell_size.1;
            child.move_to(Point::new(x, y));
        }
    }

    /// 📊️ Matrix of string entries with optional brackets.
    pub struct Matrix {
        pub group: Group,
        pub rows: usize,
        pub cols: usize,
    }

    impl Matrix {
        pub fn from_rows(rows: Vec<Vec<String>>, cell_size: (f64, f64), color: Color) -> Self {
            let nrows = rows.len();
            let ncols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
            let mut children: Vec<Sobjects> = Vec::new();
            for row in rows {
                for cell in row {
                    let t = Text::new(cell, color);
                    children.push((t.inner).into());
                }
            }
            let mut group = Group::new(children);
            arrange_grid(&mut group, nrows, ncols, cell_size);
            Self { group, rows: nrows, cols: ncols }
        }

        pub fn math(entries: &[&str], cell_size: (f64, f64), color: Color) -> Self {
            let children: Vec<Sobjects> = entries
                .iter()
                .map(|e| {
                    let m = MathText::new(*e, color);
                    (m.inner).into()
                })
                .collect();
            let cols = (entries.len() as f64).sqrt().ceil() as usize;
            let rows = entries.len().div_ceil(cols);
            let mut group = Group::new(children);
            arrange(&mut group, Vec2::new(1.0, 0.0), cell_size.0 * 0.15);
            Self { group, rows, cols }
        }

        pub fn with_brackets(mut self, color: Color, padding: f64) -> Self {
            let b = self.group.bounds();
            let w = b.width() + padding * 2.0;
            let h = b.height() + padding * 2.0;
            let c = b.center();
            let frame = rectangle(w, h, c, Color::TRANSPARENT, Some(color), 3.0);
            self.group.add_child((frame).into());
            self
        }
    }

    /// 📋️ Table with header row and body rows in a 2D grid.
    pub struct Table {
        pub group: Group,
        pub rows: usize,
        pub cols: usize,
    }

    impl Table {
        pub fn new(headers: Vec<String>, rows: &[Vec<String>], cell_size: (f64, f64), color: Color) -> Self {
            let ncols = headers.len().max(rows.iter().map(|r| r.len()).max().unwrap_or(0));
            let nrows = rows.len() + 1;
            let mut children: Vec<Sobjects> = Vec::new();
            for header in headers {
                children.push((Text::new(header, color).inner).into());
            }
            for row in rows {
                for cell in row {
                    children.push((Text::new(cell.clone(), color).inner).into());
                }
                let pad = ncols.saturating_sub(row.len());
                for _ in 0..pad {
                    children.push((Text::new("", color).inner).into());
                }
            }
            let mut group = Group::new(children);
            arrange_grid(&mut group, nrows, ncols, cell_size);
            Self { group, rows: nrows, cols: ncols }
        }

        pub fn with_frame(mut self, color: Color, padding: f64) -> Self {
            let b = self.group.bounds();
            let frame = rectangle(b.width() + padding * 2.0, b.height() + padding * 2.0, b.center(), Color::TRANSPARENT, Some(color), 2.0);
            self.group.add_child((frame).into());
            self
        }
    }

    /// 🧮️ Decimal matrix for numeric interpolation animations.
    #[derive(Clone, Debug)]
    pub struct DecimalMatrix {
        pub values: Vec<Vec<f64>>,
    }

    impl DecimalMatrix {
        pub fn new(values: Vec<Vec<f64>>) -> Self {
            Self { values }
        }

        pub fn lerp(&self, other: &Self, t: f64) -> Self {
            let rows = self.values.len().min(other.values.len());
            let mut out = Vec::with_capacity(rows);
            for r in 0..rows {
                let cols = self.values[r].len().min(other.values[r].len());
                let mut row = Vec::with_capacity(cols);
                for c in 0..cols {
                    let a = self.values[r][c];
                    let b = other.values[r][c];
                    row.push(a + (b - a) * t);
                }
                out.push(row);
            }
            Self { values: out }
        }

        pub fn to_matrix_sobject(&self, cell_size: (f64, f64), color: Color) -> Matrix {
            let rows: Vec<Vec<String>> = self.values.iter().map(|row| row.iter().map(|v| format!("{v:.2}")).collect()).collect();
            Matrix::from_rows(rows, cell_size, color)
        }
    }

    #[cfg(test)]
    include!("🧪️tests/🔬️matrix-unit/🦀️.rs");
}

//#region 🧩️ArtifactItems
pub mod artifact_items {
pub const PRESENTATION_DOCUMENT_SCHEMA: &str = "animate.presentation";
}
//#endregion 🧩️ArtifactItems

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
/// 🖼️ The transport URL of the shared demo figure and the crop of it the deck covers.
///
/// It names a REAL shipped asset: `🧰️framework/🔨️modules/🖼️assets/🖼️images/🏙️architecture/🏘️habitat-67.png`
/// (2560 × 1707), which `semioAssetsVitePlugin` serves — and a release build copies — at
/// `/🖼️assets/*` for every host that mounts this app (the `semio-tech play` grid, this plugin's own
/// playground, and the CDN bundle alike). The previous `/🖼️bauteilbörse.png` existed nowhere in the
/// repo, so every host answered the figure request with its SPA fallback (`200 text/html`) and the
/// booted deck rendered fifteen blank tiles — see the law
/// `the_demo_figure_source_names_a_shipped_asset`.
pub const DEMO_FIGURE_SRC: &str = "/🖼️assets/🖼️images/🏙️architecture/🏘️habitat-67.png";
}
//#endregion 🧩️ArtifactItems4

//#region 🧩️ArtifactItems5
pub mod artifact_items_5 {
/// 📐️ `DEMO_FIGURE_SRC`'s own pixel dimensions — the crop maths needs the physical aspect.
pub const DEMO_FIGURE_PIXEL_WIDTH: f64 = 2560.0;
}
//#endregion 🧩️ArtifactItems5

//#region 🧩️ArtifactItems6
pub mod artifact_items_6 {
pub const DEMO_FIGURE_PIXEL_HEIGHT: f64 = 1707.0;
}
//#endregion 🧩️ArtifactItems6

//#region 🧩️ArtifactItems7
pub mod artifact_items_7 {
use crate::{FigureTileFrame, FigureTileSource};
use crate::{DEMO_FIGURE_SRC};
use crate::{DEMO_FIGURE_PIXEL_WIDTH};
use crate::{DEMO_FIGURE_PIXEL_HEIGHT};

pub fn default_figure_tile_source() -> FigureTileSource {
    FigureTileSource { src: DEMO_FIGURE_SRC.into(), kind: "figure".into(), frame: FigureTileFrame { x: 0.127, y: 0.1, width: 0.746, height: 0.75 }, source_aspect: Some(DEMO_FIGURE_PIXEL_WIDTH / DEMO_FIGURE_PIXEL_HEIGHT), pdf_page: None }
}
}
//#endregion 🧩️ArtifactItems7

//#region 🧩️ArtifactItems10
pub mod artifact_items_10 {
use crate::{FigureTileDraft, FigureTileFrame, FigureTileSource};
use crate::{default_figure_tile_source};

//#region 🔖️PresentationBridge
/// 🕸️ Owned CHILD handle types for the composed `s.stdio.semio@v1/presentation`/`s.stdio.semio@v1/animation`
/// documents — ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (`animate→C:presentation,animation`):
/// the shared source figure + its named tile crops now live in the composed `presentation` child's
/// slide-deck structure instead of inline `source`/`tiles` fields on `PresentationSnapshot`. `animation`
/// is composed too, per the design mapping's `animate→C:presentation,animation` line, but carries no
/// content today — this artifact's persisted document has no time-based data at all (the Manim-class
/// scene/keyframe engine under `✏️editor/⚙️engine` constructs its scenes in Rust code at
/// render/export time, never from persisted document state — see `animation_child_handle`'s own doc
/// comment for the honest gap this leaves).
pub type PresentationChild = store::os_store::artifact_child::ArtifactChild<semio_s_artifact_stdio_semio_presentation::schema::snapshot::SemioPresentationSnapshot>;
pub type AnimationChild = store::os_store::artifact_child::ArtifactChild<semio_s_artifact_stdio_semio_animation::schema::snapshot::SemioAnimationSnapshot>;

/// 🌉 REAL bidirectional converter (forward half): `(source, tiles)` -> one composed
/// `SemioPresentationSnapshot`. `source` becomes the deck's single `SlideMaster` (id `"source"`, one
/// `Picture` shape spanning `source.frame`); each tile becomes its own `Slide` referencing that
/// master's image, with the tile's own `crop` as its `Picture` shape's frame and the tile's `name`
/// carried as the slide's own `notes` (the closest lossless slot presentation offers a per-slide
/// display string). `source.kind` is reused verbatim as `SlidePictureImage.mime` — not a real MIME
/// type, but presentation's `Picture` shape has no dedicated "kind" tag of its own, and reusing the
/// nearest string slot losslessly beats inventing a MIME taxonomy that would only need an unmapping
/// table right back. **Lossy**: `source.source_aspect`/`source.pdf_page` have no representable slot
/// in `presentation`'s schema at all and are dropped by this forward conversion — every in-process
/// mutation round-trip still preserves them exactly via the working-scene cache below (never routed
/// through this lossy projection), so this only matters for a genuinely fresh reload with an empty
/// cache, the same class of documented gap every `UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` exemplar
/// (lowpoly/cad/writer) has left for its own composed slot.
pub fn presentation_snapshot_from_source_tiles(source: &FigureTileSource, tiles: &[FigureTileDraft]) -> semio_s_artifact_stdio_semio_presentation::schema::snapshot::SemioPresentationSnapshot {
    use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
    use semio_s_artifact_stdio_semio_document::schema::snapshot::DocBlock;
    use semio_s_artifact_stdio_semio_presentation::schema::snapshot::{SemioPresentationSnapshot, Slide, SlideFrame, SlideMaster, SlidePictureImage, SlideShape, STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA};

    const SOURCE_MASTER_ID: &str = "source";
    let frame_of = |frame: &FigureTileFrame| SlideFrame { origin: SemioPoint2 { x: frame.x, y: frame.y }, width: frame.width, height: frame.height };
    let image_of = || SlidePictureImage { asset_id: source.src.clone(), mime: source.kind.clone(), bytes: Vec::new() };

    let master = SlideMaster { id: SOURCE_MASTER_ID.into(), shapes: vec![SlideShape::Picture { frame: frame_of(&source.frame), image: image_of() }] };
    let slides = tiles.iter().map(|tile| Slide { id: tile.id.clone(), layout_id: None, shapes: vec![SlideShape::Picture { frame: frame_of(&tile.crop), image: image_of() }], notes: vec![DocBlock::paragraph(tile.name.clone())] }).collect();
    SemioPresentationSnapshot { schema: STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA.into(), masters: vec![master], layouts: Vec::new(), slides }
}

/// 🌉 Inverse of [`presentation_snapshot_from_source_tiles`] — reads the `"source"` master's first
/// `Picture` shape back into a `FigureTileSource` (honestly `source_aspect: None`, `pdf_page: None` —
/// see the forward converter's doc comment) and each `Slide`'s first `Picture` shape + its `notes`'s
/// first paragraph text back into a `FigureTileDraft`. A master/slide with no `Picture` shape at all
/// (never produced by the forward converter, but a composed child can in principle arrive from
/// elsewhere) falls back to `default_figure_tile_source()`/an empty name rather than panicking.
pub fn source_tiles_from_presentation_snapshot(snapshot: &semio_s_artifact_stdio_semio_presentation::schema::snapshot::SemioPresentationSnapshot) -> (FigureTileSource, Vec<FigureTileDraft>) {
    use semio_s_artifact_stdio_semio_document::schema::snapshot::DocBlock;
    use semio_s_artifact_stdio_semio_presentation::schema::snapshot::SlideShape;

    fn frame_from(shapes: &[SlideShape]) -> Option<(FigureTileFrame, String, String)> {
        shapes.iter().find_map(|shape| match shape {
            SlideShape::Picture { frame, image } => Some((FigureTileFrame { x: frame.origin.x, y: frame.origin.y, width: frame.width, height: frame.height }, image.asset_id.clone(), image.mime.clone())),
            _ => None,
        })
    }
    fn text_from_notes(notes: &[DocBlock]) -> String {
        notes
            .iter()
            .find_map(|block| match block {
                DocBlock::Paragraph { runs, .. } => Some(runs.iter().map(|run| run.text.as_str()).collect::<String>()),
                _ => None,
            })
            .unwrap_or_default()
    }

    let source = snapshot.masters.first().and_then(|master| frame_from(&master.shapes)).map_or_else(default_figure_tile_source, |(frame, src, kind)| FigureTileSource { src, kind, frame, source_aspect: None, pdf_page: None });

    let tiles = snapshot
        .slides
        .iter()
        .map(|slide| {
            let crop = frame_from(&slide.shapes).map_or(FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 }, |(frame, ..)| frame);
            FigureTileDraft { id: slide.id.clone(), name: text_from_notes(&slide.notes), crop }
        })
        .collect();
    (source, tiles)
}

/// 🕸️ Deterministic content-addressed CHILD handle for the composed `presentation` deck — same
/// `(child_id, target)` for identical `(source, tiles)`, a different pair once the content actually
/// changes, mirroring lowpoly's `mesh_child_handle`/writer's `document_child_handle`.
pub fn presentation_child_handle(source: &FigureTileSource, tiles: &[FigureTileDraft]) -> PresentationChild {
    let content = presentation_snapshot_from_source_tiles(source, tiles);
    let content_json = semio_framework_pack_json::to_json_string(&content);
    let child_id = store::content_id("presentation", content_json.as_bytes());
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "presentation".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target)
}

/// 🕸️ Deterministic content-addressed CHILD handle for the composed `animation` set. Always the SAME
/// handle today (content is always the empty default `SemioAnimationSnapshot`) — honest reflection of
/// the fact that nothing in this plugin yet produces per-tile keyframe/timeline data; composed per the
/// design mapping's `animate→C:presentation,animation` line so the slot exists for a future wave (a
/// natural extension: per-tile camera-pan/transition timing) without another schema migration.
pub fn animation_child_handle() -> AnimationChild {
    use semio_s_artifact_stdio_semio_animation::schema::snapshot::SemioAnimationSnapshot;
    let content_json = semio_framework_pack_json::to_json_string(&SemioAnimationSnapshot::default());
    let child_id = store::content_id("animation", content_json.as_bytes());
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "animation".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target)
}
//#endregion 🔖️PresentationBridge
}
//#endregion 🧩️ArtifactItems10

//#region 🧩️ArtifactItems11
pub mod artifact_items_11 {
use crate::{FigureTileDraft, FigureTileSource};
use crate::{PRESENTATION_DOCUMENT_SCHEMA};
use crate::{animation_child_handle, presentation_child_handle, presentation_snapshot_from_source_tiles};
pub use crate::snapshot::schema::PresentationSnapshot;

//#region 🔖️WorkingScene
// 🌱 The composed `presentation` child's live `(source, tiles)` content lives in the parent's own
// PERSISTED `PresentationSnapshot::source`/`tiles` payload fields (see their doc comments), never in a
// process-global cache: a cache is invisible to `ArtifactPack`, so every whole-document load through
// the host's member-less archive door used to derive an EMPTY deck.

/// 🔎 Reads the current document's live `(source, tiles)` — the single read call site every
/// mutation/render/export/inference path in this plugin uses.
pub fn presentation_working_scene(snapshot: &PresentationSnapshot) -> (FigureTileSource, Vec<FigureTileDraft>) {
    (snapshot.source.clone(), snapshot.tiles.clone())
}

/// 🌐️ World units per normalized figure unit on every Presentation canvas (editor and viewer alike).
pub const PRESENTATION_CANVAS_SCALE: f64 = 1000.0;

/// 🎥️ The world extent a Presentation canvas frames at first paint: the narrowest window pane the
/// play grid measures (469 px, see `🏛️architect`'s graph-fit law) less a margin, so the whole deck is in
/// view in every host before anyone pans.
pub const PRESENTATION_CANVAS_FIT_EXTENT: f64 = 420.0;

/// 🎥️ The first-paint camera `(x, y, zoom)` of a Presentation canvas. A Canvas2d camera names the VIEW
/// CENTRE (`Canvas2dHost`'s `worldToScreen`), so a fixed `(0, 0)` put the deck's top-left corner at the
/// middle of the pane and clipped everything past it. This centres the union of the source frame and
/// every tile crop and zooms it down to [`PRESENTATION_CANVAS_FIT_EXTENT`], never magnifying past 1:1.
pub fn presentation_canvas_camera(snapshot: &PresentationSnapshot) -> (f64, f64, f64) {
    let frames = std::iter::once(&snapshot.source.frame).chain(snapshot.tiles.iter().map(|tile| &tile.crop));
    let (min_x, min_y, max_x, max_y) = frames.fold((f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY), |(min_x, min_y, max_x, max_y), frame| {
        let (x, y) = (frame.x * PRESENTATION_CANVAS_SCALE, frame.y * PRESENTATION_CANVAS_SCALE);
        (min_x.min(x), min_y.min(y), max_x.max(x + frame.width * PRESENTATION_CANVAS_SCALE), max_y.max(y + frame.height * PRESENTATION_CANVAS_SCALE))
    });
    let extent = (max_x - min_x).max(max_y - min_y);
    let zoom = if extent > PRESENTATION_CANVAS_FIT_EXTENT { PRESENTATION_CANVAS_FIT_EXTENT / extent } else { 1.0 };
    ((min_x + max_x) * 0.5, (min_y + max_y) * 0.5, zoom)
}

/// 🏗️ Builds a full `PresentationSnapshot` from literal `(source, tiles)` — the standard
/// fixture/import constructor: it persists the payload AND mints the content-addressed `presentation`
/// handle that payload derives, so the two can never drift apart.
pub fn presentation_snapshot_with_tiles(source: &FigureTileSource, tiles: &[FigureTileDraft]) -> PresentationSnapshot {
    PresentationSnapshot { schema: PRESENTATION_DOCUMENT_SCHEMA.into(), source: source.clone(), tiles: tiles.to_vec(), presentation: presentation_child_handle(source, tiles), animation: animation_child_handle() }
}

/// 🌱️ `ArtifactEditor`/`ArtifactViewer::genesis_child_pack` for the composed `presentation` deck
/// (projected from the handle's working `(source, tiles)`) and the empty `animation` set. The react
/// shell's `loadDocumentPair` sends `members: []`, so a whole-document load (`setActiveExample` →
/// `Effect::LoadDocument`) derives both slots here; without it the archive closure completes
/// `Incomplete` and the load is refused (`document-archive-replacement.closure-rejected`).
pub fn genesis_presentation_child_pack(snapshot: &PresentationSnapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
    use semio_s_artifact_stdio_semio_animation::schema::snapshot::SemioAnimationSnapshot;
    use semio_s_artifact_stdio_semio_presentation::schema::snapshot::SemioPresentationSnapshot;
    use pack::snapshot::ArtifactPack;
    match slot {
        "presentation" if child_id == snapshot.presentation.child_id => {
            let (source, tiles) = presentation_working_scene(snapshot);
            Some(<SemioPresentationSnapshot as ArtifactPack>::encode_pack(&presentation_snapshot_from_source_tiles(&source, &tiles)))
        }
        "animation" if child_id == snapshot.animation.child_id => Some(<SemioAnimationSnapshot as ArtifactPack>::encode_pack(&SemioAnimationSnapshot::default())),
        _ => None,
    }
}
//#endregion 🔖️WorkingScene
}
//#endregion 🧩️ArtifactItems11
