//! 🎭 Statechart kernel, actor runtime and hosts — XState-parity FSMs as static tables.
//!
//! Machine structure is compile-time (dense static tables addressed by `NodeId`s);
//! machine state and actor instances are runtime. See [`Machine`] and [`statechart!`].

extern crate self as fsm;

mod host;
mod inspect;
mod kernel;
mod persist;
mod runtime;
#[cfg(any(test, feature = "testing"))]
mod testing;

//#region 🔖Ids

/// 🔢 Dense index of a state node within a compiled [`MachineDefinition`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub u16);

/// 🔢 Dense index of an event kind within a [`StatechartEvent`] enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventId(pub u16);

/// 🔢 Dense index of a transition within a compiled [`MachineDefinition`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransitionId(pub u16);

/// 🔢 Dense index of a guard function within a compiled [`MachineDefinition`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GuardId(pub u16);

/// 🔢 Dense index of a reducer/emitter action within a compiled [`MachineDefinition`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActionId(pub u16);

/// 🔢 Dense index of an `invoke` declaration within a compiled [`MachineDefinition`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InvokeId(pub u16);

/// 🔢 Dense index of an `after` (delayed transition) timer within a compiled [`MachineDefinition`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimerId(pub u16);

/// 🔢 Runtime-assigned identity of a spawned actor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActorId(pub u32);

/// 🧮 Hand-rolled fixed-size bitset over `W` `u64` words — one bit per [`NodeId`].
///
/// The `statechart!` macro emits a concrete `BitSet<W>` sized to the machine's
/// state count; the kernel only ever operates on it through [`Configuration`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BitSet<const W: usize> {
    words: [u64; W],
}

impl<const W: usize> BitSet<W> {
    /// 🧮 An empty bitset with no bits set.
    pub const fn empty() -> Self {
        Self { words: [0u64; W] }
    }
}

impl<const W: usize> Default for BitSet<W> {
    fn default() -> Self {
        Self::empty()
    }
}

/// 🧩 Active-state configuration operations, implemented by the macro-generated `BitSet<W>`.
///
/// The active configuration of a running machine is a set of atomic `NodeId`s —
/// never a single enum variant — so that parallel regions can hold multiple
/// simultaneously-active states.
pub trait Configuration: Clone + PartialEq + Default {
    /// ➕ Marks `id` as part of the active configuration.
    fn set(&mut self, id: NodeId);
    /// ➖ Removes `id` from the active configuration.
    fn clear(&mut self, id: NodeId);
    /// ❓ Whether `id` is part of the active configuration.
    fn contains(&self, id: NodeId) -> bool;
    /// 🔁 Iterates active `NodeId`s in ascending order.
    fn iter_ones(&self) -> ConfigurationIter<'_, Self>;
    /// 🧹 Clears every bit.
    fn clear_all(&mut self);
    /// ❓ Whether no bit is set.
    fn is_empty(&self) -> bool;
}

/// 🔁 Iterator over the active `NodeId`s of a [`Configuration`].
pub struct ConfigurationIter<'a, C: Configuration + ?Sized> {
    words: &'a [u64],
    word_index: usize,
    current: u64,
    _marker: core::marker::PhantomData<C>,
}

impl<'a, C: Configuration + ?Sized> Iterator for ConfigurationIter<'a, C> {
    type Item = NodeId;

    fn next(&mut self) -> Option<NodeId> {
        loop {
            if self.current != 0 {
                let bit = self.current.trailing_zeros();
                self.current &= self.current - 1;
                return Some(NodeId((self.word_index as u16 - 1) * 64 + bit as u16));
            }
            self.word_index += 1;
            self.current = *self.words.get(self.word_index - 1)?;
        }
    }
}

impl<const W: usize> Configuration for BitSet<W> {
    fn set(&mut self, id: NodeId) {
        let (word, bit) = (id.0 as usize / 64, id.0 as usize % 64);
        self.words[word] |= 1u64 << bit;
    }

    fn clear(&mut self, id: NodeId) {
        let (word, bit) = (id.0 as usize / 64, id.0 as usize % 64);
        self.words[word] &= !(1u64 << bit);
    }

