//! 🖼️ Application-neutral tile-based infinite canvas; extend via `CanvasExtension`.
#![allow(clippy::missing_errors_doc, reason = "Canvas bundle is internal infrastructure.")]

// #region 🔖Renderer
mod renderer {
    // #region 🏷️VelloBackend
    pub(super) mod vello_backend {
        pub use vello;
        pub use vello::kurbo;
        pub use vello::peniko;
        #[cfg(target_arch = "wasm32")]
        pub use vello::util;
        #[cfg(target_arch = "wasm32")]
        pub use vello::wgpu;
        pub use vello::Scene;
        pub use vello_svg;
        pub use vello_svg::usvg;
    }
    // #endregion 🏷️VelloBackend

    use mathematical_geometry::{Affine, ShapeRef};
    use std::sync::Arc as SharedArc;
    use vello_backend as backend;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Cap {
        Butt,
        Round,
        Square,
    }

    impl From<Cap> for backend::kurbo::Cap {
        fn from(value: Cap) -> Self {
            match value {
                Cap::Butt => Self::Butt,
                Cap::Round => Self::Round,
                Cap::Square => Self::Square,
            }
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Stroke(pub(crate) backend::kurbo::Stroke);

    impl Stroke {
        pub fn new(width: f64) -> Self {
            Self(backend::kurbo::Stroke::new(width))
        }
        pub fn set_dash_pattern(&mut self, pattern: Vec<f64>) {
            self.0.dash_pattern = pattern.into();
        }
        pub fn set_start_cap(&mut self, cap: Cap) {
            self.0.start_cap = cap.into();
        }
        pub fn set_end_cap(&mut self, cap: Cap) {
            self.0.end_cap = cap.into();
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Rgba8 {
        pub r: u8,
        pub g: u8,
        pub b: u8,
        pub a: u8,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Color(pub(crate) backend::peniko::Color);

    impl Color {
        pub fn new(rgba: [f32; 4]) -> Self {
            Self(backend::peniko::Color::new(rgba))
        }
        pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
            Self(backend::peniko::Color::from_rgba8(r, g, b, a))
        }
        pub fn to_rgba8(self) -> Rgba8 {
            let c = self.0.to_rgba8();
            Rgba8 { r: c.r, g: c.g, b: c.b, a: c.a }
        }
        pub fn components(self) -> [f32; 4] {
            self.0.components
        }
        pub fn multiply_alpha(self, alpha: f32) -> Self {
            Self(self.0.multiply_alpha(alpha))
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FillRule {
        NonZero,
        EvenOdd,
    }

    impl From<FillRule> for backend::peniko::Fill {
        fn from(value: FillRule) -> Self {
            match value {
                FillRule::NonZero => Self::NonZero,
                FillRule::EvenOdd => Self::EvenOdd,
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BlendMode {
        Normal,
        Multiply,
        Screen,
        Overlay,
        Darken,
        Lighten,
        ColorDodge,
        ColorBurn,
        HardLight,
        SoftLight,
        Difference,
        Exclusion,
        Hue,
        Saturation,
        Color,
        Luminosity,
    }

    impl From<BlendMode> for backend::peniko::Mix {
        fn from(value: BlendMode) -> Self {
            match value {
                BlendMode::Normal => Self::Normal,
                BlendMode::Multiply => Self::Multiply,
                BlendMode::Screen => Self::Screen,
                BlendMode::Overlay => Self::Overlay,
                BlendMode::Darken => Self::Darken,
                BlendMode::Lighten => Self::Lighten,
                BlendMode::ColorDodge => Self::ColorDodge,
                BlendMode::ColorBurn => Self::ColorBurn,
                BlendMode::HardLight => Self::HardLight,
                BlendMode::SoftLight => Self::SoftLight,
                BlendMode::Difference => Self::Difference,
                BlendMode::Exclusion => Self::Exclusion,
                BlendMode::Hue => Self::Hue,
                BlendMode::Saturation => Self::Saturation,
                BlendMode::Color => Self::Color,
                BlendMode::Luminosity => Self::Luminosity,
            }
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Paint {
        Solid(Color),
    }

    impl From<Color> for Paint {
        fn from(value: Color) -> Self {
            Self::Solid(value)
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct RasterImage(pub(crate) backend::peniko::ImageData);

    impl RasterImage {
        /// @emoji 🖼️ Builds an RGBA8 raster image for scene drawing.
        pub fn rgba8(width: u32, height: u32, data: SharedArc<Vec<u8>>) -> Self {
            Self(backend::peniko::ImageData { data: backend::peniko::Blob::new(data), format: backend::peniko::ImageFormat::Rgba8, alpha_type: backend::peniko::ImageAlphaType::Alpha, width, height })
        }
        pub fn clone_data(&self) -> Self {
            Self(self.0.clone())
        }
        pub fn width(&self) -> u32 {
            self.0.width
        }
        pub fn height(&self) -> u32 {
            self.0.height
        }
    }

    #[derive(Clone, Default)]
    pub struct Scene(pub(crate) backend::Scene);

    impl Scene {
        pub fn new() -> Self {
            Self(backend::Scene::new())
        }
        pub fn fill<'a>(&mut self, rule: FillRule, transform: Affine, paint: impl Into<Paint>, brush_transform: Option<Affine>, shape: impl Into<ShapeRef<'a>>) {
            let paint = paint.into();
            let brush_transform = brush_transform.map(|a| a.to_kurbo());
            let transform = transform.to_kurbo();
            match (paint, shape.into()) {
                (Paint::Solid(color), ShapeRef::Rect(s)) => self.0.fill(rule.into(), transform, color.0, brush_transform, &s.to_kurbo()),
                (Paint::Solid(color), ShapeRef::RoundedRect(s)) => self.0.fill(rule.into(), transform, color.0, brush_transform, &s.to_kurbo()),
                (Paint::Solid(color), ShapeRef::Circle(s)) => self.0.fill(rule.into(), transform, color.0, brush_transform, &s.to_kurbo()),
                (Paint::Solid(color), ShapeRef::Line(s)) => self.0.fill(rule.into(), transform, color.0, brush_transform, &s.to_kurbo()),
                (Paint::Solid(color), ShapeRef::Arc(s)) => self.0.fill(rule.into(), transform, color.0, brush_transform, &s.to_kurbo()),
                (Paint::Solid(color), ShapeRef::CubicBez(s)) => self.0.fill(rule.into(), transform, color.0, brush_transform, &s.to_kurbo()),
                (Paint::Solid(color), ShapeRef::BezPath(s)) => self.0.fill(rule.into(), transform, color.0, brush_transform, &s.to_kurbo()),
            }
        }
        pub fn stroke<'a>(&mut self, stroke: &Stroke, transform: Affine, paint: impl Into<Paint>, brush_transform: Option<Affine>, shape: impl Into<ShapeRef<'a>>) {
            let paint = paint.into();
            let brush_transform = brush_transform.map(|a| a.to_kurbo());
            let transform = transform.to_kurbo();
            match (paint, shape.into()) {
                (Paint::Solid(color), ShapeRef::Rect(s)) => self.0.stroke(&stroke.0, transform, color.0, brush_transform, &s.to_kurbo()),
                (Paint::Solid(color), ShapeRef::RoundedRect(s)) => self.0.stroke(&stroke.0, transform, color.0, brush_transform, &s.to_kurbo()),
                (Paint::Solid(color), ShapeRef::Circle(s)) => self.0.stroke(&stroke.0, transform, color.0, brush_transform, &s.to_kurbo()),
                (Paint::Solid(color), ShapeRef::Line(s)) => self.0.stroke(&stroke.0, transform, color.0, brush_transform, &s.to_kurbo()),
                (Paint::Solid(color), ShapeRef::Arc(s)) => self.0.stroke(&stroke.0, transform, color.0, brush_transform, &s.to_kurbo()),
                (Paint::Solid(color), ShapeRef::CubicBez(s)) => self.0.stroke(&stroke.0, transform, color.0, brush_transform, &s.to_kurbo()),
                (Paint::Solid(color), ShapeRef::BezPath(s)) => self.0.stroke(&stroke.0, transform, color.0, brush_transform, &s.to_kurbo()),
            }
        }
        pub fn draw_image(&mut self, image: &RasterImage, transform: Affine) {
            self.0.draw_image(&backend::peniko::ImageBrush::new(image.0.clone()), transform.to_kurbo());
        }
        pub fn append(&mut self, other: &Scene, transform: Option<Affine>) {
            self.0.append(&other.0, transform.map(|a| a.to_kurbo()));
        }
        pub fn push_layer<'a>(&mut self, rule: FillRule, blend: BlendMode, alpha: f32, transform: Affine, clip: impl Into<ShapeRef<'a>>) {
            let style: backend::peniko::Fill = rule.into();
            let blend: backend::peniko::Mix = blend.into();
            let transform = transform.to_kurbo();
            mathematical_geometry::with_shape_ref!(clip.into(), |s| {
                self.0.push_layer(style, blend, alpha, transform, &s.to_kurbo());
            });
        }
        pub fn pop_layer(&mut self) {
            self.0.pop_layer();
        }
        pub fn push_clip_layer<'a>(&mut self, rule: FillRule, transform: Affine, clip: impl Into<ShapeRef<'a>>) {
            let style: backend::peniko::Fill = rule.into();
            let transform = transform.to_kurbo();
            mathematical_geometry::with_shape_ref!(clip.into(), |s| {
                self.0.push_clip_layer(style, transform, &s.to_kurbo());
            });
        }
        pub fn is_empty(&self) -> bool {
            self.0.encoding().is_empty()
        }
        pub fn path_count(&self) -> usize {
            self.0.encoding().path_tags.len()
        }
        /// @emoji 🔓 Escape hatch exposing the raw `vello` encoding for callers that need path-tag-level introspection (e.g. LOD/label test assertions) beyond `is_empty`/`path_count`.
        pub fn encoding(&self) -> &vello_encoding::Encoding {
            self.0.encoding()
        }

        pub fn vello_scene(&self) -> &backend::Scene {
            &self.0
        }
    }

    /// @emoji 🏷️ Parsed SVG document for icon and label rasterization.
    pub struct SvgDocument(pub(crate) backend::usvg::Tree);

    impl SvgDocument {
        pub(crate) fn from_tree(tree: backend::usvg::Tree) -> Self {
            Self(tree)
        }

        /// @emoji 🏷️ Appends the SVG tree into a scene.
        pub fn append_to_scene(&self, scene: &mut Scene) {
            vello_svg::append_tree(&mut scene.0, &self.0);
        }
    }

    /// @emoji 🏷️ Appends a parsed SVG document into a scene.
    pub fn append_svg_document(scene: &mut Scene, doc: &SvgDocument) {
        doc.append_to_scene(scene);
    }
}

pub use mathematical_geometry::{append_shape_to_path, geom_sel, Affine, Arc, BezPath, Circle, CubicBez, Line, PathEl, Point, Rect, RoundedRect, RoundedRectRadii, ShapeRef, Vec2};
pub(crate) use renderer::vello_backend::usvg;
pub use renderer::{append_svg_document, BlendMode, Cap, Color, FillRule, Paint, RasterImage, Rgba8, Scene, Stroke, SvgDocument};
// #endregion 🔖Renderer

// #region ⚠️ Errors
/// @emoji 🚨 SVG-parse failures raised by canvas icon/label rendering.
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum CanvasError {
    /// @emoji 🏷️ SVG source failed to parse into a `usvg` tree.
    #[error("SVG parse failed: {0}")]
    SvgParse(String),
}
// #endregion ⚠️ Errors

pub mod theme {
    // #region theme
    //! @emoji 🎨 Default canvas paint helpers from centralized styling tokens.

    use crate::Color;
    use ui_styling::{appearance::AppearanceName, CANVAS_LIGHT};

    /// @emoji 🌈 Maps a linear-sRGB token color to `Color`.
    pub fn linear_color(rgba: [f32; 4]) -> Color {
        Color::new(rgba)
    }

    /// @emoji 🎨 Shared default clear color for graph board canvases.
    pub fn default_raster_clear() -> Color {
        linear_color(CANVAS_LIGHT.raster_clear)
    }

    /// @emoji 🎨 Default themed icon foreground paint.
    pub fn default_icon_fg() -> Color {
        linear_color(CANVAS_LIGHT.icon_fg)
    }

    /// @emoji 🎨 Default themed icon background paint.
    pub fn default_icon_bg() -> Color {
        linear_color(CANVAS_LIGHT.icon_bg)
    }

    /// @emoji 🎨 Resolves canvas paints for a theme name.
    pub fn canvas_clear_for(theme: AppearanceName) -> Color {
        linear_color(theme.canvas().raster_clear)
    }

    /// @emoji 🌈 Parses an sRGB8888 JSON array into `Color`.
    pub fn color_from_json_rgba8(arr: &[serde_json::Value]) -> Option<Color> {
        let r = u8::try_from(arr.first()?.as_u64().unwrap_or(0).min(255)).ok()?;
        let g = u8::try_from(arr.get(1)?.as_u64().unwrap_or(0).min(255)).ok()?;
        let b = u8::try_from(arr.get(2)?.as_u64().unwrap_or(0).min(255)).ok()?;
        let a = u8::try_from(arr.get(3).and_then(|x| x.as_u64()).unwrap_or(255).min(255)).ok()?;
        Some(Color::from_rgba8(r, g, b, a))
    }

    /// @emoji 🎨 Merges one camelCase color field from a canvas theme JSON object.
    pub fn merge_color_field(next: &mut Color, v: &serde_json::Value, key: &str) {
        if let Some(arr) = v.get(key).and_then(|x| x.as_array()) {
            if let Some(c) = color_from_json_rgba8(arr) {
                *next = c;
            }
        }
    }

    /// @emoji 🌓 Returns whether a canvas clear color reads as a light background.
    pub fn clear_is_light(clear: Color) -> bool {
        let [r, g, b, _] = clear.components();
        0.2126 * f64::from(r) + 0.7152 * f64::from(g) + 0.0722 * f64::from(b) > 0.5
    }

    /// @emoji 🎨 Checkerboard cell shades for transparent raster layers.
    pub fn checkerboard_shades_for_clear(clear: Color) -> (u8, u8) {
        if clear_is_light(clear) {
            (220, 180)
        } else {
            (64, 48)
        }
    }
    // #endregion theme
}

// #region 🏷️IconAssets

pub mod icon_assets {
    //! @emoji 📎 Static bytes for icon rendering; `include_bytes!` paths are relative to this `lib.rs` file.

    pub static NOTO_COLOR_EMOJI_SUBSET_TTF: &[u8] = include_bytes!("asset/NotoColorEmoji-subset.ttf");

    pub static MAP_LABEL_SANS_TTF: &[u8] = include_bytes!("asset/MapLabelSans.ttf");
}

// #endregion 🏷️IconAssets

pub mod svg_icon {
    use std::sync::{Arc, OnceLock};

    use crate::usvg;
    use crate::{Affine, BezPath, Color, FillRule, Point, Scene, ShapeRef, Stroke};

    // #region 🔖IconUsvgParseOptions

    static ICON_USVG_OPTIONS: OnceLock<usvg::Options<'static>> = OnceLock::new();

    /// @emoji 🔤 Shared `usvg` parse options with bundled Noto Color Emoji so `<text>` in Typst `emoji:` SVG matches the Typst font book; avoids system fallback glyphs.
    pub fn usvg_options_icons() -> &'static usvg::Options<'static> {
        ICON_USVG_OPTIONS.get_or_init(|| {
            let mut db = fontdb::Database::new();
            db.load_font_data(super::icon_assets::NOTO_COLOR_EMOJI_SUBSET_TTF.to_vec());
            usvg::Options { fontdb: Arc::new(db), font_family: ui_styling::canvas_fonts::NOTO_COLOR_EMOJI.into(), ..Default::default() }
        })
    }

    // #endregion 🔖IconUsvgParseOptions

    fn to_affine(ts: &usvg::Transform) -> Affine {
        let usvg::Transform { sx, kx, ky, sy, tx, ty } = *ts;
        Affine::new([sx, ky, kx, sy, tx, ty].map(f64::from))
    }

    fn to_bez_path(path: &usvg::Path) -> BezPath {
        let mut local_path = BezPath::new();
        let mut just_closed = false;
        let mut most_recent_initial = (0_f64, 0_f64);
        for elt in path.data().segments() {
            match elt {
                usvg::tiny_skia_path::PathSegment::MoveTo(p) => {
                    if std::mem::take(&mut just_closed) {
                        local_path.move_to(most_recent_initial);
                    }
                    most_recent_initial = (p.x.into(), p.y.into());
                    local_path.move_to(most_recent_initial);
                }
                usvg::tiny_skia_path::PathSegment::LineTo(p) => {
                    if std::mem::take(&mut just_closed) {
                        local_path.move_to(most_recent_initial);
                    }
                    local_path.line_to(Point::new(p.x as f64, p.y as f64));
                }
                usvg::tiny_skia_path::PathSegment::QuadTo(p1, p2) => {
                    if std::mem::take(&mut just_closed) {
                        local_path.move_to(most_recent_initial);
                    }
                    local_path.quad_to(Point::new(p1.x as f64, p1.y as f64), Point::new(p2.x as f64, p2.y as f64));
                }
                usvg::tiny_skia_path::PathSegment::CubicTo(p1, p2, p3) => {
                    if std::mem::take(&mut just_closed) {
                        local_path.move_to(most_recent_initial);
                    }
                    local_path.curve_to(Point::new(p1.x as f64, p1.y as f64), Point::new(p2.x as f64, p2.y as f64), Point::new(p3.x as f64, p3.y as f64));
                }
                usvg::tiny_skia_path::PathSegment::Close => {
                    just_closed = true;
                    local_path.close_path();
                }
            }
        }
        local_path
    }

    fn map_solid_icon_paint(paint: &usvg::Paint, opacity: usvg::Opacity, fg: Color, bg: Color) -> Option<Color> {
        let usvg::Paint::Color(c) = paint else {
            return None;
        };
        let a = opacity.get();
        if c.red < 22 && c.green < 22 && c.blue < 22 {
            return Some(fg.multiply_alpha(a));
        }
        if c.red > 233 && c.green > 233 && c.blue > 233 {
            return Some(bg.multiply_alpha(a));
        }
        Some(Color::from_rgba8(c.red, c.green, c.blue, opacity.to_u8()))
    }

    fn stroke_path(scene: &mut Scene, path: &usvg::Path, transform: Affine, local_path: &BezPath, fg: Color, bg: Color) {
        if let Some(stroke) = path.stroke() {
            if let Some(color) = map_solid_icon_paint(stroke.paint(), stroke.opacity(), fg, bg) {
                let conv = Stroke::new(f64::from(stroke.width().get()));
                scene.stroke(&conv, transform, color, None, ShapeRef::BezPath(local_path));
            }
        }
    }

    fn fill_path(scene: &mut Scene, path: &usvg::Path, transform: Affine, local_path: &BezPath, fg: Color, bg: Color) {
        if let Some(fill) = path.fill() {
            if let Some(color) = map_solid_icon_paint(fill.paint(), fill.opacity(), fg, bg) {
                scene.fill(
                    match fill.rule() {
                        usvg::FillRule::NonZero => FillRule::NonZero,
                        usvg::FillRule::EvenOdd => FillRule::EvenOdd,
                    },
                    transform,
                    color,
                    None,
                    ShapeRef::BezPath(local_path),
                );
            }
        }
    }

    fn render_path(scene: &mut Scene, path: &usvg::Path, fg: Color, bg: Color, stroke_first: bool) {
        if !path.is_visible() {
            return;
        }
        let transform = to_affine(&path.abs_transform());
        let local_path = to_bez_path(path);
        if stroke_first {
            stroke_path(scene, path, transform, &local_path, fg, bg);
            fill_path(scene, path, transform, &local_path, fg, bg);
        } else {
            fill_path(scene, path, transform, &local_path, fg, bg);
            stroke_path(scene, path, transform, &local_path, fg, bg);
        }
    }

    fn render_group(scene: &mut Scene, group: &usvg::Group, fg: Color, bg: Color, stroke_first: bool) {
        for node in group.children() {
            match node {
                usvg::Node::Group(g) => render_group(scene, g, fg, bg, stroke_first),
                usvg::Node::Path(path) => render_path(scene, path, fg, bg, stroke_first),
                usvg::Node::Text(t) => render_group(scene, t.flattened(), fg, bg, true),
                _ => {}
            }
        }
    }

    fn literal_paint(paint: &usvg::Paint, opacity: usvg::Opacity) -> Option<Color> {
        let usvg::Paint::Color(c) = paint else {
            return None;
        };
        Some(Color::from_rgba8(c.red, c.green, c.blue, opacity.to_u8()))
    }

    fn stroke_path_literal(scene: &mut Scene, path: &usvg::Path, transform: Affine, local_path: &BezPath) {
        if let Some(stroke) = path.stroke() {
            if let Some(color) = literal_paint(stroke.paint(), stroke.opacity()) {
                let conv = Stroke::new(f64::from(stroke.width().get()));
                scene.stroke(&conv, transform, color, None, ShapeRef::BezPath(local_path));
            }
        }
    }

    fn fill_path_literal(scene: &mut Scene, path: &usvg::Path, transform: Affine, local_path: &BezPath) {
        if let Some(fill) = path.fill() {
            if let Some(color) = literal_paint(fill.paint(), fill.opacity()) {
                scene.fill(
                    match fill.rule() {
                        usvg::FillRule::NonZero => FillRule::NonZero,
                        usvg::FillRule::EvenOdd => FillRule::EvenOdd,
                    },
                    transform,
                    color,
                    None,
                    ShapeRef::BezPath(local_path),
                );
            }
        }
    }

    fn render_path_literal(scene: &mut Scene, path: &usvg::Path, stroke_first: bool) {
        if !path.is_visible() {
            return;
        }
        let transform = to_affine(&path.abs_transform());
        let local_path = to_bez_path(path);
        if stroke_first {
            stroke_path_literal(scene, path, transform, &local_path);
            fill_path_literal(scene, path, transform, &local_path);
        } else {
            fill_path_literal(scene, path, transform, &local_path);
            stroke_path_literal(scene, path, transform, &local_path);
        }
    }

    fn render_group_literal(scene: &mut Scene, group: &usvg::Group, stroke_first: bool) {
        for node in group.children() {
            match node {
                usvg::Node::Group(g) => render_group_literal(scene, g, stroke_first),
                usvg::Node::Path(path) => render_path_literal(scene, path, stroke_first),
                usvg::Node::Text(t) => render_group_literal(scene, t.flattened(), true),
                _ => {}
            }
        }
    }

    /// @emoji 🏷️ Renders SVG tree paints literally (no icon fg/bg remapping); used for map labels.
    pub fn render_svg_tree_literal(scene: &mut Scene, tree: &usvg::Tree) {
        render_group_literal(scene, tree.root(), false);
    }

    fn icon_rect_xywh(r: usvg::Rect) -> Option<(f64, f64, f64, f64)> {
        let w = f64::from(r.width());
        let h = f64::from(r.height());
        if !(w > 1e-6 && h > 1e-6 && w.is_finite() && h.is_finite()) {
            return None;
        }
        Some((f64::from(r.x()), f64::from(r.y()), w, h))
    }

    fn icon_rect_nonzero(r: usvg::tiny_skia_path::NonZeroRect) -> (f64, f64, f64, f64) {
        (f64::from(r.x()), f64::from(r.y()), f64::from(r.width()), f64::from(r.height()))
    }

    fn icon_union_xywh(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
        let ax1 = a.0 + a.2;
        let ay1 = a.1 + a.3;
        let bx1 = b.0 + b.2;
        let by1 = b.1 + b.3;
        let x0 = a.0.min(b.0);
        let y0 = a.1.min(b.1);
        let x1 = ax1.max(bx1);
        let y1 = ay1.max(by1);
        (x0, y0, x1 - x0, y1 - y0)
    }

    fn icon_union_rects_into(acc: &mut Option<(f64, f64, f64, f64)>, r: usvg::Rect) {
        if let Some(xy) = icon_rect_xywh(r) {
            *acc = Some(match acc.take() {
                None => xy,
                Some(a) => icon_union_xywh(a, xy),
            });
        }
    }

    fn icon_visit_node_bounds(node: &usvg::Node, acc: &mut Option<(f64, f64, f64, f64)>) {
        match node {
            usvg::Node::Group(g) => {
                for c in g.children() {
                    icon_visit_node_bounds(c, acc);
                }
            }
            usvg::Node::Path(p) => {
                if !p.is_visible() {
                    return;
                }
                icon_union_rects_into(acc, p.abs_bounding_box());
                icon_union_rects_into(acc, p.abs_stroke_bounding_box());
            }
            usvg::Node::Image(img) => {
                if !img.is_visible() {
                    return;
                }
                icon_union_rects_into(acc, img.abs_bounding_box());
            }
            usvg::Node::Text(t) => {
                icon_union_rects_into(acc, t.abs_bounding_box());
                icon_union_rects_into(acc, t.abs_stroke_bounding_box());
            }
        }
    }

    /// @emoji 📐 Union of visible paint bounds (paths, raster images, text) in absolute SVG space for uniform scale-and-center fits.
    pub fn svg_icon_content_bounds(tree: &usvg::Tree) -> (f64, f64, f64, f64) {
        let mut acc = None::<(f64, f64, f64, f64)>;
        for c in tree.root().children() {
            icon_visit_node_bounds(c, &mut acc);
        }
        if let Some(u) = acc {
            let (_, _, bw, bh) = u;
            if bw > 1e-6 && bh > 1e-6 {
                return u;
            }
        }
        let root = tree.root();
        let mut u = icon_rect_nonzero(root.abs_layer_bounding_box());
        if let Some(r) = icon_rect_xywh(root.abs_stroke_bounding_box()) {
            u = icon_union_xywh(u, r);
        }
        if let Some(r) = icon_rect_xywh(root.abs_bounding_box()) {
            u = icon_union_xywh(u, r);
        }
        let (_, _, bw, bh) = u;
        if bw > 1e-6 && bh > 1e-6 {
            return u;
        }
        let w = f64::from(tree.size().width());
        let h = f64::from(tree.size().height());
        (0.0, 0.0, w.max(1.0), h.max(1.0))
    }

    pub fn render_svg_tree_themed(scene: &mut Scene, tree: &usvg::Tree, fg: Color, bg: Color) {
        render_group(scene, tree.root(), fg, bg, false);
    }

    /// @emoji 🏷️ Parses SVG source and renders it themed into `scene`.
    pub fn append_svg_str_themed(scene: &mut Scene, svg: &str, fg: Color, bg: Color) -> Result<(), crate::CanvasError> {
        let tree = usvg::Tree::from_str(svg, usvg_options_icons()).map_err(|e| crate::CanvasError::SvgParse(e.to_string()))?;
        render_svg_tree_themed(scene, &tree, fg, bg);
        Ok(())
    }

    /// @emoji 🏷️ Parses SVG source and renders it with the default icon theme into `scene`.
    pub fn append_svg_str(scene: &mut Scene, svg: &str) -> Result<(), crate::CanvasError> {
        append_svg_str_themed(scene, svg, crate::theme::default_icon_fg(), crate::theme::default_icon_bg())
    }

    /// @emoji 📐 Parses SVG and returns visible content bounds in absolute SVG space.
    pub fn svg_icon_content_bounds_from_str(svg: &str) -> Result<(f64, f64, f64, f64), crate::CanvasError> {
        let tree = usvg::Tree::from_str(svg, usvg_options_icons()).map_err(|e| crate::CanvasError::SvgParse(e.to_string()))?;
        Ok(svg_icon_content_bounds(&tree))
    }
}

impl SvgDocument {
    /// @emoji 🏷️ Parses icon SVG with bundled emoji font options.
    pub fn parse_icons(svg: &str) -> Result<Self, CanvasError> {
        let tree = usvg::Tree::from_str(svg, svg_icon::usvg_options_icons()).map_err(|e| CanvasError::SvgParse(e.to_string()))?;
        Ok(Self::from_tree(tree))
    }

    /// @emoji 📐 Visible content bounds in absolute SVG space.
    pub fn content_bounds(&self) -> (f64, f64, f64, f64) {
        svg_icon::svg_icon_content_bounds(&self.0)
    }

    /// @emoji 🏷️ Renders themed icon paints into a scene.
    pub fn render_themed(&self, scene: &mut Scene, fg: Color, bg: Color) {
        svg_icon::render_svg_tree_themed(scene, &self.0, fg, bg);
    }

    /// @emoji 🏷️ Renders literal SVG paints into a scene.
    pub fn render_literal(&self, scene: &mut Scene) {
        svg_icon::render_svg_tree_literal(scene, &self.0);
    }
}

// #region 🔖Text
pub mod text {
    use std::sync::{Arc, OnceLock};

    use crate::svg_icon::render_svg_tree_literal;
    use crate::usvg;
    use crate::{Affine, Color, Point, Scene, Vec2};

    static MAP_LABEL_USVG_OPTIONS: OnceLock<usvg::Options<'static>> = OnceLock::new();

    /// @emoji 🔤 `usvg` options with bundled map label sans for place-name labels.
    pub fn usvg_options_map_labels() -> &'static usvg::Options<'static> {
        MAP_LABEL_USVG_OPTIONS.get_or_init(|| {
            let mut db = fontdb::Database::new();
            db.load_font_data(super::icon_assets::MAP_LABEL_SANS_TTF.to_vec());
            let family = db.faces().next().and_then(|face| face.families.first().map(|(name, _)| name.clone())).unwrap_or_else(|| ui_styling::canvas_fonts::MAP_LABEL_SANS_FALLBACK.into());
            usvg::Options { fontdb: Arc::new(db), font_family: family, ..Default::default() }
        })
    }

    fn escape_xml_attr(s: &str) -> String {
        s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
    }

    fn color_to_svg(c: Color) -> String {
        let rgba = c.to_rgba8();
        if rgba.a == 255 {
            format!("#{:02x}{:02x}{:02x}", rgba.r, rgba.g, rgba.b)
        } else {
            let a = f64::from(rgba.a) / 255.0;
            format!("rgba({},{},{},{a})", rgba.r, rgba.g, rgba.b)
        }
    }

    /// @emoji 📐 Estimated label box size in screen px for layout (matches `append_label` padding).
    pub fn label_extent(label: &str, px: f64) -> (f64, f64) {
        let trimmed = label.trim();
        if trimmed.is_empty() || px < ui_styling::metrics::label::MIN_PX {
            return (0.0, 0.0);
        }
        let pad = px * ui_styling::metrics::label::PAD_RATIO;
        let w = (trimmed.len() as f64 * px * ui_styling::metrics::label::CHAR_WIDTH_RATIO + pad * 2.0).clamp(ui_styling::metrics::label::WIDTH_MIN, ui_styling::metrics::label::WIDTH_MAX);
        let h = (px * ui_styling::metrics::label::HEIGHT_RATIO + pad * 2.0).clamp(ui_styling::metrics::label::HEIGHT_MIN, ui_styling::metrics::label::HEIGHT_MAX);
        (w, h)
    }

    /// @emoji ↔️ Horizontal text advance inside a label box (excludes outer padding).
    pub fn label_advance(label: &str, px: f64) -> f64 {
        if label.is_empty() || px < ui_styling::metrics::label::MIN_PX {
            return 0.0;
        }
        label.len() as f64 * px * ui_styling::metrics::label::CHAR_WIDTH_RATIO
    }

    /// @emoji 📏 Left inset from label origin to first glyph baseline start.
    pub fn label_text_inset(px: f64) -> f64 {
        if px < ui_styling::metrics::label::MIN_PX {
            return 0.0;
        }
        px * ui_styling::metrics::label::PAD_RATIO
    }

    #[derive(Clone, Copy, Debug)]
    struct LabelLineLayout {
        bx: f64,
        scale: f64,
        pad: f64,
    }

    fn label_line_layout(line: &str, px: f64) -> Option<LabelLineLayout> {
        if px < ui_styling::metrics::label::MIN_PX {
            return None;
        }
        let pad = px * ui_styling::metrics::label::PAD_RATIO;
        let extent_line = if line.is_empty() { " " } else { line };
        let (w, h) = label_extent(extent_line, px);
        let text_y = pad + px;
        let family = usvg_options_map_labels().font_family.clone();
        let body = if line.is_empty() { " " } else { line };
        let svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}"><text x="{pad}" y="{text_y}" font-size="{px}" font-family="{family}">{text}</text></svg>"##,
            w = w,
            h = h,
            pad = pad,
            text_y = text_y,
            px = px,
            family = escape_xml_attr(&family),
            text = escape_xml_attr(body),
        );
        let tree = usvg::Tree::from_str(&svg, usvg_options_map_labels()).ok()?;
        let (bx, _, bw, bh) = crate::svg_icon::svg_icon_content_bounds(&tree);
        if bw <= 0.0 || bh <= 0.0 {
            return None;
        }
        let scale = (px * ui_styling::metrics::label::SCALE_RATIO / bh).min(ui_styling::metrics::label::SCALE_MAX);
        Some(LabelLineLayout { bx, scale, pad })
    }

    fn label_prefix_advance_svg(line: &str, byte_end: usize, px: f64) -> f64 {
        let end = byte_end.min(line.len());
        if end == 0 {
            return 0.0;
        }
        if !line.is_char_boundary(end) {
            let prev = line[..end].char_indices().next_back().map_or(0, |(i, _)| i);
            return label_prefix_advance_svg(line, prev, px);
        }
        let prefix = &line[..end];
        let pad = px * ui_styling::metrics::label::PAD_RATIO;
        let (w, h) = label_extent(prefix, px);
        let text_y = pad + px;
        let family = usvg_options_map_labels().font_family.clone();
        let svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}"><text x="{pad}" y="{text_y}" font-size="{px}" font-family="{family}">{text}</text></svg>"##,
            w = w,
            h = h,
            pad = pad,
            text_y = text_y,
            px = px,
            family = escape_xml_attr(&family),
            text = escape_xml_attr(prefix),
        );
        let Ok(tree) = usvg::Tree::from_str(&svg, usvg_options_map_labels()) else {
            return label_advance(prefix, px);
        };
        let (bx, _, bw, bh) = crate::svg_icon::svg_icon_content_bounds(&tree);
        if bw <= 0.0 || bh <= 0.0 {
            return label_advance(prefix, px);
        }
        (bx + bw) - pad
    }

    /// @emoji ↔️ World x for a byte offset in a code line (matches `append_label_tspans` layout).
    pub fn label_byte_world_x(line: &str, byte_offset: usize, origin_x: f64, px: f64) -> f64 {
        let Some(layout) = label_line_layout(line, px) else {
            return origin_x;
        };
        let advance = label_prefix_advance_svg(line, byte_offset, px);
        origin_x + (layout.pad + advance - layout.bx) * layout.scale
    }

    /// @emoji ↔️ World x range for a byte span in a code line.
    pub fn label_span_world_x(line: &str, byte_start: usize, byte_end: usize, origin_x: f64, px: f64) -> (f64, f64) {
        (label_byte_world_x(line, byte_start, origin_x, px), label_byte_world_x(line, byte_end, origin_x, px))
    }

    /// @emoji 🏷️ Renders a single map label via SVG text at `origin` (screen px, baseline).
    pub fn append_label(scene: &mut Scene, label: &str, origin: Point, px: f64, fill: Color, halo: Color) {
        let trimmed = label.trim();
        if trimmed.is_empty() || px < ui_styling::metrics::label::MIN_PX {
            return;
        }
        let pad = px * ui_styling::metrics::label::PAD_RATIO;
        let (w, h) = label_extent(trimmed, px);
        let text_y = pad + px;
        let family = usvg_options_map_labels().font_family.clone();
        let svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}"><text x="{pad}" y="{text_y}" font-size="{px}" font-family="{family}" fill="{fill}" stroke="{halo}" stroke-width="{stroke}" paint-order="stroke">{text}</text></svg>"##,
            w = w,
            h = h,
            pad = pad,
            text_y = text_y,
            px = px,
            family = escape_xml_attr(&family),
            fill = color_to_svg(fill),
            halo = color_to_svg(halo),
            stroke = (px * ui_styling::metrics::label::HALO_STROKE_RATIO).max(ui_styling::metrics::label::HALO_STROKE_MIN),
            text = escape_xml_attr(trimmed),
        );
        let Ok(tree) = usvg::Tree::from_str(&svg, usvg_options_map_labels()) else {
            return;
        };
        let (bx, by, bw, bh) = crate::svg_icon::svg_icon_content_bounds(&tree);
        if bw <= 0.0 || bh <= 0.0 {
            return;
        }
        let scale = (px * ui_styling::metrics::label::SCALE_RATIO / bh).min(ui_styling::metrics::label::SCALE_MAX);
        let mut label_scene = Scene::new();
        render_svg_tree_literal(&mut label_scene, &tree);
        let aff = Affine::IDENTITY.translate(Vec2::new(origin.x() - bx * scale, origin.y() - by * scale - px * ui_styling::metrics::label::VERTICAL_OFFSET_RATIO)).scale(scale);
        scene.append(&label_scene, Some(aff));
    }

    /// @emoji 🏷️ Renders one label with colored inline tspans (single padding box, no per-span gaps).
    pub fn append_label_tspans(scene: &mut Scene, line: &str, spans: &[(usize, usize, Color)], origin: Point, px: f64, _halo: Color) {
        if line.is_empty() || spans.is_empty() || px < ui_styling::metrics::label::MIN_PX {
            return;
        }
        let pad = px * ui_styling::metrics::label::PAD_RATIO;
        let (w, h) = label_extent(line, px);
        let text_y = pad + px;
        let family = usvg_options_map_labels().font_family.clone();
        let mut inner = String::new();
        for &(start, end, fill) in spans {
            if start >= end || end > line.len() {
                continue;
            }
            let slice = &line[start..end];
            if slice.is_empty() {
                continue;
            }
            inner.push_str(&format!(r#"<tspan fill="{fill}">{text}</tspan>"#, fill = color_to_svg(fill), text = escape_xml_attr(slice),));
        }
        if inner.is_empty() {
            return;
        }
        let svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}"><text x="{pad}" y="{text_y}" font-size="{px}" font-family="{family}">{inner}</text></svg>"##,
            w = w,
            h = h,
            pad = pad,
            text_y = text_y,
            px = px,
            family = escape_xml_attr(&family),
            inner = inner,
        );
        let Ok(tree) = usvg::Tree::from_str(&svg, usvg_options_map_labels()) else {
            return;
        };
        let (bx, by, bw, bh) = crate::svg_icon::svg_icon_content_bounds(&tree);
        if bw <= 0.0 || bh <= 0.0 {
            return;
        }
        let scale = (px * ui_styling::metrics::label::SCALE_RATIO / bh).min(ui_styling::metrics::label::SCALE_MAX);
        let mut label_scene = Scene::new();
        render_svg_tree_literal(&mut label_scene, &tree);
        let aff = Affine::IDENTITY.translate(Vec2::new(origin.x() - bx * scale, origin.y() - by * scale - px * ui_styling::metrics::label::VERTICAL_OFFSET_RATIO)).scale(scale);
        scene.append(&label_scene, Some(aff));
    }
}
// #endregion 🔖Text

// #region 🔖Camera
pub mod camera {
    use crate::{Affine, Point};

    pub const CANVAS_CAMERA_ZOOM_MIN: f64 = ui_styling::metrics::camera::ZOOM_MIN;
    pub const CANVAS_CAMERA_ZOOM_MAX: f64 = ui_styling::metrics::camera::ZOOM_MAX;

    #[derive(Clone, Debug)]
    pub struct Camera {
        pub x: f64,
        pub y: f64,
        pub zoom: f64,
    }

    impl Default for Camera {
        fn default() -> Self {
            Self { x: 0.0, y: 0.0, zoom: 1.0 }
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Viewport {
        pub width: u32,
        pub height: u32,
        pub dpr: f64,
    }

    impl Default for Viewport {
        fn default() -> Self {
            Self { width: 1, height: 1, dpr: 1.0 }
        }
    }

    impl Viewport {
        pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
            self.width = width.max(1);
            self.height = height.max(1);
            self.dpr = dpr.max(1.0);
        }

        pub fn physical_size(&self) -> (u32, u32) {
            let pw = ((self.width as f64 * self.dpr).round() as u32).max(1);
            let ph = ((self.height as f64 * self.dpr).round() as u32).max(1);
            (pw, ph)
        }
    }

    pub fn clamp_zoom(zoom: f64) -> f64 {
        zoom.clamp(CANVAS_CAMERA_ZOOM_MIN, CANVAS_CAMERA_ZOOM_MAX)
    }

    pub fn world_to_screen(camera: &Camera, viewport: &Viewport, p: Point) -> Point {
        Point::new((p.x - camera.x) * camera.zoom + viewport.width as f64 / 2.0, (p.y - camera.y) * camera.zoom + viewport.height as f64 / 2.0)
    }

    pub fn screen_to_world(camera: &Camera, viewport: &Viewport, p: Point) -> Point {
        Point::new((p.x - viewport.width as f64 / 2.0) / camera.zoom + camera.x, (p.y - viewport.height as f64 / 2.0) / camera.zoom + camera.y)
    }

    pub fn camera_content_affine(camera: &Camera, viewport: &Viewport) -> Affine {
        let z = camera.zoom;
        Affine::new([z, 0.0, 0.0, z, viewport.width as f64 * 0.5 - camera.x * z, viewport.height as f64 * 0.5 - camera.y * z])
    }

    pub fn wheel_screen(camera: &mut Camera, viewport: &Viewport, sx: f64, sy: f64, delta_y: f64) {
        let zoom_factor = if delta_y < 0.0 { ui_styling::metrics::camera::WHEEL_ZOOM_IN_FACTOR } else { ui_styling::metrics::camera::WHEEL_ZOOM_OUT_FACTOR };
        let next_zoom = clamp_zoom(camera.zoom * zoom_factor);
        let screen = Point::new(sx, sy);
        let world_before = screen_to_world(camera, viewport, screen);
        camera.x = world_before.x - (sx - viewport.width as f64 / 2.0) / next_zoom;
        camera.y = world_before.y - (sy - viewport.height as f64 / 2.0) / next_zoom;
        camera.zoom = next_zoom;
    }
}
// #endregion 🔖Camera

// #region 🔖Lod
pub mod lod {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Lod {
        pub id: &'static str,
        pub name: &'static str,
        pub description: &'static str,
        pub max_zoom: f64,
    }

    #[derive(Clone, Copy, Debug)]
    pub struct LodScale {
        pub lods: &'static [Lod],
    }

    impl LodScale {
        pub fn resolve_index(self, zoom: f64) -> usize {
            for (i, lod) in self.lods.iter().enumerate() {
                if zoom < lod.max_zoom {
                    return i;
                }
            }
            self.lods.len().saturating_sub(1)
        }

        pub fn resolve(self, zoom: f64) -> &'static Lod {
            &self.lods[self.resolve_index(zoom)]
        }

        pub fn index_of(self, id: &str) -> Option<usize> {
            self.lods.iter().position(|lod| lod.id == id)
        }
    }

    /// @emoji 🔤 Fixed screen label px for a LOD band; stays constant while zooming inside the band.
    pub fn band_label_screen_px(band_px: &[f64], band_index: usize, fallback: f64) -> f64 {
        band_px.get(band_index).copied().unwrap_or(fallback)
    }

    /// @emoji 🔤 Lower camera-zoom bound for a LOD band (previous band `max_zoom`, or `zoom_min`).
    pub fn band_floor_zoom(band_floor_zoom: &[f64], band_index: usize, zoom_min: f64) -> f64 {
        band_floor_zoom.get(band_index).copied().unwrap_or(zoom_min).max(zoom_min)
    }

    /// @emoji 🔤 Label screen px scaled with camera zoom inside one LOD band so text keeps the same proportion to world geometry.
    pub fn lod_band_label_screen_px(base_screen_px: f64, zoom: f64, band_floor_zoom: f64) -> f64 {
        let z = zoom.max(ui_styling::metrics::camera::LOD_ZOOM_FLOOR);
        let floor = band_floor_zoom.max(ui_styling::metrics::camera::LOD_ZOOM_FLOOR);
        base_screen_px * z / floor
    }
}
// #endregion 🔖Lod

// #region 🔖Raster
pub mod raster {
    use crate::{Affine, RasterImage, Scene};
    use std::collections::HashMap;
    use std::sync::Arc;

    pub fn draw_image(scene: &mut Scene, image: &RasterImage, transform: Affine) {
        scene.draw_image(image, transform);
    }

    pub fn draw_image_arc(scene: &mut Scene, image: &Arc<RasterImage>, transform: Affine) {
        scene.draw_image(image, transform);
    }

    #[derive(Clone, Default)]
    pub struct RasterImageCache {
        entries: HashMap<String, Arc<RasterImage>>,
    }

    impl RasterImageCache {
        pub fn get(&self, key: &str) -> Option<Arc<RasterImage>> {
            self.entries.get(key).cloned()
        }

        pub fn insert(&mut self, key: String, image: RasterImage) -> Arc<RasterImage> {
            let arc = Arc::new(image);
            self.entries.insert(key, arc.clone());
            arc
        }
    }
}
// #endregion 🔖Raster

// #region 🔖Render
pub mod render {
    use crate::{Affine, Scene};

    /// @emoji 📐 Scales a logical-viewport scene to the physical GPU surface (device pixel ratio).
    pub fn scale_scene_for_device_pixel_ratio(scene: Scene, dpr: f64) -> Scene {
        let scale = dpr.max(1.0);
        if (scale - 1.0).abs() < f64::EPSILON {
            return scene;
        }
        let mut scaled = Scene::new();
        scaled.append(&scene, Some(Affine::IDENTITY.scale(scale)));
        scaled
    }
}
// #endregion 🔖Render

// #region 🔖CanvasContent
pub mod canvas_content {
    use crate::{Color, Scene};

    pub trait CanvasContent {
        fn build_scene(&self) -> Scene;
        fn clear_color(&self) -> Color;
    }
}
// #endregion 🔖CanvasContent

// #region 🔖GpuSession
#[cfg(target_arch = "wasm32")]
pub mod gpu_session {
    use crate::renderer::vello_backend::{util, vello, wgpu};
    use crate::{Color, Scene};
    use wasm_bindgen::prelude::JsValue;
    use web_sys::HtmlCanvasElement;

    #[derive(Default)]
    pub struct CanvasGpuSession {
        #[allow(dead_code, reason = "Retains canvas for the WebGPU surface lifetime.")]
        canvas: Option<HtmlCanvasElement>,
        render_ctx: Option<util::RenderContext>,
        renderer: Option<vello::Renderer>,
        surface: Option<util::RenderSurface<'static>>,
    }

    impl CanvasGpuSession {
        pub fn gpu_ready(&self) -> bool {
            self.surface.is_some()
        }

        /// @emoji 🖥️ WebGPU surface bring-up; returns `String` (not `CanvasError`) because every call site is a wasm-bindgen boundary fn that immediately erases the error into a `JsValue` for JS — see `render_frame` below for the same convention.
        pub async fn create_canvas_surface(canvas: HtmlCanvasElement, pw: u32, ph: u32) -> Result<(util::RenderContext, vello::Renderer, util::RenderSurface<'static>), String> {
            let mut render_ctx = util::RenderContext::new();
            let surface = render_ctx.create_surface(wgpu::SurfaceTarget::Canvas(canvas), pw, ph, wgpu::PresentMode::AutoVsync).await.map_err(|err| format!("{err:?}"))?;
            let dev = &render_ctx.devices[surface.dev_id].device;
            let renderer =
                vello::Renderer::new(dev, vello::RendererOptions { use_cpu: false, antialiasing_support: vello::AaSupport::area_only(), num_init_threads: std::num::NonZeroUsize::new(1), pipeline_cache: None }).map_err(|err| format!("{err:?}"))?;
            Ok((render_ctx, renderer, surface))
        }

        pub fn finish_attach(&mut self, canvas: HtmlCanvasElement, render_ctx: util::RenderContext, renderer: vello::Renderer, surface: util::RenderSurface<'static>) {
            self.canvas = Some(canvas);
            self.render_ctx = Some(render_ctx);
            self.renderer = Some(renderer);
            self.surface = Some(surface);
        }

        pub fn resize_surface(&mut self, pw: u32, ph: u32) {
            if let (Some(surface), Some(render_ctx)) = (self.surface.as_mut(), self.render_ctx.as_mut()) {
                let cur_w = surface.config.width;
                let cur_h = surface.config.height;
                if cur_w != pw || cur_h != ph {
                    render_ctx.resize_surface(surface, pw, ph);
                }
            }
        }

        pub fn detach(&mut self) {
            self.canvas = None;
            self.render_ctx = None;
            self.renderer = None;
            self.surface = None;
        }

        pub fn render_frame(&mut self, scene: &Scene, clear_color: Color) -> Result<(), JsValue> {
            for _attempt in 0..3u8 {
                let (surface, renderer, render_ctx) = match (self.surface.as_mut(), self.renderer.as_mut(), self.render_ctx.as_mut()) {
                    (Some(s), Some(r), Some(rc)) => (s, r, rc),
                    _ => return Ok(()),
                };
                let dh = &render_ctx.devices[surface.dev_id];
                let pw = surface.config.width.max(1);
                let ph = surface.config.height.max(1);
                let params = vello::RenderParams { base_color: clear_color.0, width: pw, height: ph, antialiasing_method: vello::AaConfig::Area };
                renderer.render_to_texture(&dh.device, &dh.queue, &scene.0, &surface.target_view, &params).map_err(|err| JsValue::from_str(&format!("{err:?}")))?;

                let surface_tex = match surface.surface.get_current_texture() {
                    Ok(t) => t,
                    Err(wgpu::SurfaceError::Outdated) => {
                        surface.surface.configure(&dh.device, &surface.config);
                        continue;
                    }
                    Err(wgpu::SurfaceError::Timeout) | Err(wgpu::SurfaceError::Other) => return Ok(()),
                    Err(wgpu::SurfaceError::Lost) | Err(wgpu::SurfaceError::OutOfMemory) => {
                        return Err(JsValue::from_str("surface lost or validation error"));
                    }
                };
                let view = surface_tex.texture.create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = dh.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("infinite_cavas_surface_blit") });
                surface.blitter.copy(&dh.device, &mut encoder, &surface.target_view, &view);
                dh.queue.submit(std::iter::once(encoder.finish()));
                surface_tex.present();
                let _ = dh.device.poll(wgpu::PollType::Poll).ok();
                return Ok(());
            }
            Ok(())
        }
    }
}
// #endregion 🔖GpuSession

// #region 🔖IconCodec
pub mod icon_codec {
    // #region icon_codec
    //! 🖼️ Generic icon encoding resolver for board nodes (url, shortcode, typst, emoji, raster, inline SVG, catalog, text).

