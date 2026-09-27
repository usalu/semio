//! ✂️ Range-text operations — the Rust twin of `✂️text-splice/🟦️.ts`. A splice names what its author saw around
//! the replaced run, so it lands where its author meant it in any text other authors changed meanwhile, the same way in every
//! language. Positions and lengths count Unicode scalar values (`char`s).
//!
//! @see ../🧬️schema/✂️text-splice/🔣️.json

/// ✂️ One range-text operation as its author saw it: at scalar `start` of the author's text, `deleted` was replaced by
/// `insert`; `before`/`after` are up to [`TEXT_SPLICE_CONTEXT_SCALARS`] scalars around the replaced run.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextSplice {
    pub start: u32,
    pub deleted: String,
    pub insert: String,
    pub before: String,
    pub after: String,
}

/// 📍️ Where a splice lands in a concrete text: the run it replaces there, and whether its `deleted` run was already gone
/// (`clamped`: nothing is deleted, the insert still lands) or its context was nowhere to be found.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocatedTextSplice {
    pub start: u32,
    pub delete_length: u32,
    pub clamped: bool,
}

/// 🧾️ A splice applied to a text: the new text, where it landed, and the inverse splice that restores the text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppliedTextSplice {
    pub text: String,
    pub located: LocatedTextSplice,
    pub inverse: TextSplice,
}

/// 📏️ Context scalars a splice carries on each side.
pub const TEXT_SPLICE_CONTEXT_SCALARS: usize = 32;

/// 🔗️ Two-sided context patterns shorter than this many scalars per side are never searched: two scalars of context match all
/// over a real document, one-sided context of the same length near the author's position is the better witness.
pub const TEXT_SPLICE_MIN_TWO_SIDED_SCALARS: usize = 4;

/// 🔎️ Every start index of `needle` in `hay` (overlapping); an empty needle matches at every position.
fn occurrences(hay: &[char], needle: &[char]) -> Vec<usize> {
    if needle.is_empty() {
        return (0..=hay.len()).collect();
    }
    if needle.len() > hay.len() {
        return Vec::new();
    }
    (0..=hay.len() - needle.len()).filter(|&index| hay[index..index + needle.len()] == *needle).collect()
}

/// 🎯️ The candidate nearest to `expected`; ties take the lowest position.
fn nearest(positions: impl IntoIterator<Item = usize>, expected: usize) -> Option<usize> {
    positions.into_iter().min_by_key(|&position| (position.abs_diff(expected), position))
}

fn joined(parts: &[&[char]]) -> Vec<char> {
    parts.iter().flat_map(|part| part.iter().copied()).collect()
}

impl TextSplice {
    /// 📍️ Locates the splice in `text`, deterministically and identically to the TS twin:
    /// 1. the run with context on both sides, `k` = the longer side's length down to [`TEXT_SPLICE_MIN_TWO_SIDED_SCALARS`];
    /// 2. the run with context on one side, `k` = the longer side's length down to 1 (both sides of one `k` compete);
    /// 3. the run alone (only when it is not empty);
    /// 4. a run that is gone deletes nothing (`clamped`): steps 1–2 again with an empty run find the insertion point;
    /// 5. otherwise the author's `start`, clamped to the text (`clamped` when the splice carried any context).
    /// Every step takes the match nearest to the author's `start`, ties to the lowest position.
    pub fn locate(&self, text: &str) -> LocatedTextSplice {
        let hay: Vec<char> = text.chars().collect();
        let (before, deleted, after): (Vec<char>, Vec<char>, Vec<char>) = (self.before.chars().collect(), self.deleted.chars().collect(), self.after.chars().collect());
        let start = self.start as usize;
        let longest = before.len().max(after.len());
        let anchored = |run: &[char]| -> Option<usize> {
            let mut k = longest;
            while k >= TEXT_SPLICE_MIN_TWO_SIDED_SCALARS {
                let head = &before[before.len() - k.min(before.len())..];
                let tail = &after[..k.min(after.len())];
                if head.is_empty() || tail.is_empty() {
                    break;
                }
                if let Some(found) = nearest(occurrences(&hay, &joined(&[head, run, tail])).into_iter().map(|index| index + head.len()), start) {
                    return Some(found);
                }
                k -= 1;
            }
            for k in (1..=longest).rev() {
                let head = &before[before.len() - k.min(before.len())..];
                let tail = &after[..k.min(after.len())];
                let left = if head.is_empty() { Vec::new() } else { occurrences(&hay, &joined(&[head, run])).into_iter().map(|index| index + head.len()).collect() };
                let right = if tail.is_empty() { Vec::new() } else { occurrences(&hay, &joined(&[run, tail])) };
                if let Some(found) = nearest(left.into_iter().chain(right), start) {
                    return Some(found);
                }
            }
            None
        };
        let exact = anchored(&deleted).or_else(|| if deleted.is_empty() { None } else { nearest(occurrences(&hay, &deleted), start) });
        if let Some(found) = exact {
            return LocatedTextSplice { start: found as u32, delete_length: deleted.len() as u32, clamped: false };
        }
        if !deleted.is_empty() {
            if let Some(found) = anchored(&[]) {
                return LocatedTextSplice { start: found as u32, delete_length: 0, clamped: true };
            }
        }
        LocatedTextSplice { start: start.min(hay.len()) as u32, delete_length: 0, clamped: !deleted.is_empty() || !before.is_empty() || !after.is_empty() }
    }

