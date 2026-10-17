//! ✍️ Edit command work of the VCS play app: the text edit phases, the remnant and the resumable edit command work that drives them. See [`super`].

use super::*;

//#region 🔖️VcsPlayApp
//#region 🧵️RetainedCommands
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VcsEditPhase {
    Decode,
    Reserve,
    Scalars,
    CurrentIndex,
    NextIndex,
    Additions,
    Removals,
    Complete,
}

#[derive(semio_framework_value::RetireOwned)]
struct VcsEditRemnant {
    pub next: Option<VcsSnapshot>,
    pub current_tags: BTreeSet<String>,
    pub next_tags: BTreeSet<String>,
    pub mutations: Vec<VcsDemoMutation>,
}

pub struct VcsEditCommandWork {
    remnant: Option<semio_framework_value::retirement::controlled::ControlledRetirement<VcsEditRemnant>>,
    pub tool_id: &'static str,
    phase: VcsEditPhase,
    pub cursor: usize,
    pub next: Option<VcsSnapshot>,
    pub current_tags: BTreeSet<String>,
    pub next_tags: BTreeSet<String>,
    pub mutations: Vec<VcsDemoMutation>,
    output_bytes: usize,
    steps: u64,
    replay_target: u64,
    complete: bool,
    closing: bool,
}

impl VcsEditCommandWork {
    pub fn new(tool_id: &'static str) -> Self {
        Self { remnant: None, tool_id, phase: VcsEditPhase::Decode, cursor: 0, next: None, current_tags: BTreeSet::new(), next_tags: BTreeSet::new(), mutations: Vec::new(), output_bytes: 0, steps: 0, replay_target: 0, complete: false, closing: false }
    }

    fn workspace_is_empty(&self) -> bool {
        self.next.is_none() && self.current_tags.is_empty() && self.next_tags.is_empty() && self.mutations.is_empty()
    }

    fn charge_output(&mut self, bytes: usize) -> Result<(), Fault> {
        self.output_bytes = self.output_bytes.checked_add(bytes).filter(|total| *total <= VCS_EDIT_MAXIMUM_OUTPUT_BYTES).ok_or_else(|| Fault::from("vcs-edit-output-capacity"))?;
        Ok(())
    }

