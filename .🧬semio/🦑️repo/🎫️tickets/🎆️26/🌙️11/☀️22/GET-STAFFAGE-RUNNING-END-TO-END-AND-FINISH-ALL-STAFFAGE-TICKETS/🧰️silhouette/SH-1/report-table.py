"""🧾️ The per-person table of the report of batch SH-1: failing checks before (the baseline measure) and after (the proof measure), the proof line and what the ops of the person change, as Markdown on stdout.
python3 report-table.py <before.txt> <proof dir> <proof.txt>"""
import collections, json, os, re, sys

before_file, proof_dir, proof_file = sys.argv[1:4]
HERE = os.path.dirname(os.path.abspath(__file__))
LIMIT = {"50": 15, "100": 30, "200": 60, "500": 150}


def failing(lines):
    out = []
    for line in lines:
        m = re.match(r"(\S+) 1:(\d+) bbox (\S+) (\S+) (\S+)\s+(.*)", line)
        if not m:
            continue
        scale = m.group(2)
        for axis, cell in zip(("width", "height", "depth"), m.group(3, 4, 5)):
            if cell.endswith("!"):
                body, declared = cell[:-1].split("/")
                out.append(f"1:{scale} {axis} {float(body):.3f}/{declared} m")
        for view, iou, h in re.findall(r"(elevation|front|plan) ([\d.]+!?)/([\d.]+mm!?)", m.group(6)):
            if iou.endswith("!"):
                out.append(f"1:{scale} {view} IoU {float(iou[:-1]):.3f}")
            if h.endswith("!"):
                out.append(f"1:{scale} {view} {float(h[:-3]):.1f} mm")
    return out


before = collections.defaultdict(list)
for line in open(before_file, encoding="utf-8"):
    if not line.startswith(" "):
        before[line.split(" ")[0].rstrip(":")].append(line)
proof = {l.split(" | ")[0]: l.strip().split(" | ") for l in open(proof_file, encoding="utf-8") if " | " in l}
print("| Person | Failing checks before | Failing checks after | Proof (measure, `validateAsset`, check CLI) | Changed |")
print("|---|---|---|---|---|")
for slug in proof:
    after = failing(open(os.path.join(proof_dir, f"{slug}-measure.log"), encoding="utf-8").read().splitlines())
    ops = json.load(open(os.path.join(HERE, "ops", f"{slug}.json"), encoding="utf-8")) if os.path.exists(os.path.join(HERE, "ops", f"{slug}.json")) else []
    c = collections.Counter()
    for o in ops:
        g = str(o.get("group", ""))
        if o["op"] == "dims":
            c["dims " + ", ".join(f"{k} {v}" for k, v in o["set"].items())] += 1
        elif o["op"].startswith("solid"):
            c["volume solids changed or added"] += 1
        elif o["op"] == "shape-add" and g.startswith("plate|"):
            c["outlines traced from the volume (plates)"] += 1
        elif o["op"] == "shape-add" and g.startswith("prism|"):
            c["solids drawn as prisms"] += 1
        elif o["op"] == "shape-add":
            c["shapes added"] += 1
        elif o["op"] == "shape-remove":
            c["shapes given way"] += 1
        else:
            c["shapes redrawn or refitted"] += 1
    changed = "; ".join(f"{n} {k}" if not k.startswith("dims") else k for k, n in c.items()) or "nothing"
    p = proof[slug]
    print(f"| `{slug}` | {'; '.join(failing(before[slug])) or 'none'} | {'; '.join(after) or 'none'} | {p[1].split(': ')[1]}, {p[2]}, {p[3][:1]} | {changed} |")
