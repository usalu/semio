//! 🧊️ Real extension sessions drive the public Generation3d app law corpus.
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
use semio_s_artifact_procedural_generation3d::*;
use semio_framework_plugin::*;
use semio_framework_os_flow::*;
use semio_framework_artifact_flow_flow::{FlowHostSnapshot, FlowGraphStep, Widget};
use semio_framework_os_flow::forms_bridge::flow_host_snapshot_to_form_spec;
use serde_json::Value;

/// 📸️ A concrete law fixture owns and retires each projection read.
pub(crate) struct SnapshotRead(Option<Generation3dSnapshot>);

impl SnapshotRead {
    pub(crate) fn new(snapshot: Generation3dSnapshot) -> Self { Self(Some(snapshot)) }
    pub(crate) fn into_inner(mut self) -> Generation3dSnapshot { self.0.take().expect("projection read owns its snapshot") }
}

impl std::ops::Deref for SnapshotRead {
    type Target = Generation3dSnapshot;
    fn deref(&self) -> &Self::Target { self.0.as_ref().expect("projection read owns its snapshot") }
}

impl std::ops::DerefMut for SnapshotRead {
    fn deref_mut(&mut self) -> &mut Self::Target { self.0.as_mut().expect("projection read owns its snapshot") }
}

impl Drop for SnapshotRead {
    fn drop(&mut self) { if let Some(snapshot) = self.0.take() { snapshot.retire_cold(); } }
}

impl PartialEq for SnapshotRead {
    fn eq(&self, other: &Self) -> bool { **self == **other }
}

impl PartialEq<Generation3dSnapshot> for SnapshotRead {
    fn eq(&self, other: &Generation3dSnapshot) -> bool { **self == *other }
}

impl std::fmt::Debug for SnapshotRead {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Debug::fmt(&**self, formatter) }
}

/// 🧭️ One coordinate-literal parameter (`{x, y, z}`) of a graph operator, or `default` when the operator or the
/// parameter is absent — what a gumball law reads off the transform operator its gesture composed into.
pub(crate) fn gumball_param_vector(snapshot: &FlowHostSnapshot, id: &str, key: &str, default: [f64; 3]) -> [f64; 3] {
    let widget = snapshot.widgets.iter().find(|widget| widget_id(widget) == id).map(semio_framework_value::ToValue::to_value);
    let literal = widget.as_ref().and_then(|widget| widget.get("params")).and_then(|params| params.get(key));
    std::array::from_fn(|axis| literal.and_then(|value| value.get(["x", "y", "z"][axis])).and_then(semio_framework_value::DslValue::as_f64).unwrap_or(default[axis]))
}

/// 🔢️ One number-literal parameter (`{value}`) of a graph operator, or `default` when the operator or the parameter is
/// absent.
pub(crate) fn gumball_param_number(snapshot: &FlowHostSnapshot, id: &str, key: &str, default: f64) -> f64 {
    let widget = snapshot.widgets.iter().find(|widget| widget_id(widget) == id).map(semio_framework_value::ToValue::to_value);
    widget.as_ref().and_then(|widget| widget.get("params")).and_then(|params| params.get(key)).and_then(|literal| literal.get("value")).and_then(semio_framework_value::DslValue::as_f64).unwrap_or(default)
}

#[path = "🧪️tests/🔒️serial-lock-discipline/🦀️.rs"]
pub(crate) mod serial_lock_discipline;

#[path = "🧪️tests/🔬️brep-extension/🦀️.rs"]
pub(crate) mod brep_extension;

#[path = "🧪️tests/🔬️publication-authority/🦀️.rs"]
pub(crate) mod publication_authority;

#[path = "🧪️tests/🔬️serial/🦀️.rs"]
pub(crate) mod test_serial;

include!("🔭️viewer-laws/🦀️.rs");
include!("🪟️editor-mode-laws/🦀️.rs");
include!("🧭️editor-structure-laws/🦀️.rs");
include!("🎮️editor-command-laws/🦀️.rs");

#[path = "🧪️tests/🔬️flow-operators/🦀️.rs"]
pub(crate) mod flow_operators;