    /// ✂️ Applies the splice to `text` where it locates, answering the new text and the inverse splice (the removed run back in
    /// place of the inserted one, with `context` scalars of the new text around it).
    pub fn apply(&self, text: &str, context: usize) -> AppliedTextSplice {
        let located = self.locate(text);
        let hay: Vec<char> = text.chars().collect();
        let insert: Vec<char> = self.insert.chars().collect();
        let (at, length) = (located.start as usize, located.delete_length as usize);
        let removed: String = hay[at..at + length].iter().collect();
        let next: Vec<char> = joined(&[&hay[..at], &insert, &hay[at + length..]]);
        let end = at + insert.len();
        AppliedTextSplice {
            text: next.iter().collect(),
            located,
            inverse: TextSplice {
                start: located.start,
                deleted: self.insert.clone(),
                insert: removed,
                before: next[at.saturating_sub(context)..at].iter().collect(),
                after: next[end..(end + context).min(next.len())].iter().collect(),
            },
        }
    }

    /// ⌨️ The ONE splice an editor sends for its own change `previous → next`: the common scalar prefix and suffix bound the
    /// changed run, the context is what the author saw around it; `None` when nothing changed.
    pub fn from_edit(previous: &str, next: &str, context: usize) -> Option<Self> {
        let (a, b): (Vec<char>, Vec<char>) = (previous.chars().collect(), next.chars().collect());
        let prefix = a.iter().zip(&b).take_while(|(left, right)| left == right).count();
        if prefix == a.len() && prefix == b.len() {
            return None;
        }
        let suffix = a[prefix..].iter().rev().zip(b[prefix..].iter().rev()).take_while(|(left, right)| left == right).count();
        Some(Self {
            start: prefix as u32,
            deleted: a[prefix..a.len() - suffix].iter().collect(),
            insert: b[prefix..b.len() - suffix].iter().collect(),
            before: a[prefix.saturating_sub(context)..prefix].iter().collect(),
            after: a[a.len() - suffix..(a.len() - suffix + context).min(a.len())].iter().collect(),
        })
    }

    /// 🧷️ A zero-width splice marking scalar `position` of `text` — how a caret or a selection end travels through other
    /// authors' splices: locating the marker in the new text is the position's new place.
    pub fn marker(text: &str, position: usize, context: usize) -> Self {
        let hay: Vec<char> = text.chars().collect();
        let at = position.min(hay.len());
        Self { start: at as u32, deleted: String::new(), insert: String::new(), before: hay[at.saturating_sub(context)..at].iter().collect(), after: hay[at..(at + context).min(hay.len())].iter().collect() }
    }
}

/// 🔁️ An editor host's view after the guest published `remote`: every splice the guest has not applied yet, in the order the
/// host sent them, folded onto `remote`, and the local selection (scalar `anchor`/`caret` in `local`) carried along.
pub fn rebase_text_edits(remote: &str, unapplied: &[TextSplice], local: &str, anchor: usize, caret: usize) -> (String, usize, usize) {
    let text = unapplied.iter().fold(remote.to_string(), |text, splice| splice.apply(&text, TEXT_SPLICE_CONTEXT_SCALARS).text);
    let place = |position: usize| TextSplice::marker(local, position, TEXT_SPLICE_CONTEXT_SCALARS).locate(&text).start as usize;
    let (anchor, caret) = (place(anchor), place(caret));
    (text, anchor, caret)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "../🧪️tests/✂️text-splice/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
