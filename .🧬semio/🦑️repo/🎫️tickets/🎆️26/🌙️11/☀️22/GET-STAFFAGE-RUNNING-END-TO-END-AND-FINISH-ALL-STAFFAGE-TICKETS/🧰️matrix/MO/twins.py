"""👯️ The shapes of given bones that a band draws under one id in more than one view, one line per id with kind, point count and authored box per view, and the count of the other shapes per view: `python3 twins.py <asset.json> <bone,bone> [scale,scale]`."""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import box
a = json.load(open(sys.argv[1]))["asset"]
bones = set(sys.argv[2].split(","))
want = sys.argv[3].split(",") if len(sys.argv) > 3 else None
for k, r in a["representations"].items():
    if (want and k not in want) or "geometry" not in r:
        continue
    by = {}
    for view, shapes in r["geometry"].items():
        for s in shapes:
            if s["bone"] in bones:
                by.setdefault((s["bone"], s["id"]), {})[view] = s
    lonely = {}
    for (bone, sid), views in by.items():
        if len(views) < 2:
            lonely.setdefault(bone, []).append(f"{sid}:{list(views)[0][:1]}")
            continue
        parts = []
        for view in ("elevation", "front", "plan"):
            s = views.get(view)
            if s is None:
                parts.append(f"{view[:1]} -")
                continue
            try:
                b = " ".join(f"{v:.3f}" for v in box(s))
            except Exception:
                b = "curved"
            n = len(s.get("points", [])) or ""
            parts.append(f"{view[:1]} {s['kind'][:4]}{n}{'*' + (s['solid'].get('sweep') or 'r') if 'solid' in s else ''}{'' if s.get('filled') else '○'} {b}")
        print(f"1:{k} {sid} [{bone}] | " + " | ".join(parts))
    for bone, ids in lonely.items():
        print(f"1:{k} [{bone}] single-view: {' '.join(ids)[:300]}")
