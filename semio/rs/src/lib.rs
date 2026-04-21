//! semio: purely object-oriented, pointer-first in-memory graph.
//!
//! Every aggregate owns its children through `Arc<RwLock<T>>`; children hold
//! `Weak<RwLock<T>>` back-references to their parents. Content-addressable
//! hashes are computed lazily through interior-mutable `Cache` on each entity.
//! GUIDs exist only as stable identity at serialization boundaries and in
//! DTO resolvers; the in-memory graph walks pointers.

#![allow(clippy::new_without_default)]

pub mod attribute {
    use serde::{Deserialize, Serialize};
    use std::sync::{RwLock, Weak};

    use crate::design::DesignStoreWeak;
    use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
    use crate::guid::Guid;
    use crate::hash::{Cache, HashWriter};
    use crate::kit::KitStoreWeak;
    use crate::typ::TypeStoreWeak;

    use crate::connection::ConnectionStoreWeak;
    use crate::connector::ConnectorStoreWeak;
    use crate::piece::PieceStoreWeak;
    use crate::port::PortStoreWeak;
    use crate::representation::RepresentationStoreWeak;

    pub type AttributeStoreRef = std::sync::Arc<RwLock<AttributeStore>>;
    pub type AttributeStoreWeak = Weak<RwLock<AttributeStore>>;

    /// A name/value pair attached to pretty much any domain entity.
    #[derive(Debug)]
    pub struct AttributeStore {
        pub guid: Guid,
        pub key: String,
        pub value: String,
        pub definition: Option<String>,
        pub parent_kit: Option<KitStoreWeak>,
        pub parent_design: Option<DesignStoreWeak>,
        pub parent_type: Option<TypeStoreWeak>,
        pub parent_piece: Option<PieceStoreWeak>,
        pub parent_port: Option<PortStoreWeak>,
        pub parent_connection: Option<ConnectionStoreWeak>,
        pub parent_representation: Option<RepresentationStoreWeak>,
        pub parent_connector: Option<ConnectorStoreWeak>,
        pub(crate) event_bus: Weak<EventBus>,
        hash_cache: Cache<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct AttributeIdDto {
        pub guid: Guid,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct AttributeMetadataDto {
        pub guid: Guid,
        pub key: String,
        pub value: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub definition: Option<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct AttributeShallowDto {
        pub guid: Guid,
        pub key: String,
        pub value: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub definition: Option<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct AttributeFullDto {
        pub guid: Guid,
        pub key: String,
        pub value: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub definition: Option<String>,
    }

    impl AttributeStore {
        pub(crate) fn empty_shell(guid: Guid) -> Self {
            Self {
                guid,
                key: String::new(),
                value: String::new(),
                definition: None,
                parent_kit: None,
                parent_design: None,
                parent_type: None,
                parent_piece: None,
                parent_port: None,
                parent_connection: None,
                parent_representation: None,
                parent_connector: None,
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        #[inline]
        fn emit_ev(&self, ev: KitEvent) {
            emit_weak(&self.event_bus, ev);
        }

        fn entity_ref(&self) -> EntityRef {
            EntityRef::new(EntityKind::Attribute, self.guid.clone())
        }

        pub(crate) fn apply_full_dto_fields(&mut self, d: AttributeFullDto) {
            self.guid = d.guid;
            self.key = d.key;
            self.value = d.value;
            self.definition = d.definition;
            self.hash_cache.invalidate();
        }

        pub(crate) fn from_shallow_dto(d: AttributeShallowDto) -> Self {
            let mut s = Self::empty_shell(d.guid.clone());
            s.key = d.key;
            s.value = d.value;
            s.definition = d.definition;
            s.hash_cache.invalidate();
            s
        }

        pub(crate) fn from_full_dto(d: AttributeFullDto) -> Self {
            let mut s = Self::empty_shell(d.guid.clone());
            s.apply_full_dto_fields(d);
            s
        }

        pub fn set_key(&mut self, key: String) -> crate::error::SetResult {
            if let Err(e) = crate::validate::attribute_key(&key, "key") {
                self.emit_ev(KitEvent::SetRejected {
                    entity: self.entity_ref(),
                    field: "key".into(),
                    error: e.clone(),
                });
                return Err(e);
            }
            if self.key == key {
                return Ok(());
            }
            self.key = key;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "key",
            });
            self.invalidate_local_and_bubble();
            Ok(())
        }

        pub fn set_value(&mut self, value: String) -> crate::error::SetResult {
            if let Err(e) = crate::validate::required_non_empty(&value, "value") {
                self.emit_ev(KitEvent::SetRejected {
                    entity: self.entity_ref(),
                    field: "value".into(),
                    error: e.clone(),
                });
                return Err(e);
            }
            if self.value == value {
                return Ok(());
            }
            self.value = value;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "value",
            });
            self.invalidate_local_and_bubble();
            Ok(())
        }

        pub fn set_definition(&mut self, definition: Option<String>) -> crate::error::SetResult {
            if self.definition == definition {
                return Ok(());
            }
            self.definition = definition;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "definition",
            });
            self.invalidate_local_and_bubble();
            Ok(())
        }

        fn invalidate_local_and_bubble(&mut self) {
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            if let Some(w) = &self.parent_kit {
                if let Some(k) = w.upgrade() {
                    if let Ok(k) = k.read() {
                        k.invalidate_hash();
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
            } else if let Some(w) = &self.parent_kit {
                if let Some(k) = w.upgrade() {
                    if let Ok(k) = k.read() {
                        k.invalidate_validation();
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
            if let Some(w) = &self.parent_piece {
                if let Some(p) = w.upgrade() {
                    if let Ok(p) = p.read() {
                        p.invalidate_hash();
                    }
                }
            }
            if let Some(w) = &self.parent_port {
                if let Some(p) = w.upgrade() {
                    if let Ok(p) = p.read() {
                        p.invalidate_hash();
                    }
                }
            }
            if let Some(w) = &self.parent_connection {
                if let Some(c) = w.upgrade() {
                    if let Ok(c) = c.read() {
                        c.notify_aggregate_change();
                    }
                }
            }
            if let Some(w) = &self.parent_representation {
                if let Some(r) = w.upgrade() {
                    if let Ok(r) = r.read() {
                        r.invalidate_hash();
                    }
                }
            }
            if let Some(w) = &self.parent_connector {
                if let Some(c) = w.upgrade() {
                    if let Ok(c) = c.read() {
                        c.invalidate_hash();
                    }
                }
            }
        }

        pub fn to_id_dto(&self) -> AttributeIdDto {
            AttributeIdDto {
                guid: self.guid.clone(),
            }
        }

        pub fn to_metadata_dto(&self) -> AttributeMetadataDto {
            AttributeMetadataDto {
                guid: self.guid.clone(),
                key: self.key.clone(),
                value: self.value.clone(),
                definition: self.definition.clone(),
            }
        }

        pub fn to_shallow_dto(&self) -> AttributeShallowDto {
            let m = self.to_metadata_dto();
            AttributeShallowDto {
                guid: m.guid,
                key: m.key,
                value: m.value,
                definition: m.definition,
            }
        }

        pub fn to_full_dto(&self) -> AttributeFullDto {
            let m = self.to_metadata_dto();
            AttributeFullDto {
                guid: m.guid,
                key: m.key,
                value: m.value,
                definition: m.definition,
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
            w.tag("attr")
                .str(self.guid.as_str())
                .str(&self.key)
                .str(&self.value)
                .opt_str(self.definition.as_deref());
        }
    }
}

pub mod author {
    use serde::{Deserialize, Serialize};
    use std::sync::{RwLock, Weak};

    use crate::design::DesignStoreWeak;
    use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
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
        pub(crate) event_bus: Weak<EventBus>,
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
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        #[inline]
        fn emit_ev(&self, ev: KitEvent) {
            emit_weak(&self.event_bus, ev);
        }

        fn entity_ref(&self) -> EntityRef {
            EntityRef::new(EntityKind::Author, self.guid.clone())
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

        pub fn set_name(&mut self, name: String) -> crate::error::SetResult {
            let name = name.trim().to_string();
            if let Err(e) = crate::validate::required_name(&name, "name") {
                self.emit_ev(KitEvent::SetRejected {
                    entity: self.entity_ref(),
                    field: "name".into(),
                    error: e.clone(),
                });
                return Err(e);
            }
            if self.name == name {
                return Ok(());
            }
            self.name = name;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "name",
            });
            self.bubble();
            Ok(())
        }

        pub fn set_email(&mut self, email: String) -> crate::error::SetResult {
            if let Err(e) = crate::validate::email_basic(&email, "email") {
                self.emit_ev(KitEvent::SetRejected {
                    entity: self.entity_ref(),
                    field: "email".into(),
                    error: e.clone(),
                });
                return Err(e);
            }
            if self.email == email {
                return Ok(());
            }
            self.email = email;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "email",
            });
            self.bubble();
            Ok(())
        }

        pub fn set_role(&mut self, role: Option<String>) -> crate::error::SetResult {
            if self.role == role {
                return Ok(());
            }
            self.role = role;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "role",
            });
            self.bubble();
            Ok(())
        }

        pub fn set_rank(&mut self, rank: Option<i64>) -> crate::error::SetResult {
            if self.rank == rank {
                return Ok(());
            }
            self.rank = rank;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "rank",
            });
            self.bubble();
            Ok(())
        }

        fn bubble(&mut self) {
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            if let Some(w) = &self.parent_kit {
                if let Some(k) = w.upgrade() {
                    if let Ok(k) = k.read() {
                        k.invalidate_hash();
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
            } else if let Some(w) = &self.parent_kit {
                if let Some(k) = w.upgrade() {
                    if let Ok(k) = k.read() {
                        k.invalidate_validation();
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
            AuthorIdDto {
                guid: self.guid.clone(),
            }
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
}

pub mod benchmark {
    use serde::{Deserialize, Serialize};
    use std::sync::{RwLock, Weak};

    use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
    use crate::guid::Guid;
    use crate::hash::{Cache, HashWriter};
    use crate::quality::QualityStoreWeak;

    pub type BenchmarkStoreRef = std::sync::Arc<RwLock<BenchmarkStore>>;
    pub type BenchmarkStoreWeak = Weak<RwLock<BenchmarkStore>>;

    /// Numeric range benchmark used to qualify quality measurements.
    #[derive(Debug)]
    pub struct BenchmarkStore {
        pub guid: Guid,
        pub name: String,
        pub min: Option<f64>,
        pub max: Option<f64>,
        pub min_excluded: Option<bool>,
        pub max_excluded: Option<bool>,
        pub parent_quality: Option<QualityStoreWeak>,
        pub(crate) event_bus: Weak<EventBus>,
        hash_cache: Cache<String>,
    }

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
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            rename = "minExcluded"
        )]
        pub min_excluded: Option<bool>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            rename = "maxExcluded"
        )]
        pub max_excluded: Option<bool>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct BenchmarkShallowDto {
        pub guid: Guid,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub min: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub max: Option<f64>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            rename = "minExcluded"
        )]
        pub min_excluded: Option<bool>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            rename = "maxExcluded"
        )]
        pub max_excluded: Option<bool>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct BenchmarkFullDto {
        pub guid: Guid,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub min: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub max: Option<f64>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            rename = "minExcluded"
        )]
        pub min_excluded: Option<bool>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            rename = "maxExcluded"
        )]
        pub max_excluded: Option<bool>,
    }

    impl BenchmarkStore {
        pub(crate) fn empty_shell(guid: Guid) -> Self {
            Self {
                guid,
                name: String::new(),
                min: None,
                max: None,
                min_excluded: None,
                max_excluded: None,
                parent_quality: None,
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        #[inline]
        fn emit_ev(&self, ev: KitEvent) {
            emit_weak(&self.event_bus, ev);
        }

        fn entity_ref(&self) -> EntityRef {
            EntityRef::new(EntityKind::Benchmark, self.guid.clone())
        }

        pub(crate) fn apply_metadata_dto(&mut self, d: BenchmarkMetadataDto) {
            self.guid = d.guid;
            self.name = d.name;
            self.min = d.min;
            self.max = d.max;
            self.min_excluded = d.min_excluded;
            self.max_excluded = d.max_excluded;
            self.hash_cache.invalidate();
        }

        pub fn set_name(&mut self, name: String) -> crate::error::SetResult {
            if self.name == name {
                return Ok(());
            }
            self.name = name;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "name",
            });
            self.bubble();
            Ok(())
        }

        pub fn set_min(&mut self, min: Option<f64>) -> crate::error::SetResult {
            if self.min == min {
                return Ok(());
            }
            self.min = min;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "min",
            });
            self.bubble();
            Ok(())
        }

        pub fn set_max(&mut self, max: Option<f64>) -> crate::error::SetResult {
            if self.max == max {
                return Ok(());
            }
            self.max = max;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "max",
            });
            self.bubble();
            Ok(())
        }

        pub fn set_min_excluded(&mut self, v: Option<bool>) -> crate::error::SetResult {
            if self.min_excluded == v {
                return Ok(());
            }
            self.min_excluded = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "minExcluded",
            });
            self.bubble();
            Ok(())
        }

        pub fn set_max_excluded(&mut self, v: Option<bool>) -> crate::error::SetResult {
            if self.max_excluded == v {
                return Ok(());
            }
            self.max_excluded = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "maxExcluded",
            });
            self.bubble();
            Ok(())
        }

        fn bubble(&mut self) {
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            if let Some(w) = &self.parent_quality {
                if let Some(q) = w.upgrade() {
                    if let Ok(q) = q.read() {
                        q.invalidate_hash();
                    }
                }
            }
        }

        pub fn to_id_dto(&self) -> BenchmarkIdDto {
            BenchmarkIdDto {
                guid: self.guid.clone(),
            }
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
            let m = self.to_metadata_dto();
            BenchmarkShallowDto {
                guid: m.guid,
                name: m.name,
                min: m.min,
                max: m.max,
                min_excluded: m.min_excluded,
                max_excluded: m.max_excluded,
            }
        }

        pub fn to_full_dto(&self) -> BenchmarkFullDto {
            let m = self.to_metadata_dto();
            BenchmarkFullDto {
                guid: m.guid,
                name: m.name,
                min: m.min,
                max: m.max,
                min_excluded: m.min_excluded,
                max_excluded: m.max_excluded,
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
            w.tag("benchmark")
                .str(self.guid.as_str())
                .str(&self.name)
                .opt_f64(self.min)
                .opt_f64(self.max)
                .opt_bool(self.min_excluded)
                .opt_bool(self.max_excluded);
        }
    }
}

pub mod concept {
    use serde::{Deserialize, Serialize};
    use std::sync::{RwLock, Weak};

    use crate::design::DesignStoreWeak;
    use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
    use crate::guid::Guid;
    use crate::hash::{Cache, HashWriter};
    use crate::kit::KitStoreWeak;
    use crate::typ::TypeStoreWeak;

    pub type ConceptStoreRef = std::sync::Arc<RwLock<ConceptStore>>;
    pub type ConceptStoreWeak = Weak<RwLock<ConceptStore>>;

    /// Conceptual / semantic label grouping types and designs.
    #[derive(Debug)]
    pub struct ConceptStore {
        pub guid: Guid,
        pub name: String,
        pub description: Option<String>,
        pub order: Option<i64>,
        pub parent_kit: Option<KitStoreWeak>,
        pub parent_design: Option<DesignStoreWeak>,
        pub parent_type: Option<TypeStoreWeak>,
        pub(crate) event_bus: Weak<EventBus>,
        hash_cache: Cache<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct ConceptIdDto {
        pub guid: Guid,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct ConceptMetadataDto {
        pub guid: Guid,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub order: Option<i64>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct ConceptShallowDto {
        pub guid: Guid,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub order: Option<i64>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct ConceptFullDto {
        pub guid: Guid,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub order: Option<i64>,
    }

    impl ConceptStore {
        pub(crate) fn empty_shell(guid: Guid) -> Self {
            Self {
                guid,
                name: String::new(),
                description: None,
                order: None,
                parent_kit: None,
                parent_design: None,
                parent_type: None,
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        #[inline]
        fn emit_ev(&self, ev: KitEvent) {
            emit_weak(&self.event_bus, ev);
        }

        fn entity_ref(&self) -> EntityRef {
            EntityRef::new(EntityKind::Concept, self.guid.clone())
        }

        pub(crate) fn apply_full_dto_fields(&mut self, d: ConceptFullDto) {
            self.guid = d.guid;
            self.name = d.name;
            self.description = d.description;
            self.order = d.order;
            self.hash_cache.invalidate();
        }

        pub(crate) fn from_full_dto(d: ConceptFullDto) -> Self {
            let mut s = Self::empty_shell(d.guid.clone());
            s.apply_full_dto_fields(d);
            s
        }

        pub fn set_name(&mut self, name: String) -> crate::error::SetResult {
            if self.name == name {
                return Ok(());
            }
            self.name = name;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "name",
            });
            self.bubble();
            Ok(())
        }

        pub fn set_description(&mut self, description: Option<String>) -> crate::error::SetResult {
            if self.description == description {
                return Ok(());
            }
            self.description = description;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "description",
            });
            self.bubble();
            Ok(())
        }

        pub fn set_order(&mut self, order: Option<i64>) -> crate::error::SetResult {
            if self.order == order {
                return Ok(());
            }
            self.order = order;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "order",
            });
            self.bubble();
            Ok(())
        }

        fn bubble(&mut self) {
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            if let Some(w) = &self.parent_kit {
                if let Some(k) = w.upgrade() {
                    if let Ok(k) = k.read() {
                        k.invalidate_hash();
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
            } else if let Some(w) = &self.parent_kit {
                if let Some(k) = w.upgrade() {
                    if let Ok(k) = k.read() {
                        k.invalidate_validation();
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

        pub fn to_id_dto(&self) -> ConceptIdDto {
            ConceptIdDto {
                guid: self.guid.clone(),
            }
        }

        pub fn to_metadata_dto(&self) -> ConceptMetadataDto {
            ConceptMetadataDto {
                guid: self.guid.clone(),
                name: self.name.clone(),
                description: self.description.clone(),
                order: self.order,
            }
        }

        pub fn to_shallow_dto(&self) -> ConceptShallowDto {
            let m = self.to_metadata_dto();
            ConceptShallowDto {
                guid: m.guid,
                name: m.name,
                description: m.description,
                order: m.order,
            }
        }

        pub fn to_full_dto(&self) -> ConceptFullDto {
            let m = self.to_metadata_dto();
            ConceptFullDto {
                guid: m.guid,
                name: m.name,
                description: m.description,
                order: m.order,
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
            w.tag("concept")
                .str(self.guid.as_str())
                .str(&self.name)
                .opt_str(self.description.as_deref());
            if let Some(o) = self.order {
                w.f64(o as f64);
            }
        }
    }
}

pub mod connection {
    use serde::{Deserialize, Serialize};
    use std::sync::{Arc, RwLock, Weak};

    use crate::attribute::{AttributeFullDto, AttributeShallowDto, AttributeStoreRef};
    use crate::connector::ConnectorStore;
    use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
    use crate::flatten_math::{self, compute_child_center_uv};
    use crate::geom::{Coord, Plane};
    use crate::guid::Guid;
    use crate::hash::{Cache, HashWriter};
    use crate::side::{SideMetadataDto, SideStore, SideStoreRef};

    pub type ConnectionStoreRef = Arc<RwLock<ConnectionStore>>;
    pub type ConnectionStoreWeak = Weak<RwLock<ConnectionStore>>;

    /// Join between two [`crate::piece::PieceStore`] instances.
    #[derive(Debug)]
    pub struct ConnectionStore {
        pub guid: Guid,
        pub connected: SideStoreRef,
        pub connecting: SideStoreRef,
        pub gap: Option<f64>,
        pub shift: Option<f64>,
        pub rise: Option<f64>,
        pub rotation: Option<f64>,
        pub turn: Option<f64>,
        pub tilt: Option<f64>,
        pub x: Option<f64>,
        pub y: Option<f64>,
        pub description: Option<String>,
        pub attributes: Vec<AttributeStoreRef>,
        pub parent_design: Weak<RwLock<crate::design::DesignStore>>,
        pub(crate) event_bus: Weak<EventBus>,
        hash_cache: Cache<String>,
        child_plane_matrix: Cache<nalgebra::Matrix4<f64>>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct ConnectionIdDto {
        pub guid: Guid,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct ConnectionMetadataDto {
        pub guid: Guid,
        pub connected: SideMetadataDto,
        pub connecting: SideMetadataDto,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub gap: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub shift: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rise: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rotation: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub turn: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tilt: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub x: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub y: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct ConnectionShallowDto {
        pub guid: Guid,
        pub connected: SideMetadataDto,
        pub connecting: SideMetadataDto,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub gap: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub shift: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rise: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rotation: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub turn: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tilt: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub x: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub y: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub attributes: Vec<AttributeShallowDto>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct ConnectionFullDto {
        pub guid: Guid,
        pub connected: SideMetadataDto,
        pub connecting: SideMetadataDto,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub gap: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub shift: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rise: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rotation: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub turn: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tilt: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub x: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub y: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub attributes: Vec<AttributeFullDto>,
    }

    /// Port-local anchor for a connector (type space).
    pub fn connector_anchor_ports(c: &ConnectorStore) -> (Coord, Coord) {
        if let Some(w) = &c.port {
            if let Some(p) = w.upgrade() {
                if let Ok(p) = p.read() {
                    let pt = p.point.unwrap_or(Coord::ZERO);
                    let dir = p.direction.unwrap_or(Coord::new(0.0, 0.0, 1.0));
                    let n = dir.length();
                    let d = if n > 1e-10 {
                        Coord::new(dir.x / n, dir.y / n, dir.z / n)
                    } else {
                        Coord::new(0.0, 0.0, 1.0)
                    };
                    return (pt, d);
                }
            }
        }
        (Coord::ZERO, Coord::new(0.0, 0.0, 1.0))
    }

    impl ConnectionStore {
        pub(crate) fn empty_with_sides(
            guid: Guid,
            connected: SideStoreRef,
            connecting: SideStoreRef,
        ) -> Self {
            Self {
                guid,
                connected,
                connecting,
                gap: None,
                shift: None,
                rise: None,
                rotation: None,
                turn: None,
                tilt: None,
                x: None,
                y: None,
                description: None,
                attributes: Vec::new(),
                parent_design: Weak::new(),
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
                child_plane_matrix: Cache::default(),
            }
        }

        #[inline]
        fn emit_ev(&self, ev: KitEvent) {
            emit_weak(&self.event_bus, ev);
        }

        fn entity_ref(&self) -> EntityRef {
            EntityRef::new(EntityKind::Connection, self.guid.clone())
        }

        /// Invalidate this connection and all design-level aggregates (flatten, validation).
        pub(crate) fn notify_aggregate_change(&self) {
            self.hash_cache.invalidate();
            self.child_plane_matrix.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            if let Some(d) = self.parent_design.upgrade() {
                if let Ok(dr) = d.read() {
                    dr.invalidate_hash();
                    dr.invalidate_flatten();
                    dr.invalidate_validation();
                }
            }
        }

        pub(crate) fn apply_metadata_fields(&mut self, d: ConnectionMetadataDto) {
            self.guid = d.guid;
            self.gap = d.gap;
            self.shift = d.shift;
            self.rise = d.rise;
            self.rotation = d.rotation;
            self.turn = d.turn;
            self.tilt = d.tilt;
            self.x = d.x;
            self.y = d.y;
            self.description = d.description;
            self.hash_cache.invalidate();
            self.child_plane_matrix.invalidate();
        }

        pub fn set_gap(&mut self, v: Option<f64>) -> crate::error::SetResult {
            if self.gap == v {
                return Ok(());
            }
            self.gap = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "gap",
            });
            self.bubble();
            Ok(())
        }
        pub fn set_shift(&mut self, v: Option<f64>) -> crate::error::SetResult {
            if self.shift == v {
                return Ok(());
            }
            self.shift = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "shift",
            });
            self.bubble();
            Ok(())
        }
        pub fn set_rise(&mut self, v: Option<f64>) -> crate::error::SetResult {
            if self.rise == v {
                return Ok(());
            }
            self.rise = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "rise",
            });
            self.bubble();
            Ok(())
        }
        pub fn set_rotation(&mut self, v: Option<f64>) -> crate::error::SetResult {
            if self.rotation == v {
                return Ok(());
            }
            self.rotation = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "rotation",
            });
            self.bubble();
            Ok(())
        }
        pub fn set_turn(&mut self, v: Option<f64>) -> crate::error::SetResult {
            if self.turn == v {
                return Ok(());
            }
            self.turn = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "turn",
            });
            self.bubble();
            Ok(())
        }
        pub fn set_tilt(&mut self, v: Option<f64>) -> crate::error::SetResult {
            if self.tilt == v {
                return Ok(());
            }
            self.tilt = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "tilt",
            });
            self.bubble();
            Ok(())
        }
        pub fn set_x(&mut self, v: Option<f64>) -> crate::error::SetResult {
            if self.x == v {
                return Ok(());
            }
            self.x = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "x",
            });
            self.bubble();
            Ok(())
        }
        pub fn set_y(&mut self, v: Option<f64>) -> crate::error::SetResult {
            if self.y == v {
                return Ok(());
            }
            self.y = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "y",
            });
            self.bubble();
            Ok(())
        }
        pub fn set_description(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.description == v {
                return Ok(());
            }
            self.description = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "description",
            });
            self.bubble();
            Ok(())
        }

        fn bubble(&mut self) {
            self.notify_aggregate_change();
        }

        /// World-space child plane from parent plane and connector geometry (Python `computeChildPlaneDict`).
        pub fn compute_child_plane_for_flatten(
            &self,
            parent_plane: &Plane,
            parent_connector: &ConnectorStore,
            child_connector: &ConnectorStore,
        ) -> Plane {
            let (pp, pd) = connector_anchor_ports(parent_connector);
            let (cp, cd) = connector_anchor_ports(child_connector);
            flatten_math::compute_child_plane(
                parent_plane,
                pp,
                pd,
                cp,
                cd,
                self.gap.unwrap_or(0.0),
                self.shift.unwrap_or(0.0),
                self.rise.unwrap_or(0.0),
                self.rotation.unwrap_or(0.0),
                self.turn.unwrap_or(0.0),
                self.tilt.unwrap_or(0.0),
            )
        }

        /// UV-style center for child piece (Python BFS `child_center`).
        pub fn compute_child_center_for_flatten(
            &self,
            parent_center: Coord,
            parent_connector: &ConnectorStore,
        ) -> Coord {
            let (_, pd) = connector_anchor_ports(parent_connector);
            let connection_u = self.x.unwrap_or(0.0);
            let connection_v = self.y.unwrap_or(0.0);
            let t = match parent_connector.port.as_ref().and_then(|w| w.upgrade()) {
                Some(p) => p.read().ok().and_then(|g| g.t).unwrap_or(0.0),
                None => 0.0,
            };
            compute_child_center_uv(parent_center, connection_u, connection_v, pd.z, t)
        }

        pub fn to_id_dto(&self) -> ConnectionIdDto {
            ConnectionIdDto {
                guid: self.guid.clone(),
            }
        }

        pub fn to_metadata_dto(&self) -> ConnectionMetadataDto {
            ConnectionMetadataDto {
                guid: self.guid.clone(),
                connected: self
                    .connected
                    .read()
                    .map(|s| s.to_metadata_dto())
                    .unwrap_or_default(),
                connecting: self
                    .connecting
                    .read()
                    .map(|s| s.to_metadata_dto())
                    .unwrap_or_default(),
                gap: self.gap,
                shift: self.shift,
                rise: self.rise,
                rotation: self.rotation,
                turn: self.turn,
                tilt: self.tilt,
                x: self.x,
                y: self.y,
                description: self.description.clone(),
            }
        }

        pub fn to_shallow_dto(&self) -> ConnectionShallowDto {
            let m = self.to_metadata_dto();
            ConnectionShallowDto {
                guid: m.guid,
                connected: m.connected,
                connecting: m.connecting,
                gap: m.gap,
                shift: m.shift,
                rise: m.rise,
                rotation: m.rotation,
                turn: m.turn,
                tilt: m.tilt,
                x: m.x,
                y: m.y,
                description: m.description,
                attributes: self
                    .attributes
                    .iter()
                    .filter_map(|a| a.read().ok().map(|a| a.to_shallow_dto()))
                    .collect(),
            }
        }

        pub fn to_full_dto(&self) -> ConnectionFullDto {
            let m = self.to_metadata_dto();
            ConnectionFullDto {
                guid: m.guid,
                connected: m.connected,
                connecting: m.connecting,
                gap: m.gap,
                shift: m.shift,
                rise: m.rise,
                rotation: m.rotation,
                turn: m.turn,
                tilt: m.tilt,
                x: m.x,
                y: m.y,
                description: m.description,
                attributes: self
                    .attributes
                    .iter()
                    .filter_map(|a| a.read().ok().map(|a| a.to_full_dto()))
                    .collect(),
            }
        }

        pub fn invalidate_hash(&self) {
            self.hash_cache.invalidate();
            self.child_plane_matrix.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
        }

        pub fn hash(&self) -> String {
            self.hash_cache.get_or_init(|| {
                let mut w = HashWriter::new();
                self.hash_into(&mut w);
                w.finalize()
            })
        }

        pub fn hash_into(&self, w: &mut HashWriter) {
            w.tag("connection").str(self.guid.as_str());
            if let Ok(s) = self.connected.read() {
                s.hash_into(w);
            }
            if let Ok(s) = self.connecting.read() {
                s.hash_into(w);
            }
            w.opt_f64(self.gap)
                .opt_f64(self.shift)
                .opt_f64(self.rise)
                .opt_f64(self.rotation)
                .opt_f64(self.turn)
                .opt_f64(self.tilt)
                .opt_f64(self.x)
                .opt_f64(self.y)
                .opt_str(self.description.as_deref());
            for a in &self.attributes {
                if let Ok(a) = a.read() {
                    a.hash_into(w);
                }
            }
        }
    }

    impl Default for ConnectionStore {
        fn default() -> Self {
            let s1 = Arc::new(RwLock::new(SideStore::default()));
            let s2 = Arc::new(RwLock::new(SideStore::default()));
            Self::empty_with_sides(crate::guid::Guid::new_v7(), s1, s2)
        }
    }
}

pub mod connector {
    use serde::{Deserialize, Serialize};
    use std::sync::{Arc, RwLock, Weak};

    use crate::attribute::{AttributeFullDto, AttributeShallowDto, AttributeStore};
    use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
    use crate::guid::Guid;
    use crate::hash::{Cache, HashWriter};
    use crate::port::{PortIdDto, PortStoreWeak};
    use crate::quality::{QualityFullDto, QualityShallowDto, QualityStore, QualityStoreRef};

    pub type ConnectorStoreRef = Arc<RwLock<ConnectorStore>>;
    pub type ConnectorStoreWeak = Weak<RwLock<ConnectorStore>>;

    /// A named socket on a [`crate::typ::TypeStore`] that references a concrete port.
    #[derive(Debug)]
    pub struct ConnectorStore {
        pub guid: Guid,
        pub code: String,
        pub description: Option<String>,
        pub port: Option<PortStoreWeak>,
        pub qualities: Vec<QualityStoreRef>,
        pub attributes: Vec<AttributeStore>,
        /// Back-reference to the owning type.
        pub parent_type: Weak<RwLock<crate::typ::TypeStore>>,
        pub(crate) event_bus: Weak<EventBus>,
        hash_cache: Cache<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct ConnectorIdDto {
        pub guid: Guid,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct ConnectorMetadataDto {
        pub guid: Guid,
        pub code: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub port: Option<PortIdDto>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct ConnectorShallowDto {
        pub guid: Guid,
        pub code: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub port: Option<PortIdDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub qualities: Vec<QualityShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub attributes: Vec<AttributeShallowDto>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct ConnectorFullDto {
        pub guid: Guid,
        pub code: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub port: Option<PortIdDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub qualities: Vec<QualityFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub attributes: Vec<AttributeFullDto>,
    }

    impl ConnectorStore {
        pub fn new(code: impl Into<String>) -> Self {
            Self {
                guid: Guid::new_v7(),
                code: code.into(),
                description: None,
                port: None,
                qualities: Vec::new(),
                attributes: Vec::new(),
                parent_type: Weak::new(),
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        #[inline]
        fn emit_ev(&self, ev: KitEvent) {
            emit_weak(&self.event_bus, ev);
        }

        fn entity_ref(&self) -> EntityRef {
            EntityRef::new(EntityKind::Connector, self.guid.clone())
        }

        pub fn from_id_dto(d: ConnectorIdDto) -> Self {
            Self {
                guid: d.guid,
                code: String::new(),
                description: None,
                port: None,
                qualities: Vec::new(),
                attributes: Vec::new(),
                parent_type: Weak::new(),
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        pub fn from_metadata_dto(d: ConnectorMetadataDto) -> Self {
            Self {
                guid: d.guid,
                code: d.code,
                description: d.description,
                port: None,
                qualities: Vec::new(),
                attributes: Vec::new(),
                parent_type: Weak::new(),
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        pub fn from_shallow_dto(d: ConnectorShallowDto) -> Self {
            let mut s = Self::from_metadata_dto(ConnectorMetadataDto {
                guid: d.guid,
                code: d.code,
                description: d.description,
                port: d.port,
            });
            s.qualities = d
                .qualities
                .into_iter()
                .map(|q| Arc::new(RwLock::new(QualityStore::from_shallow_dto(q))))
                .collect();
            s.attributes = d
                .attributes
                .into_iter()
                .map(AttributeStore::from_shallow_dto)
                .collect();
            s
        }

        pub fn from_full_dto(d: ConnectorFullDto) -> Self {
            let mut s = Self::from_metadata_dto(ConnectorMetadataDto {
                guid: d.guid,
                code: d.code,
                description: d.description,
                port: d.port,
            });
            s.qualities = d
                .qualities
                .into_iter()
                .map(|q| Arc::new(RwLock::new(QualityStore::from_full_dto(q))))
                .collect();
            s.attributes = d
                .attributes
                .into_iter()
                .map(AttributeStore::from_full_dto)
                .collect();
            s
        }

        pub fn to_id_dto(&self) -> ConnectorIdDto {
            ConnectorIdDto {
                guid: self.guid.clone(),
            }
        }

        pub fn to_metadata_dto(&self) -> ConnectorMetadataDto {
            let port = self
                .port
                .as_ref()
                .and_then(|p| p.upgrade())
                .and_then(|p| p.read().ok().map(|p| p.to_id_dto()));
            ConnectorMetadataDto {
                guid: self.guid.clone(),
                code: self.code.clone(),
                description: self.description.clone(),
                port,
            }
        }

        pub fn to_shallow_dto(&self) -> ConnectorShallowDto {
            let m = self.to_metadata_dto();
            ConnectorShallowDto {
                guid: m.guid,
                code: m.code,
                description: m.description,
                port: m.port,
                qualities: self
                    .qualities
                    .iter()
                    .filter_map(|q| q.read().ok().map(|q| q.to_shallow_dto()))
                    .collect(),
                attributes: self
                    .attributes
                    .iter()
                    .map(AttributeStore::to_shallow_dto)
                    .collect(),
            }
        }

        pub fn to_full_dto(&self) -> ConnectorFullDto {
            let m = self.to_metadata_dto();
            ConnectorFullDto {
                guid: m.guid,
                code: m.code,
                description: m.description,
                port: m.port,
                qualities: self
                    .qualities
                    .iter()
                    .filter_map(|q| q.read().ok().map(|q| q.to_full_dto()))
                    .collect(),
                attributes: self
                    .attributes
                    .iter()
                    .map(AttributeStore::to_full_dto)
                    .collect(),
            }
        }

        pub fn set_code(&mut self, code: String) -> crate::error::SetResult {
            if let Err(e) = crate::validate::required_non_empty(&code, "code") {
                self.emit_ev(KitEvent::SetRejected {
                    entity: self.entity_ref(),
                    field: "code".into(),
                    error: e.clone(),
                });
                return Err(e);
            }
            if self.code == code {
                return Ok(());
            }
            self.code = code;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "code",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_description(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.description == v {
                return Ok(());
            }
            self.description = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "description",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_port_weak(&mut self, port: Option<PortStoreWeak>) -> crate::error::SetResult {
            self.port = port;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "port",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn invalidate_hash(&self) {
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            if let Some(t) = self.parent_type.upgrade() {
                if let Ok(tr) = t.read() {
                    tr.invalidate_hash();
                }
            }
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
            w.tag("connector")
                .str(self.guid.as_str())
                .str(&self.code)
                .opt_str(self.description.as_deref());
            if let Some(port) = self.port.as_ref().and_then(|p| p.upgrade()) {
                if let Ok(port) = port.read() {
                    w.str(port.guid.as_str());
                }
            }
            for q in &self.qualities {
                if let Ok(q) = q.read() {
                    q.hash_into(w);
                }
            }
            for a in &self.attributes {
                a.hash_into(w);
            }
        }
    }
}

pub mod design {
    use serde::{Deserialize, Serialize};
    use std::collections::{HashMap, HashSet, VecDeque};
    use std::sync::{Arc, RwLock, Weak};

    use crate::attribute::{
        AttributeFullDto, AttributeShallowDto, AttributeStore, AttributeStoreRef,
    };
    use crate::author::{AuthorFullDto, AuthorShallowDto, AuthorStore, AuthorStoreRef};
    use crate::concept::{ConceptFullDto, ConceptShallowDto, ConceptStore, ConceptStoreRef};
    use crate::connection::{
        ConnectionFullDto, ConnectionMetadataDto, ConnectionShallowDto, ConnectionStore,
        ConnectionStoreRef,
    };
    use crate::connector::ConnectorStoreRef;
    use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
    use crate::geom::{Camera, Coord, Location, Plane};
    use crate::group::{GroupFullDto, GroupShallowDto, GroupStore, GroupStoreRef};
    use crate::guid::Guid;
    use crate::hash::{Cache, HashWriter};
    use crate::kit::KitStore;
    use crate::layer::{LayerFullDto, LayerShallowDto, LayerStore, LayerStoreRef};
    use crate::piece::{PieceFullDto, PieceShallowDto, PieceStore, PieceStoreRef};
    use crate::prop::{PropFullDto, PropShallowDto, PropStore, PropStoreRef};
    use crate::quality::{QualityFullDto, QualityShallowDto, QualityStore, QualityStoreRef};
    use crate::side::{SideStore, SideStoreRef};
    use crate::stat::{StatFullDto, StatShallowDto, StatStore, StatStoreRef};
    use crate::tag::{TagFullDto, TagShallowDto, TagStore, TagStoreRef};
    use crate::typ::TypeStoreRef;

    pub type DesignStoreRef = Arc<RwLock<DesignStore>>;
    pub type DesignStoreWeak = Weak<RwLock<DesignStore>>;

    /// A placed/composed design: a scene of pieces joined by connections.
    #[derive(Debug)]
    pub struct DesignStore {
        pub guid: Guid,
        pub name: String,
        pub description: Option<String>,
        pub icon: Option<String>,
        pub image: Option<String>,
        pub variant: Option<String>,
        pub view: Option<String>,
        pub location: Option<Location>,
        pub camera: Option<Camera>,
        pub unit: Option<String>,
        pub pieces: Vec<PieceStoreRef>,
        pub connections: Vec<ConnectionStoreRef>,
        pub layers: Vec<LayerStoreRef>,
        pub groups: Vec<GroupStoreRef>,
        pub authors: Vec<AuthorStoreRef>,
        pub concepts: Vec<ConceptStoreRef>,
        pub tags: Vec<TagStoreRef>,
        pub qualities: Vec<QualityStoreRef>,
        pub props: Vec<PropStoreRef>,
        pub attributes: Vec<AttributeStoreRef>,
        pub stats: Vec<StatStoreRef>,
        pub created: Option<String>,
        pub updated: Option<String>,
        pub parent_kit: Weak<RwLock<crate::kit::KitStore>>,
        pub(crate) event_bus: Weak<EventBus>,
        hash_cache: Cache<String>,
        flatten_cache: Cache<HashMap<Guid, (Plane, Coord)>>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct DesignIdDto {
        pub guid: Guid,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct DesignMetadataDto {
        pub guid: Guid,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub image: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub variant: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub view: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub location: Option<Location>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub camera: Option<Camera>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub created: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub updated: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub kit: Option<crate::kit::KitIdDto>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct DesignShallowDto {
        pub guid: Guid,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub image: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub variant: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub view: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub location: Option<Location>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub camera: Option<Camera>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub created: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub updated: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub kit: Option<crate::kit::KitIdDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub pieces: Vec<PieceShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub connections: Vec<ConnectionShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub layers: Vec<LayerShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub groups: Vec<GroupShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub authors: Vec<AuthorShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub concepts: Vec<ConceptShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tags: Vec<TagShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub qualities: Vec<QualityShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub props: Vec<PropShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub attributes: Vec<AttributeShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub stats: Vec<StatShallowDto>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct DesignFullDto {
        pub guid: Guid,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub image: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub variant: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub view: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub location: Option<Location>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub camera: Option<Camera>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub created: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub updated: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub kit: Option<crate::kit::KitIdDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub pieces: Vec<PieceFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub connections: Vec<ConnectionFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub layers: Vec<LayerFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub groups: Vec<GroupFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub authors: Vec<AuthorFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub concepts: Vec<ConceptFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tags: Vec<TagFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub qualities: Vec<QualityFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub props: Vec<PropFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub attributes: Vec<AttributeFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub stats: Vec<StatFullDto>,
    }

    fn resolve_connector_for_side(
        side: &crate::side::SideStore,
        typ: &crate::typ::TypeStore,
    ) -> Option<ConnectorStoreRef> {
        if let Some(pw) = &side.port {
            if let Some(p) = pw.upgrade() {
                if let Ok(pr) = p.read() {
                    return typ.connector_for_port_guid(&pr.guid);
                }
            }
        }
        typ.connectors.first().cloned()
    }

    fn connection_from_full_dto(
        cdto: ConnectionFullDto,
        piece_index: &HashMap<Guid, PieceStoreRef>,
        design_weak: DesignStoreWeak,
    ) -> ConnectionStoreRef {
        let s1 = Arc::new(RwLock::new(SideStore::empty_shell(
            cdto.connected.guid.clone(),
        )));
        let s2 = Arc::new(RwLock::new(SideStore::empty_shell(
            cdto.connecting.guid.clone(),
        )));
        wire_side_from_dto(&cdto.connected, &s1, piece_index);
        wire_side_from_dto(&cdto.connecting, &s2, piece_index);
        let conn = Arc::new(RwLock::new(ConnectionStore::empty_with_sides(
            cdto.guid.clone(),
            s1.clone(),
            s2.clone(),
        )));
        {
            let mut cw = conn.write().expect("connection write");
            cw.apply_metadata_fields(ConnectionMetadataDto {
                guid: cdto.guid.clone(),
                connected: cdto.connected.clone(),
                connecting: cdto.connecting.clone(),
                gap: cdto.gap,
                shift: cdto.shift,
                rise: cdto.rise,
                rotation: cdto.rotation,
                turn: cdto.turn,
                tilt: cdto.tilt,
                x: cdto.x,
                y: cdto.y,
                description: cdto.description.clone(),
            });
            cw.parent_design = design_weak.clone();
            cw.attributes = cdto
                .attributes
                .into_iter()
                .map(|a| Arc::new(RwLock::new(AttributeStore::from_full_dto(a))))
                .collect();
        }
        if let Ok(mut s1w) = s1.write() {
            s1w.parent_connection = Some(Arc::downgrade(&conn));
        }
        if let Ok(mut s2w) = s2.write() {
            s2w.parent_connection = Some(Arc::downgrade(&conn));
        }
        conn
    }

    fn wire_side_from_dto(
        meta: &crate::side::SideMetadataDto,
        side_ref: &SideStoreRef,
        piece_index: &HashMap<Guid, PieceStoreRef>,
    ) {
        if let Ok(mut w) = side_ref.write() {
            w.apply_metadata_dto(meta.clone());
            if let Some(pref) = piece_index.get(&meta.piece.guid) {
                let _ = w.set_piece_weak(Arc::downgrade(pref));
                if let Some(port_id) = &meta.port {
                    if let Ok(pc) = pref.read() {
                        if let Some(tw) = &pc.type_ref {
                            if let Some(t) = tw.upgrade() {
                                if let Ok(tr) = t.read() {
                                    for pr in &tr.ports {
                                        if let Ok(prr) = pr.read() {
                                            if prr.guid == port_id.guid {
                                                let _ = w.set_port_weak(Some(Arc::downgrade(pr)));
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if let Some(dp) = &meta.design_piece {
                    if let Some(dpref) = piece_index.get(&dp.guid) {
                        let _ = w.set_design_piece_weak(Some(Arc::downgrade(dpref)));
                    }
                }
            }
        }
    }

    impl DesignStore {
        pub fn new(name: impl Into<String>) -> Self {
            Self {
                guid: Guid::new_v7(),
                name: name.into(),
                description: None,
                icon: None,
                image: None,
                variant: None,
                view: None,
                location: None,
                camera: None,
                unit: None,
                pieces: Vec::new(),
                connections: Vec::new(),
                layers: Vec::new(),
                groups: Vec::new(),
                authors: Vec::new(),
                concepts: Vec::new(),
                tags: Vec::new(),
                qualities: Vec::new(),
                props: Vec::new(),
                attributes: Vec::new(),
                stats: Vec::new(),
                created: None,
                updated: None,
                parent_kit: Weak::new(),
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
                flatten_cache: Cache::default(),
            }
        }

        pub(crate) fn empty_shell(guid: Guid, name: String) -> Self {
            Self {
                guid,
                name,
                description: None,
                icon: None,
                image: None,
                variant: None,
                view: None,
                location: None,
                camera: None,
                unit: None,
                pieces: Vec::new(),
                connections: Vec::new(),
                layers: Vec::new(),
                groups: Vec::new(),
                authors: Vec::new(),
                concepts: Vec::new(),
                tags: Vec::new(),
                qualities: Vec::new(),
                props: Vec::new(),
                attributes: Vec::new(),
                stats: Vec::new(),
                created: None,
                updated: None,
                parent_kit: Weak::new(),
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
                flatten_cache: Cache::default(),
            }
        }

        #[inline]
        pub(crate) fn emit_ev(&self, ev: KitEvent) {
            emit_weak(&self.event_bus, ev);
        }

        pub(crate) fn entity_ref(&self) -> EntityRef {
            EntityRef::new(EntityKind::Design, self.guid.clone())
        }

        pub(crate) fn apply_metadata_fields(&mut self, d: DesignMetadataDto) {
            self.guid = d.guid;
            self.name = d.name;
            self.description = d.description;
            self.icon = d.icon;
            self.image = d.image;
            self.variant = d.variant;
            self.view = d.view;
            self.location = d.location;
            self.camera = d.camera;
            self.unit = d.unit;
            self.created = d.created;
            self.updated = d.updated;
            self.hash_cache.invalidate();
            self.flatten_cache.invalidate();
        }

        pub fn invalidate_hash(&self) {
            self.invalidate_hash_local();
            if let Some(k) = self.parent_kit.upgrade() {
                if let Ok(kr) = k.read() {
                    kr.invalidate_hash();
                }
            }
        }

        /// Like [`Self::invalidate_hash`] but does not bubble to the parent kit (avoids deadlock when
        /// the kit already holds its write lock, e.g. during [`KitStore::apply_design_diff`]).
        pub(crate) fn invalidate_hash_local(&self) {
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
        }

        /// Invalidate flatten caches and emit flatten + derived events for all pieces in this design.
        ///
        /// When a [`crate::piece::PieceStore`] mutator bubbles here, it **holds that piece's write
        /// lock**; pass its guid so we can still list it in `FlattenInvalidated` without blocking on
        /// `read()` (which would deadlock).
        pub fn invalidate_flatten(&self) {
            self.invalidate_flatten_with_locked_piece(None);
        }

        pub(crate) fn invalidate_flatten_with_locked_piece(&self, locked_piece: Option<Guid>) {
            let design_guid = self.guid.clone();
            let mut piece_guids: Vec<Guid> = self
                .pieces
                .iter()
                .filter_map(|p| match p.try_read() {
                    Ok(pr) => Some(pr.guid.clone()),
                    Err(_) => locked_piece.clone(),
                })
                .collect();
            piece_guids.sort_by(|a, b| a.as_str().cmp(b.as_str()));
            piece_guids.dedup();
            self.flatten_cache.invalidate();
            self.emit_ev(KitEvent::FlattenInvalidated {
                design: design_guid,
                pieces: piece_guids.clone(),
            });
            for p in &self.pieces {
                if let Ok(pr) = p.try_read() {
                    pr.invalidate_flat_pose();
                }
            }
            for gid in &piece_guids {
                self.emit_ev(KitEvent::DerivedChanged {
                    entity: EntityRef::new(EntityKind::Piece, gid.clone()),
                    field: "flat_plane",
                });
                self.emit_ev(KitEvent::DerivedChanged {
                    entity: EntityRef::new(EntityKind::Piece, gid.clone()),
                    field: "flat_center",
                });
            }
        }

        pub fn invalidate_validation(&self) {
            if let Some(k) = self.parent_kit.upgrade() {
                if let Ok(k) = k.read() {
                    k.invalidate_validation();
                }
            }
        }

        fn bubble_to_kit(&self) {
            if let Some(k) = self.parent_kit.upgrade() {
                if let Ok(k) = k.read() {
                    k.invalidate_hash();
                    k.invalidate_validation();
                }
            }
        }

        pub fn set_name(&mut self, name: String) -> crate::error::SetResult {
            let name = name.trim().to_string();
            if let Err(e) = crate::validate::required_name(&name, "name") {
                self.emit_ev(KitEvent::SetRejected {
                    entity: self.entity_ref(),
                    field: "name".into(),
                    error: e.clone(),
                });
                return Err(e);
            }
            if self.name == name {
                return Ok(());
            }
            self.name = name;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "name",
            });
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            self.invalidate_flatten();
            self.bubble_to_kit();
            Ok(())
        }

        pub fn set_description(&mut self, v: Option<String>) -> crate::error::SetResult {
            let v = match v {
                None => None,
                Some(s) if s.trim().is_empty() => None,
                Some(s) => Some(s.trim().to_string()),
            };
            if self.description == v {
                return Ok(());
            }
            self.description = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "description",
            });
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            self.invalidate_flatten();
            self.bubble_to_kit();
            Ok(())
        }

        pub fn set_icon(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.icon == v {
                return Ok(());
            }
            self.icon = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "icon",
            });
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            self.invalidate_flatten();
            self.bubble_to_kit();
            Ok(())
        }

        pub fn set_image(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.image == v {
                return Ok(());
            }
            self.image = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "image",
            });
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            self.invalidate_flatten();
            self.bubble_to_kit();
            Ok(())
        }

        pub fn set_variant(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.variant == v {
                return Ok(());
            }
            self.variant = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "variant",
            });
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            self.invalidate_flatten();
            self.bubble_to_kit();
            Ok(())
        }

        pub fn set_view(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.view == v {
                return Ok(());
            }
            self.view = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "view",
            });
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            self.invalidate_flatten();
            self.bubble_to_kit();
            Ok(())
        }

        pub fn set_location(&mut self, v: Option<Location>) -> crate::error::SetResult {
            if self.location == v {
                return Ok(());
            }
            self.location = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "location",
            });
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            self.invalidate_flatten();
            self.bubble_to_kit();
            Ok(())
        }

        pub fn set_camera(&mut self, v: Option<Camera>) -> crate::error::SetResult {
            if self.camera == v {
                return Ok(());
            }
            self.camera = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "camera",
            });
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            self.invalidate_flatten();
            self.bubble_to_kit();
            Ok(())
        }

        pub fn set_unit(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.unit == v {
                return Ok(());
            }
            self.unit = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "unit",
            });
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            self.invalidate_flatten();
            self.bubble_to_kit();
            Ok(())
        }

        pub fn set_created(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.created == v {
                return Ok(());
            }
            self.created = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "created",
            });
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            self.invalidate_flatten();
            self.bubble_to_kit();
            Ok(())
        }

        pub fn set_updated(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.updated == v {
                return Ok(());
            }
            self.updated = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "updated",
            });
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            self.invalidate_flatten();
            self.bubble_to_kit();
            Ok(())
        }

        /// Flattened world-space plane and center per piece guid (BFS, Python `flattenDesignDict`).
        pub fn flatten_map(&self) -> HashMap<Guid, (Plane, Coord)> {
            self.flatten_cache.get_or_init(|| {
                let Some(k) = self.parent_kit.upgrade() else {
                    return self.flatten_identity_only();
                };
                let Ok(kr) = k.read() else {
                    return self.flatten_identity_only();
                };
                self.compute_flatten_with_kit(&*kr)
            })
        }

        fn flatten_identity_only(&self) -> HashMap<Guid, (Plane, Coord)> {
            let mut m = HashMap::new();
            for p in &self.pieces {
                if let Ok(pr) = p.read() {
                    let pl = pr.plane.unwrap_or_else(Plane::world_xy);
                    let ce = pr.center.unwrap_or_default();
                    m.insert(pr.guid.clone(), (pl, ce));
                }
            }
            m
        }

        fn compute_flatten_with_kit(&self, kit: &KitStore) -> HashMap<Guid, (Plane, Coord)> {
            let mut types_by_guid: HashMap<Guid, TypeStoreRef> = HashMap::new();
            for t in &kit.types {
                if let Ok(tr) = t.read() {
                    types_by_guid.insert(tr.guid.clone(), t.clone());
                }
            }
            let mut piece_map: HashMap<Guid, PieceStoreRef> = HashMap::new();
            for p in &self.pieces {
                if let Ok(pr) = p.read() {
                    piece_map.insert(pr.guid.clone(), p.clone());
                }
            }
            if piece_map.is_empty() {
                return HashMap::new();
            }
            let mut adj: HashMap<Guid, Vec<(Guid, ConnectionStoreRef)>> = HashMap::new();
            for c in &self.connections {
                let Ok(conn) = c.read() else { continue };
                let Ok(s0) = conn.connected.read() else {
                    continue;
                };
                let Ok(s1) = conn.connecting.read() else {
                    continue;
                };
                let g0 = s0
                    .piece
                    .upgrade()
                    .and_then(|p| p.read().ok().map(|x| x.guid.clone()));
                let g1 = s1
                    .piece
                    .upgrade()
                    .and_then(|p| p.read().ok().map(|x| x.guid.clone()));
                let (Some(src), Some(tgt)) = (g0, g1) else {
                    continue;
                };
                if !piece_map.contains_key(&src) || !piece_map.contains_key(&tgt) {
                    continue;
                }
                adj.entry(src.clone())
                    .or_default()
                    .push((tgt.clone(), c.clone()));
                adj.entry(tgt).or_default().push((src, c.clone()));
            }
            let mut piece_planes: HashMap<Guid, Plane> = HashMap::new();
            let mut centers: HashMap<Guid, Coord> = HashMap::new();
            let mut visited: HashSet<Guid> = HashSet::new();

            let roots: Vec<Guid> = self
                .pieces
                .iter()
                .filter_map(|p| p.read().ok().map(|pr| pr.guid.clone()))
                .collect();
            for root in roots {
                if visited.contains(&root) {
                    continue;
                }
                let mut q = VecDeque::new();
                q.push_back(root.clone());
                visited.insert(root.clone());
                if let Some(p) = piece_map.get(&root) {
                    if let Ok(pr) = p.read() {
                        let pl = if pr.plane.is_some() && pr.center.is_some() {
                            pr.plane.unwrap()
                        } else {
                            Plane::world_xy()
                        };
                        let ce = pr.center.unwrap_or_default();
                        piece_planes.insert(root.clone(), pl);
                        centers.insert(root, ce);
                    }
                }
                while let Some(current) = q.pop_front() {
                    let Some(current_plane) = piece_planes.get(&current).cloned() else {
                        continue;
                    };
                    let Some(current_piece_ref) = piece_map.get(&current) else {
                        continue;
                    };
                    let Ok(current_piece) = current_piece_ref.read() else {
                        continue;
                    };
                    for (nbr, conn_ref) in adj.get(&current).cloned().unwrap_or_default() {
                        if visited.contains(&nbr) {
                            continue;
                        }
                        let Ok(conn) = conn_ref.read() else { continue };
                        let (parent_side, child_side) = {
                            let Ok(s0) = conn.connected.read() else {
                                continue;
                            };
                            let Ok(s1) = conn.connecting.read() else {
                                continue;
                            };
                            let g0 = s0
                                .piece
                                .upgrade()
                                .and_then(|p| p.read().ok().map(|x| x.guid.clone()));
                            let g1 = s1
                                .piece
                                .upgrade()
                                .and_then(|p| p.read().ok().map(|x| x.guid.clone()));
                            let (Some(a), Some(b)) = (g0, g1) else {
                                continue;
                            };
                            if a == current && b == nbr {
                                (conn.connected.clone(), conn.connecting.clone())
                            } else if b == current && a == nbr {
                                (conn.connecting.clone(), conn.connected.clone())
                            } else {
                                continue;
                            }
                        };
                        let child_id = nbr;
                        let Ok(ps) = parent_side.read() else { continue };
                        let Ok(cs) = child_side.read() else { continue };
                        let Some(child_pref) = piece_map.get(&child_id) else {
                            continue;
                        };
                        let Ok(child_piece) = child_pref.read() else {
                            continue;
                        };
                        let parent_type_guid = current_piece
                            .type_ref
                            .as_ref()
                            .and_then(|w| w.upgrade())
                            .and_then(|t| t.read().ok().map(|t| t.guid.clone()));
                        let child_type_guid = child_piece
                            .type_ref
                            .as_ref()
                            .and_then(|w| w.upgrade())
                            .and_then(|t| t.read().ok().map(|t| t.guid.clone()));
                        let (Some(ptg), Some(ctg)) = (parent_type_guid, child_type_guid) else {
                            continue;
                        };
                        let Some(parent_type) = types_by_guid.get(&ptg) else {
                            continue;
                        };
                        let Some(child_type) = types_by_guid.get(&ctg) else {
                            continue;
                        };
                        let Ok(pt) = parent_type.read() else { continue };
                        let Ok(ct) = child_type.read() else { continue };
                        let Some(pc) = resolve_connector_for_side(&ps, &pt) else {
                            continue;
                        };
                        let Some(cc) = resolve_connector_for_side(&cs, &ct) else {
                            continue;
                        };
                        let Ok(pcr) = pc.read() else { continue };
                        let Ok(ccr) = cc.read() else { continue };
                        let child_plane =
                            conn.compute_child_plane_for_flatten(&current_plane, &pcr, &ccr);
                        let parent_center = centers.get(&current).copied().unwrap_or_default();
                        let child_center =
                            conn.compute_child_center_for_flatten(parent_center, &pcr);
                        piece_planes.insert(child_id.clone(), child_plane);
                        centers.insert(child_id.clone(), child_center);
                        visited.insert(child_id.clone());
                        q.push_back(child_id);
                    }
                }
            }
            piece_planes
                .into_iter()
                .filter_map(|(g, pl)| centers.get(&g).map(|c| (g, (pl, *c))))
                .collect()
        }

        pub fn hash(&self) -> String {
            self.hash_cache.get_or_init(|| {
                let mut w = HashWriter::new();
                self.hash_into(&mut w);
                w.finalize()
            })
        }

        pub fn hash_into(&self, w: &mut HashWriter) {
            w.tag("design")
                .str(self.guid.as_str())
                .str(&self.name)
                .opt_str(self.description.as_deref())
                .opt_str(self.variant.as_deref())
                .opt_str(self.view.as_deref())
                .opt_str(self.unit.as_deref());
            for p in &self.pieces {
                if let Ok(p) = p.read() {
                    p.hash_into(w);
                }
            }
            for c in &self.connections {
                if let Ok(c) = c.read() {
                    c.hash_into(w);
                }
            }
            for l in &self.layers {
                if let Ok(l) = l.read() {
                    l.hash_into(w);
                }
            }
            for g in &self.groups {
                if let Ok(g) = g.read() {
                    g.hash_into(w);
                }
            }
            for a in &self.authors {
                if let Ok(a) = a.read() {
                    a.hash_into(w);
                }
            }
            for c in &self.concepts {
                if let Ok(c) = c.read() {
                    c.hash_into(w);
                }
            }
            for t in &self.tags {
                if let Ok(t) = t.read() {
                    t.hash_into(w);
                }
            }
            for q in &self.qualities {
                if let Ok(q) = q.read() {
                    q.hash_into(w);
                }
            }
            for p in &self.props {
                if let Ok(p) = p.read() {
                    p.hash_into(w);
                }
            }
            for a in &self.attributes {
                if let Ok(a) = a.read() {
                    a.hash_into(w);
                }
            }
            for s in &self.stats {
                if let Ok(s) = s.read() {
                    s.hash_into(w);
                }
            }
        }

        pub fn piece(&self, guid: &str) -> Option<PieceStoreRef> {
            self.pieces
                .iter()
                .find(|p| p.read().map(|p| p.guid.as_str() == guid).unwrap_or(false))
                .cloned()
        }

        pub fn connection(&self, guid: &str) -> Option<ConnectionStoreRef> {
            self.connections
                .iter()
                .find(|c| c.read().map(|c| c.guid.as_str() == guid).unwrap_or(false))
                .cloned()
        }

        pub fn layer(&self, guid: &str) -> Option<LayerStoreRef> {
            self.layers
                .iter()
                .find(|l| l.read().map(|l| l.guid.as_str() == guid).unwrap_or(false))
                .cloned()
        }

        pub fn group(&self, guid: &str) -> Option<GroupStoreRef> {
            self.groups
                .iter()
                .find(|g| g.read().map(|g| g.guid.as_str() == guid).unwrap_or(false))
                .cloned()
        }

        /// Remove pieces (and connections touching them). When `invalidate` is false, caller must
        /// finish with [`Self::invalidate_hash_local`], [`Self::invalidate_flatten`], and kit-level
        /// validation invalidation (see [`KitStore::apply_design_diff`]).
        pub fn delete_pieces(&mut self, piece_guids: &[Guid]) -> usize {
            self.delete_pieces_inner(piece_guids, true)
        }

        pub(crate) fn delete_pieces_inner(
            &mut self,
            piece_guids: &[Guid],
            invalidate: bool,
        ) -> usize {
            let parent = self.entity_ref();
            for g in piece_guids {
                self.emit_ev(KitEvent::ChildRemoved {
                    parent: parent.clone(),
                    child: EntityRef::new(EntityKind::Piece, g.clone()),
                });
            }
            let before = self.pieces.len();
            self.pieces.retain(|p| {
                p.read()
                    .map(|p| !piece_guids.iter().any(|g| *g == p.guid))
                    .unwrap_or(true)
            });
            self.connections.retain(|c| {
                if let Ok(c) = c.read() {
                    let touches = |s: &SideStoreRef| -> bool {
                        s.read()
                            .ok()
                            .and_then(|side| {
                                side.piece.upgrade().and_then(|p| {
                                    p.read().ok().map(|p| piece_guids.contains(&p.guid))
                                })
                            })
                            .unwrap_or(false)
                    };
                    !(touches(&c.connected) || touches(&c.connecting))
                } else {
                    true
                }
            });
            if invalidate {
                self.invalidate_hash();
                self.invalidate_flatten();
                self.invalidate_validation();
            }
            before - self.pieces.len()
        }

        pub fn diff_from(&self, other: &DesignStore) -> crate::diff::DesignDiff {
            crate::diff::DesignDiff::between(&self.to_full_dto(), &other.to_full_dto())
        }

        pub fn apply_diff(
            &mut self,
            diff: &crate::diff::DesignDiff,
            type_index: &HashMap<Guid, TypeStoreRef>,
            design_weak: DesignStoreWeak,
        ) -> crate::error::Result<()> {
            let parent = self.entity_ref();
            for id in &diff.removed_connections {
                self.emit_ev(KitEvent::ChildRemoved {
                    parent: parent.clone(),
                    child: EntityRef::new(EntityKind::Connection, id.guid.clone()),
                });
            }
            for id in &diff.removed_connections {
                self.connections
                    .retain(|c| c.read().map(|c| c.guid != id.guid).unwrap_or(true));
            }
            let removed_piece_guids: Vec<Guid> =
                diff.removed_pieces.iter().map(|p| p.guid.clone()).collect();
            if !removed_piece_guids.is_empty() {
                self.delete_pieces_inner(&removed_piece_guids, false);
            }
            for p in &diff.added_pieces {
                let pref = Arc::new(RwLock::new(PieceStore::empty_shell(p.guid.clone())));
                if let Ok(mut pw) = pref.write() {
                    pw.apply_full_dto(p.clone(), design_weak.clone(), type_index);
                }
                self.emit_ev(KitEvent::ChildAdded {
                    parent: parent.clone(),
                    child: EntityRef::new(EntityKind::Piece, p.guid.clone()),
                });
                self.pieces.push(pref);
            }
            for p in &diff.modified_pieces {
                if let Some(pref) = self.piece(p.guid.as_str()) {
                    if let Ok(mut pw) = pref.write() {
                        pw.apply_full_dto(p.clone(), design_weak.clone(), type_index);
                    }
                }
            }
            let mut piece_index: HashMap<Guid, PieceStoreRef> = HashMap::new();
            for p in &self.pieces {
                if let Ok(pr) = p.read() {
                    piece_index.insert(pr.guid.clone(), p.clone());
                }
            }
            for c in &diff.added_connections {
                self.connections.push(connection_from_full_dto(
                    c.clone(),
                    &piece_index,
                    design_weak.clone(),
                ));
                self.emit_ev(KitEvent::ChildAdded {
                    parent: parent.clone(),
                    child: EntityRef::new(EntityKind::Connection, c.guid.clone()),
                });
            }
            for c in &diff.modified_connections {
                self.emit_ev(KitEvent::ChildRemoved {
                    parent: parent.clone(),
                    child: EntityRef::new(EntityKind::Connection, c.guid.clone()),
                });
                self.connections
                    .retain(|x| x.read().map(|x| x.guid != c.guid).unwrap_or(true));
                self.connections.push(connection_from_full_dto(
                    c.clone(),
                    &piece_index,
                    design_weak.clone(),
                ));
                self.emit_ev(KitEvent::ChildAdded {
                    parent: parent.clone(),
                    child: EntityRef::new(EntityKind::Connection, c.guid.clone()),
                });
            }
            // Do not bubble hash/validation to kit here: [`KitStore::apply_design_diff`] may hold the
            // kit write lock. Flatten events are design-local.
            self.invalidate_hash_local();
            self.invalidate_flatten();
            Ok(())
        }

        pub fn invert_change(change: &crate::diff::DesignChange) -> crate::diff::DesignChange {
            crate::diff::DesignChange {
                forward: change.backward.clone(),
                backward: change.forward.clone(),
                author: change.author.clone(),
                time: change.time.clone(),
                before: change.after.clone(),
                after: change.before.clone(),
            }
        }

        pub fn validate_change(
            &self,
            change: &crate::diff::DesignChange,
        ) -> crate::report::ValidationResult {
            let mut r = crate::report::ValidationResult::valid();
            if change.before.as_ref().map(|b| b.guid.clone())
                != change.after.as_ref().map(|a| a.guid.clone())
            {
                r.is_valid = false;
                r.errors.push(
                    "DesignChange before/after snapshots must refer to the same design guid".into(),
                );
            }
            r
        }

        pub fn to_id_dto(&self) -> DesignIdDto {
            DesignIdDto {
                guid: self.guid.clone(),
            }
        }

        pub fn to_metadata_dto(&self) -> DesignMetadataDto {
            let kit = self.parent_kit.upgrade().and_then(|k| {
                k.read().ok().map(|k| crate::kit::KitIdDto {
                    guid: k.guid.clone(),
                })
            });
            DesignMetadataDto {
                guid: self.guid.clone(),
                name: self.name.clone(),
                description: self.description.clone(),
                icon: self.icon.clone(),
                image: self.image.clone(),
                variant: self.variant.clone(),
                view: self.view.clone(),
                location: self.location,
                camera: self.camera,
                unit: self.unit.clone(),
                created: self.created.clone(),
                updated: self.updated.clone(),
                kit,
            }
        }

        pub fn to_shallow_dto(&self) -> DesignShallowDto {
            let m = self.to_metadata_dto();
            DesignShallowDto {
                guid: m.guid,
                name: m.name,
                description: m.description,
                icon: m.icon,
                image: m.image,
                variant: m.variant,
                view: m.view,
                location: m.location,
                camera: m.camera,
                unit: m.unit,
                created: m.created,
                updated: m.updated,
                kit: m.kit,
                pieces: self
                    .pieces
                    .iter()
                    .filter_map(|p| p.read().ok().map(|p| p.to_shallow_dto()))
                    .collect(),
                connections: self
                    .connections
                    .iter()
                    .filter_map(|c| c.read().ok().map(|c| c.to_shallow_dto()))
                    .collect(),
                layers: self
                    .layers
                    .iter()
                    .filter_map(|l| l.read().ok().map(|l| l.to_shallow_dto()))
                    .collect(),
                groups: self
                    .groups
                    .iter()
                    .filter_map(|g| g.read().ok().map(|g| g.to_shallow_dto()))
                    .collect(),
                authors: self
                    .authors
                    .iter()
                    .filter_map(|a| a.read().ok().map(|a| a.to_shallow_dto()))
                    .collect(),
                concepts: self
                    .concepts
                    .iter()
                    .filter_map(|c| c.read().ok().map(|c| c.to_shallow_dto()))
                    .collect(),
                tags: self
                    .tags
                    .iter()
                    .filter_map(|t| t.read().ok().map(|t| t.to_shallow_dto()))
                    .collect(),
                qualities: self
                    .qualities
                    .iter()
                    .filter_map(|q| q.read().ok().map(|q| q.to_shallow_dto()))
                    .collect(),
                props: self
                    .props
                    .iter()
                    .filter_map(|p| p.read().ok().map(|p| p.to_shallow_dto()))
                    .collect(),
                attributes: self
                    .attributes
                    .iter()
                    .filter_map(|a| a.read().ok().map(|a| a.to_shallow_dto()))
                    .collect(),
                stats: self
                    .stats
                    .iter()
                    .filter_map(|s| s.read().ok().map(|s| s.to_shallow_dto()))
                    .collect(),
            }
        }

        pub fn to_full_dto(&self) -> DesignFullDto {
            let m = self.to_metadata_dto();
            DesignFullDto {
                guid: m.guid,
                name: m.name,
                description: m.description,
                icon: m.icon,
                image: m.image,
                variant: m.variant,
                view: m.view,
                location: m.location,
                camera: m.camera,
                unit: m.unit,
                created: m.created,
                updated: m.updated,
                kit: m.kit,
                pieces: self
                    .pieces
                    .iter()
                    .filter_map(|p| p.read().ok().map(|p| p.to_full_dto()))
                    .collect(),
                connections: self
                    .connections
                    .iter()
                    .filter_map(|c| c.read().ok().map(|c| c.to_full_dto()))
                    .collect(),
                layers: self
                    .layers
                    .iter()
                    .filter_map(|l| l.read().ok().map(|l| l.to_full_dto()))
                    .collect(),
                groups: self
                    .groups
                    .iter()
                    .filter_map(|g| g.read().ok().map(|g| g.to_full_dto()))
                    .collect(),
                authors: self
                    .authors
                    .iter()
                    .filter_map(|a| a.read().ok().map(|a| a.to_full_dto()))
                    .collect(),
                concepts: self
                    .concepts
                    .iter()
                    .filter_map(|c| c.read().ok().map(|c| c.to_full_dto()))
                    .collect(),
                tags: self
                    .tags
                    .iter()
                    .filter_map(|t| t.read().ok().map(|t| t.to_full_dto()))
                    .collect(),
                qualities: self
                    .qualities
                    .iter()
                    .filter_map(|q| q.read().ok().map(|q| q.to_full_dto()))
                    .collect(),
                props: self
                    .props
                    .iter()
                    .filter_map(|p| p.read().ok().map(|p| p.to_full_dto()))
                    .collect(),
                attributes: self
                    .attributes
                    .iter()
                    .filter_map(|a| a.read().ok().map(|a| a.to_full_dto()))
                    .collect(),
                stats: self
                    .stats
                    .iter()
                    .filter_map(|s| s.read().ok().map(|s| s.to_full_dto()))
                    .collect(),
            }
        }

        /// Rebuild design graph from DTO (pieces, connections with [`SideStore`] ends, nested leaves).
        /// Only [`crate::kit::KitStore::from_full_dto`] should construct designs in host code.
        pub(crate) fn hydrate_from_full_dto(
            d: DesignFullDto,
            type_index: &HashMap<Guid, TypeStoreRef>,
        ) -> DesignStoreRef {
            let DesignFullDto {
                guid,
                name,
                description,
                icon,
                image,
                variant,
                view,
                location,
                camera,
                unit,
                created,
                updated,
                kit,
                pieces: piece_dtos,
                connections: connection_dtos,
                layers: layer_dtos,
                groups: group_dtos,
                authors: author_dtos,
                concepts: concept_dtos,
                tags: tag_dtos,
                qualities: quality_dtos,
                props: prop_dtos,
                attributes: attribute_dtos,
                stats: stat_dtos,
            } = d;

            let design = Arc::new(RwLock::new(DesignStore::empty_shell(
                guid.clone(),
                name.clone(),
            )));
            {
                let mut dw = design.write().expect("design write");
                dw.apply_metadata_fields(DesignMetadataDto {
                    guid,
                    name,
                    description,
                    icon,
                    image,
                    variant,
                    view,
                    location,
                    camera,
                    unit,
                    created,
                    updated,
                    kit,
                });
            }

            let dw = Arc::downgrade(&design);

            let piece_guids: Vec<Guid> = piece_dtos.iter().map(|p| p.guid.clone()).collect();
            let mut piece_index: HashMap<Guid, PieceStoreRef> = HashMap::new();
            for pd in &piece_dtos {
                piece_index.insert(
                    pd.guid.clone(),
                    Arc::new(RwLock::new(PieceStore::empty_shell(pd.guid.clone()))),
                );
            }
            for pdto in piece_dtos {
                if let Some(p) = piece_index.get(&pdto.guid) {
                    if let Ok(mut pw) = p.write() {
                        pw.apply_full_dto(pdto, dw.clone(), type_index);
                    }
                }
            }

            let pieces_ordered: Vec<PieceStoreRef> = piece_guids
                .into_iter()
                .filter_map(|g| piece_index.remove(&g))
                .collect();

            let layers: Vec<LayerStoreRef> = layer_dtos
                .into_iter()
                .map(|ldto| {
                    let mut layer = LayerStore::from_full_dto(ldto);
                    layer.parent_design = dw.clone();
                    Arc::new(RwLock::new(layer))
                })
                .collect();

            let groups: Vec<GroupStoreRef> = group_dtos
                .into_iter()
                .map(|gdto| {
                    let mut g = GroupStore::from_full_dto(gdto);
                    g.parent_design = dw.clone();
                    Arc::new(RwLock::new(g))
                })
                .collect();

            let authors: Vec<AuthorStoreRef> = author_dtos
                .into_iter()
                .map(|a| {
                    let mut s = AuthorStore::from_full_dto(a);
                    s.parent_design = Some(dw.clone());
                    Arc::new(RwLock::new(s))
                })
                .collect();

            let concepts: Vec<ConceptStoreRef> = concept_dtos
                .into_iter()
                .map(|c| {
                    let mut s = ConceptStore::from_full_dto(c);
                    s.parent_design = Some(dw.clone());
                    Arc::new(RwLock::new(s))
                })
                .collect();

            let tags: Vec<TagStoreRef> = tag_dtos
                .into_iter()
                .map(|t| {
                    let mut s = TagStore::from_full_dto(t);
                    s.parent_design = Some(dw.clone());
                    Arc::new(RwLock::new(s))
                })
                .collect();

            let qualities: Vec<QualityStoreRef> = quality_dtos
                .into_iter()
                .map(|q| {
                    let mut s = QualityStore::from_full_dto(q);
                    s.parent_design = Some(dw.clone());
                    Arc::new(RwLock::new(s))
                })
                .collect();

            let props: Vec<PropStoreRef> = prop_dtos
                .into_iter()
                .map(|p| {
                    let mut s = PropStore::from_full_dto(p);
                    s.parent_design = Some(dw.clone());
                    Arc::new(RwLock::new(s))
                })
                .collect();

            let attributes: Vec<AttributeStoreRef> = attribute_dtos
                .into_iter()
                .map(|a| {
                    let mut s = AttributeStore::from_full_dto(a);
                    s.parent_design = Some(dw.clone());
                    Arc::new(RwLock::new(s))
                })
                .collect();

            let stats: Vec<StatStoreRef> = stat_dtos
                .into_iter()
                .map(|s| {
                    let mut st = StatStore::from_full_dto(s);
                    st.parent_design = Some(dw.clone());
                    Arc::new(RwLock::new(st))
                })
                .collect();

            let mut piece_index_ordered: HashMap<Guid, PieceStoreRef> = HashMap::new();
            for p in &pieces_ordered {
                if let Ok(pr) = p.read() {
                    piece_index_ordered.insert(pr.guid.clone(), p.clone());
                }
            }

            let connections: Vec<ConnectionStoreRef> = connection_dtos
                .into_iter()
                .map(|cdto| connection_from_full_dto(cdto, &piece_index_ordered, dw.clone()))
                .collect();

            {
                let mut dw = design.write().expect("design write");
                dw.pieces = pieces_ordered;
                dw.connections = connections;
                dw.layers = layers;
                dw.groups = groups;
                dw.authors = authors;
                dw.concepts = concepts;
                dw.tags = tags;
                dw.qualities = qualities;
                dw.props = props;
                dw.attributes = attributes;
                dw.stats = stats;
            }

            design
        }
    }
}

pub mod diff {
    use serde::{Deserialize, Serialize};
    use std::collections::{HashMap, HashSet};

    use crate::connection::{ConnectionFullDto, ConnectionIdDto};
    use crate::design::{DesignFullDto, DesignStore};
    use crate::events::{EntityKind, EntityRef, KitEvent};
    use crate::guid::Guid;
    use crate::piece::{PieceFullDto, PieceIdDto};
    use crate::report::SemioReport;

    /// A symmetric description of a modification to a design: forward re-plays
    /// the change, backward undoes it. `before`/`after` hold full snapshots for
    /// hosts that prefer replacement over patching.
    #[derive(Clone, Debug, Serialize, Deserialize, Default)]
    pub struct DesignChange {
        pub forward: DesignDiff,
        pub backward: DesignDiff,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub author: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub time: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub before: Option<DesignFullDto>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub after: Option<DesignFullDto>,
    }

    /// Structural delta between two [`DesignStore`] states, expressed in DTO shape.
    #[derive(Clone, Debug, Serialize, Deserialize, Default)]
    pub struct DesignDiff {
        #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "addedPieces")]
        pub added_pieces: Vec<PieceFullDto>,
        #[serde(
            default,
            skip_serializing_if = "Vec::is_empty",
            rename = "removedPieces"
        )]
        pub removed_pieces: Vec<PieceIdDto>,
        #[serde(
            default,
            skip_serializing_if = "Vec::is_empty",
            rename = "modifiedPieces"
        )]
        pub modified_pieces: Vec<PieceFullDto>,
        #[serde(
            default,
            skip_serializing_if = "Vec::is_empty",
            rename = "addedConnections"
        )]
        pub added_connections: Vec<ConnectionFullDto>,
        #[serde(
            default,
            skip_serializing_if = "Vec::is_empty",
            rename = "removedConnections"
        )]
        pub removed_connections: Vec<ConnectionIdDto>,
        #[serde(
            default,
            skip_serializing_if = "Vec::is_empty",
            rename = "modifiedConnections"
        )]
        pub modified_connections: Vec<ConnectionFullDto>,
    }

    impl DesignDiff {
        /// Structural delta from `before` snapshot to `after` snapshot (DTO-level).
        pub fn between(before: &DesignFullDto, after: &DesignFullDto) -> Self {
            let mut diff = DesignDiff::default();

            let bp: HashMap<Guid, &PieceFullDto> =
                before.pieces.iter().map(|p| (p.guid.clone(), p)).collect();
            let ap: HashMap<Guid, &PieceFullDto> =
                after.pieces.iter().map(|p| (p.guid.clone(), p)).collect();
            let kb: HashSet<Guid> = bp.keys().cloned().collect();
            let ka: HashSet<Guid> = ap.keys().cloned().collect();
            for g in kb.difference(&ka) {
                diff.removed_pieces.push(PieceIdDto { guid: g.clone() });
            }
            for g in ka.difference(&kb) {
                diff.added_pieces.push((*ap[g]).clone());
            }
            for g in ka.intersection(&kb) {
                let b = bp[g];
                let a = ap[g];
                if *b != *a {
                    diff.modified_pieces.push((*a).clone());
                }
            }

            let bc: HashMap<Guid, &ConnectionFullDto> = before
                .connections
                .iter()
                .map(|c| (c.guid.clone(), c))
                .collect();
            let ac: HashMap<Guid, &ConnectionFullDto> = after
                .connections
                .iter()
                .map(|c| (c.guid.clone(), c))
                .collect();
            let kbc: HashSet<Guid> = bc.keys().cloned().collect();
            let kac: HashSet<Guid> = ac.keys().cloned().collect();
            for g in kbc.difference(&kac) {
                diff.removed_connections
                    .push(ConnectionIdDto { guid: g.clone() });
            }
            for g in kac.difference(&kbc) {
                diff.added_connections.push((*ac[g]).clone());
            }
            for g in kac.intersection(&kbc) {
                let b = bc[g];
                let a = ac[g];
                if *b != *a {
                    diff.modified_connections.push((*a).clone());
                }
            }

            diff
        }
    }

    impl DesignChange {
        pub fn empty() -> Self {
            Self::default()
        }

        pub fn with_before(mut self, design: &DesignStore) -> Self {
            self.before = Some(design.to_full_dto());
            self
        }

        pub fn with_after(mut self, design: &DesignStore) -> Self {
            self.after = Some(design.to_full_dto());
            self
        }
    }

    impl DesignStore {
        pub fn delete_change(
            &mut self,
            piece_guids: &[Guid],
            connection_guids: &[Guid],
        ) -> SemioReport<DesignChange> {
            let before = self.to_full_dto();

            let mut removed_pieces: Vec<PieceFullDto> = Vec::new();
            let mut removed_connections: Vec<ConnectionFullDto> = Vec::new();
            for pg in piece_guids {
                if let Some(p) = self.piece(pg.as_str()) {
                    if let Ok(p) = p.read() {
                        removed_pieces.push(p.to_full_dto());
                    }
                }
            }
            for cg in connection_guids {
                if let Some(c) = self.connection(cg.as_str()) {
                    if let Ok(c) = c.read() {
                        removed_connections.push(c.to_full_dto());
                    }
                }
            }

            let parent = self.entity_ref();
            for cg in connection_guids {
                self.emit_ev(KitEvent::ChildRemoved {
                    parent: parent.clone(),
                    child: EntityRef::new(EntityKind::Connection, cg.clone()),
                });
            }
            self.connections.retain(|c| {
                c.read()
                    .map(|c| !connection_guids.iter().any(|g| *g == c.guid))
                    .unwrap_or(true)
            });
            let _deleted = self.delete_pieces(piece_guids);

            let after = self.to_full_dto();

            let backward = DesignDiff {
                added_pieces: removed_pieces.clone(),
                added_connections: removed_connections.clone(),
                ..DesignDiff::default()
            };
            let forward = DesignDiff {
                removed_pieces: removed_pieces
                    .iter()
                    .map(|p| PieceIdDto {
                        guid: p.guid.clone(),
                    })
                    .collect(),
                removed_connections: removed_connections
                    .iter()
                    .map(|c| ConnectionIdDto {
                        guid: c.guid.clone(),
                    })
                    .collect(),
                ..DesignDiff::default()
            };

            let change = DesignChange {
                forward,
                backward,
                author: None,
                time: None,
                before: Some(before),
                after: Some(after),
            };
            SemioReport::ok(change)
        }

        pub fn flatten_change(&self) -> SemioReport<DesignChange> {
            let before = self.to_full_dto();
            let mut modified_pieces: Vec<PieceFullDto> = Vec::new();
            for piece in &self.pieces {
                if let Ok(p) = piece.read() {
                    let mut dto = p.to_full_dto();
                    dto.plane = Some(p.flat_plane());
                    dto.center = Some(p.flat_center());
                    modified_pieces.push(dto);
                }
            }
            let forward = DesignDiff {
                modified_pieces: modified_pieces.clone(),
                ..DesignDiff::default()
            };
            let backward_mod: Vec<PieceFullDto> = before
                .pieces
                .iter()
                .filter(|p| modified_pieces.iter().any(|m| m.guid == p.guid))
                .cloned()
                .collect();
            let backward = DesignDiff {
                modified_pieces: backward_mod,
                ..DesignDiff::default()
            };
            SemioReport::ok(DesignChange {
                forward,
                backward,
                author: None,
                time: None,
                before: Some(before.clone()),
                after: Some(before),
            })
        }
    }
}

pub mod error {
    use serde::{Deserialize, Serialize};
    use thiserror::Error;

    use crate::guid::Guid;

    /// 🧾 User-visible / wire rejection for a single field write (WASM + hooks).
    #[derive(Error, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(tag = "kind", content = "message")]
    pub enum SetError {
        #[error("illegal name: {0}")]
        IllegalName(String),
        #[error("name too long: {0}")]
        NameTooLong(String),
        #[error("invalid url: {0}")]
        InvalidUrl(String),
        #[error("invalid value: {0}")]
        InvalidValue(String),
        #[error("duplicate guid: {0}")]
        DuplicateGuid(String),
        #[error("not found: {0}")]
        NotFound(String),
        #[error("cyclic reference: {0}")]
        CyclicReference(String),
        #[error("port family mismatch: {0}")]
        PortFamilyMismatch(String),
        #[error("readonly: {0}")]
        Readonly(String),
        #[error("disposed: {0}")]
        Disposed(String),
        #[error("timeout: {0}")]
        Timeout(String),
        #[error("lock poisoned: {0}")]
        LockPoisoned(String),
        #[error("internal: {0}")]
        Internal(String),
    }

    pub type SetResult = std::result::Result<(), SetError>;

    #[derive(Error, Debug)]
    pub enum SemioError {
        #[error("entity not found: {kind} '{guid}'")]
        NotFound { kind: &'static str, guid: Guid },

        #[error("invalid operation: {0}")]
        InvalidOperation(String),

        #[error("lock poisoned accessing {0}")]
        LockPoisoned(&'static str),

        #[error("json error: {0}")]
        Json(#[from] serde_json::Error),

        #[error("io error: {0}")]
        Io(#[from] std::io::Error),

        #[cfg(not(target_arch = "wasm32"))]
        #[error("sqlite error: {0}")]
        Sqlite(#[from] rusqlite::Error),

        #[cfg(not(target_arch = "wasm32"))]
        #[error("zip error: {0}")]
        Zip(#[from] zip::result::ZipError),

        #[error("other: {0}")]
        Other(String),
    }

    pub type Result<T> = std::result::Result<T, SemioError>;

    impl From<SetError> for SemioError {
        fn from(e: SetError) -> Self {
            SemioError::InvalidOperation(e.to_string())
        }
    }
}

pub mod validate {
    //! 🧾 Field-level validation helpers returning [`crate::error::SetError`].

    use crate::error::{SetError, SetResult};

    const MAX_NAME_LEN: usize = 512;
    const MAX_KEY_LEN: usize = 256;
    const MAX_URL_LEN: usize = 4096;

    pub fn required_name(s: &str, label: &str) -> SetResult {
        let t = s.trim();
        if t.is_empty() {
            return Err(SetError::IllegalName(format!("{label} cannot be empty")));
        }
        if t.len() > MAX_NAME_LEN {
            return Err(SetError::NameTooLong(format!(
                "{label} exceeds {MAX_NAME_LEN} chars"
            )));
        }
        Ok(())
    }

    pub fn optional_display_name(s: &Option<String>, label: &str) -> SetResult {
        match s {
            None => Ok(()),
            Some(x) => {
                let t = x.trim();
                if t.is_empty() {
                    return Err(SetError::IllegalName(format!("{label} cannot be empty")));
                }
                if t.len() > MAX_NAME_LEN {
                    return Err(SetError::NameTooLong(format!(
                        "{label} exceeds {MAX_NAME_LEN} chars"
                    )));
                }
                Ok(())
            }
        }
    }

    pub fn required_non_empty(s: &str, label: &str) -> SetResult {
        if s.trim().is_empty() {
            return Err(SetError::InvalidValue(format!("{label} cannot be empty")));
        }
        Ok(())
    }

    pub fn required_url(s: &str, label: &str) -> SetResult {
        let t = s.trim();
        if t.is_empty() {
            return Err(SetError::InvalidUrl(format!("{label} cannot be empty")));
        }
        if t.len() > MAX_URL_LEN {
            return Err(SetError::InvalidUrl(format!(
                "{label} exceeds {MAX_URL_LEN} chars"
            )));
        }
        if t.starts_with('/') || t.starts_with("./") {
            return Ok(());
        }
        if t.contains("://") {
            return Ok(());
        }
        Err(SetError::InvalidUrl(format!(
            "{label} must be a URL or path"
        )))
    }

    pub fn optional_url(s: &Option<String>, label: &str) -> SetResult {
        match s {
            None => Ok(()),
            Some(u) => {
                if u.trim().is_empty() {
                    return Ok(());
                }
                required_url(u, label)
            }
        }
    }

    pub fn optional_opaque_uri(s: &Option<String>, label: &str) -> SetResult {
        match s {
            None => Ok(()),
            Some(u) => {
                let t = u.trim();
                if t.is_empty() {
                    return Ok(());
                }
                if t.len() > MAX_URL_LEN {
                    return Err(SetError::InvalidUrl(format!("{label} exceeds {MAX_URL_LEN} chars")));
                }
                Ok(())
            }
        }
    }

    pub fn email_basic(s: &str, label: &str) -> SetResult {
        let t = s.trim();
        if t.is_empty() {
            return Err(SetError::InvalidValue(format!("{label} cannot be empty")));
        }
        if !t.contains('@') || t.len() < 3 {
            return Err(SetError::InvalidValue(format!(
                "{label} must look like an email"
            )));
        }
        Ok(())
    }

    pub fn attribute_key(s: &str, label: &str) -> SetResult {
        let t = s.trim();
        if t.is_empty() {
            return Err(SetError::IllegalName(format!("{label} cannot be empty")));
        }
        if t.len() > MAX_KEY_LEN {
            return Err(SetError::NameTooLong(format!(
                "{label} exceeds {MAX_KEY_LEN} chars"
            )));
        }
        Ok(())
    }
}

pub mod events {
    //! Kit-scoped async broadcast bus (no tokio). WASM-safe.

    use std::sync::{Arc, Weak};

    use async_broadcast::{broadcast, InactiveReceiver, Receiver, Sender};

    use crate::guid::Guid;

    /// Entity discriminator for [`KitEvent`].
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Serialize)]
    pub enum EntityKind {
        Kit,
        Type,
        Design,
        Piece,
        Connection,
        Side,
        Port,
        Connector,
        Representation,
        File,
        Folder,
        Layer,
        Group,
        Author,
        Concept,
        Tag,
        Prop,
        Attribute,
        Quality,
        Stat,
        Benchmark,
    }

    /// Stable identity for event payloads.
    #[derive(Clone, Debug, Eq, PartialEq, Hash, serde::Serialize)]
    pub struct EntityRef {
        pub kind: EntityKind,
        pub guid: Guid,
    }

    impl EntityRef {
        pub const fn new(kind: EntityKind, guid: Guid) -> Self {
            Self { kind, guid }
        }
    }

    /// All observable changes on a kit graph.
    #[derive(Clone, Debug, PartialEq, serde::Serialize)]
    pub enum KitEvent {
        FieldChanged {
            entity: EntityRef,
            field: &'static str,
        },
        ChildAdded {
            parent: EntityRef,
            child: EntityRef,
        },
        ChildRemoved {
            parent: EntityRef,
            child: EntityRef,
        },
        HashInvalidated {
            entity: EntityRef,
        },
        FlattenInvalidated {
            design: Guid,
            pieces: Vec<Guid>,
        },
        ValidationInvalidated,
        DerivedChanged {
            entity: EntityRef,
            field: &'static str,
        },
        SetRejected {
            entity: EntityRef,
            field: String,
            error: crate::error::SetError,
        },
    }

    /// Broadcast channel wrapper; cloneable [`Sender`] shares one channel.
    #[derive(Debug)]
    pub struct EventBus {
        sender: Sender<KitEvent>,
        /// Keeps the channel open when no [`Receiver`] is subscribed yet (dropping the initial
        /// receiver from [`broadcast`] would close the channel and break all later subscribers).
        #[allow(dead_code)]
        _inactive: InactiveReceiver<KitEvent>,
    }

    impl EventBus {
        pub fn new(capacity: usize) -> Arc<Self> {
            let (mut sender, r) = broadcast(capacity);
            sender.set_overflow(true);
            let inactive = r.deactivate();
            Arc::new(Self {
                sender,
                _inactive: inactive,
            })
        }

        pub fn subscribe(&self) -> Receiver<KitEvent> {
            self.sender.new_receiver()
        }

        pub fn emit(&self, event: KitEvent) {
            let _ = self.sender.try_broadcast(event);
        }
    }

    #[inline]
    pub fn emit_weak(bus: &Weak<EventBus>, event: KitEvent) {
        if let Some(b) = bus.upgrade() {
            b.emit(event);
        }
    }
}

pub(crate) mod event_wire {
    //! Assign [`crate::events::EventBus`] weak handles to every node after kit construction.

    use std::sync::{Arc, Weak};

    use crate::benchmark::BenchmarkStoreRef;
    use crate::connection::ConnectionStoreRef;
    use crate::connector::ConnectorStoreRef;
    use crate::design::DesignStoreRef;
    use crate::events::EventBus;
    use crate::file::FileStoreRef;
    use crate::folder::FolderStoreRef;
    use crate::group::GroupStoreRef;
    use crate::kit::KitStoreRef;
    use crate::layer::LayerStoreRef;
    use crate::piece::PieceStoreRef;
    use crate::port::PortStoreRef;
    use crate::quality::QualityStoreRef;
    use crate::representation::RepresentationStoreRef;
    use crate::stat::StatStoreRef;
    use crate::typ::TypeStoreRef;

    pub(crate) fn wire_graph_bus(kit: &KitStoreRef) {
        let w = {
            let kr = kit.read().expect("kit read");
            Arc::downgrade(&kr.event_bus)
        };
        let kg = kit.read().expect("kit read");
        for t in &kg.types {
            wire_type(t, &w);
        }
        for d in &kg.designs {
            wire_design(d, &w);
        }
        for f in &kg.files {
            wire_file(f, &w);
        }
        for f in &kg.folders {
            wire_folder(f, &w);
        }
        for a in &kg.authors {
            if let Ok(mut g) = a.write() {
                g.event_bus = w.clone();
            }
        }
        for c in &kg.concepts {
            if let Ok(mut g) = c.write() {
                g.event_bus = w.clone();
            }
        }
        for t in &kg.tags {
            if let Ok(mut g) = t.write() {
                g.event_bus = w.clone();
            }
        }
        for q in &kg.qualities {
            wire_quality(q, &w);
        }
        for p in &kg.props {
            if let Ok(mut g) = p.write() {
                g.event_bus = w.clone();
            }
        }
        for a in &kg.attributes {
            if let Ok(mut g) = a.write() {
                g.event_bus = w.clone();
            }
        }
    }

    fn wire_type(t: &TypeStoreRef, w: &Weak<EventBus>) {
        if let Ok(mut g) = t.write() {
            g.event_bus = w.clone();
            for p in &g.ports {
                wire_port(p, w);
            }
            for c in &g.connectors {
                wire_connector(c, w);
            }
            for r in &g.representations {
                wire_representation(r, w);
            }
            for a in &g.authors {
                if let Ok(mut aw) = a.write() {
                    aw.event_bus = w.clone();
                }
            }
            for c in &g.concepts {
                if let Ok(mut cw) = c.write() {
                    cw.event_bus = w.clone();
                }
            }
            for tg in &g.tags {
                if let Ok(mut tw) = tg.write() {
                    tw.event_bus = w.clone();
                }
            }
            for q in &g.qualities {
                wire_quality(q, w);
            }
            for p in &g.props {
                if let Ok(mut pw) = p.write() {
                    pw.event_bus = w.clone();
                }
            }
            for a in &g.attributes {
                if let Ok(mut aw) = a.write() {
                    aw.event_bus = w.clone();
                }
            }
        }
    }

    fn wire_port(p: &PortStoreRef, w: &Weak<EventBus>) {
        if let Ok(mut g) = p.write() {
            g.event_bus = w.clone();
            for q in &g.qualities {
                wire_quality(q, w);
            }
            for a in g.attributes.iter_mut() {
                a.event_bus = w.clone();
            }
        }
    }

    fn wire_connector(c: &ConnectorStoreRef, w: &Weak<EventBus>) {
        if let Ok(mut g) = c.write() {
            g.event_bus = w.clone();
            for q in &g.qualities {
                wire_quality(q, w);
            }
            for a in g.attributes.iter_mut() {
                a.event_bus = w.clone();
            }
        }
    }

    fn wire_representation(r: &RepresentationStoreRef, w: &Weak<EventBus>) {
        if let Ok(mut g) = r.write() {
            g.event_bus = w.clone();
            for tg in g.tags.iter_mut() {
                tg.event_bus = w.clone();
            }
            for q in &g.qualities {
                wire_quality(q, w);
            }
            for a in g.attributes.iter_mut() {
                a.event_bus = w.clone();
            }
        }
    }

    fn wire_design(d: &DesignStoreRef, w: &Weak<EventBus>) {
        if let Ok(mut g) = d.write() {
            g.event_bus = w.clone();
            for p in &g.pieces {
                wire_piece(p, w);
            }
            for c in &g.connections {
                wire_connection(c, w);
            }
            for l in &g.layers {
                wire_layer(l, w);
            }
            for gr in &g.groups {
                wire_group(gr, w);
            }
            for a in &g.authors {
                if let Ok(mut aw) = a.write() {
                    aw.event_bus = w.clone();
                }
            }
            for c in &g.concepts {
                if let Ok(mut cw) = c.write() {
                    cw.event_bus = w.clone();
                }
            }
            for t in &g.tags {
                if let Ok(mut tw) = t.write() {
                    tw.event_bus = w.clone();
                }
            }
            for q in &g.qualities {
                wire_quality(q, w);
            }
            for p in &g.props {
                if let Ok(mut pw) = p.write() {
                    pw.event_bus = w.clone();
                }
            }
            for a in &g.attributes {
                if let Ok(mut aw) = a.write() {
                    aw.event_bus = w.clone();
                }
            }
            for s in &g.stats {
                wire_stat(s, w);
            }
        }
    }

    fn wire_piece(p: &PieceStoreRef, w: &Weak<EventBus>) {
        if let Ok(mut g) = p.write() {
            g.event_bus = w.clone();
            for pr in &g.props {
                if let Ok(mut pw) = pr.write() {
                    pw.event_bus = w.clone();
                }
            }
            for a in &g.attributes {
                if let Ok(mut aw) = a.write() {
                    aw.event_bus = w.clone();
                }
            }
        }
    }

    fn wire_connection(c: &ConnectionStoreRef, w: &Weak<EventBus>) {
        if let Ok(mut g) = c.write() {
            g.event_bus = w.clone();
            if let Ok(mut s) = g.connected.write() {
                s.event_bus = w.clone();
            }
            if let Ok(mut s) = g.connecting.write() {
                s.event_bus = w.clone();
            }
            for a in &g.attributes {
                if let Ok(mut aw) = a.write() {
                    aw.event_bus = w.clone();
                }
            }
        }
    }

    fn wire_layer(l: &LayerStoreRef, w: &Weak<EventBus>) {
        if let Ok(mut g) = l.write() {
            g.event_bus = w.clone();
        }
    }

    fn wire_group(gr: &GroupStoreRef, w: &Weak<EventBus>) {
        if let Ok(mut g) = gr.write() {
            g.event_bus = w.clone();
        }
    }

    fn wire_file(f: &FileStoreRef, w: &Weak<EventBus>) {
        if let Ok(mut g) = f.write() {
            g.event_bus = w.clone();
        }
    }

    fn wire_folder(f: &FolderStoreRef, w: &Weak<EventBus>) {
        if let Ok(mut g) = f.write() {
            g.event_bus = w.clone();
        }
    }

    fn wire_quality(q: &QualityStoreRef, w: &Weak<EventBus>) {
        if let Ok(mut g) = q.write() {
            g.event_bus = w.clone();
            for b in &g.benchmarks {
                wire_benchmark(b, w);
            }
        }
    }

    fn wire_stat(s: &StatStoreRef, w: &Weak<EventBus>) {
        if let Ok(mut g) = s.write() {
            g.event_bus = w.clone();
        }
    }

    fn wire_benchmark(b: &BenchmarkStoreRef, w: &Weak<EventBus>) {
        if let Ok(mut g) = b.write() {
            g.event_bus = w.clone();
        }
    }
}

pub(crate) mod flatten_math {
    //! Geometric helpers for design flatten (mirrors `semio/py/main.py` plane/connector math).

    use nalgebra::{Matrix4, Vector3};

    use crate::geom::{Coord, Plane};

    pub(crate) const FLATTEN_TOLERANCE: f64 = 1e-5;

    fn v3(c: Coord) -> Vector3<f64> {
        Vector3::new(c.x, c.y, c.z)
    }

    #[allow(dead_code)]
    fn coord(v: Vector3<f64>) -> Coord {
        Coord::new(v.x, v.y, v.z)
    }

    fn normalize(v: Vector3<f64>) -> Vector3<f64> {
        let n = v.norm();
        if n < 1e-10 {
            Vector3::new(0.0, 0.0, 1.0)
        } else {
            v / n
        }
    }

    pub(crate) fn plane_to_matrix(p: &Plane) -> Matrix4<f64> {
        let o = v3(p.origin);
        let x = normalize(v3(p.x_axis));
        let y = normalize(v3(p.y_axis));
        let z = normalize(x.cross(&y));
        let mut m = Matrix4::identity();
        m[(0, 0)] = x.x;
        m[(1, 0)] = x.y;
        m[(2, 0)] = x.z;
        m[(0, 1)] = y.x;
        m[(1, 1)] = y.y;
        m[(2, 1)] = y.z;
        m[(0, 2)] = z.x;
        m[(1, 2)] = z.y;
        m[(2, 2)] = z.z;
        m[(0, 3)] = o.x;
        m[(1, 3)] = o.y;
        m[(2, 3)] = o.z;
        m
    }

    pub(crate) fn matrix_to_plane(m: &Matrix4<f64>) -> Plane {
        let ox = m[(0, 3)];
        let oy = m[(1, 3)];
        let oz = m[(2, 3)];
        Plane {
            origin: Coord::new(ox, oy, oz),
            x_axis: Coord::new(m[(0, 0)], m[(1, 0)], m[(2, 0)]),
            y_axis: Coord::new(m[(0, 1)], m[(1, 1)], m[(2, 1)]),
        }
    }

    // Quaternion (qx,qy,qz,qw) from two unit vectors
    fn quat_from_unit_vectors_full(
        v_from: Vector3<f64>,
        v_to: Vector3<f64>,
    ) -> (f64, f64, f64, f64) {
        let vf = normalize(v_from);
        let vt = normalize(v_to);
        let r = vf.dot(&vt) + 1.0;
        let (qx, qy, qz, qw) = if r < 0.000001 {
            if vf.x.abs() > vf.z.abs() {
                (-vf.y, vf.x, 0.0, 0.0)
            } else {
                (0.0, -vf.z, vf.y, 0.0)
            }
        } else {
            let c = vf.cross(&vt);
            (c.x, c.y, c.z, r)
        };
        let n = (qx * qx + qy * qy + qz * qz + qw * qw).sqrt();
        (qx / n, qy / n, qz / n, qw / n)
    }

    fn quat_from_axis_angle(axis: Vector3<f64>, angle: f64) -> (f64, f64, f64, f64) {
        let a = normalize(axis);
        let half = angle / 2.0;
        let s = half.sin();
        (a.x * s, a.y * s, a.z * s, half.cos())
    }

    fn quat_to_mat4(q: (f64, f64, f64, f64)) -> Matrix4<f64> {
        let (x, y, z, w) = q;
        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;
        let xx = x * x2;
        let xy = x * y2;
        let xz = x * z2;
        let yy = y * y2;
        let yz = y * z2;
        let zz = z * z2;
        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;
        let mut m = Matrix4::identity();
        m[(0, 0)] = 1.0 - (yy + zz);
        m[(0, 1)] = xy - wz;
        m[(0, 2)] = xz + wy;
        m[(1, 0)] = xy + wz;
        m[(1, 1)] = 1.0 - (xx + zz);
        m[(1, 2)] = yz - wx;
        m[(2, 0)] = xz - wy;
        m[(2, 1)] = yz + wx;
        m[(2, 2)] = 1.0 - (xx + yy);
        m
    }

    fn make_rotation_axis(axis: Vector3<f64>, angle: f64) -> Matrix4<f64> {
        quat_to_mat4(quat_from_axis_angle(axis, angle))
    }

    fn make_translation(x: f64, y: f64, z: f64) -> Matrix4<f64> {
        let mut m = Matrix4::identity();
        m[(0, 3)] = x;
        m[(1, 3)] = y;
        m[(2, 3)] = z;
        m
    }

    fn apply_mat3_upper(m: &Matrix4<f64>, v: Vector3<f64>) -> Vector3<f64> {
        Vector3::new(
            m[(0, 0)] * v.x + m[(0, 1)] * v.y + m[(0, 2)] * v.z,
            m[(1, 0)] * v.x + m[(1, 1)] * v.y + m[(1, 2)] * v.z,
            m[(2, 0)] * v.x + m[(2, 1)] * v.y + m[(2, 2)] * v.z,
        )
    }

    fn round_tol(x: f64) -> f64 {
        (x / FLATTEN_TOLERANCE).round() * FLATTEN_TOLERANCE
    }

    /// `parent_connector` / `child_connector`: local anchor point + direction in type space.
    pub(crate) fn compute_child_plane(
        parent_plane: &Plane,
        parent_point: Coord,
        parent_direction: Coord,
        child_point: Coord,
        child_direction: Coord,
        gap: f64,
        shift: f64,
        rise: f64,
        rotation_deg: f64,
        turn_deg: f64,
        tilt_deg: f64,
    ) -> Plane {
        let parent_matrix = plane_to_matrix(parent_plane);
        let parent_pt = v3(parent_point);
        let parent_dir = normalize(v3(parent_direction));
        let child_pt = v3(child_point);
        let child_dir = normalize(v3(child_direction));

        let rotation_rad = rotation_deg.to_radians();
        let turn_rad = turn_deg.to_radians();
        let tilt_rad = tilt_deg.to_radians();

        let reverse_child_dir = -child_dir;
        let cross_vec = parent_dir.cross(&reverse_child_dir);
        let cross_len = cross_vec.norm();
        let align_quat = if cross_len < 0.01 {
            if parent_dir.z.abs() < FLATTEN_TOLERANCE {
                quat_from_axis_angle(Vector3::new(0.0, 0.0, 1.0), std::f64::consts::PI)
            } else {
                let axis = normalize(Vector3::new(0.0, 0.0, 1.0).cross(&parent_dir));
                quat_from_axis_angle(axis, std::f64::consts::PI)
            }
        } else {
            quat_from_unit_vectors_full(reverse_child_dir, parent_dir)
        };

        let direction_t = quat_to_mat4(align_quat);
        let y_axis = Vector3::new(0.0, 1.0, 0.0);
        let parent_connector_quat = quat_from_unit_vectors_full(y_axis, parent_dir);
        let parent_rotation_t = quat_to_mat4(parent_connector_quat);

        let gap_direction = apply_mat3_upper(&parent_rotation_t, Vector3::new(0.0, 1.0, 0.0));
        let shift_direction = apply_mat3_upper(&parent_rotation_t, Vector3::new(1.0, 0.0, 0.0));
        let raise_direction = apply_mat3_upper(&parent_rotation_t, Vector3::new(0.0, 0.0, 1.0));
        let mut turn_axis = apply_mat3_upper(&parent_rotation_t, Vector3::new(0.0, 0.0, 1.0));
        let mut tilt_axis = apply_mat3_upper(&parent_rotation_t, Vector3::new(1.0, 0.0, 0.0));

        let mut orientation_t = direction_t;
        let rotate_t = make_rotation_axis(parent_dir, -rotation_rad);
        orientation_t = rotate_t * orientation_t;
        turn_axis = apply_mat3_upper(&rotate_t, turn_axis);
        tilt_axis = apply_mat3_upper(&rotate_t, tilt_axis);
        let turn_t = make_rotation_axis(turn_axis, turn_rad);
        orientation_t = turn_t * orientation_t;
        let tilt_t = make_rotation_axis(tilt_axis, tilt_rad);
        orientation_t = tilt_t * orientation_t;

        let center_child_t = make_translation(-child_pt.x, -child_pt.y, -child_pt.z);
        let mut transform = orientation_t * center_child_t;

        let gap_transform = make_translation(
            gap_direction.x * gap,
            gap_direction.y * gap,
            gap_direction.z * gap,
        );
        let shift_transform = make_translation(
            shift_direction.x * shift,
            shift_direction.y * shift,
            shift_direction.z * shift,
        );
        let raise_transform = make_translation(
            raise_direction.x * rise,
            raise_direction.y * rise,
            raise_direction.z * rise,
        );
        let translation_t = raise_transform * shift_transform * gap_transform;
        transform = translation_t * transform;
        let move_to_parent = make_translation(parent_pt.x, parent_pt.y, parent_pt.z);
        transform = move_to_parent * transform;

        let final_matrix = parent_matrix * transform;
        let mut pl = matrix_to_plane(&final_matrix);
        pl.origin.x = round_tol(pl.origin.x);
        pl.origin.y = round_tol(pl.origin.y);
        pl.origin.z = round_tol(pl.origin.z);
        pl.x_axis.x = round_tol(pl.x_axis.x);
        pl.x_axis.y = round_tol(pl.x_axis.y);
        pl.x_axis.z = round_tol(pl.x_axis.z);
        pl.y_axis.x = round_tol(pl.y_axis.x);
        pl.y_axis.y = round_tol(pl.y_axis.y);
        pl.y_axis.z = round_tol(pl.y_axis.z);
        pl
    }

    pub(crate) const FLATTEN_RADIUS: f64 = 2.697;
    pub(crate) const FLATTEN_VERTICAL_V_EXTRA: f64 = 1.0;
    pub(crate) const FLATTEN_HORIZONTAL_SCALE: f64 = 3.0633;

    /// UV center for child from parent center and connection u/v (matches Python BFS).
    pub(crate) fn compute_child_center_uv(
        parent_center: Coord,
        connection_u: f64,
        connection_v: f64,
        parent_connector_dir_z: f64,
        parent_t: f64,
    ) -> Coord {
        let pu = parent_center.x;
        let pv = parent_center.y;
        if pu.abs() < FLATTEN_TOLERANCE && pv.abs() < FLATTEN_TOLERANCE {
            let angle = 2.0 * std::f64::consts::PI * parent_t;
            Coord::new(
                round_tol(FLATTEN_RADIUS * angle.sin()),
                round_tol(FLATTEN_RADIUS * angle.cos()),
                0.0,
            )
        } else {
            let is_vertical = parent_connector_dir_z.abs() > 0.5;
            let (cu, cv) = if is_vertical {
                (
                    pu + connection_u,
                    pv + connection_v + FLATTEN_VERTICAL_V_EXTRA,
                )
            } else {
                (
                    pu + connection_u * FLATTEN_HORIZONTAL_SCALE,
                    pv + connection_v * FLATTEN_HORIZONTAL_SCALE,
                )
            };
            Coord::new(round_tol(cu), round_tol(cv), 0.0)
        }
    }
}

pub mod file {
    use serde::{Deserialize, Serialize};
    use std::sync::{Arc, RwLock, Weak};

    use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
    use crate::guid::Guid;
    use crate::hash::{Cache, HashWriter};
    use crate::kit::KitStoreWeak;

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
        pub parent_kit: Option<KitStoreWeak>,
        pub(crate) event_bus: Weak<EventBus>,
        hash_cache: Cache<String>,
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
    pub struct FileFullDto {
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
                parent_kit: None,
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        #[inline]
        fn emit_ev(&self, ev: KitEvent) {
            emit_weak(&self.event_bus, ev);
        }

        fn entity_ref(&self) -> EntityRef {
            EntityRef::new(EntityKind::File, self.guid.clone())
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
                parent_kit: None,
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
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
                parent_kit: None,
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        pub fn from_shallow_dto(d: FileShallowDto) -> Self {
            Self::from_metadata_dto(FileMetadataDto {
                guid: d.guid,
                url: d.url,
                mime: d.mime,
                size: d.size,
                hash: d.hash,
                description: d.description,
                created: d.created,
                updated: d.updated,
            })
        }

        pub fn from_full_dto(d: FileFullDto) -> Self {
            Self::from_metadata_dto(FileMetadataDto {
                guid: d.guid,
                url: d.url,
                mime: d.mime,
                size: d.size,
                hash: d.hash,
                description: d.description,
                created: d.created,
                updated: d.updated,
            })
        }

        pub fn to_id_dto(&self) -> FileIdDto {
            FileIdDto {
                guid: self.guid.clone(),
            }
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
            let m = self.to_metadata_dto();
            FileShallowDto {
                guid: m.guid,
                url: m.url,
                mime: m.mime,
                size: m.size,
                hash: m.hash,
                description: m.description,
                created: m.created,
                updated: m.updated,
            }
        }

        pub fn to_full_dto(&self) -> FileFullDto {
            let m = self.to_metadata_dto();
            FileFullDto {
                guid: m.guid,
                url: m.url,
                mime: m.mime,
                size: m.size,
                hash: m.hash,
                description: m.description,
                created: m.created,
                updated: m.updated,
            }
        }

        pub fn set_url(&mut self, url: String) -> crate::error::SetResult {
            if let Err(e) = crate::validate::required_url(&url, "url") {
                self.emit_ev(KitEvent::SetRejected {
                    entity: self.entity_ref(),
                    field: "url".into(),
                    error: e.clone(),
                });
                return Err(e);
            }
            if self.url == url {
                return Ok(());
            }
            self.url = url;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "url",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_mime(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.mime == v {
                return Ok(());
            }
            self.mime = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "mime",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_size(&mut self, v: Option<i64>) -> crate::error::SetResult {
            if self.size == v {
                return Ok(());
            }
            self.size = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "size",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_hash(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.hash == v {
                return Ok(());
            }
            self.hash = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "hash",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_description(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.description == v {
                return Ok(());
            }
            self.description = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "description",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_created(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.created == v {
                return Ok(());
            }
            self.created = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "created",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_updated(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.updated == v {
                return Ok(());
            }
            self.updated = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "updated",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn invalidate_hash(&self) {
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            if let Some(w) = &self.parent_kit {
                if let Some(k) = w.upgrade() {
                    if let Ok(kr) = k.read() {
                        kr.invalidate_hash();
                        kr.invalidate_validation();
                    }
                }
            }
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
}

pub mod folder {
    use serde::{Deserialize, Serialize};
    use std::sync::{Arc, RwLock, Weak};

    use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
    use crate::guid::Guid;
    use crate::hash::{Cache, HashWriter};
    use crate::kit::KitStoreWeak;

    pub type FolderStoreRef = Arc<RwLock<FolderStore>>;
    pub type FolderStoreWeak = Weak<RwLock<FolderStore>>;

    /// Logical folder grouping files inside a kit.
    #[derive(Debug)]
    pub struct FolderStore {
        pub guid: Guid,
        pub path: String,
        pub description: Option<String>,
        pub parent_kit: Option<KitStoreWeak>,
        pub(crate) event_bus: Weak<EventBus>,
        hash_cache: Cache<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct FolderIdDto {
        pub guid: Guid,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct FolderMetadataDto {
        pub guid: Guid,
        pub path: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct FolderShallowDto {
        pub guid: Guid,
        pub path: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct FolderFullDto {
        pub guid: Guid,
        pub path: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }

    impl FolderStore {
        pub fn new(path: impl Into<String>) -> Self {
            Self {
                guid: Guid::new_v7(),
                path: path.into(),
                description: None,
                parent_kit: None,
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        #[inline]
        fn emit_ev(&self, ev: KitEvent) {
            emit_weak(&self.event_bus, ev);
        }

        fn entity_ref(&self) -> EntityRef {
            EntityRef::new(EntityKind::Folder, self.guid.clone())
        }

        pub fn from_id_dto(d: FolderIdDto) -> Self {
            Self {
                guid: d.guid,
                path: String::new(),
                description: None,
                parent_kit: None,
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        pub fn from_metadata_dto(d: FolderMetadataDto) -> Self {
            Self {
                guid: d.guid,
                path: d.path,
                description: d.description,
                parent_kit: None,
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        pub fn from_shallow_dto(d: FolderShallowDto) -> Self {
            Self::from_metadata_dto(FolderMetadataDto {
                guid: d.guid,
                path: d.path,
                description: d.description,
            })
        }

        pub fn from_full_dto(d: FolderFullDto) -> Self {
            Self::from_metadata_dto(FolderMetadataDto {
                guid: d.guid,
                path: d.path,
                description: d.description,
            })
        }

        pub fn to_id_dto(&self) -> FolderIdDto {
            FolderIdDto {
                guid: self.guid.clone(),
            }
        }

        pub fn to_metadata_dto(&self) -> FolderMetadataDto {
            FolderMetadataDto {
                guid: self.guid.clone(),
                path: self.path.clone(),
                description: self.description.clone(),
            }
        }

        pub fn to_shallow_dto(&self) -> FolderShallowDto {
            let m = self.to_metadata_dto();
            FolderShallowDto {
                guid: m.guid,
                path: m.path,
                description: m.description,
            }
        }

        pub fn to_full_dto(&self) -> FolderFullDto {
            let m = self.to_metadata_dto();
            FolderFullDto {
                guid: m.guid,
                path: m.path,
                description: m.description,
            }
        }

        pub fn set_path(&mut self, path: String) -> crate::error::SetResult {
            if self.path == path {
                return Ok(());
            }
            self.path = path;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "path",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_description(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.description == v {
                return Ok(());
            }
            self.description = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "description",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn invalidate_hash(&self) {
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            if let Some(w) = &self.parent_kit {
                if let Some(k) = w.upgrade() {
                    if let Ok(kr) = k.read() {
                        kr.invalidate_hash();
                        kr.invalidate_validation();
                    }
                }
            }
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
            w.tag("folder")
                .str(self.guid.as_str())
                .str(&self.path)
                .opt_str(self.description.as_deref());
        }
    }
}

pub mod geom {
    use serde::{Deserialize, Serialize};

    use crate::hash::HashWriter;

    /// 3D coordinate (right-handed).
    #[derive(Clone, Copy, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct Coord {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    impl Coord {
        pub const ZERO: Coord = Coord {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        pub fn new(x: f64, y: f64, z: f64) -> Self {
            Self { x, y, z }
        }

        pub fn hash_into(&self, w: &mut HashWriter) {
            w.f64(self.x).f64(self.y).f64(self.z);
        }

        pub fn add(&self, other: &Coord) -> Coord {
            Coord::new(self.x + other.x, self.y + other.y, self.z + other.z)
        }

        pub fn sub(&self, other: &Coord) -> Coord {
            Coord::new(self.x - other.x, self.y - other.y, self.z - other.z)
        }

        pub fn scale(&self, s: f64) -> Coord {
            Coord::new(self.x * s, self.y * s, self.z * s)
        }

        pub fn length(&self) -> f64 {
            (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
        }
    }

    /// 3D unit vector (the type is not enforced at construction time).
    pub type Vector = Coord;

    /// Oriented plane: origin `p`, x-axis and y-axis directions.
    #[derive(Clone, Copy, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct Plane {
        #[serde(default)]
        pub origin: Coord,
        #[serde(default = "Plane::default_x_axis")]
        pub x_axis: Vector,
        #[serde(default = "Plane::default_y_axis")]
        pub y_axis: Vector,
    }

    impl Plane {
        fn default_x_axis() -> Vector {
            Vector::new(1.0, 0.0, 0.0)
        }
        fn default_y_axis() -> Vector {
            Vector::new(0.0, 1.0, 0.0)
        }

        pub fn world_xy() -> Self {
            Self {
                origin: Coord::ZERO,
                x_axis: Self::default_x_axis(),
                y_axis: Self::default_y_axis(),
            }
        }

        pub fn hash_into(&self, w: &mut HashWriter) {
            self.origin.hash_into(w);
            self.x_axis.hash_into(w);
            self.y_axis.hash_into(w);
        }
    }

    /// Simple orbital camera descriptor.
    #[derive(Clone, Copy, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct Camera {
        #[serde(default)]
        pub position: Coord,
        #[serde(default)]
        pub target: Coord,
        #[serde(default = "Camera::default_up")]
        pub up: Vector,
        #[serde(default = "Camera::default_fov")]
        pub fov: f64,
    }

    impl Camera {
        fn default_up() -> Vector {
            Vector::new(0.0, 0.0, 1.0)
        }
        fn default_fov() -> f64 {
            45.0
        }
    }

    /// 2D location on the diagram canvas.
    #[derive(Clone, Copy, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct Location {
        pub x: f64,
        pub y: f64,
    }

    impl Location {
        pub fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }

        pub fn hash_into(&self, w: &mut HashWriter) {
            w.f64(self.x).f64(self.y);
        }
    }
}

pub mod group {
    use serde::{Deserialize, Serialize};
    use std::sync::{Arc, RwLock, Weak};

    use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
    use crate::guid::Guid;
    use crate::hash::{Cache, HashWriter};
    use crate::piece::{PieceIdDto, PieceStoreWeak};

    pub type GroupStoreRef = Arc<RwLock<GroupStore>>;
    pub type GroupStoreWeak = Weak<RwLock<GroupStore>>;

    /// User-defined group of pieces inside a [`crate::design::DesignStore`].
    #[derive(Debug)]
    pub struct GroupStore {
        pub guid: Guid,
        pub name: String,
        pub description: Option<String>,
        pub color: Option<String>,
        pub icon: Option<String>,
        pub pieces: Vec<PieceStoreWeak>,
        pub parent_design: Weak<RwLock<crate::design::DesignStore>>,
        pub(crate) event_bus: Weak<EventBus>,
        hash_cache: Cache<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct GroupIdDto {
        pub guid: Guid,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct GroupMetadataDto {
        pub guid: Guid,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub color: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub pieces: Vec<PieceIdDto>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct GroupShallowDto {
        pub guid: Guid,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub color: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub pieces: Vec<PieceIdDto>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct GroupFullDto {
        pub guid: Guid,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub color: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub pieces: Vec<PieceIdDto>,
    }

    impl GroupStore {
        pub fn new(name: impl Into<String>) -> Self {
            Self {
                guid: Guid::new_v7(),
                name: name.into(),
                description: None,
                color: None,
                icon: None,
                pieces: Vec::new(),
                parent_design: Weak::new(),
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        pub fn from_id_dto(d: GroupIdDto) -> Self {
            Self {
                guid: d.guid,
                name: String::new(),
                description: None,
                color: None,
                icon: None,
                pieces: Vec::new(),
                parent_design: Weak::new(),
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        pub fn from_metadata_dto(d: GroupMetadataDto) -> Self {
            Self {
                guid: d.guid,
                name: d.name,
                description: d.description,
                color: d.color,
                icon: d.icon,
                pieces: Vec::new(),
                parent_design: Weak::new(),
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        pub fn from_shallow_dto(d: GroupShallowDto) -> Self {
            Self::from_metadata_dto(GroupMetadataDto {
                guid: d.guid,
                name: d.name,
                description: d.description,
                color: d.color,
                icon: d.icon,
                pieces: d.pieces,
            })
        }

        pub fn from_full_dto(d: GroupFullDto) -> Self {
            Self::from_metadata_dto(GroupMetadataDto {
                guid: d.guid,
                name: d.name,
                description: d.description,
                color: d.color,
                icon: d.icon,
                pieces: d.pieces,
            })
        }

        pub fn to_id_dto(&self) -> GroupIdDto {
            GroupIdDto {
                guid: self.guid.clone(),
            }
        }

        pub fn to_metadata_dto(&self) -> GroupMetadataDto {
            GroupMetadataDto {
                guid: self.guid.clone(),
                name: self.name.clone(),
                description: self.description.clone(),
                color: self.color.clone(),
                icon: self.icon.clone(),
                pieces: self
                    .pieces
                    .iter()
                    .filter_map(|p| p.upgrade())
                    .filter_map(|p| p.read().ok().map(|p| p.to_id_dto()))
                    .collect(),
            }
        }

        pub fn to_shallow_dto(&self) -> GroupShallowDto {
            let m = self.to_metadata_dto();
            GroupShallowDto {
                guid: m.guid,
                name: m.name,
                description: m.description,
                color: m.color,
                icon: m.icon,
                pieces: m.pieces,
            }
        }

        pub fn to_full_dto(&self) -> GroupFullDto {
            let m = self.to_metadata_dto();
            GroupFullDto {
                guid: m.guid,
                name: m.name,
                description: m.description,
                color: m.color,
                icon: m.icon,
                pieces: m.pieces,
            }
        }

        #[inline]
        fn emit_ev(&self, ev: KitEvent) {
            emit_weak(&self.event_bus, ev);
        }

        fn entity_ref(&self) -> EntityRef {
            EntityRef::new(EntityKind::Group, self.guid.clone())
        }

        pub fn set_name(&mut self, name: String) -> crate::error::SetResult {
            if self.name == name {
                return Ok(());
            }
            self.name = name;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "name",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_description(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.description == v {
                return Ok(());
            }
            self.description = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "description",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_color(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.color == v {
                return Ok(());
            }
            self.color = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "color",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_icon(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.icon == v {
                return Ok(());
            }
            self.icon = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "icon",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_pieces(&mut self, pieces: Vec<PieceStoreWeak>) -> crate::error::SetResult {
            self.pieces = pieces;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "pieces",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn invalidate_hash(&self) {
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            if let Some(d) = self.parent_design.upgrade() {
                if let Ok(dr) = d.read() {
                    dr.invalidate_hash();
                }
            }
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
}

pub mod guid {
    use serde::{Deserialize, Serialize};
    use std::borrow::Borrow;
    use std::fmt;
    use std::ops::Deref;

    /// Stable identity used at serialization boundaries and as a dictionary key when
    /// resolving DTOs into the in-memory graph. Never used for in-graph traversal.
    #[derive(
        Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord,
    )]
    #[serde(transparent)]
    pub struct Guid(String);

    impl Guid {
        /// Produces a fresh UUIDv7 (monotonic) wrapped as a `Guid`.
        pub fn new_v7() -> Self {
            Self(uuid::Uuid::now_v7().to_string())
        }

        /// Borrow the underlying string slice.
        pub fn as_str(&self) -> &str {
            &self.0
        }

        /// Consume the [`Guid`], returning the inner [`String`].
        pub fn into_string(self) -> String {
            self.0
        }
    }

    impl fmt::Display for Guid {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(&self.0)
        }
    }

    impl Deref for Guid {
        type Target = str;
        fn deref(&self) -> &str {
            &self.0
        }
    }

    impl AsRef<str> for Guid {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }

    impl Borrow<str> for Guid {
        fn borrow(&self) -> &str {
            &self.0
        }
    }

    impl From<String> for Guid {
        fn from(s: String) -> Self {
            Self(s)
        }
    }

    impl From<&str> for Guid {
        fn from(s: &str) -> Self {
            Self(s.to_owned())
        }
    }

    impl From<Guid> for String {
        fn from(g: Guid) -> Self {
            g.0
        }
    }

    impl PartialEq<str> for Guid {
        fn eq(&self, other: &str) -> bool {
            self.0 == other
        }
    }

    impl PartialEq<&str> for Guid {
        fn eq(&self, other: &&str) -> bool {
            self.0 == *other
        }
    }
}

pub mod hash {
    use sha2::{Digest, Sha256};
    use std::sync::Mutex;

    /// Interior-mutable lazy cache. Enables `invalidate_*(&self)` on entities so
    /// children can bubble invalidation via `parent.read()?.invalidate_hash()`
    /// without acquiring a write lock on the parent.
    #[derive(Debug)]
    pub struct Cache<T> {
        inner: Mutex<Option<T>>,
    }

    impl<T> Default for Cache<T> {
        fn default() -> Self {
            Self {
                inner: Mutex::new(None),
            }
        }
    }

    impl<T: Clone> Cache<T> {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn get_or_init<F: FnOnce() -> T>(&self, f: F) -> T {
            let mut g = self.inner.lock().expect("cache poisoned");
            if g.is_none() {
                *g = Some(f());
            }
            g.clone().unwrap()
        }

        pub fn invalidate(&self) {
            *self.inner.lock().expect("cache poisoned") = None;
        }
    }

    /// Stable content-hash writer that feeds `sha2::Sha256` with deterministic
    /// string primitives. Used by domain entities to produce their own canonical
    /// hash fingerprint on demand.
    pub struct HashWriter {
        inner: Sha256,
    }

    impl HashWriter {
        pub fn new() -> Self {
            Self {
                inner: Sha256::new(),
            }
        }

        pub fn tag(&mut self, tag: &str) -> &mut Self {
            self.inner.update(tag.as_bytes());
            self.inner.update(b"\0");
            self
        }

        pub fn str(&mut self, s: &str) -> &mut Self {
            self.inner.update((s.len() as u64).to_le_bytes());
            self.inner.update(s.as_bytes());
            self
        }

        pub fn opt_str(&mut self, s: Option<&str>) -> &mut Self {
            match s {
                Some(v) => {
                    self.inner.update(b"S");
                    self.str(v);
                }
                None => {
                    self.inner.update(b"N");
                }
            }
            self
        }

        pub fn f64(&mut self, v: f64) -> &mut Self {
            self.inner.update(v.to_le_bytes());
            self
        }

        pub fn opt_f64(&mut self, v: Option<f64>) -> &mut Self {
            match v {
                Some(x) => {
                    self.inner.update(b"F");
                    self.f64(x);
                }
                None => {
                    self.inner.update(b"N");
                }
            }
            self
        }

        pub fn bool(&mut self, v: bool) -> &mut Self {
            self.inner.update([if v { 1u8 } else { 0u8 }]);
            self
        }

        pub fn opt_bool(&mut self, v: Option<bool>) -> &mut Self {
            match v {
                Some(x) => {
                    self.inner.update(b"B");
                    self.bool(x);
                }
                None => {
                    self.inner.update(b"N");
                }
            }
            self
        }

        pub fn finalize(self) -> String {
            hex::encode(self.inner.finalize())
        }
    }

    impl Default for HashWriter {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub mod kit {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::sync::{Arc, RwLock, Weak};

    use async_broadcast::Receiver;

    use crate::attribute::{
        AttributeFullDto, AttributeShallowDto, AttributeStore, AttributeStoreRef,
    };
    use crate::author::{AuthorFullDto, AuthorShallowDto, AuthorStore, AuthorStoreRef};
    use crate::concept::{ConceptFullDto, ConceptShallowDto, ConceptStore, ConceptStoreRef};
    use crate::design::{DesignFullDto, DesignStore, DesignStoreRef};
    use crate::error::{Result, SemioError, SetError, SetResult};
    use crate::event_wire;
    use crate::events::{EntityKind, EntityRef, EventBus, KitEvent};
    use crate::file::{FileFullDto, FileStore, FileStoreRef};
    use crate::folder::{FolderFullDto, FolderStore, FolderStoreRef};
    use crate::diff::DesignDiff;
    use crate::guid::Guid;
    use crate::hash::{Cache, HashWriter};
    use crate::piece::{PieceFullDto, PieceIdDto, PieceStoreRef};
    use crate::prop::{PropFullDto, PropShallowDto, PropStore, PropStoreRef};
    use crate::quality::{QualityFullDto, QualityShallowDto, QualityStore, QualityStoreRef};
    use crate::report::{SemioReport, ValidationResult};
    use crate::tag::{TagFullDto, TagShallowDto, TagStore, TagStoreRef};
    use crate::typ::{TypeFullDto, TypeStore, TypeStoreRef};

    pub type KitStoreRef = Arc<RwLock<KitStore>>;
    pub type KitStoreWeak = Weak<RwLock<KitStore>>;

    /// Root aggregate: a kit owns all components of the system.
    #[derive(Debug)]
    pub struct KitStore {
        pub guid: Guid,
        pub name: String,
        pub description: Option<String>,
        pub icon: Option<String>,
        pub image: Option<String>,
        pub preview: Option<String>,
        pub version: Option<String>,
        pub remote: Option<String>,
        pub homepage: Option<String>,
        pub license: Option<String>,
        pub uri: Option<String>,
        pub created: Option<String>,
        pub updated: Option<String>,
        pub types: Vec<TypeStoreRef>,
        pub designs: Vec<DesignStoreRef>,
        pub files: Vec<FileStoreRef>,
        pub folders: Vec<FolderStoreRef>,
        pub authors: Vec<AuthorStoreRef>,
        pub concepts: Vec<ConceptStoreRef>,
        pub tags: Vec<TagStoreRef>,
        pub qualities: Vec<QualityStoreRef>,
        pub props: Vec<PropStoreRef>,
        pub attributes: Vec<AttributeStoreRef>,
        /// Broadcast bus for graph change notifications (kit holds strong ref).
        pub(crate) event_bus: Arc<EventBus>,
        hash_cache: Cache<String>,
        validation_cache: Cache<ValidationResult>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct KitIdDto {
        pub guid: Guid,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct KitMetadataDto {
        pub guid: Guid,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub image: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub preview: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub remote: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub homepage: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub license: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub uri: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub created: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub updated: Option<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct KitShallowDto {
        pub guid: Guid,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub image: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub preview: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub remote: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub homepage: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub license: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub uri: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub created: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub updated: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub types: Vec<crate::typ::TypeShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub designs: Vec<crate::design::DesignShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub files: Vec<crate::file::FileShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub folders: Vec<crate::folder::FolderShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub authors: Vec<AuthorShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub concepts: Vec<ConceptShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tags: Vec<TagShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub qualities: Vec<QualityShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub props: Vec<PropShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub attributes: Vec<AttributeShallowDto>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct KitFullDto {
        pub guid: Guid,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub image: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub preview: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub remote: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub homepage: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub license: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub uri: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub created: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub updated: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub types: Vec<TypeFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub designs: Vec<DesignFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub files: Vec<FileFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub folders: Vec<FolderFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub authors: Vec<AuthorFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub concepts: Vec<ConceptFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tags: Vec<TagFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub qualities: Vec<QualityFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub props: Vec<PropFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub attributes: Vec<AttributeFullDto>,
    }

    impl KitStore {
        pub fn new(name: impl Into<String>) -> Self {
            Self {
                guid: Guid::new_v7(),
                name: name.into(),
                description: None,
                icon: None,
                image: None,
                preview: None,
                version: None,
                remote: None,
                homepage: None,
                license: None,
                uri: None,
                created: None,
                updated: None,
                types: Vec::new(),
                designs: Vec::new(),
                files: Vec::new(),
                folders: Vec::new(),
                authors: Vec::new(),
                concepts: Vec::new(),
                tags: Vec::new(),
                qualities: Vec::new(),
                props: Vec::new(),
                attributes: Vec::new(),
                event_bus: EventBus::new(4096),
                hash_cache: Cache::default(),
                validation_cache: Cache::default(),
            }
        }

        #[inline]
        fn emit_ev(&self, ev: KitEvent) {
            self.event_bus.emit(ev);
        }

        fn entity_ref(&self) -> EntityRef {
            EntityRef::new(EntityKind::Kit, self.guid.clone())
        }

        /// Subscribe to all [`KitEvent`]s for this kit (MPMC broadcast).
        pub fn subscribe(&self) -> Receiver<KitEvent> {
            self.event_bus.subscribe()
        }

        pub fn invalidate_hash(&self) {
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
        }

        pub fn invalidate_validation(&self) {
            self.validation_cache.invalidate();
            self.emit_ev(KitEvent::ValidationInvalidated);
        }

        pub fn set_name(&mut self, name: String) -> crate::error::SetResult {
            let name = name.trim().to_string();
            if let Err(e) = crate::validate::required_name(&name, "name") {
                self.emit_ev(KitEvent::SetRejected {
                    entity: self.entity_ref(),
                    field: "name".into(),
                    error: e.clone(),
                });
                return Err(e);
            }
            if self.name == name {
                return Ok(());
            }
            self.name = name;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "name",
            });
            self.invalidate_hash();
            self.invalidate_validation();
            Ok(())
        }

        pub fn set_description(&mut self, v: Option<String>) -> crate::error::SetResult {
            let v = match v {
                None => None,
                Some(s) if s.trim().is_empty() => None,
                Some(s) => Some(s.trim().to_string()),
            };
            if self.description == v {
                return Ok(());
            }
            self.description = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "description",
            });
            self.invalidate_hash();
            self.invalidate_validation();
            Ok(())
        }

        pub fn set_icon(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.icon == v {
                return Ok(());
            }
            self.icon = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "icon",
            });
            self.invalidate_hash();
            self.invalidate_validation();
            Ok(())
        }

        pub fn set_image(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.image == v {
                return Ok(());
            }
            self.image = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "image",
            });
            self.invalidate_hash();
            self.invalidate_validation();
            Ok(())
        }

        pub fn set_preview(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.preview == v {
                return Ok(());
            }
            self.preview = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "preview",
            });
            self.invalidate_hash();
            self.invalidate_validation();
            Ok(())
        }

        pub fn set_version(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.version == v {
                return Ok(());
            }
            self.version = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "version",
            });
            self.invalidate_hash();
            self.invalidate_validation();
            Ok(())
        }

        pub fn set_remote(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.remote == v {
                return Ok(());
            }
            self.remote = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "remote",
            });
            self.invalidate_hash();
            self.invalidate_validation();
            Ok(())
        }

        pub fn set_homepage(&mut self, v: Option<String>) -> crate::error::SetResult {
            if let Err(e) = crate::validate::optional_url(&v, "homepage") {
                self.emit_ev(KitEvent::SetRejected {
                    entity: self.entity_ref(),
                    field: "homepage".into(),
                    error: e.clone(),
                });
                return Err(e);
            }
            if self.homepage == v {
                return Ok(());
            }
            self.homepage = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "homepage",
            });
            self.invalidate_hash();
            self.invalidate_validation();
            Ok(())
        }

        pub fn set_license(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.license == v {
                return Ok(());
            }
            self.license = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "license",
            });
            self.invalidate_hash();
            self.invalidate_validation();
            Ok(())
        }

        pub fn set_uri(&mut self, v: Option<String>) -> crate::error::SetResult {
            if let Err(e) = crate::validate::optional_opaque_uri(&v, "uri") {
                self.emit_ev(KitEvent::SetRejected {
                    entity: self.entity_ref(),
                    field: "uri".into(),
                    error: e.clone(),
                });
                return Err(e);
            }
            if self.uri == v {
                return Ok(());
            }
            self.uri = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "uri",
            });
            self.invalidate_hash();
            self.invalidate_validation();
            Ok(())
        }

        pub fn set_created(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.created == v {
                return Ok(());
            }
            self.created = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "created",
            });
            self.invalidate_hash();
            self.invalidate_validation();
            Ok(())
        }

        pub fn set_updated(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.updated == v {
                return Ok(());
            }
            self.updated = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "updated",
            });
            self.invalidate_hash();
            self.invalidate_validation();
            Ok(())
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
            w.tag("kit")
                .str(self.guid.as_str())
                .str(&self.name)
                .opt_str(self.description.as_deref())
                .opt_str(self.version.as_deref())
                .opt_str(self.license.as_deref());
            for t in &self.types {
                if let Ok(t) = t.read() {
                    t.hash_into(w);
                }
            }
            for d in &self.designs {
                if let Ok(d) = d.read() {
                    d.hash_into(w);
                }
            }
            for f in &self.files {
                if let Ok(f) = f.read() {
                    f.hash_into(w);
                }
            }
            for f in &self.folders {
                if let Ok(f) = f.read() {
                    f.hash_into(w);
                }
            }
            for a in &self.authors {
                if let Ok(a) = a.read() {
                    a.hash_into(w);
                }
            }
            for c in &self.concepts {
                if let Ok(c) = c.read() {
                    c.hash_into(w);
                }
            }
            for t in &self.tags {
                if let Ok(t) = t.read() {
                    t.hash_into(w);
                }
            }
            for q in &self.qualities {
                if let Ok(q) = q.read() {
                    q.hash_into(w);
                }
            }
            for p in &self.props {
                if let Ok(p) = p.read() {
                    p.hash_into(w);
                }
            }
            for a in &self.attributes {
                if let Ok(a) = a.read() {
                    a.hash_into(w);
                }
            }
        }

        pub fn semio_type(&self, guid: &str) -> Option<TypeStoreRef> {
            self.types
                .iter()
                .find(|t| t.read().map(|t| t.guid.as_str() == guid).unwrap_or(false))
                .cloned()
        }

        pub fn design(&self, guid: &str) -> Option<DesignStoreRef> {
            self.designs
                .iter()
                .find(|d| d.read().map(|d| d.guid.as_str() == guid).unwrap_or(false))
                .cloned()
        }

        pub fn file(&self, guid: &str) -> Option<FileStoreRef> {
            self.files
                .iter()
                .find(|f| f.read().map(|f| f.guid.as_str() == guid).unwrap_or(false))
                .cloned()
        }

        pub fn folder(&self, guid: &str) -> Option<FolderStoreRef> {
            self.folders
                .iter()
                .find(|f| f.read().map(|f| f.guid.as_str() == guid).unwrap_or(false))
                .cloned()
        }

        pub fn quality(&self, guid: &str) -> Option<QualityStoreRef> {
            self.qualities
                .iter()
                .find(|q| q.read().map(|q| q.guid.as_str() == guid).unwrap_or(false))
                .cloned()
        }

        /// Flatten a design by guid: returns a report with a [`crate::diff::DesignChange`] describing pose updates.
        pub fn flatten_design(
            &self,
            design_guid: &str,
        ) -> Result<SemioReport<crate::diff::DesignChange>> {
            let d = self
                .design(design_guid)
                .ok_or_else(|| SemioError::NotFound {
                    kind: "Design",
                    guid: Guid::from(design_guid),
                })?;
            let report = match d.read() {
                Ok(dr) => dr.flatten_change(),
                Err(_) => return Err(SemioError::LockPoisoned("design")),
            };
            Ok(report)
        }

        /// Apply a structural [`crate::diff::DesignDiff`] to the named design (mutable kit).
        pub fn apply_design_diff(
            &mut self,
            design_guid: &str,
            diff: &crate::diff::DesignDiff,
        ) -> Result<()> {
            let dref = self
                .design(design_guid)
                .ok_or_else(|| SemioError::NotFound {
                    kind: "Design",
                    guid: Guid::from(design_guid),
                })?;
            let type_index: HashMap<Guid, TypeStoreRef> = self
                .types
                .iter()
                .filter_map(|t| t.read().ok().map(|r| (r.guid.clone(), t.clone())))
                .collect();
            let dw = Arc::downgrade(&dref);
            dref.write()
                .map_err(|_| SemioError::LockPoisoned("design"))?
                .apply_diff(diff, &type_index, dw)?;
            self.invalidate_hash();
            self.invalidate_validation();
            Ok(())
        }

        /// 🌐 Parse entity kind label from JS (`"Piece"`, `"Kit"`, …).
        pub fn parse_entity_kind(s: &str) -> std::result::Result<EntityKind, SetError> {
            match s {
                "Kit" => Ok(EntityKind::Kit),
                "Type" => Ok(EntityKind::Type),
                "Design" => Ok(EntityKind::Design),
                "Piece" => Ok(EntityKind::Piece),
                "Connection" => Ok(EntityKind::Connection),
                "Side" => Ok(EntityKind::Side),
                "Port" => Ok(EntityKind::Port),
                "Connector" => Ok(EntityKind::Connector),
                "Representation" => Ok(EntityKind::Representation),
                "File" => Ok(EntityKind::File),
                "Folder" => Ok(EntityKind::Folder),
                "Layer" => Ok(EntityKind::Layer),
                "Group" => Ok(EntityKind::Group),
                "Author" => Ok(EntityKind::Author),
                "Concept" => Ok(EntityKind::Concept),
                "Tag" => Ok(EntityKind::Tag),
                "Prop" => Ok(EntityKind::Prop),
                "Attribute" => Ok(EntityKind::Attribute),
                "Quality" => Ok(EntityKind::Quality),
                "Stat" => Ok(EntityKind::Stat),
                "Benchmark" => Ok(EntityKind::Benchmark),
                _ => Err(SetError::InvalidValue(format!("unknown entity kind '{s}'"))),
            }
        }

        /// 🌐 Worker boundary: set one scalar field (extend match as hooks grow).
        pub fn set_field_rpc(
            kit: &KitStoreRef,
            entity_kind: EntityKind,
            guid: &str,
            field: &str,
            value: serde_json::Value,
        ) -> SetResult {
            match entity_kind {
                EntityKind::Kit => {
                    let mut g = kit
                        .write()
                        .map_err(|_| SetError::LockPoisoned("kit".into()))?;
                    if g.guid.as_str() != guid {
                        return Err(SetError::NotFound(format!("kit {guid}")));
                    }
                    match field {
                        "name" => {
                            let s: String = serde_json::from_value(value)
                                .map_err(|e| SetError::InvalidValue(e.to_string()))?;
                            g.set_name(s)
                        }
                        _ => Err(SetError::InvalidValue(format!("unknown kit field '{field}'"))),
                    }
                }
                EntityKind::Design => {
                    let d = {
                        let g = kit
                            .read()
                            .map_err(|_| SetError::LockPoisoned("kit".into()))?;
                        g.design(guid)
                            .ok_or_else(|| SetError::NotFound(format!("design {guid}")))?
                    };
                    let mut dw = d
                        .write()
                        .map_err(|_| SetError::LockPoisoned("design".into()))?;
                    match field {
                        "name" => {
                            let s: String = serde_json::from_value(value)
                                .map_err(|e| SetError::InvalidValue(e.to_string()))?;
                            dw.set_name(s)
                        }
                        _ => Err(SetError::InvalidValue(format!(
                            "unknown design field '{field}'"
                        ))),
                    }
                }
                EntityKind::Type => {
                    let t = {
                        let g = kit
                            .read()
                            .map_err(|_| SetError::LockPoisoned("kit".into()))?;
                        g.semio_type(guid)
                            .ok_or_else(|| SetError::NotFound(format!("type {guid}")))?
                    };
                    let mut tw = t
                        .write()
                        .map_err(|_| SetError::LockPoisoned("type".into()))?;
                    match field {
                        "name" => {
                            let s: String = serde_json::from_value(value)
                                .map_err(|e| SetError::InvalidValue(e.to_string()))?;
                            tw.set_name(s)
                        }
                        _ => Err(SetError::InvalidValue(format!("unknown type field '{field}'"))),
                    }
                }
                EntityKind::Piece => {
                    let pref = {
                        let g = kit
                            .read()
                            .map_err(|_| SetError::LockPoisoned("kit".into()))?;
                        let mut found: Option<PieceStoreRef> = None;
                        for d in &g.designs {
                            if let Ok(dr) = d.read() {
                                if let Some(p) = dr.piece(guid) {
                                    found = Some(p);
                                    break;
                                }
                            }
                        }
                        found.ok_or_else(|| SetError::NotFound(format!("piece {guid}")))?
                    };
                    let mut pw = pref
                        .write()
                        .map_err(|_| SetError::LockPoisoned("piece".into()))?;
                    match field {
                        "name" => {
                            let v: Option<String> = serde_json::from_value(value)
                                .map_err(|e| SetError::InvalidValue(e.to_string()))?;
                            pw.set_name(v)
                        }
                        "color" => {
                            let v: Option<String> = serde_json::from_value(value)
                                .map_err(|e| SetError::InvalidValue(e.to_string()))?;
                            pw.set_color(v)
                        }
                        _ => Err(SetError::InvalidValue(format!(
                            "unknown piece field '{field}'"
                        ))),
                    }
                }
                _ => Err(SetError::InvalidValue(format!(
                    "set_field_rpc not implemented for {entity_kind:?}"
                ))),
            }
        }

        /// 🌐 Read one field as JSON (read-only).
        pub fn get_field_rpc(
            kit: &KitStoreRef,
            entity_kind: EntityKind,
            guid: &str,
            field: &str,
        ) -> std::result::Result<serde_json::Value, SetError> {
            let g = kit
                .read()
                .map_err(|_| SetError::LockPoisoned("kit".into()))?;
            match entity_kind {
                EntityKind::Kit => {
                    if g.guid.as_str() != guid {
                        return Err(SetError::NotFound(format!("kit {guid}")));
                    }
                    match field {
                        "name" => Ok(serde_json::json!(g.name)),
                        _ => Err(SetError::InvalidValue(format!("unknown kit field '{field}'"))),
                    }
                }
                EntityKind::Piece => {
                    for d in &g.designs {
                        if let Ok(dr) = d.read() {
                            if let Some(p) = dr.piece(guid) {
                                if let Ok(pr) = p.read() {
                                    return match field {
                                        "name" => Ok(serde_json::to_value(&pr.name).unwrap()),
                                        "color" => Ok(serde_json::to_value(&pr.color).unwrap()),
                                        _ => Err(SetError::InvalidValue(format!(
                                            "unknown piece field '{field}'"
                                        ))),
                                    };
                                }
                            }
                        }
                    }
                    Err(SetError::NotFound(format!("piece {guid}")))
                }
                EntityKind::Design => {
                    let d = g
                        .design(guid)
                        .ok_or_else(|| SetError::NotFound(format!("design {guid}")))?;
                    let dr = d
                        .read()
                        .map_err(|_| SetError::LockPoisoned("design".into()))?;
                    match field {
                        "name" => Ok(serde_json::json!(dr.name)),
                        _ => Err(SetError::InvalidValue(format!(
                            "unknown design field '{field}'"
                        ))),
                    }
                }
                EntityKind::Type => {
                    let t = g
                        .semio_type(guid)
                        .ok_or_else(|| SetError::NotFound(format!("type {guid}")))?;
                    let tr = t
                        .read()
                        .map_err(|_| SetError::LockPoisoned("type".into()))?;
                    match field {
                        "name" => Ok(serde_json::json!(tr.name)),
                        _ => Err(SetError::InvalidValue(format!(
                            "unknown type field '{field}'"
                        ))),
                    }
                }
                _ => Err(SetError::InvalidValue(format!(
                    "get_field_rpc not implemented for {entity_kind:?}"
                ))),
            }
        }

        fn map_semio_err(e: SemioError) -> SetError {
            match e {
                SemioError::NotFound { kind, guid } => {
                    SetError::NotFound(format!("{} {}", kind, guid.as_str()))
                }
                SemioError::LockPoisoned(s) => SetError::LockPoisoned(s.to_string()),
                SemioError::InvalidOperation(m) => SetError::Internal(m),
                SemioError::Json(j) => SetError::InvalidValue(j.to_string()),
                SemioError::Io(i) => SetError::Internal(i.to_string()),
                SemioError::Other(o) => SetError::Internal(o),
                #[cfg(not(target_arch = "wasm32"))]
                SemioError::Sqlite(s) => SetError::Internal(s.to_string()),
                #[cfg(not(target_arch = "wasm32"))]
                SemioError::Zip(z) => SetError::Internal(z.to_string()),
            }
        }

        /// Apply a structural design diff through the same path as [`KitStore::apply_design_diff`], returning [`SetResult`].
        pub fn apply_design_diff_rpc(
            kit: &KitStoreRef,
            design_guid: &str,
            diff: serde_json::Value,
        ) -> SetResult {
            let diff: DesignDiff = serde_json::from_value(diff)
                .map_err(|e| SetError::InvalidValue(e.to_string()))?;
            let mut g = kit
                .write()
                .map_err(|_| SetError::LockPoisoned("kit".into()))?;
            g.apply_design_diff(design_guid, &diff)
                .map_err(Self::map_semio_err)
        }

        /// Add a child entity under `parent` (currently `Design → Piece` only).
        pub fn add_child_rpc(
            kit: &KitStoreRef,
            parent_kind: EntityKind,
            parent_guid: &str,
            child_kind: EntityKind,
            dto: serde_json::Value,
        ) -> SetResult {
            match (parent_kind, child_kind) {
                (EntityKind::Design, EntityKind::Piece) => {
                    let piece: PieceFullDto = serde_json::from_value(dto)
                        .map_err(|e| SetError::InvalidValue(e.to_string()))?;
                    let mut diff = DesignDiff::default();
                    diff.added_pieces.push(piece);
                    let mut g = kit
                        .write()
                        .map_err(|_| SetError::LockPoisoned("kit".into()))?;
                    g.apply_design_diff(parent_guid, &diff)
                        .map_err(Self::map_semio_err)
                }
                _ => Err(SetError::InvalidValue(format!(
                    "add_child_rpc not implemented for {parent_kind:?} -> {child_kind:?}"
                ))),
            }
        }

        /// Remove a child entity from `parent` (currently `Design → Piece` only).
        pub fn remove_child_rpc(
            kit: &KitStoreRef,
            parent_kind: EntityKind,
            parent_guid: &str,
            child_kind: EntityKind,
            child_guid: &str,
        ) -> SetResult {
            match (parent_kind, child_kind) {
                (EntityKind::Design, EntityKind::Piece) => {
                    let mut diff = DesignDiff::default();
                    diff.removed_pieces.push(PieceIdDto {
                        guid: Guid::from(child_guid),
                    });
                    let mut g = kit
                        .write()
                        .map_err(|_| SetError::LockPoisoned("kit".into()))?;
                    g.apply_design_diff(parent_guid, &diff)
                        .map_err(Self::map_semio_err)
                }
                _ => Err(SetError::InvalidValue(format!(
                    "remove_child_rpc not implemented for {parent_kind:?} -> {child_kind:?}"
                ))),
            }
        }

        pub fn validate(&self) -> ValidationResult {
            self.validation_cache
                .get_or_init(|| self.compute_validation())
        }

        fn compute_validation(&self) -> ValidationResult {
            let mut result = ValidationResult::valid();
            if self.name.trim().is_empty() {
                result.is_valid = false;
                result.errors.push("kit.name must not be empty".into());
            }

            let mut guids: Vec<String> = Vec::new();
            guids.push(self.guid.as_str().to_string());

            for t in &self.types {
                if let Ok(t) = t.read() {
                    if t.name.trim().is_empty() {
                        result.is_valid = false;
                        result
                            .errors
                            .push(format!("type {} has empty name", t.guid));
                    }
                    guids.push(t.guid.as_str().to_string());
                    for p in &t.ports {
                        if let Ok(p) = p.read() {
                            guids.push(p.guid.as_str().to_string());
                        }
                    }
                    for c in &t.connectors {
                        if let Ok(c) = c.read() {
                            guids.push(c.guid.as_str().to_string());
                        }
                    }
                    for r in &t.representations {
                        if let Ok(r) = r.read() {
                            if r.url.trim().is_empty() {
                                result.is_valid = false;
                                result
                                    .errors
                                    .push(format!("representation {} has empty url", r.guid));
                            }
                            guids.push(r.guid.as_str().to_string());
                        }
                    }
                }
            }

            for f in &self.files {
                if let Ok(f) = f.read() {
                    if f.url.trim().is_empty() {
                        result.is_valid = false;
                        result.errors.push(format!("file {} has empty url", f.guid));
                    }
                    guids.push(f.guid.as_str().to_string());
                }
            }

            for d in &self.designs {
                if let Ok(d) = d.read() {
                    if d.name.trim().is_empty() {
                        result.is_valid = false;
                        result
                            .errors
                            .push(format!("design {} has empty name", d.guid));
                    }
                    guids.push(d.guid.as_str().to_string());
                    for p in &d.pieces {
                        if let Ok(p) = p.read() {
                            if p.type_ref.as_ref().and_then(|t| t.upgrade()).is_none() {
                                result.is_valid = false;
                                result
                                    .errors
                                    .push(format!("piece {} has no valid type reference", p.guid));
                            }
                            guids.push(p.guid.as_str().to_string());
                        }
                    }
                    for c in &d.connections {
                        if let Ok(conn) = c.read() {
                            guids.push(conn.guid.as_str().to_string());
                            if let Ok(s0) = conn.connected.read() {
                                if s0.piece.upgrade().is_none() {
                                    result.is_valid = false;
                                    result.errors.push(format!(
                                        "connection {} connected side has no piece",
                                        conn.guid
                                    ));
                                }
                            }
                            if let Ok(s1) = conn.connecting.read() {
                                if s1.piece.upgrade().is_none() {
                                    result.is_valid = false;
                                    result.errors.push(format!(
                                        "connection {} connecting side has no piece",
                                        conn.guid
                                    ));
                                }
                            }
                        }
                    }
                }
            }

            guids.sort();
            for w in guids.windows(2) {
                if w[0] == w[1] {
                    result.is_valid = false;
                    result.errors.push(format!("duplicate guid: {}", w[0]));
                    break;
                }
            }

            // Simple cycle heuristic: in each design, if there are more connections than pieces, flag.
            for d in &self.designs {
                if let Ok(d) = d.read() {
                    if d.connections.len() > d.pieces.len() && !d.pieces.is_empty() {
                        result.is_valid = false;
                        result.warnings.push(format!(
                        "design {} has more connections than pieces (possible cycle or bad graph)",
                        d.guid
                    ));
                    }
                }
            }

            result
        }

        pub fn are_equal(&self, other: &KitStore) -> bool {
            self.hash() == other.hash()
        }

        pub fn from_id_dto(d: KitIdDto) -> Self {
            Self {
                guid: d.guid,
                name: String::new(),
                description: None,
                icon: None,
                image: None,
                preview: None,
                version: None,
                remote: None,
                homepage: None,
                license: None,
                uri: None,
                created: None,
                updated: None,
                types: Vec::new(),
                designs: Vec::new(),
                files: Vec::new(),
                folders: Vec::new(),
                authors: Vec::new(),
                concepts: Vec::new(),
                tags: Vec::new(),
                qualities: Vec::new(),
                props: Vec::new(),
                attributes: Vec::new(),
                event_bus: EventBus::new(4096),
                hash_cache: Cache::default(),
                validation_cache: Cache::default(),
            }
        }

        pub fn from_metadata_dto(d: KitMetadataDto) -> Self {
            Self {
                guid: d.guid,
                name: d.name,
                description: d.description,
                icon: d.icon,
                image: d.image,
                preview: d.preview,
                version: d.version,
                remote: d.remote,
                homepage: d.homepage,
                license: d.license,
                uri: d.uri,
                created: d.created,
                updated: d.updated,
                types: Vec::new(),
                designs: Vec::new(),
                files: Vec::new(),
                folders: Vec::new(),
                authors: Vec::new(),
                concepts: Vec::new(),
                tags: Vec::new(),
                qualities: Vec::new(),
                props: Vec::new(),
                attributes: Vec::new(),
                event_bus: EventBus::new(4096),
                hash_cache: Cache::default(),
                validation_cache: Cache::default(),
            }
        }

        /// Hydrate the full kit graph from a [`KitFullDto`].
        pub fn from_full_dto(d: KitFullDto) -> KitStoreRef {
            let KitFullDto {
                guid,
                name,
                description,
                icon,
                image,
                preview,
                version,
                remote,
                homepage,
                license,
                uri,
                created,
                updated,
                types,
                designs,
                files,
                folders,
                authors,
                concepts,
                tags,
                qualities,
                props,
                attributes,
            } = d;

            let kit = Arc::new(RwLock::new(KitStore {
                guid: guid.clone(),
                name: name.clone(),
                description: description.clone(),
                icon: icon.clone(),
                image: image.clone(),
                preview: preview.clone(),
                version: version.clone(),
                remote: remote.clone(),
                homepage: homepage.clone(),
                license: license.clone(),
                uri: uri.clone(),
                created: created.clone(),
                updated: updated.clone(),
                types: Vec::new(),
                designs: Vec::new(),
                files: Vec::new(),
                folders: Vec::new(),
                authors: authors
                    .into_iter()
                    .map(|a| Arc::new(RwLock::new(AuthorStore::from_full_dto(a))))
                    .collect(),
                concepts: concepts
                    .into_iter()
                    .map(|c| Arc::new(RwLock::new(ConceptStore::from_full_dto(c))))
                    .collect(),
                tags: tags
                    .into_iter()
                    .map(|t| Arc::new(RwLock::new(TagStore::from_full_dto(t))))
                    .collect(),
                qualities: qualities
                    .into_iter()
                    .map(|q| Arc::new(RwLock::new(QualityStore::from_full_dto(q))))
                    .collect(),
                props: props
                    .into_iter()
                    .map(|p| Arc::new(RwLock::new(PropStore::from_full_dto(p))))
                    .collect(),
                attributes: attributes
                    .into_iter()
                    .map(|a| Arc::new(RwLock::new(AttributeStore::from_full_dto(a))))
                    .collect(),
                event_bus: EventBus::new(4096),
                hash_cache: Cache::default(),
                validation_cache: Cache::default(),
            }));

            let kw = Arc::downgrade(&kit);
            if let Ok(k) = kit.write() {
                for a in &k.authors {
                    if let Ok(mut aw) = a.write() {
                        aw.parent_kit = Some(kw.clone());
                    }
                }
                for c in &k.concepts {
                    if let Ok(mut cw) = c.write() {
                        cw.parent_kit = Some(kw.clone());
                    }
                }
                for t in &k.tags {
                    if let Ok(mut tw) = t.write() {
                        tw.parent_kit = Some(kw.clone());
                    }
                }
                for q in &k.qualities {
                    if let Ok(mut qw) = q.write() {
                        qw.parent_kit = Some(kw.clone());
                    }
                }
                for p in &k.props {
                    if let Ok(mut pw) = p.write() {
                        pw.parent_kit = Some(kw.clone());
                    }
                }
                for a in &k.attributes {
                    if let Ok(mut aw) = a.write() {
                        aw.parent_kit = Some(kw.clone());
                    }
                }
            }

            let file_refs: Vec<FileStoreRef> = files
                .into_iter()
                .map(|f| Arc::new(RwLock::new(FileStore::from_full_dto(f))))
                .collect();
            let folder_refs: Vec<FolderStoreRef> = folders
                .into_iter()
                .map(|f| Arc::new(RwLock::new(FolderStore::from_full_dto(f))))
                .collect();

            let mut type_refs: Vec<TypeStoreRef> = Vec::with_capacity(types.len());
            let mut type_index: HashMap<Guid, TypeStoreRef> = HashMap::new();
            for tdto in types {
                let t = TypeStore::hydrate_from_full_dto(tdto, &kit, &file_refs);
                if let Ok(tr) = t.read() {
                    type_index.insert(tr.guid.clone(), t.clone());
                }
                type_refs.push(t);
            }

            let design_refs: Vec<DesignStoreRef> = designs
                .into_iter()
                .map(|ddto| {
                    let design = DesignStore::hydrate_from_full_dto(ddto, &type_index);
                    if let Ok(mut dw) = design.write() {
                        dw.parent_kit = Arc::downgrade(&kit);
                    }
                    design
                })
                .collect();

            if let Ok(mut k) = kit.write() {
                let kw = Arc::downgrade(&kit);
                for f in &file_refs {
                    if let Ok(mut fw) = f.write() {
                        fw.parent_kit = Some(kw.clone());
                    }
                }
                for f in &folder_refs {
                    if let Ok(mut fw) = f.write() {
                        fw.parent_kit = Some(kw.clone());
                    }
                }
                k.types = type_refs;
                k.designs = design_refs;
                k.files = file_refs;
                k.folders = folder_refs;
            }
            event_wire::wire_graph_bus(&kit);
            kit
        }

        pub fn to_id_dto(&self) -> KitIdDto {
            KitIdDto {
                guid: self.guid.clone(),
            }
        }

        pub fn to_metadata_dto(&self) -> KitMetadataDto {
            KitMetadataDto {
                guid: self.guid.clone(),
                name: self.name.clone(),
                description: self.description.clone(),
                icon: self.icon.clone(),
                image: self.image.clone(),
                preview: self.preview.clone(),
                version: self.version.clone(),
                remote: self.remote.clone(),
                homepage: self.homepage.clone(),
                license: self.license.clone(),
                uri: self.uri.clone(),
                created: self.created.clone(),
                updated: self.updated.clone(),
            }
        }

        pub fn to_shallow_dto(&self) -> KitShallowDto {
            let m = self.to_metadata_dto();
            KitShallowDto {
                guid: m.guid,
                name: m.name,
                description: m.description,
                icon: m.icon,
                image: m.image,
                preview: m.preview,
                version: m.version,
                remote: m.remote,
                homepage: m.homepage,
                license: m.license,
                uri: m.uri,
                created: m.created,
                updated: m.updated,
                types: self
                    .types
                    .iter()
                    .filter_map(|t| t.read().ok().map(|t| t.to_shallow_dto()))
                    .collect(),
                designs: self
                    .designs
                    .iter()
                    .filter_map(|d| d.read().ok().map(|d| d.to_shallow_dto()))
                    .collect(),
                files: self
                    .files
                    .iter()
                    .filter_map(|f| f.read().ok().map(|f| f.to_shallow_dto()))
                    .collect(),
                folders: self
                    .folders
                    .iter()
                    .filter_map(|f| f.read().ok().map(|f| f.to_shallow_dto()))
                    .collect(),
                authors: self
                    .authors
                    .iter()
                    .filter_map(|a| a.read().ok().map(|a| a.to_shallow_dto()))
                    .collect(),
                concepts: self
                    .concepts
                    .iter()
                    .filter_map(|c| c.read().ok().map(|c| c.to_shallow_dto()))
                    .collect(),
                tags: self
                    .tags
                    .iter()
                    .filter_map(|t| t.read().ok().map(|t| t.to_shallow_dto()))
                    .collect(),
                qualities: self
                    .qualities
                    .iter()
                    .filter_map(|q| q.read().ok().map(|q| q.to_shallow_dto()))
                    .collect(),
                props: self
                    .props
                    .iter()
                    .filter_map(|p| p.read().ok().map(|p| p.to_shallow_dto()))
                    .collect(),
                attributes: self
                    .attributes
                    .iter()
                    .filter_map(|a| a.read().ok().map(|a| a.to_shallow_dto()))
                    .collect(),
            }
        }

        pub fn to_full_dto(&self) -> KitFullDto {
            let m = self.to_metadata_dto();
            KitFullDto {
                guid: m.guid,
                name: m.name,
                description: m.description,
                icon: m.icon,
                image: m.image,
                preview: m.preview,
                version: m.version,
                remote: m.remote,
                homepage: m.homepage,
                license: m.license,
                uri: m.uri,
                created: m.created,
                updated: m.updated,
                types: self
                    .types
                    .iter()
                    .filter_map(|t| t.read().ok().map(|t| t.to_full_dto()))
                    .collect(),
                designs: self
                    .designs
                    .iter()
                    .filter_map(|d| d.read().ok().map(|d| d.to_full_dto()))
                    .collect(),
                files: self
                    .files
                    .iter()
                    .filter_map(|f| f.read().ok().map(|f| f.to_full_dto()))
                    .collect(),
                folders: self
                    .folders
                    .iter()
                    .filter_map(|f| f.read().ok().map(|f| f.to_full_dto()))
                    .collect(),
                authors: self
                    .authors
                    .iter()
                    .filter_map(|a| a.read().ok().map(|a| a.to_full_dto()))
                    .collect(),
                concepts: self
                    .concepts
                    .iter()
                    .filter_map(|c| c.read().ok().map(|c| c.to_full_dto()))
                    .collect(),
                tags: self
                    .tags
                    .iter()
                    .filter_map(|t| t.read().ok().map(|t| t.to_full_dto()))
                    .collect(),
                qualities: self
                    .qualities
                    .iter()
                    .filter_map(|q| q.read().ok().map(|q| q.to_full_dto()))
                    .collect(),
                props: self
                    .props
                    .iter()
                    .filter_map(|p| p.read().ok().map(|p| p.to_full_dto()))
                    .collect(),
                attributes: self
                    .attributes
                    .iter()
                    .filter_map(|a| a.read().ok().map(|a| a.to_full_dto()))
                    .collect(),
            }
        }
    }
}

pub mod layer {
    use serde::{Deserialize, Serialize};
    use std::sync::{Arc, RwLock, Weak};

    use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
    use crate::guid::Guid;
    use crate::hash::{Cache, HashWriter};

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
        pub(crate) event_bus: Weak<EventBus>,
        hash_cache: Cache<String>,
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
    pub struct LayerFullDto {
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
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
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
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
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
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        pub fn from_shallow_dto(d: LayerShallowDto) -> Self {
            Self::from_metadata_dto(LayerMetadataDto {
                guid: d.guid,
                name: d.name,
                description: d.description,
                color: d.color,
                order: d.order,
                visible: d.visible,
                locked: d.locked,
            })
        }

        pub fn from_full_dto(d: LayerFullDto) -> Self {
            Self::from_metadata_dto(LayerMetadataDto {
                guid: d.guid,
                name: d.name,
                description: d.description,
                color: d.color,
                order: d.order,
                visible: d.visible,
                locked: d.locked,
            })
        }

        pub fn to_id_dto(&self) -> LayerIdDto {
            LayerIdDto {
                guid: self.guid.clone(),
            }
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
            let m = self.to_metadata_dto();
            LayerShallowDto {
                guid: m.guid,
                name: m.name,
                description: m.description,
                color: m.color,
                order: m.order,
                visible: m.visible,
                locked: m.locked,
            }
        }

        pub fn to_full_dto(&self) -> LayerFullDto {
            let m = self.to_metadata_dto();
            LayerFullDto {
                guid: m.guid,
                name: m.name,
                description: m.description,
                color: m.color,
                order: m.order,
                visible: m.visible,
                locked: m.locked,
            }
        }

        #[inline]
        fn emit_ev(&self, ev: KitEvent) {
            emit_weak(&self.event_bus, ev);
        }

        fn entity_ref(&self) -> EntityRef {
            EntityRef::new(EntityKind::Layer, self.guid.clone())
        }

        pub fn set_name(&mut self, name: String) -> crate::error::SetResult {
            if self.name == name {
                return Ok(());
            }
            self.name = name;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "name",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_description(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.description == v {
                return Ok(());
            }
            self.description = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "description",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_color(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.color == v {
                return Ok(());
            }
            self.color = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "color",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_order(&mut self, v: Option<i64>) -> crate::error::SetResult {
            if self.order == v {
                return Ok(());
            }
            self.order = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "order",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_visible(&mut self, v: Option<bool>) -> crate::error::SetResult {
            if self.visible == v {
                return Ok(());
            }
            self.visible = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "visible",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_locked(&mut self, v: Option<bool>) -> crate::error::SetResult {
            if self.locked == v {
                return Ok(());
            }
            self.locked = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "locked",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn invalidate_hash(&self) {
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            if let Some(d) = self.parent_design.upgrade() {
                if let Ok(dr) = d.read() {
                    dr.invalidate_hash();
                }
            }
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
}

pub mod piece {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::sync::{Arc, RwLock, Weak};

    use crate::attribute::{
        AttributeFullDto, AttributeShallowDto, AttributeStore, AttributeStoreRef,
    };
    use crate::design::DesignStoreWeak;
    use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
    use crate::geom::{Coord, Plane};
    use crate::guid::Guid;
    use crate::hash::{Cache, HashWriter};
    use crate::prop::{PropFullDto, PropShallowDto, PropStore, PropStoreRef};
    use crate::typ::{TypeIdDto, TypeStoreRef, TypeStoreWeak};

    pub type PieceStoreRef = Arc<RwLock<PieceStore>>;
    pub type PieceStoreWeak = Weak<RwLock<PieceStore>>;

    /// Placed instance of a [`crate::typ::TypeStore`] inside a [`crate::design::DesignStore`].
    #[derive(Debug)]
    pub struct PieceStore {
        pub guid: Guid,
        pub id: Option<String>,
        pub name: Option<String>,
        pub description: Option<String>,
        pub plane: Option<Plane>,
        pub center: Option<Coord>,
        pub scale: Option<f64>,
        pub mirror_plane: Option<Plane>,
        pub hidden: Option<bool>,
        pub locked: Option<bool>,
        pub color: Option<String>,
        pub props: Vec<PropStoreRef>,
        pub attributes: Vec<AttributeStoreRef>,
        pub type_ref: Option<TypeStoreWeak>,
        pub parent_design: DesignStoreWeak,
        pub(crate) event_bus: Weak<EventBus>,
        hash_cache: Cache<String>,
        flat_plane: Cache<Plane>,
        flat_center: Cache<Coord>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct PieceIdDto {
        pub guid: Guid,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct PieceMetadataDto {
        pub guid: Guid,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub plane: Option<Plane>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub center: Option<Coord>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scale: Option<f64>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            rename = "mirrorPlane"
        )]
        pub mirror_plane: Option<Plane>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hidden: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub locked: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub color: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
        pub r#type: Option<TypeIdDto>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub design: Option<crate::design::DesignIdDto>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct PieceShallowDto {
        pub guid: Guid,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub plane: Option<Plane>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub center: Option<Coord>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scale: Option<f64>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            rename = "mirrorPlane"
        )]
        pub mirror_plane: Option<Plane>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hidden: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub locked: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub color: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
        pub r#type: Option<TypeIdDto>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub design: Option<crate::design::DesignIdDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub props: Vec<PropShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub attributes: Vec<AttributeShallowDto>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct PieceFullDto {
        pub guid: Guid,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub plane: Option<Plane>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub center: Option<Coord>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scale: Option<f64>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            rename = "mirrorPlane"
        )]
        pub mirror_plane: Option<Plane>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hidden: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub locked: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub color: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
        pub r#type: Option<TypeIdDto>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub design: Option<crate::design::DesignIdDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub props: Vec<PropFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub attributes: Vec<AttributeFullDto>,
    }

    impl PieceStore {
        pub fn new() -> Self {
            Self {
                guid: Guid::new_v7(),
                id: None,
                name: None,
                description: None,
                plane: None,
                center: None,
                scale: None,
                mirror_plane: None,
                hidden: None,
                locked: None,
                color: None,
                props: Vec::new(),
                attributes: Vec::new(),
                type_ref: None,
                parent_design: Weak::new(),
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
                flat_plane: Cache::default(),
                flat_center: Cache::default(),
            }
        }

        pub(crate) fn empty_shell(guid: Guid) -> Self {
            Self {
                guid,
                id: None,
                name: None,
                description: None,
                plane: None,
                center: None,
                scale: None,
                mirror_plane: None,
                hidden: None,
                locked: None,
                color: None,
                props: Vec::new(),
                attributes: Vec::new(),
                type_ref: None,
                parent_design: Weak::new(),
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
                flat_plane: Cache::default(),
                flat_center: Cache::default(),
            }
        }

        #[inline]
        fn emit_ev(&self, ev: KitEvent) {
            emit_weak(&self.event_bus, ev);
        }

        fn entity_ref(&self) -> EntityRef {
            EntityRef::new(EntityKind::Piece, self.guid.clone())
        }

        pub(crate) fn apply_metadata_fields(&mut self, d: PieceMetadataDto) {
            self.guid = d.guid;
            self.id = d.id;
            self.name = d.name;
            self.description = d.description;
            self.plane = d.plane;
            self.center = d.center;
            self.scale = d.scale;
            self.mirror_plane = d.mirror_plane;
            self.hidden = d.hidden;
            self.locked = d.locked;
            self.color = d.color;
            self.hash_cache.invalidate();
            self.flat_plane.invalidate();
            self.flat_center.invalidate();
        }

        pub(crate) fn apply_full_dto(
            &mut self,
            d: PieceFullDto,
            design_weak: DesignStoreWeak,
            type_index: &HashMap<Guid, TypeStoreRef>,
        ) {
            self.apply_metadata_fields(PieceMetadataDto {
                guid: d.guid,
                id: d.id,
                name: d.name,
                description: d.description,
                plane: d.plane,
                center: d.center,
                scale: d.scale,
                mirror_plane: d.mirror_plane,
                hidden: d.hidden,
                locked: d.locked,
                color: d.color,
                r#type: d.r#type.clone(),
                design: d.design.clone(),
            });
            if let Some(tid) = d.r#type.as_ref().map(|t| t.guid.clone()) {
                if let Some(tr) = type_index.get(&tid) {
                    self.type_ref = Some(Arc::downgrade(tr));
                }
            }
            self.parent_design = design_weak;
            self.props = d
                .props
                .into_iter()
                .map(|p| Arc::new(RwLock::new(PropStore::from_full_dto(p))))
                .collect();
            self.attributes = d
                .attributes
                .into_iter()
                .map(|a| Arc::new(RwLock::new(AttributeStore::from_full_dto(a))))
                .collect();
        }

        pub fn invalidate_flat_pose(&self) {
            self.flat_plane.invalidate();
            self.flat_center.invalidate();
        }

        pub fn invalidate_hash(&self) {
            self.hash_cache.invalidate();
            self.invalidate_flat_pose();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            if let Some(d) = self.parent_design.upgrade() {
                if let Ok(dr) = d.read() {
                    dr.invalidate_hash();
                }
            }
        }

        fn bubble_design_flatten(&self) {
            if let Some(d) = self.parent_design.upgrade() {
                if let Ok(d) = d.read() {
                    d.invalidate_flatten_with_locked_piece(Some(self.guid.clone()));
                }
            }
        }

        pub fn set_plane(&mut self, plane: Option<Plane>) -> crate::error::SetResult {
            if self.plane == plane {
                return Ok(());
            }
            self.plane = plane;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "plane",
            });
            self.invalidate_hash();
            self.bubble_design_flatten();
            Ok(())
        }

        pub fn set_center(&mut self, center: Option<Coord>) -> crate::error::SetResult {
            if self.center == center {
                return Ok(());
            }
            self.center = center;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "center",
            });
            self.invalidate_hash();
            self.bubble_design_flatten();
            Ok(())
        }

        pub fn set_color(&mut self, color: Option<String>) -> crate::error::SetResult {
            if self.color == color {
                return Ok(());
            }
            self.color = color;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "color",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_type_weak(
            &mut self,
            type_ref: Option<TypeStoreWeak>,
        ) -> crate::error::SetResult {
            self.type_ref = type_ref;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "type",
            });
            self.invalidate_hash();
            self.bubble_design_flatten();
            Ok(())
        }

        pub fn set_id(&mut self, id: Option<String>) -> crate::error::SetResult {
            if self.id == id {
                return Ok(());
            }
            self.id = id;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "id",
            });
            self.invalidate_hash();
            self.bubble_design_flatten();
            Ok(())
        }

        pub fn set_name(&mut self, name: Option<String>) -> crate::error::SetResult {
            let name = match name {
                None => None,
                Some(s) if s.trim().is_empty() => None,
                Some(s) => Some(s.trim().to_string()),
            };
            if let Err(e) = crate::validate::optional_display_name(&name, "name") {
                self.emit_ev(KitEvent::SetRejected {
                    entity: self.entity_ref(),
                    field: "name".into(),
                    error: e.clone(),
                });
                return Err(e);
            }
            if self.name == name {
                return Ok(());
            }
            self.name = name;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "name",
            });
            self.invalidate_hash();
            self.bubble_design_flatten();
            Ok(())
        }

        pub fn set_description(&mut self, description: Option<String>) -> crate::error::SetResult {
            let description = match description {
                None => None,
                Some(s) if s.trim().is_empty() => None,
                Some(s) => Some(s.trim().to_string()),
            };
            if self.description == description {
                return Ok(());
            }
            self.description = description;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "description",
            });
            self.invalidate_hash();
            self.bubble_design_flatten();
            Ok(())
        }

        pub fn set_scale(&mut self, scale: Option<f64>) -> crate::error::SetResult {
            if self.scale == scale {
                return Ok(());
            }
            self.scale = scale;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "scale",
            });
            self.invalidate_hash();
            self.bubble_design_flatten();
            Ok(())
        }

        pub fn set_mirror_plane(&mut self, mirror_plane: Option<Plane>) -> crate::error::SetResult {
            if self.mirror_plane == mirror_plane {
                return Ok(());
            }
            self.mirror_plane = mirror_plane;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "mirrorPlane",
            });
            self.invalidate_hash();
            self.bubble_design_flatten();
            Ok(())
        }

        pub fn set_hidden(&mut self, hidden: Option<bool>) -> crate::error::SetResult {
            if self.hidden == hidden {
                return Ok(());
            }
            self.hidden = hidden;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "hidden",
            });
            self.invalidate_hash();
            self.bubble_design_flatten();
            Ok(())
        }

        pub fn set_locked(&mut self, locked: Option<bool>) -> crate::error::SetResult {
            if self.locked == locked {
                return Ok(());
            }
            self.locked = locked;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "locked",
            });
            self.invalidate_hash();
            self.bubble_design_flatten();
            Ok(())
        }

        /// World-space plane from design flatten cache.
        pub fn flat_plane(&self) -> Plane {
            self.flat_plane.get_or_init(|| {
                if let Some(d) = self.parent_design.upgrade() {
                    if let Ok(d) = d.read() {
                        if let Some((pl, _)) = d.flatten_map().get(&self.guid) {
                            return *pl;
                        }
                    }
                }
                self.plane.unwrap_or_else(Plane::world_xy)
            })
        }

        /// World-space center from design flatten cache.
        pub fn flat_center(&self) -> Coord {
            self.flat_center.get_or_init(|| {
                if let Some(d) = self.parent_design.upgrade() {
                    if let Ok(d) = d.read() {
                        if let Some((_, ce)) = d.flatten_map().get(&self.guid) {
                            return *ce;
                        }
                    }
                }
                self.center.unwrap_or_default()
            })
        }

        pub fn hash(&self) -> String {
            self.hash_cache.get_or_init(|| {
                let mut w = HashWriter::new();
                self.hash_into(&mut w);
                w.finalize()
            })
        }

        pub fn hash_into(&self, w: &mut HashWriter) {
            w.tag("piece")
                .str(self.guid.as_str())
                .opt_str(self.id.as_deref())
                .opt_str(self.name.as_deref())
                .opt_str(self.description.as_deref());
            if let Some(p) = &self.plane {
                p.hash_into(w);
            }
            if let Some(c) = &self.center {
                c.hash_into(w);
            }
            w.opt_f64(self.scale);
            if let Some(p) = &self.mirror_plane {
                p.hash_into(w);
            }
            w.opt_bool(self.hidden)
                .opt_bool(self.locked)
                .opt_str(self.color.as_deref());
            for p in &self.props {
                if let Ok(p) = p.read() {
                    p.hash_into(w);
                }
            }
            for a in &self.attributes {
                if let Ok(a) = a.read() {
                    a.hash_into(w);
                }
            }
            if let Some(t) = self.type_ref.as_ref().and_then(|t| t.upgrade()) {
                if let Ok(t) = t.read() {
                    w.str(t.guid.as_str());
                }
            }
        }

        pub fn to_id_dto(&self) -> PieceIdDto {
            PieceIdDto {
                guid: self.guid.clone(),
            }
        }

        pub fn to_metadata_dto(&self) -> PieceMetadataDto {
            let r#type = self
                .type_ref
                .as_ref()
                .and_then(|t| t.upgrade())
                .and_then(|t| {
                    t.read().ok().map(|t| TypeIdDto {
                        guid: t.guid.clone(),
                    })
                });
            let design = self.parent_design.upgrade().and_then(|d| {
                d.read().ok().map(|d| crate::design::DesignIdDto {
                    guid: d.guid.clone(),
                })
            });
            PieceMetadataDto {
                guid: self.guid.clone(),
                id: self.id.clone(),
                name: self.name.clone(),
                description: self.description.clone(),
                plane: self.plane,
                center: self.center,
                scale: self.scale,
                mirror_plane: self.mirror_plane,
                hidden: self.hidden,
                locked: self.locked,
                color: self.color.clone(),
                r#type,
                design,
            }
        }

        pub fn to_shallow_dto(&self) -> PieceShallowDto {
            let m = self.to_metadata_dto();
            PieceShallowDto {
                guid: m.guid,
                id: m.id,
                name: m.name,
                description: m.description,
                plane: m.plane,
                center: m.center,
                scale: m.scale,
                mirror_plane: m.mirror_plane,
                hidden: m.hidden,
                locked: m.locked,
                color: m.color,
                r#type: m.r#type,
                design: m.design,
                props: self
                    .props
                    .iter()
                    .filter_map(|p| p.read().ok().map(|p| p.to_shallow_dto()))
                    .collect(),
                attributes: self
                    .attributes
                    .iter()
                    .filter_map(|a| a.read().ok().map(|a| a.to_shallow_dto()))
                    .collect(),
            }
        }

        pub fn to_full_dto(&self) -> PieceFullDto {
            let m = self.to_metadata_dto();
            PieceFullDto {
                guid: m.guid,
                id: m.id,
                name: m.name,
                description: m.description,
                plane: m.plane,
                center: m.center,
                scale: m.scale,
                mirror_plane: m.mirror_plane,
                hidden: m.hidden,
                locked: m.locked,
                color: m.color,
                r#type: m.r#type,
                design: m.design,
                props: self
                    .props
                    .iter()
                    .filter_map(|p| p.read().ok().map(|p| p.to_full_dto()))
                    .collect(),
                attributes: self
                    .attributes
                    .iter()
                    .filter_map(|a| a.read().ok().map(|a| a.to_full_dto()))
                    .collect(),
            }
        }
    }

    impl Default for PieceStore {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub mod port {
    use serde::{Deserialize, Serialize};
    use std::sync::{Arc, RwLock, Weak};

    use crate::attribute::{AttributeFullDto, AttributeShallowDto, AttributeStore};
    use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
    use crate::geom::{Coord, Vector};
    use crate::guid::Guid;
    use crate::hash::{Cache, HashWriter};
    use crate::quality::{QualityFullDto, QualityShallowDto, QualityStore, QualityStoreRef};
    use crate::typ::TypeStoreWeak;

    pub type PortStoreRef = Arc<RwLock<PortStore>>;
    pub type PortStoreWeak = std::sync::Weak<RwLock<PortStore>>;

    /// Connection anchor on a [`crate::typ::TypeStore`].
    #[derive(Debug)]
    pub struct PortStore {
        pub guid: Guid,
        pub id: Option<String>,
        pub family: Option<String>,
        pub compatible_families: Vec<String>,
        pub mandatory: Option<bool>,
        pub t: Option<f64>,
        pub description: Option<String>,
        pub point: Option<Coord>,
        pub direction: Option<Vector>,
        pub qualities: Vec<QualityStoreRef>,
        pub attributes: Vec<AttributeStore>,
        pub parent_type: TypeStoreWeak,
        pub(crate) event_bus: Weak<EventBus>,
        hash_cache: Cache<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct PortIdDto {
        pub guid: Guid,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct PortMetadataDto {
        pub guid: Guid,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub family: Option<String>,
        #[serde(
            default,
            skip_serializing_if = "Vec::is_empty",
            rename = "compatibleFamilies"
        )]
        pub compatible_families: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mandatory: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub t: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub point: Option<Coord>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub direction: Option<Vector>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct PortShallowDto {
        pub guid: Guid,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub family: Option<String>,
        #[serde(
            default,
            skip_serializing_if = "Vec::is_empty",
            rename = "compatibleFamilies"
        )]
        pub compatible_families: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mandatory: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub t: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub point: Option<Coord>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub direction: Option<Vector>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub qualities: Vec<QualityShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub attributes: Vec<AttributeShallowDto>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct PortFullDto {
        pub guid: Guid,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub family: Option<String>,
        #[serde(
            default,
            skip_serializing_if = "Vec::is_empty",
            rename = "compatibleFamilies"
        )]
        pub compatible_families: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mandatory: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub t: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub point: Option<Coord>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub direction: Option<Vector>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub qualities: Vec<QualityFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub attributes: Vec<AttributeFullDto>,
    }

    impl PortStore {
        pub fn new() -> Self {
            Self {
                guid: Guid::new_v7(),
                id: None,
                family: None,
                compatible_families: Vec::new(),
                mandatory: None,
                t: None,
                description: None,
                point: None,
                direction: None,
                qualities: Vec::new(),
                attributes: Vec::new(),
                parent_type: Weak::new(),
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        #[inline]
        fn emit_ev(&self, ev: KitEvent) {
            emit_weak(&self.event_bus, ev);
        }

        fn entity_ref(&self) -> EntityRef {
            EntityRef::new(EntityKind::Port, self.guid.clone())
        }

        pub fn from_id_dto(d: PortIdDto) -> Self {
            Self {
                guid: d.guid,
                id: None,
                family: None,
                compatible_families: Vec::new(),
                mandatory: None,
                t: None,
                description: None,
                point: None,
                direction: None,
                qualities: Vec::new(),
                attributes: Vec::new(),
                parent_type: Weak::new(),
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        pub fn from_metadata_dto(d: PortMetadataDto) -> Self {
            Self {
                guid: d.guid,
                id: d.id,
                family: d.family,
                compatible_families: d.compatible_families,
                mandatory: d.mandatory,
                t: d.t,
                description: d.description,
                point: d.point,
                direction: d.direction,
                qualities: Vec::new(),
                attributes: Vec::new(),
                parent_type: Weak::new(),
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        pub fn from_shallow_dto(d: PortShallowDto) -> Self {
            let mut s = Self::from_metadata_dto(PortMetadataDto {
                guid: d.guid,
                id: d.id,
                family: d.family,
                compatible_families: d.compatible_families,
                mandatory: d.mandatory,
                t: d.t,
                description: d.description,
                point: d.point,
                direction: d.direction,
            });
            s.qualities = d
                .qualities
                .into_iter()
                .map(|q| Arc::new(RwLock::new(QualityStore::from_shallow_dto(q))))
                .collect();
            s.attributes = d
                .attributes
                .into_iter()
                .map(AttributeStore::from_shallow_dto)
                .collect();
            s
        }

        pub fn from_full_dto(d: PortFullDto) -> Self {
            let mut s = Self::from_metadata_dto(PortMetadataDto {
                guid: d.guid,
                id: d.id,
                family: d.family,
                compatible_families: d.compatible_families,
                mandatory: d.mandatory,
                t: d.t,
                description: d.description,
                point: d.point,
                direction: d.direction,
            });
            s.qualities = d
                .qualities
                .into_iter()
                .map(|q| Arc::new(RwLock::new(QualityStore::from_full_dto(q))))
                .collect();
            s.attributes = d
                .attributes
                .into_iter()
                .map(AttributeStore::from_full_dto)
                .collect();
            s
        }

        pub fn to_id_dto(&self) -> PortIdDto {
            PortIdDto {
                guid: self.guid.clone(),
            }
        }

        pub fn to_metadata_dto(&self) -> PortMetadataDto {
            PortMetadataDto {
                guid: self.guid.clone(),
                id: self.id.clone(),
                family: self.family.clone(),
                compatible_families: self.compatible_families.clone(),
                mandatory: self.mandatory,
                t: self.t,
                description: self.description.clone(),
                point: self.point,
                direction: self.direction,
            }
        }

        pub fn to_shallow_dto(&self) -> PortShallowDto {
            let m = self.to_metadata_dto();
            PortShallowDto {
                guid: m.guid,
                id: m.id,
                family: m.family,
                compatible_families: m.compatible_families,
                mandatory: m.mandatory,
                t: m.t,
                description: m.description,
                point: m.point,
                direction: m.direction,
                qualities: self
                    .qualities
                    .iter()
                    .filter_map(|q| q.read().ok().map(|q| q.to_shallow_dto()))
                    .collect(),
                attributes: self
                    .attributes
                    .iter()
                    .map(AttributeStore::to_shallow_dto)
                    .collect(),
            }
        }

        pub fn to_full_dto(&self) -> PortFullDto {
            let m = self.to_metadata_dto();
            PortFullDto {
                guid: m.guid,
                id: m.id,
                family: m.family,
                compatible_families: m.compatible_families,
                mandatory: m.mandatory,
                t: m.t,
                description: m.description,
                point: m.point,
                direction: m.direction,
                qualities: self
                    .qualities
                    .iter()
                    .filter_map(|q| q.read().ok().map(|q| q.to_full_dto()))
                    .collect(),
                attributes: self
                    .attributes
                    .iter()
                    .map(AttributeStore::to_full_dto)
                    .collect(),
            }
        }

        pub fn set_id(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.id == v {
                return Ok(());
            }
            self.id = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "id",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_family(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.family == v {
                return Ok(());
            }
            self.family = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "family",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_compatible_families(&mut self, v: Vec<String>) -> crate::error::SetResult {
            if self.compatible_families == v {
                return Ok(());
            }
            self.compatible_families = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "compatibleFamilies",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_mandatory(&mut self, v: Option<bool>) -> crate::error::SetResult {
            if self.mandatory == v {
                return Ok(());
            }
            self.mandatory = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "mandatory",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_t(&mut self, v: Option<f64>) -> crate::error::SetResult {
            if self.t == v {
                return Ok(());
            }
            self.t = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "t",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_description(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.description == v {
                return Ok(());
            }
            self.description = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "description",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_point(&mut self, v: Option<Coord>) -> crate::error::SetResult {
            if self.point == v {
                return Ok(());
            }
            self.point = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "point",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_direction(&mut self, v: Option<Vector>) -> crate::error::SetResult {
            if self.direction == v {
                return Ok(());
            }
            self.direction = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "direction",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn invalidate_hash(&self) {
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            if let Some(t) = self.parent_type.upgrade() {
                if let Ok(tr) = t.read() {
                    tr.invalidate_hash();
                }
            }
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
            w.tag("port")
                .str(self.guid.as_str())
                .opt_str(self.id.as_deref())
                .opt_str(self.family.as_deref());
            for f in &self.compatible_families {
                w.str(f);
            }
            w.opt_bool(self.mandatory).opt_f64(self.t);
            if let Some(p) = &self.point {
                p.hash_into(w);
            }
            if let Some(d) = &self.direction {
                d.hash_into(w);
            }
            for q in &self.qualities {
                if let Ok(q) = q.read() {
                    q.hash_into(w);
                }
            }
            for a in &self.attributes {
                a.hash_into(w);
            }
        }
    }

    impl Default for PortStore {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub mod prop {
    use serde::{Deserialize, Serialize};
    use std::sync::{RwLock, Weak};

    use crate::design::DesignStoreWeak;
    use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
    use crate::guid::Guid;
    use crate::hash::{Cache, HashWriter};
    use crate::kit::KitStoreWeak;
    use crate::piece::PieceStoreWeak;
    use crate::typ::TypeStoreWeak;

    pub type PropStoreRef = std::sync::Arc<RwLock<PropStore>>;
    pub type PropStoreWeak = Weak<RwLock<PropStore>>;

    /// A typed property value (distinct from free-form Attributes: props carry
    /// meaning in the domain, attributes are auxiliary metadata).
    #[derive(Debug)]
    pub struct PropStore {
        pub guid: Guid,
        pub key: String,
        pub value: String,
        pub unit: Option<String>,
        pub parent_kit: Option<KitStoreWeak>,
        pub parent_design: Option<DesignStoreWeak>,
        pub parent_type: Option<TypeStoreWeak>,
        pub parent_piece: Option<PieceStoreWeak>,
        pub(crate) event_bus: Weak<EventBus>,
        hash_cache: Cache<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct PropIdDto {
        pub guid: Guid,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct PropMetadataDto {
        pub guid: Guid,
        pub key: String,
        pub value: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct PropShallowDto {
        pub guid: Guid,
        pub key: String,
        pub value: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct PropFullDto {
        pub guid: Guid,
        pub key: String,
        pub value: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
    }

    impl PropStore {
        pub(crate) fn empty_shell(guid: Guid) -> Self {
            Self {
                guid,
                key: String::new(),
                value: String::new(),
                unit: None,
                parent_kit: None,
                parent_design: None,
                parent_type: None,
                parent_piece: None,
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        #[inline]
        fn emit_ev(&self, ev: KitEvent) {
            emit_weak(&self.event_bus, ev);
        }

        fn entity_ref(&self) -> EntityRef {
            EntityRef::new(EntityKind::Prop, self.guid.clone())
        }

        pub(crate) fn apply_full_dto_fields(&mut self, d: PropFullDto) {
            self.guid = d.guid;
            self.key = d.key;
            self.value = d.value;
            self.unit = d.unit;
            self.hash_cache.invalidate();
        }

        pub(crate) fn from_full_dto(d: PropFullDto) -> Self {
            let mut s = Self::empty_shell(d.guid.clone());
            s.apply_full_dto_fields(d);
            s
        }

        pub fn set_key(&mut self, key: String) -> crate::error::SetResult {
            if self.key == key {
                return Ok(());
            }
            self.key = key;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "key",
            });
            self.bubble();
            Ok(())
        }

        pub fn set_value(&mut self, value: String) -> crate::error::SetResult {
            if self.value == value {
                return Ok(());
            }
            self.value = value;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "value",
            });
            self.bubble();
            Ok(())
        }

        pub fn set_unit(&mut self, unit: Option<String>) -> crate::error::SetResult {
            if self.unit == unit {
                return Ok(());
            }
            self.unit = unit;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "unit",
            });
            self.bubble();
            Ok(())
        }

        fn bubble(&mut self) {
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            if let Some(w) = &self.parent_kit {
                if let Some(k) = w.upgrade() {
                    if let Ok(k) = k.read() {
                        k.invalidate_hash();
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
            } else if let Some(w) = &self.parent_kit {
                if let Some(k) = w.upgrade() {
                    if let Ok(k) = k.read() {
                        k.invalidate_validation();
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
            if let Some(w) = &self.parent_piece {
                if let Some(p) = w.upgrade() {
                    if let Ok(p) = p.read() {
                        p.invalidate_hash();
                    }
                }
            }
        }

        pub fn to_id_dto(&self) -> PropIdDto {
            PropIdDto {
                guid: self.guid.clone(),
            }
        }

        pub fn to_metadata_dto(&self) -> PropMetadataDto {
            PropMetadataDto {
                guid: self.guid.clone(),
                key: self.key.clone(),
                value: self.value.clone(),
                unit: self.unit.clone(),
            }
        }

        pub fn to_shallow_dto(&self) -> PropShallowDto {
            let m = self.to_metadata_dto();
            PropShallowDto {
                guid: m.guid,
                key: m.key,
                value: m.value,
                unit: m.unit,
            }
        }

        pub fn to_full_dto(&self) -> PropFullDto {
            let m = self.to_metadata_dto();
            PropFullDto {
                guid: m.guid,
                key: m.key,
                value: m.value,
                unit: m.unit,
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
            w.tag("prop")
                .str(self.guid.as_str())
                .str(&self.key)
                .str(&self.value)
                .opt_str(self.unit.as_deref());
        }
    }
}

pub mod quality {
    use serde::{Deserialize, Serialize};
    use std::sync::{Arc, RwLock, Weak};

    use crate::benchmark::{
        BenchmarkFullDto, BenchmarkMetadataDto, BenchmarkStore, BenchmarkStoreRef,
    };
    use crate::connector::ConnectorStoreWeak;
    use crate::design::DesignStoreWeak;
    use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
    use crate::guid::Guid;
    use crate::hash::{Cache, HashWriter};
    use crate::kit::KitStoreWeak;
    use crate::port::PortStoreWeak;
    use crate::representation::RepresentationStoreWeak;
    use crate::typ::TypeStoreWeak;

    pub type QualityStoreRef = Arc<RwLock<QualityStore>>;
    pub type QualityStoreWeak = Weak<RwLock<QualityStore>>;

    /// Measurable/named quality that can be attached to ports, types, designs, etc.
    #[derive(Debug)]
    pub struct QualityStore {
        pub guid: Guid,
        pub key: String,
        pub value: Option<String>,
        pub unit: Option<String>,
        pub definition: Option<String>,
        pub description: Option<String>,
        pub benchmarks: Vec<BenchmarkStoreRef>,
        pub parent_kit: Option<KitStoreWeak>,
        pub parent_design: Option<DesignStoreWeak>,
        pub parent_type: Option<TypeStoreWeak>,
        pub parent_port: Option<PortStoreWeak>,
        pub parent_connector: Option<ConnectorStoreWeak>,
        pub parent_representation: Option<RepresentationStoreWeak>,
        pub(crate) event_bus: Weak<EventBus>,
        hash_cache: Cache<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct QualityIdDto {
        pub guid: Guid,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct QualityMetadataDto {
        pub guid: Guid,
        pub key: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub definition: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct QualityShallowDto {
        pub guid: Guid,
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
        pub benchmarks: Vec<BenchmarkMetadataDto>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct QualityFullDto {
        pub guid: Guid,
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
        pub benchmarks: Vec<BenchmarkFullDto>,
    }

    impl QualityStore {
        pub(crate) fn empty_shell(guid: Guid) -> Self {
            Self {
                guid,
                key: String::new(),
                value: None,
                unit: None,
                definition: None,
                description: None,
                benchmarks: Vec::new(),
                parent_kit: None,
                parent_design: None,
                parent_type: None,
                parent_port: None,
                parent_connector: None,
                parent_representation: None,
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        #[inline]
        fn emit_ev(&self, ev: KitEvent) {
            emit_weak(&self.event_bus, ev);
        }

        fn entity_ref(&self) -> EntityRef {
            EntityRef::new(EntityKind::Quality, self.guid.clone())
        }

        pub(crate) fn apply_metadata_fields(&mut self, d: QualityMetadataDto) {
            self.guid = d.guid;
            self.key = d.key;
            self.value = d.value;
            self.unit = d.unit;
            self.definition = d.definition;
            self.description = d.description;
            self.hash_cache.invalidate();
        }

        pub fn set_key(&mut self, key: String) -> crate::error::SetResult {
            if self.key == key {
                return Ok(());
            }
            self.key = key;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "key",
            });
            self.bubble();
            Ok(())
        }

        pub fn set_value(&mut self, value: Option<String>) -> crate::error::SetResult {
            if self.value == value {
                return Ok(());
            }
            self.value = value;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "value",
            });
            self.bubble();
            Ok(())
        }

        pub fn set_unit(&mut self, unit: Option<String>) -> crate::error::SetResult {
            if self.unit == unit {
                return Ok(());
            }
            self.unit = unit;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "unit",
            });
            self.bubble();
            Ok(())
        }

        pub fn set_definition(&mut self, definition: Option<String>) -> crate::error::SetResult {
            if self.definition == definition {
                return Ok(());
            }
            self.definition = definition;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "definition",
            });
            self.bubble();
            Ok(())
        }

        pub fn set_description(&mut self, description: Option<String>) -> crate::error::SetResult {
            if self.description == description {
                return Ok(());
            }
            self.description = description;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "description",
            });
            self.bubble();
            Ok(())
        }

        fn bubble(&mut self) {
            self.invalidate_hash();
        }

        pub(crate) fn from_shallow_dto(d: QualityShallowDto) -> Self {
            let mut s = Self::empty_shell(d.guid.clone());
            s.apply_metadata_fields(QualityMetadataDto {
                guid: d.guid,
                key: d.key,
                value: d.value,
                unit: d.unit,
                definition: d.definition,
                description: d.description,
            });
            s.benchmarks = d
                .benchmarks
                .into_iter()
                .map(|b| {
                    let mut bs = BenchmarkStore::empty_shell(b.guid.clone());
                    bs.apply_metadata_dto(b);
                    Arc::new(RwLock::new(bs))
                })
                .collect();
            s
        }

        pub(crate) fn from_full_dto(d: QualityFullDto) -> Self {
            let QualityFullDto {
                guid,
                key,
                value,
                unit,
                definition,
                description,
                benchmarks,
            } = d;
            let mut s = Self::empty_shell(guid.clone());
            s.apply_metadata_fields(QualityMetadataDto {
                guid,
                key,
                value,
                unit,
                definition,
                description,
            });
            s.benchmarks = benchmarks
                .into_iter()
                .map(|b| {
                    let mut bs = BenchmarkStore::empty_shell(b.guid.clone());
                    bs.apply_metadata_dto(BenchmarkMetadataDto {
                        guid: b.guid,
                        name: b.name,
                        min: b.min,
                        max: b.max,
                        min_excluded: b.min_excluded,
                        max_excluded: b.max_excluded,
                    });
                    Arc::new(RwLock::new(bs))
                })
                .collect();
            s
        }

        pub fn to_id_dto(&self) -> QualityIdDto {
            QualityIdDto {
                guid: self.guid.clone(),
            }
        }

        pub fn to_metadata_dto(&self) -> QualityMetadataDto {
            QualityMetadataDto {
                guid: self.guid.clone(),
                key: self.key.clone(),
                value: self.value.clone(),
                unit: self.unit.clone(),
                definition: self.definition.clone(),
                description: self.description.clone(),
            }
        }

        pub fn to_shallow_dto(&self) -> QualityShallowDto {
            let m = self.to_metadata_dto();
            QualityShallowDto {
                guid: m.guid,
                key: m.key,
                value: m.value,
                unit: m.unit,
                definition: m.definition,
                description: m.description,
                benchmarks: self
                    .benchmarks
                    .iter()
                    .filter_map(|b| b.read().ok().map(|b| b.to_metadata_dto()))
                    .collect(),
            }
        }

        pub fn to_full_dto(&self) -> QualityFullDto {
            let m = self.to_metadata_dto();
            QualityFullDto {
                guid: m.guid,
                key: m.key,
                value: m.value,
                unit: m.unit,
                definition: m.definition,
                description: m.description,
                benchmarks: self
                    .benchmarks
                    .iter()
                    .filter_map(|b| b.read().ok().map(|b| b.to_full_dto()))
                    .collect(),
            }
        }

        pub fn invalidate_hash(&self) {
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            if let Some(w) = &self.parent_kit {
                if let Some(k) = w.upgrade() {
                    if let Ok(k) = k.read() {
                        k.invalidate_hash();
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
            } else if let Some(w) = &self.parent_kit {
                if let Some(k) = w.upgrade() {
                    if let Ok(k) = k.read() {
                        k.invalidate_validation();
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
            if let Some(w) = &self.parent_port {
                if let Some(p) = w.upgrade() {
                    if let Ok(p) = p.read() {
                        p.invalidate_hash();
                    }
                }
            }
            if let Some(w) = &self.parent_connector {
                if let Some(c) = w.upgrade() {
                    if let Ok(c) = c.read() {
                        c.invalidate_hash();
                    }
                }
            }
            if let Some(w) = &self.parent_representation {
                if let Some(r) = w.upgrade() {
                    if let Ok(r) = r.read() {
                        r.invalidate_hash();
                    }
                }
            }
        }

        pub fn hash(&self) -> String {
            self.hash_cache.get_or_init(|| {
                let mut w = HashWriter::new();
                self.hash_into(&mut w);
                w.finalize()
            })
        }

        pub fn hash_into(&self, w: &mut HashWriter) {
            w.tag("quality")
                .str(self.guid.as_str())
                .str(&self.key)
                .opt_str(self.value.as_deref())
                .opt_str(self.unit.as_deref())
                .opt_str(self.definition.as_deref());
            for b in &self.benchmarks {
                if let Ok(b) = b.read() {
                    b.hash_into(w);
                }
            }
        }
    }
}

pub mod report {
    use serde::{Deserialize, Serialize};

    /// A single note attached to the outcome of an operation.
    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct OperationNote {
        #[serde(default)]
        pub severity: NoteSeverity,
        pub message: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub pointer: Option<String>,
    }

    #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum NoteSeverity {
        #[default]
        Info,
        Warning,
        Error,
    }

    /// Outcome of an operation, always carrying the completeness flag and the
    /// collected notes. For structural operations the payload lives in `value`.
    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct SemioReport<T> {
        pub ok: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<T>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub infos: Vec<OperationNote>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub warnings: Vec<OperationNote>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub errors: Vec<OperationNote>,
    }

    impl<T> SemioReport<T> {
        pub fn ok(value: T) -> Self {
            Self {
                ok: true,
                value: Some(value),
                infos: Vec::new(),
                warnings: Vec::new(),
                errors: Vec::new(),
            }
        }

        pub fn err(message: impl Into<String>) -> Self {
            Self {
                ok: false,
                value: None,
                infos: Vec::new(),
                warnings: Vec::new(),
                errors: vec![OperationNote {
                    severity: NoteSeverity::Error,
                    message: message.into(),
                    pointer: None,
                }],
            }
        }

        pub fn with_infos(mut self, infos: Vec<OperationNote>) -> Self {
            self.infos = infos;
            self
        }

        pub fn with_warnings(mut self, warnings: Vec<OperationNote>) -> Self {
            self.warnings = warnings;
            self
        }
    }

    /// Outcome of a validation pass.
    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct ValidationResult {
        pub is_valid: bool,
        #[serde(default)]
        pub errors: Vec<String>,
        #[serde(default)]
        pub warnings: Vec<String>,
    }

    impl ValidationResult {
        pub fn valid() -> Self {
            Self {
                is_valid: true,
                errors: Vec::new(),
                warnings: Vec::new(),
            }
        }

        pub fn with_error(msg: impl Into<String>) -> Self {
            Self {
                is_valid: false,
                errors: vec![msg.into()],
                warnings: Vec::new(),
            }
        }
    }
}

pub mod representation {
    use serde::{Deserialize, Serialize};
    use std::sync::{Arc, RwLock, Weak};

    use crate::attribute::{AttributeFullDto, AttributeShallowDto, AttributeStore};
    use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
    use crate::file::{FileIdDto, FileStoreWeak};
    use crate::guid::Guid;
    use crate::hash::{Cache, HashWriter};
    use crate::quality::{QualityFullDto, QualityShallowDto, QualityStore, QualityStoreRef};
    use crate::tag::{TagFullDto, TagShallowDto, TagStore};

    pub type RepresentationStoreRef = Arc<RwLock<RepresentationStore>>;
    pub type RepresentationStoreWeak = Weak<RwLock<RepresentationStore>>;

    /// Rendering / geometric representation of a [`crate::typ::TypeStore`].
    #[derive(Debug)]
    pub struct RepresentationStore {
        pub guid: Guid,
        pub url: String,
        pub description: Option<String>,
        pub tags: Vec<TagStore>,
        pub file: Option<FileStoreWeak>,
        pub qualities: Vec<QualityStoreRef>,
        pub attributes: Vec<AttributeStore>,
        pub parent_type: Weak<RwLock<crate::typ::TypeStore>>,
        pub(crate) event_bus: Weak<EventBus>,
        hash_cache: Cache<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct RepresentationIdDto {
        pub guid: Guid,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct RepresentationMetadataDto {
        pub guid: Guid,
        pub url: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub file: Option<FileIdDto>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct RepresentationShallowDto {
        pub guid: Guid,
        pub url: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub file: Option<FileIdDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tags: Vec<TagShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub qualities: Vec<QualityShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub attributes: Vec<AttributeShallowDto>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct RepresentationFullDto {
        pub guid: Guid,
        pub url: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub file: Option<FileIdDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tags: Vec<TagFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub qualities: Vec<QualityFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub attributes: Vec<AttributeFullDto>,
    }

    impl RepresentationStore {
        pub fn new(url: impl Into<String>) -> Self {
            Self {
                guid: Guid::new_v7(),
                url: url.into(),
                description: None,
                tags: Vec::new(),
                file: None,
                qualities: Vec::new(),
                attributes: Vec::new(),
                parent_type: Weak::new(),
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        #[inline]
        fn emit_ev(&self, ev: KitEvent) {
            emit_weak(&self.event_bus, ev);
        }

        fn entity_ref(&self) -> EntityRef {
            EntityRef::new(EntityKind::Representation, self.guid.clone())
        }

        pub fn from_id_dto(d: RepresentationIdDto) -> Self {
            Self {
                guid: d.guid,
                url: String::new(),
                description: None,
                tags: Vec::new(),
                file: None,
                qualities: Vec::new(),
                attributes: Vec::new(),
                parent_type: Weak::new(),
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        pub fn from_metadata_dto(d: RepresentationMetadataDto) -> Self {
            Self {
                guid: d.guid,
                url: d.url,
                description: d.description,
                tags: Vec::new(),
                file: None,
                qualities: Vec::new(),
                attributes: Vec::new(),
                parent_type: Weak::new(),
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        pub fn from_shallow_dto(d: RepresentationShallowDto) -> Self {
            let mut s = Self::from_metadata_dto(RepresentationMetadataDto {
                guid: d.guid,
                url: d.url,
                description: d.description,
                file: d.file,
            });
            s.tags = d.tags.into_iter().map(TagStore::from_shallow_dto).collect();
            s.qualities = d
                .qualities
                .into_iter()
                .map(|q| Arc::new(RwLock::new(QualityStore::from_shallow_dto(q))))
                .collect();
            s.attributes = d
                .attributes
                .into_iter()
                .map(AttributeStore::from_shallow_dto)
                .collect();
            s
        }

        pub fn from_full_dto(d: RepresentationFullDto) -> Self {
            let mut s = Self::from_metadata_dto(RepresentationMetadataDto {
                guid: d.guid,
                url: d.url,
                description: d.description,
                file: d.file,
            });
            s.tags = d.tags.into_iter().map(TagStore::from_full_dto).collect();
            s.qualities = d
                .qualities
                .into_iter()
                .map(|q| Arc::new(RwLock::new(QualityStore::from_full_dto(q))))
                .collect();
            s.attributes = d
                .attributes
                .into_iter()
                .map(AttributeStore::from_full_dto)
                .collect();
            s
        }

        pub fn to_id_dto(&self) -> RepresentationIdDto {
            RepresentationIdDto {
                guid: self.guid.clone(),
            }
        }

        pub fn to_metadata_dto(&self) -> RepresentationMetadataDto {
            let file = self
                .file
                .as_ref()
                .and_then(|f| f.upgrade())
                .and_then(|f| f.read().ok().map(|f| f.to_id_dto()));
            RepresentationMetadataDto {
                guid: self.guid.clone(),
                url: self.url.clone(),
                description: self.description.clone(),
                file,
            }
        }

        pub fn to_shallow_dto(&self) -> RepresentationShallowDto {
            let m = self.to_metadata_dto();
            RepresentationShallowDto {
                guid: m.guid,
                url: m.url,
                description: m.description,
                file: m.file,
                tags: self.tags.iter().map(TagStore::to_shallow_dto).collect(),
                qualities: self
                    .qualities
                    .iter()
                    .filter_map(|q| q.read().ok().map(|q| q.to_shallow_dto()))
                    .collect(),
                attributes: self
                    .attributes
                    .iter()
                    .map(AttributeStore::to_shallow_dto)
                    .collect(),
            }
        }

        pub fn to_full_dto(&self) -> RepresentationFullDto {
            let m = self.to_metadata_dto();
            RepresentationFullDto {
                guid: m.guid,
                url: m.url,
                description: m.description,
                file: m.file,
                tags: self.tags.iter().map(TagStore::to_full_dto).collect(),
                qualities: self
                    .qualities
                    .iter()
                    .filter_map(|q| q.read().ok().map(|q| q.to_full_dto()))
                    .collect(),
                attributes: self
                    .attributes
                    .iter()
                    .map(AttributeStore::to_full_dto)
                    .collect(),
            }
        }

        pub fn set_url(&mut self, url: String) -> crate::error::SetResult {
            if let Err(e) = crate::validate::required_url(&url, "url") {
                self.emit_ev(KitEvent::SetRejected {
                    entity: self.entity_ref(),
                    field: "url".into(),
                    error: e.clone(),
                });
                return Err(e);
            }
            if self.url == url {
                return Ok(());
            }
            self.url = url;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "url",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn set_description(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.description == v {
                return Ok(());
            }
            self.description = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "description",
            });
            self.invalidate_hash();
            Ok(())
        }

        pub fn invalidate_hash(&self) {
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            if let Some(t) = self.parent_type.upgrade() {
                if let Ok(tr) = t.read() {
                    tr.invalidate_hash();
                }
            }
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
            w.tag("representation")
                .str(self.guid.as_str())
                .str(&self.url)
                .opt_str(self.description.as_deref());
            for t in &self.tags {
                t.hash_into(w);
            }
            if let Some(file) = self.file.as_ref().and_then(|f| f.upgrade()) {
                if let Ok(file) = file.read() {
                    w.str(file.guid.as_str());
                }
            }
            for q in &self.qualities {
                if let Ok(q) = q.read() {
                    q.hash_into(w);
                }
            }
            for a in &self.attributes {
                a.hash_into(w);
            }
        }
    }
}

pub mod session {
    use std::sync::{Arc, Mutex, RwLock};

    use crate::diff::DesignChange;
    use crate::error::{Result, SemioError};
    use crate::kit::{KitStore, KitStoreRef};

    /// In-memory transaction boundary around a [`KitStore`].
    pub struct KitGraphSession {
        inner: Mutex<Inner>,
    }

    struct Inner {
        kit: KitStoreRef,
        undo: Vec<DesignChange>,
        redo: Vec<DesignChange>,
    }

    impl KitGraphSession {
        pub fn new(kit: KitStore) -> Self {
            Self {
                inner: Mutex::new(Inner {
                    kit: Arc::new(RwLock::new(kit)),
                    undo: Vec::new(),
                    redo: Vec::new(),
                }),
            }
        }

        pub fn from_ref(kit: KitStoreRef) -> Self {
            Self {
                inner: Mutex::new(Inner {
                    kit,
                    undo: Vec::new(),
                    redo: Vec::new(),
                }),
            }
        }

        pub fn kit_handle(&self) -> Result<KitStoreRef> {
            self.inner
                .lock()
                .map(|g| g.kit.clone())
                .map_err(|_| SemioError::LockPoisoned("session"))
        }

        pub fn map_kit<T, F: FnOnce(&KitStore) -> T>(&self, f: F) -> Result<T> {
            let g = self
                .inner
                .lock()
                .map_err(|_| SemioError::LockPoisoned("session"))?;
            let kit = g.kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?;
            Ok(f(&kit))
        }

        pub fn map_kit_mut<T, F: FnOnce(&mut KitStore) -> T>(&self, f: F) -> Result<T> {
            let g = self
                .inner
                .lock()
                .map_err(|_| SemioError::LockPoisoned("session"))?;
            let mut kit = g.kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
            Ok(f(&mut kit))
        }

        pub fn commit(&self, change: DesignChange) -> Result<()> {
            let mut g = self
                .inner
                .lock()
                .map_err(|_| SemioError::LockPoisoned("session"))?;
            g.undo.push(change);
            g.redo.clear();
            Ok(())
        }

        pub fn undo_depth(&self) -> Result<usize> {
            let g = self
                .inner
                .lock()
                .map_err(|_| SemioError::LockPoisoned("session"))?;
            Ok(g.undo.len())
        }

        pub fn redo_depth(&self) -> Result<usize> {
            let g = self
                .inner
                .lock()
                .map_err(|_| SemioError::LockPoisoned("session"))?;
            Ok(g.redo.len())
        }

        pub fn last_change(&self) -> Result<Option<DesignChange>> {
            let g = self
                .inner
                .lock()
                .map_err(|_| SemioError::LockPoisoned("session"))?;
            Ok(g.undo.last().cloned())
        }
    }
}

pub mod side {
    use serde::{Deserialize, Serialize};
    use std::sync::{Arc, RwLock, Weak};

    use crate::connection::ConnectionStoreWeak;
    use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
    use crate::guid::Guid;
    use crate::hash::{Cache, HashWriter};
    use crate::piece::PieceStoreWeak;
    use crate::port::PortStoreWeak;

    pub type SideStoreRef = Arc<RwLock<SideStore>>;
    pub type SideStoreWeak = Weak<RwLock<SideStore>>;

    /// One end of a [`crate::connection::ConnectionStore`].
    #[derive(Debug)]
    pub struct SideStore {
        pub guid: Guid,
        pub piece: PieceStoreWeak,
        pub port: Option<PortStoreWeak>,
        /// Optional "design piece" for designs that include other designs.
        pub design_piece: Option<PieceStoreWeak>,
        pub parent_connection: Option<ConnectionStoreWeak>,
        pub(crate) event_bus: Weak<EventBus>,
        hash_cache: Cache<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct SideIdDto {
        pub guid: Guid,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct SideMetadataDto {
        pub guid: Guid,
        pub piece: crate::piece::PieceIdDto,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub port: Option<crate::port::PortIdDto>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            rename = "designPiece"
        )]
        pub design_piece: Option<crate::piece::PieceIdDto>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct SideShallowDto {
        pub guid: Guid,
        pub piece: crate::piece::PieceIdDto,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub port: Option<crate::port::PortIdDto>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            rename = "designPiece"
        )]
        pub design_piece: Option<crate::piece::PieceIdDto>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct SideFullDto {
        pub guid: Guid,
        pub piece: crate::piece::PieceIdDto,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub port: Option<crate::port::PortIdDto>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            rename = "designPiece"
        )]
        pub design_piece: Option<crate::piece::PieceIdDto>,
    }

    impl SideStore {
        pub(crate) fn empty_shell(guid: Guid) -> Self {
            Self {
                guid,
                piece: Weak::new(),
                port: None,
                design_piece: None,
                parent_connection: None,
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        #[inline]
        fn emit_ev(&self, ev: KitEvent) {
            emit_weak(&self.event_bus, ev);
        }

        fn entity_ref(&self) -> EntityRef {
            EntityRef::new(EntityKind::Side, self.guid.clone())
        }

        pub(crate) fn apply_metadata_dto(&mut self, d: SideMetadataDto) {
            self.guid = d.guid;
            self.hash_cache.invalidate();
        }

        pub fn set_piece_weak(&mut self, piece: PieceStoreWeak) -> crate::error::SetResult {
            self.piece = piece;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "piece",
            });
            self.bubble();
            Ok(())
        }

        pub fn set_port_weak(&mut self, port: Option<PortStoreWeak>) -> crate::error::SetResult {
            self.port = port;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "port",
            });
            self.bubble();
            Ok(())
        }

        pub fn set_design_piece_weak(
            &mut self,
            design_piece: Option<PieceStoreWeak>,
        ) -> crate::error::SetResult {
            self.design_piece = design_piece;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "designPiece",
            });
            self.bubble();
            Ok(())
        }

        fn bubble(&mut self) {
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            if let Some(w) = &self.parent_connection {
                if let Some(c) = w.upgrade() {
                    if let Ok(c) = c.read() {
                        c.notify_aggregate_change();
                    }
                }
            }
        }

        pub fn to_id_dto(&self) -> SideIdDto {
            SideIdDto {
                guid: self.guid.clone(),
            }
        }

        pub fn to_metadata_dto(&self) -> SideMetadataDto {
            let piece_guid = self
                .piece
                .upgrade()
                .and_then(|p| p.read().ok().map(|p| p.guid.clone()))
                .unwrap_or_default();
            let port = self
                .port
                .as_ref()
                .and_then(|p| p.upgrade())
                .and_then(|p| p.read().ok().map(|p| p.to_id_dto()));
            let design_piece = self
                .design_piece
                .as_ref()
                .and_then(|p| p.upgrade())
                .and_then(|p| p.read().ok().map(|p| p.to_id_dto()));
            SideMetadataDto {
                guid: self.guid.clone(),
                piece: crate::piece::PieceIdDto { guid: piece_guid },
                port,
                design_piece,
            }
        }

        pub fn to_shallow_dto(&self) -> SideShallowDto {
            let m = self.to_metadata_dto();
            SideShallowDto {
                guid: m.guid,
                piece: m.piece,
                port: m.port,
                design_piece: m.design_piece,
            }
        }

        pub fn to_full_dto(&self) -> SideFullDto {
            let m = self.to_metadata_dto();
            SideFullDto {
                guid: m.guid,
                piece: m.piece,
                port: m.port,
                design_piece: m.design_piece,
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
            w.str(self.guid.as_str());
            if let Some(p) = self.piece.upgrade() {
                if let Ok(p) = p.read() {
                    w.str(p.guid.as_str());
                }
            }
            if let Some(p) = self.port.as_ref().and_then(|p| p.upgrade()) {
                if let Ok(p) = p.read() {
                    w.str(p.guid.as_str());
                }
            }
            if let Some(p) = self.design_piece.as_ref().and_then(|p| p.upgrade()) {
                if let Ok(p) = p.read() {
                    w.str(p.guid.as_str());
                }
            }
        }
    }

    impl Default for SideStore {
        fn default() -> Self {
            Self {
                guid: Guid::new_v7(),
                piece: Weak::new(),
                port: None,
                design_piece: None,
                parent_connection: None,
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }
    }
}

pub mod stat {
    use serde::{Deserialize, Serialize};
    use std::sync::{RwLock, Weak};

    use crate::design::DesignStoreWeak;
    use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
    use crate::guid::Guid;
    use crate::hash::{Cache, HashWriter};
    use crate::kit::KitStoreWeak;

    pub type StatStoreRef = std::sync::Arc<RwLock<StatStore>>;
    pub type StatStoreWeak = Weak<RwLock<StatStore>>;

    /// Computed/summary stat attached to a design or kit (e.g. piece count).
    #[derive(Debug)]
    pub struct StatStore {
        pub guid: Guid,
        pub key: String,
        pub value: String,
        pub unit: Option<String>,
        pub description: Option<String>,
        pub parent_kit: Option<KitStoreWeak>,
        pub parent_design: Option<DesignStoreWeak>,
        pub(crate) event_bus: Weak<EventBus>,
        hash_cache: Cache<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct StatIdDto {
        pub guid: Guid,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct StatMetadataDto {
        pub guid: Guid,
        pub key: String,
        pub value: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct StatShallowDto {
        pub guid: Guid,
        pub key: String,
        pub value: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct StatFullDto {
        pub guid: Guid,
        pub key: String,
        pub value: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }

    impl StatStore {
        pub(crate) fn empty_shell(guid: Guid) -> Self {
            Self {
                guid,
                key: String::new(),
                value: String::new(),
                unit: None,
                description: None,
                parent_kit: None,
                parent_design: None,
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        #[inline]
        fn emit_ev(&self, ev: KitEvent) {
            emit_weak(&self.event_bus, ev);
        }

        fn entity_ref(&self) -> EntityRef {
            EntityRef::new(EntityKind::Stat, self.guid.clone())
        }

        pub(crate) fn apply_full_dto_fields(&mut self, d: StatFullDto) {
            self.guid = d.guid;
            self.key = d.key;
            self.value = d.value;
            self.unit = d.unit;
            self.description = d.description;
            self.hash_cache.invalidate();
        }

        pub(crate) fn from_full_dto(d: StatFullDto) -> Self {
            let mut s = Self::empty_shell(d.guid.clone());
            s.apply_full_dto_fields(d);
            s
        }

        pub fn set_key(&mut self, key: String) -> crate::error::SetResult {
            if self.key == key {
                return Ok(());
            }
            self.key = key;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "key",
            });
            self.bubble();
            Ok(())
        }

        pub fn set_value(&mut self, value: String) -> crate::error::SetResult {
            if self.value == value {
                return Ok(());
            }
            self.value = value;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "value",
            });
            self.bubble();
            Ok(())
        }

        pub fn set_unit(&mut self, unit: Option<String>) -> crate::error::SetResult {
            if self.unit == unit {
                return Ok(());
            }
            self.unit = unit;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "unit",
            });
            self.bubble();
            Ok(())
        }

        pub fn set_description(&mut self, description: Option<String>) -> crate::error::SetResult {
            if self.description == description {
                return Ok(());
            }
            self.description = description;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "description",
            });
            self.bubble();
            Ok(())
        }

        fn bubble(&mut self) {
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            if let Some(w) = &self.parent_kit {
                if let Some(k) = w.upgrade() {
                    if let Ok(k) = k.read() {
                        k.invalidate_hash();
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
            } else if let Some(w) = &self.parent_kit {
                if let Some(k) = w.upgrade() {
                    if let Ok(k) = k.read() {
                        k.invalidate_validation();
                    }
                }
            }
        }

        pub fn to_id_dto(&self) -> StatIdDto {
            StatIdDto {
                guid: self.guid.clone(),
            }
        }

        pub fn to_metadata_dto(&self) -> StatMetadataDto {
            StatMetadataDto {
                guid: self.guid.clone(),
                key: self.key.clone(),
                value: self.value.clone(),
                unit: self.unit.clone(),
                description: self.description.clone(),
            }
        }

        pub fn to_shallow_dto(&self) -> StatShallowDto {
            let m = self.to_metadata_dto();
            StatShallowDto {
                guid: m.guid,
                key: m.key,
                value: m.value,
                unit: m.unit,
                description: m.description,
            }
        }

        pub fn to_full_dto(&self) -> StatFullDto {
            let m = self.to_metadata_dto();
            StatFullDto {
                guid: m.guid,
                key: m.key,
                value: m.value,
                unit: m.unit,
                description: m.description,
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
            w.tag("stat")
                .str(self.guid.as_str())
                .str(&self.key)
                .str(&self.value)
                .opt_str(self.unit.as_deref())
                .opt_str(self.description.as_deref());
        }
    }
}

pub mod tag {
    use serde::{Deserialize, Serialize};
    use std::sync::{RwLock, Weak};

    use crate::design::DesignStoreWeak;
    use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
    use crate::guid::Guid;
    use crate::hash::{Cache, HashWriter};
    use crate::kit::KitStoreWeak;
    use crate::typ::TypeStoreWeak;

    pub type TagStoreRef = std::sync::Arc<RwLock<TagStore>>;
    pub type TagStoreWeak = Weak<RwLock<TagStore>>;

    /// Freely choosable label used for filtering/grouping in the UI.
    #[derive(Debug)]
    pub struct TagStore {
        pub guid: Guid,
        pub name: String,
        pub order: Option<i64>,
        pub parent_kit: Option<KitStoreWeak>,
        pub parent_design: Option<DesignStoreWeak>,
        pub parent_type: Option<TypeStoreWeak>,
        pub(crate) event_bus: Weak<EventBus>,
        hash_cache: Cache<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct TagIdDto {
        pub guid: Guid,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct TagMetadataDto {
        pub guid: Guid,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub order: Option<i64>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct TagShallowDto {
        pub guid: Guid,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub order: Option<i64>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct TagFullDto {
        pub guid: Guid,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub order: Option<i64>,
    }

    impl TagStore {
        pub(crate) fn empty_shell(guid: Guid) -> Self {
            Self {
                guid,
                name: String::new(),
                order: None,
                parent_kit: None,
                parent_design: None,
                parent_type: None,
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        #[inline]
        fn emit_ev(&self, ev: KitEvent) {
            emit_weak(&self.event_bus, ev);
        }

        fn entity_ref(&self) -> EntityRef {
            EntityRef::new(EntityKind::Tag, self.guid.clone())
        }

        pub(crate) fn apply_full_dto_fields(&mut self, d: TagFullDto) {
            self.guid = d.guid;
            self.name = d.name;
            self.order = d.order;
            self.hash_cache.invalidate();
        }

        pub(crate) fn from_shallow_dto(d: TagShallowDto) -> Self {
            let mut s = Self::empty_shell(d.guid.clone());
            s.name = d.name;
            s.order = d.order;
            s.hash_cache.invalidate();
            s
        }

        pub(crate) fn from_full_dto(d: TagFullDto) -> Self {
            let mut s = Self::empty_shell(d.guid.clone());
            s.apply_full_dto_fields(d);
            s
        }

        pub fn set_name(&mut self, name: String) -> crate::error::SetResult {
            if self.name == name {
                return Ok(());
            }
            self.name = name;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "name",
            });
            self.bubble();
            Ok(())
        }

        pub fn set_order(&mut self, order: Option<i64>) -> crate::error::SetResult {
            if self.order == order {
                return Ok(());
            }
            self.order = order;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "order",
            });
            self.bubble();
            Ok(())
        }

        fn bubble(&mut self) {
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            if let Some(w) = &self.parent_kit {
                if let Some(k) = w.upgrade() {
                    if let Ok(k) = k.read() {
                        k.invalidate_hash();
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
            } else if let Some(w) = &self.parent_kit {
                if let Some(k) = w.upgrade() {
                    if let Ok(k) = k.read() {
                        k.invalidate_validation();
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

        pub fn to_id_dto(&self) -> TagIdDto {
            TagIdDto {
                guid: self.guid.clone(),
            }
        }

        pub fn to_metadata_dto(&self) -> TagMetadataDto {
            TagMetadataDto {
                guid: self.guid.clone(),
                name: self.name.clone(),
                order: self.order,
            }
        }

        pub fn to_shallow_dto(&self) -> TagShallowDto {
            let m = self.to_metadata_dto();
            TagShallowDto {
                guid: m.guid,
                name: m.name,
                order: m.order,
            }
        }

        pub fn to_full_dto(&self) -> TagFullDto {
            let m = self.to_metadata_dto();
            TagFullDto {
                guid: m.guid,
                name: m.name,
                order: m.order,
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
            w.tag("tag").str(self.guid.as_str()).str(&self.name);
            if let Some(o) = self.order {
                w.f64(o as f64);
            }
        }
    }
}

pub mod typ {
    use serde::{Deserialize, Serialize};
    use std::sync::{Arc, RwLock, Weak};

    use crate::attribute::{
        AttributeFullDto, AttributeShallowDto, AttributeStore, AttributeStoreRef,
    };
    use crate::author::{AuthorFullDto, AuthorShallowDto, AuthorStore, AuthorStoreRef};
    use crate::concept::{ConceptFullDto, ConceptShallowDto, ConceptStore, ConceptStoreRef};
    use crate::connector::{
        ConnectorFullDto, ConnectorShallowDto, ConnectorStore, ConnectorStoreRef,
    };
    use crate::events::{emit_weak, EntityKind, EntityRef, EventBus, KitEvent};
    use crate::geom::Location;
    use crate::guid::Guid;
    use crate::hash::{Cache, HashWriter};
    use crate::port::{PortFullDto, PortShallowDto, PortStore, PortStoreRef};
    use crate::prop::{PropFullDto, PropShallowDto, PropStore, PropStoreRef};
    use crate::quality::{QualityFullDto, QualityShallowDto, QualityStore, QualityStoreRef};
    use crate::representation::{
        RepresentationFullDto, RepresentationShallowDto, RepresentationStore,
        RepresentationStoreRef,
    };
    use crate::tag::{TagFullDto, TagShallowDto, TagStore, TagStoreRef};

    pub type TypeStoreRef = Arc<RwLock<TypeStore>>;
    pub type TypeStoreWeak = Weak<RwLock<TypeStore>>;

    /// Reusable component definition: a type.
    #[derive(Debug)]
    pub struct TypeStore {
        pub guid: Guid,
        pub name: String,
        pub description: Option<String>,
        pub icon: Option<String>,
        pub image: Option<String>,
        pub variant: Option<String>,
        pub stock: Option<i64>,
        pub virtual_: Option<bool>,
        pub unit: Option<String>,
        pub location: Option<Location>,
        pub ports: Vec<PortStoreRef>,
        pub connectors: Vec<ConnectorStoreRef>,
        pub representations: Vec<RepresentationStoreRef>,
        pub authors: Vec<AuthorStoreRef>,
        pub concepts: Vec<ConceptStoreRef>,
        pub tags: Vec<TagStoreRef>,
        pub qualities: Vec<QualityStoreRef>,
        pub props: Vec<PropStoreRef>,
        pub attributes: Vec<AttributeStoreRef>,
        pub created: Option<String>,
        pub updated: Option<String>,
        pub parent_kit: Weak<RwLock<crate::kit::KitStore>>,
        pub(crate) event_bus: Weak<EventBus>,
        hash_cache: Cache<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct TypeIdDto {
        pub guid: Guid,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct TypeMetadataDto {
        pub guid: Guid,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub image: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub variant: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub stock: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none", rename = "virtual")]
        pub virtual_: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub location: Option<Location>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub created: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub updated: Option<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct TypeShallowDto {
        pub guid: Guid,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub image: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub variant: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub stock: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none", rename = "virtual")]
        pub virtual_: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub location: Option<Location>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub created: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub updated: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ports: Vec<PortShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub connectors: Vec<ConnectorShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub representations: Vec<RepresentationShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub authors: Vec<AuthorShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub concepts: Vec<ConceptShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tags: Vec<TagShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub qualities: Vec<QualityShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub props: Vec<PropShallowDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub attributes: Vec<AttributeShallowDto>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct TypeFullDto {
        pub guid: Guid,
        pub name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub image: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub variant: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub stock: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none", rename = "virtual")]
        pub virtual_: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub location: Option<Location>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub created: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub updated: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub ports: Vec<PortFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub connectors: Vec<ConnectorFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub representations: Vec<RepresentationFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub authors: Vec<AuthorFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub concepts: Vec<ConceptFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tags: Vec<TagFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub qualities: Vec<QualityFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub props: Vec<PropFullDto>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub attributes: Vec<AttributeFullDto>,
    }

    impl TypeStore {
        pub fn new(name: impl Into<String>) -> Self {
            Self {
                guid: Guid::new_v7(),
                name: name.into(),
                description: None,
                icon: None,
                image: None,
                variant: None,
                stock: None,
                virtual_: None,
                unit: None,
                location: None,
                ports: Vec::new(),
                connectors: Vec::new(),
                representations: Vec::new(),
                authors: Vec::new(),
                concepts: Vec::new(),
                tags: Vec::new(),
                qualities: Vec::new(),
                props: Vec::new(),
                attributes: Vec::new(),
                created: None,
                updated: None,
                parent_kit: Weak::new(),
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        #[inline]
        fn emit_ev(&self, ev: KitEvent) {
            emit_weak(&self.event_bus, ev);
        }

        fn entity_ref(&self) -> EntityRef {
            EntityRef::new(EntityKind::Type, self.guid.clone())
        }

        pub fn invalidate_hash(&self) {
            self.hash_cache.invalidate();
            self.emit_ev(KitEvent::HashInvalidated {
                entity: self.entity_ref(),
            });
            if let Some(k) = self.parent_kit.upgrade() {
                if let Ok(kr) = k.read() {
                    kr.invalidate_hash();
                }
            }
        }

        pub fn invalidate_validation(&self) {
            if let Some(k) = self.parent_kit.upgrade() {
                if let Ok(kr) = k.read() {
                    kr.invalidate_validation();
                }
            }
        }

        pub fn set_name(&mut self, name: String) -> crate::error::SetResult {
            let name = name.trim().to_string();
            if let Err(e) = crate::validate::required_name(&name, "name") {
                self.emit_ev(KitEvent::SetRejected {
                    entity: self.entity_ref(),
                    field: "name".into(),
                    error: e.clone(),
                });
                return Err(e);
            }
            if self.name == name {
                return Ok(());
            }
            self.name = name;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "name",
            });
            self.invalidate_hash();
            self.invalidate_validation();
            Ok(())
        }

        pub fn set_description(&mut self, v: Option<String>) -> crate::error::SetResult {
            let v = match v {
                None => None,
                Some(s) if s.trim().is_empty() => None,
                Some(s) => Some(s.trim().to_string()),
            };
            if self.description == v {
                return Ok(());
            }
            self.description = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "description",
            });
            self.invalidate_hash();
            self.invalidate_validation();
            Ok(())
        }

        pub fn set_icon(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.icon == v {
                return Ok(());
            }
            self.icon = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "icon",
            });
            self.invalidate_hash();
            self.invalidate_validation();
            Ok(())
        }

        pub fn set_image(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.image == v {
                return Ok(());
            }
            self.image = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "image",
            });
            self.invalidate_hash();
            self.invalidate_validation();
            Ok(())
        }

        pub fn set_variant(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.variant == v {
                return Ok(());
            }
            self.variant = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "variant",
            });
            self.invalidate_hash();
            self.invalidate_validation();
            Ok(())
        }

        pub fn set_stock(&mut self, v: Option<i64>) -> crate::error::SetResult {
            if self.stock == v {
                return Ok(());
            }
            self.stock = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "stock",
            });
            self.invalidate_hash();
            self.invalidate_validation();
            Ok(())
        }

        pub fn set_virtual(&mut self, v: Option<bool>) -> crate::error::SetResult {
            if self.virtual_ == v {
                return Ok(());
            }
            self.virtual_ = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "virtual",
            });
            self.invalidate_hash();
            self.invalidate_validation();
            Ok(())
        }

        pub fn set_unit(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.unit == v {
                return Ok(());
            }
            self.unit = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "unit",
            });
            self.invalidate_hash();
            self.invalidate_validation();
            Ok(())
        }

        pub fn set_location(&mut self, v: Option<Location>) -> crate::error::SetResult {
            if self.location == v {
                return Ok(());
            }
            self.location = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "location",
            });
            self.invalidate_hash();
            self.invalidate_validation();
            Ok(())
        }

        pub fn set_created(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.created == v {
                return Ok(());
            }
            self.created = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "created",
            });
            self.invalidate_hash();
            self.invalidate_validation();
            Ok(())
        }

        pub fn set_updated(&mut self, v: Option<String>) -> crate::error::SetResult {
            if self.updated == v {
                return Ok(());
            }
            self.updated = v;
            self.emit_ev(KitEvent::FieldChanged {
                entity: self.entity_ref(),
                field: "updated",
            });
            self.invalidate_hash();
            self.invalidate_validation();
            Ok(())
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
            w.tag("type")
                .str(self.guid.as_str())
                .str(&self.name)
                .opt_str(self.description.as_deref())
                .opt_str(self.variant.as_deref())
                .opt_str(self.unit.as_deref());
            for p in &self.ports {
                if let Ok(p) = p.read() {
                    p.hash_into(w);
                }
            }
            for c in &self.connectors {
                if let Ok(c) = c.read() {
                    c.hash_into(w);
                }
            }
            for r in &self.representations {
                if let Ok(r) = r.read() {
                    r.hash_into(w);
                }
            }
            for a in &self.authors {
                if let Ok(a) = a.read() {
                    a.hash_into(w);
                }
            }
            for c in &self.concepts {
                if let Ok(c) = c.read() {
                    c.hash_into(w);
                }
            }
            for t in &self.tags {
                if let Ok(t) = t.read() {
                    t.hash_into(w);
                }
            }
            for q in &self.qualities {
                if let Ok(q) = q.read() {
                    q.hash_into(w);
                }
            }
            for p in &self.props {
                if let Ok(p) = p.read() {
                    p.hash_into(w);
                }
            }
            for a in &self.attributes {
                if let Ok(a) = a.read() {
                    a.hash_into(w);
                }
            }
        }

        pub fn port(&self, guid: &str) -> Option<PortStoreRef> {
            self.ports
                .iter()
                .find(|p| p.read().map(|p| p.guid.as_str() == guid).unwrap_or(false))
                .cloned()
        }

        pub fn connector(&self, guid: &str) -> Option<ConnectorStoreRef> {
            self.connectors
                .iter()
                .find(|c| c.read().map(|c| c.guid.as_str() == guid).unwrap_or(false))
                .cloned()
        }

        pub fn connector_for_port_guid(&self, port_guid: &Guid) -> Option<ConnectorStoreRef> {
            self.connectors
                .iter()
                .find(|c| {
                    c.read()
                        .ok()
                        .and_then(|cr| {
                            cr.port
                                .as_ref()
                                .and_then(|w| w.upgrade())
                                .and_then(|p| p.read().ok().map(|pr| pr.guid == *port_guid))
                        })
                        .unwrap_or(false)
                })
                .cloned()
        }

        pub fn representation(&self, guid: &str) -> Option<RepresentationStoreRef> {
            self.representations
                .iter()
                .find(|r| r.read().map(|r| r.guid.as_str() == guid).unwrap_or(false))
                .cloned()
        }

        pub fn from_id_dto(d: TypeIdDto) -> Self {
            Self {
                guid: d.guid,
                name: String::new(),
                description: None,
                icon: None,
                image: None,
                variant: None,
                stock: None,
                virtual_: None,
                unit: None,
                location: None,
                ports: Vec::new(),
                connectors: Vec::new(),
                representations: Vec::new(),
                authors: Vec::new(),
                concepts: Vec::new(),
                tags: Vec::new(),
                qualities: Vec::new(),
                props: Vec::new(),
                attributes: Vec::new(),
                created: None,
                updated: None,
                parent_kit: Weak::new(),
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        pub fn from_metadata_dto(d: TypeMetadataDto) -> Self {
            Self {
                guid: d.guid,
                name: d.name,
                description: d.description,
                icon: d.icon,
                image: d.image,
                variant: d.variant,
                stock: d.stock,
                virtual_: d.virtual_,
                unit: d.unit,
                location: d.location,
                ports: Vec::new(),
                connectors: Vec::new(),
                representations: Vec::new(),
                authors: Vec::new(),
                concepts: Vec::new(),
                tags: Vec::new(),
                qualities: Vec::new(),
                props: Vec::new(),
                attributes: Vec::new(),
                created: d.created,
                updated: d.updated,
                parent_kit: Weak::new(),
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }
        }

        /// Hydrate type graph from full DTO (ports, connectors, representations, kit link).
        /// Only [`crate::kit::KitStore::from_full_dto`] should construct types in host code.
        pub(crate) fn hydrate_from_full_dto(
            d: TypeFullDto,
            kit: &Arc<RwLock<crate::kit::KitStore>>,
            file_refs: &[crate::file::FileStoreRef],
        ) -> TypeStoreRef {
            let TypeFullDto {
                guid,
                name,
                description,
                icon,
                image,
                variant,
                stock,
                virtual_,
                unit,
                location,
                created,
                updated,
                ports,
                connectors,
                representations,
                authors,
                concepts,
                tags,
                qualities,
                props,
                attributes,
            } = d;

            let t = Arc::new(RwLock::new(TypeStore {
                guid: guid.clone(),
                name: name.clone(),
                description: description.clone(),
                icon: icon.clone(),
                image: image.clone(),
                variant: variant.clone(),
                stock,
                virtual_,
                unit: unit.clone(),
                location,
                ports: Vec::new(),
                connectors: Vec::new(),
                representations: Vec::new(),
                authors: authors
                    .into_iter()
                    .map(|a| Arc::new(RwLock::new(AuthorStore::from_full_dto(a))))
                    .collect(),
                concepts: concepts
                    .into_iter()
                    .map(|c| Arc::new(RwLock::new(ConceptStore::from_full_dto(c))))
                    .collect(),
                tags: tags
                    .into_iter()
                    .map(|t| Arc::new(RwLock::new(TagStore::from_full_dto(t))))
                    .collect(),
                qualities: qualities
                    .into_iter()
                    .map(|q| Arc::new(RwLock::new(QualityStore::from_full_dto(q))))
                    .collect(),
                props: props
                    .into_iter()
                    .map(|p| Arc::new(RwLock::new(PropStore::from_full_dto(p))))
                    .collect(),
                attributes: attributes
                    .into_iter()
                    .map(|a| Arc::new(RwLock::new(AttributeStore::from_full_dto(a))))
                    .collect(),
                created: created.clone(),
                updated: updated.clone(),
                parent_kit: Arc::downgrade(kit),
                event_bus: Weak::new(),
                hash_cache: Cache::default(),
            }));

            let tw_pre = Arc::downgrade(&t);
            let port_refs: Vec<PortStoreRef> = ports
                .into_iter()
                .map(|p| {
                    let mut port = PortStore::from_full_dto(p);
                    port.parent_type = tw_pre.clone();
                    Arc::new(RwLock::new(port))
                })
                .collect();

            let mut connector_refs: Vec<ConnectorStoreRef> = Vec::with_capacity(connectors.len());
            for cdto in connectors {
                let port_guid = cdto.port.as_ref().map(|p| p.guid.clone());
                let mut c = ConnectorStore::from_full_dto(cdto);
                c.parent_type = Arc::downgrade(&t);
                if let Some(pg) = port_guid {
                    if let Some(pref) = port_refs
                        .iter()
                        .find(|p| p.read().map(|p| p.guid == pg).unwrap_or(false))
                    {
                        c.port = Some(Arc::downgrade(pref));
                    }
                }
                connector_refs.push(Arc::new(RwLock::new(c)));
            }

            let mut rep_refs: Vec<RepresentationStoreRef> =
                Vec::with_capacity(representations.len());
            for rdto in representations {
                let file_guid = rdto.file.as_ref().map(|f| f.guid.clone());
                let mut r = RepresentationStore::from_full_dto(rdto);
                r.parent_type = Arc::downgrade(&t);
                if let Some(fg) = file_guid {
                    if let Some(fref) = file_refs
                        .iter()
                        .find(|f| f.read().map(|f| f.guid == fg).unwrap_or(false))
                    {
                        r.file = Some(Arc::downgrade(fref));
                    }
                }
                rep_refs.push(Arc::new(RwLock::new(r)));
            }

            if let Ok(mut t_mut) = t.write() {
                let tw = Arc::downgrade(&t);
                for a in &t_mut.authors {
                    if let Ok(mut aw) = a.write() {
                        aw.parent_type = Some(tw.clone());
                    }
                }
                for c in &t_mut.concepts {
                    if let Ok(mut cw) = c.write() {
                        cw.parent_type = Some(tw.clone());
                    }
                }
                for tag in &t_mut.tags {
                    if let Ok(mut tw0) = tag.write() {
                        tw0.parent_type = Some(tw.clone());
                    }
                }
                for q in &t_mut.qualities {
                    if let Ok(mut qw) = q.write() {
                        qw.parent_type = Some(tw.clone());
                    }
                }
                for p in &t_mut.props {
                    if let Ok(mut pw) = p.write() {
                        pw.parent_type = Some(tw.clone());
                    }
                }
                for a in &t_mut.attributes {
                    if let Ok(mut aw) = a.write() {
                        aw.parent_type = Some(tw.clone());
                    }
                }
                t_mut.ports = port_refs;
                t_mut.connectors = connector_refs;
                t_mut.representations = rep_refs;
            }
            t
        }

        pub fn to_id_dto(&self) -> TypeIdDto {
            TypeIdDto {
                guid: self.guid.clone(),
            }
        }

        pub fn to_metadata_dto(&self) -> TypeMetadataDto {
            TypeMetadataDto {
                guid: self.guid.clone(),
                name: self.name.clone(),
                description: self.description.clone(),
                icon: self.icon.clone(),
                image: self.image.clone(),
                variant: self.variant.clone(),
                stock: self.stock,
                virtual_: self.virtual_,
                unit: self.unit.clone(),
                location: self.location,
                created: self.created.clone(),
                updated: self.updated.clone(),
            }
        }

        pub fn to_shallow_dto(&self) -> TypeShallowDto {
            let m = self.to_metadata_dto();
            TypeShallowDto {
                guid: m.guid,
                name: m.name,
                description: m.description,
                icon: m.icon,
                image: m.image,
                variant: m.variant,
                stock: m.stock,
                virtual_: m.virtual_,
                unit: m.unit,
                location: m.location,
                created: m.created,
                updated: m.updated,
                ports: self
                    .ports
                    .iter()
                    .filter_map(|p| p.read().ok().map(|p| p.to_shallow_dto()))
                    .collect(),
                connectors: self
                    .connectors
                    .iter()
                    .filter_map(|c| c.read().ok().map(|c| c.to_shallow_dto()))
                    .collect(),
                representations: self
                    .representations
                    .iter()
                    .filter_map(|r| r.read().ok().map(|r| r.to_shallow_dto()))
                    .collect(),
                authors: self
                    .authors
                    .iter()
                    .filter_map(|a| a.read().ok().map(|a| a.to_shallow_dto()))
                    .collect(),
                concepts: self
                    .concepts
                    .iter()
                    .filter_map(|c| c.read().ok().map(|c| c.to_shallow_dto()))
                    .collect(),
                tags: self
                    .tags
                    .iter()
                    .filter_map(|t| t.read().ok().map(|t| t.to_shallow_dto()))
                    .collect(),
                qualities: self
                    .qualities
                    .iter()
                    .filter_map(|q| q.read().ok().map(|q| q.to_shallow_dto()))
                    .collect(),
                props: self
                    .props
                    .iter()
                    .filter_map(|p| p.read().ok().map(|p| p.to_shallow_dto()))
                    .collect(),
                attributes: self
                    .attributes
                    .iter()
                    .filter_map(|a| a.read().ok().map(|a| a.to_shallow_dto()))
                    .collect(),
            }
        }

        pub fn to_full_dto(&self) -> TypeFullDto {
            let m = self.to_metadata_dto();
            TypeFullDto {
                guid: m.guid,
                name: m.name,
                description: m.description,
                icon: m.icon,
                image: m.image,
                variant: m.variant,
                stock: m.stock,
                virtual_: m.virtual_,
                unit: m.unit,
                location: m.location,
                created: m.created,
                updated: m.updated,
                ports: self
                    .ports
                    .iter()
                    .filter_map(|p| p.read().ok().map(|p| p.to_full_dto()))
                    .collect(),
                connectors: self
                    .connectors
                    .iter()
                    .filter_map(|c| c.read().ok().map(|c| c.to_full_dto()))
                    .collect(),
                representations: self
                    .representations
                    .iter()
                    .filter_map(|r| r.read().ok().map(|r| r.to_full_dto()))
                    .collect(),
                authors: self
                    .authors
                    .iter()
                    .filter_map(|a| a.read().ok().map(|a| a.to_full_dto()))
                    .collect(),
                concepts: self
                    .concepts
                    .iter()
                    .filter_map(|c| c.read().ok().map(|c| c.to_full_dto()))
                    .collect(),
                tags: self
                    .tags
                    .iter()
                    .filter_map(|t| t.read().ok().map(|t| t.to_full_dto()))
                    .collect(),
                qualities: self
                    .qualities
                    .iter()
                    .filter_map(|q| q.read().ok().map(|q| q.to_full_dto()))
                    .collect(),
                props: self
                    .props
                    .iter()
                    .filter_map(|p| p.read().ok().map(|p| p.to_full_dto()))
                    .collect(),
                attributes: self
                    .attributes
                    .iter()
                    .filter_map(|a| a.read().ok().map(|a| a.to_full_dto()))
                    .collect(),
            }
        }
    }
}

mod async_kit {
    //! Async facades for [`crate::kit::KitStore`]: no lock held across `.await`.
    #![allow(dead_code)]

    use futures_lite::future::ready;

    use crate::diff::{DesignChange, DesignDiff};
    use crate::error::{Result, SemioError};
    use crate::kit::{KitStore, KitStoreRef};
    use crate::report::{SemioReport, ValidationResult};

    impl KitStore {
        pub async fn set_name_async(this: &KitStoreRef, name: String) -> Result<()> {
            let r = (|| {
                let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
                g.set_name(name)?;
                Ok(())
            })();
            ready(r).await
        }

        pub async fn set_description_async(this: &KitStoreRef, v: Option<String>) -> Result<()> {
            let r = (|| {
                let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
                g.set_description(v)?;
                Ok(())
            })();
            ready(r).await
        }

        pub async fn set_icon_async(this: &KitStoreRef, v: Option<String>) -> Result<()> {
            let r = (|| {
                let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
                g.set_icon(v)?;
                Ok(())
            })();
            ready(r).await
        }

        pub async fn set_image_async(this: &KitStoreRef, v: Option<String>) -> Result<()> {
            let r = (|| {
                let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
                g.set_image(v)?;
                Ok(())
            })();
            ready(r).await
        }

        pub async fn set_preview_async(this: &KitStoreRef, v: Option<String>) -> Result<()> {
            let r = (|| {
                let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
                g.set_preview(v)?;
                Ok(())
            })();
            ready(r).await
        }

        pub async fn set_version_async(this: &KitStoreRef, v: Option<String>) -> Result<()> {
            let r = (|| {
                let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
                g.set_version(v)?;
                Ok(())
            })();
            ready(r).await
        }

        pub async fn set_remote_async(this: &KitStoreRef, v: Option<String>) -> Result<()> {
            let r = (|| {
                let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
                g.set_remote(v)?;
                Ok(())
            })();
            ready(r).await
        }

        pub async fn set_homepage_async(this: &KitStoreRef, v: Option<String>) -> Result<()> {
            let r = (|| {
                let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
                g.set_homepage(v)?;
                Ok(())
            })();
            ready(r).await
        }

        pub async fn set_license_async(this: &KitStoreRef, v: Option<String>) -> Result<()> {
            let r = (|| {
                let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
                g.set_license(v)?;
                Ok(())
            })();
            ready(r).await
        }

        pub async fn set_uri_async(this: &KitStoreRef, v: Option<String>) -> Result<()> {
            let r = (|| {
                let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
                g.set_uri(v)?;
                Ok(())
            })();
            ready(r).await
        }

        pub async fn set_created_async(this: &KitStoreRef, v: Option<String>) -> Result<()> {
            let r = (|| {
                let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
                g.set_created(v)?;
                Ok(())
            })();
            ready(r).await
        }

        pub async fn set_updated_async(this: &KitStoreRef, v: Option<String>) -> Result<()> {
            let r = (|| {
                let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
                g.set_updated(v)?;
                Ok(())
            })();
            ready(r).await
        }

        pub async fn hash_async(this: &KitStoreRef) -> Result<String> {
            let r = match this.read() {
                Ok(g) => Ok(g.hash()),
                Err(_) => Err(SemioError::LockPoisoned("kit")),
            };
            ready(r).await
        }

        pub async fn validate_async(this: &KitStoreRef) -> Result<ValidationResult> {
            let r = match this.read() {
                Ok(g) => Ok(g.validate()),
                Err(_) => Err(SemioError::LockPoisoned("kit")),
            };
            ready(r).await
        }

        pub async fn apply_design_diff_async(
            this: &KitStoreRef,
            design_guid: &str,
            diff: &DesignDiff,
        ) -> Result<()> {
            let r = (|| {
                let mut g = this.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
                g.apply_design_diff(design_guid, diff)
            })();
            ready(r).await
        }

        pub async fn flatten_design_async(
            this: &KitStoreRef,
            design_guid: &str,
        ) -> Result<SemioReport<DesignChange>> {
            let r = match this.read() {
                Ok(g) => g.flatten_design(design_guid),
                Err(_) => Err(SemioError::LockPoisoned("kit")),
            };
            ready(r).await
        }
    }
}

pub mod io {
    //! I/O backends for kit persistence. Each backend implements methods on
    //! [`crate::kit::KitStore`] behind its own cfg, keeping the domain layer free of
    //! transport concerns.

    pub mod json {
        use crate::error::Result;
        use crate::kit::{KitFullDto, KitStore, KitStoreRef};

        impl KitStore {
            /// Parse a kit from a JSON string into a fully hydrated graph.
            pub fn from_json_str(s: &str) -> Result<KitStoreRef> {
                let dto: KitFullDto = serde_json::from_str(s)?;
                Ok(KitStore::from_full_dto(dto))
            }

            pub fn to_json_pretty(&self) -> Result<String> {
                Ok(serde_json::to_string_pretty(&self.to_full_dto())?)
            }

            pub fn to_json(&self) -> Result<String> {
                Ok(serde_json::to_string(&self.to_full_dto())?)
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub mod sqlite {
        //! SQLite persistence: stores the full hydrated kit as JSON in a single row.
        //! A normalized multi-table layout can be layered on later without changing
        //! [`KitStore::from_full_dto`] / [`KitStore::to_json`] boundaries.

        use std::path::Path;

        use rusqlite::Connection;

        use crate::error::Result;
        use crate::kit::{KitStore, KitStoreRef};

        impl KitStore {
            /// Preferred API (plan): write kit JSON snapshot to `path`.
            pub fn save_sqlite(&self, path: &Path) -> Result<()> {
                let conn = Connection::open(path)?;
                conn.execute_batch(
                    "CREATE TABLE IF NOT EXISTS semio_kit_snapshot (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                payload TEXT NOT NULL
            );",
                )?;
                let payload = self.to_json()?;
                conn.execute(
                    "INSERT OR REPLACE INTO semio_kit_snapshot (id, payload) VALUES (1, ?1)",
                    [&payload],
                )?;
                Ok(())
            }

            /// Preferred API (plan): load kit from JSON snapshot stored in SQLite.
            pub fn load_sqlite(path: &Path) -> Result<KitStoreRef> {
                let conn = Connection::open(path)?;
                let payload: String = conn.query_row(
                    "SELECT payload FROM semio_kit_snapshot WHERE id = 1",
                    [],
                    |r| r.get(0),
                )?;
                KitStore::from_json_str(&payload)
            }

            /// Back-compat alias for [`KitStore::load_sqlite`].
            pub fn from_sqlite(path: &Path) -> Result<KitStoreRef> {
                Self::load_sqlite(path)
            }

            /// Back-compat alias for [`KitStore::save_sqlite`].
            pub fn to_sqlite(&self, path: &Path) -> Result<()> {
                self.save_sqlite(path)
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub mod zip {
        //! ZIP bundle: `kit.json` (pretty JSON) at archive root. Asset files may be
        //! added in a follow-up; hosts should prefer JSON round-trip for fidelity today.

        use std::fs::File;
        use std::io::{Read, Write};
        use std::path::Path;

        use zip::write::SimpleFileOptions;
        use zip::{CompressionMethod, ZipArchive, ZipWriter};

        use crate::error::{Result, SemioError};
        use crate::kit::{KitStore, KitStoreRef};

        const KIT_JSON: &str = "kit.json";

        impl KitStore {
            /// Preferred API (plan): write `kit.json` into a zip at `path`.
            pub fn save_zip(&self, path: &Path) -> Result<()> {
                let file = File::create(path)?;
                let mut zip = ZipWriter::new(file);
                let opts =
                    SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
                zip.start_file(KIT_JSON, opts)?;
                zip.write_all(self.to_json_pretty()?.as_bytes())?;
                zip.finish()?;
                Ok(())
            }

            /// Preferred API (plan): read `kit.json` from a zip at `path`.
            pub fn load_zip(path: &Path) -> Result<KitStoreRef> {
                let file = File::open(path)?;
                let mut archive = ZipArchive::new(file)?;
                let mut kit_json = String::new();
                for i in 0..archive.len() {
                    let mut entry = archive.by_index(i)?;
                    if entry.name() == KIT_JSON {
                        entry.read_to_string(&mut kit_json)?;
                        break;
                    }
                }
                if kit_json.is_empty() {
                    return Err(SemioError::InvalidOperation(format!(
                        "zip missing {KIT_JSON}"
                    )));
                }
                KitStore::from_json_str(&kit_json)
            }

            pub fn from_zip(path: &Path) -> Result<KitStoreRef> {
                Self::load_zip(path)
            }

            pub fn to_zip(&self, path: &Path) -> Result<()> {
                self.save_zip(path)
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub mod wasm {
    //! WASM bindings preserving the JS-visible names used by the TypeScript SDK.
    //! I/O-style entry points return Promises (`future_to_promise`) so hosts can await them.

    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::future_to_promise;

    use crate::error::SetError;
    use crate::guid::Guid;
    use crate::kit::{KitStore, KitStoreRef};

    fn js_settle_set(r: crate::error::SetResult) -> Result<JsValue, JsValue> {
        match r {
            Ok(()) => serde_wasm_bindgen::to_value(&serde_json::json!({ "ok": true }))
                .map_err(|e| JsValue::from_str(&e.to_string())),
            Err(err) => serde_wasm_bindgen::to_value(&serde_json::json!({
                "ok": false,
                "error": err
            }))
            .map_err(|e| JsValue::from_str(&e.to_string())),
        }
    }

    #[wasm_bindgen(js_name = generateGuid)]
    pub fn wasm_generate_guid() -> String {
        Guid::new_v7().into_string()
    }

    #[wasm_bindgen(js_name = kitFromJson)]
    pub fn wasm_kit_from_json(s: &str) -> js_sys::Promise {
        let s = s.to_string();
        future_to_promise(async move {
            match KitStore::from_json_str(&s) {
                Ok(kit) => {
                    let guard = kit
                        .read()
                        .map_err(|_| JsValue::from_str("kit lock poisoned"))?;
                    serde_wasm_bindgen::to_value(&guard.to_full_dto())
                        .map_err(|e| JsValue::from_str(&e.to_string()))
                }
                Err(e) => Err(JsValue::from_str(&e.to_string())),
            }
        })
    }

    #[wasm_bindgen(js_name = kitToJson)]
    pub fn wasm_kit_to_json(value: JsValue) -> js_sys::Promise {
        future_to_promise(async move {
            let dto: crate::kit::KitFullDto = serde_wasm_bindgen::from_value(value)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            let kit = KitStore::from_full_dto(dto);
            let guard = kit
                .read()
                .map_err(|_| JsValue::from_str("kit lock poisoned"))?;
            guard
                .to_json_pretty()
                .map(|json| JsValue::from_str(&json))
                .map_err(|e| JsValue::from_str(&e.to_string()))
        })
    }

    #[wasm_bindgen(js_name = kitValidate)]
    pub fn wasm_kit_validate(value: JsValue) -> js_sys::Promise {
        future_to_promise(async move {
            let dto: crate::kit::KitFullDto = serde_wasm_bindgen::from_value(value)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            let kit = KitStore::from_full_dto(dto);
            match KitStore::validate_async(&kit).await {
                Ok(v) => {
                    serde_wasm_bindgen::to_value(&v).map_err(|e| JsValue::from_str(&e.to_string()))
                }
                Err(e) => Err(JsValue::from_str(&e.to_string())),
            }
        })
    }

    #[wasm_bindgen(js_name = kitsAreEqual)]
    pub fn wasm_kits_are_equal(a: JsValue, b: JsValue) -> js_sys::Promise {
        future_to_promise(async move {
            let a: crate::kit::KitFullDto =
                serde_wasm_bindgen::from_value(a).map_err(|e| JsValue::from_str(&e.to_string()))?;
            let b: crate::kit::KitFullDto =
                serde_wasm_bindgen::from_value(b).map_err(|e| JsValue::from_str(&e.to_string()))?;
            let ka = KitStore::from_full_dto(a);
            let kb = KitStore::from_full_dto(b);
            let ga = ka.read().map_err(|_| JsValue::from_str("a poisoned"))?;
            let gb = kb.read().map_err(|_| JsValue::from_str("b poisoned"))?;
            Ok(JsValue::from_bool(ga.are_equal(&gb)))
        })
    }

    #[wasm_bindgen(js_name = flattenDesign)]
    pub fn wasm_flatten_design(kit: JsValue, design_guid: &str) -> js_sys::Promise {
        let design_guid = design_guid.to_string();
        future_to_promise(async move {
            let dto: crate::kit::KitFullDto = serde_wasm_bindgen::from_value(kit)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            let k = KitStore::from_full_dto(dto);
            match KitStore::flatten_design_async(&k, &design_guid).await {
                Ok(rep) => serde_wasm_bindgen::to_value(&rep)
                    .map_err(|e| JsValue::from_str(&e.to_string())),
                Err(e) => Err(JsValue::from_str(&e.to_string())),
            }
        })
    }

    #[wasm_bindgen(js_name = semioRound)]
    pub fn wasm_round(value: f64, decimals: i32) -> f64 {
        let m = 10f64.powi(decimals);
        (value * m).round() / m
    }

    #[wasm_bindgen(js_name = semioNormalizeName)]
    pub fn wasm_normalize_name(s: &str) -> String {
        s.trim()
            .to_ascii_lowercase()
            .replace(|c: char| c.is_whitespace(), "-")
    }

    /// 🌐 Stateful [`KitStoreRef`] for web-worker-hosted mutations + event stream.
    #[wasm_bindgen]
    pub struct KitStoreHandle {
        inner: KitStoreRef,
    }

    #[wasm_bindgen]
    impl KitStoreHandle {
        #[wasm_bindgen(js_name = create)]
        pub fn create(dto: JsValue) -> Result<KitStoreHandle, JsValue> {
            let dto: crate::kit::KitFullDto = serde_wasm_bindgen::from_value(dto)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(KitStoreHandle {
                inner: KitStore::from_full_dto(dto),
            })
        }

        #[wasm_bindgen(js_name = snapshot)]
        pub fn snapshot(&self) -> Result<JsValue, JsValue> {
            let g = self
                .inner
                .read()
                .map_err(|_| JsValue::from_str("kit lock poisoned"))?;
            serde_wasm_bindgen::to_value(&g.to_full_dto())
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = getField)]
        pub fn get_field(&self, kind: &str, guid: &str, field: &str) -> Result<JsValue, JsValue> {
            let ek = KitStore::parse_entity_kind(kind)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            let v = KitStore::get_field_rpc(&self.inner, ek, guid, field)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            serde_wasm_bindgen::to_value(&v).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = setField)]
        pub fn set_field(
            &self,
            kind: &str,
            guid: &str,
            field: &str,
            value: JsValue,
        ) -> js_sys::Promise {
            let inner = self.inner.clone();
            let kind = kind.to_string();
            let guid = guid.to_string();
            let field = field.to_string();
            future_to_promise(async move {
                let ek = match KitStore::parse_entity_kind(&kind) {
                    Ok(v) => v,
                    Err(e) => return js_settle_set(Err(e)),
                };
                let val: serde_json::Value = match serde_wasm_bindgen::from_value(value) {
                    Ok(v) => v,
                    Err(e) => {
                        return js_settle_set(Err(SetError::InvalidValue(e.to_string())));
                    }
                };
                js_settle_set(KitStore::set_field_rpc(&inner, ek, &guid, &field, val))
            })
        }

        #[wasm_bindgen(js_name = addChild)]
        pub fn add_child(
            &self,
            parent_kind: &str,
            parent_guid: &str,
            child_kind: &str,
            dto: JsValue,
        ) -> js_sys::Promise {
            let inner = self.inner.clone();
            let pk = parent_kind.to_string();
            let pg = parent_guid.to_string();
            let ck = child_kind.to_string();
            future_to_promise(async move {
                let pk = match KitStore::parse_entity_kind(&pk) {
                    Ok(v) => v,
                    Err(e) => return js_settle_set(Err(e)),
                };
                let ck = match KitStore::parse_entity_kind(&ck) {
                    Ok(v) => v,
                    Err(e) => return js_settle_set(Err(e)),
                };
                let dto: serde_json::Value = match serde_wasm_bindgen::from_value(dto) {
                    Ok(v) => v,
                    Err(e) => {
                        return js_settle_set(Err(SetError::InvalidValue(e.to_string())));
                    }
                };
                js_settle_set(KitStore::add_child_rpc(
                    &inner, pk, &pg, ck, dto,
                ))
            })
        }

        #[wasm_bindgen(js_name = removeChild)]
        pub fn remove_child(
            &self,
            parent_kind: &str,
            parent_guid: &str,
            child_kind: &str,
            child_guid: &str,
        ) -> js_sys::Promise {
            let inner = self.inner.clone();
            let pk = parent_kind.to_string();
            let pg = parent_guid.to_string();
            let ck = child_kind.to_string();
            let cg = child_guid.to_string();
            future_to_promise(async move {
                let pk = match KitStore::parse_entity_kind(&pk) {
                    Ok(v) => v,
                    Err(e) => return js_settle_set(Err(e)),
                };
                let ck = match KitStore::parse_entity_kind(&ck) {
                    Ok(v) => v,
                    Err(e) => return js_settle_set(Err(e)),
                };
                js_settle_set(KitStore::remove_child_rpc(
                    &inner, pk, &pg, ck, &cg,
                ))
            })
        }

        #[wasm_bindgen(js_name = applyDesignDiff)]
        pub fn apply_design_diff(&self, design_guid: &str, diff: JsValue) -> js_sys::Promise {
            let inner = self.inner.clone();
            let dg = design_guid.to_string();
            future_to_promise(async move {
                let diff: serde_json::Value = match serde_wasm_bindgen::from_value(diff) {
                    Ok(v) => v,
                    Err(e) => {
                        return js_settle_set(Err(SetError::InvalidValue(e.to_string())));
                    }
                };
                js_settle_set(KitStore::apply_design_diff_rpc(&inner, &dg, diff))
            })
        }

        #[wasm_bindgen(js_name = subscribe)]
        pub fn subscribe(&self, callback: &js_sys::Function) -> Result<(), JsValue> {
            let mut rx = self
                .inner
                .read()
                .map_err(|_| JsValue::from_str("kit lock poisoned"))?
                .subscribe();
            let cb = callback.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loop {
                    match rx.recv().await {
                        Ok(ev) => {
                            if let Ok(v) = serde_wasm_bindgen::to_value(&ev) {
                                let _ = cb.call1(&JsValue::NULL, &v);
                            }
                        }
                        Err(async_broadcast::RecvError::Closed) => break,
                        Err(async_broadcast::RecvError::Overflowed(_)) => {}
                    }
                }
            });
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    //! Integration-style tests (in-crate) for JSON/hash and I/O helpers.
    #![allow(unused_must_use)]

    mod io_json {
        use std::sync::{Arc, RwLock};

        use crate::kit::KitStore;

        #[test]
        fn kit_json_roundtrip_hash_stable() {
            let kit = Arc::new(RwLock::new(KitStore::new("roundtrip-test")));
            let json = kit.read().expect("read").to_json_pretty().expect("to json");
            let kit2 = KitStore::from_json_str(&json).expect("from json");
            assert_eq!(
                kit.read().expect("read").hash(),
                kit2.read().expect("read2").hash(),
                "hash stable across JSON round-trip"
            );
        }
    }

    mod diff {
        use crate::design::DesignStore;
        use crate::diff::DesignDiff;

        #[test]
        fn diff_between_identical_designs_empty() {
            let a = DesignStore::new("d");
            let b = DesignStore::new("d");
            let da = a.to_full_dto();
            let db = b.to_full_dto();
            let d = DesignDiff::between(&da, &db);
            assert!(d.added_pieces.is_empty());
            assert!(d.removed_pieces.is_empty());
            assert!(d.modified_pieces.is_empty());
            assert!(d.added_connections.is_empty());
            assert!(d.removed_connections.is_empty());
            assert!(d.modified_connections.is_empty());
        }
    }

    mod flatten {
        use crate::design::DesignStore;

        #[test]
        fn flatten_map_empty_design() {
            let d = DesignStore::new("x");
            let m = d.flatten_map();
            assert!(m.is_empty());
        }
    }

    mod invalidation {
        use std::sync::{Arc, RwLock};

        use crate::kit::KitStore;

        #[test]
        fn kit_name_change_recomputes_validation() {
            let kit = Arc::new(RwLock::new(KitStore::new("ok")));
            assert!(kit.read().expect("r").validate().is_valid);
            assert!(kit.write().expect("w").set_name("   ".to_string()).is_err());
            assert!(kit.read().expect("r").validate().is_valid);
            assert_eq!(kit.read().expect("r").name, "ok");
        }
    }

    mod validation {
        use crate::kit::KitStore;

        #[test]
        fn validate_empty_kit_name_fails() {
            let mut k = KitStore::new(" ");
            k.name = "  ".to_string();
            let v = k.validate();
            assert!(!v.is_valid);
            assert!(v.errors.iter().any(|e| e.contains("kit.name")));
        }
    }

    mod entities {
        use crate::author::AuthorStore;

        #[test]
        fn author_setter_invalidates_local_hash() {
            let mut a = AuthorStore::from_full_dto(crate::author::AuthorFullDto {
                guid: crate::Guid::new_v7(),
                name: "n".into(),
                email: "e".into(),
                role: None,
                rank: None,
            });
            let h0 = a.hash();
            a.set_name("n2".into());
            let h1 = a.hash();
            assert_ne!(h0, h1);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    mod io_sqlite {
        use std::sync::{Arc, RwLock};

        use tempfile::tempdir;

        use crate::kit::KitStore;

        #[test]
        fn sqlite_snapshot_roundtrip() {
            let kit = Arc::new(RwLock::new(KitStore::new("sqlite-roundtrip")));
            let dir = tempdir().expect("tempdir");
            let path = dir.path().join("kit.db");
            kit.read().expect("read").save_sqlite(&path).expect("save");
            let kit2 = KitStore::load_sqlite(&path).expect("load");
            assert_eq!(
                kit.read().expect("r1").hash(),
                kit2.read().expect("r2").hash(),
                "SQLite JSON snapshot preserves hash"
            );
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    mod io_zip {
        use std::sync::{Arc, RwLock};

        use tempfile::tempdir;

        use crate::kit::KitStore;

        #[test]
        fn zip_kit_json_roundtrip() {
            let kit = Arc::new(RwLock::new(KitStore::new("zip-roundtrip")));
            let dir = tempdir().expect("tempdir");
            let path = dir.path().join("kit.zip");
            kit.read().expect("read").save_zip(&path).expect("save zip");
            let kit2 = KitStore::load_zip(&path).expect("load zip");
            assert_eq!(
                kit.read().expect("r1").hash(),
                kit2.read().expect("r2").hash(),
                "zip kit.json preserves hash"
            );
        }
    }

    mod events {
        //! Per-entity event sequence coverage (see workspace plan: semio-rs async events).

        mod common {
            //! Helpers for event-sequence tests.

            use crate::connection::ConnectionFullDto;
            use crate::connector::ConnectorFullDto;
            use crate::design::DesignFullDto;
            use crate::events::{EntityKind, EntityRef, KitEvent};
            use crate::file::FileFullDto;
            use crate::group::GroupFullDto;
            use crate::guid::Guid;
            use crate::kit::{KitFullDto, KitStore, KitStoreRef};
            use crate::layer::LayerFullDto;
            use crate::piece::{PieceFullDto, PieceIdDto};
            use crate::port::{PortFullDto, PortIdDto};
            use crate::side::SideMetadataDto;
            use crate::typ::{TypeFullDto, TypeIdDto};

            /// Drain all currently queued events from a broadcast receiver (non-blocking).
            pub fn drain(rx: &mut async_broadcast::Receiver<KitEvent>) -> Vec<KitEvent> {
                let mut out = Vec::new();
                while let Ok(e) = rx.try_recv() {
                    out.push(e);
                }
                out
            }

            pub fn kit_entity_ref(kit: &KitStoreRef) -> EntityRef {
                let g = kit.read().expect("kit read").guid.clone();
                EntityRef::new(EntityKind::Kit, g)
            }

            /// Minimal kit with one type, one design, one piece (valid type ref).
            pub fn kit_with_piece() -> (KitStoreRef, Guid, Guid, Guid) {
                let type_guid = Guid::new_v7();
                let design_guid = Guid::new_v7();
                let piece_guid = Guid::new_v7();
                let kit_guid = Guid::new_v7();

                let dto = KitFullDto {
                    guid: kit_guid,
                    name: "kit".into(),
                    types: vec![TypeFullDto {
                        guid: type_guid.clone(),
                        name: "typ".into(),
                        ..Default::default()
                    }],
                    designs: vec![DesignFullDto {
                        guid: design_guid.clone(),
                        name: "des".into(),
                        pieces: vec![PieceFullDto {
                            guid: piece_guid.clone(),
                            r#type: Some(TypeIdDto {
                                guid: type_guid.clone(),
                            }),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }],
                    ..Default::default()
                };
                let kit = KitStore::from_full_dto(dto);
                (kit, type_guid, design_guid, piece_guid)
            }

            /// One design with a layer and one piece (piece required for valid design content).
            pub fn kit_with_layer() -> (KitStoreRef, Guid, Guid) {
                let type_guid = Guid::new_v7();
                let design_guid = Guid::new_v7();
                let piece_guid = Guid::new_v7();
                let layer_guid = Guid::new_v7();
                let kit_guid = Guid::new_v7();
                let dto = KitFullDto {
                    guid: kit_guid,
                    name: "kit".into(),
                    types: vec![TypeFullDto {
                        guid: type_guid.clone(),
                        name: "typ".into(),
                        ..Default::default()
                    }],
                    designs: vec![DesignFullDto {
                        guid: design_guid.clone(),
                        name: "des".into(),
                        pieces: vec![PieceFullDto {
                            guid: piece_guid.clone(),
                            r#type: Some(TypeIdDto {
                                guid: type_guid.clone(),
                            }),
                            ..Default::default()
                        }],
                        layers: vec![LayerFullDto {
                            guid: layer_guid.clone(),
                            name: "L".into(),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }],
                    ..Default::default()
                };
                let kit = KitStore::from_full_dto(dto);
                (kit, design_guid, layer_guid)
            }

            /// Design with one group referencing the single piece.
            pub fn kit_with_group() -> (KitStoreRef, Guid, Guid) {
                let type_guid = Guid::new_v7();
                let design_guid = Guid::new_v7();
                let piece_guid = Guid::new_v7();
                let group_guid = Guid::new_v7();
                let kit_guid = Guid::new_v7();
                let dto = KitFullDto {
                    guid: kit_guid,
                    name: "kit".into(),
                    types: vec![TypeFullDto {
                        guid: type_guid.clone(),
                        name: "typ".into(),
                        ..Default::default()
                    }],
                    designs: vec![DesignFullDto {
                        guid: design_guid.clone(),
                        name: "des".into(),
                        pieces: vec![PieceFullDto {
                            guid: piece_guid.clone(),
                            r#type: Some(TypeIdDto {
                                guid: type_guid.clone(),
                            }),
                            ..Default::default()
                        }],
                        groups: vec![GroupFullDto {
                            guid: group_guid.clone(),
                            name: "G".into(),
                            pieces: vec![PieceIdDto {
                                guid: piece_guid.clone(),
                            }],
                            ..Default::default()
                        }],
                        ..Default::default()
                    }],
                    ..Default::default()
                };
                let kit = KitStore::from_full_dto(dto);
                (kit, design_guid, group_guid)
            }

            pub fn kit_with_type_connector() -> (KitStoreRef, Guid, Guid) {
                let type_guid = Guid::new_v7();
                let port_guid = Guid::new_v7();
                let conn_guid = Guid::new_v7();
                let kit_guid = Guid::new_v7();
                let dto = KitFullDto {
                    guid: kit_guid,
                    name: "kit".into(),
                    types: vec![TypeFullDto {
                        guid: type_guid.clone(),
                        name: "typ".into(),
                        ports: vec![PortFullDto {
                            guid: port_guid.clone(),
                            ..Default::default()
                        }],
                        connectors: vec![ConnectorFullDto {
                            guid: conn_guid.clone(),
                            code: "C".into(),
                            port: Some(PortIdDto {
                                guid: port_guid.clone(),
                            }),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }],
                    ..Default::default()
                };
                let kit = KitStore::from_full_dto(dto);
                (kit, type_guid, conn_guid)
            }

            /// Kit with one type containing one port (for port setter tests).
            pub fn kit_with_type_only() -> (KitStoreRef, Guid) {
                let type_guid = Guid::new_v7();
                let kit_guid = Guid::new_v7();
                let dto = KitFullDto {
                    guid: kit_guid,
                    name: "kit".into(),
                    types: vec![TypeFullDto {
                        guid: type_guid.clone(),
                        name: "typ".into(),
                        ..Default::default()
                    }],
                    ..Default::default()
                };
                (KitStore::from_full_dto(dto), type_guid)
            }

            pub fn kit_with_port() -> (KitStoreRef, Guid, Guid) {
                let type_guid = Guid::new_v7();
                let port_guid = Guid::new_v7();
                let kit_guid = Guid::new_v7();
                let dto = KitFullDto {
                    guid: kit_guid,
                    name: "kit".into(),
                    types: vec![TypeFullDto {
                        guid: type_guid.clone(),
                        name: "typ".into(),
                        ports: vec![PortFullDto {
                            guid: port_guid.clone(),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }],
                    ..Default::default()
                };
                let kit = KitStore::from_full_dto(dto);
                (kit, type_guid, port_guid)
            }

            pub fn kit_with_file() -> (KitStoreRef, Guid) {
                let file_guid = Guid::new_v7();
                let kit_guid = Guid::new_v7();
                let dto = KitFullDto {
                    guid: kit_guid,
                    name: "kit".into(),
                    files: vec![FileFullDto {
                        guid: file_guid.clone(),
                        url: "https://example.com/f".into(),
                        ..Default::default()
                    }],
                    ..Default::default()
                };
                let kit = KitStore::from_full_dto(dto);
                (kit, file_guid)
            }

            /// One design, two pieces, one connection (for connection / side tests).
            pub fn kit_with_connection() -> (KitStoreRef, Guid, Guid, Guid, Guid, Guid) {
                let type_guid = Guid::new_v7();
                let design_guid = Guid::new_v7();
                let piece_a = Guid::new_v7();
                let piece_b = Guid::new_v7();
                let conn_guid = Guid::new_v7();
                let side_a = Guid::new_v7();
                let side_b = Guid::new_v7();
                let kit_guid = Guid::new_v7();

                let dto = KitFullDto {
                    guid: kit_guid,
                    name: "kit".into(),
                    types: vec![TypeFullDto {
                        guid: type_guid.clone(),
                        name: "typ".into(),
                        ..Default::default()
                    }],
                    designs: vec![DesignFullDto {
                        guid: design_guid.clone(),
                        name: "des".into(),
                        pieces: vec![
                            PieceFullDto {
                                guid: piece_a.clone(),
                                r#type: Some(TypeIdDto {
                                    guid: type_guid.clone(),
                                }),
                                ..Default::default()
                            },
                            PieceFullDto {
                                guid: piece_b.clone(),
                                r#type: Some(TypeIdDto {
                                    guid: type_guid.clone(),
                                }),
                                ..Default::default()
                            },
                        ],
                        connections: vec![ConnectionFullDto {
                            guid: conn_guid.clone(),
                            connected: SideMetadataDto {
                                guid: side_a,
                                piece: PieceIdDto {
                                    guid: piece_a.clone(),
                                },
                                port: None,
                                design_piece: None,
                            },
                            connecting: SideMetadataDto {
                                guid: side_b,
                                piece: PieceIdDto {
                                    guid: piece_b.clone(),
                                },
                                port: None,
                                design_piece: None,
                            },
                            ..Default::default()
                        }],
                        ..Default::default()
                    }],
                    ..Default::default()
                };
                let kit = KitStore::from_full_dto(dto);
                (kit, type_guid, design_guid, piece_a, piece_b, conn_guid)
            }

            /// Design metadata change with a single piece: field change, design hash, flatten+derived, kit hash, validation.
            pub fn assert_design_scalar_metadata_events(
                evs: &[KitEvent],
                design_er: EntityRef,
                kit_er: EntityRef,
                piece_g: &Guid,
                field: &'static str,
            ) {
                assert_eq!(evs.len(), 7, "{evs:?}");
                assert!(
                    matches!(&evs[0], KitEvent::FieldChanged { entity, field: f } if *entity == design_er && *f == field),
                    "ev0 {:?}",
                    evs.get(0)
                );
                assert!(
                    matches!(&evs[1], KitEvent::HashInvalidated { entity } if *entity == design_er),
                    "ev1 {:?}",
                    evs.get(1)
                );
                assert!(
                    matches!(
                        &evs[2],
                        KitEvent::FlattenInvalidated { design, pieces }
                            if *design == design_er.guid && pieces.len() == 1 && pieces[0] == *piece_g
                    ),
                    "ev2 {:?}",
                    evs.get(2)
                );
                assert!(
                    matches!(
                        &evs[3],
                        KitEvent::DerivedChanged { entity, field: "flat_plane" } if entity.guid == *piece_g
                    ),
                    "ev3 {:?}",
                    evs.get(3)
                );
                assert!(
                    matches!(
                        &evs[4],
                        KitEvent::DerivedChanged { entity, field: "flat_center" } if entity.guid == *piece_g
                    ),
                    "ev4 {:?}",
                    evs.get(4)
                );
                assert!(
                    matches!(&evs[5], KitEvent::HashInvalidated { entity } if *entity == kit_er),
                    "ev5 {:?}",
                    evs.get(5)
                );
                assert!(
                    matches!(evs[6], KitEvent::ValidationInvalidated),
                    "ev6 {:?}",
                    evs.get(6)
                );
            }

            pub fn assert_piece_geometry_change(
                evs: &[KitEvent],
                piece_er: EntityRef,
                design_er: EntityRef,
                kit_er: EntityRef,
                piece_g: &Guid,
                field: &'static str,
            ) {
                assert_eq!(evs.len(), 7, "{evs:?}");
                assert!(
                    matches!(&evs[0], KitEvent::FieldChanged { entity, field: f } if *entity == piece_er && *f == field)
                );
                assert!(
                    matches!(&evs[1], KitEvent::HashInvalidated { entity } if *entity == piece_er)
                );
                assert!(
                    matches!(&evs[2], KitEvent::HashInvalidated { entity } if *entity == design_er)
                );
                assert!(
                    matches!(&evs[3], KitEvent::HashInvalidated { entity } if *entity == kit_er)
                );
                assert!(
                    matches!(&evs[4], KitEvent::FlattenInvalidated { design, pieces } if *design == design_er.guid && pieces.contains(piece_g)),
                    "ev4 {:?}",
                    evs.get(4)
                );
                assert!(
                    matches!(&evs[5], KitEvent::DerivedChanged { entity, field: "flat_plane" } if entity.guid == *piece_g)
                );
                assert!(
                    matches!(&evs[6], KitEvent::DerivedChanged { entity, field: "flat_center" } if entity.guid == *piece_g)
                );
            }

            pub fn assert_piece_scalar_hash_only(
                evs: &[KitEvent],
                piece_er: EntityRef,
                design_er: EntityRef,
                kit_er: EntityRef,
                field: &'static str,
            ) {
                assert_eq!(evs.len(), 4, "{evs:?}");
                assert!(
                    matches!(&evs[0], KitEvent::FieldChanged { entity, field: f } if *entity == piece_er && *f == field)
                );
                assert!(
                    matches!(&evs[1], KitEvent::HashInvalidated { entity } if *entity == piece_er)
                );
                assert!(
                    matches!(&evs[2], KitEvent::HashInvalidated { entity } if *entity == design_er)
                );
                assert!(
                    matches!(&evs[3], KitEvent::HashInvalidated { entity } if *entity == kit_er)
                );
            }

            pub fn assert_type_metadata_core(
                evs: &[KitEvent],
                typ_er: EntityRef,
                kit_er: EntityRef,
                field: &'static str,
            ) {
                assert_eq!(evs.len(), 4, "{evs:?}");
                assert!(
                    matches!(&evs[0], KitEvent::FieldChanged { entity, field: f } if *entity == typ_er && *f == field)
                );
                assert!(
                    matches!(&evs[1], KitEvent::HashInvalidated { entity } if *entity == typ_er)
                );
                assert!(
                    matches!(&evs[2], KitEvent::HashInvalidated { entity } if *entity == kit_er)
                );
                assert!(matches!(evs[3], KitEvent::ValidationInvalidated));
            }

            /// Assert the first events match: FieldChanged(field), HashInvalidated(self), ValidationInvalidated.
            pub fn assert_kit_metadata_core(
                evs: &[KitEvent],
                kit_ref: EntityRef,
                field: &'static str,
            ) {
                assert!(evs.len() >= 3, "expected at least 3 events, got {:?}", evs);
                assert!(
                    matches!(
                        &evs[0],
                        KitEvent::FieldChanged { entity, field: f }
                            if *entity == kit_ref && *f == field
                    ),
                    "ev[0] want FieldChanged {{ field: {} }}, got {:?}",
                    field,
                    evs.get(0)
                );
                assert!(
                    matches!(&evs[1], KitEvent::HashInvalidated { entity } if *entity == kit_ref),
                    "ev[1] want HashInvalidated kit, got {:?}",
                    evs.get(1)
                );
                assert!(
                    matches!(evs[2], KitEvent::ValidationInvalidated),
                    "ev[2] want ValidationInvalidated, got {:?}",
                    evs.get(2)
                );
            }
        }

        mod attribute {
            use crate::attribute::AttributeFullDto;
            use crate::events::KitEvent;
            use crate::guid::Guid;
            use crate::kit::{KitFullDto, KitStore};

            #[test]
            fn attribute_set_value_emits() {
                let g = Guid::new_v7();
                let kit = KitStore::from_full_dto(KitFullDto {
                    guid: Guid::new_v7(),
                    name: "k".into(),
                    attributes: vec![AttributeFullDto {
                        guid: g.clone(),
                        key: "k".into(),
                        value: "v".into(),
                        definition: None,
                    }],
                    ..Default::default()
                });
                let mut rx = kit.read().unwrap().subscribe();
                let a = {
                    let kr = kit.read().unwrap();
                    kr.attributes[0].clone()
                };
                a.write().unwrap().set_value("v2".into());
                let evs = super::common::drain(&mut rx);
                assert!(evs
                    .iter()
                    .any(|e| matches!(e, KitEvent::FieldChanged { field: "value", .. })));
            }
        }

        mod author {
            use crate::author::AuthorFullDto;
            use crate::events::KitEvent;
            use crate::guid::Guid;
            use crate::kit::{KitFullDto, KitStore};

            #[test]
            fn author_set_email_emits() {
                let ag = Guid::new_v7();
                let kit = KitStore::from_full_dto(KitFullDto {
                    guid: Guid::new_v7(),
                    name: "k".into(),
                    authors: vec![AuthorFullDto {
                        guid: ag.clone(),
                        name: "n".into(),
                        email: "e@x".into(),
                        role: None,
                        rank: None,
                    }],
                    ..Default::default()
                });
                let mut rx = kit.read().unwrap().subscribe();
                let a = {
                    let kr = kit.read().unwrap();
                    kr.authors[0].clone()
                };
                a.write().unwrap().set_email("e2@x".into());
                let evs = super::common::drain(&mut rx);
                assert!(evs
                    .iter()
                    .any(|e| matches!(e, KitEvent::FieldChanged { field: "email", .. })));
            }
        }

        mod backbone {
            use std::sync::Arc;

            use super::common::drain;
            use crate::kit::KitStore;
            use crate::KitStoreRef;

            #[test]
            fn subscribe_receives_ordered_stream() {
                let kit: KitStoreRef = Arc::new(std::sync::RwLock::new(KitStore::new("a")));
                let mut a = kit.read().unwrap().subscribe();
                let mut b = kit.read().unwrap().subscribe();
                kit.write().unwrap().set_name("b".into()).unwrap();
                let ea = drain(&mut a);
                let eb = drain(&mut b);
                assert_eq!(ea, eb);
                assert!(!ea.is_empty());
            }

            #[test]
            fn no_lock_held_across_concurrent_read_and_async_setter() {
                let kit: KitStoreRef = Arc::new(std::sync::RwLock::new(KitStore::new("c")));
                let k2 = kit.clone();
                futures_lite::future::block_on(async {
                    let _ = futures_lite::future::zip(
                        crate::KitStore::set_name_async(&k2, "d".into()),
                        async { k2.read().map(|_| ()).unwrap_or(()) },
                    )
                    .await;
                });
                assert_eq!(kit.read().unwrap().name, "d");
            }

            #[test]
            fn drop_kit_closes_bus() {
                let kit: KitStoreRef = Arc::new(std::sync::RwLock::new(KitStore::new("e")));
                let mut rx = kit.read().unwrap().subscribe();
                drop(kit);
                let r = futures_lite::future::block_on(rx.recv());
                assert_eq!(r, Err(async_broadcast::RecvError::Closed));
            }
        }

        mod benchmark {
            use crate::benchmark::BenchmarkFullDto;
            use crate::events::KitEvent;
            use crate::guid::Guid;
            use crate::kit::{KitFullDto, KitStore};
            use crate::port::PortFullDto;
            use crate::quality::QualityFullDto;
            use crate::typ::TypeFullDto;

            #[test]
            fn benchmark_set_min_emits() {
                let tg = Guid::new_v7();
                let pg = Guid::new_v7();
                let qg = Guid::new_v7();
                let bg = Guid::new_v7();
                let kit = KitStore::from_full_dto(KitFullDto {
                    guid: Guid::new_v7(),
                    name: "k".into(),
                    types: vec![TypeFullDto {
                        guid: tg.clone(),
                        name: "t".into(),
                        ports: vec![PortFullDto {
                            guid: pg.clone(),
                            qualities: vec![QualityFullDto {
                                guid: qg.clone(),
                                key: "qk".into(),
                                benchmarks: vec![BenchmarkFullDto {
                                    guid: bg.clone(),
                                    name: "bn".into(),
                                    ..Default::default()
                                }],
                                ..Default::default()
                            }],
                            ..Default::default()
                        }],
                        ..Default::default()
                    }],
                    ..Default::default()
                });
                let mut rx = kit.read().unwrap().subscribe();
                let b = {
                    let kr = kit.read().unwrap();
                    let t = kr.types[0].clone();
                    let tr = t.read().unwrap();
                    let p = tr.ports[0].clone();
                    let pr = p.read().unwrap();
                    let q = pr.qualities[0].clone();
                    let qr = q.read().unwrap();
                    qr.benchmarks[0].clone()
                };
                b.write().unwrap().set_min(Some(1.0));
                let evs = super::common::drain(&mut rx);
                assert!(evs
                    .iter()
                    .any(|e| matches!(e, KitEvent::FieldChanged { field: "min", .. })));
            }
        }

        mod concept {
            use crate::concept::ConceptFullDto;
            use crate::events::KitEvent;
            use crate::guid::Guid;
            use crate::kit::{KitFullDto, KitStore};

            #[test]
            fn concept_set_name_emits() {
                let g = Guid::new_v7();
                let kit = KitStore::from_full_dto(KitFullDto {
                    guid: Guid::new_v7(),
                    name: "k".into(),
                    concepts: vec![ConceptFullDto {
                        guid: g.clone(),
                        name: "c".into(),
                        description: None,
                        order: None,
                    }],
                    ..Default::default()
                });
                let mut rx = kit.read().unwrap().subscribe();
                let c = {
                    let kr = kit.read().unwrap();
                    kr.concepts[0].clone()
                };
                c.write().unwrap().set_name("c2".into());
                let evs = super::common::drain(&mut rx);
                assert!(evs
                    .iter()
                    .any(|e| matches!(e, KitEvent::FieldChanged { field: "name", .. })));
            }
        }

        mod connection {
            use crate::events::KitEvent;

            #[test]
            fn connection_set_gap_triggers_flatten_and_validation() {
                let (kit, _, dg, _, _, cg) = super::common::kit_with_connection();
                let mut rx = kit.read().unwrap().subscribe();
                let c = {
                    let kr = kit.read().unwrap();
                    let d = kr.design(dg.as_str()).unwrap();
                    let dr = d.read().unwrap();
                    dr.connection(cg.as_str()).unwrap().clone()
                };
                c.write().unwrap().set_gap(Some(1.0));
                let evs = super::common::drain(&mut rx);
                assert!(evs
                    .iter()
                    .any(|e| matches!(e, KitEvent::FieldChanged { field: "gap", .. })));
                assert!(evs
                    .iter()
                    .any(|e| matches!(e, KitEvent::FlattenInvalidated { .. })));
                assert!(evs
                    .iter()
                    .any(|e| matches!(e, KitEvent::ValidationInvalidated)));
            }
        }

        mod diff_apply {
            use crate::diff::DesignDiff;
            use crate::events::{EntityKind, EntityRef, KitEvent};
            use crate::guid::Guid;
            use crate::piece::PieceFullDto;
            use crate::typ::TypeIdDto;

            #[test]
            fn apply_design_diff_add_piece_emits_child_added_and_hashes() {
                let (kit, tg, dg, _) = super::common::kit_with_piece();
                let new_piece = Guid::new_v7();
                let diff = DesignDiff {
                    added_pieces: vec![PieceFullDto {
                        guid: new_piece.clone(),
                        r#type: Some(TypeIdDto { guid: tg.clone() }),
                        ..Default::default()
                    }],
                    ..Default::default()
                };
                let mut rx = kit.read().unwrap().subscribe();
                kit.write()
                    .unwrap()
                    .apply_design_diff(dg.as_str(), &diff)
                    .unwrap();
                let evs = super::common::drain(&mut rx);
                let child = EntityRef::new(EntityKind::Piece, new_piece);
                assert!(evs.iter().any(|e| matches!(
                    e,
                    KitEvent::ChildAdded { child: c, .. } if *c == child
                )));
                assert!(
                    evs.iter()
                        .filter(|e| matches!(e, KitEvent::HashInvalidated { .. }))
                        .count()
                        >= 1
                );
            }
        }

        mod connector {
            use crate::events::KitEvent;

            #[test]
            fn connector_set_code_emits() {
                let (kit, tg, cg) = super::common::kit_with_type_connector();
                let mut rx = kit.read().unwrap().subscribe();
                let c = {
                    let kr = kit.read().unwrap();
                    let t = kr.semio_type(tg.as_str()).unwrap();
                    let tr = t.read().unwrap();
                    tr.connector(cg.as_str()).unwrap().clone()
                };
                c.write().unwrap().set_code("C2".into());
                let evs = super::common::drain(&mut rx);
                assert!(evs
                    .iter()
                    .any(|e| matches!(e, KitEvent::FieldChanged { field: "code", .. })));
            }
        }

        mod design {
            use crate::events::{EntityKind, EntityRef};
            use crate::geom::{Camera, Coord, Location};

            macro_rules! design_meta_test {
                ($fn:ident, $field:literal, $op:expr) => {
                    #[test]
                    fn $fn() {
                        let (kit, _, dg, pg) = super::common::kit_with_piece();
                        let dre = EntityRef::new(EntityKind::Design, dg.clone());
                        let kre = super::common::kit_entity_ref(&kit);
                        let mut rx = kit.read().unwrap().subscribe();
                        let d = {
                            let kr = kit.read().unwrap();
                            kr.design(dg.as_str()).expect("design").clone()
                        };
                        let mut dw = d.write().unwrap();
                        $op(&mut *dw).unwrap();
                        let evs = super::common::drain(&mut rx);
                        super::common::assert_design_scalar_metadata_events(
                            &evs, dre, kre, &pg, $field,
                        );
                    }
                };
            }

            design_meta_test!(design_set_name, "name", |d: &mut crate::DesignStore| {
                d.set_name("x".into())
            });
            design_meta_test!(
                design_set_description,
                "description",
                |d: &mut crate::DesignStore| {
                    d.set_description(Some("d".into()))
                }
            );
            design_meta_test!(design_set_icon, "icon", |d: &mut crate::DesignStore| {
                d.set_icon(Some("i".into()))
            });
            design_meta_test!(design_set_image, "image", |d: &mut crate::DesignStore| {
                d.set_image(Some("m".into()))
            });
            design_meta_test!(
                design_set_variant,
                "variant",
                |d: &mut crate::DesignStore| {
                    d.set_variant(Some("v".into()))
                }
            );
            design_meta_test!(design_set_view, "view", |d: &mut crate::DesignStore| {
                d.set_view(Some("vw".into()))
            });
            design_meta_test!(
                design_set_location,
                "location",
                |d: &mut crate::DesignStore| {
                    d.set_location(Some(Location::new(1.0, 2.0)))
                }
            );
            design_meta_test!(design_set_camera, "camera", |d: &mut crate::DesignStore| {
                let mut cam = Camera::default();
                cam.position = Coord::new(0.0, 0.0, 1.0);
                d.set_camera(Some(cam))
            });
            design_meta_test!(design_set_unit, "unit", |d: &mut crate::DesignStore| {
                d.set_unit(Some("mm".into()))
            });
            design_meta_test!(
                design_set_created,
                "created",
                |d: &mut crate::DesignStore| {
                    d.set_created(Some("c".into()))
                }
            );
            design_meta_test!(
                design_set_updated,
                "updated",
                |d: &mut crate::DesignStore| {
                    d.set_updated(Some("u".into()))
                }
            );
        }

        mod file {
            use crate::events::KitEvent;

            #[test]
            fn file_set_mime_emits() {
                let (kit, fg) = super::common::kit_with_file();
                let mut rx = kit.read().unwrap().subscribe();
                let f = {
                    let kr = kit.read().unwrap();
                    kr.file(fg.as_str()).unwrap().clone()
                };
                f.write().unwrap().set_mime(Some("image/png".into()));
                let evs = super::common::drain(&mut rx);
                assert!(evs
                    .iter()
                    .any(|e| matches!(e, KitEvent::FieldChanged { field: "mime", .. })));
            }
        }

        mod folder {
            use crate::events::KitEvent;
            use crate::folder::FolderFullDto;
            use crate::guid::Guid;
            use crate::kit::{KitFullDto, KitStore};

            #[test]
            fn folder_set_description_emits() {
                let g = Guid::new_v7();
                let kit = KitStore::from_full_dto(KitFullDto {
                    guid: Guid::new_v7(),
                    name: "k".into(),
                    folders: vec![FolderFullDto {
                        guid: g.clone(),
                        path: "/p".into(),
                        description: None,
                    }],
                    ..Default::default()
                });
                let mut rx = kit.read().unwrap().subscribe();
                let f = {
                    let kr = kit.read().unwrap();
                    kr.folders[0].clone()
                };
                f.write().unwrap().set_description(Some("d".into()));
                let evs = super::common::drain(&mut rx);
                assert!(evs.iter().any(|e| matches!(
                    e,
                    KitEvent::FieldChanged {
                        field: "description",
                        ..
                    }
                )));
            }
        }

        mod group {
            use crate::events::KitEvent;

            #[test]
            fn group_set_color_emits() {
                let (kit, dg, gg) = super::common::kit_with_group();
                let mut rx = kit.read().unwrap().subscribe();
                let g = {
                    let kr = kit.read().unwrap();
                    let d = kr.design(dg.as_str()).unwrap();
                    let dr = d.read().unwrap();
                    dr.group(gg.as_str()).unwrap().clone()
                };
                g.write().unwrap().set_color(Some("#000".into()));
                let evs = super::common::drain(&mut rx);
                assert!(evs
                    .iter()
                    .any(|e| matches!(e, KitEvent::FieldChanged { field: "color", .. })));
            }
        }

        mod kit {
            macro_rules! kit_meta_test {
                ($fn:ident, $field:literal, $op:expr) => {
                    #[test]
                    fn $fn() {
                        let kit =
                            std::sync::Arc::new(std::sync::RwLock::new(crate::KitStore::new("i")));
                        let kref = super::common::kit_entity_ref(&kit);
                        let mut rx = kit.read().unwrap().subscribe();
                        {
                            let mut g = kit.write().unwrap();
                            $op(&mut *g).unwrap();
                        }
                        let evs = super::common::drain(&mut rx);
                        super::common::assert_kit_metadata_core(&evs, kref, $field);
                    }
                };
            }

            kit_meta_test!(kit_set_name, "name", |k: &mut crate::KitStore| {
                k.set_name("a".into())
            });
            kit_meta_test!(
                kit_set_description,
                "description",
                |k: &mut crate::KitStore| {
                    k.set_description(Some("d".into()))
                }
            );
            kit_meta_test!(kit_set_icon, "icon", |k: &mut crate::KitStore| {
                k.set_icon(Some("ic".into()))
            });
            kit_meta_test!(kit_set_image, "image", |k: &mut crate::KitStore| {
                k.set_image(Some("im".into()))
            });
            kit_meta_test!(kit_set_preview, "preview", |k: &mut crate::KitStore| {
                k.set_preview(Some("pr".into()))
            });
            kit_meta_test!(kit_set_version, "version", |k: &mut crate::KitStore| {
                k.set_version(Some("1".into()))
            });
            kit_meta_test!(kit_set_remote, "remote", |k: &mut crate::KitStore| {
                k.set_remote(Some("r".into()))
            });
            kit_meta_test!(kit_set_homepage, "homepage", |k: &mut crate::KitStore| {
                k.set_homepage(Some("https://example.com".into()))
            });
            kit_meta_test!(kit_set_license, "license", |k: &mut crate::KitStore| {
                k.set_license(Some("l".into()))
            });
            kit_meta_test!(kit_set_uri, "uri", |k: &mut crate::KitStore| {
                k.set_uri(Some("u".into()))
            });
            kit_meta_test!(kit_set_created, "created", |k: &mut crate::KitStore| {
                k.set_created(Some("c".into()))
            });
            kit_meta_test!(kit_set_updated, "updated", |k: &mut crate::KitStore| {
                k.set_updated(Some("u2".into()))
            });

            #[test]
            fn kit_set_name_idempotent_no_events() {
                let kit = std::sync::Arc::new(std::sync::RwLock::new(crate::KitStore::new("same")));
                let mut rx = kit.read().unwrap().subscribe();
                kit.write().unwrap().set_name("same".into()).unwrap();
                assert!(super::common::drain(&mut rx).is_empty());
            }
        }

        mod layer {
            use crate::events::KitEvent;

            #[test]
            fn layer_set_order_emits() {
                let (kit, dg, lg) = super::common::kit_with_layer();
                let mut rx = kit.read().unwrap().subscribe();
                let l = {
                    let kr = kit.read().unwrap();
                    let d = kr.design(dg.as_str()).unwrap();
                    let dr = d.read().unwrap();
                    dr.layer(lg.as_str()).unwrap().clone()
                };
                l.write().unwrap().set_order(Some(2));
                let evs = super::common::drain(&mut rx);
                assert!(evs
                    .iter()
                    .any(|e| matches!(e, KitEvent::FieldChanged { field: "order", .. })));
            }
        }

        mod piece {
            use crate::events::{EntityKind, EntityRef};
            use crate::geom::{Coord, Plane};

            macro_rules! piece_geom_test {
                ($fn:ident, $field:literal, $op:expr) => {
                    #[test]
                    fn $fn() {
                        let (kit, _, dg, pg) = super::common::kit_with_piece();
                        let pre = EntityRef::new(EntityKind::Piece, pg.clone());
                        let dre = EntityRef::new(EntityKind::Design, dg.clone());
                        let kre = super::common::kit_entity_ref(&kit);
                        let mut rx = kit.read().unwrap().subscribe();
                        let p = {
                            let kr = kit.read().unwrap();
                            let d = kr.design(dg.as_str()).unwrap();
                            let dr = d.read().unwrap();
                            dr.piece(pg.as_str()).unwrap().clone()
                        };
                        let mut pw = p.write().unwrap();
                        $op(&mut *pw).unwrap();
                        let evs = super::common::drain(&mut rx);
                        super::common::assert_piece_geometry_change(
                            &evs, pre, dre, kre, &pg, $field,
                        );
                    }
                };
            }

            piece_geom_test!(piece_set_plane, "plane", |p: &mut crate::PieceStore| {
                p.set_plane(Some(Plane::world_xy()))
            });
            piece_geom_test!(piece_set_center, "center", |p: &mut crate::PieceStore| {
                p.set_center(Some(Coord::new(1.0, 2.0, 3.0)))
            });
            piece_geom_test!(
                piece_set_mirror_plane,
                "mirrorPlane",
                |p: &mut crate::PieceStore| {
                    p.set_mirror_plane(Some(Plane::world_xy()))
                }
            );
            piece_geom_test!(piece_set_scale, "scale", |p: &mut crate::PieceStore| {
                p.set_scale(Some(2.0))
            });
            piece_geom_test!(piece_set_hidden, "hidden", |p: &mut crate::PieceStore| {
                p.set_hidden(Some(true))
            });
            piece_geom_test!(piece_set_locked, "locked", |p: &mut crate::PieceStore| {
                p.set_locked(Some(true))
            });
            piece_geom_test!(piece_set_id, "id", |p: &mut crate::PieceStore| {
                p.set_id(Some("id1".into()))
            });
            piece_geom_test!(piece_set_name, "name", |p: &mut crate::PieceStore| {
                p.set_name(Some("p".into()))
            });
            piece_geom_test!(
                piece_set_description,
                "description",
                |p: &mut crate::PieceStore| {
                    p.set_description(Some("pd".into()))
                }
            );

            #[test]
            fn piece_set_color_hash_only() {
                let (kit, _, dg, pg) = super::common::kit_with_piece();
                let pre = EntityRef::new(EntityKind::Piece, pg.clone());
                let dre = EntityRef::new(EntityKind::Design, dg.clone());
                let kre = super::common::kit_entity_ref(&kit);
                let mut rx = kit.read().unwrap().subscribe();
                let p = {
                    let kr = kit.read().unwrap();
                    let d = kr.design(dg.as_str()).unwrap();
                    let dr = d.read().unwrap();
                    dr.piece(pg.as_str()).unwrap().clone()
                };
                let mut pw = p.write().unwrap();
                pw.set_color(Some("#fff".into()));
                let evs = super::common::drain(&mut rx);
                super::common::assert_piece_scalar_hash_only(&evs, pre, dre, kre, "color");
            }

            #[test]
            fn piece_set_type_weak_geometry() {
                let (kit, tg, dg, pg) = super::common::kit_with_piece();
                let pre = EntityRef::new(EntityKind::Piece, pg.clone());
                let dre = EntityRef::new(EntityKind::Design, dg.clone());
                let kre = super::common::kit_entity_ref(&kit);
                let mut rx = kit.read().unwrap().subscribe();
                let tw = kit
                    .read()
                    .unwrap()
                    .semio_type(tg.as_str())
                    .map(|t| std::sync::Arc::downgrade(&t))
                    .unwrap();
                let p = {
                    let kr = kit.read().unwrap();
                    let d = kr.design(dg.as_str()).unwrap();
                    let dr = d.read().unwrap();
                    dr.piece(pg.as_str()).unwrap().clone()
                };
                let mut pw = p.write().unwrap();
                pw.set_type_weak(Some(tw));
                let evs = super::common::drain(&mut rx);
                super::common::assert_piece_geometry_change(&evs, pre, dre, kre, &pg, "type");
            }
        }

        mod port {
            use crate::events::KitEvent;

            #[test]
            fn port_set_family_emits_field_changed() {
                let (kit, type_guid, port_g) = super::common::kit_with_port();
                let mut rx = kit.read().unwrap().subscribe();
                let p = {
                    let kr = kit.read().unwrap();
                    let t = kr.semio_type(type_guid.as_str()).unwrap();
                    let tr = t.read().unwrap();
                    tr.port(port_g.as_str()).unwrap().clone()
                };
                p.write().unwrap().set_family(Some("f".into()));
                let evs = super::common::drain(&mut rx);
                assert!(evs.iter().any(|e| matches!(
                    e,
                    KitEvent::FieldChanged {
                        field: "family",
                        ..
                    }
                )));
            }
        }

        mod prop {
            use crate::events::KitEvent;
            use crate::guid::Guid;
            use crate::kit::{KitFullDto, KitStore};
            use crate::prop::PropFullDto;

            #[test]
            fn prop_set_unit_emits() {
                let g = Guid::new_v7();
                let kit = KitStore::from_full_dto(KitFullDto {
                    guid: Guid::new_v7(),
                    name: "k".into(),
                    props: vec![PropFullDto {
                        guid: g.clone(),
                        key: "k".into(),
                        value: "v".into(),
                        unit: None,
                    }],
                    ..Default::default()
                });
                let mut rx = kit.read().unwrap().subscribe();
                let p = {
                    let kr = kit.read().unwrap();
                    kr.props[0].clone()
                };
                p.write().unwrap().set_unit(Some("u".into()));
                let evs = super::common::drain(&mut rx);
                assert!(evs
                    .iter()
                    .any(|e| matches!(e, KitEvent::FieldChanged { field: "unit", .. })));
            }
        }

        mod quality {
            use crate::events::KitEvent;
            use crate::guid::Guid;
            use crate::kit::{KitFullDto, KitStore};
            use crate::quality::QualityFullDto;

            #[test]
            fn quality_set_key_emits() {
                let g = Guid::new_v7();
                let kit = KitStore::from_full_dto(KitFullDto {
                    guid: Guid::new_v7(),
                    name: "k".into(),
                    qualities: vec![QualityFullDto {
                        guid: g.clone(),
                        key: "k1".into(),
                        ..Default::default()
                    }],
                    ..Default::default()
                });
                let mut rx = kit.read().unwrap().subscribe();
                let q = {
                    let kr = kit.read().unwrap();
                    kr.qualities[0].clone()
                };
                q.write().unwrap().set_key("k2".into());
                let evs = super::common::drain(&mut rx);
                assert!(evs
                    .iter()
                    .any(|e| matches!(e, KitEvent::FieldChanged { field: "key", .. })));
            }
        }

        mod representation {
            use crate::events::KitEvent;
            use crate::file::{FileFullDto, FileIdDto};
            use crate::guid::Guid;
            use crate::kit::{KitFullDto, KitStore};
            use crate::representation::RepresentationFullDto;
            use crate::typ::TypeFullDto;

            #[test]
            fn representation_set_url_emits() {
                let fg = Guid::new_v7();
                let rg = Guid::new_v7();
                let tg = Guid::new_v7();
                let kit = KitStore::from_full_dto(KitFullDto {
                    guid: Guid::new_v7(),
                    name: "k".into(),
                    files: vec![FileFullDto {
                        guid: fg.clone(),
                        url: "https://f".into(),
                        ..Default::default()
                    }],
                    types: vec![TypeFullDto {
                        guid: tg.clone(),
                        name: "t".into(),
                        representations: vec![RepresentationFullDto {
                            guid: rg.clone(),
                            url: "https://r".into(),
                            file: Some(FileIdDto { guid: fg.clone() }),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }],
                    ..Default::default()
                });
                let mut rx = kit.read().unwrap().subscribe();
                let r = {
                    let kr = kit.read().unwrap();
                    let t = kr.types[0].clone();
                    let tr = t.read().unwrap();
                    tr.representation(rg.as_str()).unwrap().clone()
                };
                r.write().unwrap().set_url("https://r2".into());
                let evs = super::common::drain(&mut rx);
                assert!(evs
                    .iter()
                    .any(|e| matches!(e, KitEvent::FieldChanged { field: "url", .. })));
            }
        }

        mod side {
            use crate::events::KitEvent;

            #[test]
            fn side_set_design_piece_emits() {
                let (kit, _, dg, pb, _, cg) = super::common::kit_with_connection();
                let mut rx = kit.read().unwrap().subscribe();
                let piece_ref = {
                    let kr = kit.read().unwrap();
                    let d = kr.design(dg.as_str()).unwrap();
                    let dr = d.read().unwrap();
                    dr.piece(pb.as_str()).unwrap().clone()
                };
                let weak = std::sync::Arc::downgrade(&piece_ref);
                let connecting = {
                    let kr = kit.read().unwrap();
                    let d = kr.design(dg.as_str()).unwrap();
                    let dr = d.read().unwrap();
                    let c = dr.connection(cg.as_str()).unwrap();
                    let connecting = c.read().unwrap().connecting.clone();
                    connecting
                };
                connecting
                    .write()
                    .unwrap()
                    .set_design_piece_weak(Some(weak));
                let evs = super::common::drain(&mut rx);
                assert!(evs.iter().any(|e| matches!(
                    e,
                    KitEvent::FieldChanged {
                        field: "designPiece",
                        ..
                    }
                )));
            }
        }

        mod stat {
            use crate::design::DesignFullDto;
            use crate::events::KitEvent;
            use crate::guid::Guid;
            use crate::kit::{KitFullDto, KitStore};
            use crate::piece::PieceFullDto;
            use crate::stat::StatFullDto;
            use crate::typ::{TypeFullDto, TypeIdDto};

            #[test]
            fn stat_set_description_emits() {
                let type_guid = Guid::new_v7();
                let design_guid = Guid::new_v7();
                let piece_guid = Guid::new_v7();
                let stat_guid = Guid::new_v7();
                let kit = KitStore::from_full_dto(KitFullDto {
                    guid: Guid::new_v7(),
                    name: "k".into(),
                    types: vec![TypeFullDto {
                        guid: type_guid.clone(),
                        name: "typ".into(),
                        ..Default::default()
                    }],
                    designs: vec![DesignFullDto {
                        guid: design_guid.clone(),
                        name: "des".into(),
                        pieces: vec![PieceFullDto {
                            guid: piece_guid.clone(),
                            r#type: Some(TypeIdDto {
                                guid: type_guid.clone(),
                            }),
                            ..Default::default()
                        }],
                        stats: vec![StatFullDto {
                            guid: stat_guid.clone(),
                            key: "sk".into(),
                            value: "sv".into(),
                            description: None,
                            unit: None,
                        }],
                        ..Default::default()
                    }],
                    ..Default::default()
                });
                let mut rx = kit.read().unwrap().subscribe();
                let s = {
                    let kr = kit.read().unwrap();
                    let d = kr.designs[0].clone();
                    let dr = d.read().unwrap();
                    dr.stats[0].clone()
                };
                s.write().unwrap().set_description(Some("d".into()));
                let evs = super::common::drain(&mut rx);
                assert!(evs.iter().any(|e| matches!(
                    e,
                    KitEvent::FieldChanged {
                        field: "description",
                        ..
                    }
                )));
            }
        }

        mod tag {
            use crate::events::KitEvent;
            use crate::guid::Guid;
            use crate::kit::{KitFullDto, KitStore};
            use crate::tag::TagFullDto;

            #[test]
            fn tag_set_order_emits() {
                let g = Guid::new_v7();
                let kit = KitStore::from_full_dto(KitFullDto {
                    guid: Guid::new_v7(),
                    name: "k".into(),
                    tags: vec![TagFullDto {
                        guid: g.clone(),
                        name: "t".into(),
                        order: None,
                    }],
                    ..Default::default()
                });
                let mut rx = kit.read().unwrap().subscribe();
                let t = {
                    let kr = kit.read().unwrap();
                    kr.tags[0].clone()
                };
                t.write().unwrap().set_order(Some(1));
                let evs = super::common::drain(&mut rx);
                assert!(evs
                    .iter()
                    .any(|e| matches!(e, KitEvent::FieldChanged { field: "order", .. })));
            }
        }

        mod type_ {
            use crate::events::{EntityKind, EntityRef};

            macro_rules! type_meta_test {
                ($fn:ident, $field:literal, $op:expr) => {
                    #[test]
                    fn $fn() {
                        let (kit, tg) = super::common::kit_with_type_only();
                        let tre = EntityRef::new(EntityKind::Type, tg.clone());
                        let kre = super::common::kit_entity_ref(&kit);
                        let mut rx = kit.read().unwrap().subscribe();
                        let t = {
                            let kr = kit.read().unwrap();
                            kr.semio_type(tg.as_str()).unwrap().clone()
                        };
                        let mut tw = t.write().unwrap();
                        $op(&mut *tw).unwrap();
                        let evs = super::common::drain(&mut rx);
                        super::common::assert_type_metadata_core(&evs, tre, kre, $field);
                    }
                };
            }

            type_meta_test!(type_set_name, "name", |t: &mut crate::TypeStore| {
                t.set_name("tn".into())
            });
            type_meta_test!(
                type_set_description,
                "description",
                |t: &mut crate::TypeStore| {
                    t.set_description(Some("td".into()))
                }
            );
            type_meta_test!(type_set_icon, "icon", |t: &mut crate::TypeStore| {
                t.set_icon(Some("i".into()))
            });
            type_meta_test!(type_set_image, "image", |t: &mut crate::TypeStore| {
                t.set_image(Some("m".into()))
            });
            type_meta_test!(type_set_variant, "variant", |t: &mut crate::TypeStore| {
                t.set_variant(Some("v".into()))
            });
            type_meta_test!(type_set_stock, "stock", |t: &mut crate::TypeStore| {
                t.set_stock(Some(3))
            });
            type_meta_test!(type_set_virtual, "virtual", |t: &mut crate::TypeStore| {
                t.set_virtual(Some(true))
            });
            type_meta_test!(type_set_unit, "unit", |t: &mut crate::TypeStore| {
                t.set_unit(Some("u".into()))
            });
            type_meta_test!(type_set_location, "location", |t: &mut crate::TypeStore| {
                t.set_location(Some(crate::geom::Location::new(1.0, 2.0)))
            });
            type_meta_test!(type_set_created, "created", |t: &mut crate::TypeStore| {
                t.set_created(Some("c".into()))
            });
            type_meta_test!(type_set_updated, "updated", |t: &mut crate::TypeStore| {
                t.set_updated(Some("u".into()))
            });
        }
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_handle_tests {
    use wasm_bindgen_test::*;

    use crate::kit::KitStore;
    use crate::wasm::KitStoreHandle;

    #[wasm_bindgen_test]
    fn kit_store_handle_create_roundtrips_snapshot() {
        let kit = KitStore::new("wasm-kit-test");
        let dto = kit.read().unwrap().to_full_dto();
        let h = KitStoreHandle::create(serde_wasm_bindgen::to_value(&dto).unwrap()).unwrap();
        let snap = h.snapshot().expect("snapshot");
        let parsed: serde_json::Value = serde_wasm_bindgen::from_value(snap).unwrap();
        assert_eq!(parsed["name"], "wasm-kit-test");
    }
}

pub use attribute::{
    AttributeFullDto, AttributeIdDto, AttributeMetadataDto, AttributeShallowDto, AttributeStore,
    AttributeStoreRef, AttributeStoreWeak,
};
pub use author::{
    AuthorFullDto, AuthorIdDto, AuthorMetadataDto, AuthorShallowDto, AuthorStore, AuthorStoreRef,
    AuthorStoreWeak,
};
pub use benchmark::{
    BenchmarkFullDto, BenchmarkIdDto, BenchmarkMetadataDto, BenchmarkShallowDto, BenchmarkStore,
    BenchmarkStoreRef, BenchmarkStoreWeak,
};
pub use concept::{
    ConceptFullDto, ConceptIdDto, ConceptMetadataDto, ConceptShallowDto, ConceptStore,
    ConceptStoreRef, ConceptStoreWeak,
};
pub use connection::{
    ConnectionFullDto, ConnectionIdDto, ConnectionMetadataDto, ConnectionShallowDto,
    ConnectionStore, ConnectionStoreRef, ConnectionStoreWeak,
};
pub use connector::{
    ConnectorFullDto, ConnectorIdDto, ConnectorMetadataDto, ConnectorShallowDto, ConnectorStore,
    ConnectorStoreRef, ConnectorStoreWeak,
};
pub use design::{
    DesignFullDto, DesignIdDto, DesignMetadataDto, DesignShallowDto, DesignStore, DesignStoreRef,
    DesignStoreWeak,
};
pub use diff::{DesignChange, DesignDiff};
pub use error::{Result, SemioError, SetError, SetResult};
pub use events::{EntityKind, EntityRef, EventBus, KitEvent};
pub use file::{
    FileFullDto, FileIdDto, FileMetadataDto, FileShallowDto, FileStore, FileStoreRef, FileStoreWeak,
};
pub use folder::{
    FolderFullDto, FolderIdDto, FolderMetadataDto, FolderShallowDto, FolderStore, FolderStoreRef,
    FolderStoreWeak,
};
pub use geom::{Camera, Coord, Location, Plane, Vector};
pub use group::{
    GroupFullDto, GroupIdDto, GroupMetadataDto, GroupShallowDto, GroupStore, GroupStoreRef,
    GroupStoreWeak,
};
pub use guid::Guid;
pub use hash::{Cache, HashWriter};
pub use kit::{
    KitFullDto, KitIdDto, KitMetadataDto, KitShallowDto, KitStore, KitStoreRef, KitStoreWeak,
};
pub use layer::{
    LayerFullDto, LayerIdDto, LayerMetadataDto, LayerShallowDto, LayerStore, LayerStoreRef,
    LayerStoreWeak,
};
pub use piece::{
    PieceFullDto, PieceIdDto, PieceMetadataDto, PieceShallowDto, PieceStore, PieceStoreRef,
    PieceStoreWeak,
};
pub use port::{
    PortFullDto, PortIdDto, PortMetadataDto, PortShallowDto, PortStore, PortStoreRef, PortStoreWeak,
};
pub use prop::{
    PropFullDto, PropIdDto, PropMetadataDto, PropShallowDto, PropStore, PropStoreRef, PropStoreWeak,
};
pub use quality::{
    QualityFullDto, QualityIdDto, QualityMetadataDto, QualityShallowDto, QualityStore,
    QualityStoreRef, QualityStoreWeak,
};
pub use report::{NoteSeverity, OperationNote, SemioReport, ValidationResult};
pub use representation::{
    RepresentationFullDto, RepresentationIdDto, RepresentationMetadataDto,
    RepresentationShallowDto, RepresentationStore, RepresentationStoreRef, RepresentationStoreWeak,
};
pub use session::KitGraphSession;
pub use side::{
    SideFullDto, SideIdDto, SideMetadataDto, SideShallowDto, SideStore, SideStoreRef, SideStoreWeak,
};
pub use stat::{
    StatFullDto, StatIdDto, StatMetadataDto, StatShallowDto, StatStore, StatStoreRef, StatStoreWeak,
};
pub use tag::{
    TagFullDto, TagIdDto, TagMetadataDto, TagShallowDto, TagStore, TagStoreRef, TagStoreWeak,
};
pub use typ::{
    TypeFullDto, TypeIdDto, TypeMetadataDto, TypeShallowDto, TypeStore, TypeStoreRef, TypeStoreWeak,
};
