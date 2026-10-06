#!/usr/bin/env python3
"""🖋️ S5-STORE wave P3 (design §22.6): every edit names its author. `edit_is_local` no longer counts an unauthored edit as
local, the durable-history validation every persistence boundary runs refuses an edit without an author, and so does the paged
`.spr` edit decoder (`artifact-spr.edit-actor-missing`). Local edits are always authored (`edit_actor_from_meta`, the batch
authority's actor) and a received edit carries its envelope's actor, so only a persisted or hand-built log can be unauthored.

Every edit is keyed on an anchor that must occur exactly once; the file is written only when every anchor resolved.
Idempotent. `--check` prints what is pending and writes nothing; `--emit <dir>` writes the edited file into `<dir>`.

    python3 🧪️s5-store-authored-edits.py [--check | --emit <dir>]
"""
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
STORE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"

EDITS = [
    (
        "local",
        "    /// 🖋️ Whether `edit_id` was authored by the local actor. Unauthored (legacy) edits count\n"
        "    /// as local; every other actor is foreign and must not be undone by this store.\n"
        "    fn edit_is_local(&self, edit_id: &str) -> bool {\n"
        "        self.envelope.vcs.edits.find_near(|edit| edit.id == edit_id).is_some_and(|edit| edit.actor.is_none() || edit.actor.as_deref() == self.local_actor_id.as_deref())\n"
        "    }\n",
        "    /// 🖋️ Whether `edit_id` was authored by the local actor; every other actor is foreign and must not be undone by this\n"
        "    /// store. Every edit names its author ([`validate_durable_history`] admits no other).\n"
        "    fn edit_is_local(&self, edit_id: &str) -> bool {\n"
        "        self.envelope.vcs.edits.find_near(|edit| edit.id == edit_id).is_some_and(|edit| edit.actor.is_some() && edit.actor.as_deref() == self.local_actor_id.as_deref())\n"
        "    }\n",
        "is_some_and(|edit| edit.actor.is_some() && edit.actor.as_deref() == self.local_actor_id.as_deref())",
    ),
    (
        "admission",
        "        if edit.sequence_number < 0 || !edit_sequences.insert(edit.sequence_number) {\n"
        '            return Err(VcsError::ValidationFailed(format!("history has an invalid edit sequence {} for {}", edit.sequence_number, edit.id)));\n'
        "        }\n",
        "        if edit.sequence_number < 0 || !edit_sequences.insert(edit.sequence_number) {\n"
        '            return Err(VcsError::ValidationFailed(format!("history has an invalid edit sequence {} for {}", edit.sequence_number, edit.id)));\n'
        "        }\n"
        "        if edit.actor.as_deref().is_none_or(str::is_empty) {\n"
        '            return Err(VcsError::ValidationFailed(format!("history edit {} names no author", edit.id)));\n'
        "        }\n",
        '"history edit {} names no author"',
    ),
    (
        "decoder-read",
        '        let id = self.strings[0].take().ok_or_else(|| self.diagnostic("artifact-spr.edit-id-missing", 0))?;\n',
        '        let id = self.strings[0].take().ok_or_else(|| self.diagnostic("artifact-spr.edit-id-missing", 0))?;\n'
        '        let actor = self.strings[1].take().ok_or_else(|| self.diagnostic("artifact-spr.edit-actor-missing", 0))?;\n',
        '"artifact-spr.edit-actor-missing"',
    ),
    (
        "decoder-write",
        "            actor: self.strings[1].take(),\n            line: self.strings[6].take(),\n",
        "            actor: Some(actor),\n            line: self.strings[6].take(),\n",
        "            actor: Some(actor),\n            line: self.strings[6].take(),\n",
    ),
]


def main():
    text = STORE.read_text(encoding="utf-8")
    pending = []
    for name, old, new, marker in EDITS:
        if marker in text:
            continue
        if text.count(old) != 1:
            raise SystemExit(f"anchor `{name}` occurs {text.count(old)} times (expected 1): re-derive the wave")
        text = text.replace(old, new)
        pending.append(name)
    print("pending: " + (", ".join(pending) if pending else "none"))
    if "--emit" in sys.argv[1:]:
        target = Path(sys.argv[sys.argv.index("--emit") + 1])
        target.mkdir(parents=True, exist_ok=True)
        (target / "store.rs").write_text(text, encoding="utf-8")
        print(f"emitted to {target}")
        return
    if "--check" in sys.argv[1:] or not pending:
        return
    STORE.write_text(text, encoding="utf-8")
    print("applied")


main()
