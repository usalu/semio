import pathlib

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")
text = path.read_text()


def replace(old, new, count=1):
    global text
    assert text.count(old) == count, (text.count(old), old[:100])
    text = text.replace(old, new)


replace("""/// 🖼️ Guest documents rendered off the shell for one owed refresh, keyed by surface id, each with the
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
}""", """/// 🧾️ The reserved sections one owed refresh reads besides its guest bodies: the app catalogue (only on
/// the instance's one claimed fetch), the window engagements, the window measures and the tool measures.
#[derive(Clone, Copy, Debug, Default)]
struct ShellRefreshSections {
    catalogue: bool,
    engagements: bool,
    measures: bool,
    tools: bool,
}

/// 🖼️ What one owed refresh read off the shell for one app instance: the guest bodies it names, keyed by
/// surface id, each with the host effects its render returned, and the reserved sections it read. A read
/// the refresh no longer wants is handed back by its document's own `Drop`.
struct ShellRenderedRefresh {
    instance_id: u32,
    surfaces: HashMap<String, (Result<UiDocumentLease, String>, Vec<semio_framework::kernel::Effect>)>,
    catalogue: Option<Result<UiDocumentLease, String>>,
    engagements: Option<Result<UiDocumentLease, String>>,
    measures: Option<Result<UiDocumentLease, String>>,
    tools: Option<Result<UiDocumentLease, String>>,
}

impl ShellRenderedRefresh {
    /// 🫙️ Nothing read ahead: every wanted body and section is read by the refresh itself.
    fn empty(instance_id: u32) -> Self {
        Self { instance_id, surfaces: HashMap::new(), catalogue: None, engagements: None, measures: None, tools: None }
    }
}

/// 🖼️ Reads everything one owed refresh names, in order, off the shell: every body render and every
/// section read is a guest turn, so the settle lane hands them off ([`ShellState::advance_owed_refresh`])
/// and the frame loop keeps building frames while they run; the refresh that consumes them then only
/// applies documents ([`ShellState::refresh_ui_rendered`]).
async fn render_refresh_detached(program: ProgramBridgeEntry, instance_id: u32, jobs: Vec<(String, String, ViewModel)>, sections: ShellRefreshSections, panel_view: ViewModel, view_state: ViewModel) -> ShellRenderedRefresh {
    let mut rendered = ShellRenderedRefresh::empty(instance_id);
    for (surface_id, body_key, view) in jobs {
        let mut effects = Vec::new();
        let document = program.render_with_document(instance_id, &surface_id, &body_key, &view, None, Some(&mut effects)).await;
        rendered.surfaces.insert(surface_id, (document, effects));
    }
    if sections.catalogue {
        let body_key = semio_framework::UiRefreshSection::Catalogue.body_key();
        rendered.catalogue = Some(program.render_with_document(instance_id, body_key, body_key, &panel_view, None, None).await);
    }
    if sections.engagements {
        rendered.engagements = Some(program.window_engagements_section(instance_id, &view_state).await);
    }
    if sections.measures {
        rendered.measures = Some(program.window_measures_section(instance_id, &view_state).await);
    }
    if sections.tools {
        rendered.tools = Some(program.tool_measures_section(instance_id, &view_state).await);
    }
    rendered
}""")
replace("""    rendering: Option<ShellDetached<ShellRenderedSurfaces>>,""", """    rendering: Option<ShellDetached<ShellRenderedRefresh>>,""")

# section functions take the read-ahead
replace("""    async fn refresh_window_engagements(&mut self, program: &ProgramBridgeEntry, instance_id: u32, view_state: &ViewModel, faults: &mut Vec<(String, String, String)>) -> Result<(), String> {
        let body_key = semio_framework::UiRefreshSection::Engagements.body_key();
        match program.window_engagements_section(instance_id, view_state).await {""", """    async fn refresh_window_engagements(&mut self, program: &ProgramBridgeEntry, instance_id: u32, view_state: &ViewModel, read_ahead: Option<Result<UiDocumentLease, String>>, faults: &mut Vec<(String, String, String)>) -> Result<(), String> {
        let body_key = semio_framework::UiRefreshSection::Engagements.body_key();
        let section = match read_ahead {
            Some(section) => section,
            None => program.window_engagements_section(instance_id, view_state).await,
        };
        match section {""")
replace("""    async fn refresh_window_measures(&mut self, program: &ProgramBridgeEntry, instance_id: u32, view_state: &ViewModel, windows: &[String], faults: &mut Vec<(String, String, String)>) -> Result<(), String> {
        let body_key = semio_framework::UiRefreshSection::Measures.body_key();
        let measures = match program.window_measures_section(instance_id, view_state).await {""", """    async fn refresh_window_measures(&mut self, program: &ProgramBridgeEntry, instance_id: u32, view_state: &ViewModel, read_ahead: Option<Result<UiDocumentLease, String>>, windows: &[String], faults: &mut Vec<(String, String, String)>) -> Result<(), String> {
        let body_key = semio_framework::UiRefreshSection::Measures.body_key();
        let section = match read_ahead {
            Some(section) => section,
            None => program.window_measures_section(instance_id, view_state).await,
        };
        let measures = match section {""")
replace("""    async fn refresh_tool_measures(&mut self, program: &ProgramBridgeEntry, instance_id: u32, view_state: &ViewModel, faults: &mut Vec<(String, String, String)>) -> Result<(), String> {
        let body_key = semio_framework::UiRefreshSection::Tools.body_key();
        let measures = match program.tool_measures_section(instance_id, view_state).await {""", """    async fn refresh_tool_measures(&mut self, program: &ProgramBridgeEntry, instance_id: u32, view_state: &ViewModel, read_ahead: Option<Result<UiDocumentLease, String>>, faults: &mut Vec<(String, String, String)>) -> Result<(), String> {
        let body_key = semio_framework::UiRefreshSection::Tools.body_key();
        let section = match read_ahead {
            Some(section) => section,
            None => program.tool_measures_section(instance_id, view_state).await,
        };
        let measures = match section {""")
