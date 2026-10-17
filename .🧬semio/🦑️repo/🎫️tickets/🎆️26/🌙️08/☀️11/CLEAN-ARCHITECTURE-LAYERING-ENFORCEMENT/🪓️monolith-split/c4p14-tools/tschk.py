"""🔎️ Lossless check of the three split TypeScript targets: the stripped non-blank line multiset of each snapshot minus its new module tree, plus sizes and path lengths."""
import collections, json, os
R = "/Users/ueli/Documents/semio/"
T = R + ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/"
files = json.load(open(T + "c4p14-tools/files.json"))
new = [l.rstrip("\n") for l in open(T + "c4p14-tools/ts-new-files.txt") if l.strip()]
longest = 0
for key in ("energy_mut", "pdf_diff", "gltf_gen"):
    root = files[key]
    rd = os.path.dirname(root)
    tree = [R + n for n in new if (R + n).startswith(rd + "/")] if key != "energy_mut" else [R + n for n in new if (R + n).startswith(rd + "/")]
    tree = sorted(set(tree) | {root})
    snap = open(T + "🗑️generated/c4p14-snap/%s.ts" % key, encoding="utf-8").read().split("\n")
    c = collections.Counter(l.strip() for l in snap if l.strip())
    a = collections.Counter()
    sizes = []
    for f in tree:
        longest = max(longest, len(f[len(R):].encode()))
        ls = open(f, encoding="utf-8").read().split("\n")
        sizes.append((sum(1 for _ in ls) - 1, os.path.relpath(f, rd)))
        a.update(l.strip() for l in ls if l.strip())
    lost = c - a
    added = a - c
    print(key, "files", len(tree), "snapshot lines", len(snap) - 1, "max", max(sizes)[0], "root", [s for s, n in sizes if n == os.path.basename(root)][0])
    print("  lost", sum(lost.values()), "added", sum(added.values()))
    for l, n in list(lost.items())[:8]: print("  LOST", n, l[:150])
    for l, n in list(added.items())[:60]: print("  ADDED", n, l[:130])
print("longest relative path bytes", longest)
