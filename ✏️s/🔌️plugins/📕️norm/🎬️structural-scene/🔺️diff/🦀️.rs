//! 🔺️ Scene diff: the minimal `SceneDelta` between two scenes and its inverse application; `apply(diff(a, b), a) == b` and `diff(a, a)` is empty.
//!
//! `Add { index, entity }` is an upsert at its final position: an entity already present is moved there, which is how reorders stay one patch per id.

use crate::{Annotation, AnnotationPatch, EntityPatch, SceneDelta, SceneEntity, StructuralScene};
use std::collections::{BTreeMap, BTreeSet};

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod unit;

/// 🚫️ Why a delta does not apply to a scene.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiffError {
    BaseMismatch,
    UnknownId(String),
    IndexOutOfRange(String),
}

trait Keyed: Clone + PartialEq {
    fn key(&self) -> &str;
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

impl Keyed for SceneEntity {
    fn key(&self) -> &str {
        &self.entity.id
    }

    fn same(&self, other: &Self) -> bool {
        self.content_hash == other.content_hash && self == other
    }
}

impl Keyed for Annotation {
    fn key(&self) -> &str {
        self.id()
    }
}

/// 📋️ Patches `(final index, item, upsert)` plus removed ids for one ordered collection; items outside the longest order-preserving run become upserts at their final index.
fn changes<T: Keyed>(a: &[T], b: &[T]) -> (Vec<(usize, T, bool)>, Vec<String>) {
    let before: BTreeMap<&str, (usize, &T)> = a.iter().enumerate().map(|(i, x)| (x.key(), (i, x))).collect();
    let after: BTreeSet<&str> = b.iter().map(Keyed::key).collect();
    let kept: Vec<&str> = a.iter().map(Keyed::key).filter(|k| after.contains(k)).collect();
    let order: BTreeMap<&str, usize> = kept.iter().enumerate().map(|(i, k)| (*k, i)).collect();
    let survivors: Vec<&str> = b.iter().map(Keyed::key).filter(|k| order.contains_key(k)).collect();
    let positions: Vec<usize> = survivors.iter().map(|k| order[k]).collect();
    let mut tails: Vec<usize> = Vec::new();
    let mut link: Vec<Option<usize>> = vec![None; positions.len()];
    let mut at: Vec<usize> = Vec::new();
    for (i, &p) in positions.iter().enumerate() {
        let slot = tails.partition_point(|&t| t < p);
        if slot == tails.len() {
            tails.push(p);
            at.push(i);
        } else {
            tails[slot] = p;
            at[slot] = i;
        }
        link[i] = if slot > 0 { Some(at[slot - 1]) } else { None };
    }
    let mut stay: BTreeSet<&str> = BTreeSet::new();
    let mut cursor = at.last().copied();
    while let Some(i) = cursor {
        stay.insert(survivors[i]);
        cursor = link[i];
    }
    let mut patches = Vec::new();
    for (index, item) in b.iter().enumerate() {
        match before.get(item.key()) {
            None => patches.push((index, item.clone(), true)),
            Some(_) if !stay.contains(item.key()) => patches.push((index, item.clone(), true)),
            Some((_, old)) if !old.same(item) => patches.push((index, item.clone(), false)),
            Some(_) => {}
        }
    }
    let removed = a.iter().map(Keyed::key).filter(|k| !after.contains(k)).map(str::to_owned).collect();
    (patches, removed)
}

/// 🔺️ Minimal delta from `a` to `b`: unchanged entities and annotations are absent; reordered ones become upserts.
pub fn diff(a: &StructuralScene, b: &StructuralScene) -> SceneDelta {
    let (adds, removed) = changes(&a.entities, &b.entities);
    let (annotation_adds, annotation_removed) = changes(&a.annotations, &b.annotations);
    let mut entities: Vec<EntityPatch> = adds.into_iter().map(|(index, entity, upsert)| {
        if upsert { EntityPatch::Add { index: index as u32, entity } } else { EntityPatch::Replace { entity } }
    }).collect();
    entities.extend(removed.into_iter().map(|id| EntityPatch::Remove { id }));
    let mut annotations: Vec<AnnotationPatch> = annotation_adds.into_iter().map(|(index, annotation, upsert)| {
        if upsert { AnnotationPatch::Add { index: index as u32, annotation } } else { AnnotationPatch::Replace { annotation } }
    }).collect();
    annotations.extend(annotation_removed.into_iter().map(|id| AnnotationPatch::Remove { id }));
    SceneDelta {
        base: a.revision.clone(),
        revision: b.revision.clone(),
        bounds: (a.bounds != b.bounds).then(|| b.bounds.clone()),
        entities,
        annotations,
        legends: (a.legends != b.legends).then(|| b.legends.clone()),
        layers: (a.layers != b.layers).then(|| b.layers.clone()),
        tracks: (a.tracks != b.tracks).then(|| b.tracks.clone()),
        owners: (a.owners != b.owners).then(|| b.owners.clone()),
        description: (a.description != b.description).then(|| b.description.clone()),
    }
}

fn rebuild<T: Keyed>(items: &[T], upserts: Vec<(usize, T)>, replaces: Vec<T>, removes: &BTreeSet<String>) -> Result<Vec<T>, DiffError> {
    let moved: BTreeSet<&str> = upserts.iter().map(|(_, x)| x.key()).collect();
    let known: BTreeSet<&str> = items.iter().map(Keyed::key).collect();
    if let Some(id) = removes.iter().find(|id| !known.contains(id.as_str())) {
        return Err(DiffError::UnknownId(id.clone()));
    }
    let mut out: Vec<T> = items.iter().filter(|x| !removes.contains(x.key()) && !moved.contains(x.key())).cloned().collect();
    for r in replaces {
        let slot = out.iter().position(|x| x.key() == r.key()).ok_or_else(|| DiffError::UnknownId(r.key().to_owned()))?;
        out[slot] = r;
    }
    let mut sorted = upserts;
    sorted.sort_by_key(|(i, _)| *i);
    for (index, item) in sorted {
        if index > out.len() {
            return Err(DiffError::IndexOutOfRange(item.key().to_owned()));
        }
        out.insert(index, item);
    }
    Ok(out)
}

/// 🩹️ Applies `delta` to `a`; the delta must be based on `a`'s revision.
pub fn apply(delta: &SceneDelta, a: &StructuralScene) -> Result<StructuralScene, DiffError> {
    if delta.base != a.revision {
        return Err(DiffError::BaseMismatch);
    }
    let (mut ups, mut reps, mut rems) = (Vec::new(), Vec::new(), BTreeSet::new());
    for patch in &delta.entities {
        match patch {
            EntityPatch::Add { index, entity } => ups.push((*index as usize, entity.clone())),
            EntityPatch::Replace { entity } => reps.push(entity.clone()),
            EntityPatch::Remove { id } => {
                rems.insert(id.clone());
            }
        }
    }
    let (mut aups, mut areps, mut arems) = (Vec::new(), Vec::new(), BTreeSet::new());
    for patch in &delta.annotations {
        match patch {
            AnnotationPatch::Add { index, annotation } => aups.push((*index as usize, annotation.clone())),
            AnnotationPatch::Replace { annotation } => areps.push(annotation.clone()),
            AnnotationPatch::Remove { id } => {
                arems.insert(id.clone());
            }
        }
    }
    Ok(StructuralScene {
        schema: a.schema.clone(),
        revision: delta.revision.clone(),
        bounds: delta.bounds.clone().unwrap_or_else(|| a.bounds.clone()),
        entities: rebuild(&a.entities, ups, reps, &rems)?,
        annotations: rebuild(&a.annotations, aups, areps, &arems)?,
        legends: delta.legends.clone().unwrap_or_else(|| a.legends.clone()),
        layers: delta.layers.clone().unwrap_or_else(|| a.layers.clone()),
        tracks: delta.tracks.clone().unwrap_or_else(|| a.tracks.clone()),
        owners: delta.owners.clone().unwrap_or_else(|| a.owners.clone()),
        description: delta.description.clone().unwrap_or_else(|| a.description.clone()),
    })
}
