//! ➕️ A retained add-object command publishes a leaf to its original pane's Model document.

use super::*;
use semio_framework_job::{InteractiveJobCloseStep, OriginalTurnFailure, StepContext};
use semio_framework_value::{
    ErasedSnapshotRetirement, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep,
    RetainedWorkOutcome, RetainedWorkPoll, RetirementDemand, ValueError,
    retained_clone::{RetainedCloneSource, StagedSourceAdmission, retained_clone_grant_admits},
    retirement::controlled::ControlledRetirement,
};
use semio_plugin::app::{
    ChildEmitPreparation, MemberKeyRef,
    child_read::{
        ChildAdmissionSource, ChildReadCursor, ChildReadReference,
        ChildReadReferenceAdvance, ChildReadReferenceLenderV1,
        ChildReadSourceCustody, ChildReadStamp, OriginalChildMemberLookup,
        command::{OriginalChildEditorCommandAuthority, OriginalChildEditorCommandCursor},
        erase_child_admission_source,
    },
};
use semio_s_artifact_cad_cad_model::snapshot::{cad_pane_model, cad_pane_model_slot};
use semio_s_artifact_cad_cad_modes::windows::config::cad_pane_from_view;
use semio_s_artifact_stdio_semio_model::{
    editor::child_editor::{SEMIO_MODEL_CHILD_INSERT_ELEMENT, SEMIO_MODEL_CHILD_READ, admit_semio_model_insert_element},
    schema::{mutations::insert_element::InsertElement, snapshot::SemioModelSnapshot},
};
use store::os_store::snapshot_read::{SnapshotRead, SnapshotReadSourceStaging};

type App = EditorApp<CadPlayApp>;
type Output = Emit<CadMutation, CadConfigMutation>;
type Parent = RetainedCloneSource<CadSnapshot, SnapshotRead<CadSnapshot>>;
type Command = add_object::AddObject;
const PARENT_SOURCE: usize = 0x434144;
static REFERENCE: PaneReference = PaneReference;

fn invalid() -> ValueError {
    ValueError::invariant("CAD add-object requires the original command and its current pane Model source")
}

fn quote(copy_bytes: usize, capacity_bytes: usize) -> RetirementDemand {
    RetirementDemand { copy_bytes, capacity_bytes, depth: 1, ..Default::default() }
}

fn receipt(demand: RetirementDemand) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() }
}

struct PaneReference;
impl ChildReadReferenceLenderV1 for PaneReference {
    fn demand(&self, _: usize) -> RetirementDemand {
        quote(2 * size_of::<RetainedWorkOutcome<ChildReadReferenceAdvance<'static>>>(), 0)
    }

    fn read<'a>(&self, parent: ChildAdmissionSource<'a>, ordinal: usize, grant: RetainedCloneGrant) -> RetainedWorkOutcome<ChildReadReferenceAdvance<'a>> {
        let demand = self.demand(ordinal);
        if !retained_clone_grant_admits(grant, demand) {
            return RetainedWorkOutcome { result: Ok(ChildReadReferenceAdvance { progress: Default::default(), done: false, reference: None }), work: 0 };
        }
        let progress = receipt(demand);
        let result = (|| {
            let snapshot = parent.get().downcast_ref::<CadSnapshot>().ok_or_else(invalid)?;
            let pane = *CadPaneId::all().get(ordinal).ok_or_else(invalid)?;
            let handle = cad_pane_model(snapshot, pane).ok_or_else(invalid)?;
            let reference = ChildReadReference {
                parent_identity: snapshot as *const _ as usize,
                identity: handle as *const _ as usize,
                owner: "",
                slot: cad_pane_model_slot(pane),
                child_id: &handle.child_id,
                artifact_id: &handle.target.artifact_id,
                artifact_kind: &handle.target.dialect.artifact_kind,
                standard: &handle.target.dialect.standard,
                subset: &handle.target.dialect.subset,
            };
            Ok(ChildReadReferenceAdvance { progress, done: true, reference: Some(reference) })
        })().map_err(|error: ValueError| error.with_retained_progress(progress));
        RetainedWorkOutcome { result, work: 1 }
    }
}

