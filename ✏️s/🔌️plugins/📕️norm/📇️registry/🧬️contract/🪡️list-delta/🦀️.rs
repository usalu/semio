//! 🪡️ Keyed list deltas — the sparse, typed, order-aware difference a norm artifact's mutation leaves raise for a list of
//! identified rows (`walls`, `members`, `actions`, …). Rows are named by their stable key, never by a position that later
//! edits shift: a delta lists the keys it `removed`, the rows it `added` with the key of the row each is inserted after,
//! and the field patches it `modified` per key. The three vocabularies compose without a base, which is what makes
//! [`MutationDiff::absorb`](protocol::MutationDiff::absorb) total and base-free: patch∘patch is one patch,
//! create∘modify is a changed create, create∘delete is nothing, delete∘create is a replacement.
//!
//! 🧭️ Semantics of one delta over a base list, in this order: (1) every `added` row is inserted right after its anchor
//! (`None` is the list head; an anchor names the latest added row of that key, else the base row — a base row about to be
//! removed still anchors); (2) every `removed` key leaves the base row of that key; (3) every `modified` key patches the
//! one row that carries it. Removal acts on base rows only, so a key may be removed and added again (a replacement).
//!
//! 🧬️ [`norm_list_delta!`] defines the concrete wire type of one list and [`norm_row_patch!`] the concrete sparse patch of one
//! row type; the generic algebra lives once in [`Parts`].
//!
//! @see ../🧪️tests/🔬️unit/🦀️.rs — the randomized sequence laws

use protocol::MutationApplyError;

/// 🔑️ A row with a stable identity inside its list.
pub trait Keyed {
    /// 🏷️ The row's key, unique within its list.
    fn key(&self) -> &str;
}

/// 🩹 The sparse field patch of one row type: every field it carries is an absolute setter.
pub trait RowPatch<R>: Clone + Default + PartialEq {
    /// ✍️ Writes the patch into the row it was raised for; the central applier is the only caller.
    fn commit_into(&self, row: &mut R) -> Result<(), MutationApplyError>;
    /// ➕️ Composes `self` (row→mid) with `later` (mid→after) into row→after.
    fn absorb(&mut self, later: Self);
    /// 🔁️ The patch that, applied after `self`, restores `row` — the absolute pre-values of every field `self` carries.
    fn inverse(&self, row: &R) -> Self;
    /// 🧭️ The patch that turns `before` into `after`.
    fn between(before: &R, after: &R) -> Self;
    /// 🕳️ Whether the patch carries nothing.
    fn is_empty(&self) -> bool;
}

/// 🧱️ The plain, derive-free form of a keyed list delta; the concrete wire types from [`norm_list_delta!`] convert to and from it.
#[derive(Clone, Debug, PartialEq)]
pub struct Parts<R, Q> {
    /// ➖️ Keys of the base rows that leave.
    pub removed: Vec<String>,
    /// ➕️ Rows that enter, in insertion order, each with the key of the row it follows.
    pub added: Vec<(Option<String>, R)>,
    /// 🩹 Patches per surviving or added key.
    pub modified: Vec<(String, Q)>,
}

