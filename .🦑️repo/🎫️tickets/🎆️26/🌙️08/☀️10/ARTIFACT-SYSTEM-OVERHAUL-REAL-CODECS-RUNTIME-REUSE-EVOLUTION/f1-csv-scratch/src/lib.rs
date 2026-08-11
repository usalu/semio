//! Scratch crate: verifies the CSV diff/absorb/mutation algorithm in isolation (no
//! protocol/schema/serde infra), mirroring the real files at
//! ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/{📸️snapshot,🔺️diff,🧬️mutations}/🦀️component.rs

use std::collections::{BTreeMap, HashMap};

//#region Snapshot
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CsvField { pub value: String, pub quoted: bool }
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CsvRecord { pub fields: Vec<CsvField> }
#[derive(Clone, Debug, PartialEq)]
pub struct CsvSnapshot { pub has_header: bool, pub records: Vec<CsvRecord> }
//#endregion

//#region FieldDiff
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CsvFieldDiff { pub value: Option<String>, pub quoted: Option<bool> }
impl CsvFieldDiff {
    pub fn is_empty(&self) -> bool { self.value.is_none() && self.quoted.is_none() }
    pub fn apply(&self, base: &CsvField) -> CsvField {
        CsvField { value: self.value.clone().unwrap_or_else(|| base.value.clone()), quoted: self.quoted.unwrap_or(base.quoted) }
    }
    pub fn between(base: &CsvField, other: &CsvField) -> Self {
        Self { value: (base.value != other.value).then(|| other.value.clone()), quoted: (base.quoted != other.quoted).then_some(other.quoted) }
    }
    fn absorb(&mut self, other: Self) {
        if other.value.is_some() { self.value = other.value; }
        if other.quoted.is_some() { self.quoted = other.quoted; }
    }
}
//#endregion

//#region RecordDiff
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CsvRecordDiff { pub fields: Option<Vec<Option<CsvFieldDiff>>> }
impl CsvRecordDiff {
    pub fn is_empty(&self) -> bool {
        match &self.fields { None => true, Some(v) => v.iter().all(|f| f.is_none()) }
    }
    pub fn apply(&self, base: &CsvRecord) -> CsvRecord {
        match &self.fields {
            None => base.clone(),
            Some(patches) => {
                let mut fields = base.fields.clone();
                for (i, patch) in patches.iter().enumerate() {
                    if let Some(p) = patch { if let Some(f) = fields.get_mut(i) { *f = p.apply(f); } }
                }
                CsvRecord { fields }
            }
        }
    }
    pub fn between(base: &CsvRecord, other: &CsvRecord) -> Self {
        debug_assert_eq!(base.fields.len(), other.fields.len());
        let mut any = false;
        let patches: Vec<Option<CsvFieldDiff>> = base.fields.iter().zip(other.fields.iter()).map(|(b, o)| {
            let d = CsvFieldDiff::between(b, o);
            if d.is_empty() { None } else { any = true; Some(d) }
        }).collect();
        Self { fields: if any { Some(patches) } else { None } }
    }
    fn absorb(&mut self, other: Self) {
        match (&mut self.fields, other.fields) {
            (_, None) => {}
            (slot @ None, Some(f2)) => *slot = Some(f2),
            (Some(f1), Some(f2)) => {
                if f2.len() > f1.len() { f1.resize(f2.len(), None); }
                for (i, patch2) in f2.into_iter().enumerate() {
                    if let Some(p2) = patch2 {
                        match &mut f1[i] { Some(p1) => p1.absorb(p2), slot @ None => *slot = Some(p2) }
                    }
                }
            }
        }
    }
}
//#endregion

//#region RecordsDiff
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CsvRecordModified { pub index: usize, pub diff: CsvRecordDiff }
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CsvRecordAdded { pub index: usize, pub record: CsvRecord }
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CsvRecordsDiff { pub removed: Vec<usize>, pub modified: Vec<CsvRecordModified>, pub added: Vec<CsvRecordAdded> }
impl CsvRecordsDiff {
    pub fn is_empty(&self) -> bool { self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty() }
}
//#endregion

