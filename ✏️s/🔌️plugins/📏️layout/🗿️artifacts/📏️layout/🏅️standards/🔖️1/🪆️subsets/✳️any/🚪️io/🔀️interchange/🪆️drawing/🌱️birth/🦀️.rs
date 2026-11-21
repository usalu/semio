//! 🌱️ Drawing birth custody funds only parent reference duplication before the exact payload transfer.
use semio_framework_value::{
    OriginalRetirementCloseLoan, OriginalRetirementFailure, OriginalRetirementPoll, RetireOwned,
    RetirementDemand, ValueError, ValueRefusalKind,
    retained_clone::{
        OriginalEffectAction, OriginalEffectAuthority, OriginalEffectCallbackMode,
        OriginalEffectInvocationIdentity, OriginalEffectReply, OriginalEffectReplyComplete,
        OriginalProjectionProducer, OriginalSourceMode, OriginalSourceOwner,
        RetainedAllocationEffect, RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress,
        RetainedCloneStep, retained_clone_grant_admits,
    },
};
use semio_os_kernel::RetainedFieldPreparation;
use semio_s_artifact_layout_layout_model::{LayoutDrawingBirth, LayoutDrawingMember};
use std::mem::{ManuallyDrop, size_of};

type Reply<'r> = OriginalEffectReply<'r, RetainedCloneStep, OriginalRetirementFailure>;
type Complete<'r> = OriginalEffectReplyComplete<'r, RetainedCloneStep, OriginalRetirementFailure>;
type ProjectionAction = OriginalEffectAction<RetainedCloneStep, ValueError>;
type ProjectionReply = Result<RetainedCloneStep, ValueError>;
type ProjectionAuthority<S> = OriginalEffectAuthority<OriginalProjectionProducer<LayoutDrawingMember, LayoutDrawingBirth<S>>>;
type Output<S> = (LayoutDrawingMember, LayoutDrawingBirth<S>);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Action { Source, SourceFailure, Clone, TakeField, StartFieldClose, FieldClose, Handback, Return, CloseParent, CloseBirth, Retire, Done }

fn fault() -> ValueError { ValueError::invariant("Drawing import birth lost original custody") }

fn fixed(copy_bytes: usize) -> RetirementDemand {
    RetirementDemand { copy_bytes, depth: 1, ..Default::default() }
}

fn progress(demand: RetirementDemand) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }
}

fn admits(effect: &RetainedAllocationEffect<'_>, demand: RetirementDemand, work: u64) -> Result<bool, ValueError> {
    Ok(retained_clone_grant_admits(effect.grant(), demand) && effect.remaining_work_units()? >= work)
}

/// 🧳️ Holds one funded parent reference and the exact original typed Drawing birth.
pub struct DrawingBirthPreparation<S: RetireOwned + Sync + 'static> {
    source: OriginalSourceOwner<LayoutDrawingMember, LayoutDrawingBirth<S>>,
    field: RetainedFieldPreparation<LayoutDrawingMember>,
    parent: ManuallyDrop<Option<LayoutDrawingMember>>,
    birth: ManuallyDrop<Option<LayoutDrawingBirth<S>>>,
    retirement: RetainedCloneClose,
    projection_action: Option<ProjectionAction>,
    projection_reply: ManuallyDrop<Option<ProjectionReply>>,
    route: Option<(OriginalEffectInvocationIdentity, Action, usize)>,
    taken: bool,
    closing: bool,
}

impl<S: RetireOwned + Sync + 'static> Default for DrawingBirthPreparation<S> {
    fn default() -> Self {
        Self {
            source: Default::default(),
            field: Default::default(),
            parent: Default::default(),
            birth: Default::default(),
            retirement: Default::default(),
            projection_action: None,
            projection_reply: Default::default(),
            route: None,
            taken: false,
            closing: false,
        }
    }
}

