//! 💎️ The values a geometry widget computes and the faults it can raise — typed, in memory, never text.

use crate::standards::v1::subsets::any::schema::catalogue::{Localized, Quality, ShapeKind};
use semio_framework_3d::brep::engine::{BrepError, GeometryKind, ShapeValue};
use semio_framework_3d::brep::representation::error::KernelError;
use semio_framework_3d::mesh::HalfedgeMesh;
use std::collections::BTreeMap;
use std::sync::Arc;

pub const FAULT_PREFIX: &str = "generation3d.geometry.";

//#region 🔖️Fault
/// 🚫️ Why a widget produced no outputs: a stable code, EN+DE text and the input port it concerns.
#[derive(Clone, Debug, PartialEq)]
pub struct WidgetFault {
    pub code: String,
    pub message: Localized,
    pub port: Option<String>,
}

impl WidgetFault {
    /// 🏗️ A fault with English and German text.
    pub fn new(code: impl Into<String>, en: impl Into<String>, de: impl Into<String>) -> Self {
        Self { code: code.into(), message: Localized { en: en.into(), de: de.into() }, port: None }
    }

    /// 🔌️ The same fault attributed to an input port.
    pub fn at(mut self, port: impl Into<String>) -> Self {
        self.port = Some(port.into());
        self
    }
}

impl std::fmt::Display for WidgetFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message.en)
    }
}

impl std::error::Error for WidgetFault {}

/// 🛠️ The fault of a refused kernel call; the kernel's own message is appended in both languages.
pub fn kernel_fault(error: &BrepError) -> WidgetFault {
    WidgetFault::new(format!("{FAULT_PREFIX}kernel"), format!("The geometry kernel refused the operation: {error}."), format!("Der Geometriekern hat die Operation abgelehnt: {error}."))
}

/// 🛠️ The localized fault of a refused kernel body operation, worded the way the engine words its own conversion.
pub fn body_fault(error: &KernelError) -> WidgetFault {
    kernel_fault(&BrepError::Operation(error.to_string()))
}
//#endregion 🔖️Fault

//#region 🔖️Value
/// 🧭️ A plane by a point on it and its normal.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlaneValue {
    pub origin: [f64; 3],
    pub normal: [f64; 3],
}

/// 🎯️ The element type a selection refers to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SelectionKind {
    Face,
    Edge,
    Vertex,
}

impl SelectionKind {
    /// 🏷️ The catalogue spelling of the kind.
    pub fn name(self) -> &'static str {
        match self {
            Self::Face => "face",
            Self::Edge => "edge",
            Self::Vertex => "vertex",
        }
    }

    /// 🔎️ The kind named by a catalogue enum option.
    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "face" => Some(Self::Face),
            "edge" => Some(Self::Edge),
            "vertex" => Some(Self::Vertex),
            _ => None,
        }
    }
}

/// 🧲️ Sub-elements of a shape (B-Rep persistent label numbers) or of a mesh (component indices).
#[derive(Clone, Debug, PartialEq)]
pub struct SelectionValue {
    pub component: SelectionKind,
    pub ids: Vec<u64>,
}

/// 💎️ One value on a widget port.
#[derive(Clone, Debug)]
pub enum GeometryValue {
    Number(f64),
    Integer(i64),
    Boolean(bool),
    Text(String),
    Vector([f64; 3]),
    Point([f64; 3]),
    Plane(PlaneValue),
    Shape(Arc<ShapeValue>),
    Mesh(Arc<HalfedgeMesh>),
    Selection(SelectionValue),
    List(Vec<GeometryValue>),
}

/// 🟰️ Meshes are sealed shared values, so two mesh values are equal exactly when they are the same allocation.
impl PartialEq for GeometryValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a == b,
            (Self::Integer(a), Self::Integer(b)) => a == b,
            (Self::Boolean(a), Self::Boolean(b)) => a == b,
            (Self::Text(a), Self::Text(b)) => a == b,
            (Self::Vector(a), Self::Vector(b)) | (Self::Point(a), Self::Point(b)) => a == b,
            (Self::Plane(a), Self::Plane(b)) => a == b,
            (Self::Shape(a), Self::Shape(b)) => a == b,
            (Self::Mesh(a), Self::Mesh(b)) => Arc::ptr_eq(a, b),
            (Self::Selection(a), Self::Selection(b)) => a == b,
            (Self::List(a), Self::List(b)) => a == b,
            _ => false,
        }
    }
}

/// 📇️ What a value is, in one line a result payload can carry without the geometry itself.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValueSummary {
    pub kind: String,
    pub detail: String,
}

impl GeometryValue {
    /// 🧱️ A shared B-Rep shape value.
    pub fn shape(value: ShapeValue) -> Self {
        Self::Shape(Arc::new(value))
    }

    /// 🧱️ A shared half-edge mesh value.
    pub fn mesh(value: HalfedgeMesh) -> Self {
        Self::Mesh(Arc::new(value))
    }