//#region IndexTransport
#[derive(Clone, Copy, Debug)]
enum Slot { Base(usize), Added(usize) }
fn simulate_slots(len: usize, removed: &[usize], added_indices: &[usize]) -> Vec<Slot> {
    let mut slots: Vec<Slot> = (0..len).map(Slot::Base).collect();
    let mut removed_desc = removed.to_vec();
    removed_desc.sort_unstable_by(|a, b| b.cmp(a));
    removed_desc.dedup();
    for r in removed_desc { if r < slots.len() { slots.remove(r); } }
    let mut order: Vec<usize> = (0..added_indices.len()).collect();
    order.sort_by_key(|&i| added_indices[i]);
    for i in order { let at = added_indices[i].min(slots.len()); slots.insert(at, Slot::Added(i)); }
    slots
}
fn base_len_hint(removed: &[usize], modified_indices: impl Iterator<Item = usize>, added_indices: impl Iterator<Item = usize>) -> usize {
    removed.iter().copied().chain(modified_indices).chain(added_indices).max().map(|m| m + 1).unwrap_or(0)
}
//#endregion

//#region Diff
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CsvDiff { pub has_header: Option<bool>, pub records: Option<CsvRecordsDiff> }

impl CsvDiff {
    pub fn apply(&self, base: &CsvSnapshot) -> CsvSnapshot {
        let mut next = base.clone();
        if let Some(has_header) = self.has_header { next.has_header = has_header; }
        if let Some(rdiff) = &self.records {
            for m in &rdiff.modified {
                if let Some(rec) = next.records.get_mut(m.index) { *rec = m.diff.apply(rec); }
            }
            let mut removed_desc = rdiff.removed.clone();
            removed_desc.sort_unstable_by(|a, b| b.cmp(a));
            removed_desc.dedup();
            for idx in removed_desc { if idx < next.records.len() { next.records.remove(idx); } }
            let mut added_asc = rdiff.added.clone();
            added_asc.sort_by_key(|a| a.index);
            for a in added_asc { let at = a.index.min(next.records.len()); next.records.insert(at, a.record); }
        }
        next
    }

    pub fn absorb(&mut self, other: Self) {
        if other.has_header.is_some() { self.has_header = other.has_header; }
        let d2 = match other.records { None => return, Some(d2) => d2 };
        let d1 = match self.records.take() { None => { self.records = Some(d2); return; } Some(d1) => d1 };
        self.records = Some(absorb_records(d1, d2));
    }

    pub fn inverse(&self, base: &CsvSnapshot) -> Self {
        let applied = self.apply(base);
        Self::between(&applied, base)
    }

    pub fn between(base: &CsvSnapshot, other: &CsvSnapshot) -> Self {
        let has_header = (base.has_header != other.has_header).then_some(other.has_header);
        let mut removed = Vec::new();
        let mut modified = Vec::new();
        let mut added = Vec::new();
        let min_len = base.records.len().min(other.records.len());
        for i in 0..min_len {
            let b = &base.records[i];
            let o = &other.records[i];
            if b == o { continue; }
            if b.fields.len() == o.fields.len() {
                let d = CsvRecordDiff::between(b, o);
                if !d.is_empty() { modified.push(CsvRecordModified { index: i, diff: d }); }
            } else {
                removed.push(i);
                added.push(CsvRecordAdded { index: i, record: o.clone() });
            }
        }
        for i in min_len..base.records.len() { removed.push(i); }
        for i in min_len..other.records.len() { added.push(CsvRecordAdded { index: i, record: other.records[i].clone() }); }
        let records = if removed.is_empty() && modified.is_empty() && added.is_empty() { None } else { Some(CsvRecordsDiff { removed, modified, added }) };
        Self { has_header, records }
    }

    pub fn is_empty(&self) -> bool {
        self.has_header.is_none() && self.records.as_ref().map_or(true, CsvRecordsDiff::is_empty)
    }
}

