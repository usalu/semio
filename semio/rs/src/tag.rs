use serde::{Deserialize, Serialize};
use std::sync::{RwLock, Weak};

use crate::design::DesignStoreWeak;
use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
use crate::guid::Guid;
use crate::hash::{Cache, HashWriter};
use crate::kit::KitStoreWeak;
use crate::typ::TypeStoreWeak;

pub type TagStoreRef = std::sync::Arc<RwLock<TagStore>>;
pub type TagStoreWeak = Weak<RwLock<TagStore>>;

/// Freely choosable label used for filtering/grouping in the UI.
#[derive(Debug)]
pub struct TagStore {
    pub guid: Guid,
    pub name: String,
    pub order: Option<i64>,
    pub parent_kit: Option<KitStoreWeak>,
    pub parent_design: Option<DesignStoreWeak>,
    pub parent_type: Option<TypeStoreWeak>,
    pub(crate) event_bus: Weak<EventBus>,
    hash_cache: Cache<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct TagIdDto {
    pub guid: Guid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct TagMetadataDto {
    pub guid: Guid,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct TagShallowDto {
    pub guid: Guid,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct TagFullDto {
    pub guid: Guid,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
}

impl TagStore {
    pub(crate) fn empty_shell(guid: Guid) -> Self {
        Self {
            guid,
            name: String::new(),
            order: None,
            parent_kit: None,
            parent_design: None,
            parent_type: None,
            event_bus: Weak::new(),
            hash_cache: Cache::default(),
        }
    }

    #[inline]
    fn emit_ev(&self, ev: KitEvent) {
        emit_weak(&self.event_bus, ev);
    }

    fn entity_ref(&self) -> EntityRef {
        EntityRef::new(EntityKind::Tag, self.guid.clone())
    }

    pub(crate) fn apply_full_dto_fields(&mut self, d: TagFullDto) {
        self.guid = d.guid;
        self.name = d.name;
        self.order = d.order;
        self.hash_cache.invalidate();
    }

    pub(crate) fn from_shallow_dto(d: TagShallowDto) -> Self {
        let mut s = Self::empty_shell(d.guid.clone());
        s.name = d.name;
        s.order = d.order;
        s.hash_cache.invalidate();
        s
    }

    pub(crate) fn from_full_dto(d: TagFullDto) -> Self {
        let mut s = Self::empty_shell(d.guid.clone());
        s.apply_full_dto_fields(d);
        s
    }

    pub fn set_name(&mut self, name: String) {
        if self.name == name {
            return;
        }
        self.name = name;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "name",
        });
        self.bubble();
    }

    pub fn set_order(&mut self, order: Option<i64>) {
        if self.order == order {
            return;
        }
        self.order = order;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "order",
        });
        self.bubble();
    }

    fn bubble(&mut self) {
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
    }

    pub fn to_id_dto(&self) -> TagIdDto {
        TagIdDto { guid: self.guid.clone() }
    }

    pub fn to_metadata_dto(&self) -> TagMetadataDto {
        TagMetadataDto {
            guid: self.guid.clone(),
            name: self.name.clone(),
            order: self.order,
        }
    }

    pub fn to_shallow_dto(&self) -> TagShallowDto {
        let m = self.to_metadata_dto();
        TagShallowDto {
            guid: m.guid,
            name: m.name,
            order: m.order,
        }
    }

    pub fn to_full_dto(&self) -> TagFullDto {
        let m = self.to_metadata_dto();
        TagFullDto {
            guid: m.guid,
            name: m.name,
            order: m.order,
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
        w.tag("tag").str(self.guid.as_str()).str(&self.name);
        if let Some(o) = self.order {
            w.f64(o as f64);
        }
    }
}
