use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

use crate::guid::Guid;
use crate::hash::HashWriter;

/// Numeric range benchmark used to qualify quality measurements.
#[derive(Debug)]
pub struct BenchmarkStore {
    pub guid: Guid,
    pub name: String,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub min_excluded: Option<bool>,
    pub max_excluded: Option<bool>,
    hash_cache: OnceLock<String>,
}

pub type BenchmarkStoreRef = std::sync::Arc<std::sync::RwLock<BenchmarkStore>>;
pub type BenchmarkStoreWeak = std::sync::Weak<std::sync::RwLock<BenchmarkStore>>;

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
    #[serde(flatten)]
    pub meta: BenchmarkMetadataDto,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct BenchmarkFullDto {
    #[serde(flatten)]
    pub meta: BenchmarkMetadataDto,
}

impl BenchmarkStore {
    pub fn from_id_dto(d: BenchmarkIdDto) -> Self {
        Self {
            guid: d.guid,
            name: String::new(),
            min: None,
            max: None,
            min_excluded: None,
            max_excluded: None,
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_metadata_dto(d: BenchmarkMetadataDto) -> Self {
        Self {
            guid: d.guid,
            name: d.name,
            min: d.min,
            max: d.max,
            min_excluded: d.min_excluded,
            max_excluded: d.max_excluded,
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_shallow_dto(d: BenchmarkShallowDto) -> Self {
        Self::from_metadata_dto(d.meta)
    }

    pub fn from_full_dto(d: BenchmarkFullDto) -> Self {
        Self::from_metadata_dto(d.meta)
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
        BenchmarkShallowDto { meta: self.to_metadata_dto() }
    }

    pub fn to_full_dto(&self) -> BenchmarkFullDto {
        BenchmarkFullDto { meta: self.to_metadata_dto() }
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
        w.tag("benchmark")
            .str(self.guid.as_str())
            .str(&self.name)
            .opt_f64(self.min)
            .opt_f64(self.max)
            .opt_bool(self.min_excluded)
            .opt_bool(self.max_excluded);
    }
}
