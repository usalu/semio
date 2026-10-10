//! 📤️ Typed progress, cursors and widget results of semantic geometry evaluation.
use super::{Generation3dFaultRecord, Generation3dOutputRecord};
use crate::standards::v1::subsets::any::schema::catalogue::Quality;

pub const GEOMETRY_INFERENCE_SCHEMA: &str = "s.procedural.generation3d.geometry";
pub const GEOMETRY_ARTIFACT_KIND: &str = "s.procedural.generation3d";
pub const GEOMETRY_PROGRESS_UNIT: &str = "widget-step";
pub const GEOMETRY_PAYLOAD_SCHEMA: &str = "s.procedural.generation3d.geometry.payload";

/// 🧷️ Where a run stopped: the request it belongs to and the number of finished widgets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeometryCursor {
    pub digest: String,
    pub next: u32,
}

/// 🧮️ Finished widgets out of the plan, with the fraction including the widget in flight.
#[derive(Clone, Debug, PartialEq)]
pub struct GeometryProgress {
    pub completed: u32,
    pub total: u32,
    pub fraction: f64,
}

/// 📇️ One widget of a result: its dependency hash and the summary of its evaluation.
#[derive(Clone, Debug, PartialEq)]
pub struct GeometryWidgetResult {
    pub id: String,
    pub dep: String,
    pub quality: Quality,
    pub fault: Option<Generation3dFaultRecord>,
    pub outputs: Vec<Generation3dOutputRecord>,
}

/// 📤️ A geometry result. Handing it back as `previous_state` resumes the run it describes.
#[derive(Clone, Debug, PartialEq)]
pub struct GeometryResult {
    pub complete: bool,
    pub progress: GeometryProgress,
    pub computed: u32,
    pub cache_hits: u32,
    pub fuel_used: u32,
    pub faulted: u32,
    pub cursor: Option<GeometryCursor>,
    pub widgets: Vec<GeometryWidgetResult>,
}
//#endregion 🔖️Payload