    /// 🏷️ The variant's catalogue-facing name.
    pub fn kind_name(&self) -> &'static str {
        match self {
            Self::Number(_) => "number",
            Self::Integer(_) => "integer",
            Self::Boolean(_) => "boolean",
            Self::Text(_) => "text",
            Self::Vector(_) => "vector",
            Self::Point(_) => "point",
            Self::Plane(_) => "plane",
            Self::Shape(_) => "shape",
            Self::Mesh(_) => "mesh",
            Self::Selection(_) => "selection",
            Self::List(_) => "list",
        }
    }

    /// ⚖️ An estimate of the bytes the value holds, for cache accounting.
    pub fn byte_len(&self) -> usize {
        match self {
            Self::Number(_) | Self::Integer(_) | Self::Boolean(_) => 16,
            Self::Text(text) => 24 + text.len(),
            Self::Vector(_) | Self::Point(_) => 40,
            Self::Plane(_) => 64,
            Self::Shape(shape) => 256 + 160 * (shape.body.vertices.len() + shape.body.edges.len() + shape.body.faces.len() + shape.body.shells.len() + shape.body.solids.len()),
            Self::Mesh(mesh) => 256 + 48 * mesh.vertex_count() + 24 * mesh.halfedge_count() + 16 * mesh.face_count(),
            Self::Selection(selection) => 32 + 8 * selection.ids.len(),
            Self::List(items) => 24 + items.iter().map(Self::byte_len).sum::<usize>(),
        }
    }

    /// 📇️ A one-line summary: numbers and text print themselves, shapes name their kind and content hash, meshes their counts.
    pub fn summary(&self) -> ValueSummary {
        let detail = match self {
            Self::Number(value) => format!("{value}"),
            Self::Integer(value) => format!("{value}"),
            Self::Boolean(value) => format!("{value}"),
            Self::Text(value) => value.clone(),
            Self::Vector(axes) | Self::Point(axes) => format!("[{}, {}, {}]", axes[0], axes[1], axes[2]),
            Self::Plane(plane) => format!("origin [{}, {}, {}] normal [{}, {}, {}]", plane.origin[0], plane.origin[1], plane.origin[2], plane.normal[0], plane.normal[1], plane.normal[2]),
            Self::Shape(shape) => format!("{:?} {}", shape.kind(), shape.content_hash()).to_lowercase(),
            Self::Mesh(mesh) => format!("{} vertices, {} faces", mesh.vertex_count(), mesh.face_count()),
            Self::Selection(selection) => format!("{} {}", selection.ids.len(), selection.component.name()),
            Self::List(items) => format!("{} items", items.len()),
        };
        ValueSummary { kind: self.kind_name().to_string(), detail }
    }
}

/// 🧱️ The catalogue shape kind of a kernel geometry kind.
pub fn shape_kind(kind: GeometryKind) -> ShapeKind {
    match kind {
        GeometryKind::Vertex => ShapeKind::Vertex,
        GeometryKind::Edge => ShapeKind::Edge,
        GeometryKind::Wire => ShapeKind::Wire,
        GeometryKind::Face => ShapeKind::Face,
        GeometryKind::Shell => ShapeKind::Shell,
        GeometryKind::Solid => ShapeKind::Solid,
        GeometryKind::Compound => ShapeKind::Compound,
        GeometryKind::Curve => ShapeKind::Curve,
        GeometryKind::Surface => ShapeKind::Surface,
    }
}
//#endregion 🔖️Value

//#region 🔖️Evaluation
/// 🧮️ Outputs by catalogue output port name.
pub type Outputs = BTreeMap<String, GeometryValue>;

/// 🧮️ Builds outputs from `(port, value)` pairs.
pub fn outputs<const N: usize>(entries: [(&str, GeometryValue); N]) -> Outputs {
    entries.into_iter().map(|(port, value)| (port.to_string(), value)).collect()
}

/// 📦️ The result of one widget: its outputs, or the fault that left it without, and the fidelity it was computed with.
#[derive(Clone, Debug, PartialEq)]
pub struct WidgetEvaluation {
    pub outputs: Outputs,
    pub fault: Option<WidgetFault>,
    pub quality: Quality,
}

impl WidgetEvaluation {
    /// ✅️ A successful evaluation.
    pub fn ok(outputs: Outputs, quality: Quality) -> Self {
        Self { outputs, fault: None, quality }
    }

    /// 🚫️ A faulted evaluation: no outputs.
    pub fn faulted(fault: WidgetFault, quality: Quality) -> Self {
        Self { outputs: Outputs::new(), fault: Some(fault), quality }
    }

    /// ⚖️ An estimate of the bytes the evaluation holds, for cache accounting.
    pub fn byte_len(&self) -> usize {
        96 + self.outputs.iter().map(|(port, value)| port.len() + value.byte_len()).sum::<usize>() + self.fault.as_ref().map_or(0, |fault| fault.code.len() + fault.message.en.len() + fault.message.de.len())
    }
}
//#endregion 🔖️Evaluation
