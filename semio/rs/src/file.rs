use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock, RwLock, Weak};

use crate::guid::Guid;
use crate::hash::HashWriter;

pub type FileStoreRef = Arc<RwLock<FileStore>>;
pub type FileStoreWeak = Weak<RwLock<FileStore>>;

/// External resource referenced by a kit (3D model, texture, etc.).
#[derive(Debug)]
pub struct FileStore {
    pub guid: Guid,
    pub url: String,
    pub mime: Option<String>,
    pub size: Option<i64>,
    pub hash: Option<String>,
    pub description: Option<String>,
    pub created: Option<String>,
    pub updated: Option<String>,
    hash_cache: OnceLock<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct FileIdDto {
    pub guid: Guid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct FileMetadataDto {
    pub guid: Guid,
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

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct FileShallowDto {
    #[serde(flatten)]
    pub meta: FileMetadataDto,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct FileFullDto {
    #[serde(flatten)]
    pub meta: FileMetadataDto,
}

impl FileStore {
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
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_id_dto(d: FileIdDto) -> Self {
        Self {
            guid: d.guid,
            url: String::new(),
            mime: None,
            size: None,
            hash: None,
            description: None,
            created: None,
            updated: None,
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_metadata_dto(d: FileMetadataDto) -> Self {
        Self {
            guid: d.guid,
            url: d.url,
            mime: d.mime,
            size: d.size,
            hash: d.hash,
            description: d.description,
            created: d.created,
            updated: d.updated,
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_shallow_dto(d: FileShallowDto) -> Self {
        Self::from_metadata_dto(d.meta)
    }

    pub fn from_full_dto(d: FileFullDto) -> Self {
        Self::from_metadata_dto(d.meta)
    }

    pub fn to_id_dto(&self) -> FileIdDto {
        FileIdDto { guid: self.guid.clone() }
    }

    pub fn to_metadata_dto(&self) -> FileMetadataDto {
        FileMetadataDto {
            guid: self.guid.clone(),
            url: self.url.clone(),
            mime: self.mime.clone(),
            size: self.size,
            hash: self.hash.clone(),
            description: self.description.clone(),
            created: self.created.clone(),
            updated: self.updated.clone(),
        }
    }

    pub fn to_shallow_dto(&self) -> FileShallowDto {
        FileShallowDto { meta: self.to_metadata_dto() }
    }

    pub fn to_full_dto(&self) -> FileFullDto {
        FileFullDto { meta: self.to_metadata_dto() }
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
