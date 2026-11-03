import sys, time, importlib, copy
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
sys.path.insert(0, "C:/git/semio")
api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
model = api.load_model()
tags = sys.argv[1:] or ["RES-FIN-ROOF-GRAVEL-SL-R-705"]
slab = api._find(model, "SL-R")
all_voids = slab["geometry"]["finish_voids"]
slab["geometry"]["finish_voids"] = [v for v in all_voids if v["tag"] in tags]
print("reserves", [v["tag"] for v in slab["geometry"]["finish_voids"]], flush=True)
t = time.time()
def progress(e):
    print(round(time.time() - t, 1), e, flush=True)
try:
    api._slab_geometry_uncached(model, slab, exact=True, progress=progress)
    print("OK", round(time.time() - t, 1))
except Exception as error:
    print("ERR", round(time.time() - t, 1), error)
