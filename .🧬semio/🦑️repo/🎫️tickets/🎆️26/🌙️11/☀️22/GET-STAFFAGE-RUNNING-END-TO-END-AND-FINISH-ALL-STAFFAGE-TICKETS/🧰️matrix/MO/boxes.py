"""🗃️ The shapes of given bones per band and view in one short line each (id, kind, authored box, solid, fill): `python3 boxes.py <asset.json> <bone,bone> [scale,scale]`."""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import box
a = json.load(open(sys.argv[1]))["asset"]
bones = set(sys.argv[2].split(","))
want = sys.argv[3].split(",") if len(sys.argv) > 3 else None
for k, r in a["representations"].items():
    if want and k not in want:
        continue
    for view, shapes in r.get("geometry", {}).items():
        for s in shapes:
            if s["bone"] not in bones:
                continue
            try:
                b = " ".join(f"{v:.3f}" for v in box(s))
            except Exception:
                b = "curved path"
            solid = s.get("solid")
            print(f"1:{k} {view[:4]} {s['id']} [{s['bone']}] {s['kind']} {b}{' solid ' + (solid.get('sweep') or solid.get('axis')) + ' ' + str(solid.get('z0', '')) + ' ' + str(solid.get('z1', '')) if solid else ''}{'' if s.get('filled') else ' open' if s.get('closed') is False else ' unfilled'}{' followTip' if s.get('followTip') else ''}")
