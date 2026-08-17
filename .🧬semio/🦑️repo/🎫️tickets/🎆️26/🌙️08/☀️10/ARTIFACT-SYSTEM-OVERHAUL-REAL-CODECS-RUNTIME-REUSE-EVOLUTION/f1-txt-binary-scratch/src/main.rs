//! Standalone scratch validation of the TxtLinesDiff / BinaryDiff absorb algorithms, copied
//! (derives/serde stripped) from the real crate files, to get a fast own-target-dir signal
//! independent of the shared workspace build (which is currently blocked by a concurrent
//! session's unrelated `json` artifact breakage). Ported back once the workspace is green.

use std::collections::{HashMap, HashSet};

//#region TxtLinesDiff (copy of the real algorithm)
#[derive(Clone, Debug, PartialEq)]
struct LineAdded { index: usize, text: String }
#[derive(Clone, Debug, PartialEq)]
struct LineModified { index: usize, text: String }
#[derive(Clone, Debug, Default, PartialEq)]
struct LinesDiff { removed: Vec<usize>, modified: Vec<LineModified>, added: Vec<LineAdded> }

impl LinesDiff {
    fn is_empty(&self) -> bool { self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty() }

    fn apply(&self, base: &[String]) -> Vec<String> {
        let mut items: Vec<Option<String>> = base.iter().cloned().map(Some).collect();
        for m in &self.modified {
            if let Some(slot) = items.get_mut(m.index) { *slot = Some(m.text.clone()); }
        }
        let removed: HashSet<usize> = self.removed.iter().copied().collect();
        let mut survivors: Vec<String> = items.into_iter().enumerate()
            .filter(|(i, _)| !removed.contains(i))
            .filter_map(|(_, v)| v)
            .collect();
        let mut added = self.added.clone();
        added.sort_by_key(|a| a.index);
        for a in added {
            let pos = a.index.min(survivors.len());
            survivors.insert(pos, a.text.clone());
        }
        survivors
    }

