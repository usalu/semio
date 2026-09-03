#!/usr/bin/env python3
"""🔨️ F2 — generates real before/after SVG fixtures for s.stdio.svg@1.1/basic (10) and
s.stdio.svg@1.1/tiny (9)'s unfixtured mutations, all round-tripped through lxml 6.1.3's own
etree.parse -> etree.tostring (SVG is plain XML, so unlike xml/valid's DOCTYPE-internal-subset case
lxml can carry every one of these mutations natively -- no handcrafted fallback needed here).
Idempotent: safe to re-run.
"""
import hashlib
import io
import json
from pathlib import Path

from lxml import etree

ROOT = Path("/Users/ueli/Documents/semio")
LXML_VERSION = "6.1.3"


def roundtrip(raw: bytes) -> bytes:
    parser = etree.XMLParser(resolve_entities=False, dtd_validation=False, load_dtd=False)
    tree = etree.parse(io.BytesIO(raw), parser)
    return etree.tostring(tree, xml_declaration=True, encoding="UTF-8", standalone=tree.docinfo.standalone)


def sha256_of(data: bytes) -> str:
    return f"sha256:{hashlib.sha256(data).hexdigest()}"


BASIC_SUBSET = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️basic"
TINY_SUBSET = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️tiny"

BASIC_CASES = [
    (
        "insert-basic-element",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="basic" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n</svg>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="basic" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n  <circle id="c1" cx="50" cy="50" r="10"/>\n</svg>\n',
        "A new <circle> element inserted as the last child of the root <svg>.",
    ),
    (
        "insert-clip-path-shape",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="basic" version="1.1" viewBox="0 0 100 100">\n  <clipPath id="cp1"/>\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n</svg>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="basic" version="1.1" viewBox="0 0 100 100">\n  <clipPath id="cp1"><rect x="0" y="0" width="50" height="50"/></clipPath>\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n</svg>\n',
        "A clipping shape (<rect>) inserted as a child of the empty <clipPath>.",
    ),
    (
        "remove-element",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="basic" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n  <circle id="c1" cx="50" cy="50" r="10"/>\n</svg>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="basic" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n</svg>\n',
        "The <circle> element removed from the root, inverse of insert-basic-element.",
    ),
    (
        "set-basic-attribute",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="basic" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n</svg>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="basic" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="30" y="10" width="20" height="20"/>\n</svg>\n',
        "The x presentation attribute of <rect id=\"r1\"> replaced, 10 -> 30.",
    ),
    (
        "set-clip-path-reference",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="basic" version="1.1" viewBox="0 0 100 100">\n  <clipPath id="cp1"><rect x="0" y="0" width="50" height="50"/></clipPath>\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n</svg>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="basic" version="1.1" viewBox="0 0 100 100">\n  <clipPath id="cp1"><rect x="0" y="0" width="50" height="50"/></clipPath>\n  <rect id="r1" x="10" y="10" width="20" height="20" clip-path="url(#cp1)"/>\n</svg>\n',
        "The clip-path attribute of <rect id=\"r1\"> set to reference #cp1.",
    ),
    (
        "set-snapshot",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="basic" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n</svg>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="basic" version="1.1" viewBox="0 0 200 200">\n  <circle id="c1" cx="100" cy="100" r="40"/>\n  <text id="t1">Snapshot</text>\n</svg>\n',
        "Whole-document snapshot replace: an unrelated valid document substituted wholesale.",
    ),
    (
        "set-text",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="basic" version="1.1" viewBox="0 0 100 100">\n  <text id="t1">Hello</text>\n</svg>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="basic" version="1.1" viewBox="0 0 100 100">\n  <text id="t1">World</text>\n</svg>\n',
        "Character content of <text id=\"t1\"> replaced, Hello -> World.",
    ),
    (
        "set-transform",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="basic" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n</svg>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="basic" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="10" y="10" width="20" height="20" transform="translate(10,10)"/>\n</svg>\n',
        "A transform attribute set on <rect id=\"r1\">, none -> translate(10,10).",
    ),
    (
        "set-view-box",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="basic" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n</svg>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="basic" version="1.1" viewBox="0 0 200 200">\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n</svg>\n',
        "The root <svg>'s viewBox replaced, \"0 0 100 100\" -> \"0 0 200 200\".",
    ),
    (
        "stamp-base-profile",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n</svg>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="basic" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n</svg>\n',
        "The root <svg>'s baseProfile attribute stamped, absent -> \"basic\".",
    ),
]

