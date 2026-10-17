//! 🖱️ Graph interaction and window bodies of the generation2d editor: the resolved interaction marks and the single body-key render. See [`super`].

use super::*;

//#region 🔖️GraphInteraction
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region.
/// 🕹️ One render's resolved `graph`-domain marks — hover off the ephemeral pointer channel,
/// selection off the persisted interaction store.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GraphInteractionMarks {
    pub hovered: std::collections::BTreeSet<String>,
    pub selected: std::collections::BTreeSet<String>,
}

impl GraphInteractionMarks {
    /// 🕹️ Reads the framework-owned domain.
    pub fn from_interaction(interaction: &InteractionView<'_>) -> Self {
        Self {
            hovered: interaction.hover(GENERATION2D_INTERACTION_DOMAIN, GENERATION2D_INTERACTION_CHANNEL).ids.iter().cloned().collect(),
            selected: interaction.selection(GENERATION2D_INTERACTION_DOMAIN).ids.iter().cloned().collect(),
        }
    }

    /// 🕸️ The widget id behind any interaction id — `{w}`, `{w}@{c}` and `{w}@{c}#{i}` all resolve
    /// to `{w}`.
    pub fn widget_of(id: &str) -> &str {
        let base = id.split('#').next().unwrap_or(id);
        base.split('@').next().unwrap_or(base)
    }

    /// 🕸️ The `graph` selection split into the node and edge domains a context menu addresses.
    pub fn graph_selection_domains(&self, host_snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot) -> (Vec<String>, Vec<String>) {
        let synapses: std::collections::BTreeSet<&str> = host_snapshot.synapses.iter().map(|synapse| synapse.id.as_str()).collect();
        let mut nodes = std::collections::BTreeSet::new();
        let mut edges = std::collections::BTreeSet::new();
        for id in &self.selected {
            if synapses.contains(id.as_str()) {
                edges.insert(id.clone());
            } else {
                nodes.insert(Self::widget_of(id).to_string());
            }
        }
        (nodes.into_iter().collect(), edges.into_iter().collect())
    }

    /// 🕸️ The `graph` selection projected onto widget ids — what `NodeGraphScene::selection` paints.
    pub fn graph_selection_ids(&self) -> Vec<String> {
        self.selected.iter().map(|id| Self::widget_of(id).to_string()).collect::<std::collections::BTreeSet<String>>().into_iter().collect()
    }
}
//#endregion 🔖️GraphInteraction

/// 🧱️ Every window body of the generation2d editor, rendered against ONE already-resolved evaluation
/// session — shared by the marks-free `render` (scratch session) and `render_with_request_context`
/// (the instance's retained session) so there is exactly one body-key match in the app.
pub(super) fn generation2d_render_body(
    body_key: &str,
    doc: &ArtifactView<'_, Generation2dSnapshot>,
    cfg: &ConfigView<'_, Generation2dConfig>,
    view_state: &semio_framework_plugin::ViewModel,
    marks: &GraphInteractionMarks,
    session: &FlowEvalSession,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let labels = generation2d_labels(view_state);
    let rendered = match body_key {
        flow_window::GENERATION2D_PLAY_BODY_MAIN => {
            let window_config = flow_window::config::current(cfg);
            flow_window::render(document, &window_config, session, &marks.graph_selection_ids())
        }
        edit_preview::GENERATION2D_PLAY_BODY_PREVIEW => {
            let window_config = edit_preview::config::current(cfg);
            edit_preview::render(document, config, &window_config, session)
        }
        generations::GENERATION2D_PLAY_BODY_GENERATIONS => generations::render(
            &document.generation,
            config.selected_generation_id.as_deref(),
            view_state.locale,
            view_state.terminology,
            &semio_framework_plugin::TreeWindows::for_body(view_state, generations::GENERATION2D_PLAY_BODY_GENERATIONS),
        ),
        form::GENERATION2D_PLAY_BODY_GENERATE_FORM => form::render(document, &document.generation, labels),
        generate_preview::GENERATION2D_PLAY_BODY_GENERATE_PREVIEW => {
            let window_config = generate_preview::config::current(cfg);
            generate_preview::render(&window_config, None, labels)
        }
        document_panel::GENERATION2D_PLAY_BODY_ARTIFACT => document_panel::render(document, config, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, document_panel::GENERATION2D_PLAY_BODY_ARTIFACT)),
        catalogue_panel::GENERATION2D_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels, &semio_framework_plugin::TreeWindows::for_body(view_state, catalogue_panel::GENERATION2D_PLAY_BODY_CATALOGUE)),
        inspection_panel::GENERATION2D_PLAY_BODY_INSPECTION => inspection_panel::render(document, config, labels),
        _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.unknown-body", "fixed UI unknown-body admission failed")),
    };
    Ok(semio_framework_plugin::built_to_component_tree(rendered?))
}
