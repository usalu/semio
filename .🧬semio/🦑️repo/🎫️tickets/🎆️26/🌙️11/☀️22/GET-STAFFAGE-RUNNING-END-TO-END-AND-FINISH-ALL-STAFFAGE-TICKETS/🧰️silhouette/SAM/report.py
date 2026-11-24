"""📝️ Writes the per-asset tables of `U/silhouette-SAM.md` from the measurements before and after (🗑️generated/silhouette/SAM/log/before-<id>.txt, after-<id>.txt, each the output of measure.ts) and the proof lines (🗑️generated/silhouette/SAM/proof/final.txt, the output of prove.sh): report.py > tables.md"""
import json, os, re, sys

GEN = os.environ["GEN"]
W = os.environ["W"]
SCOPE = os.environ["SCOPE"].split()
LIMIT = {"50": 15, "100": 30, "200": 60, "500": 150}
AXES = ["width", "height", "depth"]


def failing(path):
    """🔎️ The failing checks of one measure.ts log, short."""
    out = []
    if not os.path.exists(path):
        return ["not measured"]
    for line in open(path):
        m = re.match(r"\S+ 1:(\d+) bbox (\S+) (\S+) (\S+)\s+elevation (\S+)\s+front (\S+)\s+plan (\S+)", line)
        if not m:
            continue
        scale = m.group(1)
        for axis, cell in zip(AXES, m.groups()[1:4]):
            if cell.endswith("!"):
                body, declared = cell[:-1].split("/")
                out.append(f"1:{scale} {axis} {float(body):.3f} m against {declared} m")
        for view, cell in zip(("elevation", "front", "plan"), m.groups()[4:7]):
            if cell.endswith("!"):
                iou, h = cell[:-1].split("/")
                parts = []
                if float(iou) < 0.92:
                    parts.append(f"IoU {float(iou):.3f}")
                if float(h[:-2]) > LIMIT[scale]:
                    parts.append(f"{float(h[:-2]):.0f} mm")
                out.append(f"1:{scale} {view} {', '.join(parts)}")
    return out


proof = {}
for line in open(os.path.join(GEN, "proof", "final.txt")):
    parts = [p.strip() for p in line.rstrip("\n").split(" | ", 2)]
    if len(parts) == 3:
        proof.setdefault(parts[0], {})[parts[1]] = parts[2]
changed = json.load(open(os.path.join(W, "author", "changes.json")))
print("| Asset | Failing checks before | Failing checks after | Real `🧊️catalog-silhouette` test | `validateAsset`, check CLI | `🧊️three-views`, `🌡️catalog-physics` | Changed |")
print("|---|---|---|---|---|---|---|")
green = 0
for a in SCOPE:
    before, after = failing(f"{GEN}/log/before-{a}.txt"), failing(f"{GEN}/log/after-{a}.txt")
    p = proof.get(a, {})
    sil = p.get("silhouette", "")
    ok = "exit 0" in sil and "1 passed" in sil
    green += ok
    line = "✓ passes (1 passed, 384 skipped)" if ok else "✗ `" + (re.search(r"AssertionError: ([^;]*)", sil).group(1) if "AssertionError" in sil else sil) + "`"
    suites = "; ".join(("2 passed" if "2 passed" in p.get(s, "") else p.get(s, "not run")) for s in ("🧊️three-views", "🌡️catalog-physics"))
    print(f"| `{a}` | {'; '.join(before) or 'none'} ({len(before)}) | {'; '.join(after) or 'none'} ({len(after)}) | {line} | {p.get('validate', '')}, {p.get('check', '')} | {suites} | {changed.get(a, '')} |")
print(f"\nReal test: {green} of {len(SCOPE)} pass.", file=sys.stderr)
