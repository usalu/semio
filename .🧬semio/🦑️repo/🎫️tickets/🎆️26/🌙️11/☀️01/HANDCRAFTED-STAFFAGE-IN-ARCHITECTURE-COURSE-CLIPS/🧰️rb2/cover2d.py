import sys, importlib
from collections import Counter
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
sys.path.insert(0, "C:/git/semio")
from shapely.geometry import Polygon
from shapely.ops import unary_union
api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
model = api.load_model()
slab = api._find(model, "SL-R")
voids = slab["geometry"]["finish_voids"]
owner = api._find(model, "FIN-ROOF-GRAVEL")
parts = owner["geometry"]["parts"]
polys = [(i, p["role"], Polygon(p["outline"])) for i, p in enumerate(parts) if "outline" in p]
print("owner parts", len(parts), Counter(p["role"] for p in parts), {k: v for k, v in owner["geometry"].items() if k != "parts"})
bad = []
for v in voids:
    if v["owner"] != "FIN-ROOF-GRAVEL":
        continue
    tri = Polygon(v["outline"])
    cover = unary_union([p for _, r, p in polys if r == "wearing" and p.intersects(tri)])
    miss = tri.difference(cover).area
    if miss > 1e-9:
        bad.append((v["tag"], v["owner"], tri.area, miss))
print(len(voids), "reserves;", len(bad), "with uncovered 2D area")
for b in bad[:40]:
    print(b)
print(Counter(v["owner"] for v in voids))
