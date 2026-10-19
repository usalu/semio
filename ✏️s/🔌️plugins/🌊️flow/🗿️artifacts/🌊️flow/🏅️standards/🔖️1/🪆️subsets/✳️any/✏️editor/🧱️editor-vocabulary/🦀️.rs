//! 🧱️ Flow editor core: the app and window ids, the action-value factory, the "graph" interaction vocabulary and the store capacities every taxonomy node of the editor shares.

//#region 🔖️Constants
pub const FLOW_PLAY_APP_ID: &str = "flow-play";
pub const FLOW_PLAY_WINDOW_MAIN: &str = "flow-main";
pub const FLOW_PLAY_WINDOW_GENERATIONS: &str = "flow-generations";
pub const FLOW_PLAY_WINDOW_GENERATE_FORM: &str = "flow-generate-form";
pub const FLOW_PLAY_WINDOW_GENERATE_PREVIEW: &str = "flow-generate-preview";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`☑️options/*`, `📌️panels/*`) builds its `on_change`/item actions with.
pub fn flow_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(FLOW_PLAY_APP_ID).action(action, args)
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}

/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🌳️ The framework's one node-list admission — the crate-local copy is gone (ticket
/// 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING §8.3).
pub use semio_framework_plugin::ui_node_list;

/// 🕹️ A domain pick row: [`semio_framework_plugin::tree_item_desc`] plus the granularity that marks
/// the row a pick target of the panel tree's `.interaction_domain(..)`, so no per-row
/// `interactionSelect` argument map is ever built.
pub fn pick_item<I: AsRef<str>, L: TryInto<semio_framework_ui_contract::Label>>(
    id: I,
    label: L,
    description: Option<String>,
    granularity: &str,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut node = semio_framework_plugin::tree_item_desc(id, label, description)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
        props.granularity = Some(semio_framework_plugin::UiText::try_from_str(granularity).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "interaction granularity admission failed"))?);
    }
    Ok(node)
}
//#endregion 🔖️Constants

//#region 🔖️Interaction
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the single "graph" interaction
/// domain this app declares — node/edge/handle granularities over the node-graph canvas.
pub const FLOW_INTERACTION_GRAPH: &str = "graph";

/// 🕹️ The document panel tree's own row id prefix for "node"-granularity targets (widgets) — see
/// `document_panel::render`'s doc comment; `interaction_topology` registers the SAME ids.
pub const FLOW_GRAPH_NODE_TARGET_PREFIX: &str = "flow-play-document.widget.";
/// 🕹️ Same as `FLOW_GRAPH_NODE_TARGET_PREFIX`, for "edge"-granularity targets (synapses).
pub const FLOW_GRAPH_EDGE_TARGET_PREFIX: &str = "flow-play-document.synapse.";
/// 🕹️ The scoped prefix for transient graph handle targets published by the canvas.
pub const FLOW_GRAPH_HANDLE_TARGET_PREFIX: &str = "flow-play-document.handle.";

/// 🕹️ The "graph" domain's row id for a widget (node granularity).
pub fn flow_graph_node_target_id(widget_id: &str) -> String {
    format!("{FLOW_GRAPH_NODE_TARGET_PREFIX}{widget_id}")
}

/// 🕹️ The "graph" domain's row id for a synapse (edge granularity).
pub fn flow_graph_edge_target_id(synapse_id: &str) -> String {
    format!("{FLOW_GRAPH_EDGE_TARGET_PREFIX}{synapse_id}")
}

/// 🕹️ Splits the "graph" domain's live `InteractionTarget` ids into (widget ids, synapse ids) — the
/// reverse of `flow_graph_node_target_id`/`flow_graph_edge_target_id`, mirroring note's
/// `block_id_from_tree_row_id`. "handle" targets have no persisted document data to resolve against —
/// no live UI populates them yet (the shared `NodeGraph` canvas renderer that would is framework layer,
/// unmigrated this wave) — so they never appear in either returned list.
pub fn flow_graph_selection_domains(selected: &[String]) -> (Vec<String>, Vec<String>) {
    let nodes = selected.iter().filter_map(|id| id.strip_prefix(FLOW_GRAPH_NODE_TARGET_PREFIX).map(str::to_string)).collect();
    let edges = selected.iter().filter_map(|id| id.strip_prefix(FLOW_GRAPH_EDGE_TARGET_PREFIX).map(str::to_string)).collect();
    (nodes, edges)
}
//#endregion 🔖️Interaction

//#region 📏️StoreCapacity
pub const FLOW_STORE_MAX_SCENE_ITEMS: usize = 256;
pub const FLOW_STORE_MAX_TEXT_BYTES: usize = 16_384;
pub const FLOW_STORE_MAX_MUTATION_ITEMS: usize = 256;
pub const FLOW_GRAPH_OPERATION_RAW_BYTES: usize = 16_384;
//#endregion 📏️StoreCapacity