#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
struct Owners {
    preparation: Option<ChildEmitPreparation>,
    mutation: Option<SemioModelMutation>,
    authority: Option<OriginalChildEditorCommandAuthority>,
    qualifier: Option<OriginalChildEditorCommandCursor>,
    child: ChildReadSourceCustody<SemioModelSnapshot>,
    lookup: Option<OriginalChildMemberLookup>,
    read_cursor: Option<ChildReadCursor>,
    parent: Option<Parent>,
    original: Option<SnapshotRead<CadSnapshot>>,
    sealing: SnapshotReadSourceStaging<CadSnapshot>,
    command: Option<RetainedCloneSource<Command>>,
    staged: Option<StagedSourceAdmission<Command, Command>>,
}

pub(in super::super) struct CadAddObjectWork {
    owners: ControlledRetirement<Owners>,
    qualified: bool,
    stamp: Option<ChildReadStamp>,
}

impl CadAddObjectWork {
    pub(in super::super) fn new() -> Self {
        Self {
            owners: ControlledRetirement::retain(Owners {
                preparation: None, mutation: None, authority: None, qualifier: None,
                child: ChildReadSourceCustody::new(), lookup: None, read_cursor: None,
                parent: None, original: None, sealing: Default::default(), command: None, staged: None,
            }),
            qualified: false,
            stamp: None,
        }
    }

