//! 👁️ Ephemeral local visibility shared by outliner filtering and interaction topology.

use super::*;
use std::sync::{Mutex, PoisonError};

static VISIBLE: Mutex<Vec<(String, Option<Vec<String>>)>> = Mutex::new(Vec::new());

fn with_visible<R>(domain: &str, write: impl FnOnce(&mut Option<Vec<String>>) -> R) -> R {
    let mut store = VISIBLE.lock().unwrap_or_else(PoisonError::into_inner);
    let index = store.iter().position(|(slot, _)| slot == domain).unwrap_or_else(|| {
        store.push((domain.to_owned(), None));
        store.len() - 1
    });
    write(&mut store[index].1)
}

/// 📣 Publishes which rows of `tree` the outliner filter lets through, so the interaction topology (and with it the framework `selectAll` verb) respects it; an empty filter admits every entity (`None`).
pub fn publish_visible_rows(domain: &str, tree: &DslValue, filter: &str) {
    let visible = (!filter.trim().is_empty()).then(|| match tree {
        DslValue::Object(fields) => fields.iter().filter_map(|(name, value)| value.as_array().map(|rows| (name, rows))).flat_map(|(name, rows)| collection_rows(name, rows, filter)).map(|row| row.id).filter(|id| !id.is_empty()).collect::<Vec<String>>(),
        _ => Vec::new(),
    });
    with_visible(domain, |slot| *slot = visible);
}

/// 🔎️ The ids of the rows the outliner filter of `domain` admits, or `None` when no filter narrows the list.
pub fn visible_rows(domain: &str) -> Option<Vec<String>> {
    with_visible(domain, |slot| slot.clone())
}