impl<R, Q> Default for Parts<R, Q> {
    fn default() -> Self {
        Self { removed: Vec::new(), added: Vec::new(), modified: Vec::new() }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Origin {
    Base,
    Added,
}

fn refusal(code: &'static str, message: &'static str, section: &'static str, index: usize) -> MutationApplyError {
    MutationApplyError::new(code, message).at([section.to_string(), index.to_string()])
}

impl<R: Keyed + Clone + PartialEq, Q: RowPatch<R>> Parts<R, Q> {
    /// ➕️ The delta that inserts `row` at `index` of `base`.
    pub fn insertion(base: &[R], index: usize, row: R) -> Self {
        let after = index.checked_sub(1).and_then(|previous| base.get(previous)).map(|anchor| anchor.key().to_string());
        Self { added: vec![(after, row)], ..Self::default() }
    }

    /// ➖️ The delta that removes the row `key`.
    pub fn removal(key: &str) -> Self {
        Self { removed: vec![key.to_string()], ..Self::default() }
    }

    /// 🩹 The delta that patches the row `key`.
    pub fn modification(key: &str, patch: Q) -> Self {
        Self { modified: vec![(key.to_string(), patch)], ..Self::default() }
    }

    /// 🕳️ Whether the delta changes nothing.
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.added.is_empty() && self.modified.is_empty()
    }

    fn anchor_position(working: &[(Origin, R)], anchor: &str) -> Option<usize> {
        working.iter().rposition(|(origin, row)| *origin == Origin::Added && row.key() == anchor).or_else(|| working.iter().position(|(origin, row)| *origin == Origin::Base && row.key() == anchor))
    }

    /// ✍️ The list this delta turns `base` into; the central applier is the only caller.
    pub fn commit_onto(&self, base: &[R]) -> Result<Vec<R>, MutationApplyError> {
        let mut working: Vec<(Origin, R)> = base.iter().cloned().map(|row| (Origin::Base, row)).collect();
        for (index, (after, row)) in self.added.iter().enumerate() {
            let at = match after {
                None => 0,
                Some(anchor) => Self::anchor_position(&working, anchor).ok_or_else(|| refusal("mutation.apply.missing-target", "added row follows a row that does not exist", "added", index))? + 1,
            };
            if working.iter().any(|(origin, existing)| existing.key() == row.key() && (*origin == Origin::Added || !self.removed.iter().any(|key| key == row.key()))) {
                return Err(refusal("mutation.apply.duplicate-target", "added row key already exists", "added", index));
            }
            working.insert(at, (Origin::Added, row.clone()));
        }
        for (index, key) in self.removed.iter().enumerate() {
            let at = working.iter().position(|(origin, row)| *origin == Origin::Base && row.key() == key).ok_or_else(|| refusal("mutation.apply.missing-target", "removed row does not exist", "removed", index))?;
            working.remove(at);
        }
        for (index, (key, patch)) in self.modified.iter().enumerate() {
            let (_, row) = working.iter_mut().find(|(_, row)| row.key() == key).ok_or_else(|| refusal("mutation.apply.missing-target", "modified row does not exist", "modified", index))?;
            patch.commit_into(row).map_err(|error| error.under(["modified".to_string(), index.to_string()]))?;
        }
        Ok(working.into_iter().map(|(_, row)| row).collect())
    }

    /// 🔁️ The negative delta: applied to what `self` makes of `base`, it restores `base`.
    pub fn inverse(&self, base: &[R]) -> Self {
        let removed = self.added.iter().map(|(_, row)| row.key().to_string()).collect();
        let added = base
            .iter()
            .enumerate()
            .filter(|(_, row)| self.removed.iter().any(|key| key == row.key()))
            .map(|(position, row)| (position.checked_sub(1).map(|previous| base[previous].key().to_string()), row.clone()))
            .collect();
        let modified = self
            .modified
            .iter()
            .filter(|(key, _)| !self.added.iter().any(|(_, row)| row.key() == key) && !self.removed.contains(key))
            .filter_map(|(key, patch)| base.iter().find(|row| row.key() == key).map(|row| (key.clone(), patch.inverse(row))))
            .collect();
        Self { removed, added, modified }
    }

    /// 🧭️ The delta that turns `base` into `other` — state differencing for sync and import, never for a mutation leaf.
    pub fn between(base: &[R], other: &[R]) -> Self {
        let in_other = |key: &str| other.iter().any(|row| row.key() == key);
        let in_base = |key: &str| base.iter().any(|row| row.key() == key);
        let common_in_base: Vec<&str> = base.iter().map(Keyed::key).filter(|key| in_other(key)).collect();
        let common_in_other: Vec<&str> = other.iter().map(Keyed::key).filter(|key| in_base(key)).collect();
        let reordered = common_in_base != common_in_other;
        let replaced = |key: &str| reordered && in_base(key) && in_other(key);
        let removed = base.iter().map(Keyed::key).filter(|key| !in_other(key) || replaced(key)).map(str::to_string).collect();
        let added = other
            .iter()
            .enumerate()
            .filter(|(_, row)| !in_base(row.key()) || replaced(row.key()))
            .map(|(position, row)| (position.checked_sub(1).map(|previous| other[previous].key().to_string()), row.clone()))
            .collect();
        let modified = if reordered {
            Vec::new()
        } else {
            base.iter().filter_map(|before| other.iter().find(|row| row.key() == before.key()).filter(|after| *after != before).map(|after| (before.key().to_string(), Q::between(before, after)))).collect()
        };
        Self { removed, added, modified }
    }

    fn cancel(added: &mut Vec<(Option<String>, R)>, target: usize) {
        let target_key = added[target].1.key().to_string();
        let mut chains = Chains::default();
        let mut rewrites = Vec::new();
        for (position, (after, row)) in added.iter().enumerate() {
            if position > target && after.as_deref() == Some(target_key.as_str()) {
                if let Some(predecessor) = chains.predecessor(&target_key) {
                    rewrites.push((position, predecessor));
                }
            }
            chains.insert(after.as_deref(), row.key());
        }
        for (position, anchor) in rewrites {
            added[position].0 = anchor;
        }
        added.remove(target);
    }

    /// ➕️ Composes `self` (base→mid) with `later` (mid→after) into base→after, in place.
    pub fn absorb(&mut self, later: Self) {
        let mut earlier_rows = self.added.len();
        self.added.extend(later.added);
        for key in later.removed {
            self.modified.retain(|(modified, _)| *modified != key);
            match self.added[..earlier_rows].iter().position(|(_, row)| row.key() == key) {
                Some(position) => {
                    Self::cancel(&mut self.added, position);
                    earlier_rows -= 1;
                }
                None => self.removed.push(key),
            }
        }
        for (key, patch) in later.modified {
            match self.added.iter_mut().rev().find(|(_, row)| row.key() == key) {
                Some((_, row)) => {
                    if patch.commit_into(row).is_err() {
                        self.modified.push((key, patch));
                    }
                }
                None => match self.modified.iter_mut().find(|(modified, _)| *modified == key) {
                    Some((_, existing)) => existing.absorb(patch),
                    None => self.modified.push((key, patch)),
                },
            }
        }
    }
}

#[derive(Default)]
struct Chains {
    roots: Vec<(Option<String>, Vec<String>)>,
}

impl Chains {
    fn insert(&mut self, after: Option<&str>, key: &str) {
        if let Some(anchor) = after {
            if let Some((_, chain)) = self.roots.iter_mut().find(|(_, chain)| chain.iter().any(|member| member == anchor)) {
                let at = chain.iter().position(|member| member == anchor).map_or(0, |position| position + 1);
                chain.insert(at, key.to_string());
                return;
            }
        }
        match self.roots.iter_mut().find(|(root, _)| root.as_deref() == after) {
            Some((_, chain)) => chain.insert(0, key.to_string()),
            None => self.roots.push((after.map(str::to_string), vec![key.to_string()])),
        }
    }

