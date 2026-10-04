#!/usr/bin/env python3
"""🪟️ The byte-authoritative BMP v3 vocabulary's fixture-backed vectors (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING):
`set-snapshot-applied` rewritten from the retired decoded-pixel snapshot to `{schema, bytes}` readings (the committed small
indexed document replaced by the committed 2x2 top-down direct-colour one), a new `patch-snapshot-applied` vector (one octet
of the small indexed document's pixel array, its after-reading computed by the host's independent `patched_snapshot`), and
fixture manifests registering the committed `🧬️history-edits` paint vectors. Pillow (the repo `.venv`) must decode every
reading's octets and see the patched pixel move. Idempotent; `--check` exits 1 while anything is pending.

usage: .venv/bin/python 🧪️s4-stdio-bmp-vectors.py [--check]
"""
from __future__ import annotations

import hashlib
import importlib.util
import io
import json
import pathlib
import sys

from PIL import Image

ROOT = pathlib.Path(__file__).resolve().parents[7]
SUBSET = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any"
FIXTURES = SUBSET / "🧫️fixtures"
CATALOG = SUBSET / "🔮️oracles/🔣️.json"
SMALL = FIXTURES / "🎨️replace-palette-entry-applied/⬅️before.bmp"
DIRECT = FIXTURES / "🧬️canonical-byte-authority/direct-rgb24-top-down.bmp"
PATCH = {"operation": "set", "path": "/bytes/82", "value": 5}


def host():
    spec = importlib.util.spec_from_file_location("semio_repo_test", ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py")
    loaded = importlib.util.module_from_spec(spec)
    sys.modules["semio_repo_test"] = loaded
    spec.loader.exec_module(loaded)
    return loaded


def reading(octets: bytes) -> dict:
    return {"schema": "stdio.bmp", "bytes": list(octets)}


def text(value) -> str:
    return json.dumps(value, indent=2) + "\n"


def decoded(value: dict) -> Image.Image:
    image = Image.open(io.BytesIO(bytes(value["bytes"])))
    image.load()
    return image


def files() -> dict[pathlib.Path, str]:
    library = host()
    small, direct = reading(SMALL.read_bytes()), reading(DIRECT.read_bytes())
    patched = library.patched_snapshot(small, PATCH)
    assert decoded(small).tobytes() != decoded(patched).tobytes(), "Pillow must see the patched pixel move"
    assert library.patched_snapshot(patched, library.snapshot_patch_inverse(small, PATCH)) == small, "the exact inverse must restore the reading"
    decoded(direct)
    return {
        FIXTURES / "📸️set-snapshot-applied/⬅️before.json": text(small),
        FIXTURES / "📸️set-snapshot-applied/➡️after.json": text(direct),
        FIXTURES / "🩹️patch-snapshot-applied/⬅️before.json": text(small),
        FIXTURES / "🩹️patch-snapshot-applied/🦠️mutation.json": text({"mutation": "patch-snapshot", "payload": {"patch": PATCH}}),
        FIXTURES / "🩹️patch-snapshot-applied/➡️after.json": text(patched),
    }


def entry(written: dict[pathlib.Path, str], identity: str, mutation: str, roles: list[tuple[str, pathlib.Path, str]], notes: str) -> dict:
    def described(role: str, path: pathlib.Path, media: str) -> dict:
        content = written[path].encode() if path in written else path.read_bytes()
        return {"role": role, "path": "../" + str(path.relative_to(SUBSET)), "mediaType": media, "sha256": "sha256:" + hashlib.sha256(content).hexdigest(), "bytes": len(content)}

    return {
        "schema": "semio.repository-test.fixture/v2",
        "id": identity,
        "class": "handcrafted",
        "target": {"artifact": "s.stdio.bmp", "standard": "v3", "subset": "any"},
        "mutation": mutation,
        "outcome": "applied",
        "units": {"length": "unitless", "angle": "degree"},
        "files": [described(role, path, media) for role, path, media in roles],
        "provenance": {"source": "authored", "license": "public-domain (handcrafted by this repository)", "attribution": "Handcrafted before/after vector over committed BMP octets; every reading decodes through Pillow.", "security": "scanned-clean", "privacy": "no-personal-data"},
        "comparisonProfile": "ordered-json-v1",
        "reproducible": True,
        "family": "bmp-carrier",
        "notes": notes,
    }


def with_catalog(current: str, written: dict[pathlib.Path, str]) -> str:
    catalog = json.loads(current)
    history = FIXTURES / "🧬️history-edits"
    json_type, dsl_type = "application/json", "text/plain"
    entries = [
        entry(written, "set-snapshot-applied", "set-snapshot", [("expected-before", FIXTURES / "📸️set-snapshot-applied/⬅️before.json", json_type), ("expected-after", FIXTURES / "📸️set-snapshot-applied/➡️after.json", json_type)], "SetSnapshot replaces the committed small indexed document's octets with the committed 2x2 top-down direct-colour document's."),
        entry(written, "patch-snapshot-applied", "patch-snapshot", [("expected-before", FIXTURES / "🩹️patch-snapshot-applied/⬅️before.json", json_type), ("mutation-json", FIXTURES / "🩹️patch-snapshot-applied/🦠️mutation.json", json_type), ("expected-after", FIXTURES / "🩹️patch-snapshot-applied/➡️after.json", json_type)], "One RFC 6901 set of octet 82 — the first stored pixel index of the small indexed document — from 2 to 5; the after-reading is the host's independent patched_snapshot, and Pillow decodes both."),
    ]
    for kind, leaf in (("paint-indexed-region", "🎨️paint-indexed-region"), ("paint-direct-region", "🖌️paint-direct-region")):
        vector = history / leaf / "🎯️direct"
        entries.append(entry(written, f"{kind}-applied", kind, [("expected-before", vector / "📸️snapshot/⬅️before/🗣️.dsl.semio", dsl_type), ("mutation-json", vector / "🦠️mutation/🔣️.json", json_type), ("expected-after", vector / "📸️snapshot/➡️after/🗣️.dsl.semio", dsl_type), ("outcome-json", vector / "🎯️outcome/🔣️.json", json_type)], f"The committed revision-guarded {kind} history-edit vector: before and after as the snapshot's hex DSL, the guarded payload and its applied outcome."))
    identities = {item["id"] for item in entries}
    catalog["fixtureManifests"] = [existing for existing in catalog["fixtureManifests"] if existing["id"] not in identities] + entries
    indent = 2 if current.startswith('{\n  "') else 1
    return json.dumps(catalog, indent=indent, ensure_ascii=False) + "\n"


def main() -> int:
    check = "--check" in sys.argv
    written = files()
    wanted = dict(written)
    wanted[CATALOG] = with_catalog(CATALOG.read_text(encoding="utf-8"), written)
    pending = 0
    for path, content in wanted.items():
        current = path.read_text(encoding="utf-8") if path.exists() else None
        if current != content:
            pending += 1
            print(f"{'pending' if check else 'written'}: {path.relative_to(SUBSET)}")
            if not check:
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(content, encoding="utf-8")
    print(f"bmp vectors verified through Pillow (patched pixel moves, inverse restores); {pending} file(s) {'pending' if check else 'written'}")
    return 1 if check and pending else 0


if __name__ == "__main__":
    sys.exit(main())