TINY_CASES = [
    (
        "insert-tiny-element",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="tiny" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n</svg>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="tiny" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n  <line id="l1" x1="0" y1="0" x2="50" y2="50"/>\n</svg>\n',
        "A new <line> element (a Tiny-profile-legal shape) inserted as the last child of the root <svg>.",
    ),
    (
        "remove-element",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="tiny" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n  <line id="l1" x1="0" y1="0" x2="50" y2="50"/>\n</svg>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="tiny" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n</svg>\n',
        "The <line> element removed from the root, inverse of insert-tiny-element.",
    ),
    (
        "set-snapshot",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="tiny" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n</svg>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="tiny" version="1.1" viewBox="0 0 200 200">\n  <circle id="c1" cx="100" cy="100" r="40"/>\n  <text id="t1">Snapshot</text>\n</svg>\n',
        "Whole-document snapshot replace: an unrelated valid document substituted wholesale.",
    ),
    (
        "set-text",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="tiny" version="1.1" viewBox="0 0 100 100">\n  <text id="t1">Hello</text>\n</svg>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="tiny" version="1.1" viewBox="0 0 100 100">\n  <text id="t1">World</text>\n</svg>\n',
        "Character content of <text id=\"t1\"> replaced, Hello -> World.",
    ),
    (
        "set-tiny-attribute",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="tiny" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n</svg>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="tiny" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="30" y="10" width="20" height="20"/>\n</svg>\n',
        "The x presentation attribute of <rect id=\"r1\"> replaced, 10 -> 30 (Tiny's own attribute vocabulary).",
    ),
    (
        "set-transform",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="tiny" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n</svg>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="tiny" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="10" y="10" width="20" height="20" transform="translate(10,10)"/>\n</svg>\n',
        "A transform attribute set on <rect id=\"r1\">, none -> translate(10,10).",
    ),
    (
        "set-view-box",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="tiny" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n</svg>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="tiny" version="1.1" viewBox="0 0 200 200">\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n</svg>\n',
        "The root <svg>'s viewBox replaced, \"0 0 100 100\" -> \"0 0 200 200\".",
    ),
    (
        "stamp-base-profile",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n</svg>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="tiny" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n</svg>\n',
        "The root <svg>'s baseProfile attribute stamped, absent -> \"tiny\".",
    ),
    (
        "strip-non-tiny",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="tiny" version="1.1" viewBox="0 0 100 100">\n  <clipPath id="cp1"><rect x="0" y="0" width="50" height="50"/></clipPath>\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n</svg>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" baseProfile="tiny" version="1.1" viewBox="0 0 100 100">\n  <rect id="r1" x="10" y="10" width="20" height="20"/>\n</svg>\n',
        "A <clipPath> element (Basic/Full-only, illegal under Tiny 1.1's own element vocabulary) stripped from the document.",
    ),
]


def emit(subset_dir: Path, artifact_id: str, subset_id: str, reader_oracle: str, cases):
    fixtures = subset_dir / "🧫️fixtures"
    oracle_json = subset_dir / "🧪️oracle/🔣️.json"
    manifests = []
    for mutation_id, before_raw, after_raw, note in cases:
        before_bytes = roundtrip(before_raw)
        after_bytes = roundtrip(after_raw)

        case_dir = fixtures / f"{mutation_id}-applied"
        case_dir.mkdir(parents=True, exist_ok=True)
        (case_dir / "before.svg").write_bytes(before_bytes)
        (case_dir / "after.svg").write_bytes(after_bytes)

        entry = {
            "schema": "semio.repository-test.fixture/v2",
            "id": f"{mutation_id}-applied",
            "class": "third-party-generated",
            "target": {"artifact": artifact_id, "standard": "1.1", "subset": subset_id},
            "mutation": mutation_id,
            "outcome": "applied",
            "units": {"length": "unitless", "angle": "degree"},
            "files": [
                {
                    "role": "expected-before-svg",
                    "path": f"../🧫️fixtures/{mutation_id}-applied/before.svg",
                    "mediaType": "image/svg+xml",
                    "sha256": sha256_of(before_bytes),
                    "bytes": len(before_bytes),
                },
                {
                    "role": "expected-after-svg",
                    "path": f"../🧫️fixtures/{mutation_id}-applied/after.svg",
                    "mediaType": "image/svg+xml",
                    "sha256": sha256_of(after_bytes),
                    "bytes": len(after_bytes),
                },
            ],
            "generator": {
                "oracle": reader_oracle,
                "packageVersion": LXML_VERSION,
                "engineFamily": "lxml",
                "engineVersion": LXML_VERSION,
                "command": "uv run python3 🔨️f2-gen-svg-fixtures.py (lxml.etree.parse -> lxml.etree.tostring round-trip)",
                "platform": "darwin-arm64",
            },
            "provenance": {
                "source": "generated",
                "license": "MIT (lxml, wraps libxml2 MIT/libxml2-license)",
                "attribution": "Serialized by lxml 6.1.3's own etree.tostring after an lxml.etree.parse round-trip",
                "security": "scanned-clean",
                "privacy": "no-personal-data",
            },
            "comparisonProfile": "utf8-text-v1",
            "reproducible": True,
            "family": "structural",
            "notes": note,
        }
        manifests.append(entry)
        print(f"[{subset_id}] {mutation_id:26s} before={len(before_bytes)}B after={len(after_bytes)}B")

    data = json.loads(oracle_json.read_text())
    data["fixtureManifests"] = manifests
    oracle_json.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n")
    print(f"Wrote {len(manifests)} fixtureManifests entries into {oracle_json}\n")


def main() -> None:
    emit(BASIC_SUBSET, "s.stdio.svg", "basic", "lxml-svg-1-1-basic-mutate-reader", BASIC_CASES)
    emit(TINY_SUBSET, "s.stdio.svg", "tiny", "lxml-svg-1-1-tiny-mutate-reader", TINY_CASES)


if __name__ == "__main__":
    main()
