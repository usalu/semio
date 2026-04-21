use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock, RwLock, Weak};

use crate::benchmark::Benchmark;
use crate::guid::Guid;
use crate::hash::HashWriter;

pub type QualityRef = Arc<RwLock<Quality>>;
pub type QualityWeak = Weak<RwLock<Quality>>;

/// Measurable/named quality that can be attached to ports, types, designs, etc.
#[derive(Debug)]
pub struct Quality {
    pub guid: Guid,
    pub key: String,
    pub value: Option<String>,
    pub unit: Option<String>,
    pub definition: Option<String>,
    pub description: Option<String>,
    pub benchmarks: Vec<Benchmark>,
    hash_cache: OnceLock<String>,
}

impl Quality {
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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct QualityDto {
    #[serde(default)]
    pub guid: Option<Guid>,
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
    pub benchmarks: Vec<Benchmark>,
}

impl From<&Quality> for QualityDto {
    fn from(q: &Quality) -> Self {
        QualityDto {
            guid: Some(q.guid.clone()),
            key: q.key.clone(),
            value: q.value.clone(),
            unit: q.unit.clone(),
            definition: q.definition.clone(),
            description: q.description.clone(),
            benchmarks: q.benchmarks.clone(),
        }
    }
}

impl From<QualityDto> for Quality {
    fn from(d: QualityDto) -> Self {
        Self {
            guid: d.guid.unwrap_or_else(Guid::new_v7),
            key: d.key,
            value: d.value,
            unit: d.unit,
            definition: d.definition,
            description: d.description,
            benchmarks: d.benchmarks,
            hash_cache: OnceLock::new(),
        }
    }
}