    fn advance(&mut self, input: &ArtifactCommandInputs<'_, App>, cx: &mut StepContext<'_>) -> Result<Option<Output>, OriginalTurnFailure> {
        if cx.should_yield() || cx.is_cancelled() {
            return Ok(None);
        }
        let owners = self.owners.original_mut().ok_or_else(invalid)?;
        if owners.parent.is_none() || !owners.sealing.terminal_is_empty() {
            if owners.sealing.terminal_is_empty() {
                if let Some(original) = owners.original.take() {
                    let demand = quote(2 * size_of::<SnapshotRead<CadSnapshot>>(), 0);
                    if !cx.retained_work_can_enter(demand) {
                        owners.original = Some(original);
                        return Ok(None);
                    }
                    cx.consume_retained(receipt(demand))?;
                    cx.consume_fuel(1);
                    if let Err((error, original)) = owners.sealing.begin(original) {
                        owners.original = Some(original);
                        return Err(error.into());
                    }
                    return Ok(None);
                }
                owners.original = input.snapshot_read.take_current(input.operation.generation, input.operation.canonical_base_revision, cx)?;
                return Ok(None);
            }
            let demand = owners.sealing.next_demand()?;
            if !cx.retained_work_can_enter(demand) { return Ok(None); }
            if let Some((turn, work)) = cx.with_original_source_effect(cx.retained_grant(), |_, effect| owners.sealing.step(&mut owners.parent, effect))? {
                cx.consume_fuel(u64::try_from(work).unwrap_or(u64::MAX));
                match turn.result {
                    Ok(step) => cx.consume_retained(step.progress())?,
                    Err(error) => return Err(OriginalTurnFailure::debit_original(error, cx)),
                }
            }
            return Ok(None);
        }
        let parent = owners.parent.as_ref().ok_or_else(invalid)?.try_borrow()?;
        let command = owners.command.as_ref().ok_or_else(invalid)?.try_borrow()?;
        let context = input.context.ok_or_else(invalid)?;
        let pane = context.view_state.as_deref().and_then(|view| cad_pane_from_view(view).ok()).unwrap_or(CadPaneId::Shape);
        let handle = cad_pane_model(parent.get(), pane).ok_or_else(invalid)?;
        let slot = cad_pane_model_slot(pane);
        let key = MemberKeyRef::root(slot, &handle.child_id);

        if owners.preparation.is_some() {
            let seed = &input.operation.authoring_seed;
            let bytes = if seed.is_empty() { 0 } else { seed.len() + 160 };
            let demand = quote(2 * size_of::<Output>() + size_of::<ChildEmitPreparation>() + bytes, size_of::<ChildEmitPreparation>() + bytes);
            if !cx.retained_work_can_enter(demand) { return Ok(None); }
            let mut output = Output::default();
            output.child_preparations.try_reserve_exact(1).map_err(|_| invalid())?;
            output.child_preparations.push_back(owners.preparation.take().ok_or_else(invalid)?);
            output.transaction = (!seed.is_empty()).then(|| protocol::TransactionRef::mint(
                &protocol::ActorId(seed.as_str().into()),
                &protocol::HybridLogicalTimestamp { actor: 0, physical_ms: semio_framework_job::default_now_ms().unwrap_or(0), logical: 0 },
                "s.cad.cad@1/*#editor#addObject",
            ));
            output.ui_scope = semio_framework::kernel::UiDirtyScope::Full;
            cx.consume_retained(receipt(demand))?;
            cx.consume_fuel(1);
            return Ok(Some(output));
        }
        if owners.authority.is_some() {
            if owners.mutation.is_none() {
                let typology = command.get().typology.as_deref().unwrap_or("spatial.shape.primitive.box");
                let bytes = 2 * typology.len() + 2048;
                let demand = quote(bytes, bytes);
                if !cx.retained_work_can_enter(demand) { return Ok(None); }
                let count = owners.child.source()?.get().elements.len();
                let object = make_object_for_typology(typology, count, pane);
                owners.mutation = Some(SemioModelMutation::InsertElement(InsertElement { element: model_element_from_cad_object(&object), at: None }));
                cx.consume_retained(receipt(demand))?;
                cx.consume_fuel(1);
                return Ok(None);
            }
            let demand = ChildEmitPreparation::owned_qualified_birth_demand::<SemioModelMutation>(slot.len(), handle.child_id.len())?;
            if !cx.retained_work_can_enter(demand) { return Ok(None); }
            cx.consume_fuel(1);
            match admit_semio_model_insert_element(slot, &handle.child_id, &mut owners.mutation, &mut owners.authority, cx.retained_grant()) {
                Ok(Some((preparation, progress))) => {
                    owners.preparation = Some(preparation);
                    cx.consume_retained(progress)?;
                }
                Ok(None) => {}
                Err(error) => return Err(OriginalTurnFailure::debit_original(error, cx)),
            }
            return Ok(None);
        }
        if owners.lookup.is_none() {
            let demand = OriginalChildMemberLookup::birth_demand();
            if cx.retained_work_can_enter(demand) {
                let (lookup, progress) = OriginalChildMemberLookup::admit(cx.retained_grant())?;
                owners.lookup = Some(lookup);
                cx.consume_retained(progress)?;
                cx.consume_fuel(1);
            }
            return Ok(None);
        }
        if owners.read_cursor.is_none() {
            let demand = ChildReadCursor::birth_demand();
            if cx.retained_work_can_enter(demand) {
                let (cursor, progress) = ChildReadCursor::admit(cx.retained_grant())?;
                owners.read_cursor = Some(cursor);
                cx.consume_retained(progress)?;
                cx.consume_fuel(1);
            }
            return Ok(None);
        }
        if owners.child.ready() && owners.qualifier.is_none() {
            let demand = OriginalChildEditorCommandCursor::birth_demand();
            if cx.retained_work_can_enter(demand) {
                let (cursor, progress) = OriginalChildEditorCommandCursor::admit(cx.retained_grant())?;
                owners.qualifier = Some(cursor);
                cx.consume_retained(progress)?;
                cx.consume_fuel(1);
            }
            return Ok(None);
        }
        let lookup = owners.lookup.as_mut().ok_or_else(invalid)?;
        if !lookup.ready() {
            let demand = lookup.next_demand::<SemioModelSnapshot, _>(parent, &context.children, key, input.operation)?;
            if !cx.retained_work_can_enter(demand) { return Ok(None); }
            cx.consume_fuel(1);
            match SEMIO_MODEL_CHILD_READ.read_original(parent, &context.children, key, input.operation, lookup, cx.retained_grant()) {
                Ok(step) => cx.consume_retained(step.progress)?,
                Err(error) => return Err(OriginalTurnFailure::debit_original(error, cx)),
            }
            return Ok(None);
        }

        let original = erase_child_admission_source(parent, PARENT_SOURCE);
        let demand = lookup.quote_ready_receiving::<SemioModelSnapshot, _>(parent, &context.children, key, input.operation, |read| {
            if self.stamp.is_none() {
                owners.read_cursor.as_ref().ok_or_else(invalid)?.next_demand(original, &REFERENCE, pane.index(), read, &SEMIO_MODEL_CHILD_READ.identity)
            } else if !owners.child.ready() {
                owners.child.next_admit_demand()
            } else if self.qualified {
                owners.qualifier.as_ref().ok_or_else(invalid)?.take_authority_demand()
            } else {
                owners.qualifier.as_ref().ok_or_else(invalid)?.next_demand()
            }
        })?;
        if !cx.retained_work_can_enter(demand) { return Ok(None); }
        let loan = lookup.borrow_ready_demand::<SemioModelSnapshot>();
        let turn = lookup.borrow_ready::<SemioModelSnapshot, _>(parent, &context.children, key, input.operation, cx.retained_grant());
        let borrowed = match turn.result {
            Ok(borrowed) => borrowed,
            Err(error) => {
                cx.consume_fuel(u64::try_from(turn.work).unwrap_or(u64::MAX));
                return Err(OriginalTurnFailure::debit_original(error, cx));
            }
        };
        let read = borrowed.read.ok_or_else(invalid)?;
        let grant = RetainedCloneGrant { maximum_copy_bytes: cx.retained_grant().maximum_copy_bytes - loan.copy_bytes, ..cx.retained_grant() };
        let result = cx.with_original_source_effect(grant, |_, effect| -> Result<RetainedCloneProgress, ValueError> {
            let Some(stamp) = self.stamp.as_ref() else {
                let cursor = owners.read_cursor.as_mut().ok_or_else(invalid)?;
                let advance = SEMIO_MODEL_CHILD_READ.qualify(original, &REFERENCE, pane.index(), &read, cursor, effect)?;
                self.stamp = advance.stamp;
                return Ok(advance.progress);
            };
            if !owners.child.ready() {
                return Ok(SEMIO_MODEL_CHILD_READ.admit_original_source(&read, stamp, &mut owners.child, effect)?.progress());
            }
            let cursor = owners.qualifier.as_mut().ok_or_else(invalid)?;
            if self.qualified {
                let advance = cursor.take_authority(&owners.child, &read, effect)?;
                if advance.authority.is_some() { owners.authority = advance.authority; }
                return Ok(advance.progress);
            }
            let advance = SEMIO_MODEL_CHILD_INSERT_ELEMENT.qualify_original(parent.project(PARENT_SOURCE, |value| value), command, stamp, &read, cursor, effect)?;
            self.qualified = advance.stamp.is_some();
            Ok(advance.progress)
        })?;
        let Some((result, work)) = result else {
            cx.consume_fuel(u64::try_from(turn.work).unwrap_or(u64::MAX));
            cx.consume_retained(borrowed.progress)?;
            return Ok(None);
        };
        cx.consume_fuel(u64::try_from(work.max(turn.work)).unwrap_or(u64::MAX));
        let combine = |progress: RetainedCloneProgress| RetainedCloneProgress {
            copied_items: progress.copied_items.max(borrowed.progress.copied_items),
            copied_bytes: progress.copied_bytes + borrowed.progress.copied_bytes,
            retained_capacity_bytes: progress.retained_capacity_bytes + borrowed.progress.retained_capacity_bytes,
            released_bytes: progress.released_bytes + borrowed.progress.released_bytes,
        };
        match result {
            Ok(progress) => cx.consume_retained(combine(progress))?,
            Err(error) => {
                let progress = combine(error.retained_progress());
                return Err(OriginalTurnFailure::debit_original(error.with_retained_progress(progress), cx));
            }
        }
        Ok(None)
    }
}

