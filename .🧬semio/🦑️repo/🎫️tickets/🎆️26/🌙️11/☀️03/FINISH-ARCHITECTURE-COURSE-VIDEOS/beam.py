"""🪵 Measure roof-beam cuts through the gravel reserve that has no joint owner."""
import importlib, sys, time
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
sys.path.insert(0, r"C:\git\semio")
from shapely.geometry import Polygon
api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
TAG = "RES-FIN-ROOF-GRAVEL-SL-R-705"
started = time.time()

def log(*items):
    """📣 Print one timed probe line."""
    print(round(time.time() - started, 1), *items, flush=True)

model = api.load_model()
slab = api._find(model, "SL-R")
voids = slab["geometry"]["finish_voids"]
reserve = next(row for row in voids if row["tag"] == TAG)
tri = Polygon(reserve["outline"])
jointed = [row for row in voids if row["owner"] == "FIN-ROOF-GRAVEL" and row.get("joint_owners")]
log("jointed reserves", len(jointed))
for row in jointed:
    ring = Polygon(row["outline"])
    log(row["tag"], row["joint_owners"], "area", ring.area, "meet", ring.intersection(tri).area, "bounds", ring.bounds)
for tag in ("BEAM-PRIMARY-R-10-11", "BEAM-PRIMARY-R-34-10"):
    obj = api._find(model, tag)
    g = obj["geometry"]
    log("beam", tag, "class", obj["class"], "keys", sorted(g))
slab["geometry"]["finish_voids"] = []
log("host")
host = api._slab_geometry_uncached(model, slab, exact=True)["finish_parts"]
slab["geometry"]["finish_voids"] = [reserve, *jointed]
log("reservation")
try:
    api._finish_reservations(model, slab, host, exact=True)
    log("reservation ok")
except ValueError as error:
    log("reservation", error)
