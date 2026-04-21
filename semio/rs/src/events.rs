//! Kit-scoped async broadcast bus (no tokio). WASM-safe.

use std::sync::{Arc, Weak};

use async_broadcast::{broadcast, Receiver, Sender};

use crate::guid::Guid;

/// Entity discriminator for [`KitEvent`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum EntityKind {
    Kit,
    Type,
    Design,
    Piece,
    Connection,
    Side,
    Port,
    Connector,
    Representation,
    File,
    Folder,
    Layer,
    Group,
    Author,
    Concept,
    Tag,
    Prop,
    Attribute,
    Quality,
    Stat,
    Benchmark,
}

/// Stable identity for event payloads.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct EntityRef {
    pub kind: EntityKind,
    pub guid: Guid,
}

impl EntityRef {
    pub const fn new(kind: EntityKind, guid: Guid) -> Self {
        Self { kind, guid }
    }
}

/// All observable changes on a kit graph.
#[derive(Clone, Debug, PartialEq)]
pub enum KitEvent {
    FieldChanged {
        entity: EntityRef,
        field: &'static str,
    },
    ChildAdded {
        parent: EntityRef,
        child: EntityRef,
    },
    ChildRemoved {
        parent: EntityRef,
        child: EntityRef,
    },
    HashInvalidated {
        entity: EntityRef,
    },
    FlattenInvalidated {
        design: Guid,
        pieces: Vec<Guid>,
    },
    ValidationInvalidated,
    DerivedChanged {
        entity: EntityRef,
        field: &'static str,
    },
}

/// Broadcast channel wrapper; cloneable [`Sender`] shares one channel.
#[derive(Debug)]
pub struct EventBus {
    sender: Sender<KitEvent>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Arc<Self> {
        let (mut sender, _rx) = broadcast(capacity);
        sender.set_overflow(true);
        Arc::new(Self { sender })
    }

    pub fn subscribe(&self) -> Receiver<KitEvent> {
        self.sender.new_receiver()
    }

    pub fn emit(&self, event: KitEvent) {
        let _ = self.sender.try_broadcast(event);
    }
}

#[inline]
pub fn emit_weak(bus: &Weak<EventBus>, event: KitEvent) {
    if let Some(b) = bus.upgrade() {
        b.emit(event);
    }
}
