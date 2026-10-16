"""🧮️ Multiset delta of two dependency-cruiser `err` outputs, with new violations filtered to the relocation slices' areas.

Usage: python3 cruiser_delta.py <before.txt> <after.txt>
"""
import collections, re, sys

AREAS = ["🛂️manifest", "🎓️onboarding", "⏪️time-travel", "⏯️tool-run", "🖐️gumball-audience", "🗺️tiled-map", "🎛️map-policy", "🌍️gis", "🌎️hub/💡️inference", "🌎️hub/🧩️compositions/🌍️gis", "♾️infinite/🎲️board", "Board2dHost", "🎲️board2d", "🔌️plugin/🧵️instance", "🔌️plugin/🖥️host", "🎬️scene", "styling", "TiledMapHost", "🗣️Interpreter", "🧑‍🎨engine/🧱️elements/🛠️ShellHelpers", "WasmSessionLoader"]


def lines(path):
    out = []
    for raw in open(path, encoding="utf-8"):
        m = re.match(r"\s*(error|warn) (\S+): (.+?) → (.+)$", raw.rstrip())
        if m:
            out.append((m.group(2), m.group(3), m.group(4)))
    return collections.Counter(out)


before, after = lines(sys.argv[1]), lines(sys.argv[2])
added, removed = after - before, before - after
rule = lambda c: collections.Counter({k[0]: 0 for k in c} | {}) 
per = collections.Counter()
for (r, _, _), n in added.items():
    per[r] += n
for (r, _, _), n in removed.items():
    per[r] -= n
print(f"before {sum(before.values())} after {sum(after.values())} added {sum(added.values())} removed {sum(removed.values())}")
print("## per-rule net delta (nonzero, top 25)")
for r, n in sorted(per.items(), key=lambda kv: -abs(kv[1]))[:25]:
    if n:
        print(f"{n:+6d} {r}")
hit = lambda k: any(a in k[1] or a in k[2] for a in AREAS)
print("## added in relocation areas")
for k, n in sorted(added.items()):
    if hit(k):
        print(f"+{n} {k[0]}: {k[1]} -> {k[2]}")
print("## removed in relocation areas")
for k, n in sorted(removed.items()):
    if hit(k):
        print(f"-{n} {k[0]}: {k[1]} -> {k[2]}")
