#!/usr/bin/env python3
"""🛂️ Independent oracle of the check-ownership rule: owning subject, embedded id (longest run, then leftmost), first user of a referenced row, documented document-level checks. The cases equal the Rust unit tests of `🦀️.rs`. Usage: python3 -I 🐍️.py"""
import json

DOCUMENT = {"projectId": "p", "members": [{"id": "m1", "sectionId": "s1"}, {"id": "m2"}], "sections": [{"id": "s1"}, {"id": "s2"}]}


def rows(document):
    return {name: [r for r in value if isinstance(r, dict) and "id" in r] for name, value in document.items() if isinstance(value, list)}


def names(row, ident):
    return any(v == ident or (isinstance(v, dict) and names(v, ident)) or (isinstance(v, list) and any(names(i, ident) if isinstance(i, dict) else i == ident for i in v)) for k, v in row.items() if k != "id")


def file_checks(document, owning, prefixes, checks):
    tables = rows(document)
    owners = [r for name, rs in tables.items() if not owning or name in owning for r in rs]
    ids = {r["id"] for r in owners}
    referenced = {r["id"] for name, rs in tables.items() if owning and name not in owning for r in rs}
    out = []
    for check_id, subject in checks:
        if subject in ids:
            out.append(subject)
            continue
        parts = check_id.split(".")
        embedded = next((c for length in range(len(parts), 0, -1) for start in range(len(parts) - length + 1) for c in [".".join(parts[start:start + length])] if c in ids), None)
        user = next((r["id"] for r in owners if r["id"] != subject and names(r, subject)), None) if subject in referenced else None
        owner = embedded or user
        out.append(owner if owner else ("" if any(check_id.startswith(p) for p in prefixes) else subject))
    return out


cases = [
    (["members"], ["g."], [("en.shear.m2", "m1")], ["m1"]),
    (["members"], ["g."], [("en.6.2.shear.m2", ""), ("en.6.2.shear.m2", "p"), ("en.m1.m2", "")], ["m2", "m2", "m1"]),
    (["members"], ["g."], [("en.section.s1", "s1"), ("en.section.s2", "s2")], ["m1", "s2"]),
    (["members"], ["g."], [("g.annex", "p"), ("en.other", "p")], ["", "p"]),
    ([], [], [("en.shear.s2", "")], ["s2"]),
]
for owning, prefixes, checks, want in cases:
    got = file_checks(DOCUMENT, owning, prefixes, checks)
    assert got == want, (checks, got, want)
print(json.dumps({"cases": len(cases)}))
