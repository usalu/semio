#!/usr/bin/env python3
"""Appends conversions to specs/ARTIFACT.json from a compact python table (typed enums v4)."""
import json, sys
def add(art, rows):
    p = f"/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/typed-enums/specs/{art}.json"
    try:
        d = json.load(open(p, encoding="utf-8"))
    except FileNotFoundError:
        d = {"artifact": art, "conversions": []}
    have = {c["enum"] for c in d["conversions"]}
    for hosts, enum, doc, default, variants in rows:
        if enum in have:
            continue
        vs = []
        for v in variants:
            name, en = v[0], v[1]
            de = v[2] if len(v) > 2 else en
            legacy = v[3] if len(v) > 3 else [name.lower()]
            vs.append([name, legacy, en, de])
        d["conversions"].append({"hosts": hosts.split(","), "enum": enum, "doc": doc, "default": default, "variants": vs})
    json.dump(d, open(p, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
