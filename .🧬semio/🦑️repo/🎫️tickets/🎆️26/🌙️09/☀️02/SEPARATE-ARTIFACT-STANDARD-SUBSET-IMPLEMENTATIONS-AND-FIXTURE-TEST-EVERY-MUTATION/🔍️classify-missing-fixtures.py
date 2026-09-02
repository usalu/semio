#!/usr/bin/env python3
"""🔍️ Splits every `missing-fixture` breach into DRIFT (the file exists under a kind-only basename)
and REAL (nothing on disk answers the reference), so the fleet works on genuine fixture gaps only.

`asset://` resolves against the case OWNER, `shared://` against the owner's `🧫️fixtures`, and
`local://` against the case's own `🧫️fixtures`.
"""
from __future__ import annotations

import collections
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
BREACHES = ROOT / ".🧬semio" / "🦑️repo" / "⚡️cache" / "breaches" / "testing.json"
OUT = Path(__file__).resolve().parent / "🗑️generated"

URI_RE = re.compile(r"Fixture (\S+) does not resolve")
EMOJI_ENTRY_RE = re.compile(r"^([^\w\s]+)([A-Za-z0-9._-]*)(\.[A-Za-z0-9]+)$")


def owner_of(scope: str) -> Path:
    """🧭️ The directory that holds `🧪️tests` for a case whose feature path is `scope`."""
    parts = scope.split("/")
    for index in range(len(parts) - 1, -1, -1):
        if parts[index] == "🧪️tests":
            return ROOT / "/".join(parts[:index])
    return ROOT / "/".join(parts[:-1])


def case_dir_of(scope: str) -> Path:
    return ROOT / "/".join(scope.split("/")[:-1])


def resolve(uri: str, scope: str) -> Path | None:
    owner = owner_of(scope)
    if uri.startswith("asset://"):
        return owner / uri[len("asset://") :]
    if uri.startswith("shared://"):
        return owner / "🧫️fixtures" / uri[len("shared://") :]
    if uri.startswith("local://"):
        return case_dir_of(scope) / "🧫️fixtures" / uri[len("local://") :]
    return None


def kind_only(name: str) -> str | None:
    """🏷️ `🔣️component.json` → `🔣️.json`; returns None when the name already is kind-only."""
    match = EMOJI_ENTRY_RE.match(name)
    if not match:
        return None
    prefix, stem, ext = match.groups()
    if not stem:
        return None
    return f"{prefix}{ext}"


def main() -> int:
    OUT.mkdir(exist_ok=True)
    breaches = json.loads(BREACHES.read_text(encoding="utf-8"))
    missing = [b for b in breaches if b["id"] == "missing-fixture"]

    drift: list[dict] = []
    real: list[dict] = []
    unresolvable: list[dict] = []

    for breach in missing:
        match = URI_RE.search(breach["summary"])
        if not match:
            unresolvable.append(breach)
            continue
        uri = match.group(1)
        target = resolve(uri, breach["scope"])
        if target is None:
            unresolvable.append(breach)
            continue
        record = {"scope": breach["scope"], "uri": uri, "target": str(target.relative_to(ROOT))}
        if target.exists():
            record["note"] = "exists — policy resolves it differently"
            real.append(record)
            continue
        renamed = kind_only(target.name)
        if renamed and (target.parent / renamed).exists():
            record["kind_only"] = str((target.parent / renamed).relative_to(ROOT))
            drift.append(record)
            continue
        real.append(record)

    (OUT / "missing-fixture-drift.json").write_text(json.dumps(drift, ensure_ascii=False, indent=2), encoding="utf-8")
    (OUT / "missing-fixture-real.json").write_text(json.dumps(real, ensure_ascii=False, indent=2), encoding="utf-8")

    print(f"missing-fixture total={len(missing)} drift={len(drift)} real={len(real)} unparsed={len(unresolvable)}")
    print("\n-- DRIFT by feature file (top 30)")
    for scope, count in collections.Counter(d["scope"] for d in drift).most_common(30):
        print(f"{count:5d}  {scope}")
    print("\n-- REAL by artifact (top 40)")
    for key, count in collections.Counter("/".join(r["scope"].split("/")[:5]) for r in real).most_common(40):
        print(f"{count:5d}  {key}")
    print("\n-- REAL sample")
    for record in real[:15]:
        print(f"   {record['scope']}\n     -> {record['uri']}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