fn absorb_records(d1: CsvRecordsDiff, d2: CsvRecordsDiff) -> CsvRecordsDiff {
    let d1_added_indices: Vec<usize> = d1.added.iter().map(|a| a.index).collect();
    // 📏 The tight bound from d1's OWN references isn't always enough: d2's removed/modified
    // may query mid positions d1 never touched (e.g. d1 = a single InsertRecord with no
    // removed/modified at all, d2 = RemoveRecord at a position past it) — widen base_len so
    // the simulated mid array is long enough to answer those queries too.
    let removed_count = { let mut r = d1.removed.clone(); r.sort_unstable(); r.dedup(); r.len() };
    let needed_mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max().map(|m| m + 1).unwrap_or(0);
    let base_len = base_len_hint(&d1.removed, d1.modified.iter().map(|m| m.index), d1_added_indices.iter().copied())
        .max((needed_mid_len + removed_count).saturating_sub(d1.added.len()));
    let mid_slots = simulate_slots(base_len, &d1.removed, &d1_added_indices);
    let mut base_to_mid: HashMap<usize, usize> = HashMap::new();
    for (pos, slot) in mid_slots.iter().enumerate() { if let Slot::Base(b) = slot { base_to_mid.insert(*b, pos); } }
    let _ = &base_to_mid; // (kept for symmetry with the real file; not needed by this direction)

    let mut final_removed: Vec<usize> = d1.removed.clone();
    let mut modified_map: BTreeMap<usize, CsvRecordDiff> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
    let mut added_alive: Vec<Option<CsvRecordAdded>> = d1.added.into_iter().map(Some).collect();

    for mid_idx in &d2.removed {
        match mid_slots.get(*mid_idx) {
            Some(Slot::Base(b)) => { final_removed.push(*b); modified_map.remove(b); }
            Some(Slot::Added(ai)) => { added_alive[*ai] = None; }
            None => {}
        }
    }
    for m2 in &d2.modified {
        match mid_slots.get(m2.index) {
            Some(Slot::Base(b)) => { modified_map.entry(*b).or_default().absorb(m2.diff.clone()); }
            Some(Slot::Added(ai)) => { if let Some(added) = added_alive[*ai].as_mut() { added.record = m2.diff.apply(&added.record); } }
            None => {}
        }
    }

    final_removed.sort_unstable();
    final_removed.dedup();
    for r in &final_removed { modified_map.remove(r); }
    let mut final_modified: Vec<CsvRecordModified> = modified_map.into_iter().filter(|(_, d)| !d.is_empty()).map(|(index, diff)| CsvRecordModified { index, diff }).collect();
    final_modified.sort_by_key(|m| m.index);

    let alive_mid_positions: Vec<usize> = mid_slots.iter().enumerate().filter_map(|(pos, slot)| match slot { Slot::Added(ai) if added_alive[*ai].is_some() => Some(pos), _ => None }).collect();
    let d2_added_indices: Vec<usize> = d2.added.iter().map(|a| a.index).collect();
    let mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).chain(alive_mid_positions.iter().copied()).chain(d2_added_indices.iter().copied()).max().map(|m| m + 1).unwrap_or(0);
    let after_slots = simulate_slots(mid_len, &d2.removed, &d2_added_indices);
    let mut mid_to_after: HashMap<usize, usize> = HashMap::new();
    for (pos, slot) in after_slots.iter().enumerate() { if let Slot::Base(m) = slot { mid_to_after.insert(*m, pos); } }

    let mut final_added: Vec<CsvRecordAdded> = Vec::new();
    for (ai, alive) in added_alive.into_iter().enumerate() {
        if let Some(added) = alive {
            let mid_pos = mid_slots.iter().position(|s| matches!(s, Slot::Added(idx) if *idx == ai)).expect("added_alive index always has a corresponding mid slot");
            if let Some(after_pos) = mid_to_after.get(&mid_pos) { final_added.push(CsvRecordAdded { index: *after_pos, record: added.record }); }
        }
    }
    for a2 in d2.added { final_added.push(a2); }
    final_added.sort_by_key(|a| a.index);

    CsvRecordsDiff { removed: final_removed, modified: final_modified, added: final_added }
}
//#endregion

