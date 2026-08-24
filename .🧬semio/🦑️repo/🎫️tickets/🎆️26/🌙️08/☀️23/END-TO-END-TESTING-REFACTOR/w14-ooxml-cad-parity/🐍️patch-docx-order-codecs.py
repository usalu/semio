#!/usr/bin/env python3
"""🔀️ DOCX-specific half of the `NamedTripleDiff::order` repair: the two hand-rolled generic
triple codecs gain a fourth section, the hand-rolled relationships-by-owner triple (a `HashMap`,
so genuinely unordered) states why it carries none, and a test pins the law in both directions.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR, wave 14.
"""
PATH = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"

with open(PATH, encoding="utf-8") as handle:
    text = handle.read()


def sub(old: str, new: str, count: int = 1) -> None:
    global text
    found = text.count(old)
    assert found == count, f"expected {count}, found {found}:\n{old[:200]}"
    text = text.replace(old, new)


# 1️⃣ text codec — a fourth `[order]` section
sub(
    """/// 🏷️ `[removed];[modified];[added]` -- generic over `NamedTripleDiff<K,D,T>`'s own `K`/`D`/`T`,""",
    """/// 🏷️ `[removed];[modified];[added];[order]` -- generic over `NamedTripleDiff<K,D,T>`'s own `K`/`D`/`T`,""",
)
sub(
    """    let added = diff.added.iter().map(|t| enc_t(t)).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}""",
    """    let added = diff.added.iter().map(|t| enc_t(t)).collect::<Vec<_>>().join(",");
    let order = diff.order.iter().map(|k| enc_k(k)).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}];[{order}]")
}""",
)
sub(
    """    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("named triple: expected 3 sections, got {}", three.len())) };""",
    """    let four = split_top_level(body, ';');
    let [removed_s, modified_s, added_s, order_s] = four.as_slice() else { return Err(format!("named triple: expected 4 sections, got {}", four.len())) };""",
)
sub(
    """    let added = split_top_level(strip_brackets(added_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|s| dec_t(s)).collect::<Result<Vec<_>, String>>()?;
    Ok(NamedTripleDiff { removed, modified, added })
}""",
    """    let added = split_top_level(strip_brackets(added_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|s| dec_t(s)).collect::<Result<Vec<_>, String>>()?;
    let order = split_top_level(strip_brackets(order_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|s| dec_k(s)).collect::<Result<Vec<_>, String>>()?;
    Ok(NamedTripleDiff { removed, modified, added, order })
}""",
)

# 2️⃣ binary codec — a fourth varint-counted section
sub(
    """/// 🏷️ Binary twin of `enc_named_triple`/`dec_named_triple` -- three varint-counted sections
/// (removed keys / modified key+diff pairs / added whole items), generic over `K`/`D`/`T`.""",
    """/// 🏷️ Binary twin of `enc_named_triple`/`dec_named_triple` -- four varint-counted sections
/// (removed keys / modified key+diff pairs / added whole items / the final key order), generic
/// over `K`/`D`/`T`.""",
)
sub(
    """    store::pack_rt::write_varint_u64(out, diff.added.len() as u64);
    for t in &diff.added {
        enc_t(t, out);
    }
}""",
    """    store::pack_rt::write_varint_u64(out, diff.added.len() as u64);
    for t in &diff.added {
        enc_t(t, out);
    }
    store::pack_rt::write_varint_u64(out, diff.order.len() as u64);
    for k in &diff.order {
        enc_k(k, out);
    }
}""",
)
sub(
    """    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        added.push(dec_t(reader)?);
    }
    Ok(NamedTripleDiff { removed, modified, added })
}""",
    """    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        added.push(dec_t(reader)?);
    }
    let order_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut order = Vec::with_capacity(order_count as usize);
    for _ in 0..order_count {
        order.push(dec_k(reader)?);
    }
    Ok(NamedTripleDiff { removed, modified, added, order })
}""",
)

# 3️⃣ relationships-by-owner: a `HashMap`, so it HAS no order to transport — said, not assumed.
sub(
    """    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        Some(DocxOpcRelationshipsDiff { removed, modified, added })
    }
}""",
    """    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        // 🗺️ Relationships live in a `HashMap<owner, …>`, which HAS no order to transport, so this
        // triple's `order` is empty by construction rather than by omission — `apply_relationships`
        // inserts by key and never reads one.
        Some(DocxOpcRelationshipsDiff { removed, modified, added, order: Vec::new() })
    }
}""",
)
sub(
    """    DocxOpcRelationshipsDiff { removed, modified, added }
}""",
    """    DocxOpcRelationshipsDiff { removed, modified, added, order: Vec::new() }
}""",
)

# 4️⃣ the law, pinned at unit level in both directions
sub(
    """    #[semio_framework_async_macros::async_test]
    async fn rejects_missing_style_target_without_mutating_base() {""",
    """    /// 🧪️ `SetSnapshot` is a TOTAL replacement, so `DocxDiff::between(base, next)` applied to
    /// `base` has to land on `next` EXACTLY — the ORDER of the name-keyed style list included,
    /// because `w:styles`' declaration order is what `semantic-docx-ecma-376-mutate-v1` projects
    /// by index. Until wave 14 the named triple was order-blind (survivors kept their base order,
    /// additions were appended), so undoing `set-snapshot` on the real `📜️example-readme.docx`
    /// returned all seven real styles with six of them in the wrong place — 12 differences
    /// against the `zip`+`quick-xml` oracle in `mutate-docx-ecma-376::inverse-set-snapshot`. The
    /// fixture's own seven styles and the case's own three-style `set-snapshot` target are used
    /// here verbatim, so this test fails for the same reason the case did.
    #[test]
    fn set_snapshot_and_its_inverse_reproduce_the_exact_style_order() {
        let of = |ids: &[&str]| DocxSnapshot::from_parts(OpcPackage::empty(), DocxDocument { body: Vec::new(), styles: ids.iter().map(|id| DocxStyle { id: (*id).into(), name: (*id).into(), based_on: None }).collect() });
        let base = of(&["Normal", "Title", "Heading1", "Heading2", "Heading3", "Code", "TableCell"]);
        let next = of(&["Normal", "Heading1", "TableCell"]);

        let forward = DocxDiff::between(&base, &next);
        assert_eq!(forward.apply(&base).expect("the forward diff applies"), next, "set-snapshot must land on exactly the snapshot it carries");
        assert_eq!(forward.inverse(&base).apply(&next).expect("the inverse applies"), base, "undoing set-snapshot must restore the style order it found");

        // A pure REORDER carries no removal, no modification and no addition whatsoever, so the
        // order field is the only thing in the triple that can express it at all.
        let shuffled = of(&["TableCell", "Normal", "Heading1"]);
        let reorder = DocxDiff::between(&next, &shuffled);
        assert!(!reorder.is_empty(), "a pure reorder must not diff to nothing");
        assert_eq!(reorder.apply(&next).expect("the reorder applies"), shuffled);
    }

    #[semio_framework_async_macros::async_test]
    async fn rejects_missing_style_target_without_mutating_base() {""",
)

with open(PATH, "w", encoding="utf-8") as handle:
    handle.write(text)
print(f"patched {PATH}")