    use base64::Engine as _;
    use serde::{Deserialize, Serialize};
    use std::path::PathBuf;
    use std::sync::{Arc, OnceLock};

    use typst::foundations::{Bytes, Datetime};
    use typst::layout::{Abs, PagedDocument};
    use typst::syntax::{FileId, Source, VirtualPath};
    use typst::text::Font;
    use typst::utils::LazyHash;
    use typst::Library;
    use typst::LibraryExt;
    use typst::World;

    mod icon_shortcodes {
        include!(concat!(env!("OUT_DIR"), "/icon_shortcode_match.rs"));
    }

    /// 🔍 Optional lookup for domain-themed SVG icons (e.g. puzzle metabolism table).
    pub type ThemedSvgLookup = fn(&str) -> Option<&'static str>;

    // #region 🏷️IconUnion

    /// @emoji 🖼 Canonical structured icon payload shared across canvases and UI chrome.
    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(tag = "kind", rename_all = "camelCase")]
    pub enum Icon {
        Url { url: String },
        Shortcode { code: String },
        Data { data: String },
        Emoji { emoji: String },
        Typst { src: String },
        Text { text: String },
        Svg { svg: String },
        Catalog { key: String },
    }

    fn is_raster_data_url_payload(s: &str) -> bool {
        let u = s.trim().to_ascii_lowercase();
        u.starts_with("data:image/png;base64,") || u.starts_with("data:image/jpeg;base64,") || u.starts_with("data:image/jpg;base64,") || u.starts_with("data:image/webp;base64,") || u.starts_with("data:image/gif;base64,")
    }

