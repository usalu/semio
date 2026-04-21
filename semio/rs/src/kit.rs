use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

use crate::attribute::{AttributeFullDto, AttributeShallowDto, AttributeStore};
use crate::author::{AuthorFullDto, AuthorShallowDto, AuthorStore};
use crate::concept::{ConceptFullDto, ConceptShallowDto, ConceptStore};
use crate::design::{DesignFullDto, DesignStore, DesignStoreRef};
use crate::error::{Result, SemioError};
use crate::file::{FileFullDto, FileStore, FileStoreRef};
use crate::folder::{FolderFullDto, FolderStore, FolderStoreRef};
use crate::guid::Guid;
use crate::hash::HashWriter;
use crate::prop::{PropFullDto, PropShallowDto, PropStore};
use crate::quality::{QualityFullDto, QualityShallowDto, QualityStore, QualityStoreRef};
use crate::report::{SemioReport, ValidationResult};
use crate::tag::{TagFullDto, TagShallowDto, TagStore};
use crate::typ::{TypeFullDto, TypeStore, TypeStoreRef};

pub type KitStoreRef = Arc<RwLock<KitStore>>;

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
    pub authors: Vec<AuthorStore>,
    pub concepts: Vec<ConceptStore>,
    pub tags: Vec<TagStore>,
    pub qualities: Vec<QualityStoreRef>,
    pub props: Vec<PropStore>,
    pub attributes: Vec<AttributeStore>,
    hash_cache: OnceLock<String>,
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
            a.hash_into(w);
        }
        for c in &self.concepts {
            c.hash_into(w);
        }
        for t in &self.tags {
            t.hash_into(w);
        }
        for q in &self.qualities {
            if let Ok(q) = q.read() {
                q.hash_into(w);
            }
        }
        for p in &self.props {
            p.hash_into(w);
        }
        for a in &self.attributes {
            a.hash_into(w);
        }
    }

    pub fn semio_type(&self, guid: &str) -> Option<TypeStoreRef> {
        self.types
            .iter()
            .find(|t| t.read().map(|t| t.guid.as_str() == guid).unwrap_or(false))
            .cloned()
    }

    pub fn semio_type_mut(&self, guid: &str) -> Option<TypeStoreRef> {
        self.semio_type(guid)
    }

    pub fn design(&self, guid: &str) -> Option<DesignStoreRef> {
        self.designs
            .iter()
            .find(|d| d.read().map(|d| d.guid.as_str() == guid).unwrap_or(false))
            .cloned()
    }

    pub fn design_mut(&self, guid: &str) -> Option<DesignStoreRef> {
        self.design(guid)
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
    pub fn flatten_design(&self, design_guid: &str) -> Result<SemioReport<crate::diff::DesignChange>> {
        let d = self
            .design(design_guid)
            .ok_or_else(|| SemioError::NotFound { kind: "Design", guid: Guid::from(design_guid) })?;
        let report = match d.read() {
            Ok(dr) => dr.flatten_change(),
            Err(_) => return Err(SemioError::LockPoisoned("design")),
        };
        Ok(report)
    }

    pub fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult::valid();
        if self.name.trim().is_empty() {
            result.is_valid = false;
            result.errors.push("kit.name must not be empty".into());
        }
        for d in &self.designs {
            if let Ok(d) = d.read() {
                for p in &d.pieces {
                    if let Ok(p) = p.read() {
                        if p.type_ref.as_ref().and_then(|t| t.upgrade()).is_none() {
                            result.is_valid = false;
                            result
                                .errors
                                .push(format!("piece {} has no valid type reference", p.guid));
                        }
                    }
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
            hash_cache: OnceLock::new(),
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
            hash_cache: OnceLock::new(),
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
            authors: authors.into_iter().map(AuthorStore::from_full_dto).collect(),
            concepts: concepts.into_iter().map(ConceptStore::from_full_dto).collect(),
            tags: tags.into_iter().map(TagStore::from_full_dto).collect(),
            qualities: qualities
                .into_iter()
                .map(|q| Arc::new(RwLock::new(QualityStore::from_full_dto(q))))
                .collect(),
            props: props.into_iter().map(PropStore::from_full_dto).collect(),
            attributes: attributes.into_iter().map(AttributeStore::from_full_dto).collect(),
            hash_cache: OnceLock::new(),
        }));

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
            k.types = type_refs;
            k.designs = design_refs;
            k.files = file_refs;
            k.folders = folder_refs;
        }
        kit
    }

    pub fn to_id_dto(&self) -> KitIdDto {
        KitIdDto { guid: self.guid.clone() }
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
            authors: self.authors.iter().map(AuthorStore::to_shallow_dto).collect(),
            concepts: self.concepts.iter().map(ConceptStore::to_shallow_dto).collect(),
            tags: self.tags.iter().map(TagStore::to_shallow_dto).collect(),
            qualities: self
                .qualities
                .iter()
                .filter_map(|q| q.read().ok().map(|q| q.to_shallow_dto()))
                .collect(),
            props: self.props.iter().map(PropStore::to_shallow_dto).collect(),
            attributes: self.attributes.iter().map(AttributeStore::to_shallow_dto).collect(),
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
            authors: self.authors.iter().map(AuthorStore::to_full_dto).collect(),
            concepts: self.concepts.iter().map(ConceptStore::to_full_dto).collect(),
            tags: self.tags.iter().map(TagStore::to_full_dto).collect(),
            qualities: self
                .qualities
                .iter()
                .filter_map(|q| q.read().ok().map(|q| q.to_full_dto()))
                .collect(),
            props: self.props.iter().map(PropStore::to_full_dto).collect(),
            attributes: self.attributes.iter().map(AttributeStore::to_full_dto).collect(),
        }
    }
}
