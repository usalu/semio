use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock, RwLock, Weak};

use crate::guid::Guid;
use crate::hash::HashWriter;

pub type LayerRef = Arc<RwLock<Layer>>;
pub type LayerWeak = Weak<RwLock<Layer>>;

/// Visual layer inside a [`crate::design::Design`].
#[derive(Debug)]
pub struct Layer {
    pub guid: Guid,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub order: Option<i64>,
    pub visible: Option<bool>,
    pub locked: Option<bool>,
    pub parent_design: Weak<RwLock<crate::design::Design>>,
    hash_cache: OnceLock<String>,
}

impl Layer {
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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct LayerDto {
    #[serde(default)]
    pub guid: Option<Guid>,
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

impl From<&Layer> for LayerDto {
    fn from(l: &Layer) -> Self {
        LayerDto {
            guid: Some(l.guid.clone()),
            name: l.name.clone(),
            description: l.description.clone(),
            color: l.color.clone(),
            order: l.order,
            visible: l.visible,
            locked: l.locked,
        }
    }
}

impl From<LayerDto> for Layer {
    fn from(d: LayerDto) -> Self {
        Self {
            guid: d.guid.unwrap_or_else(Guid::new_v7),
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
}
