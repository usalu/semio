#!/usr/bin/env python3
"""🗂️ Prints the remodeling lib/test closure grouped by taxonomy prefix depth: lib source, lib embedded, test-only source, test-only embedded."""
import json, sys, collections
P = "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/"
depth = int(sys.argv[1]) if len(sys.argv) > 1 else 3
prefix = sys.argv[2] if len(sys.argv) > 2 else ""
lib = {r["path"]: r for r in json.load(open("🗑️generated/lib.json"))}
test = json.load(open("🗑️generated/test.json"))
agg = collections.defaultdict(lambda: [0] * 8)
for r in test:
    p = r["path"][len(P):] if r["path"].startswith(P) else "EXT/" + r["path"]
    p = p.replace("🏅️standards/🔖️1/🪆️subsets/✳️any/", "S/")
    if not p.startswith(prefix): continue
    parts = [x for x in p.split("/") if not x.startswith("🧪️tests")] if False else p.split("/")
    g = "/".join(parts[:depth])
    i = (0 if r["path"] in lib else 4) + (0 if r["kind"] == "source" else 2)
    agg[g][i] += 1; agg[g][i + 1] += r["activeLines"] if r["kind"] == "source" else r["bytes"]
print(f'{"libF":>5}{"libL":>7}{"embF":>5}{"embB":>8} |{"tF":>5}{"tL":>7}{"teF":>5}{"teB":>9}  group')
for g, a in sorted(agg.items()):
    print(f"{a[0]:5}{a[1]:7}{a[2]:5}{a[3]:8} |{a[4]:5}{a[5]:7}{a[6]:5}{a[7]:9}  {g}")
