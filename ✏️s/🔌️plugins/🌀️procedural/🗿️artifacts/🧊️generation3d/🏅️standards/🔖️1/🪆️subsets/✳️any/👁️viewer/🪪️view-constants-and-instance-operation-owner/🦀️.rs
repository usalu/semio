//! 🪪️ View constants and instance operation owner: the view app id and action, the viewed document, the instance operation owner and the view command. See [`super`].

use super::*;

//#region 🔖️Constants
pub const GENERATION3D_VIEW_APP_ID: &str = "procedural3d-view";

/// 🎯️ An `ActionDescriptor` addressed at this viewer — the single factory every chrome measure
/// builds its `on_change` with.
pub fn generation3d_view_action(action: &str, args: Option<semio_framework::DslValue>) -> ActionDescriptor {
    ActionDescriptor { controller_id: GENERATION3D_VIEW_APP_ID.into(), action: action.into(), args }
}

/// 🪟️ The window kinds this surface's `flowEvalTick` chain may be addressed to — this viewer has
/// exactly one, and it is the ONLY window holding a retained evaluation publication. Handed to the
/// shared [`preview_eval::attached_preview_windows`] / [`preview_eval::preview_kind`], which is
/// where the editor's two-preview roster is expressed the same way.
pub(super) const GENERATION3D_VIEW_PREVIEW_KINDS: &[&str] = &[preview::WINDOW_KIND_ID];

/// 🪟️ Every attached viewer preview window, off the shell's trusted `ViewModel` roster.
pub(super) fn generation3d_view_preview_windows(view: Option<&semio_framework_plugin::ViewModel>) -> Vec<(&str, &'static str)> {
    preview_eval::attached_preview_windows(view, GENERATION3D_VIEW_PREVIEW_KINDS)
}

/// 📚️ What this read-only surface is LOOKING at: the document the session opened, or — once the
/// navbar picker named one — the bundled example's own projection.
///
/// 🔒️ This is how a viewer "loads" a document without owning write authority. The editor's
/// `setActiveExample` replaces the artifact's fixture; this surface cannot, so the picked id lives
/// on its CONFIG (`Generation3dViewConfig::active_example_id`) and every read path — the preview
/// render, the `flowEvalTick` chain and the interaction topology — resolves it here, exactly once,
/// through the surface-neutral `🧬️schema::example_snapshot`.
///
/// 🕳️ Three states, not two — the picker's own. `None` on the config is "nothing picked", which is
/// the document this session OPENED; `Some("")` is the picker's `No example` row, which is the EMPTY
/// document; `Some(id)` is that bundled example. Collapsing the first two onto one empty string made
/// `No example` show the opened document, and the opened document in the playground IS a bundled
/// example — so the row that promises no example painted the hexagonal mushroom column's three
/// meshes (measured on the served viewer 2026-09-13, `meshesLen 3641`). The sibling surface clears
/// on the same row (`✏️editor/🎮️commands/🎨️set`); both surfaces now agree.
///
/// 🧹️ `Example` and `Empty` own a real `Generation3dSnapshot` whose neural `Dictionary` roots abort
/// the process on a bare drop, so every caller MUST finish with [`Self::retire`] (ticket
/// 26/09/09/PROCEDURAL-3D-END-TO-END).
pub(super) enum Generation3dViewedDocument<'a> {
    Opened(&'a Generation3dSnapshot),
    Example(Generation3dSnapshot),
    Empty(Generation3dSnapshot),
}

impl<'a> Generation3dViewedDocument<'a> {
    pub(super) fn resolve(snapshot: &'a Generation3dSnapshot, config: &Generation3dViewConfig) -> Self {
        let Some(example_id) = config.active_example_id.as_deref() else { return Self::Opened(snapshot) };
        if example_id.is_empty() {
            return Self::Empty(semio_s_artifact_procedural_3d_model::empty_generation3d_snapshot());
        }
        match semio_s_artifact_procedural_3d_io_text::snapshot::example_snapshot(example_id) {
            Some(example) => Self::Example(example),
            None => Self::Opened(snapshot),
        }
    }

    pub(super) fn snapshot(&self) -> &Generation3dSnapshot {
        match self {
            Self::Opened(snapshot) => snapshot,
            Self::Example(example) | Self::Empty(example) => example,
        }
    }

