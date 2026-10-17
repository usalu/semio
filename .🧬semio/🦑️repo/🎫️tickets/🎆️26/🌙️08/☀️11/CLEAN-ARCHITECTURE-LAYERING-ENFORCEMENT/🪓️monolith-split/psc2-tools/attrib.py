"""Attributes added cruiser error lines to this slice: importer touched by a rewrite, or target created/extended by an apply. Usage: python3 attrib.py before.txt after.txt snapdir [rule-substring]"""
import re, sys, json, glob, unicodedata, collections
b, a, snapdir = sys.argv[1:4]
flt = sys.argv[4] if len(sys.argv) > 4 else None
nfc = lambda s: re.sub(r"[^\x20-\x7e]", "", s)
mine, created = set(), set()
for p in glob.glob(snapdir + "/*-rewrite.json"):
    mine |= {nfc(k) for k in json.load(open(p))}
for p in glob.glob(snapdir + "/*-apply*.json"):
    created |= {nfc(k) for k in json.load(open(p))}
def load(p):
    s = set()
    for l in open(p, encoding="utf8"):
        m = re.match(r"^  error ([^:]+): (.*) → (.*)$", nfc(l.rstrip("\n")))
        if m: s.add((m.group(1), m.group(2), m.group(3)))
    return s
B, A = load(b), load(a)
added = A - B
rows = collections.defaultdict(lambda: [0, 0, 0])
for r, f, t in added:
    k = "no-cross-package-relative-*" if r.startswith("no-cross-package-relative-") else r
    rows[k][0] += 1
    if f in mine: rows[k][1] += 1
    if t in created or any(t.endswith("/" + c.split("/")[-1]) and c in t for c in ()): rows[k][2] += 1
print("rule: added | importer touched by rewrite | target created/extended by apply")
for k, v in sorted(rows.items(), key=lambda x: -x[1][0])[:14]: print(f"  {k[:60]:60} {v[0]:5} {v[1]:5} {v[2]:5}")
if flt:
    for r, f, t in sorted(added):
        if flt in r and (f in mine or t in created): print("   +", r[:40], f[-70:], "->", t[-70:])
