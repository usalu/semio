use serde::{Deserialize, Serialize};
use std::sync::{RwLock, Weak};

use crate::design::DesignStoreWeak;
use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
use crate::guid::Guid;
use crate::hash::{Cache, HashWriter};
use crate::kit::KitStoreWeak;
use crate::typ::TypeStoreWeak;

use crate::connection::ConnectionStoreWeak;
use crate::connector::ConnectorStoreWeak;
use crate::piece::PieceStoreWeak;
use crate::port::PortStoreWeak;
use crate::representation::RepresentationStoreWeak;

pub type AttributeStoreRef = std::sync::Arc<RwLock<AttributeStore>>;
pub type AttributeStoreWeak = Weak<RwLock<AttributeStore>>;

/// A name/value pair attached to pretty much any domain entity.
#[derive(Debug)]
pub struct AttributeStore {
    pub guid: Guid,
    pub key: String,
    pub value: String,
    pub definition: Option<String>,
    pub parent_kit: Option<KitStoreWeak>,
    pub parent_design: Option<DesignStoreWeak>,
    pub parent_type: Option<TypeStoreWeak>,
    pub parent_piece: Option<PieceStoreWeak>,
    pub parent_port: Option<PortStoreWeak>,
    pub parent_connection: Option<ConnectionStoreWeak>,
    pub parent_representation: Option<RepresentationStoreWeak>,
    pub parent_connector: Option<ConnectorStoreWeak>,
    pub(crate) event_bus: Weak<EventBus>,
    hash_cache: Cache<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct AttributeIdDto {
    pub guid: Guid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct AttributeMetadataDto {
    pub guid: Guid,
    pub key: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct AttributeShallowDto {
    pub guid: Guid,
    pub key: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct AttributeFullDto {
    pub guid: Guid,
    pub key: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}

impl AttributeStore {
    pub(crate) fn empty_shell(guid: Guid) -> Self {
        Self {
            guid,
            key: String::new(),
            value: String::new(),
            definition: None,
            parent_kit: None,
            parent_design: None,
            parent_type: None,
            parent_piece: None,
            parent_port: None,
            parent_connection: None,
            parent_representation: None,
            parent_connector: None,
            event_bus: Weak::new(),
            hash_cache: Cache::default(),
        }
    }

    #[inline]
    fn emit_ev(&self, ev: KitEvent) {
        emit_weak(&self.event_bus, ev);
    }

    fn entity_ref(&self) -> EntityRef {
        EntityRef::new(EntityKind::Attribute, self.guid.clone())
    }

    pub(crate) fn apply_full_dto_fields(&mut self, d: AttributeFullDto) {
        self.guid = d.guid;
        self.key = d.key;
        self.value = d.value;
        self.definition = d.definition;
        self.hash_cache.invalidate();
    }

    pub(crate) fn from_shallow_dto(d: AttributeShallowDto) -> Self {
        let mut s = Self::empty_shell(d.guid.clone());
        s.key = d.key;
        s.value = d.value;
        s.definition = d.definition;
        s.hash_cache.invalidate();
        s
    }

    pub(crate) fn from_full_dto(d: AttributeFullDto) -> Self {
        let mut s = Self::empty_shell(d.guid.clone());
        s.apply_full_dto_fields(d);
        s
    }

    pub fn set_key(&mut self, key: String) {
        if self.key == key {
            return;
        }
        self.key = key;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "key",
        });
        self.invalidate_local_and_bubble();
    }

    pub fn set_value(&mut self, value: String) {
        if self.value == value {
            return;
        }
        self.value = value;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "value",
        });
        self.invalidate_local_and_bubble();
    }

    pub fn set_definition(&mut self, definition: Option<String>) {
        if self.definition == definition {
            return;
        }
        self.definition = definition;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "definition",
        });
        self.invalidate_local_and_bubble();
    }

    fn invalidate_local_and_bubble(&mut self) {
        self.hash_cache.invalidate();
        self.emit_ev(KitEvent::HashInvalidated {
            entity: self.entity_ref(),
        });
        if let Some(w) = &self.parent_kit {
            if let Some(k) = w.upgrade() {
                if let Ok(k) = k.read() {
                    k.invalidate_hash();
                }
            }
        }
        if let Some(w) = &self.parent_design {
            if let Some(d) = w.upgrade() {
                if let Ok(d) = d.read() {
                    d.invalidate_hash();
                    d.invalidate_flatten();
                    d.invalidate_validation();
                }
            }
        } else if let Some(w) = &self.parent_kit {
            if let Some(k) = w.upgrade() {
                if let Ok(k) = k.read() {
                    k.invalidate_validation();
                }
            }
        }
        if let Some(w) = &self.parent_type {
            if let Some(t) = w.upgrade() {
                if let Ok(t) = t.read() {
                    t.invalidate_hash();
                }
            }
        }
        if let Some(w) = &self.parent_piece {
            if let Some(p) = w.upgrade() {
                if let Ok(p) = p.read() {
                    p.invalidate_hash();
                }
            }
        }
        if let Some(w) = &self.parent_port {
            if let Some(p) = w.upgrade() {
                if let Ok(p) = p.read() {
                    p.invalidate_hash();
                }
            }
        }
        if let Some(w) = &self.parent_connection {
            if let Some(c) = w.upgrade() {
                if let Ok(c) = c.read() {
                    c.notify_aggregate_change();
                }
            }
        }
        if let Some(w) = &self.parent_representation {
            if let Some(r) = w.upgrade() {
                if let Ok(r) = r.read() {
                    r.invalidate_hash();
                }
            }
        }
        if let Some(w) = &self.parent_connector {
            if let Some(c) = w.upgrade() {
                if let Ok(c) = c.read() {
                    c.invalidate_hash();
                }
            }
        }
    }

    pub fn to_id_dto(&self) -> AttributeIdDto {
        AttributeIdDto { guid: self.guid.clone() }
    }

    pub fn to_metadata_dto(&self) -> AttributeMetadataDto {
        AttributeMetadataDto {
            guid: self.guid.clone(),
            key: self.key.clone(),
            value: self.value.clone(),
            definition: self.definition.clone(),
        }
    }

    pub fn to_shallow_dto(&self) -> AttributeShallowDto {
        let m = self.to_metadata_dto();
        AttributeShallowDto {
            guid: m.guid,
            key: m.key,
            value: m.value,
            definition: m.definition,
        }
    }

    pub fn to_full_dto(&self) -> AttributeFullDto {
        let m = self.to_metadata_dto();
        AttributeFullDto {
            guid: m.guid,
            key: m.key,
            value: m.value,
            definition: m.definition,
        }
    }

    pub fn invalidate_hash(&self) {
        self.hash_cache.invalidate();
    }

    pub fn hash(&self) -> String {
        self.hash_cache.get_or_init(|| {
            let mut w = HashWriter::new();
            self.hash_into(&mut w);
            w.finalize()
        })
    }

    pub fn hash_into(&self, w: &mut HashWriter) {
        w.tag("attr")
            .str(self.guid.as_str())
            .str(&self.key)
            .str(&self.value)
            .opt_str(self.definition.as_deref());
    }
}
