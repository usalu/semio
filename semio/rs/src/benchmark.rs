use serde::{Deserialize, Serialize};
use std::sync::{RwLock, Weak};

use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
use crate::guid::Guid;
use crate::hash::{Cache, HashWriter};
use crate::quality::QualityStoreWeak;

pub type BenchmarkStoreRef = std::sync::Arc<RwLock<BenchmarkStore>>;
pub type BenchmarkStoreWeak = Weak<RwLock<BenchmarkStore>>;

/// Numeric range benchmark used to qualify quality measurements.
#[derive(Debug)]
pub struct BenchmarkStore {
    pub guid: Guid,
    pub name: String,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub min_excluded: Option<bool>,
    pub max_excluded: Option<bool>,
    pub parent_quality: Option<QualityStoreWeak>,
    pub(crate) event_bus: Weak<EventBus>,
    hash_cache: Cache<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct BenchmarkIdDto {
    pub guid: Guid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct BenchmarkMetadataDto {
    pub guid: Guid,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minExcluded")]
    pub min_excluded: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxExcluded")]
    pub max_excluded: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct BenchmarkShallowDto {
    pub guid: Guid,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minExcluded")]
    pub min_excluded: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxExcluded")]
    pub max_excluded: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct BenchmarkFullDto {
    pub guid: Guid,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minExcluded")]
    pub min_excluded: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxExcluded")]
    pub max_excluded: Option<bool>,
}

impl BenchmarkStore {
    pub(crate) fn empty_shell(guid: Guid) -> Self {
        Self {
            guid,
            name: String::new(),
            min: None,
            max: None,
            min_excluded: None,
            max_excluded: None,
            parent_quality: None,
            event_bus: Weak::new(),
            hash_cache: Cache::default(),
        }
    }

    #[inline]
    fn emit_ev(&self, ev: KitEvent) {
        emit_weak(&self.event_bus, ev);
    }

    fn entity_ref(&self) -> EntityRef {
        EntityRef::new(EntityKind::Benchmark, self.guid.clone())
    }

    pub(crate) fn apply_metadata_dto(&mut self, d: BenchmarkMetadataDto) {
        self.guid = d.guid;
        self.name = d.name;
        self.min = d.min;
        self.max = d.max;
        self.min_excluded = d.min_excluded;
        self.max_excluded = d.max_excluded;
        self.hash_cache.invalidate();
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

    pub fn set_min(&mut self, min: Option<f64>) {
        if self.min == min {
            return;
        }
        self.min = min;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "min",
        });
        self.bubble();
    }

    pub fn set_max(&mut self, max: Option<f64>) {
        if self.max == max {
            return;
        }
        self.max = max;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "max",
        });
        self.bubble();
    }

    pub fn set_min_excluded(&mut self, v: Option<bool>) {
        if self.min_excluded == v {
            return;
        }
        self.min_excluded = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "minExcluded",
        });
        self.bubble();
    }

    pub fn set_max_excluded(&mut self, v: Option<bool>) {
        if self.max_excluded == v {
            return;
        }
        self.max_excluded = v;
        self.emit_ev(KitEvent::FieldChanged {
            entity: self.entity_ref(),
            field: "maxExcluded",
        });
        self.bubble();
    }

    fn bubble(&mut self) {
        self.hash_cache.invalidate();
        self.emit_ev(KitEvent::HashInvalidated {
            entity: self.entity_ref(),
        });
        if let Some(w) = &self.parent_quality {
            if let Some(q) = w.upgrade() {
                if let Ok(q) = q.read() {
                    q.invalidate_hash();
                }
            }
        }
    }

    pub fn to_id_dto(&self) -> BenchmarkIdDto {
        BenchmarkIdDto { guid: self.guid.clone() }
    }

    pub fn to_metadata_dto(&self) -> BenchmarkMetadataDto {
        BenchmarkMetadataDto {
            guid: self.guid.clone(),
            name: self.name.clone(),
            min: self.min,
            max: self.max,
            min_excluded: self.min_excluded,
            max_excluded: self.max_excluded,
        }
    }

    pub fn to_shallow_dto(&self) -> BenchmarkShallowDto {
        let m = self.to_metadata_dto();
        BenchmarkShallowDto {
            guid: m.guid,
            name: m.name,
            min: m.min,
            max: m.max,
            min_excluded: m.min_excluded,
            max_excluded: m.max_excluded,
        }
    }

    pub fn to_full_dto(&self) -> BenchmarkFullDto {
        let m = self.to_metadata_dto();
        BenchmarkFullDto {
            guid: m.guid,
            name: m.name,
            min: m.min,
            max: m.max,
            min_excluded: m.min_excluded,
            max_excluded: m.max_excluded,
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
        w.tag("benchmark")
            .str(self.guid.as_str())
            .str(&self.name)
            .opt_f64(self.min)
            .opt_f64(self.max)
            .opt_bool(self.min_excluded)
            .opt_bool(self.max_excluded);
    }
}
