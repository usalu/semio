import pathlib

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")
text = path.read_text()


def replace(old, new, count=1):
    global text
    assert text.count(old) == count, (text.count(old), old[:100])
    text = text.replace(old, new)


# settle pump field
replace(
    """pub struct ShellSettlePump {
    owed: bool,
    steps: u64,
    crossings: u64,
    traced: Option<ShellSettleStep>,
    watches: HashMap<String, ShellSettleWatch>,
}
""",
    """pub struct ShellSettlePump {
    owed: bool,
    steps: u64,
    crossings: u64,
    traced: Option<ShellSettleStep>,
    watches: HashMap<String, ShellSettleWatch>,
    rendering: Option<ShellDetached<ShellRenderedSurfaces>>,
}
""",
)

# detached render types + helper beside seed_document_genesis
replace(
    """//#endregion 🧵️ShellDetached
""",
    """/// 🖼️ Guest documents rendered off the shell for one owed refresh, keyed by surface id, each with the
/// host effects its render returned.
type ShellRenderedSurfaces = HashMap<String, (Result<UiDocumentLease, String>, Vec<semio_framework::kernel::Effect>)>;

/// 🖼️ Renders the guest bodies one owed refresh names, in order, off the shell: every render is the
/// guest's own turn, so the settle lane hands them off ([`ShellState::advance_owed_refresh`]) and the
/// frame loop keeps building frames while they run.
async fn render_surfaces_detached(program: ProgramBridgeEntry, instance_id: u32, jobs: Vec<(String, String, ViewModel)>) -> ShellRenderedSurfaces {
    let mut rendered = HashMap::with_capacity(jobs.len());
    for (surface_id, body_key, view) in jobs {
        let mut effects = Vec::new();
        let document = program.render_with_document(instance_id, &surface_id, &body_key, &view, None, Some(&mut effects)).await;
        rendered.insert(surface_id, (document, effects));
    }
    rendered
}
//#endregion 🧵️ShellDetached
""",
)

# refresh_ui delegates to refresh_ui_rendered
replace(
    """    pub async fn refresh_ui(&mut self, ask: UiDirtyScope) -> Result<(), String> {
        let mut latency = crate::frame_latency::FrameLatencyTimer::start(crate::frame_latency::latest_frame_authority(), crate::frame_latency::FrameLatencyStage::ShellRefresh, 1);""",
    """    pub async fn refresh_ui(&mut self, ask: UiDirtyScope) -> Result<(), String> {
        self.refresh_ui_rendered(ask, ShellRenderedSurfaces::new()).await
    }

    /// 🖼️ [`Self::refresh_ui`] with the guest bodies some of whose renders already happened off the
    /// shell: a surface found in `rendered` takes that document, every other wanted surface renders
    /// here, and a rendered document no longer wanted is handed back by its own `Drop`.
    async fn refresh_ui_rendered(&mut self, ask: UiDirtyScope, mut rendered_surfaces: ShellRenderedSurfaces) -> Result<(), String> {
        let mut latency = crate::frame_latency::FrameLatencyTimer::start(crate::frame_latency::latest_frame_authority(), crate::frame_latency::FrameLatencyStage::ShellRefresh, 1);""",
)
replace(
    """                match program.render_with_document(session.instance_id, &window_id, &kind.body_key, &window_view, None, Some(&mut refresh_effects)).await {
                    Ok(document) => {
                        self.window_ui.insert(window_id.clone(), document);
                    }""",
    """                let document = match rendered_surfaces.remove(&window_id) {
                    Some((document, mut effects)) => {
                        refresh_effects.append(&mut effects);
                        document
                    }
                    None => program.render_with_document(session.instance_id, &window_id, &kind.body_key, &window_view, None, Some(&mut refresh_effects)).await,
                };
                match document {
                    Ok(document) => {
                        self.window_ui.insert(window_id.clone(), document);
                    }""",
)
replace(
    """            match program.render_with_document(session.instance_id, &tab_id, &body_key, &panel_view, None, Some(&mut refresh_effects)).await {
                Ok(document) => {
                    let reveal = tab_id == FRAMEWORK_PANEL_TAB_TOOL_RUN_ID && tool_run_panel_reveals(&mut self.tool_run_panel_runs, &document)?;""",
    """            let document = match rendered_surfaces.remove(&tab_id) {
                Some((document, mut effects)) => {
                    refresh_effects.append(&mut effects);
                    document
                }
                None => program.render_with_document(session.instance_id, &tab_id, &body_key, &panel_view, None, Some(&mut refresh_effects)).await,
            };
            match document {
                Ok(document) => {
                    let reveal = tab_id == FRAMEWORK_PANEL_TAB_TOOL_RUN_ID && tool_run_panel_reveals(&mut self.tool_run_panel_runs, &document)?;""",
)

