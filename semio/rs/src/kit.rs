use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

use crate::attribute::Attribute;
use crate::author::Author;
use crate::concept::Concept;
use crate::design::{Design, DesignDto, DesignRef, FlattenedDesign};
use crate::error::{Result, SemioError};
use crate::file::{File, FileDto, FileRef};
use crate::folder::{Folder, FolderDto, FolderRef};
use crate::guid::Guid;
use crate::hash::HashWriter;
use crate::prop::Prop;
use crate::quality::{Quality, QualityDto, QualityRef};
use crate::report::{SemioReport, ValidationResult};
use crate::representation::Representation;
use crate::tag::Tag;
use crate::typ::{Type, TypeDto, TypeRef};

pub type KitRef = Arc<RwLock<Kit>>;

/// Root aggregate: a kit owns all components of the system.
#[derive(Debug)]
pub struct Kit {
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
    pub types: Vec<TypeRef>,
    pub designs: Vec<DesignRef>,
    pub files: Vec<FileRef>,
    pub folders: Vec<FolderRef>,
    pub authors: Vec<Author>,
    pub concepts: Vec<Concept>,
    pub tags: Vec<Tag>,
    pub qualities: Vec<QualityRef>,
    pub props: Vec<Prop>,
    pub attributes: Vec<Attribute>,
    hash_cache: OnceLock<String>,
}

