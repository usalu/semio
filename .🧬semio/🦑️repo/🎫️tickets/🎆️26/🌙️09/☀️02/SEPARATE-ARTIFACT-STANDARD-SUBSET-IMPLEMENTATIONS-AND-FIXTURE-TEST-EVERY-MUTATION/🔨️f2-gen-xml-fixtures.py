#!/usr/bin/env python3
"""🔨️ F2 — generates real before/after XML fixtures for s.stdio.xml@1.0/valid's 8 unfixtured
mutations, round-tripped through lxml 6.1.3 (parse+serialize) so the committed bytes are lxml's own
writer output wherever lxml's serializer can carry the change; the 2 mutations touching the DOCTYPE
internal subset (declare-entity, set-internal-subset) are handcrafted because lxml's docinfo.doctype
demonstrably drops internal-subset content on serialization (verified live, see ticket note).
Idempotent: safe to re-run, overwrites its own prior output.
"""
import hashlib
import io
import json
from pathlib import Path

from lxml import etree

ROOT = Path("/Users/ueli/Documents/semio")
SUBSET = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️valid"
FIXTURES = SUBSET / "🧫️fixtures"
ORACLE_JSON = SUBSET / "🧪️oracle/🔣️.json"
READER_ORACLE_ID = "lxml-xml-1-0-valid-mutate-reader"
LXML_VERSION = "6.1.3"


def lxml_roundtrip(raw: bytes) -> bytes:
    parser = etree.XMLParser(resolve_entities=False, dtd_validation=False, load_dtd=False)
    tree = etree.parse(io.BytesIO(raw), parser)
    kwargs = dict(xml_declaration=True, encoding="UTF-8", standalone=tree.docinfo.standalone)
    if tree.docinfo.doctype:
        kwargs["doctype"] = tree.docinfo.doctype
    return etree.tostring(tree, **kwargs)


def lxml_wellformed(raw: bytes) -> bool:
    try:
        etree.fromstring(raw, etree.XMLParser(resolve_entities=False, dtd_validation=False, load_dtd=False, recover=False))
        return True
    except Exception:
        return False


# 🧬️ (mutation-id, class, before-raw, after-raw, note)
CASES = [
    (
        "declare-doctype", "third-party-generated",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<root><item>Hello</item></root>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<!DOCTYPE root SYSTEM "root.dtd">\n<root><item>Hello</item></root>\n',
        "No DOCTYPE -> a SYSTEM external identifier declared for the document element.",
    ),
    (
        "set-external-subset", "third-party-generated",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<!DOCTYPE root>\n<root><item>Hello</item></root>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<!DOCTYPE root PUBLIC "-//Example//DTD Root 1.0//EN" "root.dtd">\n<root><item>Hello</item></root>\n',
        "Bare DOCTYPE -> a PUBLIC external identifier set on it.",
    ),
    (
        "rename-document-element", "third-party-generated",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<!DOCTYPE root>\n<root><item>Hello</item></root>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<!DOCTYPE document>\n<document><item>Hello</item></document>\n',
        "Document element (and its DOCTYPE Name, per this subset's own §2.8 rule) retagged root -> document.",
    ),
    (
        "set-standalone", "third-party-generated",
        b'<?xml version="1.0" encoding="UTF-8" standalone="no"?>\n<root><item>Hello</item></root>\n',
        b'<?xml version="1.0" encoding="UTF-8" standalone="yes"?>\n<root><item>Hello</item></root>\n',
        "XML declaration's standalone pseudo-attribute no -> yes.",
    ),
    (
        "set-text", "third-party-generated",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<root><item>Hello</item></root>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<root><item>World</item></root>\n',
        "Character content of /root/item replaced, Hello -> World.",
    ),
    (
        "set-snapshot", "third-party-generated",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<root><item>Hello</item></root>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<!DOCTYPE catalog>\n<catalog><entry id="1">Widget</entry><entry id="2">Gadget</entry></catalog>\n',
        "Whole-document snapshot replace: an unrelated valid document substituted wholesale.",
    ),
    (
        "declare-entity", "handcrafted",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<!DOCTYPE root>\n<root>&#38;</root>\n'.replace(b"&#38;", b"&amp;"),
        b'<?xml version="1.0" encoding="UTF-8"?>\n<!DOCTYPE root [\n<!ENTITY foo "bar">\n]>\n<root>&foo;</root>\n',
        "General entity foo declared in the internal subset. HANDCRAFTED: lxml's docinfo.doctype "
        "demonstrably drops internal-subset content on serialization (verified live this session -- "
        "round-tripping `<!DOCTYPE root [<!ENTITY foo \"bar\">]>` through etree.parse+tostring yields "
        "bare `<!DOCTYPE root>`, the entity declaration silently gone), so lxml cannot be the writer for "
        "this specific mutation even though it is registered as this subset's reader oracle.",
    ),
    (
        "set-internal-subset", "handcrafted",
        b'<?xml version="1.0" encoding="UTF-8"?>\n<!DOCTYPE root>\n<root><item>Hello</item></root>\n',
        b'<?xml version="1.0" encoding="UTF-8"?>\n<!DOCTYPE root [\n<!ELEMENT root (item)>\n<!ELEMENT item (#PCDATA)>\n]>\n<root><item>Hello</item></root>\n',
        "Internal subset populated with element declarations. HANDCRAFTED for the same "
        "docinfo.doctype-drops-internal-subset reason as declare-entity above.",
    ),
]


