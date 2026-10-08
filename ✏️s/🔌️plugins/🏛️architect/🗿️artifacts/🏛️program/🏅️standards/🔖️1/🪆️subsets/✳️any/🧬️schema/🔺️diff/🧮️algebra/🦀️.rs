//! 🧮️ Identified-collection delta algebra of the program diff — one generic implementation of apply, absorb, inverse and
//! between for every `added / removed / patched / reordered` delta, so each of the program's register deltas shares one
//! sound set of laws instead of 66 hand-copied variants.
//!
//! A delta applies in the fixed order remove → add → patch → reorder. `absorb` coalesces same-id entries
//! (patch∘patch → one patch, create∘delete → nothing, delete∘create → replace, create∘patch → create, patch∘delete →
//! delete); `inverse` rebuilds the exact negative delta from the base rows, restoring row positions.

use crate::kernel::EntityId;
use crate::registers::RowPatch;
use protocol::{Identified, MutationApplyError, MutationApplyResult, Patchable};
use std::collections::{HashMap, HashSet};

/// 🩹 One `{ id, patch }` entry of a delta's `patched` list.
pub trait PatchEntry<P>: Sized {
    fn new(id: String, patch: P) -> Self;
    fn id(&self) -> &str;
    fn patch(&self) -> &P;
    fn patch_mut(&mut self) -> &mut P;
    fn into_parts(self) -> (String, P);
}

/// 📦️ The four lists of a delta, owned.
pub type DeltaParts<D> = (Vec<<D as CollectionDelta>::Row>, Vec<String>, Vec<<D as CollectionDelta>::Entry>, Option<Vec<String>>);

/// 🧩 An identified-collection delta: rows `added`, ids `removed`, rows `patched`, and a complete `reordered` permutation.
pub trait CollectionDelta: Clone + Default + PartialEq + Sized {
    type Row: Clone + PartialEq + Identified<EntityId> + Patchable<Self::Patch>;
    type Patch: RowPatch<Self::Row> + Clone;
    type Entry: PatchEntry<Self::Patch>;
    fn from_parts(added: Vec<Self::Row>, removed: Vec<String>, patched: Vec<Self::Entry>, reordered: Option<Vec<String>>) -> Self;
    fn into_parts(self) -> DeltaParts<Self>;
    fn parts(&self) -> (&[Self::Row], &[String], &[Self::Entry], Option<&[String]>);
}

fn row_id<R: Identified<EntityId>>(row: &R) -> &str {
    row.id().0.as_str()
}

fn reject(code: &'static str, message: &'static str, section: &'static str, index: usize) -> MutationApplyError {
    MutationApplyError::new(code, message).at([section.to_string(), index.to_string()])
}

/// 🕳️ Whether the delta changes nothing.
pub fn is_empty<D: CollectionDelta>(delta: &D) -> bool {
    let (added, removed, patched, reordered) = delta.parts();
    added.is_empty() && removed.is_empty() && patched.is_empty() && reordered.is_none()
}

/// 🎯️ Applies `delta` to `items` in place: remove, add, patch, reorder. On `Err` `items` is partially written and must be discarded.
pub fn apply<D: CollectionDelta>(delta: &D, items: &mut Vec<D::Row>) -> MutationApplyResult<()> {
    let (added, removed, patched, reordered) = delta.parts();
    {
        let existing: HashSet<&str> = items.iter().map(row_id).collect();
        let mut seen: HashSet<&str> = HashSet::with_capacity(removed.len());
        for (index, id) in removed.iter().enumerate() {
            if !existing.contains(id.as_str()) {
                return Err(reject("mutation.apply.missing-target", "removed entity does not exist", "removed", index));
            }
            if !seen.insert(id.as_str()) {
                return Err(reject("mutation.apply.duplicate-target", "entity is removed more than once", "removed", index));
            }
        }
    }
    if !removed.is_empty() {
        let gone: HashSet<&str> = removed.iter().map(String::as_str).collect();
        items.retain(|row| !gone.contains(row_id(row)));
    }
    {
        let mut present: HashSet<&str> = items.iter().map(row_id).collect();
        for (index, row) in added.iter().enumerate() {
            if !present.insert(row_id(row)) {
                return Err(reject("mutation.apply.duplicate-target", "added entity identity already exists", "added", index));
            }
        }
    }
    items.extend(added.iter().cloned());
    if !patched.is_empty() {
        let mut targets = Vec::with_capacity(patched.len());
        {
            let position: HashMap<&str, usize> = items.iter().enumerate().map(|(at, row)| (row_id(row), at)).collect();
            let mut seen: HashSet<&str> = HashSet::with_capacity(patched.len());
            for (index, entry) in patched.iter().enumerate() {
                let Some(&at) = position.get(entry.id()) else {
                    return Err(reject("mutation.apply.missing-target", "patched entity does not exist", "patched", index));
                };
                if !seen.insert(entry.id()) {
                    return Err(reject("mutation.apply.duplicate-target", "entity is patched more than once", "patched", index));
                }
                targets.push(at);
            }
        }
        for (entry, at) in patched.iter().zip(targets) {
            items[at].apply_patch(entry.patch());
        }
    }
    if let Some(order) = reordered {
        let invalid = || MutationApplyError::new("mutation.apply.invalid-order", "entity reorder must be a complete unique permutation").at(["reordered"]);
        if order.len() != items.len() {
            return Err(invalid());
        }
        let index: HashMap<String, usize> = items.iter().enumerate().map(|(at, row)| (row_id(row).to_owned(), at)).collect();
        let mut rows: Vec<Option<D::Row>> = items.drain(..).map(Some).collect();
        for id in order {
            let row = index.get(id).and_then(|&at| rows[at].take()).ok_or_else(invalid)?;
            items.push(row);
        }
    }
    Ok(())
}

