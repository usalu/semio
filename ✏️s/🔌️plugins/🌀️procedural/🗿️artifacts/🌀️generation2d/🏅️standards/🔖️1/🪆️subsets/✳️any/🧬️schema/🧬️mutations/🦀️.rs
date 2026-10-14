//! 🧬️ Generation2d artifact — semantic document mutation dispatch enum. Every variant is a
//! single-field tuple wrapping a handcrafted `protocol::MutationKind` payload: eight live in the
//! `🧬️mutations/<slug>/` triad leaves wired by `🦀️.rs` (their directory/module names are
//! leftovers of the generic slots they were repurposed from — see this ticket's wave2 report for
//! the glue.rs rename that would align them), the rest — those with no pre-wired slot — live inline
//! below as `mod <slug> { 🦠️mutation / 🔺️diff / ↩️inverse }` regions, same shape, same file.
//! `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<Generation2dSnapshot>` and
//! `impl protocol::SemanticMutation<Generation2dSnapshot>` from those payloads — no hand-written
//! apply/diff/inverse dispatch here.
#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate self as semio_s_artifact_procedural_2d_mutations;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
//#endregion 📖️SemioGrammar

use semio_s_artifact_procedural_2d_model::diff::Generation2dDiff;
use semio_s_artifact_procedural_2d_model::widget_id;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use protocol::Mutation;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_os_flow::FlowHost;
#[cfg(test)]
use semio_framework_artifact_playbook_playbook::FormGeneration;
use semio_framework_artifact_playbook_playbook::GenerationMutation;
use semio_framework_value_derive::{FromValue, ToValue};
use store::{os_store::store_schema::ArtifactEnvelope, ArtifactStore};

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = Generation2dSnapshot, diff = Generation2dDiff, schema = "generation.2d")]
pub enum Generation2dMutation {
    CreateWidget(semio_s_artifact_procedural_2d_mut_widgets::create_widget::CreateWidget),
    ReplaceWidget(semio_s_artifact_procedural_2d_mut_widgets::replace_widget::ReplaceWidget),
    DeleteWidget(semio_s_artifact_procedural_2d_mut_widgets::delete_widget::DeleteWidget),
    ConnectSynapse(semio_s_artifact_procedural_2d_mut_synapses::connect_synapse::ConnectSynapse),
    ReplaceSynapse(semio_s_artifact_procedural_2d_mut_synapses::replace_synapse::ReplaceSynapse),
    DisconnectSynapse(semio_s_artifact_procedural_2d_mut_synapses::disconnect_synapse::DisconnectSynapse),
    MoveWidget(semio_s_artifact_procedural_2d_mut_widgets::move_widget::MoveWidget),
    ClearWidgetLayout(semio_s_artifact_procedural_2d_mut_widgets::clear_widget_layout::ClearWidgetLayout),
    UpdateCamera(semio_s_artifact_procedural_2d_mut_document::update_camera::UpdateCamera),
    ChangeSchema(semio_s_artifact_procedural_2d_mut_document::change_schema::ChangeSchema),
    CreateGeneration(semio_s_artifact_procedural_2d_mut_generations::create_generation::CreateGeneration),
    DeleteGeneration(semio_s_artifact_procedural_2d_mut_generations::delete_generation::DeleteGeneration),
    RenameGeneration(semio_s_artifact_procedural_2d_mut_generations::rename_generation::RenameGeneration),
    ChangeGenerationValue(semio_s_artifact_procedural_2d_mut_generations::change_generation_value::ChangeGenerationValue),
    ChangeSliderValue(semio_s_artifact_procedural_2d_mut_widgets::change_slider_value::ChangeSliderValue),
    MoveNodes(semio_s_artifact_procedural_2d_mut_widgets::move_nodes::MoveNodes),
    SelectGeneration(semio_s_artifact_procedural_2d_mut_generations::select_generation::SelectGeneration),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`Generation2dMutation`] variant, in declaration order — the exact
/// vocabulary the `procedural-2d-1-any` mutation catalog (`../../🔣️oracle.json`) declares and
/// the `🌀️mutate-procedural-2d-1` exhaustive case measures itself against. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest against both.
pub const KINDS: &[&str] = &[
    "create-widget",
    "replace-widget",
    "delete-widget",
    "connect-synapse",
    "replace-synapse",
    "disconnect-synapse",
    "move-widget",
    "clear-widget-layout",
    "update-camera",
    "change-schema",
    "create-generation",
    "delete-generation",
    "rename-generation",
    "change-generation-value",
    "change-slider-value",
    "move-nodes",
    "select-generation",
];
//#endregion 🏷️Kinds

//#endregion 🔖️Mutations

