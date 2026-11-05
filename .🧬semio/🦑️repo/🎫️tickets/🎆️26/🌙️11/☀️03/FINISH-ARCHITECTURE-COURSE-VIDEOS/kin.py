"""🔎 List gravel reserves and joint owners that meet the short roof window."""
import importlib, sys
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
sys.path.insert(0, r"C:\git\semio")
from shapely.geometry import Polygon
api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
TAG = "RES-FIN-ROOF-GRAVEL-SL-R-705"
model = api.load_model()
slab = api._find(model, "SL-R")
voids = slab["geometry"]["finish_voids"]
index = next(i for i, row in enumerate(voids) if row["tag"] == TAG)
tri = Polygon(voids[index]["outline"])
print("index", index, "of", len(voids), "area", tri.area, flush=True)
overlap = []
joints = {}
for i, row in enumerate(voids):
    owners = row.get("joint_owners") or []
    if owners:
        joints.setdefault(row["owner"], set()).update(owners)
    if i >= index or row["owner"] != "FIN-ROOF-GRAVEL":
        continue
    other = Polygon(row["outline"])
    area = tri.intersection(other).area
    if area > 0:
        overlap.append((row["tag"], round(area, 8), owners, row["outline"]))
print("earlier gravel overlaps", len(overlap), "area", sum(a for _, a, _, _ in overlap), flush=True)
for tag, area, owners, _ in overlap:
    print(" ", tag, area, owners, flush=True)
print("joint owners by reserve owner", {k: sorted(v) for k, v in joints.items()}, flush=True)
later = []
for row in voids[index + 1:]:
    if row["owner"] != "FIN-ROOF-GRAVEL":
        continue
    area = tri.intersection(Polygon(row["outline"])).area
    if area > 0:
        later.append((row["tag"], round(area, 8), row.get("joint_owners")))
print("later gravel overlaps", len(later), "area", sum(a for _, a, _ in later), flush=True)
for item in later[:40]:
    print(" ", item, flush=True)
print("705 joint", voids[index].get("joint_owners"), flush=True)
