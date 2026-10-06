#!/usr/bin/env python3
"""🔢️ S5-STORE wave RN (found by `viewer_head_tests::the_viewer_head_corpus_matches_two_stores`, 2026-10-05): a remote edit's
merge renumbers every applied edit by its applied position, but told the revision accumulator only "dirty from the insertion
point `k`". An applied edit BEFORE `k` whose number changed — one that kept a higher number because a hidden edit (another
line's, or one past an explicit checkpoint) had briefly been ordered before it — kept a stale revision record: the
incremental content revision no longer equalled a rebuilt one. The renumbering now reports the first position it changed
and the accumulator is dirty from there; it also walks the ledger once instead of searching it once per applied edit.

One edit keyed on an anchor that must occur exactly once. Idempotent. `--check` writes nothing.

    python3 🧪️s5-store-ingest-renumber-dirt.py [--check]
"""
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
STORE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
OLD = '''        for (index, edit_id) in self.applied_edit_ids.iter().enumerate() {
            if let Some(edit) = self.envelope.vcs.edits.iter_mut().find(|edit| edit.id == *edit_id) {
                edit.sequence_number = index as i32 + 1;
            }
        }
        self.edit_sequence = self.applied_edit_ids.len() as i32;
        self.replace_tail_undo_cache_retained(None)?;
        self.replace_current_retained(state)?;
        self.revision_dirty_from = Some(self.revision_dirty_from.map_or(k, |dirty| dirty.min(k)));
'''
NEW = '''        let positions: HashMap<&str, usize> = self.applied_edit_ids.iter().enumerate().map(|(index, edit_id)| (edit_id.as_str(), index)).collect();
        let mut renumbered = k;
        for edit in self.envelope.vcs.edits.iter_mut() {
            if let Some(index) = positions.get(edit.id.as_str()).copied().filter(|index| edit.sequence_number != *index as i32 + 1) {
                edit.sequence_number = index as i32 + 1;
                renumbered = renumbered.min(index);
            }
        }
        drop(positions);
        self.edit_sequence = self.applied_edit_ids.len() as i32;
        self.replace_tail_undo_cache_retained(None)?;
        self.replace_current_retained(state)?;
        self.revision_dirty_from = Some(self.revision_dirty_from.map_or(renumbered, |dirty| dirty.min(renumbered)));
'''


def main():
    text = STORE.read_text(encoding="utf-8")
    if NEW in text:
        print("pending: none")
        return
    if text.count(OLD) != 1:
        raise SystemExit(f"anchor occurs {text.count(OLD)} times (expected 1): re-derive the wave")
    print("pending: store:renumber")
    if "--check" in sys.argv[1:]:
        return
    STORE.write_text(text.replace(OLD, NEW), encoding="utf-8")
    print("applied")


main()
