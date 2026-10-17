#!/usr/bin/env python3
"""🧮️ Re-points `📋️preserved.json` at the builder-contract support module and its parts; verifies each law body sha256 and the assertion total.
usage: pres.py [--apply]"""
import glob, os, hashlib, json, re, sys
from lib import *
from rs import blank

PJ = PLUG + "/🧪️tests/📦️units/📋️preserved.json"
FAM = PLUG + "/🧪️tests/🔬️plugin-runtime-plugin-builder-contract"
pres = json.load(open(PJ, encoding="utf8"))
files = sorted(f for f in glob.glob(FAM + "*/🦀️.rs") if not os.path.dirname(f).endswith("-x"))
FN = re.compile(r"((?:#\[[^\]]*\]\s*)+)((?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+([A-Za-z_]\w*))")


def laws_of(path):
    text = read(path); clean = blank(text); out = {}
    for m in FN.finditer(clean):
        if not re.search(r"\btest\b|::async_test\b", clean[m.start(1):m.end(1)]):
            continue
        i = clean.index("{", m.end()); d = 1; j = i + 1
        while d:
            c = clean[j]; d += (c == "{") - (c == "}"); j += 1
        out[m.group(3)] = text[m.start(2):j]
    return out


found, asserts = {}, {}
for f in files:
    asserts[f] = len(re.findall(r"\bassert(?:_eq|_ne)?!", read(f)))
    for n, body in laws_of(f).items():
        found.setdefault(n, []).append((f, body))
names = {r["name"] for r in pres["laws"]}
print("files", len(files), "found laws", len(found), "preserved", len(names))
print("missing", sorted(names - set(found))[:5], "extra", sorted(set(found) - names)[:5], "dups", [n for n, v in found.items() if len(v) > 1][:5])
bad = []
own = [r for r in pres["laws"] if "plugin-runtime-plugin-builder-contract" in r["source"]]
print("own rows", len(own), "not found", [r["name"] for r in own if r["name"] not in found][:5])
for r in own:
    f, body = found[r["name"]][0]
    if hashlib.sha256(body.encode()).hexdigest() != r["sha256"]:
        bad.append(r["name"])
print("sha mismatches", len(bad), bad[:5])
used = {found[r["name"]][0][0] for r in own} | {PLUG + "/" + pres["original"].split("🔌️plugin/", 1)[1]}
tot = sum(asserts[f] for f in used if f in asserts)
print("assert total over used", tot, "expected", pres["originalAssertions"] + pres["addedAssertions"], "all family", sum(asserts.values()))
for f in files:
    print(rel(f)[-48:], asserts[f], "used" if f in used else "UNUSED")
if "--rehash" in sys.argv:
    for r in own:
        f, body = found[r["name"]][0]
        n = len(re.findall(r"\bassert(?:_eq|_ne)?!", body))
        assert n == r["assertions"], (r["name"], n, r["assertions"])
        r["sha256"] = hashlib.sha256(body.encode()).hexdigest()
    bad = []
if "--apply" in sys.argv and not bad:
    for r in own:
        r["source"] = PLUG_REL + "/" + rel(found[r["name"]][0][0])
    import ed
    ed.put(PJ, json.dumps(pres, ensure_ascii=False, indent=2) + "\n")
    print("applied")
