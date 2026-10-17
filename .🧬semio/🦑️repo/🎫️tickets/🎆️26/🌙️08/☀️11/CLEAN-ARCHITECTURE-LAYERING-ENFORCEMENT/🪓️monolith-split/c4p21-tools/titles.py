"""🔎 Test-title multiset and line-multiset check of the cad suite split.

usage: titles.py   (compares the snapshot f1.ts with the live root plus its child folders)
"""
import collections, glob, json, os, re

F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
live = json.load(open(F + "/c4p21-tools/files.json", encoding="utf-8"))["f1"]
old = open(F + "/🗑️generated/c4p21-snap/f1.ts", encoding="utf-8").read()
new_files = [live] + sorted(glob.glob(os.path.dirname(live) + "/*/🟦️.ts"))
new = "\n".join(open(p, encoding="utf-8").read() for p in new_files)
T = re.compile(r"\b(describe|it|test)\(\s*(\"(?:[^\"\\]|\\.)*\"|'(?:[^'\\]|\\.)*'|`(?:[^`\\]|\\.)*`)")
def titles(text):
    return collections.Counter((m.group(1), m.group(2)) for m in T.finditer(text))
a, b = titles(old), titles(new)
print("titles before %d after %d; lost %d added %d" % (sum(a.values()), sum(b.values()), sum((a - b).values()), sum((b - a).values())))
def lines(text):
    return collections.Counter(l.strip() for l in text.split("\n") if l.strip())
la, lb = lines(old), lines(new)
lost, added = la - lb, lb - la
print("lines (stripped non-blank): lost %d added %d" % (sum(lost.values()), sum(added.values())))
for t, n in lost.items():
    print("  LOST", n, t[:160])
for t, n in list(added.items())[:60]:
    print("  ADDED", n, t[:160])
assert a == b, "title multisets differ"
print("TITLES IDENTICAL")
