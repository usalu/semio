use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock, Weak};

use crate::benchmark::{BenchmarkFullDto, BenchmarkMetadataDto, BenchmarkStore, BenchmarkStoreRef};
use crate::connector::ConnectorStoreWeak;
use crate::design::DesignStoreWeak;
use crate::guid::Guid;
use crate::hash::{Cache, HashWriter};
use crate::kit::KitStoreWeak;
use crate::port::PortStoreWeak;
use crate::representation::RepresentationStoreWeak;
use crate::typ::TypeStoreWeak;

pub type QualityStoreRef = Arc<RwLock<QualityStore>>;
pub type QualityStoreWeak = Weak<RwLock<QualityStore>>;

/// Measurable/named quality that can be attached to ports, types, designs, etc.
#[derive(Debug)]
pub struct QualityStore {
    pub guid: Guid,
    pub key: String,
    pub value: Option<String>,
    pub unit: Option<String>,
    pub definition: Option<String>,
    pub description: Option<String>,
    pub benchmarks: Vec<BenchmarkStoreRef>,
    pub parent_kit: Option<KitStoreWeak>,
    pub parent_design: Option<DesignStoreWeak>,
    pub parent_type: Option<TypeStoreWeak>,
    pub parent_port: Option<PortStoreWeak>,
    pub parent_connector: Option<ConnectorStoreWeak>,
    pub parent_representation: Option<RepresentationStoreWeak>,
    hash_cache: Cache<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct QualityIdDto {
    pub guid: Guid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct QualityMetadataDto {
    pub guid: Guid,
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct QualityShallowDto {
    pub guid: Guid,
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub benchmarks: Vec<BenchmarkMetadataDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct QualityFullDto {
    pub guid: Guid,
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub benchmarks: Vec<BenchmarkFullDto>,
}

impl QualityStore {
    pub(crate) fn empty_shell(guid: Guid) -> Self {
        Self {
            guid,
            key: String::new(),
            value: None,
            unit: None,
            definition: None,
            description: None,
            benchmarks: Vec::new(),
            parent_kit: None,
            parent_design: None,
            parent_type: None,
            parent_port: None,
            parent_connector: None,
            parent_representation: None,
            hash_cache: Cache::default(),
        }
    }

    pub(crate) fn apply_metadata_fields(&mut self, d: QualityMetadataDto) {
        self.guid = d.guid;
        self.key = d.key;
        self.value = d.value;
        self.unit = d.unit;
        self.definition = d.definition;
        self.description = d.description;
        self.hash_cache.invalidate();
    }

    pub fn set_key(&mut self, key: String) {
        self.key = key;
        self.bubble();
    }

    pub fn set_value(&mut self, value: Option<String>) {
        self.value = value;
        self.bubble();
    }

    pub fn set_unit(&mut self, unit: Option<String>) {
        self.unit = unit;
        self.bubble();
    }

    pub fn set_definition(&mut self, definition: Option<String>) {
        self.definition = definition;
        self.bubble();
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
        self.bubble();
    }

    fn bubble(&mut self) {
        self.hash_cache.invalidate();
        if let Some(w) = &self.parent_kit {
            if let Some(k) = w.upgrade() {
                if let Ok(k) = k.read() {
                    k.invalidate_hash();
                    k.invalidate_validation();
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
        }
        if let Some(w) = &self.parent_type {
            if let Some(t) = w.upgrade() {
                if let Ok(t) = t.read() {
                    t.invalidate_hash();
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
        if let Some(w) = &self.parent_connector {
            if let Some(c) = w.upgrade() {
                if let Ok(c) = c.read() {
                    c.invalidate_hash();
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
    }

    pub(crate) fn from_shallow_dto(d: QualityShallowDto) -> Self {
        let mut s = Self::empty_shell(d.guid.clone());
        s.apply_metadata_fields(QualityMetadataDto {
            guid: d.guid,
            key: d.key,
            value: d.value,
            unit: d.unit,
            definition: d.definition,
            description: d.description,
        });
        s.benchmarks = d
            .benchmarks
            .into_iter()
            .map(|b| {
                let mut bs = BenchmarkStore::empty_shell(b.guid.clone());
                bs.apply_metadata_dto(b);
                Arc::new(RwLock::new(bs))
            })
            .collect();
        s
    }

    pub(crate) fn from_full_dto(d: QualityFullDto) -> Self {
        let QualityFullDto {
            guid,
            key,
            value,
            unit,
            definition,
            description,
            benchmarks,
        } = d;
        let mut s = Self::empty_shell(guid.clone());
        s.apply_metadata_fields(QualityMetadataDto {
            guid,
            key,
            value,
            unit,
            definition,
            description,
        });
        s.benchmarks = benchmarks
            .into_iter()
            .map(|b| {
                let mut bs = BenchmarkStore::empty_shell(b.guid.clone());
                bs.apply_metadata_dto(BenchmarkMetadataDto {
                    guid: b.guid,
                    name: b.name,
                    min: b.min,
                    max: b.max,
                    min_excluded: b.min_excluded,
                    max_excluded: b.max_excluded,
                });
                Arc::new(RwLock::new(bs))
            })
            .collect();
        s
    }

    pub fn to_id_dto(&self) -> QualityIdDto {
        QualityIdDto { guid: self.guid.clone() }
    }

    pub fn to_metadata_dto(&self) -> QualityMetadataDto {
        QualityMetadataDto {
            guid: self.guid.clone(),
            key: self.key.clone(),
            value: self.value.clone(),
            unit: self.unit.clone(),
            definition: self.definition.clone(),
            description: self.description.clone(),
        }
    }

    pub fn to_shallow_dto(&self) -> QualityShallowDto {
        let m = self.to_metadata_dto();
        QualityShallowDto {
            guid: m.guid,
            key: m.key,
            value: m.value,
            unit: m.unit,
            definition: m.definition,
            description: m.description,
            benchmarks: self
                .benchmarks
                .iter()
                .filter_map(|b| b.read().ok().map(|b| b.to_metadata_dto()))
                .collect(),
        }
    }

    pub fn to_full_dto(&self) -> QualityFullDto {
        let m = self.to_metadata_dto();
        QualityFullDto {
            guid: m.guid,
            key: m.key,
            value: m.value,
            unit: m.unit,
            definition: m.definition,
            description: m.description,
            benchmarks: self
                .benchmarks
                .iter()
                .filter_map(|b| b.read().ok().map(|b| b.to_full_dto()))
                .collect(),
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
        w.tag("quality")
            .str(self.guid.as_str())
            .str(&self.key)
            .opt_str(self.value.as_deref())
            .opt_str(self.unit.as_deref())
            .opt_str(self.definition.as_deref());
        for b in &self.benchmarks {
            if let Ok(b) = b.read() {
                b.hash_into(w);
            }
        }
    }
}
