#!/usr/bin/env python3
"""🔗️ Guest/store half of the opaque-concurrency patch (ticket 26/09/23 H9 session 12, coordinator item (a)).

A mutation authored without declared dependencies is stamped, AT AUTHORING TIME, with the store's causal head: the
mutation the store last applied (its own or one ingested from the hub) — the base the author actually saw. Each next
mutation of the same batch depends on its predecessor. Stamping at send time instead would hide exactly C10's case (an
edit authored during a cut would claim the post-reconnect head). The relays (Rust `🔄️sync` actors, the TS worker) pass
dependencies through unchanged already. Lands TOGETHER with `db-opaque-concurrency.py` (guest-linked: rule 20).

usage: store-causal-dependencies.py [--reverse] [--dry-run]
"""
import pathlib
import sys

ROOT = next(parent for parent in pathlib.Path(__file__).resolve().parents if (parent / ".git").exists())
STORE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"

EDITS = [
    (STORE,
     """    /// 🎥️ Tail uncommitted document edit that a coalesced batch may absorb into, matching `amend_command`.
""",
     """    /// @emoji 🔗️ The mutation this store applied last — its own or one ingested from the hub —, the base a mutation
    /// authored now has seen; `None` for a document with no applied edit. Stamped as the dependency of every
    /// mutation that declares none, so the hub can tell an edit that saw the document's head from one authored
    /// concurrently (ticket 26/09/23 C10: a same-field edit authored during a 12 s cut carried `dependencies: []`).
    fn causal_head(&self) -> Option<MutationId> {
        let last = self.applied_edit_ids.last()?;
        let edit = self.envelope.vcs.edits.iter().find(|edit| edit.id == *last)?;
        edit.mutation_meta.iter().rev().find_map(|meta| meta.mutation_id.clone())
    }

    /// 🎥️ Tail uncommitted document edit that a coalesced batch may absorb into, matching `amend_command`.
"""),
    (STORE,
     """        let mut candidate_clock = self.clock;
""",
     """        let mut candidate_clock = self.clock;
        let mut causal_head = self.causal_head();
"""),
    (STORE,
     """            mutation_meta.push(MutationMeta {
                mutation_id: Some(mutation_id),
                dependencies: mutation.dependencies(),
""",
     """            let declared = mutation.dependencies();
            let dependencies = if declared.is_empty() { causal_head.iter().cloned().collect() } else { declared };
            causal_head = Some(mutation_id.clone());
            mutation_meta.push(MutationMeta {
                mutation_id: Some(mutation_id),
                dependencies,
"""),
]


def main() -> int:
    reverse = "--reverse" in sys.argv
    dry = "--dry-run" in sys.argv
    texts = {path: path.read_text(encoding="utf-8") for path in {edit[0] for edit in EDITS}}
    failures = 0
    for index, (path, before, after) in enumerate(EDITS):
        find, put = (after, before) if reverse else (before, after)
        count = texts[path].count(find)
        if count != 1:
            print(f"edit {index} {path.name}: anchor found {count} times", file=sys.stderr)
            failures += 1
            continue
        texts[path] = texts[path].replace(find, put, 1)
        print(f"edit {index} {path.parent.name}/{path.name}: ok")
    if failures:
        return 1
    if not dry:
        for path, text in texts.items():
            path.write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    sys.exit(main())