    fn contains(&self, id: NodeId) -> bool {
        let (word, bit) = (id.0 as usize / 64, id.0 as usize % 64);
        self.words[word] & (1u64 << bit) != 0
    }

    fn iter_ones(&self) -> ConfigurationIter<'_, Self> {
        ConfigurationIter {
            words: &self.words,
            word_index: 0,
            current: 0,
            _marker: core::marker::PhantomData,
        }
    }

    fn clear_all(&mut self) {
        self.words = [0u64; W];
    }

    fn is_empty(&self) -> bool {
        self.words.iter().all(|w| *w == 0)
    }
}

//#endregion 🔖Ids

//#region 🔖Schema

/// 🏷️ A consumer-defined event enum, reflected into a dense [`EventId`] space.
///
/// Implemented via `#[derive(StatechartEvent)]` — see `fsm_macros`.
pub trait StatechartEvent: Clone {
    /// 🔢 Number of distinct event variants.
    const EVENT_COUNT: u16;
    /// 🔢 The dense id of this event's variant.
    fn event_id(&self) -> EventId;
    /// 🏷️ The variant's declared name, for diagnostics/inspection/manifest.
    fn event_name(id: EventId) -> &'static str;
}

/// 🎭 A compiled statechart: consumer-owned types bound to a static [`MachineDefinition`].
///
/// Implemented by the marker type generated by `statechart! { machine name { … } }`.
pub trait Machine: Sized + 'static {
    /// 📦 Consumer-owned, mutable data carried alongside the active configuration.
    type Context;
    /// 📨 Consumer-owned event enum, reflected via [`StatechartEvent`].
    type Event: StatechartEvent;
    /// 📥 Consumer-owned input to [`kernel::init`].
    type Input;
    /// 📤 Consumer-owned output produced when the machine reaches a final state.
    type Output: Clone;
    /// 🎇 Consumer-owned effect payloads requested via [`kernel::Command::Effect`].
    type Effect;
    /// 🧮 The macro-sized `BitSet<W>` for this machine's state count.
    type Config: Configuration;

    /// 📐 The compiled static definition backing this machine.
    fn definition() -> &'static kernel::MachineDefinition<Self>;
}

/// 🏷️ Struct-level field name/type metadata, embedded as JSON — feeds TypeScript
/// generation tooling. Implemented via `#[derive(StatechartSchema)]`.
pub trait StatechartSchema {
    /// 🏷️ `{"fields":[{"name":"..","type":".."}, ..]}` for this struct's named fields.
    const SCHEMA_JSON: &'static str;
}

//#endregion 🔖Schema

//#region 🔖Reexports

pub use host::{Host, NativeHost, TestHost};
pub use inspect::{InspectionEvent, Inspector, MicrostepTrace, NullInspector, TraceInspector};
pub use kernel::{
    init, macrostep, timer_elapsed, ActionFn, Command, CommandSink, GuardFn, InputFn, MachineDefinition,
    NodeDef, NodeKind, OutputFn, Snapshot, Status, StepReport, TransitionDef, TransitionKind, Trigger,
    MICROSTEP_LIMIT, ROOT,
};
pub use persist::{persist, restore, Migration, PersistedSnapshot, RestoreError};
pub use runtime::{route_command, ActorLogic, ActorSystem, MachineLogic};

#[cfg(any(test, feature = "testing"))]
pub use testing::{check_invariants, explore, run_conformance, ConformanceStep, Coverage, Invariant, Model};

// 🪄 `StatechartEvent` here re-exports the derive macro — it shares its name with the
// `StatechartEvent` trait above without conflict since macros and traits live in
// separate namespaces (the same pattern `serde`/`serde_derive` use).
#[cfg(feature = "macros")]
pub use fsm_macros::{statechart, StatechartEvent, StatechartSchema};

#[cfg(all(feature = "macros", target_arch = "wasm32"))]
pub use fsm_macros::export_wasm_machine;

//#endregion 🔖Reexports

//#region 🔖WasmBridge