//#region Mutations
#[derive(Clone, Debug, PartialEq)]
pub enum CsvMutation {
    SetHasHeader { has_header: bool },
    InsertRecord { index: usize, record: CsvRecord },
    RemoveRecord { index: usize },
    SetField { record_index: usize, field_index: usize, value: String, quoted: bool },
}
impl CsvMutation {
    pub fn diff(&self, base: &CsvSnapshot) -> CsvDiff {
        match self {
            CsvMutation::SetHasHeader { has_header } => CsvDiff { has_header: Some(*has_header), records: None },
            CsvMutation::InsertRecord { index, record } => CsvDiff { has_header: None, records: Some(CsvRecordsDiff { removed: vec![], modified: vec![], added: vec![CsvRecordAdded { index: *index, record: record.clone() }] }) },
            CsvMutation::RemoveRecord { index } => CsvDiff { has_header: None, records: Some(CsvRecordsDiff { removed: vec![*index], modified: vec![], added: vec![] }) },
            CsvMutation::SetField { record_index, field_index, value, quoted } => {
                let mut fields = vec![None; field_index + 1];
                fields[*field_index] = Some(CsvFieldDiff { value: Some(value.clone()), quoted: Some(*quoted) });
                CsvDiff { has_header: None, records: Some(CsvRecordsDiff { removed: vec![], modified: vec![CsvRecordModified { index: *record_index, diff: CsvRecordDiff { fields: Some(fields) } }], added: vec![] }) }
            }
        }
    }
    pub fn inverse(&self, base: &CsvSnapshot) -> Self {
        match self {
            CsvMutation::SetHasHeader { .. } => CsvMutation::SetHasHeader { has_header: base.has_header },
            CsvMutation::InsertRecord { index, .. } => CsvMutation::RemoveRecord { index: *index },
            CsvMutation::RemoveRecord { index } => CsvMutation::InsertRecord { index: *index, record: base.records[*index].clone() },
            CsvMutation::SetField { record_index, field_index, .. } => {
                let f = &base.records[*record_index].fields[*field_index];
                CsvMutation::SetField { record_index: *record_index, field_index: *field_index, value: f.value.clone(), quoted: f.quoted }
            }
        }
    }
}
//#endregion

#[cfg(test)]
mod tests {
    use super::*;

    fn field(value: &str, quoted: bool) -> CsvField { CsvField { value: value.into(), quoted } }
    fn record(fields: &[(&str, bool)]) -> CsvRecord { CsvRecord { fields: fields.iter().map(|(v, q)| field(v, *q)).collect() } }
    fn base_snapshot() -> CsvSnapshot {
        CsvSnapshot { has_header: true, records: vec![record(&[("name", false), ("note", true)]), record(&[("a", false), ("b", false)]), record(&[("x", false), ("y", false)])] }
    }
    fn sweep_a() -> CsvSnapshot {
        CsvSnapshot { has_header: true, records: vec![record(&[("gone", false), ("also-gone", true)]), record(&[("old-a", false), ("old-b", true)]), record(&[("stable", false)])] }
    }
    fn sweep_b() -> CsvSnapshot {
        CsvSnapshot { has_header: false, records: vec![record(&[("new-a", true), ("new-b", false)]), record(&[("stable", false)]), record(&[("brand-new", true)])] }
    }

    #[test]
    fn mutation_diff_law() {
        let base = base_snapshot();
        let variants = vec![
            CsvMutation::SetHasHeader { has_header: false },
            CsvMutation::InsertRecord { index: 1, record: record(&[("new", true)]) },
            CsvMutation::RemoveRecord { index: 0 },
            CsvMutation::SetField { record_index: 1, field_index: 0, value: "changed".into(), quoted: true },
        ];
        for m in variants {
            let diff = m.diff(&base);
            let expected = diff.apply(&base);
            let d2 = m.diff(&base);
            assert_eq!(d2, diff);
            assert_eq!(diff.apply(&base), expected);
        }
    }

    #[test]
    fn inverse_law() {
        let base = base_snapshot();
        let variants = vec![
            CsvMutation::SetHasHeader { has_header: false },
            CsvMutation::InsertRecord { index: 1, record: record(&[("new", true)]) },
            CsvMutation::RemoveRecord { index: 0 },
            CsvMutation::SetField { record_index: 1, field_index: 0, value: "changed".into(), quoted: true },
        ];
        for m in variants {
            let d = m.diff(&base);
            let mid = d.apply(&base);
            let back = d.inverse(&base).apply(&mid);
            assert_eq!(back, base, "diff-level inverse failed for {m:?}");

            let inv = m.inverse(&base);
            let forward = m.diff(&base).apply(&base);
            let round = inv.diff(&forward).apply(&forward);
            assert_eq!(round, base, "mutation-level inverse failed for {m:?}");
        }
    }

    #[test]
    fn absorb_law_insert_remove_before() {
        let base = base_snapshot();
        let d1 = CsvMutation::InsertRecord { index: 2, record: record(&[("ins", false)]) }.diff(&base);
        let mid = d1.apply(&base);
        let d2 = CsvMutation::RemoveRecord { index: 0 }.diff(&mid);
        let after = d2.apply(&mid);
        let mut composed = d1.clone();
        composed.absorb(d2.clone());
        assert_eq!(composed.apply(&base), after);
    }

