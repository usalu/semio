use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

use crate::guid::Guid;
use crate::hash::HashWriter;

/// A human author attached to a design, type, or kit.
#[derive(Debug)]
pub struct AuthorStore {
    pub guid: Guid,
    pub name: String,
    pub email: String,
    pub role: Option<String>,
    pub rank: Option<i64>,
    hash_cache: OnceLock<String>,
}

pub type AuthorStoreRef = std::sync::Arc<std::sync::RwLock<AuthorStore>>;
pub type AuthorStoreWeak = std::sync::Weak<std::sync::RwLock<AuthorStore>>;

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct AuthorIdDto {
    pub guid: Guid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct AuthorMetadataDto {
    pub guid: Guid,
    pub name: String,
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct AuthorShallowDto {
    #[serde(flatten)]
    pub meta: AuthorMetadataDto,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct AuthorFullDto {
    #[serde(flatten)]
    pub meta: AuthorMetadataDto,
}

impl AuthorStore {
    pub fn new(name: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            guid: Guid::new_v7(),
            name: name.into(),
            email: email.into(),
            role: None,
            rank: None,
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_id_dto(d: AuthorIdDto) -> Self {
        Self {
            guid: d.guid,
            name: String::new(),
            email: String::new(),
            role: None,
            rank: None,
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_metadata_dto(d: AuthorMetadataDto) -> Self {
        Self {
            guid: d.guid,
            name: d.name,
            email: d.email,
            role: d.role,
            rank: d.rank,
            hash_cache: OnceLock::new(),
        }
    }

    pub fn from_shallow_dto(d: AuthorShallowDto) -> Self {
        Self::from_metadata_dto(d.meta)
    }

    pub fn from_full_dto(d: AuthorFullDto) -> Self {
        Self::from_metadata_dto(d.meta)
    }

    pub fn to_id_dto(&self) -> AuthorIdDto {
        AuthorIdDto { guid: self.guid.clone() }
    }

    pub fn to_metadata_dto(&self) -> AuthorMetadataDto {
        AuthorMetadataDto {
            guid: self.guid.clone(),
            name: self.name.clone(),
            email: self.email.clone(),
            role: self.role.clone(),
            rank: self.rank,
        }
    }

    pub fn to_shallow_dto(&self) -> AuthorShallowDto {
        AuthorShallowDto { meta: self.to_metadata_dto() }
    }

    pub fn to_full_dto(&self) -> AuthorFullDto {
        AuthorFullDto { meta: self.to_metadata_dto() }
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
        w.tag("author")
            .str(self.guid.as_str())
            .str(&self.name)
            .str(&self.email)
            .opt_str(self.role.as_deref());
        if let Some(r) = self.rank {
            w.f64(r as f64);
        }
    }
}
