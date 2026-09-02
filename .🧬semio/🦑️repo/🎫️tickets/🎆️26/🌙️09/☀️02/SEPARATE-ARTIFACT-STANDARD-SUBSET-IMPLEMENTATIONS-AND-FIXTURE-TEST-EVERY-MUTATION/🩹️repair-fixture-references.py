#!/usr/bin/env python3
"""🩹️ Repoints every dead fixture reference at the kind-only basename the file actually carries.

The kind-only basename migration renamed `🔣️component.json` → `🔣️.json` (and named assets such as
`🖊️bus-shelter-r12.dxf` → `🖊️.dxf`) without following the references, so 3676 mutation fixture URIs
resolve to nothing and every mutation they were supposed to witness tests nothing. Rewrites the
`🥒️.feature` files and every sibling adapter in the same case that repeats the same URI.

Run with `--check` to report without writing.
"""
from __future__ import annotations

import json
import sys
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
DRIFT = Path(__file__).resolve().parent / "🗑️generated" / "missing-fixture-drift.json"


def rewritten_uri(uri: str, target: str, kind_only: str) -> str:
    """🔁️ Swaps only the final path segment, so the scheme and every parent segment survive."""
    old = target.rsplit("/", 1)[-1]
    new = kind_only.rsplit("/", 1)[-1]
    if not uri.endswith(old):
        raise ValueError(f"URI {uri!r} does not end in {old!r}")
    return uri[: -len(old)] + new


def main() -> int:
    check = "--check" in sys.argv
    drift = json.loads(DRIFT.read_text(encoding="utf-8"))

    by_case: dict[Path, set[tuple[str, str]]] = defaultdict(set)
    for record in drift:
        case_dir = ROOT / "/".join(record["scope"].split("/")[:-1])
        by_case[case_dir].add((record["uri"], rewritten_uri(record["uri"], record["target"], record["kind_only"])))

    changed_files = 0
    total_replacements = 0
    for case_dir, swaps in sorted(by_case.items()):
        for path in sorted(case_dir.rglob("*")):
            if not path.is_file():
                continue
            try:
                text = path.read_text(encoding="utf-8")
            except (UnicodeDecodeError, OSError):
                continue
            updated = text
            hits = 0
            for old, new in sorted(swaps, key=lambda pair: -len(pair[0])):
                count = updated.count(old)
                if count:
                    updated = updated.replace(old, new)
                    hits += count
            if hits and updated != text:
                total_replacements += hits
                changed_files += 1
                print(f"{'would fix' if check else 'fixed'} {hits:5d}  {path.relative_to(ROOT)}")
                if not check:
                    path.write_text(updated, encoding="utf-8")

    print(f"\ncases={len(by_case)} files={changed_files} replacements={total_replacements}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
