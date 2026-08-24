#!/usr/bin/env python3
"""🔀️ Teach one artifact's `NamedTripleDiff` engine to transport the collection's ORDER.

`between_named`/`apply_named` identify items by KEY and rebuild the collection as
"survivors in base order, then `added` appended".  For a name-keyed collection whose
items nonetheless sit in a SIGNIFICANT order (`w:styles`' declaration order is projected
by index) that is not a faithful diff: `apply(base, between(base, other))` reorders.
This adds an `order` field carrying the exact final key sequence, populated only when the
default sequence would not reproduce it.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR, wave 14.
"""
import sys

REPO = "/Users/ueli/Documents/semio"


def patch(path: str) -> None:
    with open(path, encoding="utf-8") as handle:
        text = handle.read()
    original = text

    def sub(old: str, new: str, count: int = 1) -> None:
        nonlocal text
        found = text.count(old)
        assert found == count, f"{path}: expected {count} occurrence(s), found {found} of:\n{old[:200]}"
        text = text.replace(old, new)

    # 1️⃣ the struct itself + its Default
    sub(
        """pub struct NamedTripleDiff<K, D, T> {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<K>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<NamedModified<K, D>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<T>,
}

impl<K, D, T> Default for NamedTripleDiff<K, D, T> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}""",
        """pub struct NamedTripleDiff<K, D, T> {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<K>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<NamedModified<K, D>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<T>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub order: Vec<K>,
}

impl<K, D, T> Default for NamedTripleDiff<K, D, T> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new(), order: Vec::new() }
    }
}""",
    )

    # 2️⃣ the two order helpers, in front of `between_named`
    sub(
        """//#region 🔖️GenericNamedEngine
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_named<K, T, D>(""",
        """//#region 🔖️GenericNamedEngine
/// 🧮️ The key sequence `removed`/`added` alone imply — survivors in base order, then the additions
/// in carried order. `NamedTripleDiff::order` is populated exactly when the real target sequence is
/// NOT this one, so an order-insignificant collection never pays for the field.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn default_named_order<K: PartialEq + Clone>(base_keys: &[K], removed: &[K], added_keys: &[K]) -> Vec<K> {
    base_keys.iter().filter(|k| !removed.contains(k)).chain(added_keys.iter()).cloned().collect()
}

/// 🔀️ Rebuilds `items` into `order` when one is carried, and fails loudly when it is not a
/// permutation of what the collection actually holds — a silently dropped or duplicated item is
/// precisely the failure this field exists to prevent.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn reorder_named<K: PartialEq, T>(items: &mut Vec<T>, order: &[K], key_of: impl Fn(&T) -> K) -> MutationApplyResult<()> {
    if order.is_empty() {
        return Ok(());
    }
    if order.len() != items.len() {
        return Err(MutationApplyError::new("mutation.apply.invalid-order", "named ordering does not cover the resulting collection").at(["order"]));
    }
    let mut pool: Vec<Option<T>> = std::mem::take(items).into_iter().map(Some).collect();
    for key in order {
        let slot = pool
            .iter()
            .position(|held| matches!(held, Some(item) if key_of(item) == *key))
            .ok_or_else(|| MutationApplyError::new("mutation.apply.invalid-order", "named ordering names an item the collection does not carry").at(["order"]))?;
        items.push(pool[slot].take().expect("the slot was located as occupied one line above"));
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_named<K, T, D>(""",
    )

    # 3️⃣ between_named records the target order
    sub(
        """    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        Some(NamedTripleDiff { removed, modified, added })
    }
}""",
        """    let base_keys: Vec<K> = base.iter().map(&key_of).collect();
    let other_keys: Vec<K> = other.iter().map(&key_of).collect();
    let added_keys: Vec<K> = added.iter().map(&key_of).collect();
    let order = if default_named_order(&base_keys, &removed, &added_keys) == other_keys { Vec::new() } else { other_keys };
    if removed.is_empty() && modified.is_empty() && added.is_empty() && order.is_empty() {
        None
    } else {
        Some(NamedTripleDiff { removed, modified, added, order })
    }
}""",
    )

    # 4️⃣ apply_named honours it
    sub(
        """    for item in &diff.added {
        items.push(item.clone());
    }
    Ok(())
}""",
        """    for item in &diff.added {
        items.push(item.clone());
    }
    reorder_named(items, &diff.order, &key_of)?;
    Ok(())
}""",
    )

    # 5️⃣ inverse_named restores the BASE order
    sub(
        """    NamedTripleDiff { removed, modified, added }
}""",
        """    let base_keys: Vec<K> = base_items.iter().map(&key_of).collect();
    let other_keys = if diff.order.is_empty() { default_named_order(&base_keys, &diff.removed, &removed) } else { diff.order.clone() };
    let added_keys: Vec<K> = added.iter().map(&key_of).collect();
    let order = if default_named_order(&other_keys, &removed, &added_keys) == base_keys { Vec::new() } else { base_keys };
    NamedTripleDiff { removed, modified, added, order }
}""",
    )

    # 6️⃣ absorb_named composes the two orders — `d2`'s wins outright because it is already the
    #    FINAL sequence; otherwise `d1`'s is carried through `d2`'s own removals and additions.
    sub(
        """    let d1_added_keys: Vec<K> = d1.added.iter().map(&key_of).collect();
    let mut removed = d1.removed.clone();""",
        """    let d1_added_keys: Vec<K> = d1.added.iter().map(&key_of).collect();
    let d1_order = d1.order.clone();
    let mut removed = d1.removed.clone();""",
    )
    sub(
        """    NamedTripleDiff { removed, modified, added: working_added }
}""",
        """    let order = if !d2.order.is_empty() {
        d2.order.clone()
    } else if d1_order.is_empty() {
        Vec::new()
    } else {
        let mut composed: Vec<K> = d1_order.into_iter().filter(|k| !d2.removed.contains(k)).collect();
        for a2 in &d2.added {
            let k2 = key_of(a2);
            if !composed.contains(&k2) {
                composed.push(k2);
            }
        }
        composed
    };
    NamedTripleDiff { removed, modified, added: working_added, order }
}""",
    )

    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)
    print(f"patched {path} ({len(original)} -> {len(text)} bytes)")


if __name__ == "__main__":
    for target in sys.argv[1:]:
        patch(target if target.startswith("/") else f"{REPO}/{target}")