    pub fn advance(&mut self, command: &VcsCommand, snapshot: &VcsSnapshot) -> Result<Option<Emit<VcsDemoMutation, VcsDemoConfigMutation, NoDraftMutation>>, Fault> {
        use crate::mutations::{add_tag_at, change_counter, change_notes, change_status, remove_tag, rename_vcs};
        match self.phase {
            VcsEditPhase::Decode => {
                let text = vcs_edit_text(command).ok_or_else(|| Fault::from("vcs-edit-command-mismatch"))?;
                if text.len() > VCS_BOUNDED_RAW_BYTES || snapshot.tags.len() > VCS_EDIT_MAXIMUM_TAGS {
                    return Err(Fault::from("vcs-edit-input-capacity"));
                }
                match semio_framework_pack_json::from_json_str::<VcsSnapshot>(text, semio_framework_pack_json::JsonMemberPolicy::Reject) {
                    Ok(next) if next.tags.len() <= VCS_EDIT_MAXIMUM_TAGS => {
                        self.next = Some(next);
                        self.phase = VcsEditPhase::Reserve;
                    }
                    Ok(_) => return Err(Fault::from("vcs-edit-tag-capacity")),
                    Err(_) => self.phase = VcsEditPhase::Complete,
                }
            }
            VcsEditPhase::Reserve => {
                let next_tags = self.next.as_ref().map_or(0, |next| next.tags.len());
                let capacity = snapshot.tags.len().checked_add(next_tags).and_then(|count| count.checked_add(4)).ok_or_else(|| Fault::from("vcs-edit-mutation-capacity"))?;
                self.mutations.try_reserve_exact(capacity).map_err(|_| Fault::from("vcs-edit-mutation-capacity"))?;
                self.phase = VcsEditPhase::Scalars;
            }
            VcsEditPhase::Scalars => {
                let next = self.next.as_ref().ok_or_else(|| Fault::from("vcs-edit-next-snapshot-absent"))?;
                let mut staged = Vec::with_capacity(4);
                let mut bytes = 0_usize;
                if next.title != snapshot.title {
                    bytes = bytes.checked_add(next.title.len()).ok_or_else(|| Fault::from("vcs-edit-output-capacity"))?;
                    staged.push(rename_vcs(next.title.clone()));
                }
                if next.counter != snapshot.counter {
                    staged.push(change_counter(next.counter));
                }
                if next.status != snapshot.status {
                    bytes = bytes.checked_add(next.status.len()).ok_or_else(|| Fault::from("vcs-edit-output-capacity"))?;
                    staged.push(change_status(next.status.clone()));
                }
                if next.notes != snapshot.notes {
                    bytes = bytes.checked_add(next.notes.len()).ok_or_else(|| Fault::from("vcs-edit-output-capacity"))?;
                    staged.push(change_notes(next.notes.clone()));
                }
                self.charge_output(bytes)?;
                self.mutations.extend(staged);
                self.cursor = 0;
                self.phase = VcsEditPhase::CurrentIndex;
            }
            VcsEditPhase::CurrentIndex => {
                if let Some(tag) = snapshot.tags.get(self.cursor) {
                    if tag.len() > VCS_BOUNDED_RAW_BYTES {
                        return Err(Fault::from("vcs-edit-current-tag-capacity"));
                    }
                    self.current_tags.insert(tag.clone());
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.phase = VcsEditPhase::NextIndex;
                }
            }
            VcsEditPhase::NextIndex => {
                let next = self.next.as_ref().ok_or_else(|| Fault::from("vcs-edit-next-snapshot-absent"))?;
                if let Some(tag) = next.tags.get(self.cursor) {
                    if tag.len() > VCS_BOUNDED_RAW_BYTES {
                        return Err(Fault::from("vcs-edit-next-tag-capacity"));
                    }
                    self.next_tags.insert(tag.clone());
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.phase = VcsEditPhase::Removals;
                }
            }
            VcsEditPhase::Removals => {
                if let Some(tag) = snapshot.tags.get(self.cursor) {
                    let mutation = (!self.next_tags.contains(tag.as_str())).then(|| remove_tag(tag.clone()));
                    if let Some(mutation) = mutation {
                        self.charge_output(tag.len())?;
                        self.mutations.push(mutation);
                    }
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.phase = VcsEditPhase::Additions;
                }
            }
            VcsEditPhase::Additions => {
                let next = self.next.as_ref().ok_or_else(|| Fault::from("vcs-edit-next-snapshot-absent"))?;
                if let Some(tag) = next.tags.get(self.cursor) {
                    let mutation = (!self.current_tags.contains(tag.as_str())).then(|| add_tag_at(tag.clone(), self.cursor as u32));
                    if let Some(mutation) = mutation {
                        self.charge_output(tag.len())?;
                        self.mutations.push(mutation);
                    }
                    self.cursor += 1;
                } else {
                    self.phase = VcsEditPhase::Complete;
                }
            }
            VcsEditPhase::Complete => {
                if self.complete {
                    return Err(Fault::from("vcs-edit-work-repeated"));
                }
                self.complete = true;
                let mutations = std::mem::take(&mut self.mutations);
                return Ok(Some(Emit::mutations(mutations)));
            }
        }
        Ok(None)
    }
}

