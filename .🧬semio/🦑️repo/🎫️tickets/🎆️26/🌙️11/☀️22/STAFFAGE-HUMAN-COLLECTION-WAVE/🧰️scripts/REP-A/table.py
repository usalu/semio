"""📊️ The before/after table of the repair report from the measure logs of HW-03/measure.ts (🗑️generated/REP-A/before-<slug>.log and after-<slug>.log): failing silhouette views (bbox, IoU below 0.92, boundary distance above two features) and `checkVolumes` findings per rule. python3 table.py > table.md"""
import collections, os, re

HERE = os.path.dirname(os.path.abspath(__file__))
GEN = os.path.join(HERE, "..", "..", "🗑️generated", "REP-A")


def read(kind, slug):
    views, rules = [], collections.Counter()
    path = os.path.join(GEN, f"{kind}-{slug}.log")
    if not os.path.exists(path):
        return None
    for line in open(path, encoding="utf-8"):
        m = re.match(rf"{slug} 1:(\d+) bbox (.*?)  (.*)", line)
        if m:
            if "!" in m.group(2):
                views.append(f"1:{m.group(1)} bbox")
            for part in m.group(3).split("  "):
                if part.endswith("!"):
                    name, value = part[:-1].split(" ")
                    views.append(f"1:{m.group(1)} {name} {value}")
        m = re.match(r"\s+(volumes-[a-z-]+) ", line)
        if m:
            rules[m.group(1).replace("volumes-", "")] += 1
    return views, rules


def cell(data):
    if data is None:
        return "not measured", "not measured"
    views, rules = data
    return ("; ".join(views) or "none"), (", ".join(f"{n} {r}" for r, n in sorted(rules.items())) or "none")


print("| Person | Silhouette views failing before | after | checkVolumes findings before | after |")
print("|---|---|---|---|---|")
for slug in open(os.path.join(HERE, "people.txt")).read().split():
    b, a = cell(read("before", slug)), cell(read("after", slug))
    print(f"| `{slug}` | {b[0]} | {a[0]} | {b[1]} | {a[1]} |")
