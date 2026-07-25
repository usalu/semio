//! 🖊️ Drawing kernel interface: scene nodes, styles, and export contracts.

pub mod compute {
    // #region compute
    //! ⚙️ Offload CPU-heavy drawing kernel work to the rayon thread pool.

    use std::future::Future;

    /// 🧵 Run a closure on the rayon pool (or inline when `parallel` is disabled).
    pub async fn run_blocking<F, R>(f: F) -> R
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        #[cfg(feature = "parallel")]
        {
            let (tx, rx) = futures::channel::oneshot::channel();
            rayon::spawn(move || {
                let _ = tx.send(f());
            });
            // Canceled only if the rayon worker panicked before sending; that panic already
            // surfaced once, so re-panicking here on the awaiting side is the correct terminal point.
            rx.await.expect("blocking task dropped")
        }
        #[cfg(not(feature = "parallel"))]
        {
            f()
        }
    }

    /// ⏳ Block the current thread until an async kernel call completes.
    pub fn block_on<F>(future: F) -> F::Output
    where
        F: Future,
    {
        pollster::block_on(future)
    }
    // #endregion compute
}

pub use compute::{block_on, run_blocking};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

// #region 🔖Types
/// 📐 Column vector `[x,y]`.
pub type Vec2 = [f64; 2];

/// 🧭 Drawing entity kind carried by a handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DrawingKind {
    Rect,
    Ellipse,
    Circle,
    Line,
    Polygon,
    Path,
    Text,
    Group,
}

/// 🧭 Opaque drawing handle (`drawing-3`, …).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DrawingHandle(pub String);

impl DrawingHandle {
    pub fn new(kind: DrawingKind, id: u32) -> Self {
        let prefix = match kind {
            DrawingKind::Rect => "rect",
            DrawingKind::Ellipse => "ellipse",
            DrawingKind::Circle => "circle",
            DrawingKind::Line => "line",
            DrawingKind::Polygon => "polygon",
            DrawingKind::Path => "path",
            DrawingKind::Text => "text",
            DrawingKind::Group => "group",
        };
        Self(format!("drawing-{prefix}-{id}"))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// ✏️ Path segment in local coordinates.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum PathSegment {
    Move { to: Vec2 },
    Line { to: Vec2 },
    Quad { ctrl: Vec2, to: Vec2 },
    Cubic { ctrl1: Vec2, ctrl2: Vec2, to: Vec2 },
    Arc { rx: f64, ry: f64, rotation: f64, large_arc: bool, sweep: bool, to: Vec2 },
    Close,
}

/// 🎨 Gradient color stop.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GradientStop {
    pub offset: f64,
    pub color: [f64; 4],
}

/// 🪣 Fill style.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum FillStyle {
    Solid { color: [f64; 4] },
    LinearGradient { x1: f64, y1: f64, x2: f64, y2: f64, stops: Vec<GradientStop> },
    RadialGradient { cx: f64, cy: f64, r: f64, stops: Vec<GradientStop> },
}

/// 🖌️ Stroke style.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StrokeStyle {
    pub color: [f64; 4],
    pub width: f64,
    pub cap: LineCap,
    pub join: LineJoin,
    #[serde(default)]
    pub dash: Vec<f64>,
}

/// 🔚 Line cap.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

/// 🔗 Line join.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

/// 📐 Affine 2D transform `[a,b,c,d,e,f]` mapping `(x,y)` to `(a*x+c*y+e, b*x+d*y+f)`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Affine2D(pub [f64; 6]);

impl Default for Affine2D {
    fn default() -> Self {
        Self([1.0, 0.0, 0.0, 1.0, 0.0, 0.0])
    }
}

impl Affine2D {
    pub fn identity() -> Self {
        Self::default()
    }

    pub fn translate(dx: f64, dy: f64) -> Self {
        Self([1.0, 0.0, 0.0, 1.0, dx, dy])
    }

    pub fn rotate(angle: f64) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self([cos, sin, -sin, cos, 0.0, 0.0])
    }

    pub fn scale(sx: f64, sy: f64) -> Self {
        Self([sx, 0.0, 0.0, sy, 0.0, 0.0])
    }

    pub fn multiply(self, other: Self) -> Self {
        let [a, b, c, d, e, f] = self.0;
        let [a2, b2, c2, d2, e2, f2] = other.0;
        Self([a * a2 + c * b2, b * a2 + d * b2, a * c2 + c * d2, b * c2 + d * d2, a * e2 + c * f2 + e, b * e2 + d * f2 + f])
    }

    pub fn transform_point(self, point: Vec2) -> Vec2 {
        let [a, b, c, d, e, f] = self.0;
        [a * point[0] + c * point[1] + e, b * point[0] + d * point[1] + f]
    }
}

/// 🧩 Scene-graph node variants.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DrawingNode {
    Rect { x: f64, y: f64, width: f64, height: f64 },
    Ellipse { cx: f64, cy: f64, rx: f64, ry: f64 },
    Circle { cx: f64, cy: f64, r: f64 },
    Line { x1: f64, y1: f64, x2: f64, y2: f64 },
    Polygon { points: Vec<Vec2> },
    Path { segments: Vec<PathSegment> },
    Text { x: f64, y: f64, content: String, size: f64 },
    Group { children: Vec<String> },
}

/// 🖼️ Flattened scene node for preview and export.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SceneNode {
    pub transform: Affine2D,
    pub node: DrawingNode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<FillStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<StrokeStyle>,
    #[serde(default = "default_opacity")]
    pub opacity: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip: Option<Vec<PathSegment>>,
}

