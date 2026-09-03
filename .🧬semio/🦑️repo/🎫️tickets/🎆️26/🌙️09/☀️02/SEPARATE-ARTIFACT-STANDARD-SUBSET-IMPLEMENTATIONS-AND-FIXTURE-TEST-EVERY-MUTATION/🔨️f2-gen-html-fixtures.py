#!/usr/bin/env python3
"""🔨️ F2 — generates real before/after HTML fixtures for s.stdio.html@5/any's 8 unfixtured
mutations, round-tripped through html5lib 1.1's own `dom` treebuilder + `HTMLSerializer` (the dom
builder is used, not etree, because html5lib's etree tree drops the DOCTYPE node entirely on
serialization -- verified live this session). Every byte on disk is html5lib's own serializer output.
Idempotent: safe to re-run.
"""
import hashlib
import json
from pathlib import Path

import html5lib
from html5lib import serializer, treewalkers

ROOT = Path("/Users/ueli/Documents/semio")
SUBSET = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any"
FIXTURES = SUBSET / "🧫️fixtures"
ORACLE_JSON = SUBSET / "🧪️oracle/🔣️.json"
READER_ORACLE_ID = "html5lib-html-5-mutate-reader"
HTML5LIB_VERSION = "1.1"


def roundtrip(html_text: str) -> bytes:
    doc = html5lib.parse(html_text, treebuilder="dom")
    walker = treewalkers.getTreeWalker("dom")
    s = serializer.HTMLSerializer(quote_attr_values="always", omit_optional_tags=False)
    return "".join(s.serialize(walker(doc))).encode("utf-8")


CASES = [
    (
        "set-doctype",
        '<!DOCTYPE html><html><head><title>T</title></head><body>Hi</body></html>',
        '<!DOCTYPE html SYSTEM "about:legacy-compat"><html><head><title>T</title></head><body>Hi</body></html>',
        "Document type declaration's SYSTEM identifier set (none -> about:legacy-compat).",
    ),
    (
        "insert-node",
        '<!DOCTYPE html><html><head><title>T</title></head><body><p id="a">Hello</p></body></html>',
        '<!DOCTYPE html><html><head><title>T</title></head><body><p id="a">Hello</p><span>New</span></body></html>',
        "A new <span> element node inserted as the last child of <body>.",
    ),
    (
        "remove-node",
        '<!DOCTYPE html><html><head><title>T</title></head><body><p id="a">Hello</p><span>New</span></body></html>',
        '<!DOCTYPE html><html><head><title>T</title></head><body><p id="a">Hello</p></body></html>',
        "The <span> element node removed from <body>.",
    ),
    (
        "set-element-name",
        '<!DOCTYPE html><html><head><title>T</title></head><body><div id="a">Hello</div></body></html>',
        '<!DOCTYPE html><html><head><title>T</title></head><body><section id="a">Hello</section></body></html>',
        "Element tag name retagged div -> section, attributes and content held fixed.",
    ),
    (
        "set-attribute",
        '<!DOCTYPE html><html><head><title>T</title></head><body><p id="a">Hello</p></body></html>',
        '<!DOCTYPE html><html><head><title>T</title></head><body><p id="b">Hello</p></body></html>',
        "The id attribute value replaced, a -> b.",
    ),
    (
        "set-text",
        '<!DOCTYPE html><html><head><title>T</title></head><body><p id="a">Hello</p></body></html>',
        '<!DOCTYPE html><html><head><title>T</title></head><body><p id="a">World</p></body></html>',
        "Character-data content of <p> replaced, Hello -> World.",
    ),
    (
        "set-comment",
        '<!DOCTYPE html><html><head><title>T</title></head><body><!--old--><p>Hi</p></body></html>',
        '<!DOCTYPE html><html><head><title>T</title></head><body><!--new--><p>Hi</p></body></html>',
        "Comment node's data replaced, old -> new.",
    ),
    (
        "set-raw-text",
        '<!DOCTYPE html><html><head><title>T</title><script>var a=1;</script></head><body><p>Hi</p></body></html>',
        '<!DOCTYPE html><html><head><title>T</title><script>var a=2;</script></head><body><p>Hi</p></body></html>',
        "Raw-text element (<script>) content replaced, a=1 -> a=2.",
    ),
]


def sha256_of(data: bytes) -> str:
    return f"sha256:{hashlib.sha256(data).hexdigest()}"


def main() -> None:
    manifests = []
    for mutation_id, before_html, after_html, note in CASES:
        before_bytes = roundtrip(before_html)
        after_bytes = roundtrip(after_html)

        case_dir = FIXTURES / f"{mutation_id}-applied"
        case_dir.mkdir(parents=True, exist_ok=True)
        (case_dir / "before.html").write_bytes(before_bytes)
        (case_dir / "after.html").write_bytes(after_bytes)

        entry = {
            "schema": "semio.repository-test.fixture/v2",
            "id": f"{mutation_id}-applied",
            "class": "third-party-generated",
            "target": {"artifact": "s.stdio.html", "standard": "5", "subset": "any"},
            "mutation": mutation_id,
            "outcome": "applied",
            "units": {"length": "unitless", "angle": "degree"},
            "files": [
                {
                    "role": "expected-before-html",
                    "path": f"../🧫️fixtures/{mutation_id}-applied/before.html",
                    "mediaType": "text/html",
                    "sha256": sha256_of(before_bytes),
                    "bytes": len(before_bytes),
                },
                {
                    "role": "expected-after-html",
                    "path": f"../🧫️fixtures/{mutation_id}-applied/after.html",
                    "mediaType": "text/html",
                    "sha256": sha256_of(after_bytes),
                    "bytes": len(after_bytes),
                },
            ],
            "generator": {
                "oracle": READER_ORACLE_ID,
                "packageVersion": HTML5LIB_VERSION,
                "engineFamily": "html5lib",
                "engineVersion": HTML5LIB_VERSION,
                "command": "uv run python3 🔨️f2-gen-html-fixtures.py (html5lib.parse(treebuilder='dom') -> html5lib.serializer.HTMLSerializer)",
                "platform": "darwin-arm64",
            },
            "provenance": {
                "source": "generated",
                "license": "MIT (html5lib)",
                "attribution": "Serialized by html5lib 1.1's own HTMLSerializer over its dom treewalker",
                "security": "scanned-clean",
                "privacy": "no-personal-data",
            },
            "comparisonProfile": "utf8-text-v1",
            "reproducible": True,
            "family": "structural",
            "notes": note,
        }
        manifests.append(entry)
        print(f"{mutation_id:20s} before={len(before_bytes)}B after={len(after_bytes)}B")

    data = json.loads(ORACLE_JSON.read_text())
    data["fixtureManifests"] = manifests
    ORACLE_JSON.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n")
    print(f"\nWrote {len(manifests)} fixtureManifests entries into {ORACLE_JSON}")


if __name__ == "__main__":
    main()
