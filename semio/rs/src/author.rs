use serde::{Deserialize, Serialize};
use std::sync::{RwLock, Weak};

use crate::design::DesignStoreWeak;
use crate::guid::Guid;
use crate::hash::{Cache, HashWriter};
use crate::kit::KitStoreWeak;
use crate::typ::TypeStoreWeak;

pub type AuthorStoreRef = std::sync::Arc<RwLock<AuthorStore>>;
pub type AuthorStoreWeak = Weak<RwLock<AuthorStore>>;

/// A human author attached to a design, type, or kit.
#[derive(Debug)]
pub struct AuthorStore {
    pub guid: Guid,
    pub name: String,
    pub email: String,
    pub role: Option<String>,
    pub rank: Option<i64>,
    pub parent_kit: Option<KitStoreWeak>,
    pub parent_design: Option<DesignStoreWeak>,
    pub parent_type: Option<TypeStoreWeak>,
    hash_cache: Cache<String>,
}

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
    pub guid: Guid,
    pub name: String,
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct AuthorFullDto {
    pub guid: Guid,
    pub name: String,
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
}

impl AuthorStore {
    pub(crate) fn empty_shell(guid: Guid) -> Self {
        Self {
            guid,
            name: String::new(),
            email: String::new(),
            role: None,
            rank: None,
            parent_kit: None,
            parent_design: None,
            parent_type: None,
            hash_cache: Cache::default(),
        }
    }

    pub(crate) fn apply_full_dto_fields(&mut self, d: AuthorFullDto) {
        self.guid = d.guid;
        self.name = d.name;
        self.email = d.email;
        self.role = d.role;
        self.rank = d.rank;
        self.hash_cache.invalidate();
    }

    pub(crate) fn from_full_dto(d: AuthorFullDto) -> Self {
        let mut s = Self::empty_shell(d.guid.clone());
        s.apply_full_dto_fields(d);
        s
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
        self.bubble();
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
        self.bubble();
    }

    pub fn set_role(&mut self, role: Option<String>) {
        self.role = role;
        self.bubble();
    }

    pub fn set_rank(&mut self, rank: Option<i64>) {
        self.rank = rank;
        self.bubble();
    }

    fn bubble(&mut self) {
        self.hash_cache.invalidate();
        if let Some(w) = &self.parent_kit {
            if let Some(k) = w.upgrade() {
                if let Ok(k) = k.read() {
                    k.invalidate_hash();
                    k.invalidate_validation();
                }
            }
        }
        if let Some(w) = &self.parent_design {
            if let Some(d) = w.upgrade() {
                if let Ok(d) = d.read() {
                    d.invalidate_hash();
                    d.invalidate_flatten();
                    d.invalidate_validation();
                }
            }
        }
        if let Some(w) = &self.parent_type {
            if let Some(t) = w.upgrade() {
                if let Ok(t) = t.read() {
                    t.invalidate_hash();
                }
            }
        }
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
        let m = self.to_metadata_dto();
        AuthorShallowDto {
            guid: m.guid,
            name: m.name,
            email: m.email,
            role: m.role,
            rank: m.rank,
        }
    }

    pub fn to_full_dto(&self) -> AuthorFullDto {
        let m = self.to_metadata_dto();
        AuthorFullDto {
            guid: m.guid,
            name: m.name,
            email: m.email,
            role: m.role,
            rank: m.rank,
        }
    }

    pub fn invalidate_hash(&self) {
        self.hash_cache.invalidate();
    }

    pub fn hash(&self) -> String {
        self.hash_cache.get_or_init(|| {
            let mut w = HashWriter::new();
            self.hash_into(&mut w);
            w.finalize()
        })
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