    fn is_svg_data_url_payload(s: &str) -> bool {
        s.trim().to_ascii_lowercase().starts_with("data:image/svg+xml")
    }

    fn looks_like_shortcode_token(t: &str) -> bool {
        t.len() >= 3 && t.starts_with(':') && t.ends_with(':') && t[1..t.len() - 1].chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '+' | '-'))
    }

    fn shortcode_inner(t: &str) -> Option<&str> {
        if !looks_like_shortcode_token(t) {
            return None;
        }
        Some(&t[1..t.len() - 1])
    }

    fn looks_like_ascii_catalogish_stem(s: &str) -> bool {
        let t = s.trim();
        if t.is_empty() || !t.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-')) {
            return false;
        }
        matches!(t.chars().next(), Some(c) if c.is_ascii_alphabetic() || c == '_') && (t.contains('.') || t.contains('_') || t.contains('-') || t.len() > 48)
    }

    fn is_extended_pictographic_char(c: char) -> bool {
        let cp = c as u32;
        matches!(cp, 0x1F1E6..=0x1F1FF | 0x1F300..=0x1FAFF | 0x2600..=0x27BF | 0x2300..=0x23FF) || matches!(c, '©' | '®' | '™' | '☺' | '☻' | '♥' | '♦' | '♣' | '♠' | '✓' | '✔' | '✕' | '✖' | '✗' | '✘') || c == '\u{FE0F}' || c == '\u{200D}'
    }

    fn looks_like_bare_emoji(s: &str) -> bool {
        let t = s.trim();
        !t.is_empty() && t.chars().any(is_extended_pictographic_char)
    }

    fn looks_like_bare_url(s: &str) -> bool {
        let lower = s.trim().to_ascii_lowercase();
        lower.starts_with("http://") || lower.starts_with("https://")
    }

    fn resolve_inline_svg(encoded: &str) -> Option<String> {
        let t = encoded.trim();
        if t.is_empty() {
            return None;
        }
        let lower = t.to_ascii_lowercase();
        if lower.starts_with("<?xml") || lower.contains("<svg") {
            return Some(t.to_string());
        }
        None
    }

    /// @emoji 🔤 Decodes a canonical icon string into a structured {@link Icon}.
    pub fn decode_icon(encoded: &str) -> Option<Icon> {
        let t = encoded.trim();
        if t.is_empty() {
            return None;
        }
        if let Some(url) = t.strip_prefix("url:") {
            let url = url.trim();
            return (!url.is_empty()).then(|| Icon::Url { url: url.to_string() });
        }
        if looks_like_bare_url(t) {
            return Some(Icon::Url { url: t.to_string() });
        }
        if let Some(code) = shortcode_inner(t) {
            return Some(Icon::Shortcode { code: code.to_string() });
        }
        if let Some(src) = t.strip_prefix("typst:") {
            let src = src.trim();
            return (!src.is_empty()).then(|| Icon::Typst { src: src.to_string() });
        }
        if t.starts_with('$') {
            return Some(Icon::Typst { src: t.to_string() });
        }
        if let Some(em) = t.strip_prefix("emoji:") {
            let em = em.trim();
            return (!em.is_empty()).then(|| Icon::Emoji { emoji: em.to_string() });
        }
        if let Some(text) = t.strip_prefix("text:") {
            let text = text.trim();
            return (!text.is_empty()).then(|| Icon::Text { text: text.to_string() });
        }
        if is_raster_data_url_payload(t) || is_svg_data_url_payload(t) || t.to_ascii_lowercase().starts_with("data:") {
            return Some(Icon::Data { data: t.to_string() });
        }
        if let Some(svg) = resolve_inline_svg(t) {
            return Some(Icon::Svg { svg });
        }
        if looks_like_ascii_catalogish_stem(t) {
            return Some(Icon::Catalog { key: t.to_string() });
        }
        if looks_like_bare_emoji(t) {
            return Some(Icon::Emoji { emoji: t.to_string() });
        }
        if t.chars().count() <= 16 {
            return Some(Icon::Text { text: t.to_string() });
        }
        Some(Icon::Catalog { key: t.to_string() })
    }

    /// @emoji 🔤 Encodes a structured {@link Icon} into the canonical wire string.
    pub fn encode_icon(icon: &Icon) -> String {
        match icon {
            Icon::Url { url } => format!("url:{}", url.trim()),
            Icon::Shortcode { code } => format!(":{code}:"),
            Icon::Data { data } => data.trim().to_string(),
            Icon::Emoji { emoji } => format!("emoji:{}", emoji.trim()),
            Icon::Typst { src } => {
                let s = src.trim();
                if s.starts_with('$') {
                    s.to_string()
                } else {
                    format!("typst:{s}")
                }
            }
            Icon::Text { text } => format!("text:{}", text.trim()),
            Icon::Svg { svg } => svg.trim().to_string(),
            Icon::Catalog { key } => key.trim().to_string(),
        }
    }

    // #endregion 🏷️IconUnion

    #[derive(Debug)]
    pub enum BoardResolvedIcon {
        None,
        SvgThemed(String),
        SvgPlain(String),
        RasterRgba8 { rgba: Arc<[u8]>, w: u32, h: u32 },
    }

    struct RgbaImage {
        data: Arc<[u8]>,
        w: u32,
        h: u32,
    }

    fn decode_data_url_svg(s: &str) -> Option<String> {
        let t = s.trim();
        let lower = t.to_ascii_lowercase();
        if lower.starts_with("data:image/svg+xml;base64,") {
            let rest = t.split_once(',').map(|(_, b)| b.trim())?;
            let raw = base64::engine::general_purpose::STANDARD.decode(rest).ok()?;
            return String::from_utf8(raw).ok();
        }
        if lower.starts_with("data:image/svg+xml,") {
            let rest = t.split_once(',').map(|(_, b)| b.trim())?;
            return Some(percent_decode_utf8(rest));
        }
        None
    }

    fn percent_decode_utf8(input: &str) -> String {
        let bytes = input.as_bytes();
        let mut out = Vec::with_capacity(bytes.len());
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'%' && i + 2 < bytes.len() {
                if let (Some(h), Some(l)) = (hex_nibble(bytes[i + 1]), hex_nibble(bytes[i + 2])) {
                    out.push((h << 4) | l);
                    i += 3;
                    continue;
                }
            }
            out.push(bytes[i]);
            i += 1;
        }
        String::from_utf8_lossy(&out).into_owned()
    }

    fn hex_nibble(b: u8) -> Option<u8> {
        match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 10),
            b'A'..=b'F' => Some(b - b'A' + 10),
            _ => None,
        }
    }

    fn decode_raster_icon_bytes(t: &str) -> Option<RgbaImage> {
        let s = t.trim();
        let rest = s
            .strip_prefix("data:image/png;base64,")
            .or_else(|| s.strip_prefix("data:image/jpeg;base64,"))
            .or_else(|| s.strip_prefix("data:image/jpg;base64,"))
            .or_else(|| s.strip_prefix("data:image/webp;base64,"))
            .or_else(|| s.strip_prefix("data:image/gif;base64,"))?;
        let raw = base64::engine::general_purpose::STANDARD.decode(rest.trim()).ok()?;
        let img = image::load_from_memory(&raw).ok()?;
        let rgba = img.to_rgba8();
        let (w, h) = rgba.dimensions();
        if w == 0 || h == 0 {
            return None;
        }
        Some(RgbaImage { data: Arc::from(rgba.into_raw().into_boxed_slice()), w, h })
    }

    fn typst_asset_font_list() -> Vec<Font> {
        let mut out = Vec::new();
        for bytes in typst_assets::fonts() {
            let blob = Bytes::new(bytes);
            let mut idx = 0u32;
            while let Some(f) = Font::new(blob.clone(), idx) {
                out.push(f);
                idx = idx.saturating_add(1);
            }
        }
        out
    }

    fn typst_asset_font_list_plus_noto_color_emoji() -> Vec<Font> {
        let mut out = typst_asset_font_list();
        let emoji_blob = Bytes::new(crate::icon_assets::NOTO_COLOR_EMOJI_SUBSET_TTF);
        let mut idx = 0u32;
        while let Some(f) = Font::new(emoji_blob.clone(), idx) {
            out.push(f);
            idx = idx.saturating_add(1);
        }
        out
    }

    fn board_typst_compile_markup_to_svg(markup: &str, fonts: &'static [Font], book: &'static LazyHash<typst::text::FontBook>) -> Option<String> {
        static LIB: OnceLock<LazyHash<Library>> = OnceLock::new();
        static MAIN: OnceLock<FileId> = OnceLock::new();
        let library = LIB.get_or_init(|| LazyHash::new(Library::default()));
        let main = *MAIN.get_or_init(|| FileId::new(None, VirtualPath::new("/board.typ")));
        let source = Source::new(main, markup.to_string());
        struct BoardTypstWorld<'a> {
            library: &'static LazyHash<Library>,
            book: &'static LazyHash<typst::text::FontBook>,
            main: FileId,
            source: Source,
            fonts: &'a [Font],
        }
        impl World for BoardTypstWorld<'_> {
            fn library(&self) -> &LazyHash<Library> {
                self.library
            }
            fn book(&self) -> &LazyHash<typst::text::FontBook> {
                self.book
            }
            fn main(&self) -> FileId {
                self.main
            }
            fn source(&self, id: FileId) -> typst::diag::FileResult<Source> {
                if id == self.main {
                    Ok(self.source.clone())
                } else {
                    Err(typst::diag::FileError::NotFound(PathBuf::from("board.typ")))
                }
            }
            fn file(&self, _id: FileId) -> typst::diag::FileResult<Bytes> {
                Err(typst::diag::FileError::NotFound(PathBuf::from("board.bin")))
            }
            fn font(&self, index: usize) -> Option<Font> {
                self.fonts.get(index).cloned()
            }
            fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
                None
            }
        }
        let w = BoardTypstWorld { library, book, main, source, fonts };
        let warned = typst::compile::<PagedDocument>(&w);
        let doc = warned.output.ok()?;
        if doc.pages.is_empty() {
            return None;
        }
        Some(typst_svg::svg_merged(&doc, Abs::pt(ui_styling::metrics::typst::SVG_MARGIN_PT)))
    }

    static TYPST_ASSET_FONTS: OnceLock<Vec<Font>> = OnceLock::new();
    static TYPST_ASSET_BOOK: OnceLock<LazyHash<typst::text::FontBook>> = OnceLock::new();
    static TYPST_ICON_EMOJI_FONTS: OnceLock<Vec<Font>> = OnceLock::new();
    static TYPST_ICON_EMOJI_BOOK: OnceLock<LazyHash<typst::text::FontBook>> = OnceLock::new();

    pub fn board_typst_markup_to_svg(markup: &str) -> Option<String> {
        let fonts = TYPST_ASSET_FONTS.get_or_init(typst_asset_font_list);
        let book = TYPST_ASSET_BOOK.get_or_init(|| LazyHash::new(typst::text::FontBook::from_fonts(fonts.iter())));
        board_typst_compile_markup_to_svg(markup, fonts.as_slice(), book)
    }

    fn board_typst_markup_to_svg_for_icon_emoji(markup: &str) -> Option<String> {
        let fonts = TYPST_ICON_EMOJI_FONTS.get_or_init(typst_asset_font_list_plus_noto_color_emoji);
        let book = TYPST_ICON_EMOJI_BOOK.get_or_init(|| LazyHash::new(typst::text::FontBook::from_fonts(fonts.iter())));
        board_typst_compile_markup_to_svg(markup, fonts.as_slice(), book)
    }

    fn board_typst_markup_to_svg_for_icon_text(markup: &str) -> Option<String> {
        let fonts = TYPST_ASSET_FONTS.get_or_init(typst_asset_font_list);
        let book = TYPST_ASSET_BOOK.get_or_init(|| LazyHash::new(typst::text::FontBook::from_fonts(fonts.iter())));
        board_typst_compile_markup_to_svg(markup, fonts.as_slice(), book)
    }

    fn resolve_typst_src(src: &str) -> BoardResolvedIcon {
        let src = src.trim();
        if src.is_empty() {
            return BoardResolvedIcon::None;
        }
        let wrapped = format!("#set page(width: {}pt, height: {}pt, margin: {}pt, fill: none)\n{src}", ui_styling::metrics::typst::ICON_PAGE_PT, ui_styling::metrics::typst::ICON_PAGE_PT, ui_styling::metrics::typst::ICON_MARGIN_PT);
        match board_typst_markup_to_svg(&wrapped) {
            Some(s) => BoardResolvedIcon::SvgPlain(s),
            None => BoardResolvedIcon::None,
        }
    }

    fn resolve_emoji_body(em: &str) -> BoardResolvedIcon {
        let em = em.trim();
        if em.is_empty() {
            return BoardResolvedIcon::None;
        }
        let wrapped = format!(
            "#set page(width: {}pt, height: {}pt, margin: {}pt, fill: none)\n#set align(center + horizon)\n#set text(size: {}pt, font: \"{}\")\n{em}",
            ui_styling::metrics::typst::EMOJI_PAGE_PT,
            ui_styling::metrics::typst::EMOJI_PAGE_PT,
            ui_styling::metrics::typst::EMOJI_MARGIN_PT,
            ui_styling::metrics::typst::EMOJI_TEXT_PT,
            ui_styling::canvas_fonts::NOTO_COLOR_EMOJI
        );
        match board_typst_markup_to_svg_for_icon_emoji(&wrapped) {
            Some(s) => BoardResolvedIcon::SvgPlain(s),
            None => BoardResolvedIcon::None,
        }
    }

    fn resolve_text_body(text: &str) -> BoardResolvedIcon {
        let text = text.trim();
        if text.is_empty() {
            return BoardResolvedIcon::None;
        }
        let escaped = text.replace('\\', "\\\\").replace('"', "\\\"");
        let wrapped = format!(
            "#set page(width: {}pt, height: {}pt, margin: {}pt, fill: none)\n#set align(center + horizon)\n#set text(size: {}pt)\n\"{escaped}\"",
            ui_styling::metrics::typst::ICON_PAGE_PT,
            ui_styling::metrics::typst::ICON_PAGE_PT,
            ui_styling::metrics::typst::ICON_MARGIN_PT,
            ui_styling::metrics::typst::TEXT_ICON_PT
        );
        match board_typst_markup_to_svg_for_icon_text(&wrapped) {
            Some(s) => BoardResolvedIcon::SvgPlain(s),
            None => BoardResolvedIcon::None,
        }
    }

    fn resolve_icon_data(data: &str) -> BoardResolvedIcon {
        if let Some(svg) = decode_data_url_svg(data) {
            return BoardResolvedIcon::SvgPlain(svg);
        }
        if let Some(img) = decode_raster_icon_bytes(data) {
            return BoardResolvedIcon::RasterRgba8 { rgba: img.data, w: img.w, h: img.h };
        }
        BoardResolvedIcon::None
    }

    fn resolve_structured_icon(icon: &Icon, themed_lookup: ThemedSvgLookup) -> BoardResolvedIcon {
        match icon {
            Icon::Url { .. } => BoardResolvedIcon::None,
            Icon::Shortcode { code } => match icon_shortcodes::icon_shortcode_resolve(code) {
                Some(icon_shortcodes::ShortcodeResolved::Emoji(em)) => resolve_emoji_body(em),
                Some(icon_shortcodes::ShortcodeResolved::SvgPlain(svg)) => BoardResolvedIcon::SvgPlain(svg.to_string()),
                Some(icon_shortcodes::ShortcodeResolved::SvgThemed(svg)) => BoardResolvedIcon::SvgThemed(svg.to_string()),
                None => themed_lookup(code).map_or(BoardResolvedIcon::None, |svg| BoardResolvedIcon::SvgThemed(svg.to_string())),
            },
            Icon::Data { data } => resolve_icon_data(data),
            Icon::Emoji { emoji } => resolve_emoji_body(emoji),
            Icon::Typst { src } => resolve_typst_src(src),
            Icon::Text { text } => resolve_text_body(text),
            Icon::Svg { svg } => {
                if themed_lookup(svg).is_some() {
                    BoardResolvedIcon::SvgThemed(svg.clone())
                } else {
                    BoardResolvedIcon::SvgPlain(svg.clone())
                }
            }
            Icon::Catalog { key } => themed_lookup(key).map_or(BoardResolvedIcon::None, |svg| BoardResolvedIcon::SvgThemed(svg.to_string())),
        }
    }

    /// @emoji 🔍 Resolves an icon encoding to paintable content; `themed_lookup` marks SVG as themed when present.
    pub fn board_resolve_icon_kind(encoded: &str, themed_lookup: ThemedSvgLookup) -> BoardResolvedIcon {
        let Some(icon) = decode_icon(encoded) else {
            return BoardResolvedIcon::None;
        };
        resolve_structured_icon(&icon, themed_lookup)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn round_trip(s: &str) {
            let icon = decode_icon(s).expect("decode");
            let encoded = encode_icon(&icon);
            let again = decode_icon(&encoded).expect("re-decode");
            assert_eq!(icon, again, "round-trip failed for {s} -> {encoded}");
        }

        #[test]
        fn icon_codec_round_trips_all_kinds() {
            round_trip("url:https://example.com/icon.png");
            round_trip(":smile:");
            round_trip("data:image/png;base64,iVBORw0KGgo=");
            round_trip("emoji:☺");
            round_trip("typst:$x^2$");
            round_trip("text:Hi");
            round_trip(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><rect width="10" height="10"/></svg>"#);
            round_trip("capsule_J");
        }

        #[test]
        fn icon_codec_decodes_catalog_stem() {
            assert!(matches!(decode_icon("capsule_J"), Some(Icon::Catalog { .. })));
        }

        #[test]
        fn icon_codec_resolves_emoji_shortcode_to_svg() {
            let r = board_resolve_icon_kind(":grinning:", |_| None);
            match r {
                BoardResolvedIcon::SvgPlain(s) => assert!(s.contains("<svg")),
                other => panic!("unexpected: {other:?}"),
            }
        }

        #[test]
        fn icon_codec_resolves_catalog_shortcode_to_svg() {
            let r = board_resolve_icon_kind(":plus:", |_| None);
            match r {
                BoardResolvedIcon::SvgPlain(s) => assert!(s.contains("<svg")),
                other => panic!("unexpected: {other:?}"),
            }
        }

        #[test]
        fn icon_codec_resolves_metabolism_shortcode_to_themed_svg() {
            let r = board_resolve_icon_kind(":capsule_J:", |_| None);
            match r {
                BoardResolvedIcon::SvgThemed(s) => assert!(s.contains("<svg")),
                other => panic!("unexpected: {other:?}"),
            }
        }

        #[test]
        fn icon_codec_resolves_text_to_svg() {
            let r = board_resolve_icon_kind("text:Hi", |_| None);
            match r {
                BoardResolvedIcon::SvgPlain(s) => assert!(s.contains("<svg")),
                other => panic!("unexpected: {other:?}"),
            }
        }

        #[test]
        fn icon_codec_url_returns_none_for_sync_resolver() {
            assert!(matches!(board_resolve_icon_kind("url:https://example.com/x.png", |_| None), BoardResolvedIcon::None));
        }
    }
    // #endregion icon_codec
}
pub use icon_codec::{board_resolve_icon_kind, board_typst_markup_to_svg, decode_icon, encode_icon, BoardResolvedIcon, Icon, ThemedSvgLookup};
// #endregion 🔖IconCodec

