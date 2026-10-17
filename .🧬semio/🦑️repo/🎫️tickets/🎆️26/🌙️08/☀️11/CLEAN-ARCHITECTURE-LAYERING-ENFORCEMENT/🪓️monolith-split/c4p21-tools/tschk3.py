"""🔎️ Lossless check of the remodel stories split: stripped non-blank line multiset of the snapshot minus the new module tree (JSON-moved data excluded)."""
import collections, glob, json, os
F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/"
live = json.load(open(F + "c4p21-tools/files.json", encoding="utf-8"))["f3"]
snap = open(F + "🗑️generated/c4p21-snap/f3.ts", encoding="utf-8").read().split("\n")
tree = [live] + sorted(glob.glob(os.path.dirname(live) + "/*/🟦️.ts"))
c = collections.Counter(l.strip() for l in snap if l.strip())
a = collections.Counter()
for f in tree:
    a.update(l.strip() for l in open(f, encoding="utf-8").read().split("\n") if l.strip())
lost, added = c - a, a - c
print("files", len(tree), "snapshot lines", len(snap) - 1, "lost", sum(lost.values()), "added", sum(added.values()))
json_lines = 0
for l, n in lost.items():
    print("  LOST", n, l[:90])
