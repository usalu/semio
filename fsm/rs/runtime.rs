//! 🎬 Actor runtime — mailboxes, spawn, and command routing atop the pure kernel.
//!
//! Every actor processes its mailbox serially; nothing here mutates a snapshot
//! concurrently, even on multithreaded native targets.

use crate::host::Host;
use crate::kernel::{init, macrostep, Command, Status};
use crate::{ActorId, Machine, NullInspector, Snapshot, StepReport};
use std::collections::VecDeque;

//#region 🔖ActorLogic

/// 🎭 The shape of runnable actor logic — implemented for any [`Machine`] via [`MachineLogic`].
pub trait ActorLogic {
    type Event;
    type Input;
    type Output;
    type Snapshot;
}

/// 🎭 Blanket [`ActorLogic`] for any compiled [`Machine`].
pub struct MachineLogic<M: Machine>(core::marker::PhantomData<M>);

impl<M: Machine> ActorLogic for MachineLogic<M> {
    type Event = M::Event;
    type Input = M::Input;
    type Output = M::Output;
    type Snapshot = Snapshot<M>;
}

//#endregion 🔖ActorLogic

//#region 🔖Actor

/// 🎬 One running machine instance: its snapshot plus a serial mailbox.
struct Actor<M: Machine> {
    id: ActorId,
    snapshot: Snapshot<M>,
    mailbox: VecDeque<M::Event>,
}

//#endregion 🔖Actor

//#region 🔖System

/// 🎬 Owns every spawned [`Actor`] for one machine type and routes their [`Command`]s
/// to a [`Host`]. Mailboxes drain in round-robin order until quiescent.
pub struct ActorSystem<M: Machine, H: Host<M>> {
    pub host: H,
    actors: Vec<Actor<M>>,
    next_id: u32,
}

impl<M: Machine, H: Host<M>> ActorSystem<M, H> {
    /// 🎬 A fresh system with no actors yet, owning `host`.
    pub fn new(host: H) -> Self {
        Self { host, actors: Vec::new(), next_id: 0 }
    }

    /// 🎬 Initializes and registers a root actor, routing its initial commands immediately.
    pub fn spawn_root(&mut self, input: M::Input) -> ActorId {
        let id = ActorId(self.next_id);
        self.next_id += 1;
        let mut buffer: Vec<Command<M>> = Vec::new();
        let snapshot = init::<M>(input, &mut buffer);
        self.actors.push(Actor { id, snapshot, mailbox: VecDeque::new() });
        self.route_commands(id, buffer);
        id
    }

    /// 🎬 The current [`Snapshot`] of an actor, if it exists.
    pub fn snapshot(&self, id: ActorId) -> Option<&Snapshot<M>> {
        self.actors.iter().find(|a| a.id == id).map(|a| &a.snapshot)
    }

    /// 🎬 Enqueues an event for delivery on the next [`ActorSystem::drain`].
    pub fn send(&mut self, to: ActorId, event: M::Event) {
        if let Some(actor) = self.actors.iter_mut().find(|a| a.id == to) {
            actor.mailbox.push_back(event);
        }
    }

    /// 🎬 Delivers a [`TimerId`](crate::TimerId) elapsed notification straight to `macrostep`'s
    /// timer entry point for `to`.
    pub fn timer_elapsed(&mut self, to: ActorId, timer: crate::TimerId) -> Option<StepReport> {
        let idx = self.actors.iter().position(|a| a.id == to)?;
        let mut buffer: Vec<Command<M>> = Vec::new();
        let mut inspector = NullInspector;
        let report = crate::kernel::timer_elapsed(&mut self.actors[idx].snapshot, timer, &mut buffer, &mut inspector);
        self.route_commands(to, buffer);
        Some(report)
    }

    /// 🎬 Drains every actor's mailbox to quiescence, running one macrostep per delivered event.
    pub fn drain(&mut self) -> Vec<StepReport> {
        let mut reports = Vec::new();
        loop {
            let mut progressed = false;
            for idx in 0..self.actors.len() {
                let Some(event) = self.actors[idx].mailbox.pop_front() else {
                    continue;
                };
                progressed = true;
                let id = self.actors[idx].id;
                let mut buffer: Vec<Command<M>> = Vec::new();
                let mut inspector = NullInspector;
                let report = macrostep(&mut self.actors[idx].snapshot, event, &mut buffer, &mut inspector);
                self.route_commands(id, buffer);
                reports.push(report);
            }
            if !progressed {
                break;
            }
        }
        reports
    }

    fn route_commands(&mut self, actor: ActorId, commands: Vec<Command<M>>) {
        let mut sends = Vec::new();
        if let Some(idx) = self.actors.iter().position(|a| a.id == actor) {
            for command in commands {
                if let Some(pair) = route_command(&mut self.host, &mut self.actors[idx].snapshot, actor, command) {
                    sends.push(pair);
                }
            }
        }
        for (to, event) in sends {
            self.send(to, event);
        }
    }
}

/// 🎬 Applies one [`Command`] to `host`/`snapshot`; returns a `Send` command's
/// `(to, event)` pair for the caller to route on, since a lone [`Host`]+[`Snapshot`]
/// pair (e.g. a single `export_wasm_machine!` instance) has no other actor to deliver it to.
pub fn route_command<M: Machine>(host: &mut impl Host<M>, snapshot: &mut Snapshot<M>, actor: ActorId, command: Command<M>) -> Option<(ActorId, M::Event)> {
    match command {
        Command::Effect(effect) => {
            host.execute_effect(actor, effect);
            None
        }
        Command::Raise(_) => {
            // The kernel's run-to-completion loop already re-processed this
            // internally; forwarded here only for host-side observability.
            None
        }
        Command::Send { to, event } => Some((to, event)),
        Command::Emit(output) => {
            snapshot.status = Status::Done(output);
            None
        }
        Command::StartInvoke(invoke) => {
            host.start_task(actor, invoke);
            None
        }
        Command::StopInvoke(invoke) => {
            host.cancel_task(actor, invoke);
            None
        }
        Command::Schedule { timer, delay_ms } => {
            host.schedule(actor, timer, delay_ms);
            None
        }
        Command::CancelTimer(timer) => {
            host.cancel_timer(actor, timer);
            None
        }
    }
}

//#endregion 🔖System

//#region 🧪Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::TestHost;
    use crate::testing::support::{UnitToggleContext, UnitToggleEvent, UnitToggleMachine};

    #[test]
    fn actor_system_drains_sent_events_through_one_macrostep_each() {
        let mut system: ActorSystem<UnitToggleMachine, TestHost<UnitToggleMachine>> = ActorSystem::new(TestHost::new());
        let root = system.spawn_root(());
        assert!(system.snapshot(root).unwrap().matches("off"));

        system.send(root, UnitToggleEvent::Flip);
        let reports = system.drain();
        assert_eq!(reports.len(), 1);
        assert!(system.snapshot(root).unwrap().matches("on"));

        system.send(root, UnitToggleEvent::Flip);
        system.drain();
        assert!(system.snapshot(root).unwrap().matches("off"));
        assert_eq!(system.snapshot(root).unwrap().context, UnitToggleContext::default());
    }
}

//#endregion 🧪Tests
