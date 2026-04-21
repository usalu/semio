use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock, Weak};

use crate::guid::Guid;
use crate::hash::HashWriter;

pub type FolderRef = Arc<RwLock<Folder>>;
pub type FolderWeak = Weak<RwLock<Folder>>;

/// Logical folder grouping files inside a kit.
#[derive(Debug)]
pub struct Folder {
    pub guid: Guid,
    pub path: String,
    pub description: Option<String>,
}

impl Folder {
    pub fn new(path: impl Into<String>) -> Self {
        Self { guid: Guid::new_v7(), path: path.into(), description: None }
    }

    pub fn hash_into(&self, w: &mut HashWriter) {
        w.tag("folder").str(self.guid.as_str()).str(&self.path).opt_str(self.description.as_deref());
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FolderDto {
    #[serde(default)]
    pub guid: Option<Guid>,
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl From<&Folder> for FolderDto {
    fn from(f: &Folder) -> Self {
        FolderDto { guid: Some(f.guid.clone()), path: f.path.clone(), description: f.description.clone() }
    }
}

impl From<FolderDto> for Folder {
    fn from(d: FolderDto) -> Self {
        Self {
            guid: d.guid.unwrap_or_else(Guid::new_v7),
            path: d.path,
            description: d.description,
        }
    }
}