replace("""    async fn refresh_app_catalogue(&mut self, program: &ProgramBridgeEntry, instance_id: u32, view_state: &ViewModel) {
        if !claim_app_catalogue_fetch(&mut self.app_catalogue_instance, instance_id) {
            self.publish_app_catalogue();
            return;
        }
        let body_key = semio_framework::UiRefreshSection::Catalogue.body_key();
        let document = match program.render_with_document(instance_id, body_key, body_key, view_state, None, None).await {""", """    async fn refresh_app_catalogue(&mut self, program: &ProgramBridgeEntry, instance_id: u32, view_state: &ViewModel, read_ahead: Option<Result<UiDocumentLease, String>>) {
        let body_key = semio_framework::UiRefreshSection::Catalogue.body_key();
        let fetched = match read_ahead {
            Some(fetched) => fetched,
            None if claim_app_catalogue_fetch(&mut self.app_catalogue_instance, instance_id) => program.render_with_document(instance_id, body_key, body_key, view_state, None, None).await,
            None => {
                self.publish_app_catalogue();
                return;
            }
        };
        let document = match fetched {""")

# refresh_ui / refresh_ui_rendered
replace("""        self.refresh_ui_rendered(ask, ShellRenderedSurfaces::new()).await
    }

    /// 🖼️ [`Self::refresh_ui`] with the guest bodies some of whose renders already happened off the
    /// shell: a surface found in `rendered` takes that document, every other wanted surface renders
    /// here, and a rendered document no longer wanted is handed back by its own `Drop`.
    async fn refresh_ui_rendered(&mut self, ask: UiDirtyScope, mut rendered_surfaces: ShellRenderedSurfaces) -> Result<(), String> {
        let mut latency = crate::frame_latency::FrameLatencyTimer::start(crate::frame_latency::latest_frame_authority(), crate::frame_latency::FrameLatencyStage::ShellRefresh, 1);
        let scope = core::mem::replace(&mut self.owed_refresh_scope, UiDirtyScope::None).merged_with(ask);
        let Some(session) = self.session.clone() else {
            return Ok(());
        };""", """        self.refresh_ui_rendered(ask, None).await
    }

    /// 🖼️ [`Self::refresh_ui`] with the guest bodies and reserved sections some of whose reads already
    /// happened off the shell: a surface or section found in `rendered` takes that document, every other
    /// wanted one is read here, and a read no longer wanted — or read for an instance that is no longer
    /// the session's — is handed back by its own `Drop`.
    async fn refresh_ui_rendered(&mut self, ask: UiDirtyScope, rendered: Option<ShellRenderedRefresh>) -> Result<(), String> {
        let mut latency = crate::frame_latency::FrameLatencyTimer::start(crate::frame_latency::latest_frame_authority(), crate::frame_latency::FrameLatencyStage::ShellRefresh, 1);
        let scope = core::mem::replace(&mut self.owed_refresh_scope, UiDirtyScope::None).merged_with(ask);
        let Some(session) = self.session.clone() else {
            return Ok(());
        };
        let mut rendered = rendered.filter(|rendered| rendered.instance_id == session.instance_id).unwrap_or_else(|| ShellRenderedRefresh::empty(session.instance_id));""")
replace("""                let document = match rendered_surfaces.remove(&window_id) {""", """                let document = match rendered.surfaces.remove(&window_id) {""")
replace("""            let document = match rendered_surfaces.remove(&tab_id) {""", """            let document = match rendered.surfaces.remove(&tab_id) {""")
replace("""        self.refresh_app_catalogue(&program, session.instance_id, &panel_view).await;""", """        self.refresh_app_catalogue(&program, session.instance_id, &panel_view, rendered.catalogue.take()).await;""")
replace("""            self.refresh_window_engagements(&program, session.instance_id, &view_state, &mut faults).await?;""", """            self.refresh_window_engagements(&program, session.instance_id, &view_state, rendered.engagements.take(), &mut faults).await?;""")
replace("""            self.refresh_window_measures(&program, session.instance_id, &view_state, &measure_windows, &mut faults).await?;""", """            self.refresh_window_measures(&program, session.instance_id, &view_state, rendered.measures.take(), &measure_windows, &mut faults).await?;""")
replace("""            self.refresh_tool_measures(&program, session.instance_id, &view_state, &mut faults).await?;""", """            self.refresh_tool_measures(&program, session.instance_id, &view_state, rendered.tools.take(), &mut faults).await?;""")

# settle lane
replace("""            if let Err(error) = self.refresh_ui_rendered(UiDirtyScope::None, rendered).await {""", """            if let Err(error) = self.refresh_ui_rendered(UiDirtyScope::None, Some(rendered)).await {""")
replace("""        self.settle_pump.rendering = Some(ShellDetached::spawn(render_surfaces_detached(program, session.instance_id, jobs)));""", """        let sections = ShellRefreshSections {
            catalogue: claim_app_catalogue_fetch(&mut self.app_catalogue_instance, session.instance_id),
            engagements: scope.wants_section(UiDirtySection::Engagements),
            measures: scope.wants_section(UiDirtySection::Measures),
            tools: scope.wants_section(UiDirtySection::Tools) && !self.tool_panel_tabs().is_empty(),
        };
        self.settle_pump.rendering = Some(ShellDetached::spawn(render_refresh_detached(program, session.instance_id, jobs, sections, view_state.for_panel(), view_state)));""")
path.write_text(text)
print("ok")