#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use crate::{ActorId, Host, InvokeId, Machine, TimerId};

    /// 🌐 A [`Host`] backed by browser timers and a JS effect callback. Timers are
    /// polled by the caller via [`WasmHost::due_timers`] (driven by the generated
    /// `export_wasm_machine!` wrapper's `tick()`), matching [`crate::NativeHost`] and
    /// [`crate::TestHost`]'s caller-driven clock — no `async fn` anywhere in the host.
    pub struct WasmHost<M: Machine> {
        effect_callback: Option<js_sys::Function>,
        pending_timers: Vec<(ActorId, TimerId, f64)>,
        started_tasks: Vec<(ActorId, InvokeId)>,
        _marker: core::marker::PhantomData<M>,
    }

    impl<M: Machine> WasmHost<M> {
        /// 🌐 A fresh host with no effect callback registered yet.
        pub fn new() -> Self {
            Self {
                effect_callback: None,
                pending_timers: Vec::new(),
                started_tasks: Vec::new(),
                _marker: core::marker::PhantomData,
            }
        }

        /// 🌐 Registers the JS function invoked as `(actorId: number, effectJson: string) => void`.
        pub fn set_effect_callback(&mut self, callback: js_sys::Function) {
            self.effect_callback = Some(callback);
        }

        /// 🚀 Tasks started via `invoke`, still pending cancellation.
        pub fn started_tasks(&self) -> &[(ActorId, InvokeId)] {
            &self.started_tasks
        }

        /// ⏱️ Removes and returns every timer whose deadline has passed, per `js_sys::Date::now()`.
        pub fn due_timers(&mut self) -> Vec<(ActorId, TimerId)> {
            let now = js_sys::Date::now();
            let mut due = Vec::new();
            self.pending_timers.retain(|(actor, timer, at)| {
                if *at <= now {
                    due.push((*actor, *timer));
                    false
                } else {
                    true
                }
            });
            due
        }
    }

    impl<M: Machine> Default for WasmHost<M> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<M: Machine> Host<M> for WasmHost<M>
    where
        M::Effect: serde::Serialize,
    {
        fn execute_effect(&mut self, actor: ActorId, effect: M::Effect) {
            if let Some(callback) = &self.effect_callback {
                if let Ok(json) = serde_json::to_string(&effect) {
                    let _ = callback.call2(&wasm_bindgen::JsValue::NULL, &wasm_bindgen::JsValue::from_f64(actor.0 as f64), &wasm_bindgen::JsValue::from_str(&json));
                }
            }
        }

        fn schedule(&mut self, actor: ActorId, timer: TimerId, delay_ms: u64) {
            self.pending_timers.push((actor, timer, js_sys::Date::now() + delay_ms as f64));
        }

        fn cancel_timer(&mut self, actor: ActorId, timer: TimerId) {
            self.pending_timers.retain(|(a, t, _)| !(*a == actor && *t == timer));
        }

        fn start_task(&mut self, actor: ActorId, invoke: InvokeId) {
            self.started_tasks.push((actor, invoke));
        }

        fn cancel_task(&mut self, actor: ActorId, invoke: InvokeId) {
            self.started_tasks.retain(|(a, i)| !(*a == actor && *i == invoke));
        }

        fn now_ms(&self) -> u64 {
            js_sys::Date::now() as u64
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_bridge::WasmHost;

/// 🎫 A minimal always-compiled `export_wasm_machine!` consumer — proves the wasm
/// target and the whole DSL → kernel → `WasmHost` → JSON-boundary path continuously,
/// independent of any downstream consumer crate.
#[cfg(target_arch = "wasm32")]
mod wasm_smoke {
    #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
    pub struct ToggleContext;

    fn build_context(_input: ()) -> ToggleContext {
        ToggleContext
    }

    crate::statechart! {
        machine toggle {
            context: ToggleContext;
            event Event { Flip }
            input: ();
            output: ();
            effect: ();
            context_from_input: build_context;
            initial: off;
            state off {
                on Flip => on;
            }
            state on {
                on Flip => off;
            }
        }
    }

    crate::export_wasm_machine!(toggle::Toggle, "ToggleMachine");
}

//#endregion 🔖WasmBridge

//#region 🧪Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitset_set_clear_contains() {
        let mut bits = BitSet::<1>::empty();
        assert!(!bits.contains(NodeId(3)));
        bits.set(NodeId(3));
        assert!(bits.contains(NodeId(3)));
        bits.clear(NodeId(3));
        assert!(!bits.contains(NodeId(3)));
    }

    #[test]
    fn bitset_iter_ones_spans_words() {
        let mut bits = BitSet::<2>::empty();
        bits.set(NodeId(0));
        bits.set(NodeId(63));
        bits.set(NodeId(64));
        bits.set(NodeId(100));
        let ids: Vec<u16> = bits.iter_ones().map(|n| n.0).collect();
        assert_eq!(ids, vec![0, 63, 64, 100]);
    }

    #[test]
    fn bitset_clear_all_and_is_empty() {
        let mut bits = BitSet::<1>::empty();
        assert!(bits.is_empty());
        bits.set(NodeId(5));
        assert!(!bits.is_empty());
        bits.clear_all();
        assert!(bits.is_empty());
    }
}