// #region 🔖CanvasExtension
/// 🧩 Extension hook for domain-specific canvas behavior (hit-test, paint, kinds).
pub trait CanvasExtension: Send + Sync {
    fn extension_id(&self) -> &str;
}

/// ⚙️ Generic infinite-canvas engine shell; domain logic lives in `E`.
pub struct CanvasEngine<E: CanvasExtension> {
    pub extension: E,
}

impl<E: CanvasExtension> CanvasEngine<E> {
    pub fn new(extension: E) -> Self {
        Self { extension }
    }
}
// #endregion 🔖CanvasExtension

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::camera::{screen_to_world, world_to_screen, Camera, Viewport};
    use super::lod::{Lod, LodScale};
    use super::text;
    use super::theme;
    use crate::{Point, Scene};

    #[test]
    fn scale_scene_for_device_pixel_ratio_scales_logical_scene() {
        let mut scene = Scene::new();
        text::append_label(&mut scene, "A", Point::new(10.0, 10.0), 12.0, theme::default_icon_fg(), theme::default_icon_bg());
        let logical = scene.path_count();
        let scaled = super::render::scale_scene_for_device_pixel_ratio(scene, 2.0);
        assert!(scaled.path_count() >= logical);
        let identity = super::render::scale_scene_for_device_pixel_ratio(Scene::new(), 1.0);
        assert_eq!(identity.path_count(), 0);
    }

    #[test]
    fn append_label_renders_glyphs() {
        let mut scene = Scene::new();
        text::append_label(&mut scene, "Zürich", Point::new(40.0, 40.0), 14.0, theme::default_icon_fg(), theme::default_icon_bg());
        assert!(!scene.path_count().eq(&0));
        let mut empty = Scene::new();
        text::append_label(&mut empty, "  ", Point::new(0.0, 0.0), 14.0, theme::default_icon_fg(), theme::default_icon_bg());
        assert!(empty.is_empty());
    }

    #[test]
    fn label_advance_excludes_outer_padding() {
        let px = 14.0;
        let (box_w, _) = text::label_extent("MATCH", px);
        let advance = text::label_advance("MATCH", px);
        assert!(box_w > advance);
        assert_eq!(advance, "MATCH".len() as f64 * px * ui_styling::metrics::label::CHAR_WIDTH_RATIO);
    }

    #[test]
    fn label_byte_world_x_is_narrower_than_char_width_estimate() {
        let line = "MATCH";
        let x0 = text::label_byte_world_x(line, 0, 0.0, 14.0);
        let x5 = text::label_byte_world_x(line, 5, 0.0, 14.0);
        let estimate = text::label_advance("MATCH", 14.0);
        assert!(x5 - x0 < estimate * 0.85);
    }

    #[test]
    fn label_extent_matches_append_label_box() {
        let (w, h) = text::label_extent("math.add", 12.0);
        assert!(w > 32.0);
        assert!(h >= 16.0);
        assert_eq!(text::label_extent("  ", 12.0), (0.0, 0.0));
    }

    #[test]
    fn camera_round_trip() {
        let camera = Camera { x: 10.0, y: -5.0, zoom: 2.0 };
        let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
        let world = Point::new(12.0, 3.0);
        let screen = world_to_screen(&camera, &viewport, world);
        let back = screen_to_world(&camera, &viewport, screen);
        assert!((back.x - world.x).abs() < 1e-9);
        assert!((back.y - world.y).abs() < 1e-9);
    }

    #[test]
    fn lod_scale_resolve() {
        const LODS: &[Lod] =
            &[Lod { id: "minimap", name: "Minimap", description: "min", max_zoom: 0.15 }, Lod { id: "overview", name: "Overview", description: "ov", max_zoom: 0.35 }, Lod { id: "micro", name: "Micro", description: "mi", max_zoom: f64::INFINITY }];
        let scale = LodScale { lods: LODS };
        assert_eq!(scale.resolve(0.1).id, "minimap");
        assert_eq!(scale.resolve(0.2).id, "overview");
        assert_eq!(scale.resolve(3.0).id, "micro");
        assert_eq!(scale.index_of("overview"), Some(1));
    }
}
// #endregion 🔖Tests