impl Kit {
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
            if let Ok(t) = t.read() { t.hash_into(w); }
        }
        for d in &self.designs {
            if let Ok(d) = d.read() { d.hash_into(w); }
        }
        for f in &self.files {
            if let Ok(f) = f.read() { f.hash_into(w); }
        }
        for f in &self.folders {
            if let Ok(f) = f.read() { f.hash_into(w); }
        }
        for a in &self.authors { a.hash_into(w); }
        for c in &self.concepts { c.hash_into(w); }
        for t in &self.tags { t.hash_into(w); }
        for q in &self.qualities {
            if let Ok(q) = q.read() { q.hash_into(w); }
        }
        for p in &self.props { p.hash_into(w); }
        for a in &self.attributes { a.hash_into(w); }
    }

    /// Find a type by guid.
    pub fn semio_type(&self, guid: &str) -> Option<TypeRef> {
        self.types
            .iter()
            .find(|t| t.read().map(|t| t.guid.as_str() == guid).unwrap_or(false))
            .cloned()
    }

    /// Find a type mutably by guid (returns the Arc, caller locks).
    pub fn semio_type_mut(&self, guid: &str) -> Option<TypeRef> {
        self.semio_type(guid)
    }

    pub fn design(&self, guid: &str) -> Option<DesignRef> {
        self.designs
            .iter()
            .find(|d| d.read().map(|d| d.guid.as_str() == guid).unwrap_or(false))
            .cloned()
    }

    pub fn design_mut(&self, guid: &str) -> Option<DesignRef> { self.design(guid) }

    pub fn file(&self, guid: &str) -> Option<FileRef> {
        self.files
            .iter()
            .find(|f| f.read().map(|f| f.guid.as_str() == guid).unwrap_or(false))
            .cloned()
    }

    pub fn folder(&self, guid: &str) -> Option<FolderRef> {
        self.folders
            .iter()
            .find(|f| f.read().map(|f| f.guid.as_str() == guid).unwrap_or(false))
            .cloned()
    }

    pub fn quality(&self, guid: &str) -> Option<QualityRef> {
        self.qualities
            .iter()
            .find(|q| q.read().map(|q| q.guid.as_str() == guid).unwrap_or(false))
            .cloned()
    }

    /// Flatten a design by guid, returning a report with notes if any.
    pub fn flatten_design(&self, design_guid: &str) -> Result<SemioReport<FlattenedDesign>> {
        let d = self
            .design(design_guid)
            .ok_or_else(|| SemioError::NotFound { kind: "Design", guid: Guid::from(design_guid) })?;
        let flat = d
            .read()
            .map_err(|_| SemioError::LockPoisoned("design"))?
            .flatten();
        Ok(SemioReport::ok(flat))
    }

    /// Validate the kit structure; currently a light check.
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

    /// Deep hash equality (ignores guid order within the same content).
    pub fn are_equal(&self, other: &Kit) -> bool {
        self.hash() == other.hash()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct KitDto {
    #[serde(default)]
    pub guid: Option<Guid>,
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
    pub types: Vec<TypeDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub designs: Vec<DesignDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<FileDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub folders: Vec<FolderDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authors: Vec<Author>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub concepts: Vec<Concept>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualities: Vec<QualityDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<Prop>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<Attribute>,
}

impl From<&Kit> for KitDto {
    fn from(k: &Kit) -> Self {
        KitDto {
            guid: Some(k.guid.clone()),
            name: k.name.clone(),
            description: k.description.clone(),
            icon: k.icon.clone(),
            image: k.image.clone(),
            preview: k.preview.clone(),
            version: k.version.clone(),
            remote: k.remote.clone(),
            homepage: k.homepage.clone(),
            license: k.license.clone(),
            uri: k.uri.clone(),
            created: k.created.clone(),
            updated: k.updated.clone(),
            types: k
                .types
                .iter()
                .filter_map(|t| t.read().ok().map(|t| TypeDto::from(&*t)))
                .collect(),
            designs: k
                .designs
                .iter()
                .filter_map(|d| d.read().ok().map(|d| DesignDto::from(&*d)))
                .collect(),
            files: k
                .files
                .iter()
                .filter_map(|f| f.read().ok().map(|f| FileDto::from(&*f)))
                .collect(),
            folders: k
                .folders
                .iter()
                .filter_map(|f| f.read().ok().map(|f| FolderDto::from(&*f)))
                .collect(),
            authors: k.authors.clone(),
            concepts: k.concepts.clone(),
            tags: k.tags.clone(),
            qualities: k
                .qualities
                .iter()
                .filter_map(|q| q.read().ok().map(|q| QualityDto::from(&*q)))
                .collect(),
            props: k.props.clone(),
            attributes: k.attributes.clone(),
        }
    }
}

impl Kit {
    /// Hydrate a kit and all its descendants from a DTO, wiring parent/child
    /// back-references throughout.
    pub fn from_dto(d: KitDto) -> KitRef {
        let kit = Arc::new(RwLock::new(Kit {
            guid: d.guid.unwrap_or_else(Guid::new_v7),
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
            authors: d.authors,
            concepts: d.concepts,
            tags: d.tags,
            qualities: d
                .qualities
                .into_iter()
                .map(|q| Arc::new(RwLock::new(Quality::from(q))))
                .collect(),
            props: d.props,
            attributes: d.attributes,
            hash_cache: OnceLock::new(),
        }));

        let file_refs: Vec<FileRef> = d
            .files
            .into_iter()
            .map(|f| Arc::new(RwLock::new(File::from(f))))
            .collect();
        let folder_refs: Vec<FolderRef> = d
            .folders
            .into_iter()
            .map(|f| Arc::new(RwLock::new(Folder::from(f))))
            .collect();

        let mut type_refs: Vec<TypeRef> = Vec::with_capacity(d.types.len());
        let mut type_index: HashMap<Guid, TypeRef> = HashMap::new();
        for tdto in d.types {
            let rep_file_map: Vec<(Option<Guid>, Guid)> = tdto
                .representations
                .iter()
                .map(|r| (r.file_guid.clone(), r.guid.clone().unwrap_or_else(Guid::new_v7)))
                .collect();
            let t = Type::from_dto(tdto);
            if let Ok(mut tw) = t.write() {
                tw.parent_kit = Arc::downgrade(&kit);
                for r in &tw.representations {
                    if let Ok(mut r_mut) = r.write() {
                        if let Some((Some(fg), _)) =
                            rep_file_map.iter().find(|(_, g)| *g == r_mut.guid)
                        {
                            if let Some(fref) = file_refs.iter().find(|f| {
                                f.read().map(|f| f.guid == *fg).unwrap_or(false)
                            }) {
                                r_mut.file = Some(Arc::downgrade(fref));
                            }
                        }
                    }
                }
            }
            if let Ok(tr) = t.read() {
                type_index.insert(tr.guid.clone(), t.clone());
            }
            type_refs.push(t);
        }

        let design_refs: Vec<DesignRef> = d
            .designs
            .into_iter()
            .map(|ddto| {
                let design = Design::from_dto(ddto, &type_index);
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

    /// Convert this kit to its wire DTO.
    pub fn to_dto(&self) -> KitDto { KitDto::from(self) }

    // Suppress warning about unused import while keeping the symbol available
    // for hash computation shadows through doc-tests.
    #[allow(dead_code)]
    fn _keep_representation_import() -> Option<Representation> { None }
}
