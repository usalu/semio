"""📏️ Lists the drawn shapes that carry an extruded `solid` (their lateral range `z0`…`z1`) beside the world lateral range of the solids of the same name (or of the names that start with the shape id) at that scale, and marks those that differ by more than 1 mm: python3 extrusions.py <id> (reads `boxes.ts` for the world boxes)."""
import glob, json, subprocess, sys, os

ident = sys.argv[1]
here = os.path.dirname(os.path.abspath(__file__))
bun = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
asset = json.load(open(glob.glob(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/{ident}/🔣️.json")[0]))["asset"]
for scale in ("50", "100", "200", "500"):
    if asset["representations"].get(scale, {}).get("status") != "authored":
        continue
    boxes = {}
    for line in subprocess.run([bun, f"{here}/boxes.ts", ident, scale], capture_output=True, text=True).stdout.splitlines():
        name = line.split("[")[0]
        w = line.split(" world ")[1].split(" | ")[0].replace(" .. ", " ").split()
        boxes[name] = (float(w[2]), float(w[5]))
    for view in ("front", "plan", "elevation"):
        for s in asset["representations"][scale]["geometry"][view]:
            solid = s.get("solid")
            if not solid or solid.get("kind") != "extruded" or s.get("filled") is False:
                continue
            names = [n for n in boxes if n == s["id"]] or [n for n in boxes if n.startswith(s["id"])] or [n for n in boxes if s["id"].startswith(n)]
            lo = min((boxes[n][0] for n in names), default=None)
            hi = max((boxes[n][1] for n in names), default=None)
            bad = lo is None or abs(lo - solid["z0"]) > 0.001 or abs(hi - solid["z1"]) > 0.001
            print(f"{scale} {view} {s['id']}[{s['bone']}] sweep {solid.get('sweep')} {solid['z0']}..{solid['z1']} | solids {','.join(names)} {lo}..{hi}{'  !' if bad else ''}")
