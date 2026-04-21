use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock, RwLock, Weak};

use crate::benchmark::{BenchmarkFullDto, BenchmarkMetadataDto, BenchmarkStore};
use crate::guid::Guid;
use crate::hash::HashWriter;

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
    pub benchmarks: Vec<BenchmarkStore>,
    hash_cache: OnceLock<String>,
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
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            guid: Guid::new_v7(),
            key: key.into(),
            value: None,
            unit: None,
            definition: None,
            description: None,
            benchmarks: Vec::new(),
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_id_dto(d: QualityIdDto) -> Self {
        Self {
            guid: d.guid,
            key: String::new(),
            value: None,
            unit: None,
            definition: None,
            description: None,
            benchmarks: Vec::new(),
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_metadata_dto(d: QualityMetadataDto) -> Self {
        Self {
            guid: d.guid,
            key: d.key,
            value: d.value,
            unit: d.unit,
            definition: d.definition,
            description: d.description,
            benchmarks: Vec::new(),
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_shallow_dto(d: QualityShallowDto) -> Self {
        let mut s = Self::from_metadata_dto(QualityMetadataDto {
            guid: d.guid,
            key: d.key,
            value: d.value,
            unit: d.unit,
            definition: d.definition,
            description: d.description,
        });
        s.benchmarks = d.benchmarks.into_iter().map(BenchmarkStore::from_metadata_dto).collect();
        s
    }

    pub fn from_full_dto(d: QualityFullDto) -> Self {
        let mut s = Self::from_metadata_dto(QualityMetadataDto {
            guid: d.guid,
            key: d.key,
            value: d.value,
            unit: d.unit,
            definition: d.definition,
            description: d.description,
        });
        s.benchmarks = d.benchmarks.into_iter().map(BenchmarkStore::from_full_dto).collect();
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
            benchmarks: self.benchmarks.iter().map(BenchmarkStore::to_metadata_dto).collect(),
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
            benchmarks: self.benchmarks.iter().map(BenchmarkStore::to_full_dto).collect(),
        }
    }

    pub fn invalidate_hash(&mut self) {
        self.hash_cache = OnceLock::new();
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
        w.tag("quality")
            .str(self.guid.as_str())
            .str(&self.key)
            .opt_str(self.value.as_deref())
            .opt_str(self.unit.as_deref())
            .opt_str(self.definition.as_deref());
        for b in &self.benchmarks {
            b.hash_into(w);
        }
    }
}
