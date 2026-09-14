//! 🧭️ 🕸️ Generation3d play app commands command family — `navigate-graph`.
//!
//! The five keyboard verbs of the node graph, kept in ONE leaf because they are one traversal seen
//! from five directions: every row below is the same anchor reduction, the same
//! [`semio_framework_artifact_flow_flow::keyboard_step`] and the same selection write, differing only
//! in which [`FlowGraphStep`] it names. Splitting them across five leaves would copy that body five
//! times.
//!
//! 🕹️ Selection is framework-owned (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), so a
//! step emits ONE `InteractionWrite` and touches nothing else: `VcsArtifactApp` applies it through
//! the very same `protocol::next_selection` machine a pointer pick goes through, which is what makes
//! the Artifact panel tree's highlight, the inspection panel and the preview gumball follow an arrow key
//! exactly as they follow a click. Declared `ArtifactToolPublicationLane::Interaction`, no document
//! and no config lane — a traversal is not an edit and must never enter undo.
//!
//! @see ../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::{FlowFixture, FlowGraphStep};
use semio_framework_plugin::{ArtifactView, Emit, InteractionWrite};
use std::collections::BTreeMap;

/// 🕹️ The `graph` interaction domain and the two granularities keyboard traversal moves between —
/// the same strings `Generation3dPlayApp::interaction_topology` publishes every widget and port under.
pub const GRAPH_DOMAIN: &str = "graph";
pub const NODE_GRANULARITY: &str = "node";
pub const HANDLE_GRANULARITY: &str = "handle";

/// 🧭️ The node a keyboard step starts from, reduced out of whatever the `graph` domain currently
/// holds selected.
///
/// Three reductions, each a rule the shared fixture states: a port handle (`{node}@{port}`, the id
/// shape `interaction_topology` mints and `activateSelection` selects) answers with the node that
/// owns it, so arrowing after opening a node closes it back onto the node; a multi-selection answers
/// with its first member in READING order rather than whichever id the set happened to hold first;
/// and an id the document no longer carries is no anchor at all, so the step re-enters the graph
/// instead of refusing.
pub(crate) fn anchor_node<'a>(fixture: &'a FlowFixture, selected: &[String]) -> Option<&'a str> {
    let order = fixture.keyboard_order();
    selected
        .iter()
        .filter_map(|id| {
            let node = id.split_once('@').map_or(id.as_str(), |(node, _)| node);
            order.iter().position(|known| *known == node).map(|at| (at, order[at]))
        })
        .min_by_key(|(at, _)| *at)
        .map(|(_, id)| id)
}

/// 🧭️ One step's whole emit: the selection write that moves the graph selection, or nothing at all
/// when the step has nowhere to go (a source node asked for its upstream). An empty emit is the
/// honest answer — a `Replace` naming the node already selected would cost a publication to change
/// nothing.
pub(crate) fn step_target<'a>(fixture: &'a FlowFixture, selected: &[String], step: FlowGraphStep) -> Option<&'a str> {
    let anchor = anchor_node(fixture, selected);
    fixture.keyboard_step(anchor, step).filter(|target| Some(*target) != anchor)
}

pub(crate) fn step_emit(doc: &ArtifactView<'_, Generation3dSnapshot>, selected: &[String], step: FlowGraphStep) -> Emit<Generation3dMutation, Generation3dConfigMutation> {
    let Some(target) = step_target(&doc.snapshot.fixture, selected, step) else { return Emit::default() };
    Emit { interaction_writes: vec![InteractionWrite::replace(GRAPH_DOMAIN, NODE_GRANULARITY, [target.to_string()])], ..Default::default() }
}

/// ⏎️ Opens the anchor node: replaces the selection with that node's own port handles, in the order
/// `interaction_topology` publishes them (inputs, then outputs).
///
/// This is the canvas's `Trigger::Activate` body — `role="application"` promises a surface handles
/// its own Enter/Space, and the React interpreter's `SurfaceAccessibilityShell` dispatches exactly
/// this verb for them. Opening a node at `handle` granularity is what lets a keyboard user see what a
/// node connects without a pointer; any arrow afterwards reduces the port back to its owning node
/// ([`anchor_node`]), so the node closes itself and traversal continues, and `escape` →
/// `clearSelection` (framework-minted) backs all the way out. A node with no visible port, and a node
/// already open, both emit nothing.
pub(crate) fn activate_ports<'a>(fixture: &FlowFixture, ports_by_node: &'a BTreeMap<String, Vec<String>>, selected: &[String]) -> Option<&'a Vec<String>> {
    let anchor = anchor_node(fixture, selected)?;
    let ports = ports_by_node.get(anchor).filter(|ports| !ports.is_empty())?;
    (selected.len() != ports.len() || !selected.iter().all(|id| ports.contains(id))).then_some(ports)
}

