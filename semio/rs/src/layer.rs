use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock, RwLock, Weak};

use crate::guid::Guid;
use crate::hash::HashWriter;

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
    hash_cache: OnceLock<String>,
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
    #[serde(flatten)]
    pub meta: LayerMetadataDto,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct LayerFullDto {
    #[serde(flatten)]
    pub meta: LayerMetadataDto,
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
            hash_cache: OnceLock::new(),
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
            hash_cache: OnceLock::new(),
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
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_shallow_dto(d: LayerShallowDto) -> Self {
        Self::from_metadata_dto(d.meta)
    }

    pub fn from_full_dto(d: LayerFullDto) -> Self {
        Self::from_metadata_dto(d.meta)
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
        LayerShallowDto { meta: self.to_metadata_dto() }
    }

    pub fn to_full_dto(&self) -> LayerFullDto {
        LayerFullDto { meta: self.to_metadata_dto() }
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