    fn between(base: &[String], next: &[String]) -> Self {
        let min_len = base.len().min(next.len());
        let mut modified = Vec::new();
        for i in 0..min_len {
            if base[i] != next[i] { modified.push(LineModified { index: i, text: next[i].clone() }); }
        }
        let removed: Vec<usize> = (next.len()..base.len()).collect();
        let added: Vec<LineAdded> = (base.len()..next.len()).map(|i| LineAdded { index: i, text: next[i].clone() }).collect();
        LinesDiff { removed, modified, added }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Lbl { Base(usize), Added1(usize), Added2(usize) }

fn simulate_labels(labels: Vec<Lbl>, removed: &[usize], added: &[(usize, Lbl)]) -> Vec<Lbl> {
    let removed_set: HashSet<usize> = removed.iter().copied().collect();
    let mut survivors: Vec<Lbl> = labels.into_iter().enumerate()
        .filter(|(i, _)| !removed_set.contains(i))
        .map(|(_, l)| l)
        .collect();
    let mut added_sorted = added.to_vec();
    added_sorted.sort_by_key(|(idx, _)| *idx);
    for (idx, label) in added_sorted {
        let pos = idx.min(survivors.len());
        survivors.insert(pos, label);
    }
    survivors
}

fn absorb_pair(d1: &LinesDiff, d2: &LinesDiff) -> LinesDiff {
    let max_ref1 = d1.removed.iter().copied()
        .chain(d1.modified.iter().map(|m| m.index))
        .chain(d1.added.iter().map(|a| a.index))
        .max();
    let l1 = max_ref1.map(|m| m + 2).unwrap_or(0);

    let base_labels: Vec<Lbl> = (0..l1).map(Lbl::Base).collect();
    let d1_added: Vec<(usize, Lbl)> = d1.added.iter().enumerate().map(|(j, a)| (a.index, Lbl::Added1(j))).collect();
    let mut mid_labels = simulate_labels(base_labels, &d1.removed, &d1_added);

    let mut mid_pos_of_base: HashMap<usize, usize> = HashMap::new();
    let mut mid_pos_of_added1: HashMap<usize, usize> = HashMap::new();
    for (pos, l) in mid_labels.iter().enumerate() {
        match l {
            Lbl::Base(i) => { mid_pos_of_base.insert(*i, pos); }
            Lbl::Added1(j) => { mid_pos_of_added1.insert(*j, pos); }
            Lbl::Added2(_) => {}
        }
    }

    let max_ref2 = d2.removed.iter().copied()
        .chain(d2.modified.iter().map(|m| m.index))
        .chain(d2.added.iter().map(|a| a.index))
        .max();
    let needed_len = max_ref2.map(|m| (m + 2).max(mid_labels.len())).unwrap_or(mid_labels.len());
    while mid_labels.len() < needed_len { mid_labels.push(Lbl::Base(usize::MAX)); }

    let d2_added: Vec<(usize, Lbl)> = d2.added.iter().enumerate().map(|(k, a)| (a.index, Lbl::Added2(k))).collect();
    let after_labels = simulate_labels(mid_labels, &d2.removed, &d2_added);

    let d2_modified_at: HashMap<usize, &str> = d2.modified.iter().map(|m| (m.index, m.text.as_str())).collect();
    let d1_modified_at: HashMap<usize, &str> = d1.modified.iter().map(|m| (m.index, m.text.as_str())).collect();

    let mut present_base: HashSet<usize> = HashSet::new();
    let mut modified = Vec::new();
    let mut added = Vec::new();

    for (pos, l) in after_labels.into_iter().enumerate() {
        match l {
            Lbl::Base(i) if i != usize::MAX => {
                present_base.insert(i);
                let mid_pos = mid_pos_of_base.get(&i).copied();
                let text = mid_pos.and_then(|m| d2_modified_at.get(&m).copied()).or_else(|| d1_modified_at.get(&i).copied());
                if let Some(text) = text { modified.push(LineModified { index: i, text: text.to_string() }); }
            }
            Lbl::Base(_) => {}
            Lbl::Added1(j) => {
                let mid_pos = mid_pos_of_added1.get(&j).copied();
                let base_text = &d1.added[j].text;
                let text = mid_pos.and_then(|m| d2_modified_at.get(&m).copied()).unwrap_or(base_text.as_str());
                added.push(LineAdded { index: pos, text: text.to_string() });
            }
            Lbl::Added2(k) => { added.push(LineAdded { index: pos, text: d2.added[k].text.clone() }); }
        }
    }

    let removed: Vec<usize> = (0..l1).filter(|i| !present_base.contains(i)).collect();
    LinesDiff { removed, modified, added }
}
//#endregion

//#region ByteSplice (copy of the real algorithm)
#[derive(Clone, Debug, PartialEq)]
struct ByteSplice { offset: usize, remove_len: usize, insert: Vec<u8> }

fn splice_apply(splices: &[ByteSplice], base: &[u8]) -> Vec<u8> {
    let mut bytes = base.to_vec();
    let mut splices = splices.to_vec();
    splices.sort_by(|a, b| b.offset.cmp(&a.offset));
    for s in splices {
        let start = s.offset.min(bytes.len());
        let end = (s.offset + s.remove_len).min(bytes.len()).max(start);
        bytes.splice(start..end, s.insert.iter().copied());
    }
    bytes
}

fn splice_between(a: &[u8], b: &[u8]) -> Vec<ByteSplice> {
    let mut prefix = 0usize;
    while prefix < a.len() && prefix < b.len() && a[prefix] == b[prefix] { prefix += 1; }
    let mut suffix = 0usize;
    while suffix < a.len() - prefix && suffix < b.len() - prefix && a[a.len() - 1 - suffix] == b[b.len() - 1 - suffix] { suffix += 1; }
    let remove_len = a.len() - prefix - suffix;
    let insert = b[prefix..b.len() - suffix].to_vec();
    if remove_len == 0 && insert.is_empty() { vec![] } else { vec![ByteSplice { offset: prefix, remove_len, insert }] }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BLbl { Base(usize), New(u8) }

fn bsim(labels: Vec<BLbl>, removed: &[usize], added: &[(usize, BLbl)]) -> Vec<BLbl> {
    let removed_set: HashSet<usize> = removed.iter().copied().collect();
    let mut survivors: Vec<BLbl> = labels.into_iter().enumerate()
        .filter(|(i, _)| !removed_set.contains(i))
        .map(|(_, l)| l)
        .collect();
    let mut added_sorted = added.to_vec();
    added_sorted.sort_by_key(|(idx, _)| *idx);
    for (idx, label) in added_sorted {
        let pos = idx.min(survivors.len());
        survivors.insert(pos, label);
    }
    survivors
}

/// 🗑️ All BASE-relative indices removed by a splice list (order-independent; each splice's
/// range is always relative to the shared base, never to a prior splice's result).
fn splice_removed_indices(splices: &[ByteSplice]) -> Vec<usize> {
    let mut out = Vec::new();
    for s in splices {
        for i in s.offset..(s.offset + s.remove_len) { out.push(i); }
    }
    out
}

/// ➕️ Per-byte insert targets in FINAL-space (i.e. positions in the array that results from
/// applying every splice in this list to the base, per [`splice_apply`]'s own semantics) — NOT
/// the raw `offset` values. Splices are walked ascending by offset, tracking a running `delta`
/// (net length change from every EARLIER splice's `insert.len() - remove_len`) so a later
/// splice's insert lands after everything an earlier splice in the same list already inserted,
/// exactly mirroring `Vec::splice`'s own left-to-right accumulation. Getting this wrong (using
/// bare `offset + k`) silently reorders sibling inserts within one absorbed diff — caught by
/// the `byte_associativity`/`byte_stress_chain_step` scratch checks, see the ticket report.
fn splice_added_targets(splices: &[ByteSplice]) -> Vec<(usize, BLbl)> {
    let mut sorted: Vec<&ByteSplice> = splices.iter().collect();
    sorted.sort_by_key(|s| s.offset);
    let mut out = Vec::new();
    let mut delta: i64 = 0;
    for s in sorted {
        let base_target = (s.offset as i64 + delta).max(0) as usize;
        for (k, byte) in s.insert.iter().enumerate() {
            out.push((base_target + k, BLbl::New(*byte)));
        }
        delta += s.insert.len() as i64 - s.remove_len as i64;
    }
    out
}

fn absorb_splices(d1: &[ByteSplice], d2: &[ByteSplice]) -> Vec<ByteSplice> {
    // 🧭️ `l1` (the virtual base's assumed size) must cover every index EITHER diff references —
    // not just `d1`'s. A `d1` that's empty/a no-op (e.g. two mutually-cancelling ops absorbed
    // already) must not collapse the virtual base to zero elements: `d2`/`d3` in a longer chain
    // may still reference real base positions `d1` never touched. Caught by randomized fuzzing
    // (a `d1` of pure zero-length no-op splices, composed with a real `d3`) — see ticket report.
    let max_ref = d1.iter().chain(d2.iter()).map(|s| s.offset + s.remove_len.max(s.insert.len())).max();
    let l1 = max_ref.map(|m| m + 8).unwrap_or(0);

    let base_labels: Vec<BLbl> = (0..l1).map(BLbl::Base).collect();
    let d1_removed = splice_removed_indices(d1);
    let d1_added = splice_added_targets(d1);
    let mut mid_labels = bsim(base_labels, &d1_removed, &d1_added);

    while mid_labels.len() < l1 { mid_labels.push(BLbl::Base(usize::MAX)); }

    let d2_removed = splice_removed_indices(d2);
    let d2_added = splice_added_targets(d2);
    let after_labels = bsim(mid_labels, &d2_removed, &d2_added);

    let mut splices = Vec::new();
    let mut expected = 0usize;
    let mut pending_insert: Vec<u8> = Vec::new();
    for l in after_labels {
        match l {
            BLbl::Base(i) if i != usize::MAX => {
                if i > expected || !pending_insert.is_empty() {
                    splices.push(ByteSplice { offset: expected, remove_len: i.saturating_sub(expected), insert: std::mem::take(&mut pending_insert) });
                }
                expected = i + 1;
            }
            BLbl::Base(_) => {}
            BLbl::New(byte) => pending_insert.push(byte),
        }
    }
    if expected < l1 || !pending_insert.is_empty() {
        splices.push(ByteSplice { offset: expected, remove_len: l1.saturating_sub(expected), insert: pending_insert });
    }
    splices
}
//#endregion

fn lines(v: &[&str]) -> Vec<String> { v.iter().map(|s| s.to_string()).collect() }

fn main() {
    let mut pass = 0;
    let mut fail = 0;
    macro_rules! check {
        ($name:expr, $cond:expr) => {
            if $cond { pass += 1; } else { fail += 1; println!("FAIL: {}", $name); }
        };
    }

    // ---- line-diff canonical cases ----
    {
        let d1 = LinesDiff { removed: vec![], modified: vec![], added: vec![LineAdded { index: 2, text: "f".into() }] };
        let d2 = LinesDiff { removed: vec![0], modified: vec![], added: vec![] };
        let merged = absorb_pair(&d1, &d2);
        check!("insert_remove_before_removed", merged.removed == vec![0]);
        check!("insert_remove_before_added", merged.added == vec![LineAdded { index: 1, text: "f".into() }]);
        let base = lines(&["a", "b", "c", "d"]);
        let mid = d1.apply(&base);
        let after = d2.apply(&mid);
        check!("insert_remove_before_sequential", LinesDiff { removed: merged.removed.clone(), modified: merged.modified.clone(), added: merged.added.clone() }.apply(&base) == after);
    }
    {
        let d1 = LinesDiff { removed: vec![], modified: vec![], added: vec![LineAdded { index: 2, text: "f".into() }] };
        let d2 = LinesDiff { removed: vec![], modified: vec![], added: vec![LineAdded { index: 2, text: "g".into() }] };
        let merged = absorb_pair(&d1, &d2);
        let base = lines(&["a", "b", "c", "d"]);
        let mid = d1.apply(&base);
        let after = d2.apply(&mid);
        check!("insert_insert_same_index_sequential", merged.apply(&base) == after);
        check!("insert_insert_same_index_both_survive", after.contains(&"f".to_string()) && after.contains(&"g".to_string()));
    }
    {
        let d1 = LinesDiff { removed: vec![], modified: vec![], added: vec![LineAdded { index: 1, text: "f".into() }] };
        let d2 = LinesDiff { removed: vec![], modified: vec![LineModified { index: 1, text: "v".into() }], added: vec![] };
        let merged = absorb_pair(&d1, &d2);
        check!("add_setfield_patches_into_added_modified_empty", merged.modified.is_empty());
        check!("add_setfield_patches_into_added_added", merged.added == vec![LineAdded { index: 1, text: "v".into() }]);
        let base = lines(&["a", "b", "c"]);
        let mid = d1.apply(&base);
        let after = d2.apply(&mid);
        check!("add_setfield_sequential", merged.apply(&base) == after);
    }
    {
        let d1 = LinesDiff { removed: vec![], modified: vec![LineModified { index: 0, text: "m".into() }], added: vec![] };
        let d2 = LinesDiff { removed: vec![0], modified: vec![], added: vec![] };
        let merged = absorb_pair(&d1, &d2);
        check!("modify_remove_drops_modify", merged.modified.is_empty() && merged.removed == vec![0]);
        let base = lines(&["a", "b"]);
        let mid = d1.apply(&base);
        let after = d2.apply(&mid);
        check!("modify_remove_sequential", merged.apply(&base) == after);
    }
    // associativity
    {
        let base = lines(&["a", "b", "c"]);
        let d1 = LinesDiff { removed: vec![1], modified: vec![], added: vec![] };
        let d2 = LinesDiff { removed: vec![], modified: vec![], added: vec![LineAdded { index: 0, text: "x".into() }] };
        let d3 = LinesDiff { removed: vec![], modified: vec![LineModified { index: 0, text: "y".into() }], added: vec![] };
        let left = { let m = absorb_pair(&d1, &d2); absorb_pair(&m, &d3) };
        let right = { let m = absorb_pair(&d2, &d3); absorb_pair(&d1, &m) };
        check!("associativity", left.apply(&base) == right.apply(&base));
        let sequential = { let s1 = d1.apply(&base); let s2 = d2.apply(&s1); d3.apply(&s2) };
        check!("associativity_matches_sequential", left.apply(&base) == sequential);
    }
    // between roundtrip
    {
        let a = lines(&["a", "b", "c"]);
        let b = lines(&["a", "x", "c", "d"]);
        check!("between_roundtrip_ab", LinesDiff::between(&a, &b).apply(&a) == b);
        check!("between_roundtrip_ba", LinesDiff::between(&b, &a).apply(&b) == a);
        check!("between_empty", LinesDiff::between(&a, &a).is_empty());
    }
    // random-ish stress: many small random ops sequences, compare absorb-chain to sequential apply
    {
        let base = lines(&["l0", "l1", "l2", "l3", "l4"]);
        let ops: Vec<LinesDiff> = vec![
            LinesDiff { removed: vec![1], modified: vec![], added: vec![] },
            LinesDiff { removed: vec![], modified: vec![], added: vec![LineAdded { index: 2, text: "n0".into() }] },
            LinesDiff { removed: vec![], modified: vec![LineModified { index: 0, text: "z0".into() }], added: vec![] },
            LinesDiff { removed: vec![0], modified: vec![], added: vec![LineAdded { index: 0, text: "n1".into() }] },
            LinesDiff { removed: vec![], modified: vec![], added: vec![LineAdded { index: 10, text: "tail".into() }] },
        ];
        let mut acc = ops[0].clone();
        let mut seq_state = ops[0].apply(&base);
        for op in &ops[1..] {
            acc = absorb_pair(&acc, op);
            seq_state = op.apply(&seq_state);
            check!("stress_chain_step", acc.apply(&base) == seq_state);
        }
    }

    // ---- byte-splice canonical cases ----
    {
        let d1 = vec![ByteSplice { offset: 2, remove_len: 0, insert: vec![0xAA] }];
        let d2 = vec![ByteSplice { offset: 0, remove_len: 1, insert: vec![] }];
        let merged = absorb_splices(&d1, &d2);
        let base = vec![1u8, 2, 3, 4];
        let mid = splice_apply(&d1, &base);
        let after = splice_apply(&d2, &mid);
        check!("byte_insert_remove_before_sequential", splice_apply(&merged, &base) == after);
    }
    {
        let d1 = vec![ByteSplice { offset: 2, remove_len: 0, insert: vec![0xAA] }];
        let d2 = vec![ByteSplice { offset: 2, remove_len: 0, insert: vec![0xBB] }];
        let merged = absorb_splices(&d1, &d2);
        let base = vec![1u8, 2, 3, 4];
        let mid = splice_apply(&d1, &base);
        let after = splice_apply(&d2, &mid);
        check!("byte_insert_insert_same_offset_sequential", splice_apply(&merged, &base) == after);
        check!("byte_insert_insert_both_survive", after.contains(&0xAA) && after.contains(&0xBB));
    }
    {
        let d1 = vec![ByteSplice { offset: 0, remove_len: 1, insert: vec![0xFF] }];
        let d2 = vec![ByteSplice { offset: 0, remove_len: 1, insert: vec![] }];
        let merged = absorb_splices(&d1, &d2);
        let base = vec![1u8, 2, 3];
        let mid = splice_apply(&d1, &base);
        let after = splice_apply(&d2, &mid);
        check!("byte_modify_remove_sequential", splice_apply(&merged, &base) == after);
    }
    // associativity
    {
        let base = vec![10u8, 20, 30, 40, 50];
        let d1 = vec![ByteSplice { offset: 1, remove_len: 1, insert: vec![] }];
        let d2 = vec![ByteSplice { offset: 0, remove_len: 0, insert: vec![99] }];
        let d3 = vec![ByteSplice { offset: 2, remove_len: 1, insert: vec![7, 8] }];
        let left = { let m = absorb_splices(&d1, &d2); absorb_splices(&m, &d3) };
        let right = { let m = absorb_splices(&d2, &d3); absorb_splices(&d1, &m) };
        check!("byte_associativity", splice_apply(&left, &base) == splice_apply(&right, &base));
        let sequential = { let s1 = splice_apply(&d1, &base); let s2 = splice_apply(&d2, &s1); splice_apply(&d3, &s2) };
        check!("byte_associativity_matches_sequential", splice_apply(&left, &base) == sequential);
    }
    // between roundtrip
    {
        let a = vec![1u8, 2, 3, 4, 5];
        let b = vec![1u8, 9, 9, 4, 5, 6];
        check!("byte_between_roundtrip_ab", splice_apply(&splice_between(&a, &b), &a) == b);
        check!("byte_between_roundtrip_ba", splice_apply(&splice_between(&b, &a), &b) == a);
        check!("byte_between_empty", splice_between(&a, &a).is_empty());
    }
    // stress chain
    {
        let base = vec![0u8, 1, 2, 3, 4, 5, 6, 7];
        let ops: Vec<Vec<ByteSplice>> = vec![
            vec![ByteSplice { offset: 1, remove_len: 2, insert: vec![100] }],
            vec![ByteSplice { offset: 0, remove_len: 0, insert: vec![200, 201] }],
            vec![ByteSplice { offset: 3, remove_len: 1, insert: vec![] }],
            vec![ByteSplice { offset: 5, remove_len: 0, insert: vec![250] }],
        ];
        let mut acc = ops[0].clone();
        let mut seq_state = splice_apply(&ops[0], &base);
        for op in &ops[1..] {
            acc = absorb_splices(&acc, op);
            seq_state = splice_apply(op, &seq_state);
            check!("byte_stress_chain_step", splice_apply(&acc, &base) == seq_state);
        }
    }

    // ---- randomized fuzz: many small random splice-diff triples, absorb-chain vs sequential ----
    // Offsets are always constructed IN-BOUNDS against their own predecessor state's real
    // length -- exactly how every real producer builds a `ByteSplice` (`between`'s prefix/
    // suffix trim, `AppendBytes`'s `base.bytes.len()`, `Splice`'s caller-supplied in-range
    // offset). An offset that's `>=` the real current length is `splice_apply`'s "append"
    // idiom via `min(offset,len)` clamping, and clamp semantics are NOT faithfully replayable
    // by this (deliberately base-free/structural) absorb algorithm across a multi-step chain --
    // a known, documented limitation (see the diff file's doc comment), not exercised by any
    // real mutation-diff construction path, only by adversarial/synthetic out-of-range input.
    {
        let mut rng: u64 = 0x243F6A8885A308D3;
        let mut next = || { rng ^= rng << 13; rng ^= rng >> 7; rng ^= rng << 17; rng };
        for trial in 0..20000u32 {
            let base: Vec<u8> = (0..8).map(|_| (next() % 50) as u8).collect();
            let mut splices_for = |state_len: usize| -> Vec<ByteSplice> {
                let n = 1 + (next() % 2) as usize;
                let mut v = Vec::new();
                let mut cursor = 0usize;
                for _ in 0..n {
                    if cursor >= state_len { break; }
                    let offset = cursor + (next() as usize % (state_len - cursor + 1).max(1));
                    let offset = offset.min(state_len);
                    let remove_len = (next() as usize % 3).min(state_len - offset);
                    let insert_len = (next() % 3) as usize;
                    let insert: Vec<u8> = (0..insert_len).map(|_| (next() % 255) as u8).collect();
                    cursor = offset + remove_len + 1;
                    v.push(ByteSplice { offset, remove_len, insert });
                }
                v
            };
            let d1 = splices_for(base.len());
            let s1 = splice_apply(&d1, &base);
            let d2 = splices_for(s1.len());
            let s2 = splice_apply(&d2, &s1);
            let d3 = splices_for(s2.len());
            let s3 = splice_apply(&d3, &s2);
            let merged12 = absorb_splices(&d1, &d2);
            let merged123 = absorb_splices(&merged12, &d3);
            let ok = splice_apply(&merged123, &base) == s3;
            if !ok {
                println!("trial={trial} base={base:?} d1={d1:?} d2={d2:?} d3={d3:?}");
                println!("got={:?} want={s3:?}", splice_apply(&merged123, &base));
            }
            check!(&format!("fuzz_trial_{trial}"), ok);
        }
    }

    println!("PASS: {pass}  FAIL: {fail}");
    if fail > 0 { std::process::exit(1); }
}