/// ➕️ Composes `first` (base → mid) with `later` (mid → after) into base → after, coalescing same-id entries.
pub fn absorb<D: CollectionDelta>(first: D, later: D) -> D {
    let (mut added, mut removed, mut patched, first_order) = first.into_parts();
    let (later_added, later_removed, later_patched, later_order) = later.into_parts();
    for id in &later_removed {
        if let Some(at) = added.iter().position(|row| row_id(row) == id.as_str()) {
            added.remove(at);
        } else {
            patched.retain(|entry| entry.id() != id.as_str());
            if !removed.contains(id) {
                removed.push(id.clone());
            }
        }
    }
    let later_added_ids: Vec<String> = later_added.iter().map(|row| row_id(row).to_owned()).collect();
    added.extend(later_added);
    for entry in later_patched {
        let (id, patch) = entry.into_parts();
        if let Some(row) = added.iter_mut().find(|row| row_id(row) == id) {
            row.apply_patch(&patch);
        } else if let Some(existing) = patched.iter_mut().find(|existing| existing.id() == id) {
            existing.patch_mut().merge(patch);
        } else {
            patched.push(D::Entry::new(id, patch));
        }
    }
    let reordered = match later_order {
        Some(order) => Some(order),
        None => first_order.map(|mut order| {
            order.retain(|id| !later_removed.contains(id));
            for id in later_added_ids {
                if !order.contains(&id) {
                    order.push(id);
                }
            }
            order
        }),
    };
    D::from_parts(added, removed, patched, reordered)
}

/// 🔁️ The negative delta of `delta` over `base`: applied to `delta`'s result it restores `base` exactly, row positions included.
pub fn inverse<D: CollectionDelta>(delta: &D, base: &[D::Row]) -> D {
    let (added, removed, patched, reordered) = delta.parts();
    let by_id: HashMap<&str, &D::Row> = base.iter().map(|row| (row_id(row), row)).collect();
    let mut undo_removed: Vec<String> = added.iter().map(|row| row_id(row).to_owned()).collect();
    let mut undo_added: Vec<D::Row> = removed.iter().filter_map(|id| by_id.get(id.as_str()).map(|row| (*row).clone())).collect();
    let mut undo_patched: Vec<D::Entry> = Vec::new();
    for entry in patched {
        if added.iter().any(|row| row_id(row) == entry.id()) || removed.iter().any(|id| id.as_str() == entry.id()) {
            continue;
        }
        let Some(row) = by_id.get(entry.id()) else { continue };
        match entry.patch().restore(row) {
            Some(restored) => undo_patched.push(D::Entry::new(entry.id().to_owned(), restored)),
            None => {
                undo_removed.push(entry.id().to_owned());
                undo_added.push((*row).clone());
            }
        }
    }
    let base_order: Vec<String> = base.iter().map(|row| row_id(row).to_owned()).collect();
    let mut after: Vec<String> = base_order.iter().filter(|id| !removed.contains(*id)).cloned().collect();
    after.extend(added.iter().map(|row| row_id(row).to_owned()));
    if let Some(order) = reordered {
        after = order.to_vec();
    }
    after.retain(|id| !undo_removed.contains(id));
    after.extend(undo_added.iter().map(|row| row_id(row).to_owned()));
    let undo_order = (after != base_order).then_some(base_order);
    D::from_parts(undo_added, undo_removed, undo_patched, undo_order)
}

/// 🧭️ The delta that turns `base` into `other`; a row that no patch can express is replaced wholesale.
pub fn between<D: CollectionDelta>(base: &[D::Row], other: &[D::Row]) -> D {
    let other_by_id: HashMap<&str, &D::Row> = other.iter().map(|row| (row_id(row), row)).collect();
    let base_ids: HashSet<&str> = base.iter().map(row_id).collect();
    let mut removed: Vec<String> = Vec::new();
    let mut added: Vec<D::Row> = Vec::new();
    let mut patched: Vec<D::Entry> = Vec::new();
    for row in base {
        let id = row_id(row);
        match other_by_id.get(id) {
            None => removed.push(id.to_owned()),
            Some(next) if *next == row => {}
            Some(next) => match row.diff_patch(next) {
                Some(patch) if !patch.is_empty() && exact::<D>(row, &patch, next) => patched.push(D::Entry::new(id.to_owned(), patch)),
                _ => {
                    removed.push(id.to_owned());
                    added.push((*next).clone());
                }
            },
        }
    }
    added.extend(other.iter().filter(|row| !base_ids.contains(row_id(row))).cloned());
    let mut after: Vec<&str> = base.iter().map(row_id).filter(|id| !removed.iter().any(|gone| gone.as_str() == *id)).collect();
    after.extend(added.iter().map(row_id));
    let other_order: Vec<&str> = other.iter().map(row_id).collect();
    let reordered = (after != other_order).then(|| other_order.iter().map(|id| (*id).to_owned()).collect());
    D::from_parts(added, removed, patched, reordered)
}

fn exact<D: CollectionDelta>(row: &D::Row, patch: &D::Patch, target: &D::Row) -> bool {
    let mut candidate = row.clone();
    candidate.apply_patch(patch);
    candidate == *target
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