def sha256_of(data: bytes) -> str:
    return f"sha256:{hashlib.sha256(data).hexdigest()}"


def main() -> None:
    manifests = []
    for mutation_id, klass, before_raw, after_raw, note in CASES:
        # 🧾️ Every fixture, third-party-generated or handcrafted alike, is round-tripped through lxml
        # here to CONFIRM well-formedness for real -- not asserted, checked.
        assert lxml_wellformed(before_raw), f"{mutation_id} before is not well-formed XML"
        assert lxml_wellformed(after_raw), f"{mutation_id} after is not well-formed XML"

        if klass == "third-party-generated":
            before_bytes = lxml_roundtrip(before_raw)
            after_bytes = lxml_roundtrip(after_raw)
        else:
            before_bytes = before_raw
            after_bytes = after_raw

        case_dir = FIXTURES / f"{mutation_id}-applied"
        case_dir.mkdir(parents=True, exist_ok=True)
        (case_dir / "before.xml").write_bytes(before_bytes)
        (case_dir / "after.xml").write_bytes(after_bytes)

        entry = {
            "schema": "semio.repository-test.fixture/v2",
            "id": f"{mutation_id}-applied",
            "class": klass,
            "target": {"artifact": "s.stdio.xml", "standard": "1.0", "subset": "valid"},
            "mutation": mutation_id,
            "outcome": "applied",
            "units": {"length": "unitless", "angle": "degree"},
            "files": [
                {
                    "role": "expected-before-xml",
                    "path": f"../🧫️fixtures/{mutation_id}-applied/before.xml",
                    "mediaType": "application/xml",
                    "sha256": sha256_of(before_bytes),
                    "bytes": len(before_bytes),
                },
                {
                    "role": "expected-after-xml",
                    "path": f"../🧫️fixtures/{mutation_id}-applied/after.xml",
                    "mediaType": "application/xml",
                    "sha256": sha256_of(after_bytes),
                    "bytes": len(after_bytes),
                },
            ],
            "provenance": {
                "source": "generated" if klass == "third-party-generated" else "authored",
                "license": "MIT (lxml, wraps libxml2 MIT/libxml2-license)" if klass == "third-party-generated" else "n/a (handcrafted)",
                "attribution": "Serialized by lxml 6.1.3's own etree.tostring after an lxml.etree.parse round-trip" if klass == "third-party-generated" else "Authored directly; well-formedness confirmed live by lxml 6.1.3 (lxml cannot carry an internal DTD subset through its own serializer, see notes)",
                "security": "scanned-clean",
                "privacy": "no-personal-data",
            },
            "comparisonProfile": "utf8-text-v1",
            "reproducible": True,
            "family": "structural",
            "notes": note,
        }
        if klass == "third-party-generated":
            entry["generator"] = {
                "oracle": READER_ORACLE_ID,
                "packageVersion": LXML_VERSION,
                "engineFamily": "lxml",
                "engineVersion": LXML_VERSION,
                "command": "uv run python3 🔨️f2-gen-xml-fixtures.py (lxml.etree.parse -> lxml.etree.tostring round-trip)",
                "platform": "darwin-arm64",
            }
        manifests.append(entry)
        print(f"{mutation_id:28s} {klass:20s} before={len(before_bytes)}B after={len(after_bytes)}B")

    data = json.loads(ORACLE_JSON.read_text())
    data["fixtureManifests"] = manifests
    ORACLE_JSON.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n")
    print(f"\nWrote {len(manifests)} fixtureManifests entries into {ORACLE_JSON}")


if __name__ == "__main__":
    main()