impl<S: RetireOwned + Sync + 'static> DrawingBirthPreparation<S> {
    /// 📏️ Quotes the move from the caller's original birth place into the resident source.
    pub fn admission_demand() -> RetirementDemand {
        OriginalSourceOwner::<LayoutDrawingMember, LayoutDrawingBirth<S>>::capture_demand()
    }

    /// 🎟️ Receives the original birth only after the caller funds its exact capture.
    pub fn admit_into<'r>(&mut self, birth: &mut Option<LayoutDrawingBirth<S>>, effect: &mut RetainedAllocationEffect<'_>, reply: Reply<'r>) -> Result<Complete<'r>, ValueError> {
        if self.closing || self.taken || !self.terminal_is_empty() { return Err(fault()); }
        let mut result = None;
        drop(self.source.capture_into(birth, effect, OriginalEffectReply::borrow(&mut result)?)?);
        Ok(reply.complete(result.ok_or_else(fault)?.map_err(Into::into)))
    }

    pub fn ready(&self) -> bool {
        !self.closing && self.parent.is_some() && self.birth.is_some() && self.field.terminal_is_empty() && self.source.terminal_is_empty() && self.route.is_none()
    }

    fn select(&self, body: usize) -> Result<Action, ValueError> {
        if self.route.is_some() { return Err(fault()); }
        if self.source.failure().is_some() { return Ok(Action::SourceFailure); }
        if self.closing {
            if !self.field.terminal_is_empty() {
                return Ok(if self.field.close_started() { Action::FieldClose } else { Action::StartFieldClose });
            }
            if !self.source.terminal_is_empty() { return self.handback(body); }
            return Ok(if !self.retirement.is_empty() { Action::Retire } else if self.parent.is_some() { Action::CloseParent } else if self.birth.is_some() { Action::CloseBirth } else { Action::Done });
        }
        if self.taken || self.terminal_is_empty() { return Err(fault()); }
        if self.parent.is_some() {
            if !self.field.terminal_is_empty() {
                return Ok(if self.field.close_started() { Action::FieldClose } else { Action::StartFieldClose });
            }
            return if !self.source.terminal_is_empty() { self.handback(body) } else { Ok(Action::Done) };
        }
        if self.field.value().is_some() { return Ok(Action::TakeField); }
        if self.source.demand(OriginalSourceMode::Original, body)?.is_some() { return Ok(Action::Source); }
        Ok(Action::Clone)
    }

    fn handback(&self, body: usize) -> Result<Action, ValueError> {
        Ok(if self.source.demand(OriginalSourceMode::Unique, body)?.is_some() { Action::Handback } else { Action::Return })
    }

    fn quote(&self, action: Action, body: usize) -> Result<(RetirementDemand, u64), ValueError> {
        Ok(match action {
            Action::Source | Action::Handback => {
                let mode = if action == Action::Source { OriginalSourceMode::Original } else { OriginalSourceMode::Unique };
                let mut demand = self.source.demand(mode, body)?.ok_or_else(fault)?;
                let mut work = self.source.work_demand(mode, body)?;
                if self.source.projection_drive_required_for(mode)? {
                    let handoff = ProjectionAuthority::<S>::reply_handoff_demand()?;
                    demand.copy_bytes = demand.copy_bytes.checked_add(handoff.copy_bytes).ok_or_else(fault)?;
                    demand.depth = demand.depth.max(handoff.depth);
                    work = work.checked_add(2).ok_or_else(fault)?;
                }
                (demand, work)
            }
            Action::Clone => (self.field.demand(self.source.original()?, body, u64::MAX)?, self.field.advance_work_demand(self.source.original()?, body, u64::MAX)?),
            Action::SourceFailure => (self.source.failure_demand(), 1),
            Action::TakeField => (self.field.exchange_demand()?, 1),
            Action::StartFieldClose => (self.field.begin_close_demand(), self.field.begin_close_work_demand()),
            Action::FieldClose => (self.field.close_demand(body)?, self.field.close_work_demand(body)?),
            Action::Return => (self.source.return_demand(), 1),
            Action::CloseParent | Action::Retire => (
                self.retirement.next_owner_demands::<LayoutDrawingMember>(self.parent.is_some(), &None, body)?,
                self.retirement.next_owner_work::<LayoutDrawingMember>(self.parent.is_some(), &None, body)?,
            ),
            Action::CloseBirth => (
                self.retirement.next_owner_demands::<LayoutDrawingBirth<S>>(true, &None, body)?,
                self.retirement.next_owner_work::<LayoutDrawingBirth<S>>(true, &None, body)?,
            ),
            Action::Done => (RetirementDemand::default(), 0),
        })
    }

    pub fn demand(&self, body: usize) -> Result<RetirementDemand, ValueError> { Ok(self.quote(self.select(body)?, body)?.0) }
    pub fn work_demand(&self, body: usize) -> Result<u64, ValueError> { Ok(self.quote(self.select(body)?, body)?.1) }

    pub fn requires_original_birth(&self, body: usize) -> Result<bool, ValueError> {
        if self.route.is_some() { return Ok(false); }
        Ok(match self.select(body)? {
            Action::FieldClose => self.field.requires_original_birth(),
            Action::Handback => self.source.requires_original_birth(),
            Action::Retire => self.retirement.requires_original_birth(),
            _ => false,
        })
    }

    pub fn original_birth_step(&mut self, loan: &mut OriginalRetirementCloseLoan<'_>, body: usize, grant: RetainedCloneGrant) -> Result<OriginalRetirementPoll, ValueError> {
        Ok(match self.select(body)? {
            Action::FieldClose if self.field.requires_original_birth() => self.field.close_original_birth_step(loan, grant),
            Action::Handback if self.source.requires_original_birth() => self.source.original_birth_step(loan, grant),
            Action::Retire if self.retirement.requires_original_birth() => self.retirement.step_original_birth(loan, grant),
            _ => return Err(fault()),
        })
    }

    pub fn requires_original_control(&self, body: usize) -> Result<bool, ValueError> {
        let action = match self.route { Some((_, action, selected)) if selected == body => action, _ => self.select(body)? };
        Ok(match action {
            Action::FieldClose => self.field.requires_original_control(),
            Action::Handback => self.source.requires_original_control(),
            Action::Retire => self.retirement.requires_original_control(),
            _ => false,
        })
    }

    /// 🪡️ Records one selected preparation or cancellation action in the caller's receiving place.
    pub fn step_into<'r>(&mut self, body: usize, loan: Option<&mut OriginalRetirementCloseLoan<'_>>, effect: &mut RetainedAllocationEffect<'_>, reply: Reply<'r>) -> Result<Complete<'r>, ValueError> {
        let action = self.select(body)?;
        if action == Action::Done { return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default())))); }
        let (demand, work) = self.quote(action, body)?;
        if !admits(effect, demand, work)? { return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default())))); }
        self.step_selected(action, body, loan, effect, reply, false)
    }

    /// 🧵️ Resumes the same selected action and keeps its original reply and owner places.
    pub fn recover_step_into<'r>(&mut self, body: usize, loan: Option<&mut OriginalRetirementCloseLoan<'_>>, effect: &mut RetainedAllocationEffect<'_>, reply: Reply<'r>) -> Result<Complete<'r>, ValueError> {
        let (_, action, selected_body) = self.route.ok_or_else(fault)?;
        if selected_body != body { return Err(fault()); }
        self.step_selected(action, body, loan, effect, reply, true)
    }

    fn step_selected<'r>(&mut self, action: Action, body: usize, loan: Option<&mut OriginalRetirementCloseLoan<'_>>, effect: &mut RetainedAllocationEffect<'_>, reply: Reply<'r>, recover: bool) -> Result<Complete<'r>, ValueError> {
        match action {
            Action::SourceFailure => {
                let (mut failure, mut result) = (None, None);
                drop(self.source.take_failure_into(&mut failure, effect, OriginalEffectReply::borrow(&mut result)?)?);
                let result = result.ok_or_else(fault)?;
                return Ok(reply.progress().complete(match (failure, result) {
                    (Some(error), Ok(step)) => Err(error.with_retained_progress(step.progress()).into()),
                    (_, result) => result.map_err(Into::into),
                }));
            }
            Action::TakeField | Action::Return => {
                let mut result = None;
                let inner = OriginalEffectReply::borrow(&mut result)?;
                drop(if action == Action::TakeField {
                    self.field.take_into(&mut self.parent, effect, inner)?
                } else {
                    self.source.return_into(&mut self.birth, effect, inner)?
                });
                let reply = if self.ready() { reply } else { reply.progress() };
                return Ok(reply.complete(result.ok_or_else(fault)?.map_err(Into::into)));
            }
            Action::StartFieldClose => {
                let turn = self.field.begin_close_paid(effect);
                return Ok(reply.complete(turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)));
            }
            Action::CloseParent | Action::CloseBirth => {
                let turn = if action == Action::CloseParent {
                    self.retirement.begin_granted(&mut self.parent, effect)
                } else {
                    self.retirement.begin_granted(&mut self.birth, effect)
                };
                return Ok(reply.complete(turn.result.map(|step| step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)));
            }
            _ => {}
        }

        let invocation = effect.invocation().ok_or_else(fault)?;
        let mode = if recover { OriginalEffectCallbackMode::Recover } else { OriginalEffectCallbackMode::Receive };
        let key = (invocation.identity(), action, body);
        if invocation.mode() != mode || (recover && self.route != Some(key)) { return Err(fault()); }
        let state = self;
        let mut scope = reply.progress().scope(&mut state.route, key)?;
        if matches!(action, Action::Source | Action::Handback) {
            let mode = if action == Action::Source { OriginalSourceMode::Original } else { OriginalSourceMode::Unique };
            if state.source.projection_drive_required_for(mode)? {
                let frame = state.source.projection_frame(effect)?;
                let mut context = frame.context()?;
                let authority = frame.authority()?;
                if authority.drive(&mut context, effect, &mut state.projection_action)? {
                    let transfer = authority.transfer_reply_into(&context, &mut state.projection_action, &mut state.projection_reply, effect);
                    if let Some(paid) = transfer.result? {
                        if !effect.performed(1)? { return Err(fault()); }
                        let result = state.projection_reply.take().ok_or_else(fault)?;
                        let copy_bytes = paid.copied_bytes + ProjectionAuthority::<S>::reply_release_demand()?.copy_bytes;
                        drop(scope.borrow()?.complete(result.map(|step| {
                            let mut posted = step.progress();
                            posted.copied_bytes += copy_bytes;
                            RetainedCloneStep::Progress(posted)
                        }).map_err(Into::into)));
                    }
                }
            } else {
                drop(match (loan, recover) {
                    (None, false) => state.source.prepare_into(mode, body, effect, scope.borrow()?)?,
                    (None, true) => state.source.recover_prepare_into(mode, body, effect, scope.borrow()?)?,
                    (Some(loan), false) => state.source.prepare_original_into(mode, body, loan, effect, scope.borrow()?)?,
                    (Some(loan), true) => state.source.recover_prepare_original_into(mode, body, loan, effect, scope.borrow()?)?,
                });
            }
        } else {
            let child = scope.borrow()?;
            drop(match (action, loan, recover) {
                (Action::Clone, None, false) => state.field.advance_into(state.source.original()?, body, u64::MAX, effect, child)?,
                (Action::Clone, None, true) => state.field.recover_advance_into(state.source.original()?, body, u64::MAX, effect, child)?,
                (Action::FieldClose, None, false) => state.field.close_into(body, effect, child)?,
                (Action::FieldClose, None, true) => state.field.recover_close_into(body, effect, child)?,
                (Action::FieldClose, Some(loan), false) => state.field.close_original_into(loan, body, effect, child)?,
                (Action::FieldClose, Some(loan), true) => state.field.recover_close_original_into(loan, body, effect, child)?,
                (Action::Retire, None, false) => state.retirement.step_into(body, effect, child)?,
                (Action::Retire, None, true) => state.retirement.recover_step_into(body, effect, child)?,
                (Action::Retire, Some(loan), false) => state.retirement.step_original_into(loan, body, effect, child)?,
                (Action::Retire, Some(loan), true) => state.retirement.recover_step_original_into(loan, body, effect, child)?,
                _ => return Err(fault()),
            });
        }
        Ok(scope.forwarded())
    }

    pub fn take_demand(&self) -> Result<RetirementDemand, ValueError> {
        if !self.ready() { return Err(fault()); }
        Ok(fixed(size_of::<Option<Output<S>>>() + size_of::<Option<LayoutDrawingMember>>() + size_of::<Option<LayoutDrawingBirth<S>>>() + size_of::<bool>()))
    }

    /// 📤️ Publishes the prepared reference and exactly the caller's original birth.
    pub fn take_into<'r>(&mut self, output: &mut Option<Output<S>>, effect: &mut RetainedAllocationEffect<'_>, reply: Reply<'r>) -> Result<Complete<'r>, ValueError> {
        let demand = self.take_demand()?;
        if output.is_some() { return Err(fault()); }
        if !admits(effect, demand, 1)? { return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default())))); }
        if !effect.performed(1)? { return Err(fault()); }
        *output = Some((self.parent.take().expect("quoted parent"), self.birth.take().expect("quoted birth")));
        self.taken = true;
        Ok(reply.complete(Ok(RetainedCloneStep::Complete(progress(demand)))))
    }

    pub fn begin_close_demand(&self) -> RetirementDemand { fixed(size_of::<bool>()) }

    /// 🚦️ Switches to funded closure after the original action has settled.
    pub fn begin_close_into<'r>(&mut self, effect: &mut RetainedAllocationEffect<'_>, reply: Reply<'r>) -> Result<Complete<'r>, ValueError> {
        if self.route.is_some() { return Err(fault()); }
        if self.closing { return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default())))); }
        let demand = self.begin_close_demand();
        if !admits(effect, demand, 1)? { return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default())))); }
        if !effect.performed(1)? { return Err(fault()); }
        self.closing = true;
        Ok(reply.complete(Ok(RetainedCloneStep::Complete(progress(demand)))))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.field.terminal_is_empty() && self.source.terminal_is_empty() && self.parent.is_none() &&
            self.birth.is_none() && self.retirement.is_empty() && self.route.is_none() &&
            self.projection_action.is_none() && self.projection_reply.is_none()
    }
}

impl<S: RetireOwned + Sync + 'static> Drop for DrawingBirthPreparation<S> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "Drawing birth preparation requires original-funded closure");
    }
}
