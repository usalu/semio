use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock, Weak};

use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
use crate::guid::Guid;
use crate::hash::{Cache, HashWriter};

pub type LayerStoreRef = Arc<RwLock<LayerStore>>;
pub type LayerStoreWeak = Weak<RwLock<LayerStore>>;

/// Visual layer inside a [`crate::design::DesignStore`].
#[derive(Debug)]
pub struct LayerStore {
    pub guid: Guid,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub order: Option<i64>,
    pub visible: Option<bool>,
    pub locked: Option<bool>,
    pub parent_design: Weak<RwLock<crate::design::DesignStore>>,
    pub(crate) event_bus: Weak<EventBus>,
    hash_cache: Cache<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct LayerIdDto {
    pub guid: Guid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct LayerMetadataDto {
    pub guid: Guid,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct LayerShallowDto {
    pub guid: Guid,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct LayerFullDto {
    pub guid: Guid,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

impl LayerStore {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            guid: Guid::new_v7(),
            name: name.into(),
            description: None,
            color: None,
            order: None,
            visible: None,
            locked: None,
            parent_design: Weak::new(),
            event_bus: Weak::new(),
            hash_cache: Cache::default(),
        }
    }

    pub fn from_id_dto(d: LayerIdDto) -> Self {
        Self {
            guid: d.guid,
            name: String::new(),
            description: None,
            color: None,
            order: None,
            visible: None,
            locked: None,
            parent_design: Weak::new(),
            event_bus: Weak::new(),
            hash_cache: Cache::default(),
        }
    }

    pub fn from_metadata_dto(d: LayerMetadataDto) -> Self {
        Self {
            guid: d.guid,
            name: d.name,
            description: d.description,
            color: d.color,
            order: d.order,
            visible: d.visible,
            locked: d.locked,
            parent_design: Weak::new(),
            event_bus: Weak::new(),
            hash_cache: Cache::default(),
        }
    }

    pub fn from_shallow_dto(d: LayerShallowDto) -> Self {
        Self::from_metadata_dto(LayerMetadataDto {
            guid: d.guid,
            name: d.name,
            description: d.description,
            color: d.color,
            order: d.order,
            visible: d.visible,
            locked: d.locked,
        })
    }

    pub fn from_full_dto(d: LayerFullDto) -> Self {
        Self::from_metadata_dto(LayerMetadataDto {
            guid: d.guid,
            name: d.name,
            description: d.description,
            color: d.color,
            order: d.order,
            visible: d.visible,
            locked: d.locked,
        })
    }

    pub fn to_id_dto(&self) -> LayerIdDto {
        LayerIdDto { guid: self.guid.clone() }
    }

    pub fn to_metadata_dto(&self) -> LayerMetadataDto {
        LayerMetadataDto {
            guid: self.guid.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            color: self.color.clone(),
            order: self.order,
            visible: self.visible,
            locked: self.locked,
        }
    }

    pub fn to_shallow_dto(&self) -> LayerShallowDto {
        let m = self.to_metadata_dto();
        LayerShallowDto {
            guid: m.guid,
            name: m.name,
            description: m.description,
            color: m.color,
            order: m.order,
            visible: m.visible,
            locked: m.locked,
        }
    }

    pub fn to_full_dto(&self) -> LayerFullDto {
        let m = self.to_metadata_dto();
        LayerFullDto {
            guid: m.guid,
            name: m.name,
            description: m.description,
            color: m.color,
            order: m.order,
            visible: m.visible,
            locked: m.locked,
        }
    }

    #[inline]
    fn emit_ev(&self, ev: KitEvent) {
        emit_weak(&self.event_bus, ev);
    }

    fn entity_ref(&self) -> EntityRef {
        EntityRef::new(EntityKind::Layer, self.guid.clone())
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
        self.invalidate_hash();
    }

    pub fn set_description(&mut self, v: Option<String>) {
        if self.description == v {
            return;
        }
        self.description = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "description",
        });
        self.invalidate_hash();
    }

    pub fn set_color(&mut self, v: Option<String>) {
        if self.color == v {
            return;
        }
        self.color = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "color",
        });
        self.invalidate_hash();
    }

    pub fn set_order(&mut self, v: Option<i64>) {
        if self.order == v {
            return;
        }
        self.order = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "order",
        });
        self.invalidate_hash();
    }

    pub fn set_visible(&mut self, v: Option<bool>) {
        if self.visible == v {
            return;
        }
        self.visible = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "visible",
        });
        self.invalidate_hash();
    }

    pub fn set_locked(&mut self, v: Option<bool>) {
        if self.locked == v {
            return;
        }
        self.locked = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "locked",
        });
        self.invalidate_hash();
    }

    pub fn invalidate_hash(&self) {
        self.hash_cache.invalidate();
        self.emit_ev(KitEvent::HashInvalidated {
            entity: self.entity_ref(),
        });
        if let Some(d) = self.parent_design.upgrade() {
            if let Ok(dr) = d.read() {
                dr.invalidate_hash();
            }
        }
    }

    pub fn hash(&self) -> String {
        self.hash_cache
            .get_or_init(|| {
                let mut w = HashWriter::new();
                self.hash_into(&mut w);
                w.finalize()
            })
            .clone()
    }

    pub fn hash_into(&self, w: &mut HashWriter) {
        w.tag("layer")
            .str(self.guid.as_str())
            .str(&self.name)
            .opt_str(self.description.as_deref())
            .opt_str(self.color.as_deref());
        if let Some(o) = self.order {
            w.f64(o as f64);
        }
        w.opt_bool(self.visible).opt_bool(self.locked);
    }
}
