#!/usr/bin/env python3
"""🧾️ Audits the stdio oracle composition ledger's `callers` rows (`🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧫️fixtures/🧩️composition/🔣️.json`)
the way its own law does (`🧪️tests/🧩️composition/🟦️.ts`: sha256 of the caller with every rewrite's `current` restored to its
`previous`), without editing anything: how many rows are stale, which owners, and for each `--trace` row the newest caller
revision the ledger still matches. Read-only; exits 1 while any row is stale.

usage: python3 🧪️s4-stdio-composition-ledger-audit.py [--trace <source substring>]...
"""
from __future__ import annotations

import collections
import hashlib
import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
LEDGER = "🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧫️fixtures/🧩️composition/🔣️.json"


def restored_digest(text: str, rewrites: list[dict]) -> str:
    for rewrite in rewrites:
        text = text.replace(rewrite["current"], rewrite["previous"])
    return hashlib.sha256(text.encode()).hexdigest()


def matching_revision(row: dict) -> str | None:
    log = subprocess.run(["git", "log", "--format=%h %cd", "--date=iso", "--", row["source"]], cwd=ROOT, capture_output=True, text=True).stdout.splitlines()
    for line in log[:60]:
        revision = line.split()[0]
        text = subprocess.run(["git", "show", f"{revision}:{row['source']}"], cwd=ROOT, capture_output=True).stdout.decode("utf-8", "replace")
        if restored_digest(text, row["rewrites"]) == row["sha256"]:
            return line
    return None


def main() -> int:
    traced = [sys.argv[index + 1] for index, flag in enumerate(sys.argv) if flag == "--trace"]
    callers = json.loads((ROOT / LEDGER).read_text(encoding="utf-8"))["callers"]
    stale = [row for row in callers if not (ROOT / row["source"]).exists() or restored_digest((ROOT / row["source"]).read_text(encoding="utf-8"), row["rewrites"]) != row["sha256"]]
    owners = collections.Counter(row["owner"] for row in stale)
    print(f"{len(stale)} of {len(callers)} caller rows stale")
    for owner, count in owners.most_common():
        print(f"  {count:4d}  {owner}")
    for needle in traced:
        for row in [row for row in callers if needle in row["source"]]:
            print(f"{row['source']}: ledger matches {matching_revision(row) or 'no revision in the last 60'}")
    return 1 if stale else 0


if __name__ == "__main__":
    sys.exit(main())
