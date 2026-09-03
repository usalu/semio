#!/usr/bin/env python3
"""🩹️ Shard F3 helper — appends one handcrafted v2 FixtureManifest to a subset's own
🧪️oracle/🔣️.json and writes the before/after fixture files it points at, computing real sha256
digests. Idempotent per mutation id (skips if a fixtureManifest with that `mutation` already
exists). Used to close `mutation-without-fixture` breaches with honest, handcrafted evidence —
see 🧰️framework/…/🧪️test/📦️packages/🟦️typescript/🟦️.ts:mutationFixtureBreaches for what this
satisfies, and 📓️f3-fixture-tail-and-stragglers.md for the per-artifact record.
"""
from __future__ import annotations
import json, hashlib, sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")


def sha256_of(path: Path) -> str:
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()


def add_fixture(
    subset_owner: str,
    artifact: str,
    standard: str,
    subset: str,
    mutation: str,
    before_text: str,
    after_text: str,
    ext: str,
    media_type: str,
    comparison_profile: str,
    notes: str,
    family: str = "structural",
    outcome: str = "applied",
    units: dict | None = None,
    fixture_id: str | None = None,
) -> str:
    owner_dir = ROOT / subset_owner
    oracle_path = owner_dir / "🧪️oracle" / "🔣️.json"
    data = json.loads(oracle_path.read_text(encoding="utf-8"))
    data.setdefault("fixtureManifests", [])

    for existing in data["fixtureManifests"]:
        if existing.get("mutation") == mutation and existing.get("target", {}).get("subset") == subset:
            return f"SKIP (already present): {subset_owner}#{mutation}"

    fixtures_dir = owner_dir / "🧫️fixtures" / mutation
    fixtures_dir.mkdir(parents=True, exist_ok=True)
    before_path = fixtures_dir / f"before.{ext}"
    after_path = fixtures_dir / f"after.{ext}"
    before_path.write_text(before_text, encoding="utf-8")
    after_path.write_text(after_text, encoding="utf-8")

    fid = fixture_id or f"{mutation}-applied"
    entry = {
        "schema": "semio.repository-test.fixture/v2",
        "id": fid,
        "class": "handcrafted",
        "target": {"artifact": artifact, "standard": standard, "subset": subset},
        "mutation": mutation,
        "outcome": outcome,
        "units": units or {"length": "unitless", "angle": "degree"},
        "files": [
            {
                "role": "expected-before",
                "path": f"../🧫️fixtures/{mutation}/before.{ext}",
                "mediaType": media_type,
                "sha256": sha256_of(before_path),
                "bytes": before_path.stat().st_size,
            },
            {
                "role": "expected-after",
                "path": f"../🧫️fixtures/{mutation}/after.{ext}",
                "mediaType": media_type,
                "sha256": sha256_of(after_path),
                "bytes": after_path.stat().st_size,
            },
        ],
        "provenance": {
            "source": "authored",
            "license": "public-domain (handcrafted by this repository)",
            "attribution": "Handcrafted before/after vector authored directly against this subset's own documented schema — Law 2's handcrafted-vector category.",
            "security": "scanned-clean",
            "privacy": "no-personal-data",
        },
        "comparisonProfile": comparison_profile,
        "reproducible": True,
        "family": family,
        "notes": notes,
    }
    data["fixtureManifests"].append(entry)
    oracle_path.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    return f"OK: {subset_owner}#{mutation}"


if __name__ == "__main__":
    print("import and call add_fixture(...) — not a standalone CLI", file=sys.stderr)
    sys.exit(1)