impl ArtifactCommandWork<App> for CadAddObjectWork {
    fn tool_id(&self) -> &'static str { "addObject" }

    fn receive_original_command(&mut self, command: &mut CadCommand, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        let CadCommand::AddObject(command) = command else { return Err(invalid().into()); };
        if cx.should_yield() || cx.is_cancelled() { return Ok(false); }
        let owners = self.owners.original_mut().ok_or_else(invalid)?;
        if owners.command.is_some() && owners.staged.is_none() { return Ok(true); }
        let demand = match owners.staged.as_ref() {
            Some(staged) => {
                if cx.fuel_remaining() < staged.next_owned_work_demand()? { return Ok(false); }
                staged.next_owned_demand()?
            }
            None => quote(size_of::<Command>() + size_of::<Option<StagedSourceAdmission<Command, Command>>>(), 0),
        };
        if !cx.retained_work_can_enter(demand) { return Ok(false); }
        let Some((result, work)) = cx.with_original_source_effect(cx.retained_grant(), |_, effect| -> Result<RetainedCloneProgress, ValueError> {
            if let Some(staged) = owners.staged.as_mut() {
                let step = staged.advance_owned(&mut owners.command, effect).result?;
                if staged.terminal_is_empty() { owners.staged = None; }
                return Ok(step.progress());
            } else if effect.performed(1)? {
                owners.staged = Some(StagedSourceAdmission::owned(Command { typology: command.typology.take() }));
                return Ok(receipt(demand));
            }
            Ok(RetainedCloneProgress::default())
        })? else { return Ok(false); };
        cx.consume_fuel(u64::try_from(work).unwrap_or(u64::MAX));
        match result {
            Ok(progress) => cx.consume_retained(progress)?,
            Err(error) => return Err(OriginalTurnFailure::debit_original(error, cx)),
        }
        Ok(owners.command.is_some() && owners.staged.is_none())
    }

