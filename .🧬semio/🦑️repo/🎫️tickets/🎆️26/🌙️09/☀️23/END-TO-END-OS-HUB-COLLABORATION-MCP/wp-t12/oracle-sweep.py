#!/usr/bin/env python3
"""🧹️ Summarises the repo-wide oracle sweep (`test-oracle --implementation python` at exhaustive, 16:28): every
`*-oracle-<implementation>` result directory written since the sweep started, with its non-passing rows bucketed by
cause. Subject-only round trips (`no oracle registration for scenario identity-round-trip`) are by design and left out.
Usage: oracle-sweep.py <epoch-seconds-start>  → markdown on stdout"""
import collections
import json
import re
import sys
from pathlib import Path

results = Path("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tests/results")
since = float(sys.argv[1])
BUCKETS = [
    ("retired fixture path", re.compile(r"is not part of this plan")),
    ("document shape drift", re.compile(r"must carry exactly|the document must carry")),
    ("scenario id drift", re.compile(r"adapter has no oracle registration for scenario")),
    ("declared refusal", re.compile(r"refuses|cannot be read by a second implementation")),
    ("needs the subject phase first", re.compile(r"run its subject phase|no raw subject output")),
]


def bucket(message):
    return next((name for name, pattern in BUCKETS if pattern.search(message)), "behaviour disagreement")


rows_out, totals = [], collections.Counter()
for directory in sorted(results.iterdir()):
    rows_path = directory / "📤️results.jsonl"
    match = re.search(r"-oracle-(python|rust|typescript)$", directory.name)
    if match is None or not rows_path.exists() or rows_path.stat().st_mtime < since:
        continue
    rows = []
    for line in rows_path.read_text(encoding="utf-8").splitlines():
        try:
            rows.append(json.loads(line))
        except json.JSONDecodeError:
            continue
    if not rows:
        continue
    bad = [row for row in rows if row["status"] != "passed" and "no oracle registration for scenario identity-round-trip" not in str(row.get("diagnostics"))]
    totals[match.group(1), "cases"] += 1
    if not bad:
        continue
    totals[match.group(1), "red cases"] += 1
    causes = collections.Counter()
    sample = {}
    for row in bad:
        message = str((row.get("diagnostics") or [{}])[0].get("message") or row["status"])
        name = bucket(message)
        causes[name] += 1
        sample.setdefault(name, message[:140].replace("|", "/").replace("\n", " "))
    owner = rows[0].get("owner", "?")
    top = causes.most_common(1)[0][0]
    rows_out.append((len(bad), len(rows), match.group(1), rows[0].get("case", directory.name), owner, top, sample[top]))
print("| implementation | cases run | cases with non-passing rows |\n|---|---:|---:|")
for implementation in ("python", "rust", "typescript"):
    print(f"| {implementation} | {totals[implementation, 'cases']} | {totals[implementation, 'red cases']} |")
print("\n| red / rows | impl | case | owner | main cause | sample |\n|---:|---|---|---|---|---|")
for bad, total, implementation, case, owner, cause, message in sorted(rows_out, reverse=True):
    print(f"| {bad}/{total} | {implementation} | `{case}` | `{owner.split('/🗿️artifacts/')[-1] if '/🗿️artifacts/' in owner else owner}` | {cause} | {message} |")