//#region 🧊️Retirement
impl Generation2dMutation {
    /// 🧊️ Explicit cold-only disposal of one owned mutation. `create-widget`/`replace-widget` carry a
    /// whole `Widget`, whose `neural::Dictionary` (and `OrderedSet`/`Tree`) fail-close on a bare drop,
    /// so a dropped mutation aborts the process. Every other variant drops freely.
    pub fn retire_cold(self) {
        match self {
            Self::CreateWidget(semio_s_artifact_procedural_2d_mut_widgets::create_widget::CreateWidget { widget, .. }) | Self::ReplaceWidget(semio_s_artifact_procedural_2d_mut_widgets::replace_widget::ReplaceWidget { widget }) => widget.retire_cold(),
            _ => {}
        }
    }
}
//#endregion 🧊️Retirement

//#region 🔖️GenerationBridge
/// 🌉️ Bridges one `semio_framework_artifact_playbook_playbook::GenerationMutation` (the framework's own generation-editing
/// vocabulary — `Add`/`Remove`/`Rename`/`UpdateValues`) onto this facet's semantic
/// `Generation2dMutation` variants, so app-layer callers that already hold a `GenerationMutation`
/// (from `semio_framework_artifact_playbook_playbook::generation_operations`) need only swap the mapping function at the call
/// site, not learn this facet's internal triad-leaf module paths. Twin of generation3d's
/// `generation_mutation_to_generation3d` — the two facets' generation payloads differ only in field
/// naming (`name`/`value` here, `new_name`/`new_value` there).
pub fn generation_mutation_to_generation2d(operation: GenerationMutation) -> Generation2dMutation {
    match operation {
        GenerationMutation::Add { generation } => Generation2dMutation::CreateGeneration(semio_s_artifact_procedural_2d_mut_generations::create_generation::CreateGeneration { generation, index: None }),
        GenerationMutation::Remove { id } => Generation2dMutation::DeleteGeneration(semio_s_artifact_procedural_2d_mut_generations::delete_generation::DeleteGeneration { id }),
        GenerationMutation::Rename { id, name } => Generation2dMutation::RenameGeneration(semio_s_artifact_procedural_2d_mut_generations::rename_generation::RenameGeneration { id, name }),
        GenerationMutation::UpdateValues { id, question_id, value } => Generation2dMutation::ChangeGenerationValue(semio_s_artifact_procedural_2d_mut_generations::change_generation_value::ChangeGenerationValue { id, question_id, value }),
    }
}
//#endregion 🔖️GenerationBridge

pub type Generation2dEnvelope = ArtifactEnvelope<Generation2dSnapshot, Generation2dMutation>;
pub type Generation2dStore = ArtifactStore<Generation2dSnapshot, Generation2dMutation>;

/// ↩️ Computes a mutation's inverse against a projection — generic over every variant.
pub fn inverse_generation2d_mutation(projection: &Generation2dSnapshot, mutation: &Generation2dMutation) -> Result<Vec<Generation2dMutation>, semio_framework_value::ValueError> {
    Ok({
    mutation.inverse(projection)?

    })
}

//#region 🔖️HostGestures
/// 🔌️ Connects two ports through the host and returns the concrete leaves the host names: a `disconnect-synapse` for every wire
/// the host displaces from the target port, then one `connect-synapse` of the minted wire at the end of the list.
pub fn host_connect_ports(host: &mut FlowHost, from_id: &str, from_port: &str, to_id: &str, to_port: &str) -> Result<Vec<crate::Generation2dMutation>, String> {
    use semio_s_artifact_procedural_2d_mut_synapses::connect_synapse;
    use semio_s_artifact_procedural_2d_mut_synapses::disconnect_synapse;
    let displaced: Vec<String> = host.host_snapshot.synapses.iter().filter(|synapse| synapse.to == to_id && synapse.to_port == to_port).map(|synapse| synapse.id.clone()).collect();
    let id = host.connect_ports(from_id, from_port, to_id, to_port).map_err(|error| error.to_string())?;
    let minted = host.host_snapshot.synapses.iter().position(|synapse| synapse.id == id).ok_or_else(|| format!("the host minted no wire {id}"))?;
    let mut leaves: Vec<_> = displaced.into_iter().map(disconnect_synapse).collect();
    leaves.push(connect_synapse(minted, host.host_snapshot.synapses[minted].clone()));
    Ok(leaves)
}

/// ✂️ Cuts one wire through the host: the single `disconnect-synapse` leaf.
pub fn host_disconnect(host: &mut FlowHost, synapse_id: &str) -> Result<Vec<crate::Generation2dMutation>, String> {
    host.disconnect(synapse_id).map_err(|error| error.to_string())?;
    Ok(vec![semio_s_artifact_procedural_2d_mut_synapses::disconnect_synapse(synapse_id.to_string())])
}

