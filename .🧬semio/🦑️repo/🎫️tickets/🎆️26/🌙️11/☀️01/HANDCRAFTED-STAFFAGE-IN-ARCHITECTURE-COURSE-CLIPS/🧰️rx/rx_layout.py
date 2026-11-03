import importlib
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
nk = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
for t in nk.capsule_types():
    x = y = row = 0.0
    out = []
    for i, f in enumerate(t.fittings):
        for _ in range(int(f["count"])):
            if x + f["width"] > t.interior_width + 1e-9:
                x, y, row = 0.0, y + row, 0.0
            out.append((f["key"], round(x, 2), round(y, 2), f["width"], f["length"]))
            x += f["width"]
            row = max(row, f["length"])
    print(t.key, "iw", t.interior_width, "il", t.interior_length, "occ", t.occupants, "wdiam", t.window_diameter, "wu", t.window_u)
    for o in out:
        print("   ", o, "top", round(o[2] + o[4], 2))