    pub(super) fn retire(self) {
        match self {
            Self::Opened(_) => {}
            Self::Example(example) | Self::Empty(example) => example.retire_cold(),
        }
    }
}
//#endregion 🔖️Constants

//#region 🔖️InstanceOperationOwner
/// 🧠️ The ONE `FlowEvalSession` this viewer instance retains across turns — its neural cache, its
/// `eval_json`, its live preview meshes and, critically, its `pending_tessellate_by_hash` table.
///
/// 🐛️ Before this owner the viewer built a throwaway `FlowEvalSession::new()` per command and
/// ticked it synchronously to completion (`👁️viewer/🦀️.rs:179-207` as of 2026-09-12): an in-flight
/// tessellate handle noted while emitting the request was gone before any answer could arrive, and
/// no answer could arrive at all because the sync loop emitted no `ExtensionInvocation`. The
/// brep/math operators are CONTRIBUTED at runtime and never linked into the guest, so that loop
/// could only ever fault — the read-only surface could never paint a brep-bearing example
/// (`📓️audit-window-inventory-2026-09-12.md` §4 P0 item 1). Mirrors the sibling surface's own
/// retained owner exactly.
pub(super) struct Generation3dViewInstanceOperationOwner {
    pub(super) eval_session: Option<FlowEvalSession>,
    /// ⏯️ The `previewEval` run's surface-owned half: attached preview roster and the live job's port.
    run_link: preview_eval::PreviewEvalRunLink,
    pub(super) closing: bool,
}

impl Generation3dViewInstanceOperationOwner {
    pub(super) fn new() -> Self {
        Self { eval_session: Some(FlowEvalSession::new()), run_link: preview_eval::PreviewEvalRunLink::default(), closing: false }
    }

    pub(super) fn with_session<R>(&mut self, body: impl FnOnce(&mut FlowEvalSession) -> R) -> Result<R, Fault> {
        if self.closing {
            return Err(Fault::from("generation3d-view-eval-session-closing"));
        }
        self.eval_session.as_mut().map(body).ok_or_else(|| Fault::from("generation3d-view-eval-session-owner-missing"))
    }

    /// ⏰️ A fold that changed the session wakes the live `previewEval` run job afterwards.
    pub(super) fn with_session_waking<R>(&mut self, body: impl FnOnce(&mut FlowEvalSession) -> R) -> Result<R, Fault> {
        let result = self.with_session(body);
        self.run_link.wake();
        result
    }