# settle lane uses the detached owed refresh
replace(
    """        if worked > 0 || !self.owed_refresh_scope.asks_for_nothing() {
            if !self.owed_refresh_scope.asks_for_nothing() {
                if let Err(error) = self.refresh_ui(UiDirtyScope::None).await {
                    Self::debug_log(&format!("[DEBUG] wgpu-shell settle pump refresh failed: {error}"));
                }
            }
            self.settle_pump.owed = true;
            return ShellSettleStep::Drained;
        }""",
    """        if worked > 0 || !self.owed_refresh_scope.asks_for_nothing() || self.settle_pump.rendering.is_some() {
            if !self.owed_refresh_scope.asks_for_nothing() || self.settle_pump.rendering.is_some() {
                self.advance_owed_refresh().await;
            }
            self.settle_pump.owed = true;
            return ShellSettleStep::Drained;
        }""",
)

# the advance fn, placed before settle_pump_cross
replace(
    """    /// 🫀️ The crossing half of a step: fund every producer that is still advancing, and drive a
    /// producer whose own witness has frozen to a terminal state.""",
    """    /// 🖼️ The settle lane's owed refresh without holding the shell across a guest turn: the first step
    /// hands the guest bodies the owed scope names to one detached render ([`render_surfaces_detached`])
    /// and returns; a later step, once they answered, runs the refresh with those documents
    /// ([`Self::refresh_ui_rendered`]). Before, the whole refresh ran inside one step: every body's
    /// render turn held the frame build — 3.5 s per full refresh of a block2d session in a debug build
    /// (ticket 26/09/23 slice WG8).
    async fn advance_owed_refresh(&mut self) {
        if let Some(rendering) = self.settle_pump.rendering.as_ref() {
            let Some(rendered) = rendering.take() else { return };
            self.settle_pump.rendering = None;
            if let Err(error) = self.refresh_ui_rendered(UiDirtyScope::None, rendered).await {
                Self::debug_log(&format!("[DEBUG] wgpu-shell settle pump refresh failed: {error}"));
            }
            return;
        }
        let Some(session) = self.session.clone() else {
            if let Err(error) = self.refresh_ui(UiDirtyScope::None).await {
                Self::debug_log(&format!("[DEBUG] wgpu-shell settle pump refresh failed: {error}"));
            }
            return;
        };
        let Some(program) = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned() else { return };
        self.drain_retained_document_arenas();
        self.sync_dock();
        let scope = self.owed_refresh_scope.clone();
        let view_state = self.live_view_state(&session);
        let mut jobs = Vec::new();
        for (window_id, window_kind_id) in self.dock.window_instances() {
            let Some(kind) = session.app.window_kinds.iter().find(|kind| kind.id == window_kind_id) else { continue };
            if !scope.wants_window_body(&kind.body_key) {
                continue;
            }
            if let Some(window_view) = view_state.for_window_instance(&window_id) {
                jobs.push((window_id, kind.body_key.clone(), window_view));
            }
        }
        let panel_view = view_state.for_panel();
        for tab in Self::flatten_panel_tab_leaves(&session.app.panel_tabs) {
            if let Some(body_key) = tab.body_key.as_deref().filter(|body_key| scope.wants_panel_body(body_key)) {
                jobs.push((tab.id().to_string(), body_key.to_string(), panel_view.clone()));
            }
        }
        self.settle_pump.rendering = Some(ShellDetached::spawn(render_surfaces_detached(program, session.instance_id, jobs)));
    }

    /// 🫀️ The crossing half of a step: fund every producer that is still advancing, and drive a
    /// producer whose own witness has frozen to a terminal state.""",
)
path.write_text(text)
print("ok")
