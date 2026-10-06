#!/usr/bin/env python3
"""🧭️ S5-STORE wave C fix-forward (2026-10-05 10:59): three Rust test oracles of the §22.28 wave named the revision oracle as
`crate::os_store::CursorRevisionAccumulator`, but the store module is `crate::os_store::component` (the accumulator is private
to it and not re-exported), so the kernel test build failed with E0433 at `📖️reader/🧪️tests/📖️reader/🦀️.rs:84,339` and
`🧵️borrowed/🧪️tests/🧵️borrowed/🦀️.rs:222`. Test-only, invisible to `--lib`. Rewrites the path in the two test files and in
the wave script, so the script keeps recognising its own hunks as applied. Idempotent. `--check` writes nothing.

    python3 🧪️s5-store-revision-digest-oracle-path.py [--check]
"""
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
STORE_DIR = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store"
FILES = [
    (STORE_DIR / "🧵️canonical-edit/🧵️borrowed/🧪️tests/🧵️borrowed/🦀️.rs", 1),
    (STORE_DIR / "🧵️canonical-edit/📖️reader/🧪️tests/📖️reader/🦀️.rs", 2),
    (Path(__file__).resolve().parent / "🧪️s5-store-revision-digest.py", 3),
]
OLD = "crate::os_store::CursorRevisionAccumulator::revision_value("
NEW = "crate::os_store::component::CursorRevisionAccumulator::revision_value("


def main():
    pending = []
    for path, expected in FILES:
        text = path.read_text(encoding="utf-8")
        if text.count(OLD) == 0 and text.count(NEW) == expected:
            continue
        if text.count(OLD) != expected:
            raise SystemExit(f"{path.name}: {text.count(OLD)} occurrences (expected {expected}): re-derive")
        pending.append((path, text.replace(OLD, NEW)))
    print("pending: " + (", ".join(path.parent.name + "/" + path.name for path, _ in pending) if pending else "none"))
    if "--check" in sys.argv[1:]:
        return
    for path, text in pending:
        path.write_text(text, encoding="utf-8")
    if pending:
        print(f"applied to {len(pending)} files")


main()