impl ArtifactCommandWork<EditorApp<VcsPlayApp>> for VcsEditCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &VcsCommand, snapshot: &VcsSnapshot, interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<VcsPlayApp>>>) -> Option<usize> {
        vcs_edit_extent(command, snapshot, interaction)
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<VcsPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<VcsPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: _, command, snapshot, config: _config, history: _history, interaction: _interaction, hover: _hover, context: _context, operation: _operation } = *input;
        let replaying = self.steps < self.replay_target;
        match self.advance(command, snapshot)? {
            Some(_emit) if replaying => Err(Fault::from("vcs-edit-checkpoint-beyond-completion")),
            Some(emit) => Ok(ArtifactCommandWorkStep::Complete(emit)),
            None => {
                self.steps = self.steps.checked_add(1).ok_or_else(|| Fault::from("vcs-edit-step-overflow"))?;
                if self.steps > VCS_EDIT_MAXIMUM_WORK_ITEMS as u64 {
                    return Err(Fault::from("vcs-edit-work-capacity"));
                }
                if replaying {
                    Ok(ArtifactCommandWorkStep::Replay { stage: "vcs-edit-replay", preview: b"{\"en\":\"Restoring text edit\",\"de\":\"Textbearbeitung wird wiederhergestellt\"}" })
                } else {
                    Ok(ArtifactCommandWorkStep::Progress { stage: "vcs-edit-diff", preview: b"{\"en\":\"Comparing text edit\",\"de\":\"Textbearbeitung wird verglichen\"}" })
                }
            }
        }
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn checkpoint_byte(&self, index: usize) -> Option<u8> {
        let mut checkpoint = [0_u8; 16];
        checkpoint[..4].copy_from_slice(b"VEC1");
        checkpoint[8..16].copy_from_slice(&self.steps.max(self.replay_target).to_le_bytes());
        checkpoint.get(index).copied()
    }

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 16 || &checkpoint[..4] != b"VEC1" || checkpoint[4..8] != [0; 4] {
            return Err(Fault::from("vcs-edit-checkpoint-invalid"));
        }
        if self.next.is_some() || !self.current_tags.is_empty() || !self.next_tags.is_empty() || !self.mutations.is_empty() {
            return Err(Fault::from("vcs-edit-checkpoint-workspace-not-empty"));
        }
        let target = u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| Fault::from("vcs-edit-checkpoint-cursor"))?);
        if target > VCS_EDIT_MAXIMUM_WORK_ITEMS as u64 {
            return Err(Fault::from("vcs-edit-checkpoint-cursor"));
        }
        self.phase = VcsEditPhase::Decode;
        self.cursor = 0;
        self.output_bytes = 0;
        self.steps = 0;
        self.replay_target = target;
        self.complete = false;
        Ok(())
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::InteractiveJobCloseStep;
        use semio_framework_value::retained_clone::RetainedCloneStep;
        if !self.closing {
            return InteractiveJobCloseStep::Blocked;
        }
        if let Some(owner) = self.remnant.as_mut() {
            let step = owner.step(grant);
            if owner.terminal_is_empty() {
                self.remnant = None;
            }
            return match step {
                Ok(RetainedCloneStep::Complete(progress)) => InteractiveJobCloseStep::Complete { progress },
                Ok(RetainedCloneStep::Progress(progress)) => InteractiveJobCloseStep::Pending { progress },
                Err(error) => InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() },
            };
        }
        if self.workspace_is_empty() {
            return InteractiveJobCloseStep::Complete { progress: Default::default() };
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return InteractiveJobCloseStep::Pending { progress: Default::default() };
        }
        let remnant = VcsEditRemnant { next: self.next.take(), current_tags: std::mem::take(&mut self.current_tags), next_tags: std::mem::take(&mut self.next_tags), mutations: std::mem::take(&mut self.mutations) };
        match semio_framework_value::retirement::controlled::ControlledRetirement::new(remnant) {
            Ok(owner) => {
                self.remnant = Some(owner);
                InteractiveJobCloseStep::Pending { progress: semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() } }
            }
            Err((error, remnant)) => {
                self.next = remnant.next;
                self.current_tags = remnant.current_tags;
                self.next_tags = remnant.next_tags;
                self.mutations = remnant.mutations;
                InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() }
            }
        }
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        self.remnant.as_ref().map_or(Ok(0), |owner| owner.next_copy_byte_demand())
    }
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        self.remnant.as_ref().map_or(Ok(0), |owner| owner.next_capacity_byte_demand(maximum_copy_bytes))
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        self.remnant.as_ref().map_or(Ok(0), |owner| owner.next_release_byte_demand())
    }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        self.remnant.as_ref().map_or(Ok(usize::from(!self.workspace_is_empty())), |owner| owner.next_depth_demand())
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        self.terminal_is_empty().then_some(size_of::<Self>())
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.remnant.is_none() && self.workspace_is_empty()
    }
}
//#endregion 🧵️RetainedCommands
//#endregion 🔖️VcsPlayApp
