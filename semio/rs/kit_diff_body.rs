// Kit-scoped sparse diff. Included by `pub mod kit_diff` in lib.rs.
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::attribute::{AttributeFullDto, AttributeIdDto};
use crate::author::{AuthorFullDto, AuthorIdDto};
use crate::concept::{ConceptFullDto, ConceptIdDto};
use crate::design::{DesignFullDto, DesignIdDto};
use crate::diff::{
    merge_opt, merge_opt_nested, AttributesDiff, AuthorsDiff, ConceptsDiff, DesignDiff, FilesDiff, FoldersDiff, PropsDiff, QualitiesDiff, TagsDiff, TypesDiff,
};
use crate::file::{FileFullDto, FileIdDto};
use crate::folder::{FolderFullDto, FolderIdDto};
use crate::id::Id;
use crate::kit::KitFullDto;
use crate::prop::{PropFullDto, PropIdDto};
use crate::quality::{QualityFullDto, QualityIdDto};
use crate::tag::{TagFullDto, TagIdDto};
use crate::typ::{TypeFullDto, TypeIdDto};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DesignDiffUpdate {
    #[serde(rename = "designId")]
    pub design_id: Id,
    #[serde(flatten)]
    pub diff: DesignDiff,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DesignsDiff {
    #[serde(default)]
    pub removed: Vec<DesignIdDto>,
    #[serde(default)]
    pub updated: Vec<DesignDiffUpdate>,
    #[serde(default)]
    pub added: Vec<DesignFullDto>,
}

impl DesignsDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.updated.is_empty() && self.added.is_empty()
    }
    pub fn merge(&self, b: &Self) -> Self {
        let mut removed: Vec<_> = self.removed.iter().chain(b.removed.iter()).cloned().collect();
        removed.sort_by(|x, y| x.id.cmp(&y.id));
        removed.dedup_by(|x, y| x.id == y.id);
        let mut m: HashMap<Id, DesignDiffUpdate> = HashMap::new();
        for u in &self.updated {
            m.insert(u.design_id.clone(), u.clone());
        }
        for u in &b.updated {
            let e = m.entry(u.design_id.clone()).or_insert_with(|| DesignDiffUpdate { design_id: u.design_id.clone(), diff: DesignDiff::default() });
            e.diff = e.diff.merge(&u.diff);
        }
        Self { removed, updated: m.into_values().collect(), added: [self.added.as_slice(), b.added.as_slice()].concat() }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct KitDiff {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub icon: Option<Option<String>>,
    pub image: Option<Option<String>>,
    pub preview: Option<Option<String>>,
    pub version: Option<Option<String>>,
    pub remote: Option<Option<String>>,
    pub homepage: Option<Option<String>>,
    pub license: Option<Option<String>>,
    pub uri: Option<Option<String>>,
    pub created: Option<Option<String>>,
    pub updated: Option<Option<String>>,
    #[serde(default)]
    pub types: Option<TypesDiff>,
    #[serde(default)]
    pub designs: Option<DesignsDiff>,
    #[serde(default)]
    pub files: Option<FilesDiff>,
    #[serde(default)]
    pub folders: Option<FoldersDiff>,
    #[serde(default)]
    pub authors: Option<AuthorsDiff>,
    #[serde(default)]
    pub concepts: Option<ConceptsDiff>,
    #[serde(default)]
    pub tags: Option<TagsDiff>,
    #[serde(default)]
    pub qualities: Option<QualitiesDiff>,
    #[serde(default)]
    pub props: Option<PropsDiff>,
    #[serde(default)]
    pub attributes: Option<AttributesDiff>,
}

impl KitDiff {
    /// Alias for [`Self::between_dto`].
    pub fn between(before: &KitFullDto, after: &KitFullDto) -> Self {
        Self::between_dto(before, after)
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.description.is_none()
            && self.icon.is_none()
            && self.image.is_none()
            && self.preview.is_none()
            && self.version.is_none()
            && self.remote.is_none()
            && self.homepage.is_none()
            && self.license.is_none()
            && self.uri.is_none()
            && self.created.is_none()
            && self.updated.is_none()
            && self.types.as_ref().map_or(true, |x| x.is_empty())
            && self.designs.as_ref().map_or(true, |x| x.is_empty())
            && self.files.as_ref().map_or(true, |x| x.is_empty())
            && self.folders.as_ref().map_or(true, |x| x.is_empty())
            && self.authors.as_ref().map_or(true, |x| x.is_empty())
            && self.concepts.as_ref().map_or(true, |x| x.is_empty())
            && self.tags.as_ref().map_or(true, |x| x.is_empty())
            && self.qualities.as_ref().map_or(true, |x| x.is_empty())
            && self.props.as_ref().map_or(true, |x| x.is_empty())
            && self.attributes.as_ref().map_or(true, |x| x.is_empty())
    }

    pub fn merge(&self, b: &Self) -> Self {
        Self {
            name: merge_opt(&self.name, &b.name),
            description: merge_opt_nested(&self.description, &b.description, |_, y| y.clone()),
            icon: merge_opt_nested(&self.icon, &b.icon, |_, y| y.clone()),
            image: merge_opt_nested(&self.image, &b.image, |_, y| y.clone()),
            preview: merge_opt_nested(&self.preview, &b.preview, |_, y| y.clone()),
            version: merge_opt_nested(&self.version, &b.version, |_, y| y.clone()),
            remote: merge_opt_nested(&self.remote, &b.remote, |_, y| y.clone()),
            homepage: merge_opt_nested(&self.homepage, &b.homepage, |_, y| y.clone()),
            license: merge_opt_nested(&self.license, &b.license, |_, y| y.clone()),
            uri: merge_opt_nested(&self.uri, &b.uri, |_, y| y.clone()),
            created: merge_opt_nested(&self.created, &b.created, |_, y| y.clone()),
            updated: merge_opt_nested(&self.updated, &b.updated, |_, y| y.clone()),
            types: merge_opt_nested(&self.types, &b.types, |x, y| x.merge(y)),
            designs: merge_opt_nested(&self.designs, &b.designs, |x, y| x.merge(y)),
            files: merge_opt_nested(&self.files, &b.files, |x, y| x.merge(y)),
            folders: merge_opt_nested(&self.folders, &b.folders, |x, y| x.merge(y)),
            authors: merge_opt_nested(&self.authors, &b.authors, |x, y| x.merge(y)),
            concepts: merge_opt_nested(&self.concepts, &b.concepts, |x, y| x.merge(y)),
            tags: merge_opt_nested(&self.tags, &b.tags, |x, y| x.merge(y)),
            qualities: merge_opt_nested(&self.qualities, &b.qualities, |x, y| x.merge(y)),
            props: merge_opt_nested(&self.props, &b.props, |x, y| x.merge(y)),
            attributes: merge_opt_nested(&self.attributes, &b.attributes, |x, y| x.merge(y)),
        }
    }

    /// Single-design patch lifted to kit scope (for command folding).
    pub fn for_design(design_id: Id, d: DesignDiff) -> Self {
        if d.is_empty() {
            return Self::default();
        }
        Self {
            designs: Some(DesignsDiff { removed: vec![], updated: vec![DesignDiffUpdate { design_id, diff: d }], added: vec![] }),
            ..Default::default()
        }
    }
}

fn diff_id_vec<T: PartialEq + Clone>(before: &[T], after: &[T], get_id: impl Fn(&T) -> &Id) -> (Vec<T>, Vec<Id>, Vec<T>) {
    let bm: HashMap<Id, &T> = before.iter().map(|t| (get_id(t).clone(), t)).collect();
    let am: HashMap<Id, &T> = after.iter().map(|t| (get_id(t).clone(), t)).collect();
    let kb: HashSet<Id> = bm.keys().cloned().collect();
    let ka: HashSet<Id> = am.keys().cloned().collect();
    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for id in ka.difference(&kb) {
        added.push((**am.get(id).expect("a")).clone());
    }
    for id in kb.difference(&ka) {
        removed.push(id.clone());
    }
    for id in ka.intersection(&kb) {
        let b = *bm.get(id).expect("b");
        let a = *am.get(id).expect("a");
        if b != a {
            modified.push(a.clone());
        }
    }
    (added, removed, modified)
}

impl KitDiff {
    /// Structural delta from `before` to `after` kit DTOs (for debugging / tooling).
    pub fn between_dto(before: &KitFullDto, after: &KitFullDto) -> Self {
        let mut d = Self::default();
        if before.name != after.name {
            d.name = Some(after.name.clone());
        }
        if before.description != after.description {
            d.description = Some(after.description.clone());
        }
        if before.icon != after.icon {
            d.icon = Some(after.icon.clone());
        }
        if before.image != after.image {
            d.image = Some(after.image.clone());
        }
        if before.preview != after.preview {
            d.preview = Some(after.preview.clone());
        }
        if before.version != after.version {
            d.version = Some(after.version.clone());
        }
        if before.remote != after.remote {
            d.remote = Some(after.remote.clone());
        }
        if before.homepage != after.homepage {
            d.homepage = Some(after.homepage.clone());
        }
        if before.license != after.license {
            d.license = Some(after.license.clone());
        }
        if before.uri != after.uri {
            d.uri = Some(after.uri.clone());
        }
        if before.created != after.created {
            d.created = Some(after.created.clone());
        }
        if before.updated != after.updated {
            d.updated = Some(after.updated.clone());
        }
        let (a_t, r_t, m_t) = diff_id_vec(&before.types, &after.types, |t| &t.id);
        if !a_t.is_empty() || !r_t.is_empty() || !m_t.is_empty() {
            d.types = Some(TypesDiff {
                removed: r_t.iter().map(|i| TypeIdDto { id: i.clone() }).collect(),
                updated: m_t
                    .iter()
                    .filter_map(|full| {
                        let b = before.types.iter().find(|t| t.id == full.id)?;
                        let diff = crate::diff::type_full_delta(b, full);
                        if diff.is_empty() {
                            None
                        } else {
                            Some(crate::diff::TypeDiffUpdate { id: TypeIdDto { id: full.id.clone() }, diff })
                        }
                    })
                    .collect(),
                added: a_t,
            });
            if d.types.as_ref().unwrap().is_empty() {
                d.types = None;
            }
        }
        let bm: HashMap<Id, &DesignFullDto> = before.designs.iter().map(|x| (x.id.clone(), x)).collect();
        let am: HashMap<Id, &DesignFullDto> = after.designs.iter().map(|x| (x.id.clone(), x)).collect();
        let kb: HashSet<Id> = bm.keys().cloned().collect();
        let ka: HashSet<Id> = am.keys().cloned().collect();
        let mut des = DesignsDiff::default();
        for id in kb.difference(&ka) {
            des.removed.push(DesignIdDto { id: id.clone() });
        }
        for id in ka.difference(&kb) {
            des.added.push((*am.get(id).expect("a")).clone());
        }
        for id in ka.intersection(&kb) {
            let b = *bm.get(id).expect("b");
            let a = *am.get(id).expect("a");
            if b != a {
                let dd = DesignDiff::between_dto(b, a);
                if !dd.is_empty() {
                    des.updated.push(DesignDiffUpdate { design_id: id.clone(), diff: dd });
                }
            }
        }
        if !des.is_empty() {
            d.designs = Some(des);
        }
        let (a_f, r_f, m_f) = diff_id_vec(&before.files, &after.files, |t| &t.id);
        if !a_f.is_empty() || !r_f.is_empty() || !m_f.is_empty() {
            d.files = Some(FilesDiff {
                removed: r_f.iter().map(|i| FileIdDto { id: i.clone() }).collect(),
                updated: m_f
                    .iter()
                    .filter_map(|full| {
                        let b = before.files.iter().find(|t| t.id == full.id)?;
                        let df = crate::diff::file_full_delta(b, full);
                        if df.is_empty() {
                            None
                        } else {
                            Some(crate::diff::FileDiffUpdate { id: FileIdDto { id: full.id.clone() }, diff: df })
                        }
                    })
                    .collect(),
                added: a_f,
            });
            if d.files.as_ref().unwrap().is_empty() {
                d.files = None;
            }
        }
        let (a_fo, r_fo, m_fo) = diff_id_vec(&before.folders, &after.folders, |t| &t.id);
        if !a_fo.is_empty() || !r_fo.is_empty() || !m_fo.is_empty() {
            d.folders = Some(FoldersDiff {
                removed: r_fo.iter().map(|i| FolderIdDto { id: i.clone() }).collect(),
                updated: m_fo
                    .iter()
                    .filter_map(|full| {
                        let b = before.folders.iter().find(|t| t.id == full.id)?;
                        let df = crate::diff::folder_full_delta(b, full);
                        if df.is_empty() {
                            None
                        } else {
                            Some(crate::diff::FolderDiffUpdate { id: FolderIdDto { id: full.id.clone() }, diff: df })
                        }
                    })
                    .collect(),
                added: a_fo,
            });
            if d.folders.as_ref().unwrap().is_empty() {
                d.folders = None;
            }
        }
        let merge_simple_authors = |bf: &[AuthorFullDto], af: &[AuthorFullDto]| -> Option<AuthorsDiff> {
            let (a, r, m) = diff_id_vec(bf, af, |t| &t.id);
            if a.is_empty() && r.is_empty() && m.is_empty() {
                return None;
            }
            Some(AuthorsDiff {
                removed: r.iter().map(|i| AuthorIdDto { id: i.clone() }).collect(),
                updated: m
                    .iter()
                    .filter_map(|full| {
                        let b = bf.iter().find(|t| t.id == full.id)?;
                        let df = crate::diff::author_full_delta(b, full);
                        if df.is_empty() {
                            None
                        } else {
                            Some(crate::diff::AuthorDiffUpdate { id: AuthorIdDto { id: full.id.clone() }, diff: df })
                        }
                    })
                    .collect(),
                added: a,
            })
        };
        d.authors = merge_simple_authors(&before.authors, &after.authors);
        d.concepts = {
            let (a, r, m) = diff_id_vec(&before.concepts, &after.concepts, |t| &t.id);
            if a.is_empty() && r.is_empty() && m.is_empty() {
                None
            } else {
                Some(ConceptsDiff {
                    removed: r.iter().map(|i| ConceptIdDto { id: i.clone() }).collect(),
                    updated: m
                        .iter()
                        .filter_map(|full| {
                            let b = before.concepts.iter().find(|t| t.id == full.id)?;
                            let df = crate::diff::concept_full_delta(b, full);
                            if df.is_empty() {
                                None
                            } else {
                                Some(crate::diff::ConceptDiffUpdate { id: ConceptIdDto { id: full.id.clone() }, diff: df })
                            }
                        })
                        .collect(),
                    added: a,
                })
            }
        };
        d.tags = {
            let (a, r, m) = diff_id_vec(&before.tags, &after.tags, |t| &t.id);
            if a.is_empty() && r.is_empty() && m.is_empty() {
                None
            } else {
                Some(TagsDiff {
                    removed: r.iter().map(|i| TagIdDto { id: i.clone() }).collect(),
                    updated: m
                        .iter()
                        .filter_map(|full| {
                            let b = before.tags.iter().find(|t| t.id == full.id)?;
                            let df = crate::diff::tag_full_delta(b, full);
                            if df.is_empty() {
                                None
                            } else {
                                Some(crate::diff::TagDiffUpdate { id: TagIdDto { id: full.id.clone() }, diff: df })
                            }
                        })
                        .collect(),
                    added: a,
                })
            }
        };
        d.qualities = {
            let (a, r, m) = diff_id_vec(&before.qualities, &after.qualities, |t| &t.id);
            if a.is_empty() && r.is_empty() && m.is_empty() {
                None
            } else {
                Some(QualitiesDiff {
                    removed: r.iter().map(|i| QualityIdDto { id: i.clone() }).collect(),
                    updated: m
                        .iter()
                        .filter_map(|full| {
                            let b = before.qualities.iter().find(|t| t.id == full.id)?;
                            let df = crate::diff::quality_full_delta(b, full);
                            if df.is_empty() {
                                None
                            } else {
                                Some(crate::diff::QualityDiffUpdate { id: QualityIdDto { id: full.id.clone() }, diff: df })
                            }
                        })
                        .collect(),
                    added: a,
                })
            }
        };
        let (a_p, r_p, m_p) = diff_id_vec(&before.props, &after.props, |t| &t.id);
        if !a_p.is_empty() || !r_p.is_empty() || !m_p.is_empty() {
            let pd = PropsDiff {
                removed: r_p.iter().map(|i| PropIdDto { id: i.clone() }).collect(),
                updated: m_p
                    .iter()
                    .filter_map(|full| {
                        let b = before.props.iter().find(|t| t.id == full.id)?;
                        let df = crate::diff::prop_full_delta(b, full);
                        if df.is_empty() {
                            None
                        } else {
                            Some(crate::diff::PropDiffUpdate { id: PropIdDto { id: full.id.clone() }, diff: df })
                        }
                    })
                    .collect(),
                added: a_p,
            };
            if !pd.is_empty() {
                d.props = Some(pd);
            }
        }
        let (a_at, r_at, m_at) = diff_id_vec(&before.attributes, &after.attributes, |t| &t.id);
        if !a_at.is_empty() || !r_at.is_empty() || !m_at.is_empty() {
            let ad = AttributesDiff {
                removed: r_at.iter().map(|i| AttributeIdDto { id: i.clone() }).collect(),
                updated: m_at
                    .iter()
                    .filter_map(|full| {
                        let b = before.attributes.iter().find(|t| t.id == full.id)?;
                        let df = crate::diff::attribute_full_delta(b, full);
                        if df.is_empty() {
                            None
                        } else {
                            Some(crate::diff::AttributeDiffUpdate { id: AttributeIdDto { id: full.id.clone() }, diff: df })
                        }
                    })
                    .collect(),
                added: a_at,
            };
            if !ad.is_empty() {
                d.attributes = Some(ad);
            }
        }
        d
    }
}
