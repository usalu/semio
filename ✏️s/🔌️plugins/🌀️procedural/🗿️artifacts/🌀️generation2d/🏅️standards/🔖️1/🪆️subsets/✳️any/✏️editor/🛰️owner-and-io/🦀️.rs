//! 🛰️ Instance operation owner and media I/O of the generation2d editor: the flow session close, the command categorisation, the owner with its retirement and maintenance steps and the typed media ports. See [`super`].

use super::*;

pub fn close_flow_session(session: &mut FlowEvalSession) {
    session.begin_close();
    while !session.terminal_is_empty() {
        let copy = session.next_close_copy_byte_demand().expect("cold session copy demand");
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant {
            maximum_items: 1,
            maximum_copy_bytes: copy,
            maximum_capacity_bytes: session.next_close_capacity_byte_demand(copy).expect("cold session capacity demand"),
            maximum_release_bytes: session.next_close_release_byte_demand().expect("cold session release demand"),
            maximum_depth: session.next_close_depth_demand().expect("cold session depth demand").max(1),
        };
        if let Err(error) = session.close_step(grant) {
            panic!("cold session close refused: {:?}", error.kind);
        }
    }
}

pub(super) fn categorized_action(id: &str, label: LocalizedLabel, kind: ActionKind, category: &str) -> ActionDefinition {
    ActionDefinition::bounded_catalog(id, label, kind).with_category(category)
}

/// 🧵️ Classifies the internal flow continuation as backed by its bounded first-step factory.
pub(super) fn migrated_command(mut definition: CommandDefinition) -> CommandDefinition {
    definition.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
    definition
}

/// 🧠️ The retained evaluation state of one app instance: the document session the flow and edit
/// preview windows render, the generation session the generate preview evaluates the selected
/// generation into, and the `previewEval` run's surface-owned link. Two sessions, because a generation
/// is a patched fixture of its own and one session holds one evaluation. Mirrors
/// `Generation3dInstanceOperationOwner` and the framework's reference `FlowInstanceOperationOwner`.
pub struct Generation2dInstanceOperationOwner {
    pub(super) eval_session: Option<FlowEvalSession>,
    pub(super) generation_session: Option<FlowEvalSession>,
    run_link: PreviewEvalRunLink,
    pub(super) closing: bool,
}

impl Generation2dInstanceOperationOwner {
    pub(super) fn new() -> Self {
        Self { eval_session: Some(FlowEvalSession::new()), generation_session: Some(FlowEvalSession::new()), run_link: PreviewEvalRunLink::default(), closing: false }
    }

    pub(super) fn with_session<R>(&mut self, body: impl FnOnce(&mut FlowEvalSession) -> R) -> Result<R, Fault> {
        if self.closing {
            return Err(Fault::from("generation2d-eval-session-closing"));
        }
        self.eval_session.as_mut().map(body).ok_or_else(|| Fault::from("generation2d-eval-session-owner-missing"))
    }

    /// 🧳️ Both sessions and the run link, while the instance is live.
    pub(super) fn parts(&mut self) -> Result<(PreviewEvalSessions<'_>, &mut PreviewEvalRunLink), Fault> {
        let Self { eval_session, generation_session, run_link, closing } = self;
        match (eval_session.as_mut(), generation_session.as_mut()) {
            (Some(document), Some(generation)) if !*closing => Ok((PreviewEvalSessions { document, generation }, run_link)),
            _ => Err(Fault::from("generation2d-eval-session-closing")),
        }
    }