/// 🗑️ Removes a widget through the host: a `disconnect-synapse` per wire it carried, its `clear-widget-layout` when it was placed,
/// then its `delete-widget`.
pub fn host_remove_widget(host: &mut FlowHost, widget_id: &str) -> Result<Vec<crate::Generation2dMutation>, String> {
    use semio_s_artifact_procedural_2d_mut_widgets::clear_widget_layout;
    use semio_s_artifact_procedural_2d_mut_widgets::delete_widget;
    use semio_s_artifact_procedural_2d_mut_synapses::disconnect_synapse;
    let wires: Vec<String> = host.host_snapshot.synapses.iter().filter(|synapse| synapse.from == widget_id || synapse.to == widget_id).map(|synapse| synapse.id.clone()).collect();
    let placed = host.host_snapshot.layout.contains_key(widget_id);
    host.remove_widget(widget_id).map_err(|error| error.to_string())?;
    let mut leaves: Vec<_> = wires.into_iter().map(disconnect_synapse).collect();
    if placed {
        leaves.push(clear_widget_layout(widget_id.to_string()));
    }
    leaves.push(delete_widget(widget_id.to_string()));
    Ok(leaves)
}

/// ➕️ Adds a widget through the host: the `create-widget` of what the host built from `descriptor_json` at the end of the list,
/// then the `move-widget` of the position the gesture named.
pub fn host_add_widget(host: &mut FlowHost, descriptor_json: &str, world_x: f64, world_y: f64) -> Result<Vec<crate::Generation2dMutation>, String> {
    use semio_s_artifact_procedural_2d_mut_widgets::create_widget;
    use semio_s_artifact_procedural_2d_mut_widgets::move_widget;
    let id = host.add_widget(descriptor_json, world_x, world_y).map_err(|error| error.to_string())?;
    let at = host.host_snapshot.widgets.iter().position(|widget| semio_s_artifact_procedural_2d_model::widget_id(widget) == id).ok_or_else(|| format!("the host built no widget {id}"))?;
    let widget = host.host_snapshot.widgets[at].clone();
    Ok(vec![create_widget(at, widget), move_widget(id, semio_framework_artifact_flow_flow::WidgetLayout { x: world_x, y: world_y })])
}

/// 🔢️ Inserts a port through the host: the `replace-widget` of the widget the host rebuilt, and a `replace-synapse` for each of its wires the
/// rebuild re-addressed.
pub fn host_insert_port(host: &mut FlowHost, widget_id: &str, input: bool, index: usize) -> Result<Vec<crate::Generation2dMutation>, String> {
    use semio_s_artifact_procedural_2d_mut_synapses::replace_synapse;
    use semio_s_artifact_procedural_2d_mut_widgets::replace_widget;
    let wires: Vec<_> = host.host_snapshot.synapses.iter().filter(|synapse| synapse.from == widget_id || synapse.to == widget_id).cloned().collect();
    if input {
        host.add_input_port(widget_id, index).map_err(|error| error.to_string())?;
    } else {
        host.add_output_port(widget_id, index).map_err(|error| error.to_string())?;
    }
    let at = host.host_snapshot.widgets.iter().position(|widget| semio_s_artifact_procedural_2d_model::widget_id(widget) == widget_id).ok_or_else(|| format!("the host lost widget {widget_id}"))?;
    let widget = host.host_snapshot.widgets[at].clone();
    let mut leaves = vec![replace_widget(widget)];
    for before in wires {
        if let Some(after) = host.host_snapshot.synapses.iter().find(|synapse| synapse.id == before.id).filter(|after| **after != before) {
            leaves.push(replace_synapse(after.clone()));
        }
    }
    Ok(leaves)
}

/// 🗺️ Reorganizes the graph through the host: one `move-widget` per widget whose placement the layout pass changed.
pub fn host_reorganize(host: &mut FlowHost, options_json: &str) -> Result<Vec<crate::Generation2dMutation>, String> {
    use semio_s_artifact_procedural_2d_mut_widgets::move_widget;
    let before: Vec<(String, semio_framework_artifact_flow_flow::WidgetLayout)> = host.host_snapshot.layout.iter().map(|(id, layout)| (id.clone(), layout.clone())).collect();
    let options: semio_framework_os_infinite::board::schema::layout::DagLayoutOptions = semio_framework_pack_json::from_json_str(options_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let mut progress = |_| true;
    let mut control = semio_framework_os_infinite::board::schema::layout::LayoutControl::new(100_000_000, &mut progress);
    host.reorganize(&options, &mut control).map_err(|error| error.to_string())?;
    Ok(host
        .host_snapshot
        .layout
        .iter()
        .filter(|(id, layout)| before.iter().find(|(held, _)| held == *id).map(|(_, previous)| previous) != Some(*layout))
        .map(|(id, layout)| move_widget(id.clone(), layout.clone()))
        .collect())
}
//#endregion 🔖️HostGestures

//#region 🧪️Tests

//#endregion 🧪️Tests

#[path = "../../../../../../🌉️apply/🦀️.rs"]
pub mod central_apply;