/// 🎫 End-to-end proof that `statechart!` DSL → kernel → runtime → [`TestHost`] timers/invoke
/// → persist/restore → inspection trace → model coverage all compose over one real machine.
#[cfg(feature = "macros")]
#[cfg(test)]
mod checkout_integration {
    use crate::{ActorSystem, CommandSink, InvokeId, Machine, TestHost, TimerId, TraceInspector};

    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct CheckoutContext {
        pub attempts: u32,
        pub method_set: bool,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Receipt {
        pub attempts: u32,
    }

    fn build_context(_input: ()) -> CheckoutContext {
        CheckoutContext::default()
    }

    fn allow_select(ctx: &CheckoutContext, _event: Option<&checkout::Event>) -> bool {
        ctx.attempts < 3
    }

    fn set_method(ctx: &mut CheckoutContext, _event: Option<&checkout::Event>, _sink: &mut dyn CommandSink<checkout::Checkout>) {
        ctx.method_set = true;
    }

    fn note_timeout(ctx: &mut CheckoutContext, _event: Option<&checkout::Event>, _sink: &mut dyn CommandSink<checkout::Checkout>) {
        ctx.attempts += 1;
    }

    fn make_receipt(ctx: &CheckoutContext) -> Receipt {
        Receipt { attempts: ctx.attempts }
    }

    crate::statechart! {
        machine checkout {
            context: CheckoutContext;
            event Event { Confirm, SelectMethod, PaymentSucceeded, PaymentFailed, Retry, Cancel, Resume, ShipDone, InvoiceDone }
            input: ();
            output: Receipt;
            effect: ();
            context_from_input: build_context;
            output_from_context: make_receipt;
            initial: cart;

            state cart {
                on Confirm => payment;
                on Resume => payment_history;
            }
            state payment {
                initial: selecting;
                history payment_history shallow;
                state selecting {
                    on SelectMethod if allow_select => processing do set_method;
                }
                state processing {
                    invoke charge;
                    after 5000 => failed do note_timeout;
                    on PaymentSucceeded => fulfilment;
                    on PaymentFailed => failed;
                    on Cancel => cart;
                }
                state failed {
                    on Retry => processing;
                }
            }
            parallel fulfilment {
                state shipping {
                    initial: ship_pending;
                    state ship_pending { on ShipDone => ship_done; }
                    final ship_done;
                }
                state invoicing {
                    initial: invoice_pending;
                    state invoice_pending { on InvoiceDone => invoice_done; }
                    final invoice_done;
                }
                on_done => done;
            }
            final done;
        }
    }

    #[test]
    fn dsl_machine_walks_cart_to_receipt() {
        let mut system: ActorSystem<checkout::Checkout, TestHost<checkout::Checkout>> = ActorSystem::new(TestHost::new());
        let root = system.spawn_root(());
        assert!(system.snapshot(root).unwrap().matches("cart"));

        system.send(root, checkout::Event::Confirm);
        system.drain();
        assert!(system.snapshot(root).unwrap().matches("selecting"));

        system.send(root, checkout::Event::SelectMethod);
        system.drain();
        assert!(system.snapshot(root).unwrap().matches("processing"));
        assert!(system.snapshot(root).unwrap().context.method_set);
        assert_eq!(system.host.started_tasks(), &[(root, InvokeId(0))]);

        system.send(root, checkout::Event::PaymentSucceeded);
        system.drain();
        assert!(system.snapshot(root).unwrap().matches("ship_pending"));
        assert!(system.snapshot(root).unwrap().matches("invoice_pending"));
        assert_eq!(system.host.cancelled_tasks(), &[(root, InvokeId(0))], "leaving processing must stop its invoke");

        system.send(root, checkout::Event::ShipDone);
        system.drain();
        assert!(system.snapshot(root).unwrap().matches("ship_done"));
        assert!(system.snapshot(root).unwrap().matches("invoice_pending"), "invoicing region must still be pending");
        system.send(root, checkout::Event::InvoiceDone);
        system.drain();

        assert!(matches!(system.snapshot(root).unwrap().status, crate::Status::Done(_)));
        if let crate::Status::Done(receipt) = &system.snapshot(root).unwrap().status {
            assert_eq!(receipt.attempts, 0);
        }
    }

    #[test]
    fn dsl_machine_cancel_resume_round_trips_via_shallow_history() {
        let mut sink: Vec<crate::Command<checkout::Checkout>> = Vec::new();
        let mut snapshot = crate::init::<checkout::Checkout>((), &mut sink);
        let mut inspector = TraceInspector::<checkout::Checkout>::default();

        crate::macrostep(&mut snapshot, checkout::Event::Confirm, &mut sink, &mut inspector);
        crate::macrostep(&mut snapshot, checkout::Event::SelectMethod, &mut sink, &mut inspector);
        assert!(snapshot.matches("processing"));

        // Cancelling from `processing` exits `payment` entirely (recording shallow
        // history), landing back in `cart`.
        crate::macrostep(&mut snapshot, checkout::Event::Cancel, &mut sink, &mut inspector);
        assert!(snapshot.matches("cart"));
        assert!(!snapshot.matches("payment"));

        // Resuming must restore `processing`, not `payment`'s default `selecting`.
        crate::macrostep(&mut snapshot, checkout::Event::Resume, &mut sink, &mut inspector);
        assert!(snapshot.matches("processing"), "shallow history must restore into processing, not the default selecting");
        assert!(!snapshot.matches("selecting"));
        assert!(!inspector.entries.is_empty());

        let fired = crate::timer_elapsed(&mut snapshot, TimerId(0), &mut sink, &mut inspector);
        assert_eq!(fired.microsteps, 1);
        assert!(snapshot.matches("failed"));
        assert_eq!(snapshot.context.attempts, 1);

        crate::macrostep(&mut snapshot, checkout::Event::Retry, &mut sink, &mut inspector);
        assert!(snapshot.matches("processing"));

        let persisted = crate::persist(&snapshot);
        assert_eq!(persisted.fingerprint, checkout::Checkout::definition().fingerprint);
        let restored = crate::restore::<checkout::Checkout>(&persisted, snapshot.context.clone(), &[]).expect("restore should succeed");
        assert!(restored.matches("processing"));
    }

    #[test]
    fn dsl_machine_coverage_reaches_every_declared_state() {
        let model = crate::Model::<checkout::Checkout>::new(vec![
            checkout::Event::Confirm,
            checkout::Event::SelectMethod,
            checkout::Event::PaymentSucceeded,
            checkout::Event::PaymentFailed,
            checkout::Event::Retry,
            checkout::Event::Cancel,
            checkout::Event::Resume,
            checkout::Event::ShipDone,
            checkout::Event::InvoiceDone,
        ]);
        let coverage = crate::explore(&model, ());
        for expected in ["cart", "selecting", "processing", "failed", "ship_pending", "ship_done", "invoice_pending", "invoice_done", "done"] {
            assert!(coverage.reached_stable_ids.contains(&expected), "expected model exploration to reach `{expected}`, got {:?}", coverage.reached_stable_ids);
        }
    }
}

//#endregion 🧪Tests
