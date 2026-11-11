import importlib
import os
import sys
import time
from pathlib import Path

root = Path.cwd()
here = Path(__file__).resolve().parent
sys.path.insert(0, str(root))
os.environ.setdefault("SEMIO_BIM_SLAB_CACHE", str(here / "🗑️generated" / "full-slab-v2"))
api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
cache = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.♻️cache.🐍️")
model = api.load_model()
slabs = [item for item in model["elements"] if item["class"] == "IfcSlab"]
folder = Path(os.environ["SEMIO_BIM_SLAB_CACHE"])
folder.mkdir(parents=True, exist_ok=True)
if list(folder.glob("*.json")):
    raise RuntimeError("Cold validation requires a dedicated empty folder")
print("[DEBUG]", {"slabs": [slab["tag"] for slab in slabs]}, flush=True)
for slab in slabs:
    events = []
    def progress(event):
        if event.get("phase") == "slab-cache":
            events.append(event["status"])
            print("[DEBUG]", {"slab": slab["tag"], **event}, flush=True)
    api._SLAB_GEOMETRIES.clear()
    start = time.monotonic()
    cold = api._slab_geometry_entry(model, slab, exact=True, progress=progress)
    cold_hash = cache.digest(cache.encode(cold))
    cold_seconds = time.monotonic() - start
    api._SLAB_GEOMETRIES.clear()
    events.clear()
    start = time.monotonic()
    warm = api._slab_geometry_entry(model, slab, exact=True, progress=progress)
    warm_seconds = time.monotonic() - start
    assert events.count("hit") >= 1 and cold == warm and cold_hash == cache.digest(cache.encode(warm))
    print("[DEBUG]", {"slab": slab["tag"], "sha256": cold_hash, "cold_seconds": cold_seconds, "warm_seconds": warm_seconds, "warm_hits": events.count("hit")}, flush=True)