pub(crate) fn activate_emit(doc: &ArtifactView<'_, Generation3dSnapshot>, ports_by_node: &BTreeMap<String, Vec<String>>, selected: &[String]) -> Emit<Generation3dMutation, Generation3dConfigMutation> {
    let Some(ports) = activate_ports(&doc.snapshot.fixture, ports_by_node, selected) else { return Emit::default() };
    Emit { interaction_writes: vec![InteractionWrite::replace(GRAPH_DOMAIN, HANDLE_GRANULARITY, ports.iter().cloned())], ..Default::default() }
}

/// 🧭️ Declares one keyboard verb: its `dsl` payload and its `app_commands!` `handle` — the macro's
/// framework-fixed four-argument shape, which carries no `interaction` slot, so this path sees no
/// selection and a step enters the graph at its first or last node. The real-selection entry point is
/// `apply_selected` below, called by `generation3d_retained_reduce`.
macro_rules! graph_keyboard_command {
    ($module:ident, $Payload:ident, $keyword:literal, $step:expr, $doc:literal) => {
        #[doc = $doc]
        pub mod $module {
            use super::{Generation3dConfig, Generation3dConfigMutation, Generation3dMutation, Generation3dSnapshot};
            use semio_framework_artifact_flow_flow::FlowGraphStep;
            use semio_framework_os_flow::FlowEvalSession;
            use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
            use semio_framework_value_derive::{FromValue, ToValue};

            /// 🧭️ The [`FlowGraphStep`] this verb names — the one thing that differs between the four.
            pub const STEP: FlowGraphStep = $step;

            #[doc = $doc]
            #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
            #[dsl(keyword = $keyword)]
            pub struct $Payload {}

            pub fn handle(_payload: &$Payload, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
                Ok(apply_selected(doc, &[]))
            }

            /// 🕹️ Retained-command-job entry point (`generation3d_retained_reduce`, editor `🦀️.rs`) —
            /// the real `graph` selection, handed in as raw ids because plugin code cannot construct an
            /// `app::InteractionView` (its fields are `pub(crate)` to the framework crate).
            pub(crate) fn apply_selected(doc: &ArtifactView<'_, Generation3dSnapshot>, selected: &[String]) -> Emit<Generation3dMutation, Generation3dConfigMutation> {
                super::step_emit(doc, selected, STEP)
            }
        }
    };
}

graph_keyboard_command!(select_next_node, SelectNextNode, "select-next-node", FlowGraphStep::Next, "⬇️ Selects the next node in reading order, wrapping at the end.");
graph_keyboard_command!(select_previous_node, SelectPreviousNode, "select-previous-node", FlowGraphStep::Previous, "⬆️ Selects the previous node in reading order, wrapping at the start.");
graph_keyboard_command!(select_upstream_node, SelectUpstreamNode, "select-upstream-node", FlowGraphStep::Upstream, "⬅️ Follows a wire INTO the anchor node and selects its source.");
graph_keyboard_command!(select_downstream_node, SelectDownstreamNode, "select-downstream-node", FlowGraphStep::Downstream, "➡️ Follows a wire OUT of the anchor node and selects its target.");

/// ⏎️ The activate verb. Its ports come from the editor's own `generation3d_port_ids_by_node`, which
/// needs the flow host, so — unlike the four steps — its real body is reached only through
/// `apply_ports`; the selection-free macro path can name no port and emits nothing.
pub mod activate_selection {
    use super::{Generation3dConfig, Generation3dConfigMutation, Generation3dMutation, Generation3dSnapshot};
    use semio_framework_os_flow::FlowEvalSession;
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
    use semio_framework_value_derive::{FromValue, ToValue};

    /// ⏎️ See [`super::activate_emit`].
    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "activate-selection")]
    pub struct ActivateSelection {}

    pub fn handle(_payload: &ActivateSelection, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
        Ok(Emit::default())
    }

    /// 🕹️ Retained-command-job entry point (`generation3d_retained_reduce`, editor `🦀️.rs`).
    pub(crate) fn apply_ports(doc: &ArtifactView<'_, Generation3dSnapshot>, ports_by_node: &std::collections::BTreeMap<String, Vec<String>>, selected: &[String]) -> Emit<Generation3dMutation, Generation3dConfigMutation> {
        super::activate_emit(doc, ports_by_node, selected)
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