fn default_opacity() -> f64 {
    1.0
}

/// 🎬 Serializable drawing scene transfer type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DrawingScene {
    pub width: f64,
    pub height: f64,
    pub nodes: Vec<SceneNode>,
}

impl Default for DrawingScene {
    fn default() -> Self {
        Self { width: 512.0, height: 512.0, nodes: Vec::new() }
    }
}

// #region ⚠️ Errors
/// ⚠️ Kernel operation error.
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum DrawingError {
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("missing handle: {0}")]
    MissingHandle(String),
    #[error("operation failed: {0}")]
    Operation(String),
}
// #endregion ⚠️ Errors
// #endregion 🔖Types

// #region 🔖Kernel
/// 🔌 Model-free 2D drawing kernel interface (fully async).
#[async_trait(?Send)]
pub trait DrawingKernel {
    // #region Primitives
    async fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) -> Result<DrawingHandle, DrawingError>;
    async fn ellipse(&mut self, cx: f64, cy: f64, rx: f64, ry: f64) -> Result<DrawingHandle, DrawingError>;
    async fn circle(&mut self, cx: f64, cy: f64, r: f64) -> Result<DrawingHandle, DrawingError>;
    async fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) -> Result<DrawingHandle, DrawingError>;
    async fn polygon(&mut self, points: &[Vec2]) -> Result<DrawingHandle, DrawingError>;
    // #endregion Primitives

    // #region Paths
    async fn polyline_path(&mut self, points: &[Vec2]) -> Result<DrawingHandle, DrawingError>;
    async fn rect_path(&mut self, x: f64, y: f64, width: f64, height: f64) -> Result<DrawingHandle, DrawingError>;
    // #endregion Paths

    // #region Style
    async fn set_fill(&mut self, handle: &DrawingHandle, fill: FillStyle) -> Result<DrawingHandle, DrawingError>;
    async fn set_stroke(&mut self, handle: &DrawingHandle, stroke: StrokeStyle) -> Result<DrawingHandle, DrawingError>;
    async fn linear_gradient_fill(&mut self, handle: &DrawingHandle, x1: f64, y1: f64, x2: f64, y2: f64, stops: &[GradientStop]) -> Result<DrawingHandle, DrawingError>;
    // #endregion Style

    // #region Transforms
    async fn translate(&mut self, handle: &DrawingHandle, dx: f64, dy: f64) -> Result<DrawingHandle, DrawingError>;
    async fn rotate(&mut self, handle: &DrawingHandle, angle: f64) -> Result<DrawingHandle, DrawingError>;
    async fn scale(&mut self, handle: &DrawingHandle, sx: f64, sy: f64) -> Result<DrawingHandle, DrawingError>;
    // #endregion Transforms

    // #region Group
    async fn group(&mut self, children: &[DrawingHandle]) -> Result<DrawingHandle, DrawingError>;
    // #endregion Group

    // #region Booleans
    async fn bool_union(&mut self, a: &DrawingHandle, b: &DrawingHandle) -> Result<DrawingHandle, DrawingError>;
    async fn bool_difference(&mut self, a: &DrawingHandle, b: &DrawingHandle) -> Result<DrawingHandle, DrawingError>;
    async fn bool_intersection(&mut self, a: &DrawingHandle, b: &DrawingHandle) -> Result<DrawingHandle, DrawingError>;
    async fn bool_xor(&mut self, a: &DrawingHandle, b: &DrawingHandle) -> Result<DrawingHandle, DrawingError>;
    async fn bool_op_many(&mut self, operation: &str, handles: &[DrawingHandle]) -> Result<DrawingHandle, DrawingError>;
    async fn boolean_segments(&self, a: &[PathSegment], b: &[PathSegment], operation: &str) -> Result<Vec<PathSegment>, DrawingError>;
    // #endregion Booleans

    // #region Trace
    async fn trace_bitmap(&mut self, width: u32, height: u32, mask_or_luma: &[u8], threshold: f64, simplify_epsilon: f64) -> Result<DrawingHandle, DrawingError>;
    // #endregion Trace

    // #region Text
    async fn text(&mut self, x: f64, y: f64, content: &str, size: f64) -> Result<DrawingHandle, DrawingError>;
    // #endregion Text

    // #region Clip
    async fn apply_clip(&mut self, target: &DrawingHandle, clip: &DrawingHandle) -> Result<DrawingHandle, DrawingError>;
    // #endregion Clip

    // #region Export
    async fn flatten_scene(&self, handle: &DrawingHandle) -> Result<DrawingScene, DrawingError>;
    async fn export_svg(&self, handle: &DrawingHandle) -> Result<String, DrawingError>;
    async fn export_pdf(&self, handle: &DrawingHandle) -> Result<Vec<u8>, DrawingError>;
    async fn export_dwg(&self, handle: &DrawingHandle) -> Result<Vec<u8>, DrawingError>;
    async fn import_dwg(&mut self, data: &[u8]) -> Result<DrawingHandle, DrawingError>;
    // #endregion Export

    // #region Core
    async fn kind(&self, handle: &DrawingHandle) -> Result<DrawingKind, DrawingError>;
    async fn dispose(&mut self, handle: &DrawingHandle);
    fn retain_sync(&mut self, live: &std::collections::HashSet<String>);
    // #endregion Core
}
// #endregion 🔖Kernel

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn affine_multiplies_identity() {
        let point = [3.0, 4.0];
        let moved = Affine2D::translate(1.0, 2.0).multiply(Affine2D::identity()).transform_point(point);
        assert_eq!(moved, [4.0, 6.0]);
    }
}
// #endregion 🔖Tests
