"""📝️ Assembles the repair report of the people of HW-01 … HW-09 (umbrella ticket, humans-a-repair.md) from the measure logs (before-/after-<slug>.log of HW-03/measure.ts), the suite logs (suite-*.log of suite.sh), the three-views log and the text blocks in report-text.txt. python3 report.py <out.md>"""
import collections, glob, os, re, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
GEN = os.path.join(HERE, "..", "..", "🗑️generated", "REP-A")
people = open(os.path.join(HERE, "people.txt")).read().split()


def measured(kind, slug):
    views, rules, lines = [], collections.Counter(), []
    path = os.path.join(GEN, f"{kind}-{slug}.log")
    if not os.path.exists(path):
        return None
    for line in open(path, encoding="utf-8"):
        m = re.match(rf"{slug} 1:(\d+) bbox (.*?)  (.*)", line)
        if m:
            lines.append(line.strip())
            if "!" in m.group(2):
                views.append(f"1:{m.group(1)} bbox")
            for part in m.group(3).split("  "):
                if part.endswith("!"):
                    name, value = part[:-1].split(" ")
                    views.append(f"1:{m.group(1)} {name} {value}")
        m = re.match(r"\s+(volumes-[a-z-]+) (\S+): (.*)", line)
        if m:
            rules[m.group(1).replace("volumes-", "")] += 1
            lines.append(f"{m.group(1)} {m.group(2)} {m.group(3)[:230].strip()}")
    return views, rules, lines


def cell(data):
    if data is None:
        return "not measured", "not measured"
    return ("; ".join(data[0]) or "none"), (", ".join(f"{n} {r}" for r, n in sorted(data[1].items())) or "none")


out = [open(os.path.join(HERE, "report-text.txt"), encoding="utf-8").read().rstrip(), "", "## Per person: failing gates before and after", "",
       "Measured with `K/🧰️scripts/HW-03/measure.ts` (the same `measureAsset` call and limits as the real test, plus `checkVolumes` of every rule). A silhouette entry reads scale, view, IoU/largest boundary distance.", "",
       "| Person | Silhouette views failing before | after | checkVolumes findings before | after |", "|---|---|---|---|---|"]
red = []
for slug in people:
    b, a = measured("before", slug), measured("after", slug)
    cb, ca = cell(b), cell(a)
    out.append(f"| `{slug}` | {cb[0]} | {ca[0]} | {cb[1]} | {ca[1]} |")
    if a and (a[0] or a[1]):
        red.append((slug, a))
out += ["", "## Final test lines", ""]
for path in sorted(glob.glob(os.path.join(GEN, "suite-*.log"))):
    text = open(path, encoding="utf-8", errors="replace").read()
    total = re.findall(r"^\s+Tests\s+(.*)$", text, re.M)
    out.append(f"- `{os.path.basename(path)[6:-4]}`: {total[-1].strip() if total else 'no summary'}")
    for line in re.findall(r"^\s+→ (.*)$", text, re.M)[:8]:
        out.append(f"  - {line.strip()[:260]}")
views = os.path.join(GEN, "three-views-final.log")
if os.path.exists(views):
    out += ["", "Three-views guard (`REP-A/views.ts`, the sweep and consistency functions of `🧪️tests/🧊️three-views/🧰️.ts`), final catalog files:", ""]
    out += [f"- {line.strip()}" for line in open(views, encoding="utf-8") if "three-views findings" in line]
out += ["", "## What stays red, with the measurement", ""]
if not red:
    out.append("Nothing in the silhouette guard or `checkVolumes`.")
for slug, a in red:
    out.append(f"- `{slug}`:")
    for line in a[2]:
        if "!" in line or line.startswith("volumes-"):
            out.append(f"  - {line[:330]}")
out += ["", "## Ops per person (design inputs applied after the worker's generator)", "", "| Person | ops files (stage-name: number of ops) |", "|---|---|"]
import json
for slug in people:
    files = sorted(glob.glob(os.path.join(HERE, "ops", f"{slug}.*.json")))
    out.append(f"| `{slug}` | {', '.join(f'{os.path.basename(f)[len(slug) + 1:-5]}: {len(json.load(open(f)))}' for f in files) or 'none (generator output unchanged)'} |")
open(sys.argv[1], "w", encoding="utf-8").write("\n".join(out) + "\n")
print(sys.argv[1], len(out), "lines")