    #[test]
    fn absorb_law_insert_insert_same_index_both_survive() {
        let base = base_snapshot();
        let d1 = CsvMutation::InsertRecord { index: 2, record: record(&[("f", false)]) }.diff(&base);
        let mid = d1.apply(&base);
        let d2 = CsvMutation::InsertRecord { index: 2, record: record(&[("g", false)]) }.diff(&mid);
        let after = d2.apply(&mid);
        let mut composed = d1.clone();
        composed.absorb(d2.clone());
        assert_eq!(composed.apply(&base), after);
        assert_eq!(after.records.len(), base.records.len() + 2);
    }

    #[test]
    fn absorb_law_add_setfield_patches_into_added() {
        let base = base_snapshot();
        let d1 = CsvMutation::InsertRecord { index: 1, record: record(&[("orig", false)]) }.diff(&base);
        let mid = d1.apply(&base);
        let d2 = CsvMutation::SetField { record_index: 1, field_index: 0, value: "patched".into(), quoted: true }.diff(&mid);
        let after = d2.apply(&mid);
        let mut composed = d1.clone();
        composed.absorb(d2.clone());
        assert_eq!(composed.apply(&base), after);
        assert_eq!(after.records[1].fields[0].value, "patched");
    }

    #[test]
    fn absorb_law_modify_remove_collapses() {
        let base = base_snapshot();
        let d1 = CsvMutation::SetField { record_index: 1, field_index: 0, value: "will-vanish".into(), quoted: false }.diff(&base);
        let mid = d1.apply(&base);
        let d2 = CsvMutation::RemoveRecord { index: 1 }.diff(&mid);
        let after = d2.apply(&mid);
        let mut composed = d1.clone();
        composed.absorb(d2.clone());
        assert_eq!(composed.apply(&base), after);
    }

    #[test]
    fn absorb_law_associative_triple() {
        let base = base_snapshot();
        let d1 = CsvMutation::InsertRecord { index: 0, record: record(&[("a", false)]) }.diff(&base);
        let s1 = d1.apply(&base);
        let d2 = CsvMutation::SetField { record_index: 0, field_index: 0, value: "a2".into(), quoted: true }.diff(&s1);
        let s2 = d2.apply(&s1);
        let d3 = CsvMutation::RemoveRecord { index: 2 }.diff(&s2);
        let s3 = d3.apply(&s2);

        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());

        let mut d23 = d2.clone();
        d23.absorb(d3.clone());
        let mut right = d1.clone();
        right.absorb(d23);

        assert_eq!(left.apply(&base), s3);
        assert_eq!(right.apply(&base), s3);
        assert_eq!(left.apply(&base), right.apply(&base));
    }

    #[test]
    fn between_roundtrip_law() {
        let a = base_snapshot();
        let b = sweep_b();
        assert_eq!(CsvDiff::between(&a, &b).apply(&a), b);
        assert_eq!(CsvDiff::between(&b, &a).apply(&b), a);

        let mut c = a.clone();
        c.records[0] = record(&[("only-one-field", false)]);
        assert_eq!(CsvDiff::between(&a, &c).apply(&a), c);
        assert_eq!(CsvDiff::between(&c, &a).apply(&c), a);

        assert!(CsvDiff::between(&a, &a).is_empty());
    }

    #[test]
    fn field_sweep_every_mutable_field_changes() {
        let a = sweep_a();
        let b = sweep_b();
        let d_ab = CsvDiff::between(&a, &b);
        assert_eq!(d_ab.apply(&a), b);
        let d_ba = CsvDiff::between(&b, &a);
        assert_eq!(d_ba.apply(&b), a);

        assert!(d_ab.has_header.is_some());
        let records = d_ab.records.as_ref().unwrap();
        assert!(!records.removed.is_empty());
        assert!(!records.modified.is_empty());
        assert!(!records.added.is_empty());
        let modified = &records.modified[0];
        let field_patches = modified.diff.fields.as_ref().unwrap();
        assert!(field_patches.iter().all(|f| f.is_some()));
        for patch in field_patches.iter().flatten() {
            assert!(patch.value.is_some() && patch.quoted.is_some());
        }
        assert!(CsvDiff::between(&a, &a).is_empty());
    }
}
