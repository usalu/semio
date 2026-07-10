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

//#endregion 🔖Schema

//#region 🔖Reexports

pub use host::{Host, NativeHost, TestHost};
pub use inspect::{InspectionEvent, Inspector, MicrostepTrace, NullInspector, TraceInspector};
pub use kernel::{
    init, macrostep, Command, CommandSink, MachineDefinition, NodeDef, NodeKind, Snapshot, Status,
    StepReport, TransitionDef, TransitionKind,
};
pub use persist::{Migration, PersistedSnapshot, RestoreError};
pub use runtime::{ActorLogic, ActorSystem, MachineLogic};

#[cfg(feature = "macros")]
pub use fsm_macros::{statechart, StatechartEvent, StatechartSchema};

//#endregion 🔖Reexports

//#region 🔖WasmBridge

#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    // Populated once a concrete consumer machine needs `export_wasm_machine!` support;
    // machine-agnostic manifest/persistence JSON helpers live here.
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

//#endregion 🧪Tests
