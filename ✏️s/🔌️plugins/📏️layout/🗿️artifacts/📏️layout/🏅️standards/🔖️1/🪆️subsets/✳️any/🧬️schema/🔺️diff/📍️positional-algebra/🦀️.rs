//! 📍️ Positional algebra of the layout diff: the positional list delta trait, its apply, absorb and inverse operations and the payload row helpers. See [`super`].

use super::*;

//#region 🔖️PositionalAlgebra
/// 📍️ The shared shape of every positional list delta: the final length when it changes plus the rows whose value differs from the base.
pub trait Positional: Default + Clone {
    type Item: Clone + PartialEq;
    fn len(&self) -> Option<usize>;
    fn rows(&self) -> Vec<(usize, &Self::Item)>;
    fn from_parts(len: Option<usize>, rows: Vec<(usize, Self::Item)>) -> Self;
}

macro_rules! impl_positional {
    ($delta:ty, $item:ty, $row:ident, $field:ident) => {
        impl Positional for $delta {
            type Item = $item;
            fn len(&self) -> Option<usize> {
                self.len
            }
            fn rows(&self) -> Vec<(usize, &$item)> {
                self.rows.iter().map(|row| (row.index, &row.$field)).collect()
            }
            fn from_parts(len: Option<usize>, rows: Vec<(usize, $item)>) -> Self {
                Self { len, rows: rows.into_iter().map(|(index, $field)| $row { index, $field }).collect() }
            }
        }
    };
}

impl_positional!(TextStyleRunsDelta, TextStyleRun, TextStyleRunRow, run);

impl_positional!(PageGuidesDelta, LayoutRect, PageGuideRow, guide);

pub fn apply_positional<D: Positional>(items: &[D::Item], delta: &D) -> MutationApplyResult<Vec<D::Item>> {
    let target = delta.len().unwrap_or(items.len());
    let rows = delta.rows();
    if let Some((index, _)) = rows.iter().find(|(index, _)| *index >= target) {
        return Err(MutationApplyError::new("mutation.apply.invalid-order", "positional row lies beyond the final length").at(["rows".to_string(), index.to_string()]));
    }
    let mut next = Vec::with_capacity(target);
    for index in 0..target {
        match rows.iter().rev().find(|(row, _)| *row == index).map(|(_, value)| (*value).clone()).or_else(|| items.get(index).cloned()) {
            Some(value) => next.push(value),
            None => return Err(MutationApplyError::new("mutation.apply.missing-target", "extended position carries no row").at(["rows".to_string(), index.to_string()])),
        }
    }
    Ok(next)
}

pub fn absorb_positional<D: Positional>(first: D, second: D) -> D {
    let len = second.len().or(first.len());
    let mut rows: std::collections::BTreeMap<usize, D::Item> = first.rows().into_iter().map(|(index, value)| (index, value.clone())).collect();
    rows.extend(second.rows().into_iter().map(|(index, value)| (index, value.clone())));
    D::from_parts(len, rows.into_iter().filter(|(index, _)| len.is_none_or(|len| *index < len)).collect())
}

pub fn inverse_positional<D: Positional>(delta: &D, base: &[D::Item]) -> D {
    let target = delta.len().unwrap_or(base.len());
    let mut indices: std::collections::BTreeSet<usize> = delta.rows().into_iter().map(|(index, _)| index).filter(|index| *index < base.len()).collect();
    indices.extend(target..base.len());
    D::from_parts((target != base.len()).then_some(base.len()), indices.into_iter().map(|index| (index, base[index].clone())).collect())
}

/// 📍️ Whether a positional delta changes nothing.
pub(super) fn is_empty_positional<D: Positional>(delta: &D) -> bool {
    delta.len().is_none() && delta.rows().is_empty()
}

/// 📍️ The style runs of a story after `delta` — the fallible half of [`Patchable`] for [`TextStoryPatch`].
pub fn apply_story_runs(runs: &[TextStyleRun], delta: &TextStyleRunsDelta) -> MutationApplyResult<Vec<TextStyleRun>> {
    apply_positional(runs, delta)
}
//#endregion 🔖️PositionalAlgebra

//#region 🔖️PayloadRows
/// 🎯️ The keyed rows that turn the held list into a whole-list payload: absent rows leave at their base index, new and changed rows enter at their payload index, and the unchanged rows outside the longest already-ordered run are relocated.
fn payload_parts<R: protocol::list_delta::Keyed<Key = String> + Clone + PartialEq>(held: &[R], payload: &[R]) -> protocol::list_delta::Parts<R, protocol::list_delta::NoPatch> {
    use protocol::list_delta::Keyed;
    let unchanged = |row: &R| payload.iter().any(|next| next.key() == row.key() && next == row);
    let removed: Vec<(String, usize)> = held.iter().enumerate().filter(|(_, row)| !unchanged(row)).map(|(at, row)| (row.key(), at)).collect();
    let inserted: Vec<(usize, R)> = payload.iter().enumerate().filter(|(_, next)| !held.iter().any(|row| row.key() == next.key() && row == *next)).map(|(at, next)| (at, next.clone())).collect();
    let survivors: Vec<(usize, usize)> = held.iter().enumerate().filter(|(_, row)| unchanged(row)).filter_map(|(from, row)| payload.iter().position(|next| next.key() == row.key()).map(|to| (from, to))).collect();
    let mut tails: Vec<usize> = Vec::new();
    let mut link: Vec<Option<usize>> = vec![None; survivors.len()];
    for (at, (_, to)) in survivors.iter().enumerate() {
        let slot = tails.partition_point(|tail| survivors[*tail].1 < *to);
        link[at] = slot.checked_sub(1).map(|before| tails[before]);
        if slot == tails.len() {
            tails.push(at);
        } else {
            tails[slot] = at;
        }
    }
    let mut kept = vec![false; survivors.len()];
    let mut cursor = tails.last().copied();
    while let Some(at) = cursor {
        kept[at] = true;
        cursor = link[at];
    }
    let moved: Vec<(String, usize, usize)> = survivors.iter().zip(&kept).filter(|(_, kept)| !**kept).map(|((from, to), _)| (held[*from].key(), *from, *to)).collect();
    protocol::list_delta::Parts { removed, inserted, moved, modified: Vec::new() }
}

/// 🎯️ The override rows that turn a page's held overrides into the payload overrides.
pub fn page_override_rows(held: &[PageOverride], payload: &[PageOverride]) -> PageOverridesDelta {
    PageOverridesDelta::from_row_parts(payload_parts(held, payload))
}

/// 🎯️ The dictionary rows that turn the held entries into the payload entries.
pub fn data_entry_rows(held: &[semio_framework_value_dictionary::QuestionDictionaryEntry], payload: &[semio_framework_value_dictionary::QuestionDictionaryEntry]) -> LayoutDataEntriesDelta {
    let wrap = |rows: &[semio_framework_value_dictionary::QuestionDictionaryEntry]| rows.iter().cloned().map(LayoutDataEntryRow).collect::<Vec<_>>();
    LayoutDataEntriesDelta::from_parts(payload_parts(&wrap(held), &wrap(payload)))
}
//#endregion 🔖️PayloadRows