    fn extent(&self, command: &CadCommand, _: &CadSnapshot, _: &protocol::InteractionState, _: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<App>>) -> Option<usize> {
        matches!(command, CadCommand::AddObject(_)).then_some(1)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, App>, cx: &mut StepContext<'_>) -> Result<ArtifactCommandWorkStep<App>, OriginalTurnFailure> {
        for _ in 0..64 {
            let before = (cx.fuel_remaining(), cx.retained_progress());
            if let Some(output) = self.advance(input, cx)? { return Ok(ArtifactCommandWorkStep::Complete(output)); }
            if cx.should_yield() || cx.is_cancelled() || before == (cx.fuel_remaining(), cx.retained_progress()) { break; }
        }
        Ok(ArtifactCommandWorkStep::Pending { stage: "cad-add-object", preview: &[] })
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        let body = cx.retained_grant().maximum_copy_bytes;
        let demand = self.owners.next_demand(body)?;
        let birth = self.owners.requires_original_birth();
        let controlled = ErasedSnapshotRetirement::requires_original_control(&self.owners);
        if !birth && !controlled {
            return semio_framework_job::close_original_effect_step(cx, demand, |effect| self.owners.step(body, effect));
        }
        let pending = || InteractiveJobCloseStep::Pending { progress: Default::default() };
        if cx.retirement_should_yield() || !cx.retained_work_can_enter(demand) { return Ok(pending()); }
        let grant = cx.retained_grant();
        let Some((polled, _)) = cx.with_original_source_effect(grant, |cx, effect| cx.with_native_retirement(|native| native.with_original_close(|original| Ok::<_, ValueError>(
            if birth { self.owners.step_original_birth(original, grant) }
            else { ErasedSnapshotRetirement::close_original_step(&mut self.owners, original, body, effect) }
        ))))? else { return Ok(pending()); };
        let turn = match polled? {
            RetainedWorkPoll::Pending { work } => RetainedWorkOutcome { result: Ok(RetainedCloneStep::Progress(Default::default())), work },
            RetainedWorkPoll::Complete(turn) => turn,
        };
        let progress = semio_framework_job::receive_original_retained_work(cx, turn)?.progress();
        Ok(if self.owners.terminal_is_empty() { InteractiveJobCloseStep::Complete { progress } } else { InteractiveJobCloseStep::Pending { progress } })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.owners.next_demand(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.owners.next_demand(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.owners.next_demand(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.owners.next_demand(0)?.depth) }
    fn terminal_frame_release_bytes(&self) -> Option<usize> { self.terminal_is_empty().then_some(size_of::<Self>()) }
    fn terminal_is_empty(&self) -> bool { self.owners.terminal_is_empty() }
}