    fn predecessor(&self, key: &str) -> Option<Option<String>> {
        self.roots.iter().find_map(|(root, chain)| chain.iter().position(|member| member == key).map(|at| if at == 0 { root.clone() } else { Some(chain[at - 1].clone()) }))
    }
}

/// 🚨️ The refusal a delta that cannot apply raises.
pub type ApplyError = MutationApplyError;

/// 🧬️ Defines the concrete wire type of one keyed list delta: `$delta` (the list), `$addition` (an inserted row with the key
/// it follows) and `$modification` (a key with its patch), plus the [`Keyed`] key of `$row` (its `$key` field). The generic
/// algebra is [`Parts`]; the methods here only convert.
#[macro_export]
macro_rules! norm_list_delta {
    ($(#[$meta:meta])* $vis:vis $delta:ident { addition: $addition:ident, modification: $modification:ident, row: $row:ty, patch: $patch:ty, key: $key:ident $(,)? }) => {
        impl $crate::list_delta::Keyed for $row {
            fn key(&self) -> &str {
                &self.$key
            }
        }

        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
        #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(test, serde(rename_all = "camelCase"))]
        #[value(rename_all = "camelCase")]
        $vis struct $addition {
            pub after: Option<String>,
            pub row: $row,
        }

        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
        #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(test, serde(rename_all = "camelCase"))]
        #[value(rename_all = "camelCase")]
        $vis struct $modification {
            pub key: String,
            pub patch: $patch,
        }

        $(#[$meta])*
        #[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
        #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(test, serde(rename_all = "camelCase", default))]
        #[value(rename_all = "camelCase", default)]
        $vis struct $delta {
            pub removed: Vec<String>,
            pub added: Vec<$addition>,
            pub modified: Vec<$modification>,
        }

        impl $delta {
            fn into_parts(self) -> $crate::list_delta::Parts<$row, $patch> {
                $crate::list_delta::Parts {
                    removed: self.removed,
                    added: self.added.into_iter().map(|entry| (entry.after, entry.row)).collect(),
                    modified: self.modified.into_iter().map(|entry| (entry.key, entry.patch)).collect(),
                }
            }

            fn from_parts(parts: $crate::list_delta::Parts<$row, $patch>) -> Self {
                Self {
                    removed: parts.removed,
                    added: parts.added.into_iter().map(|(after, row)| $addition { after, row }).collect(),
                    modified: parts.modified.into_iter().map(|(key, patch)| $modification { key, patch }).collect(),
                }
            }

            /// ➕️ The delta that inserts `row` at `index` of `base`.
            pub fn insertion(base: &[$row], index: usize, row: $row) -> Self {
                Self::from_parts($crate::list_delta::Parts::insertion(base, index, row))
            }

            /// ➖️ The delta that removes the row `key`.
            pub fn removal(key: &str) -> Self {
                Self::from_parts($crate::list_delta::Parts::removal(key))
            }

            /// 🩹 The delta that patches the row `key`.
            pub fn modification(key: &str, patch: $patch) -> Self {
                Self::from_parts($crate::list_delta::Parts::modification(key, patch))
            }

            /// ✍️ The list this delta turns `base` into; reached only through the central applier's `MutationDiff::apply`.
            pub fn commit_onto(&self, base: &[$row]) -> Result<Vec<$row>, $crate::list_delta::ApplyError> {
                self.clone().into_parts().commit_onto(base)
            }

            /// ➕️ Composes `self` with the delta `later` applied after it.
            pub fn absorb(&mut self, later: Self) {
                let mut parts = std::mem::take(self).into_parts();
                parts.absorb(later.into_parts());
                *self = Self::from_parts(parts);
            }

            /// 🔁️ The negative delta over `base`.
            pub fn inverse(&self, base: &[$row]) -> Self {
                Self::from_parts(self.clone().into_parts().inverse(base))
            }

            /// 🧭️ The delta from `base` to `other`, for sync and import.
            pub fn between(base: &[$row], other: &[$row]) -> Self {
                Self::from_parts($crate::list_delta::Parts::between(base, other))
            }

            /// 🕳️ Whether the delta changes nothing.
            pub fn is_empty(&self) -> bool {
                self.removed.is_empty() && self.added.is_empty() && self.modified.is_empty()
            }
        }
    };
}

/// 🩹 Defines the concrete sparse patch `$patch` of the row type `$row`: every `set` field is an `Option` absolute setter, every
/// `nest` field a keyed list delta of a list the row owns.
#[macro_export]
macro_rules! norm_row_patch {
    ($(#[$meta:meta])* $vis:vis $patch:ident of $row:ty { set { $($set:ident : $set_ty:ty),* $(,)? } $(nest { $($nest:ident : $nest_delta:ty),* $(,)? })? }) => {
        $(#[$meta])*
        #[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
        #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(test, serde(rename_all = "camelCase", default))]
        #[value(rename_all = "camelCase", default)]
        $vis struct $patch {
            $(pub $set: Option<$set_ty>,)*
            $($(pub $nest: $nest_delta,)*)?
        }

        impl $crate::list_delta::RowPatch<$row> for $patch {
            fn commit_into(&self, row: &mut $row) -> Result<(), $crate::list_delta::ApplyError> {
                $(if let Some(value) = &self.$set {
                    row.$set = value.clone();
                })*
                $($(row.$nest = self.$nest.commit_onto(&row.$nest).map_err(|error| error.under([stringify!($nest)]))?;)*)?
                Ok(())
            }

            fn absorb(&mut self, later: Self) {
                $(if later.$set.is_some() {
                    self.$set = later.$set;
                })*
                $($(self.$nest.absorb(later.$nest);)*)?
            }

            fn inverse(&self, row: &$row) -> Self {
                Self {
                    $($set: self.$set.as_ref().map(|_| row.$set.clone()),)*
                    $($($nest: self.$nest.inverse(&row.$nest),)*)?
                }
            }

            fn between(before: &$row, after: &$row) -> Self {
                Self {
                    $($set: (before.$set != after.$set).then(|| after.$set.clone()),)*
                    $($($nest: <$nest_delta>::between(&before.$nest, &after.$nest),)*)?
                }
            }

            fn is_empty(&self) -> bool {
                true $(&& self.$set.is_none())* $($(&& self.$nest.is_empty())*)?
            }
        }
    };
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
