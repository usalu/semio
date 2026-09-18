#!/usr/bin/env python3
"""🐍 `s.wfc.grid2d` oracle replay — the second implementation the `🥒️.feature` scenarios are
measured against. It reads ONLY the committed fixture quintets and this subset's own reference
semantics (`🐍️grid2d-oracle.py` in the ticket folder, vendored here as `reference`), never the Rust
crate: two implementations that agree are evidence, one implementation echoing itself is not."""
from __future__ import annotations

import json
import pathlib
import sys

FIXTURES = pathlib.Path(__file__).resolve().parents[2] / "🧫️fixtures/🧬️mutations"

VECTORS = [
    ("change-seed", "🎲️change-seed", "🎲️reseeds-the-solve-from-7-to-99"),
    ("resize-grid", "📐️resize-grid", "📐️shrinks-the-board-and-drops-the-outside-cells"),
    ("change-cell-size", "📏️change-cell-size", "📏️widens-every-cell"),
    ("change-periodicity", "🔁️change-periodicity", "🔁️wraps-the-x-axis"),
    ("create-tile", "🌱️create-tile", "🌱️inserts-the-corner-tile-in-sorted-order"),
    ("delete-tile", "🗑️delete-tile", "🗑️removes-the-straight-tile-and-cascades-its-rule-and-pin"),
    ("change-tile-weight", "⚖️change-tile-weight", "⚖️biases-the-solve-towards-empty"),
    ("change-tile-media", "🎨️change-tile-media", "🎨️redraws-the-empty-tile-as-a-bitmap"),
    ("create-rule", "🚦️create-rule", "🚦️lets-two-straights-stack-vertically"),
    ("delete-rule", "❌delete-rule", "❌️forbids-the-straight-pair-again"),
    ("pin-cell", "📌️pin-cell", "📌️fixes-the-right-cell-to-the-straight-tile"),
    ("unpin-cell", "📍️unpin-cell", "📍️releases-the-pinned-straight-cell"),
    ("mask-cell", "🕳️mask-cell", "🕳️cuts-the-pinned-corner-out-of-the-problem"),
    ("unmask-cell", "🔳️unmask-cell", "🔳️puts-the-hole-back-into-the-problem"),

]


def load(root: pathlib.Path, *parts: str) -> object:
    return json.loads((root.joinpath(*parts)).read_text(encoding="utf-8"))


def replay() -> int:
    failures = 0
    for kind, directory, case in VECTORS:
        root = FIXTURES / directory / case
        before = load(root, "📸️snapshot/⬅️before/🔣️.json")
        after = load(root, "📸️snapshot/➡️after/🔣️.json")
        delta = load(root, "🔺️diff/🔣️.json")
        outcome = load(root, "🎯️outcome/🔣️.json")
        mutation = load(root, "🦠️mutation/🔣️.json")
        produced = reference.apply(before, delta)
        if produced != after:
            print(f"FAIL {kind}/{case}: committed diff does not carry before to after")
            failures += 1
        rebuilt = reference.build(before, mutation)
        if json.loads(reference.dumps(rebuilt.delta)) != delta:
            print(f"FAIL {kind}/{case}: reference diff differs from the committed diff")
            failures += 1
        if json.loads(reference.dumps(rebuilt.as_json())) != outcome:
            print(f"FAIL {kind}/{case}: reference outcome differs from the committed outcome")
            failures += 1
        restored = produced
        for step in reference.inverse(before, mutation):
            restored = reference.apply(restored, reference.build(restored, step).delta)
        if restored != before:
            print(f"FAIL {kind}/{case}: inverse did not restore the before-snapshot")
            failures += 1
    print(f"{len(VECTORS)} vectors replayed, {failures} failure(s)")
    return failures


if __name__ == "__main__":
    ticket = pathlib.Path(__file__).resolve().parents[11] / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/EXTRACT-WFC-PLUGIN"
    sys.path.insert(0, str(ticket))
    import importlib

    reference = importlib.import_module("🐍️grid2d-oracle")
    raise SystemExit(1 if replay() else 0)
