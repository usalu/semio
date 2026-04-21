use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock, RwLock, Weak};

use crate::guid::Guid;
use crate::hash::HashWriter;
use crate::piece::PieceWeak;

pub type GroupRef = Arc<RwLock<Group>>;
pub type GroupWeak = Weak<RwLock<Group>>;

/// User-defined group of pieces inside a [`crate::design::Design`].
#[derive(Debug)]
pub struct Group {
    pub guid: Guid,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub pieces: Vec<PieceWeak>,
    pub parent_design: Weak<RwLock<crate::design::Design>>,
    hash_cache: OnceLock<String>,
}

impl Group {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            guid: Guid::new_v7(),
            name: name.into(),
            description: None,
            color: None,
            icon: None,
            pieces: Vec::new(),
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
        w.tag("group")
            .str(self.guid.as_str())
            .str(&self.name)
            .opt_str(self.description.as_deref())
            .opt_str(self.color.as_deref())
            .opt_str(self.icon.as_deref());
        for p in &self.pieces {
            if let Some(p) = p.upgrade() {
                if let Ok(p) = p.read() {
                    w.str(p.guid.as_str());
                }
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct GroupDto {
    #[serde(default)]
    pub guid: Option<Guid>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "pieceGuids")]
    pub piece_guids: Vec<Guid>,
}

impl From<&Group> for GroupDto {
    fn from(g: &Group) -> Self {
        GroupDto {
            guid: Some(g.guid.clone()),
            name: g.name.clone(),
            description: g.description.clone(),
            color: g.color.clone(),
            icon: g.icon.clone(),
            piece_guids: g
                .pieces
                .iter()
                .filter_map(|p| p.upgrade())
                .filter_map(|p| p.read().ok().map(|p| p.guid.clone()))
                .collect(),
        }
    }
}

impl Group {
    pub fn from_dto(d: GroupDto) -> Self {
        Self {
            guid: d.guid.unwrap_or_else(Guid::new_v7),
            name: d.name,
            description: d.description,
            color: d.color,
            icon: d.icon,
            pieces: Vec::new(),
            parent_design: Weak::new(),
            hash_cache: OnceLock::new(),
        }
    }
}
