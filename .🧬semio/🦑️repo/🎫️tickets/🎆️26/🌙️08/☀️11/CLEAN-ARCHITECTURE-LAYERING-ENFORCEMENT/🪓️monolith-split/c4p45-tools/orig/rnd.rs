//! 🖌️ The first-party scene: the paint value types, the recorded command list `Scene`, its retirement registry, the portable draw list and the parsed SVG document.

// #region 🏷️VelloBackend
pub(super) mod vello_backend {
    /// 🖊️ `vello`'s font/SVG/codec transitive tail (`vello_encoding`, `skrifa`, `read-fonts`,
    /// `font-types`, `png`, `guillotiere`, `moxcms`, `pxfm`, …) is only needed where a real
    /// `vello::Scene` is actually rasterized — the host/browser target table. `Scene` itself
    /// is a first-party command list (see `SceneCommand` below). `kurbo`/`peniko` (the
    /// value-type family `Stroke`/`Color`/`Fill`/`Mix`/`ImageData` are built on) are ALSO
    /// host/browser-only now: every guest-reachable value type (`Cap`, `Join`, `Stroke`,
    /// `Color`, `FillRule`, `BlendMode`, `RasterImage`) is first-party, converting to a real
    /// `kurbo`/`peniko` value only inside `SceneCommand::replay_into` — the same
    /// already-host-gated boundary `vello` itself crosses at. `target_arch = "wasm32"` alone
    /// is TRUE for `wasm32-wasip2`, so the gate is always the
    /// `not(all(target_arch = "wasm32", target_env = "p2"))` compound. Ticket
    /// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`,
    /// `🔍️research/📓️kurbo-peniko-first-party.md` (extends
    /// `🔍️research/📓️vello-scene-first-party.md`).
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub use vello;
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    pub use vello::util;
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    pub use vello::wgpu;
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub use vello::Scene;
    /// 🎨️ `usvg` (via `vello_svg`) is a rendering/text-shaping dependency: every real
    /// consumer — `svg_icon`'s paint pipeline, `SvgDocument`, `mod text`'s label shaper — is
    /// reachable only from the host/browser paint tree or from `semio-framework-editor`
    /// (itself absent from every plugin's `wasm32-wasip2` dependency graph), never from a
    /// WASI guest. Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-
    /// ARTIFACTS`, `🔍️research/📓️infinite-text-shaping.md`.
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub use vello_svg;
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub use vello_svg::usvg;
}
// #endregion 🏷️VelloBackend

