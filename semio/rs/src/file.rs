use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock, Weak};

use crate::guid::Guid;
use crate::hash::HashWriter;

pub type FileRef = Arc<RwLock<File>>;
pub type FileWeak = Weak<RwLock<File>>;

/// External resource referenced by a kit (3D model, texture, etc.).
#[derive(Debug)]
pub struct File {
    pub guid: Guid,
    pub url: String,
    pub mime: Option<String>,
    pub size: Option<i64>,
    pub hash: Option<String>,
    pub description: Option<String>,
    pub created: Option<String>,
    pub updated: Option<String>,
}

impl File {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            guid: Guid::new_v7(),
            url: url.into(),
            mime: None,
            size: None,
            hash: None,
            description: None,
            created: None,
            updated: None,
        }
    }

    pub fn hash_into(&self, w: &mut HashWriter) {
        w.tag("file")
            .str(self.guid.as_str())
            .str(&self.url)
            .opt_str(self.mime.as_deref())
            .opt_str(self.hash.as_deref());
        if let Some(s) = self.size {
            w.f64(s as f64);
        }
    }
}

/// Wire format for [`File`].
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FileDto {
    #[serde(default)]
    pub guid: Option<Guid>,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

impl From<&File> for FileDto {
    fn from(f: &File) -> Self {
        FileDto {
            guid: Some(f.guid.clone()),
            url: f.url.clone(),
            mime: f.mime.clone(),
            size: f.size,
            hash: f.hash.clone(),
            description: f.description.clone(),
            created: f.created.clone(),
            updated: f.updated.clone(),
        }
    }
}

impl From<FileDto> for File {
    fn from(d: FileDto) -> Self {
        Self {
            guid: d.guid.unwrap_or_else(Guid::new_v7),
            url: d.url,
            mime: d.mime,
            size: d.size,
            hash: d.hash,
            description: d.description,
            created: d.created,
            updated: d.updated,
        }
    }
}
