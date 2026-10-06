#!/usr/bin/env python3
"""🧭️ S5-STORE wave RB (live fault F4 chain, probe batch H): a finished history-edit replay is stale when the store's CONTENT
moved — its content revision — not when its generation moved. `attach_hot_backbone` / `attach_backbone` / `detach_backbone`
bump the generation (the detach refusal contract of `🧫️backbone-detach` pins that), but a port rebinding changes no event,
so it must not cost an open history edit its replay.

One edit keyed on an anchor that must occur exactly once. Idempotent. `--check` writes nothing.

    python3 🧪️s5-store-content-staleness.py [--check]
"""
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
STORE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
EDITS = [
    (
        "check",
        "        if finished.generation != self.generation || finished.revision != self.content_revision {\n            return Err(VcsError::Stale { expected_generation: finished.generation, generation: self.generation });\n",
        "        if finished.revision != self.content_revision {\n            return Err(VcsError::Stale { expected_generation: finished.generation, generation: self.generation });\n",
    ),
    (
        "doc",
        "    /// session's drafts ([`Self::begin_report_replay`]). Refused with `Stale` when this store moved since the replay began\n    /// (the session replays again) and with",
        "    /// session's drafts ([`Self::begin_report_replay`]). Refused with `Stale` when this store's content moved since the replay\n    /// began (its content revision differs; the session replays again — a port rebinding moves no event and keeps the replay) and with",
    ),
]


def main():
    text = STORE.read_text(encoding="utf-8")
    pending = []
    for name, old, new in EDITS:
        if new in text:
            continue
        if text.count(old) != 1:
            raise SystemExit(f"anchor `{name}` occurs {text.count(old)} times (expected 1): re-derive the wave")
        text = text.replace(old, new)
        pending.append(name)
    print("pending: " + (", ".join(pending) if pending else "none"))
    if "--check" in sys.argv[1:] or not pending:
        return
    STORE.write_text(text, encoding="utf-8")
    print("applied")


main()
