"""🧭 Run the finish reserves whose bounds meet the short gravel window, in source order."""
import importlib, sys, time
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
sys.path.insert(0, r"C:\git\semio")
api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
TAG = "RES-FIN-ROOF-GRAVEL-SL-R-705"
started = time.time()

def log(*items):
    """📣 Print one timed probe line."""
    print(round(time.time() - started, 1), *items, flush=True)

def bounds(ring):
    """📦 Return the axis-aligned bounds of one authored outline."""
    xs = [p[0] for p in ring]
    ys = [p[1] for p in ring]
    return min(xs), min(ys), max(xs), max(ys)

def meets(a, b, margin=0.0):
    """🤝 Report whether two planar bounds can interact."""
    return a[0] - margin < b[2] and b[0] - margin < a[2] and a[1] - margin < b[3] and b[1] - margin < a[3]

model = api.load_model()
slab = api._find(model, "SL-R")
voids = slab["geometry"]["finish_voids"]
index = next(i for i, row in enumerate(voids) if row["tag"] == TAG)
window = bounds(voids[index]["outline"])
chosen = [row for row in voids[:index + 1] if meets(bounds(row["outline"]), window, 0.0) or row.get("joint_owners")]
log("window", window, "chosen", len(chosen), "through", index)
log("owners", sorted({row["owner"] for row in chosen}))
slab["geometry"]["finish_voids"] = []
host = api._slab_geometry_uncached(model, slab, exact=True)["finish_parts"]
log("host", len(host))
slab["geometry"]["finish_voids"] = chosen
def progress(event):
    """📣 Keep the reserve phase visible."""
    if event.get("phase") == "finish-reservations":
        log("reserve", event.get("completed"), event.get("total"), event.get("tag"))
    elif event.get("phase") == "finish-owner" and event.get("status") == "completed":
        log("owner", event.get("tag"), "cells", event.get("cells"))
try:
    api._finish_reservations(model, slab, host, exact=True, progress=progress)
    log("ok")
except ValueError as error:
    log("ERR", error)