    /// 🩹️ Owes the attached previews exactly what this gesture's own emit says it owes them, and puts
    /// the run start that debt needs on that same emit when no run is live to be woken instead.
    pub(super) fn owe_attached_previews_for_mutations(&mut self, windows: &[PreviewEvalWindow<'_>], servable: bool, emit: &mut Emit<Generation2dMutation, Generation2dConfigMutation, NoDraftMutation>, grant: semio_framework_value::RetainedCloneGrant) -> Result<bool, Fault> {
        let (sessions, link) = self.parts()?;
        semio_s_artifact_procedural_2d_preview_eval::owe_attached_previews_for_mutations(sessions, link, windows, servable, emit, grant).map_err(|error| Fault::from(error.to_string()))
    }

    /// 🚦️ Owes the attached previews an evaluation unconditionally and carries the run start the debt needs.
    pub(super) fn owe_attached_previews_carrying(&mut self, windows: &[PreviewEvalWindow<'_>], servable: bool, emit: &mut Emit<Generation2dMutation, Generation2dConfigMutation, NoDraftMutation>, grant: semio_framework_value::RetainedCloneGrant) -> Result<(), Fault> {
        let (sessions, link) = self.parts()?;
        semio_s_artifact_procedural_2d_preview_eval::owe_attached_previews_carrying(sessions, link, windows, servable, emit, grant).map_err(|error| Fault::from(error.to_string()))
    }
}

impl PreviewEvalRunOwner for Generation2dInstanceOperationOwner {
    fn preview_eval_parts(&mut self) -> Option<(PreviewEvalSessions<'_>, &mut PreviewEvalRunLink)> {
        self.parts().ok()
    }
}

impl semio_framework_plugin::ArtifactInstanceOperationOwner for Generation2dInstanceOperationOwner {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if !self.closing {
            return Ok(Default::default());
        }
        let Some(session) = [self.eval_session.as_ref(), self.generation_session.as_ref()].into_iter().flatten().next() else { return Ok(Default::default()) };
        if session.terminal_is_empty() {
            return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() });
        }
        Ok(semio_framework_value::RetirementDemand { copy_bytes: session.next_close_copy_byte_demand()?, capacity_bytes: session.next_close_capacity_byte_demand(body)?, release_bytes: session.next_close_release_byte_demand()?, depth: session.next_close_depth_demand()? })
    }

    /// 🧹️ A LIVE session owns nothing retirable — `FlowEvalSession::close_step` answers `Blocked`
    /// until `begin_close`, and reporting that from the live maintenance ladder spent the runtime's
    /// zero-progress credit every idle turn (26/09/09/PROCEDURAL-3D-END-TO-END, close-ladder lane).
    fn maintenance_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_plugin::PluginLifecycleStep, semio_framework_plugin::PluginLifecycleFailure> {
        use semio_framework_plugin::PluginLifecycleStep;
        use semio_framework_value::retained_clone::RetainedCloneProgress;
        if !self.closing {
            return Ok(PluginLifecycleStep::Complete(Default::default()));
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(PluginLifecycleStep::Progress(Default::default()));
        }
        let Some(slot) = [&mut self.eval_session, &mut self.generation_session].into_iter().find(|slot| slot.is_some()) else { return Ok(PluginLifecycleStep::Complete(Default::default())) };
        let Some(session) = slot.as_mut() else { return Ok(PluginLifecycleStep::Complete(Default::default())) };
        let (progress, finished) = if session.terminal_is_empty() {
            (RetainedCloneProgress { copied_items: 1, ..Default::default() }, true)
        } else {
            match session.close_step(grant)? {
                semio_framework_job::InteractiveJobCloseStep::Pending { progress } => (progress, false),
                semio_framework_job::InteractiveJobCloseStep::Complete { progress } => (progress, true),
                semio_framework_job::InteractiveJobCloseStep::Blocked => return Ok(PluginLifecycleStep::Blocked { reason: "Generation2d evaluation session awaits its exact close grant" }),
            }
        };
        if finished && session.terminal_is_empty() {
            *slot = None;
        }
        Ok(if self.terminal_is_empty() { PluginLifecycleStep::Complete(progress) } else { PluginLifecycleStep::Progress(progress) })
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_plugin::PluginLifecycleStep, semio_framework_plugin::PluginLifecycleFailure> {
        self.closing = true;
        self.run_link.wake();
        for session in [self.eval_session.as_mut(), self.generation_session.as_mut()].into_iter().flatten() {
            session.begin_close();
        }
        self.maintenance_step(cx,grant)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.eval_session.is_none() && self.generation_session.is_none()
    }
}

//#region 🔖️ArtifactIo
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors `create_generation2d_app`'s
/// `.artifact_kind(...)` document schema/media type verbatim, plus two workflow ports: `params:in`
/// (generic Data×Value parametric input) and `drawing:out` (TwoD×Vector, tagged with draw's already-
/// registered `2d.drawing` kind id).
pub async fn generation2d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo::from_artifact(
        "generation.2d",
        MediaType { class: MediaClass::TwoD, form: MediaForm::Flow },
        semio_framework_plugin::ArtifactPresentation { id: "2d.generation".into(), name: "2D Generation".into(), dimension: "2d".into(), component_kind: "generation2d".into() },
    )
    .await
    .with_ports(vec![
        semio_framework_plugin::MediaPortSpec {
            id: "params:in".into(),
            label: "Parameters".into(),
            direction: semio_framework_plugin::MediaPortDirection::In,
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            kind_id: None,
            required: false,
            multiplicity: semio_framework::PortMultiplicity::One,
        },
        semio_framework_plugin::MediaPortSpec {
            id: "drawing:out".into(),
            label: "Drawing".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
            kind_id: Some("2d.drawing".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        },
    ])
    .await
}
//#endregion 🔖️ArtifactIo
