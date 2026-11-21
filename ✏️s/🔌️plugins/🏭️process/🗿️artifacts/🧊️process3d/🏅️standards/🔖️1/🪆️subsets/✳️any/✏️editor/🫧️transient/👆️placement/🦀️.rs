//! 🫧️ Ephemeral pointer position in the addressed Workpiece window.

use super::*;
use semio_plugin::retained_command::ArtifactCommandInputs;

#[derive(Clone, Debug, Default, PartialEq, semio_plugin::ToValue, semio_plugin::FromValue)]
pub struct PlacementHover {
    utility: Option<String>,
    position: Option<[f64; 3]>,
}
semio_framework_value::artifact_retire_struct!(PlacementHover { utility, position });
semio_plugin::transient_root! {
    state: PlacementHover, mutation: PlacementHoverMutation, diff: PlacementHoverDiff,
    owner: "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/👆️placement",
    kind: "set-placement-hover", display_name: "Set Process3d Placement Hover",
    payload_schema: "process.3d.placementhover", envelope: "process.3d.placementhover", extension: "placementhover",
    fields: { utility: Option<String>, position: Option<[f64; 3]> },
}
semio_plugin::window_transient_owners! {
    state: PlacementHover, mutation: PlacementHoverMutation,
    windows: { WorkpiecePlacementOwner => workpiece::PROCESS_3D_PLAY_WINDOW_MAIN },
}

impl PlacementHover {
    pub(super) fn position(&self, utility: &str) -> Option<[f64; 3]> {
        self.position.filter(|_| self.utility.as_deref() == Some(utility))
    }
}

type App = EditorApp<Process3dPlayApp>;
pub(super) struct PlacementWork(pub BoundedArtifactCommandWork<App>);
impl ArtifactCommandWork<App> for PlacementWork {
    fn tool_id(&self) -> &'static str { self.0.tool_id() }
    fn extent(&self, command: &Process3dCommand, snapshot: &Process3dSnapshot, interaction: &protocol::InteractionState, context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<App>>) -> Option<usize> {
        self.0.extent(command, snapshot, interaction, context)
    }
    fn work_demands(&self, input: &ArtifactCommandInputs<'_, App>, maximum: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        self.0.work_demands(input, maximum)
    }
    fn terminal_frame_release_bytes(&self) -> Option<usize> { Some(std::mem::size_of::<Self>()) }
    fn reports_progress(&self) -> bool { false }
    fn step(&mut self, input: &ArtifactCommandInputs<'_, App>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<App>, semio_framework_job::OriginalTurnFailure> {
        let outcome = self.0.step(input, cx)?;
        let ArtifactCommandWorkStep::Complete(emit) = outcome else { return Ok(outcome); };
        let Some(context) = input.context else { return Ok(ArtifactCommandWorkStep::Complete(emit)); };
        let Some(view) = context.view_state.as_deref().filter(|view| view.window_instances.iter().any(|window| Some(window.id.as_str()) == view.window_id.as_deref() && window.window_kind_id == workpiece::PROCESS_3D_PLAY_WINDOW_MAIN)) else {
            return Ok(ArtifactCommandWorkStep::Complete(emit));
        };
        let utility = view.active_utility_id.as_deref().unwrap_or(PROCESS3D_DEFAULT_UTILITY);
        let previous = from_snapshot(context.window_transient.as_ref());
        let next = match input.command {
            Process3dCommand::WorldPointerMove(payload) if workpiece::placement::placement_kind(utility).is_some() => PlacementHover { utility: Some(utility.into()), position: Some(payload.position) },
            _ => PlacementHover::default(),
        };
        if next == previous { return Ok(ArtifactCommandWorkStep::Complete(emit)); }
        let mutation = addressed(view, next).map_err(semio_framework_job::OriginalTurnFailure::from)?;
        Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: semio_plugin::EphemeralEmit { window_transient: vec![mutation], ..Default::default() } })
    }
}