    /// 🧵️ Owes every attached preview window an evaluation and carries on `emit` the run start that debt
    /// needs when no unsettled run can be woken ([`preview_eval::owe_attached_previews_carrying`]).
    pub(super) fn owe_attached_previews_carrying(&mut self, windows: &[(&str, &'static str)], servable: bool, emit: &mut Emit<Generation3dMutation, Generation3dViewConfigMutation, NoDraftMutation>, grant: semio_framework_value::RetainedCloneGrant) -> Result<(), Fault> {
        use preview_eval::PreviewEvalRunOwner as _;
        let (session, link) = self.preview_eval_parts().ok_or_else(|| Fault::from("generation3d-view-eval-session-closing"))?;
        preview_eval::owe_attached_previews_carrying(session, link, windows, servable, emit, grant).map_err(|error| Fault::from(error.to_string()))
    }
}

impl preview_eval::PreviewEvalRunOwner for Generation3dViewInstanceOperationOwner {
    fn preview_eval_parts(&mut self) -> Option<(&mut FlowEvalSession, &mut preview_eval::PreviewEvalRunLink)> {
        let Self { eval_session, run_link, closing } = self;
        eval_session.as_mut().filter(|_| !*closing).map(|session| (session, run_link))
    }
}

impl semio_framework_plugin::ArtifactInstanceOperationOwner for Generation3dViewInstanceOperationOwner {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if !self.closing {
            return Ok(Default::default());
        }
        let Some(session) = self.eval_session.as_ref() else { return Ok(Default::default()) };
        if session.terminal_is_empty() {
            return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() });
        }
        Ok(semio_framework_value::RetirementDemand { copy_bytes: session.next_close_copy_byte_demand()?, capacity_bytes: session.next_close_capacity_byte_demand(body)?, release_bytes: session.next_close_release_byte_demand()?, depth: session.next_close_depth_demand()? })
    }

    /// 🧹️ A LIVE session owns nothing retirable — `FlowEvalSession::close_step` answers `Blocked`
    /// until `begin_close`, and reporting that from the live maintenance ladder spends the
    /// runtime's zero-progress credit every idle turn.
    fn maintenance_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_plugin::PluginLifecycleStep, semio_framework_plugin::PluginLifecycleFailure> {
        use semio_framework_plugin::PluginLifecycleStep;
        use semio_framework_value::retained_clone::RetainedCloneProgress;
        if !self.closing {
            return Ok(PluginLifecycleStep::Complete(Default::default()));
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(PluginLifecycleStep::Progress(Default::default()));
        }
        let Some(session) = self.eval_session.as_mut() else { return Ok(PluginLifecycleStep::Complete(Default::default())) };
        let (progress, finished) = if session.terminal_is_empty() {
            (RetainedCloneProgress { copied_items: 1, ..Default::default() }, true)
        } else {
            match session.close_step(grant)? {
                semio_framework_job::InteractiveJobCloseStep::Pending { progress } => (progress, false),
                semio_framework_job::InteractiveJobCloseStep::Complete { progress } => (progress, true),
                semio_framework_job::InteractiveJobCloseStep::Blocked => return Ok(PluginLifecycleStep::Blocked { reason: "Generation3d viewer evaluation session awaits its exact close grant" }),
            }
        };
        if finished && session.terminal_is_empty() {
            self.eval_session = None;
        }
        Ok(if self.terminal_is_empty() { PluginLifecycleStep::Complete(progress) } else { PluginLifecycleStep::Progress(progress) })
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_plugin::PluginLifecycleStep, semio_framework_plugin::PluginLifecycleFailure> {
        self.closing = true;
        if let Some(session) = self.eval_session.as_mut() {
            session.begin_close();
        }
        self.maintenance_step(cx,grant)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.eval_session.is_none()
    }
}
//#endregion 🔖️InstanceOperationOwner

//#region 🔖️Command
semio_framework_plugin::view_commands! {
    /// 👁️ `Generation3dViewer::Command` — the SOLE dispatch surface for the read-only surface's own
    /// behavior, covering EVERY declared action. Row order is the binary variant ordinal: appending
    /// is safe, reordering is a wire-format break. `dispatch` returns `ViewEmit`, so no row of this
    /// table can ever carry an artifact or draft mutation.
    pub enum Generation3dViewCommand for Generation3dSnapshot, Generation3dViewConfig, Generation3dViewConfigMutation {
        "setShowMode" as "show-mode" => set_show_mode::SetShowMode,
        "setLodMode" as "lod-mode" => set_lod_mode::SetLodMode,
        "setCamera" as "camera" => set_camera::SetCamera,
        "toggleSun" as "toggle-sun" => toggle_sun::ToggleSun,
        "setSunAzimuth" as "sun-azimuth" => set_sun_azimuth::SetSunAzimuth,
        "setSunElevation" as "sun-elevation" => set_sun_elevation::SetSunElevation,
        "setSunIntensity" as "sun-intensity" => set_sun_intensity::SetSunIntensity,
        "setContributions" as "set-contributions" => set_contributions::SetContributions,
        "flowEvalTick" as "flow-eval-tick" => semio_s_artifact_procedural_3d_preview_eval::FlowEvalTick,
        "flowEvalResolve" as "flow-eval-resolve" => semio_s_artifact_procedural_3d_preview_eval::FlowEvalResolve,
        "flowTessellateResolve" as "flow-tessellate-resolve" => semio_s_artifact_procedural_3d_preview_eval::FlowTessellateResolve,
        "flowEvalRelease" as "flow-eval-release" => semio_s_artifact_procedural_3d_preview_eval::FlowEvalRelease,
        "flowTessellateCancelResolve" as "flow-tessellate-cancel-resolve" => semio_s_artifact_procedural_3d_preview_eval::FlowTessellateCancelResolve,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "exportDocument" as "export-document" => export_document::ExportDocument}
}

impl Default for Generation3dViewCommand {
    fn default() -> Self {
        Generation3dViewCommand::SetShowMode(set_show_mode::SetShowMode { value: semio_s_artifact_procedural_3d_viewer_config::default_view_show_mode() })
    }
}
//#endregion 🔖️Command
