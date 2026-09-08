#!/usr/bin/env python3
"""📊 Per-code and per-owner counts over a `test schema --json` report.

Owner buckets are the ones `📓️wp1c-harness-rules.md` §5.3 uses: a plugin root, a product root, the
framework module tree, a hub area, or the first two path segments. A finding with no `path` (a format
the annotation declared and the scope implements no file for) is bucketed through the catalog's
`path` for its scope, so nothing lands in an "unattributed" pile.

    python3 wp1d-owner-counts.py <report.json> [catalog.json]
"""
from __future__ import annotations

import collections
import json
import sys
from pathlib import Path


def owner_of(path: str) -> str:
    parts = [p for p in path.split("/") if p]
    if len(parts) >= 3 and parts[0] == "✏️s" and parts[1] == "\U0001f50c️plugins":
        return "/".join(parts[:3])
    if len(parts) >= 3 and parts[0] == "\U0001f9f0️framework" and parts[1] == "\U0001f6cd️products":
        return "/".join(parts[:3])
    if len(parts) >= 2 and parts[0] == "\U0001f9f0️framework" and parts[1] == "\U0001f528️modules":
        return "/".join(parts[:2])
    return "/".join(parts[:2]) if len(parts) >= 2 else (parts[0] if parts else "?")


def main() -> int:
    report = json.loads(Path(sys.argv[1]).read_text(encoding="utf8"))
    catalog_path = sys.argv[2] if len(sys.argv) > 2 else "\U0001f9f0️framework/\U0001f6cd️products/\U0001f991️repo/\U0001f528️modules/\U0001f4da️library/\U0001f523️schema-catalog.json"
    scopes = json.loads(Path(catalog_path).read_text(encoding="utf8")).get("scopes", {})
    diagnostics = report["diagnostics"]
    by_code = collections.Counter(entry["code"] for entry in diagnostics)
    by_owner: dict[str, collections.Counter] = collections.defaultdict(collections.Counter)
    for entry in diagnostics:
        path = entry.get("path") or (scopes.get(entry.get("scope") or "", {}) or {}).get("path") or ""
        by_owner[owner_of(path)][entry["code"]] += 1
    print(f"findings {len(diagnostics)}   schema-bound fixtures {len(report['fixtures'])}")
    for code, count in by_code.most_common():
        print(f"  {count:5d} × {code}")
    print()
    print("| owner root | findings | by code |")
    print("|---|---|---|")
    for owner, codes in sorted(by_owner.items(), key=lambda row: (-sum(row[1].values()), row[0])):
        detail = " ".join(f"{code.removeprefix('schema-')}={count}" for code, count in codes.most_common())
        print(f"| `{owner}` | {sum(codes.values())} | {detail} |")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