use geometry::{Affine, Arc, BezPath, Circle, CubicBez, Line, Rect, RoundedRect, ShapeRef};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use geometry::{PathEl, Point};
use std::mem::{size_of, ManuallyDrop};
use std::sync::{Arc as SharedArc, Mutex, OnceLock};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use vello_backend as backend;

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn affine_to_kurbo(value: Affine) -> kurbo::Affine {
    kurbo::Affine::new(value.as_coeffs())
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn point_to_kurbo(value: Point) -> kurbo::Point {
    kurbo::Point::new(value.x, value.y)
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn path_element_to_kurbo(value: &PathEl) -> kurbo::PathEl {
    match value {
        PathEl::MoveTo(point) => kurbo::PathEl::MoveTo(point_to_kurbo(*point)),
        PathEl::LineTo(point) => kurbo::PathEl::LineTo(point_to_kurbo(*point)),
        PathEl::QuadTo(control, point) => kurbo::PathEl::QuadTo(point_to_kurbo(*control), point_to_kurbo(*point)),
        PathEl::CurveTo(control1, control2, point) => kurbo::PathEl::CurveTo(point_to_kurbo(*control1), point_to_kurbo(*control2), point_to_kurbo(*point)),
        PathEl::ClosePath => kurbo::PathEl::ClosePath,
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn shape_path_elements(shape: ShapeRef<'_>, tolerance: f64) -> Vec<PathEl> {
    match shape {
        ShapeRef::Rect(shape) => shape.path_elements(tolerance),
        ShapeRef::RoundedRect(shape) => shape.path_elements(tolerance),
        ShapeRef::Circle(shape) => shape.path_elements(tolerance),
        ShapeRef::Line(shape) => shape.path_elements(tolerance),
        ShapeRef::Arc(shape) => shape.path_elements(tolerance),
        ShapeRef::CubicBez(shape) => shape.path_elements(tolerance),
        ShapeRef::BezPath(shape) => shape.elements(),
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
struct KurboShapeAdapter<'a>(ShapeRef<'a>);

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl kurbo::Shape for KurboShapeAdapter<'_> {
    type PathElementsIter<'iter>
        = std::vec::IntoIter<kurbo::PathEl>
    where
        Self: 'iter;

    fn path_elements(&self, tolerance: f64) -> Self::PathElementsIter<'_> {
        shape_path_elements(self.0, tolerance).iter().map(path_element_to_kurbo).collect::<Vec<_>>().into_iter()
    }
    fn area(&self) -> f64 {
        kurbo::Shape::area(&self.to_path(0.1))
    }

    fn perimeter(&self, accuracy: f64) -> f64 {
        kurbo::Shape::perimeter(&self.to_path(accuracy), accuracy)
    }

    fn winding(&self, point: kurbo::Point) -> i32 {
        kurbo::Shape::winding(&self.to_path(0.1), point)
    }

    fn bounding_box(&self) -> kurbo::Rect {
        kurbo::Shape::bounding_box(&self.to_path(0.1))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cap {
    Butt,
    Round,
    Square,
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl Cap {
    fn to_kurbo(self) -> kurbo::Cap {
        match self {
            Cap::Butt => kurbo::Cap::Butt,
            Cap::Round => kurbo::Cap::Round,
            Cap::Square => kurbo::Cap::Square,
        }
    }
}

/// 🔗️ First-party stroke join style — mirrors `kurbo::Join`. No guest call site sets it
/// explicitly today (every `Stroke` is built via `new`, matching `kurbo::Stroke::new`'s own
/// `Join::Round` default), but the field is real so `Stroke::to_kurbo` (host-only, below) can
/// rebuild a real `kurbo::Stroke` losslessly rather than hardcoding a join.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Join {
    Round,
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl Join {
    fn to_kurbo(self) -> kurbo::Join {
        match self {
            Join::Round => kurbo::Join::Round,
        }
    }
}

/// 🖊️ First-party stroke style — plain value fields, no `kurbo::Stroke` wrapper, so
/// `wasm32-wasip2` never needs `kurbo` to build/measure/dispose one. `to_kurbo` (host-only,
/// below) rebuilds a real `kurbo::Stroke` only at rasterization time, matching
/// `kurbo::Stroke::new`'s own field defaults exactly.
#[derive(Clone, Debug, PartialEq)]
pub struct Stroke {
    width: f64,
    join: Join,
    miter_limit: f64,
    start_cap: Cap,
    end_cap: Cap,
    dash_pattern: Vec<f64>,
    dash_offset: f64,
}

impl Stroke {
    pub fn new(width: f64) -> Self {
        Self { width, join: Join::Round, miter_limit: 4.0, start_cap: Cap::Round, end_cap: Cap::Round, dash_pattern: Vec::new(), dash_offset: 0.0 }
    }
    pub fn set_dash_pattern(&mut self, pattern: Vec<f64>) {
        self.dash_pattern = pattern;
    }
    /// 👁️ Borrows the original dash sequence without allocating or transferring it.
    pub fn dash_pattern(&self) -> &[f64] { &self.dash_pattern }
    /// 📦️ Observes the original dash allocation extent before custody transfers.
    pub fn dash_capacity(&self) -> usize { self.dash_pattern.capacity() }
    /// 🎁️ Transfers only the original dash buffer while retaining the scalar style.
    pub(crate) fn take_dash_pattern(&mut self) -> Vec<f64> { std::mem::take(&mut self.dash_pattern) }
    pub fn set_start_cap(&mut self, cap: Cap) {
        self.start_cap = cap;
    }
    pub fn set_end_cap(&mut self, cap: Cap) {
        self.end_cap = cap;
    }
    fn retirement_step(&mut self) -> bool {
        self.dash_pattern.pop().is_none()
    }
    fn retirement_backing_bytes(&self) -> usize {
        self.dash_pattern.capacity().saturating_mul(size_of::<f64>())
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl Stroke {
    /// 🖥️ Host/browser-only: rebuilds a real `kurbo::Stroke`, the only place this crate needs
    /// one, at the same `SceneCommand::replay_into` boundary `vello` itself is reached from.
    pub(crate) fn to_kurbo(&self) -> kurbo::Stroke {
        kurbo::Stroke {
            width: self.width,
            join: self.join.to_kurbo(),
            miter_limit: self.miter_limit,
            start_cap: self.start_cap.to_kurbo(),
            end_cap: self.end_cap.to_kurbo(),
            dash_pattern: self.dash_pattern.iter().copied().collect(),
            dash_offset: self.dash_offset,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// 🎨️ First-party straight-alpha sRGB color — the same representation `peniko::Color`
/// (`color::AlphaColor<color::Srgb>`) uses internally (`[f32; 4]`, straight not
/// premultiplied), reimplemented so `wasm32-wasip2` never needs `peniko`/`color` to build or
/// pass one. `to_peniko` (host-only, below) is a lossless `peniko::Color::new` at the
/// rasterization boundary; `to_rgba8`/`from_rgba8` byte-for-byte match
/// `color::AlphaColor::{to_rgba8,from_rgba8}` — proven differentially in `color_tests` below.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color([f32; 4]);

impl Color {
    pub fn new(rgba: [f32; 4]) -> Self {
        Self(rgba)
    }
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self([r, g, b, a].map(|c| f32::from(c) * (1.0 / 255.0)))
    }
    pub fn to_rgba8(self) -> Rgba8 {
        let [r, g, b, a] = self.0.map(|c| (c * 255.0 + 0.5) as u8);
        Rgba8 { r, g, b, a }
    }
    pub fn components(self) -> [f32; 4] {
        self.0
    }
    pub fn multiply_alpha(self, alpha: f32) -> Self {
        let [r, g, b, a] = self.0;
        Self([r, g, b, a * alpha])
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl Color {
    /// 🖥️ Host/browser-only: rebuilds a real `peniko::Color`, only ever needed at the
    /// `SceneCommand::replay_into` rasterization boundary.
    pub(crate) fn to_peniko(self) -> peniko::Color {
        peniko::Color::new(self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillRule {
    NonZero,
    EvenOdd,
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl FillRule {
    fn to_peniko(self) -> peniko::Fill {
        match self {
            FillRule::NonZero => peniko::Fill::NonZero,
            FillRule::EvenOdd => peniko::Fill::EvenOdd,
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

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl BlendMode {
    fn to_peniko(self) -> peniko::Mix {
        match self {
            BlendMode::Normal => peniko::Mix::Normal,
            BlendMode::Multiply => peniko::Mix::Multiply,
            BlendMode::Screen => peniko::Mix::Screen,
            BlendMode::Overlay => peniko::Mix::Overlay,
            BlendMode::Darken => peniko::Mix::Darken,
            BlendMode::Lighten => peniko::Mix::Lighten,
            BlendMode::ColorDodge => peniko::Mix::ColorDodge,
            BlendMode::ColorBurn => peniko::Mix::ColorBurn,
            BlendMode::HardLight => peniko::Mix::HardLight,
            BlendMode::SoftLight => peniko::Mix::SoftLight,
            BlendMode::Difference => peniko::Mix::Difference,
            BlendMode::Exclusion => peniko::Mix::Exclusion,
            BlendMode::Hue => peniko::Mix::Hue,
            BlendMode::Saturation => peniko::Mix::Saturation,
            BlendMode::Color => peniko::Mix::Color,
            BlendMode::Luminosity => peniko::Mix::Luminosity,
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

/// 🖼️ First-party RGBA8 raster image — plain `width`/`height`/`data` fields, no
/// `peniko::ImageData` wrapper, so `wasm32-wasip2` never needs `peniko` to build/measure/
/// dispose one. `to_peniko` (host-only, below) rebuilds a real `peniko::ImageData` (format
/// `Rgba8`, alpha type `Alpha` — the only combination `rgba8` ever produces) only at
/// rasterization time.
#[derive(Clone, Debug, PartialEq)]
pub struct RasterImage {
    width: u32,
    height: u32,
    data: SharedArc<Vec<u8>>,
}

impl semio_framework_value::retirement::RetireOwned for RasterImage {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::{sequence,deferred,shared::SharedControlledRetirement};sequence(vec![deferred(self.width),deferred(self.height),Box::new(SharedControlledRetirement::lease(self.data))])}
    fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::{sequence_birth_bytes,deferred_birth_bytes_for,shared::shared_retirement_birth_bytes};sequence_birth_bytes(&[deferred_birth_bytes_for(&self.width),deferred_birth_bytes_for(&self.height),shared_retirement_birth_bytes::<Vec<u8>>()])}
    fn controlled_retirement_supported()->bool{true}
}

impl RasterImage {
    /// 🖼️ Builds an RGBA8 raster image for scene drawing.
    pub fn rgba8(width: u32, height: u32, data: SharedArc<Vec<u8>>) -> Self {
        Self { width, height, data }
    }
    fn retirement_step(&mut self) -> bool {
        SharedArc::get_mut(&mut self.data).is_none_or(|data| data.pop().is_none())
    }
    fn retirement_backing_bytes(&self) -> usize {
        if SharedArc::strong_count(&self.data) == 1 {
            self.data.capacity()
        } else {
            0
        }
    }
    pub fn clone_data(&self) -> Self {
        Self { width: self.width, height: self.height, data: SharedArc::clone(&self.data) }
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn retirement_exclusive_backing_bytes(&self) -> usize {
        if SharedArc::strong_count(&self.data) == 1 {
            self.data.capacity()
        } else {
            0
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl RasterImage {
    /// 🖥️ Host/browser-only: rebuilds a real `peniko::ImageData`, only ever needed at the
    /// `SceneCommand::replay_into` rasterization boundary.
    pub(crate) fn to_peniko(&self) -> peniko::ImageData {
        peniko::ImageData { data: peniko::Blob::new(self.data.clone()), format: peniko::ImageFormat::Rgba8, alpha_type: peniko::ImageAlphaType::Alpha, width: self.width, height: self.height }
    }
}

/// 🧪️ Differential proof that the first-party `Color`/`Stroke`/`RasterImage` reimplementations
/// agree with the real `kurbo`/`peniko` (`color`) types they replaced on `wasm32-wasip2` — the
/// oracle is used directly (this crate already depends on `kurbo`/`peniko` on the host target
/// these tests run on, via the boundary conversions above), same convention as
/// `semio-framework-geometry`'s `path_seg_tests`.
#[cfg(all(test, not(all(target_arch = "wasm32", target_env = "p2"))))]
include!("../../🧪️tests/🔬️renderer-color-stroke-raster/🦀️.rs");

/// 📦️ An owned copy of a [`ShapeRef`] variant, kept exact (never flattened to a polyline at
/// record time) so a later transform — e.g. zooming an already-recorded scene — cannot make a
/// circle facet; the real curve only gets tessellated once, by `vello`, at rasterization time.
#[derive(Clone)]
#[cfg_attr(all(target_arch = "wasm32", target_env = "p2"), allow(dead_code, reason = "Fields are written by every guest build/measure/retire call but read back only by host-only replay (Scene::vello_scene) — never on this target, by design."))]
pub(crate) enum RecordedShape {
    Rect(Rect),
    RoundedRect(RoundedRect),
    Circle(Circle),
    Line(Line),
    Arc(Arc),
    CubicBez(CubicBez),
    BezPath(BezPath),
}

impl RecordedShape {
    fn retirement_step(&mut self) -> bool {
        match self {
            Self::BezPath(path) => path.retirement_step(),
            Self::Rect(_) | Self::RoundedRect(_) | Self::Circle(_) | Self::Line(_) | Self::Arc(_) | Self::CubicBez(_) => true,
        }
    }

    fn retirement_backing_bytes(&self) -> usize {
        match self {
            Self::BezPath(path) => path.retirement_backing_bytes(),
            Self::Rect(_) | Self::RoundedRect(_) | Self::Circle(_) | Self::Line(_) | Self::Arc(_) | Self::CubicBez(_) => 0,
        }
    }
}

/// 🖥️ Host/browser-only: the `ShapeRef` view is needed only to drive `geometry::with_shape_ref!`
/// inside `SceneCommand::replay_into` below.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl RecordedShape {
    fn as_shape_ref(&self) -> ShapeRef<'_> {
        match self {
            Self::Rect(s) => ShapeRef::Rect(s),
            Self::RoundedRect(s) => ShapeRef::RoundedRect(s),
            Self::Circle(s) => ShapeRef::Circle(s),
            Self::Line(s) => ShapeRef::Line(s),
            Self::Arc(s) => ShapeRef::Arc(s),
            Self::CubicBez(s) => ShapeRef::CubicBez(s),
            Self::BezPath(s) => ShapeRef::BezPath(s),
        }
    }
}

impl From<ShapeRef<'_>> for RecordedShape {
    fn from(value: ShapeRef<'_>) -> Self {
        match value {
            ShapeRef::Rect(s) => Self::Rect(*s),
            ShapeRef::RoundedRect(s) => Self::RoundedRect(*s),
            ShapeRef::Circle(s) => Self::Circle(*s),
            ShapeRef::Line(s) => Self::Line(*s),
            ShapeRef::Arc(s) => Self::Arc(*s),
            ShapeRef::CubicBez(s) => Self::CubicBez(*s),
            ShapeRef::BezPath(s) => Self::BezPath(s.clone()),
        }
    }
}

/// 🎬️ One recorded drawing instruction — the first-party replacement for `vello::Scene`'s
/// internal `vello_encoding::Encoding` buffers. `Scene` (below) is just `Vec<SceneCommand>`;
/// a real `vello::Scene` is built from it only at the host/browser rasterization call sites
/// (`Scene::vello_scene`), so `vello`/`vello_encoding` never need to compile for
/// `wasm32-wasip2` even though `Scene` itself is built, measured and disposed unconditionally.
/// `VelloFragment` is the one host-only escape hatch, for `SvgDocument::append_to_scene`
/// (icon/label painting, itself already host/browser-gated — see its docstring below).
/// Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`,
/// `🔍️research/📓️vello-scene-first-party.md`.
#[derive(Clone)]
pub(crate) enum SceneCommand {
    Fill {
        rule: FillRule,
        transform: Affine,
        paint: Paint,
        brush_transform: Option<Affine>,
        shape: RecordedShape,
    },
    Stroke {
        stroke: Stroke,
        transform: Affine,
        paint: Paint,
        brush_transform: Option<Affine>,
        shape: RecordedShape,
    },
    DrawImage {
        image: RasterImage,
        transform: Affine,
    },
    PushLayer {
        rule: FillRule,
        blend: BlendMode,
        alpha: f32,
        transform: Affine,
        clip: RecordedShape,
    },
    PushClipLayer {
        rule: FillRule,
        transform: Affine,
        clip: RecordedShape,
    },
    PopLayer,
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    VelloFragment {
        scene: SharedArc<backend::Scene>,
        transform: Affine,
    },
}

impl SceneCommand {
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn retire_vello_fragment(scene: &mut SharedArc<backend::Scene>) -> bool {
        let Some(scene) = SharedArc::get_mut(scene) else {
            return true;
        };
        let encoding = scene.encoding_mut();
        encoding.resources.patches.pop().is_none()
            && encoding.resources.color_stops.pop().is_none()
            && encoding.resources.glyphs.pop().is_none()
            && encoding.resources.glyph_runs.pop().is_none()
            && encoding.resources.normalized_coords.pop().is_none()
            && encoding.path_tags.pop().is_none()
            && encoding.path_data.pop().is_none()
            && encoding.draw_tags.pop().is_none()
            && encoding.draw_data.pop().is_none()
            && encoding.transforms.pop().is_none()
            && encoding.styles.pop().is_none()
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    #[allow(clippy::ptr_arg, reason = "retirement accounts allocation capacity, which a slice erases")]
    fn vector_backing_bytes<T>(values: &Vec<T>) -> usize {
        values.capacity().saturating_mul(size_of::<T>())
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn vello_fragment_backing_bytes(scene: &SharedArc<backend::Scene>) -> usize {
        if SharedArc::strong_count(scene) != 1 {
            return 0;
        }
        let encoding = scene.encoding();
        Self::vector_backing_bytes(&encoding.resources.patches)
            .saturating_add(Self::vector_backing_bytes(&encoding.resources.color_stops))
            .saturating_add(Self::vector_backing_bytes(&encoding.resources.glyphs))
            .saturating_add(Self::vector_backing_bytes(&encoding.resources.glyph_runs))
            .saturating_add(Self::vector_backing_bytes(&encoding.resources.normalized_coords))
            .saturating_add(Self::vector_backing_bytes(&encoding.path_tags))
            .saturating_add(Self::vector_backing_bytes(&encoding.path_data))
            .saturating_add(Self::vector_backing_bytes(&encoding.draw_tags))
            .saturating_add(Self::vector_backing_bytes(&encoding.draw_data))
            .saturating_add(Self::vector_backing_bytes(&encoding.transforms))
            .saturating_add(Self::vector_backing_bytes(&encoding.styles))
    }

    fn retirement_step(&mut self) -> bool {
        match self {
            Self::Fill { shape, .. } => shape.retirement_step(),
            Self::Stroke { stroke, shape, .. } => stroke.retirement_step() && shape.retirement_step(),
            Self::DrawImage { image, .. } => image.retirement_step(),
            Self::PushLayer { clip, .. } | Self::PushClipLayer { clip, .. } => clip.retirement_step(),
            Self::PopLayer => true,
            #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
            Self::VelloFragment { scene, .. } => Self::retire_vello_fragment(scene),
        }
    }

    fn retirement_backing_bytes(&self) -> usize {
        match self {
            Self::Fill { shape, .. } => shape.retirement_backing_bytes(),
            Self::Stroke { stroke, shape, .. } => stroke.retirement_backing_bytes().saturating_add(shape.retirement_backing_bytes()),
            Self::DrawImage { image, .. } => image.retirement_backing_bytes(),
            Self::PushLayer { clip, .. } | Self::PushClipLayer { clip, .. } => clip.retirement_backing_bytes(),
            Self::PopLayer => 0,
            #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
            Self::VelloFragment { scene, .. } => Self::vello_fragment_backing_bytes(scene),
        }
    }

    /// 🔁️ Rewrites this command's own transform(s) as `outer * existing` — the same
    /// "child-space-then-outer" composition `vello::Scene::append` itself performs on its
    /// internal per-op transform stack, applied here one first-party command at a time so
    /// `Scene::append` can flatten `other`'s commands directly into `self`'s list (keeping
    /// every command independently transferable into the retained retirement cursor, instead
    /// of nesting a whole sub-scene behind one non-incremental drop).
    fn transformed(self, outer: Affine) -> Self {
        match self {
            Self::Fill { rule, transform, paint, brush_transform, shape } => Self::Fill { rule, transform: outer * transform, paint, brush_transform: brush_transform.map(|bt| outer * bt), shape },
            Self::Stroke { stroke, transform, paint, brush_transform, shape } => Self::Stroke { stroke, transform: outer * transform, paint, brush_transform: brush_transform.map(|bt| outer * bt), shape },
            Self::DrawImage { image, transform } => Self::DrawImage { image, transform: outer * transform },
            Self::PushLayer { rule, blend, alpha, transform, clip } => Self::PushLayer { rule, blend, alpha, transform: outer * transform, clip },
            Self::PushClipLayer { rule, transform, clip } => Self::PushClipLayer { rule, transform: outer * transform, clip },
            Self::PopLayer => Self::PopLayer,
            #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
            Self::VelloFragment { scene, transform } => Self::VelloFragment { scene, transform: outer * transform },
        }
    }

    /// 🖥️ Host/browser-only: replays one command into a real, growing `vello::Scene`. The
    /// only place `vello`'s draw API is actually invoked — see the module docstring.
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn replay_into(&self, built: &mut backend::Scene) {
        match self {
            Self::Fill { rule, transform, paint, brush_transform, shape } => {
                let style: peniko::Fill = rule.to_peniko();
                let transform = affine_to_kurbo(*transform);
                let brush_transform = brush_transform.map(affine_to_kurbo);
                let Paint::Solid(color) = paint;
                built.fill(style, transform, color.to_peniko(), brush_transform, &KurboShapeAdapter(shape.as_shape_ref()));
            }
            Self::Stroke { stroke, transform, paint, brush_transform, shape } => {
                let stroke = stroke.to_kurbo();
                let transform = affine_to_kurbo(*transform);
                let brush_transform = brush_transform.map(affine_to_kurbo);
                let Paint::Solid(color) = paint;
                built.stroke(&stroke, transform, color.to_peniko(), brush_transform, &KurboShapeAdapter(shape.as_shape_ref()));
            }
            Self::DrawImage { image, transform } => {
                built.draw_image(&peniko::ImageBrush::new(image.to_peniko()), affine_to_kurbo(*transform));
            }
            Self::PushLayer { rule, blend, alpha, transform, clip } => {
                let style: peniko::Fill = rule.to_peniko();
                let mix: peniko::Mix = blend.to_peniko();
                let transform = affine_to_kurbo(*transform);
                built.push_layer(style, mix, *alpha, transform, &KurboShapeAdapter(clip.as_shape_ref()));
            }
            Self::PushClipLayer { rule, transform, clip } => {
                let style: peniko::Fill = rule.to_peniko();
                let transform = affine_to_kurbo(*transform);
                built.push_clip_layer(style, transform, &KurboShapeAdapter(clip.as_shape_ref()));
            }
            Self::PopLayer => built.pop_layer(),
            Self::VelloFragment { scene, transform } => built.append(scene, Some(affine_to_kurbo(*transform))),
        }
    }
}

#[derive(Clone, Default)]
pub struct Scene(pub(crate) Vec<SceneCommand>);

const OPAQUE_SCENE_RETIREMENT_CAPACITY: usize = 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpaqueSceneRetirementToken {
    slot: u16,
    generation: u64,
}

/// 🎬️ One exact scene-retirement turn with current credit and physical release split.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpaqueSceneRetirementStep {
    Blocked,
    Pending { released_items: usize, credited_bytes: usize, released_bytes: usize },
    Complete { released_items: usize, credited_bytes: usize, released_bytes: usize },
    Fault,
}

struct OpaqueSceneRetirementSlot {
    generation: u64,
    occupied: bool,
    credited_bytes: usize,
    scene: Option<ManuallyDrop<Scene>>,
    command: Option<ManuallyDrop<SceneCommand>>,
    command_backing_bytes: usize,
    command_credited_bytes: usize,
}

struct OpaqueSceneRetirementRegistry {
    slots: Box<[OpaqueSceneRetirementSlot; OPAQUE_SCENE_RETIREMENT_CAPACITY]>,
    faulted: bool,
}

impl Default for OpaqueSceneRetirementRegistry {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| OpaqueSceneRetirementSlot { generation: 0, occupied: false, credited_bytes: 0, scene: None, command: None, command_backing_bytes: 0, command_credited_bytes: 0 }), faulted: false }
    }
}

impl OpaqueSceneRetirementRegistry {
    fn reserve(&mut self) -> Option<OpaqueSceneRetirementToken> {
        let Some(index) = self.slots.iter().position(|slot| !slot.occupied) else {
            self.faulted = true;
            return None;
        };
        let slot = &mut self.slots[index];
        assert!(slot.scene.is_none(), "released opaque scene slot owns no scene");
        assert!(slot.command.is_none(), "released opaque scene slot owns no command");
        assert_eq!(slot.credited_bytes, 0, "released opaque scene slot owns no byte credit");
        assert_eq!(slot.command_backing_bytes, 0, "released opaque scene slot owns no command backing");
        assert_eq!(slot.command_credited_bytes, 0, "released opaque scene slot owns no command byte credit");
        slot.generation = slot.generation.wrapping_add(1).max(1);
        slot.occupied = true;
        Some(OpaqueSceneRetirementToken { slot: index as u16, generation: slot.generation })
    }

    fn token_is_current(&self, token: OpaqueSceneRetirementToken) -> bool {
        self.slots.get(usize::from(token.slot)).is_some_and(|slot| slot.occupied && slot.generation == token.generation)
    }

    fn publish(&mut self, token: OpaqueSceneRetirementToken, scene: Scene) {
        assert!(self.token_is_current(token), "opaque scene retirement token remains current before ownership transfer");
        let slot = &mut self.slots[usize::from(token.slot)];
        assert!(slot.scene.is_none(), "opaque scene retirement token remains unpublished");
        slot.scene = Some(ManuallyDrop::new(scene));
    }

    fn advance(&mut self, token: OpaqueSceneRetirementToken, maximum_items: usize, maximum_bytes: usize) -> OpaqueSceneRetirementStep {
        if maximum_items == 0 || maximum_bytes == 0 {
            return OpaqueSceneRetirementStep::Blocked;
        }
        if !self.token_is_current(token) {
            self.faulted = true;
            return OpaqueSceneRetirementStep::Fault;
        }
        let slot = &mut self.slots[usize::from(token.slot)];
        let Some(scene) = slot.scene.as_mut() else {
            return OpaqueSceneRetirementStep::Blocked;
        };
        if let Some(command) = slot.command.as_mut() {
            let remaining_bytes = slot.command_backing_bytes.saturating_sub(slot.command_credited_bytes);
            let credited_bytes = maximum_bytes.min(remaining_bytes);
            slot.command_credited_bytes = slot.command_credited_bytes.saturating_add(credited_bytes);
            if slot.command_credited_bytes != slot.command_backing_bytes {
                return OpaqueSceneRetirementStep::Pending { released_items: 0, credited_bytes, released_bytes: 0 };
            }
            if !command.retirement_step() {
                return OpaqueSceneRetirementStep::Pending { released_items: 1, credited_bytes, released_bytes: 0 };
            }
            let command = slot.command.take().expect("terminal opaque scene command remains retained");
            drop(ManuallyDrop::into_inner(command));
            let released_bytes = slot.command_backing_bytes;
            slot.command_backing_bytes = 0;
            slot.command_credited_bytes = 0;
            return OpaqueSceneRetirementStep::Pending { released_items: 1, credited_bytes, released_bytes };
        }
        if let Some(command) = scene.take_retirement_command() {
            slot.command_backing_bytes = command.retirement_backing_bytes();
            slot.command = Some(ManuallyDrop::new(command));
            return OpaqueSceneRetirementStep::Pending { released_items: 0, credited_bytes: 0, released_bytes: 0 };
        }
        let released_bytes = scene.retirement_backing_bytes();
        let remaining_bytes = released_bytes.saturating_sub(slot.credited_bytes);
        let credited_bytes = maximum_bytes.min(remaining_bytes);
        slot.credited_bytes = slot.credited_bytes.saturating_add(credited_bytes);
        if slot.credited_bytes != released_bytes {
            return OpaqueSceneRetirementStep::Pending { released_items: 0, credited_bytes, released_bytes: 0 };
        }
        let scene = slot.scene.take().expect("terminal opaque scene owner remains retained");
        drop(ManuallyDrop::into_inner(scene));
        slot.occupied = false;
        slot.credited_bytes = 0;
        debug_assert!(slot.command.is_none());
        debug_assert_eq!(slot.command_backing_bytes, 0);
        debug_assert_eq!(slot.command_credited_bytes, 0);
        OpaqueSceneRetirementStep::Complete { released_items: 1, credited_bytes, released_bytes }
    }

    fn active(&self) -> usize {
        self.slots.iter().filter(|slot| slot.occupied).count()
    }
}

fn opaque_scene_retirements() -> &'static Mutex<OpaqueSceneRetirementRegistry> {
    static REGISTRY: OnceLock<Mutex<OpaqueSceneRetirementRegistry>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(OpaqueSceneRetirementRegistry::default()))
}

pub fn reserve_opaque_scene_retirement() -> Option<OpaqueSceneRetirementToken> {
    let mut registry = opaque_scene_retirements().lock().expect("worker opaque scene retirement registry");
    registry.reserve()
}

pub fn publish_opaque_scene_retirement(token: OpaqueSceneRetirementToken, scene: Scene) {
    let mut registry = opaque_scene_retirements().lock().expect("worker opaque scene retirement registry");
    registry.publish(token, scene);
}

pub fn advance_opaque_scene_retirement(token: OpaqueSceneRetirementToken, maximum_items: usize, maximum_bytes: usize) -> OpaqueSceneRetirementStep {
    let mut registry = opaque_scene_retirements().lock().expect("worker opaque scene retirement registry");
    registry.advance(token, maximum_items, maximum_bytes)
}

pub fn opaque_scene_retirement_status() -> (usize, bool) {
    let registry = opaque_scene_retirements().lock().expect("worker opaque scene retirement registry");
    (registry.active(), registry.faulted)
}

/// ♻️ Retires exact first-party scene children and original command storage under supplied grants.
#[derive(Default)]
pub struct CanvasSceneRetirement{active:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,active_bytes:usize,image:Option<semio_framework_value::retirement::controlled::ControlledRetirement<RasterImage>>}
impl CanvasSceneRetirement{
    pub fn terminal_is_empty(&self,scene:&Scene)->bool{scene.0.is_empty()&&scene.0.capacity()==0&&self.active.is_none()&&self.image.is_none()}
    pub fn next_demands(&self,scene:&Scene,copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
        if let Some(active)=self.active.as_ref(){return if active.terminal_is_empty(){Ok(RetirementDemand{release_bytes:self.active_bytes,depth:1,..Default::default()})}else{active.next_demand(copy)}}
        if let Some(image)=self.image.as_ref(){return image.selected_demand(copy)}
        let shape=match scene.0.last(){Some(SceneCommand::Fill{shape,..})=>Some(shape),Some(SceneCommand::Stroke{stroke,shape,..})=>{if stroke.dash_capacity()!=0{return Ok(crate::CanvasStrokeRetirement::birth_demand())}Some(shape)},Some(SceneCommand::PushLayer{clip,..}|SceneCommand::PushClipLayer{clip,..})=>Some(clip),Some(SceneCommand::DrawImage{..})=>return Ok(RetirementDemand{depth:1,..Default::default()}),
            #[cfg(not(all(target_arch="wasm32",target_env="p2")))]
            Some(SceneCommand::VelloFragment{..})=>return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"opaque Vello scene has no priced original retirement owner")),_=>None};
        if shape.is_some_and(|shape|matches!(shape,RecordedShape::BezPath(path)if path.retirement_backing_bytes()!=0)){return Ok(crate::CanvasPathRetirement::birth_demand())}
        Ok(RetirementDemand{copy_bytes:if scene.0.is_empty(){0}else{size_of::<SceneCommand>()},release_bytes:if scene.0.is_empty(){scene.retirement_backing_bytes()}else{0},depth:usize::from(!self.terminal_is_empty(scene)),..Default::default()})
    }
    pub fn close_step(&mut self,scene:&mut Scene,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>{
        use semio_framework_value::{ErasedSnapshotRetirement,RetainedCloneStep as Step,RetainedCloneProgress as Progress};
        let empty=Progress::default();if self.terminal_is_empty(scene){return Ok(Step::Complete(empty))}
        let demand=self.next_demands(scene,grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(Step::Progress(empty))}
        if let Some(active)=self.active.as_mut(){if active.terminal_is_empty(){drop(self.active.take());let bytes=std::mem::take(&mut self.active_bytes);return Ok(Step::Progress(Progress{copied_items:1,released_bytes:bytes,..empty}))}return active.close_step(grant)}
        if let Some(image)=self.image.as_mut(){let step=image.step(grant)?;if image.terminal_is_empty(){self.image=None}return Ok(Step::Progress(step.progress()))}
        if let Some(SceneCommand::Stroke{stroke,..})=scene.0.last_mut(){if stroke.dash_capacity()!=0{let original=std::mem::replace(stroke,Stroke::new(0.0));let (owner,progress)=crate::CanvasStrokeRetirement::admit(original,grant).map_err(|(error,original)|{*stroke=original;error})?;self.active_bytes=size_of::<crate::CanvasStrokeRetirement>();self.active=Some(owner);return Ok(Step::Progress(progress))}}
        let shape=match scene.0.last_mut(){Some(SceneCommand::Fill{shape,..}|SceneCommand::Stroke{shape,..})=>Some(shape),Some(SceneCommand::PushLayer{clip,..}|SceneCommand::PushClipLayer{clip,..})=>Some(clip),_=>None};
        if let Some(RecordedShape::BezPath(path))=shape{if path.retirement_backing_bytes()!=0{let original=std::mem::take(path);let(owner,progress)=crate::CanvasPathRetirement::admit(original,grant).map_err(|(error,original)|{*path=original;error})?;self.active_bytes=size_of::<crate::CanvasPathRetirement>();self.active=Some(owner);return Ok(Step::Progress(progress))}}
        if matches!(scene.0.last(),Some(SceneCommand::DrawImage{..})){let Some(SceneCommand::DrawImage{image,..})=scene.0.pop()else{unreachable!()};self.image=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(image).unwrap_or_else(|_|panic!("first-party raster image retirement admitted")));return Ok(Step::Progress(Progress{copied_items:1,..empty}))}
        if scene.0.is_empty(){let bytes=scene.retirement_backing_bytes();drop(std::mem::take(&mut scene.0));return Ok(Step::Complete(Progress{copied_items:1,released_bytes:bytes,..empty}))}
        scene.0.pop();Ok(Step::Progress(Progress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}))
    }
}

impl Scene {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    /// 🎬️ Transfers one exact command into the retained retirement cursor.
    fn take_retirement_command(&mut self) -> Option<SceneCommand> {
        self.0.pop()
    }

    pub fn retirement_is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn retirement_backing_bytes(&self) -> usize {
        self.0.capacity().saturating_mul(size_of::<SceneCommand>())
    }
    /// 📦️ Bytes this scene keeps resident: its command list plus every command's own backing (path elements,
    /// stroke dashes, image pixels) — what a cache holding the scene accounts it at.
    pub fn retained_bytes(&self) -> usize {
        self.0.iter().fold(self.retirement_backing_bytes(), |total, command| total.saturating_add(command.retirement_backing_bytes()))
    }
    pub fn fill<'a>(&mut self, rule: FillRule, transform: Affine, paint: impl Into<Paint>, brush_transform: Option<Affine>, shape: impl Into<ShapeRef<'a>>) {
        self.0.push(SceneCommand::Fill { rule, transform, paint: paint.into(), brush_transform, shape: shape.into().into() });
    }
    pub fn stroke<'a>(&mut self, stroke: &Stroke, transform: Affine, paint: impl Into<Paint>, brush_transform: Option<Affine>, shape: impl Into<ShapeRef<'a>>) {
        self.0.push(SceneCommand::Stroke { stroke: stroke.clone(), transform, paint: paint.into(), brush_transform, shape: shape.into().into() });
    }
    pub fn draw_image(&mut self, image: &RasterImage, transform: Affine) {
        self.0.push(SceneCommand::DrawImage { image: image.clone_data(), transform });
    }
    /// 🔗️ Flattens `other`'s commands directly into `self` (composing `transform` into each,
    /// see `SceneCommand::transformed`) rather than nesting a sub-scene, so every appended
    /// command stays independently transferable into the retained retirement cursor.
    pub fn append(&mut self, other: &Scene, transform: Option<Affine>) {
        self.append_range(other, 0, other.0.len(), transform);
    }

    pub fn command_len(&self) -> usize {
        self.0.len()
    }

    /// 🎬️ Copies `other[start..end]` into `self`, composing `transform` the same way [`Scene::append`] does.
    pub fn append_range(&mut self, other: &Scene, start: usize, end: usize, transform: Option<Affine>) {
        let commands = &other.0[start..end];
        match transform {
            Some(outer) => self.0.extend(commands.iter().cloned().map(|command| command.transformed(outer))),
            None => self.0.extend(commands.iter().cloned()),
        }
    }
    pub fn push_layer<'a>(&mut self, rule: FillRule, blend: BlendMode, alpha: f32, transform: Affine, clip: impl Into<ShapeRef<'a>>) {
        self.0.push(SceneCommand::PushLayer { rule, blend, alpha, transform, clip: clip.into().into() });
    }
    pub fn pop_layer(&mut self) {
        self.0.push(SceneCommand::PopLayer);
    }
    pub fn push_clip_layer<'a>(&mut self, rule: FillRule, transform: Affine, clip: impl Into<ShapeRef<'a>>) {
        self.0.push(SceneCommand::PushClipLayer { rule, transform, clip: clip.into().into() });
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    /// 🔢️ A diagnostic hint (see `encoded_scene_hint`'s own docstring), not an exact-value
    /// contract — counts commands that draw or clip a path, same intent as the old
    /// `vello_encoding::Encoding::path_tags.len()` count but at first-party command
    /// granularity rather than per-verb granularity.
    pub fn path_count(&self) -> usize {
        self.0.iter().filter(|command| matches!(command, SceneCommand::Fill { .. } | SceneCommand::Stroke { .. } | SceneCommand::PushLayer { .. } | SceneCommand::PushClipLayer { .. })).count()
    }
    /// 🖥️ Host/browser-only: replays every recorded command into a freshly built
    /// `vello::Scene`, owned by the caller — the sole place this crate constructs a real
    /// `vello::Scene` from `Scene`'s first-party command list. `wasm32-wasip2` never reaches
    /// this method (nothing in `🎲️board` calls it — see module docstring).
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub(super) fn vello_scene(&self) -> backend::Scene {
        let mut built = backend::Scene::new();
        for command in &self.0 {
            command.replay_into(&mut built);
        }
        built
    }
}

// #region 🔖️DrawList
/// 🎬️ Portable replay encoding of a first-party [`Scene`] — the SAME command list
/// [`Scene::vello_scene`] rasterizes on a GPU target, written out as numbers a renderer with
/// no `vello` (a browser 2D canvas context, a conformance twin in any language) can replay
/// verb for verb. It exists because a `Scene` painted inside a wasm guest has, until now, had
/// exactly one exit: a real GPU device. Where there is none, the picture was dropped on the
/// floor and the host invented its own.
///
/// Shapes stay PRIMITIVES (`rect`/`rrect`/`circle`/`line`/`cubic`) instead of being flattened
/// to polylines, so a replaying 2D context reaches its own `roundRect`/`arc`/`bezierCurveTo`
/// and a later zoom cannot facet a circle — the same reason [`RecordedShape`] keeps them
/// exact. Only `Arc` and `BezPath` become verb streams, because no 2D primitive matches them.
///
/// Every command carries its own affine, and [`DagHost::paint_scene`] already bakes the camera
/// into those affines (`camera_content_affine`), so a replay is camera-correct with no second
/// transform pipeline. Device-pixel scaling is deliberately NOT baked in (unlike
/// [`render::scale_scene_for_device_pixel_ratio`], which the GPU path needs): the replayer sets
/// its own base transform, which keeps a draw list a pure function of scene + camera and so
/// directly comparable across implementations.
pub mod draw_list {
    use super::{BlendMode, Cap, FillRule, Paint, RecordedShape, Scene, SceneCommand};
    use geometry::{Affine, PathEl};

    /// 🔖️ Wire version of the encoding below; a replayer MUST refuse a version it does not know.
    pub const DRAW_LIST_VERSION: u32 = 1;
    /// 📐️ Flattening tolerance for the two non-primitive shapes (`Arc`, `BezPath` curves are
    /// emitted exactly; only `Arc` is flattened), in the scene's own pre-transform units.
    pub const DEFAULT_DRAW_LIST_TOLERANCE: f64 = 0.1;
    /// 🔢️ Decimal places every coordinate is rounded to. Three is ~1/1000 of a device pixel at
    /// unit scale — below any display's resolving power, and it is what makes two
    /// implementations' draw lists comparable as text.
    pub const DEFAULT_DRAW_LIST_DECIMALS: usize = 3;

    /// ⚙️ Encoder knobs. `maximum_commands` is a hard ceiling, not a hint: the encoded list
    /// travels a bounded ABI reply body, so a pathological scene must truncate loudly
    /// (`truncated: true` in the payload) rather than overrun it.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct DrawListOptions {
        pub tolerance: f64,
        pub decimals: usize,
        pub maximum_commands: usize,
    }

    impl Default for DrawListOptions {
        fn default() -> Self {
            Self { tolerance: DEFAULT_DRAW_LIST_TOLERANCE, decimals: DEFAULT_DRAW_LIST_DECIMALS, maximum_commands: 65_536 }
        }
    }

    fn fill_rule_code(rule: FillRule) -> u8 {
        match rule {
            FillRule::NonZero => 0,
            FillRule::EvenOdd => 1,
        }
    }

    fn cap_code(cap: Cap) -> u8 {
        match cap {
            Cap::Butt => 0,
            Cap::Round => 1,
            Cap::Square => 2,
        }
    }

    fn blend_code(blend: BlendMode) -> u8 {
        match blend {
            BlendMode::Normal => 0,
            BlendMode::Multiply => 1,
            BlendMode::Screen => 2,
            BlendMode::Overlay => 3,
            BlendMode::Darken => 4,
            BlendMode::Lighten => 5,
            BlendMode::ColorDodge => 6,
            BlendMode::ColorBurn => 7,
            BlendMode::HardLight => 8,
            BlendMode::SoftLight => 9,
            BlendMode::Difference => 10,
            BlendMode::Exclusion => 11,
            BlendMode::Hue => 12,
            BlendMode::Saturation => 13,
            BlendMode::Color => 14,
            BlendMode::Luminosity => 15,
        }
    }

    fn push_number(out: &mut String, value: f64, decimals: usize) {
        if !value.is_finite() {
            out.push('0');
            return;
        }
        let text = format!("{value:.decimals$}");
        let trimmed = if text.contains('.') { text.trim_end_matches('0').trim_end_matches('.') } else { &text };
        out.push_str(if trimmed.is_empty() || trimmed == "-" { "0" } else { trimmed });
    }

    fn push_numbers(out: &mut String, values: impl IntoIterator<Item = f64>, decimals: usize) {
        let mut first = true;
        for value in values {
            if !first {
                out.push(',');
            }
            first = false;
            push_number(out, value, decimals);
        }
    }

    fn push_affine(out: &mut String, transform: Affine, decimals: usize) {
        out.push('[');
        push_numbers(out, transform.as_coeffs(), decimals);
        out.push(']');
    }

    fn push_paint(out: &mut String, paint: &Paint) {
        let Paint::Solid(color) = paint;
        let rgba = color.to_rgba8();
        out.push_str(&format!("[{},{},{},{}]", rgba.r, rgba.g, rgba.b, rgba.a));
    }

    fn push_path_elements(out: &mut String, elements: &[PathEl], decimals: usize) {
        out.push_str(r#"["p",["#);
        let mut first = true;
        for element in elements {
            if !first {
                out.push(',');
            }
            first = false;
            match element {
                PathEl::MoveTo(p) => {
                    out.push('0');
                    out.push(',');
                    push_numbers(out, [p.x, p.y], decimals);
                }
                PathEl::LineTo(p) => {
                    out.push('1');
                    out.push(',');
                    push_numbers(out, [p.x, p.y], decimals);
                }
                PathEl::QuadTo(c, p) => {
                    out.push('2');
                    out.push(',');
                    push_numbers(out, [c.x, c.y, p.x, p.y], decimals);
                }
                PathEl::CurveTo(c1, c2, p) => {
                    out.push('3');
                    out.push(',');
                    push_numbers(out, [c1.x, c1.y, c2.x, c2.y, p.x, p.y], decimals);
                }
                PathEl::ClosePath => out.push('4'),
            }
        }
        out.push_str("]]");
    }

    fn push_shape(out: &mut String, shape: &RecordedShape, options: DrawListOptions) {
        let decimals = options.decimals;
        match shape {
            RecordedShape::Rect(rect) => {
                out.push_str(r#"["r","#);
                push_numbers(out, [rect.x0(), rect.y0(), rect.x1(), rect.y1()], decimals);
                out.push(']');
            }
            RecordedShape::RoundedRect(rounded) => {
                let rect = rounded.rect();
                let [tl, tr, br, bl] = rounded.radii().as_clockwise();
                out.push_str(r#"["rr","#);
                push_numbers(out, [rect.x0(), rect.y0(), rect.x1(), rect.y1(), tl, tr, br, bl], decimals);
                out.push(']');
            }
            RecordedShape::Circle(circle) => {
                let center = circle.center();
                out.push_str(r#"["ci","#);
                push_numbers(out, [center.x, center.y, circle.radius()], decimals);
                out.push(']');
            }
            RecordedShape::Line(line) => {
                let (p0, p1) = (line.p0(), line.p1());
                out.push_str(r#"["ln","#);
                push_numbers(out, [p0.x, p0.y, p1.x, p1.y], decimals);
                out.push(']');
            }
            RecordedShape::CubicBez(cubic) => {
                out.push_str(r#"["cb","#);
                push_numbers(out, [cubic.p0.x, cubic.p0.y, cubic.p1.x, cubic.p1.y, cubic.p2.x, cubic.p2.y, cubic.p3.x, cubic.p3.y], decimals);
                out.push(']');
            }
            RecordedShape::Arc(arc) => push_path_elements(out, &arc.path_elements(options.tolerance), decimals),
            RecordedShape::BezPath(path) => push_path_elements(out, &path.elements(), decimals),
        }
    }

    const BASE64_ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    /// 🔤️ First-party base64 for the one command that carries opaque bytes. Written here
    /// rather than taken from a crate because this module must stay dependency-free on every
    /// target it compiles for.
    fn push_base64(out: &mut String, bytes: &[u8]) {
        out.push('"');
        for chunk in bytes.chunks(3) {
            let b0 = u32::from(chunk[0]);
            let b1 = chunk.get(1).copied().map_or(0, u32::from);
            let b2 = chunk.get(2).copied().map_or(0, u32::from);
            let triple = (b0 << 16) | (b1 << 8) | b2;
            out.push(BASE64_ALPHABET[((triple >> 18) & 63) as usize] as char);
            out.push(BASE64_ALPHABET[((triple >> 12) & 63) as usize] as char);
            out.push(if chunk.len() > 1 { BASE64_ALPHABET[((triple >> 6) & 63) as usize] as char } else { '=' });
            out.push(if chunk.len() > 2 { BASE64_ALPHABET[(triple & 63) as usize] as char } else { '=' });
        }
        out.push('"');
    }

    fn push_command(out: &mut String, command: &SceneCommand, options: DrawListOptions) {
        let decimals = options.decimals;
        match command {
            SceneCommand::Fill { rule, transform, paint, shape, .. } => {
                out.push_str(r#"["f","#);
                out.push_str(&fill_rule_code(*rule).to_string());
                out.push(',');
                push_paint(out, paint);
                out.push(',');
                push_affine(out, *transform, decimals);
                out.push(',');
                push_shape(out, shape, options);
                out.push(']');
            }
            SceneCommand::Stroke { stroke, transform, paint, shape, .. } => {
                out.push_str(r#"["s","#);
                push_number(out, stroke.width, decimals);
                out.push_str(&format!(",{},{},", cap_code(stroke.start_cap), cap_code(stroke.end_cap)));
                out.push('[');
                push_numbers(out, stroke.dash_pattern.iter().copied(), decimals);
                out.push_str("],");
                push_number(out, stroke.dash_offset, decimals);
                out.push(',');
                push_paint(out, paint);
                out.push(',');
                push_affine(out, *transform, decimals);
                out.push(',');
                push_shape(out, shape, options);
                out.push(']');
            }
            SceneCommand::DrawImage { image, transform } => {
                out.push_str(&format!(r#"["i",{},{},"#, image.width(), image.height()));
                push_base64(out, &image.data);
                out.push(',');
                push_affine(out, *transform, decimals);
                out.push(']');
            }
            SceneCommand::PushLayer { rule, blend, alpha, transform, clip } => {
                out.push_str(r#"["pl","#);
                out.push_str(&format!("{},{},", fill_rule_code(*rule), blend_code(*blend)));
                push_number(out, f64::from(*alpha), decimals);
                out.push(',');
                push_affine(out, *transform, decimals);
                out.push(',');
                push_shape(out, clip, options);
                out.push(']');
            }
            SceneCommand::PushClipLayer { rule, transform, clip } => {
                out.push_str(r#"["pc","#);
                out.push_str(&format!("{},", fill_rule_code(*rule)));
                push_affine(out, *transform, decimals);
                out.push(',');
                push_shape(out, clip, options);
                out.push(']');
            }
            SceneCommand::PopLayer => out.push_str(r#"["po"]"#),
            #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
            SceneCommand::VelloFragment { .. } => out.push_str(r#"["po"]"#),
        }
    }

    /// 🎬️ Encodes every replayable command of `scene` as one JSON draw list.
    ///
    /// `{"version":1,"truncated":false,"commands":[…]}` — each command a tagged flat array:
    /// `["f",rule,[r,g,b,a],[a,b,c,d,e,f],shape]` fill, `["s",width,capStart,capEnd,[dash…],
    /// dashOffset,[r,g,b,a],[affine],shape]` stroke, `["i",w,h,"<base64 rgba8>",[affine]]`
    /// image, `["pl",rule,blend,alpha,[affine],clip]` layer, `["pc",rule,[affine],clip]` clip,
    /// `["po"]` pop. Rule `0` non-zero / `1` even-odd; cap `0` butt / `1` round / `2` square;
    /// blend is [`BlendMode`]'s own declaration order. Shapes: `["r",x0,y0,x1,y1]`,
    /// `["rr",x0,y0,x1,y1,tl,tr,br,bl]`, `["ci",cx,cy,r]`, `["ln",x0,y0,x1,y1]`,
    /// `["cb",8 coords]`, `["p",[verbs]]` with verb `0` move / `1` line / `2` quad / `3` cubic
    /// / `4` close.
    ///
    /// A `VelloFragment` (the one host-only escape hatch, produced solely by
    /// [`SvgDocument::append_to_scene`]) carries an opaque `vello` encoding with no
    /// first-party commands to read, so it encodes as a bare `["po"]` — a no-op a replayer
    /// skips — rather than silently shifting every later command's layer depth.
    pub fn scene_draw_list_json(scene: &Scene, options: DrawListOptions) -> String {
        let mut out = String::new();
        write_scene_draw_list(&mut out, scene, options);
        out
    }

    /// 🎬️ Appends the same encoding into a caller-owned buffer.
    ///
    /// The buffer is the point: a guest paints EVERY frame through here, and dlmalloc never
    /// returns freed pages to the module, so a fresh multi-kilobyte `String` per frame
    /// fragments the guest heap until it aborts. One buffer, cleared and refilled, keeps the
    /// allocation count per frame at zero once it has reached its working size.
    pub fn write_scene_draw_list(out: &mut String, scene: &Scene, options: DrawListOptions) {
        let truncated = scene.0.len() > options.maximum_commands;
        out.reserve(scene.0.len().min(options.maximum_commands) * 96 + 64);
        out.push_str(r#"{"version":"#);
        out.push_str(&DRAW_LIST_VERSION.to_string());
        out.push_str(if truncated { r#","truncated":true,"commands":["# } else { r#","truncated":false,"commands":["# });
        for (index, command) in scene.0.iter().take(options.maximum_commands).enumerate() {
            if index != 0 {
                out.push(',');
            }
            push_command(out, command, options);
        }
        out.push_str("]}");
    }
}
// #endregion 🔖️DrawList

/// 🏷️ Parsed SVG document for icon and label rasterization. Host/browser only — see
/// `vello_backend`'s `usvg` re-export docstring above; its only real callers are
/// `IconPaintCache::get_or_build`'s native arm and `#[cfg(test)]` code.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub struct SvgDocument(pub(crate) backend::usvg::Tree);

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl SvgDocument {
    pub(crate) fn from_tree(tree: backend::usvg::Tree) -> Self {
        Self(tree)
    }

    /// 🏷️ Appends the SVG tree into a scene. Builds a standalone real `vello::Scene`
    /// from the tree, then records it as one `SceneCommand::VelloFragment` — `Scene` itself
    /// stays a first-party command list even on this host/browser-only path.
    pub fn append_to_scene(&self, scene: &mut Scene) {
        let mut fragment = backend::Scene::new();
        vello_svg::append_tree(&mut fragment, &self.0);
        scene.0.push(SceneCommand::VelloFragment { scene: SharedArc::new(fragment), transform: Affine::IDENTITY });
    }
}

/// 🏷️ Appends a parsed SVG document into a scene.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn append_svg_document(scene: &mut Scene, doc: &SvgDocument) {
    doc.append_to_scene(scene);
}

#[cfg(test)]
include!("../../🧪️tests/🔬️renderer-opaque-scene-retirement/🦀️.rs");
