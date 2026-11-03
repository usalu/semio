import sys, time, importlib
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
sys.path.insert(0, "C:/git/semio")
api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
model = api.load_model()
slab = api._find(model, "SL-R")
t = time.time()
last = [0]
def progress(e):
    if e.get("phase") == "finish-reservations" and e["completed"] % 50 == 0 and e["completed"] != last[0]:
        last[0] = e["completed"]; print(round(time.time() - t), e, flush=True)
try:
    api._slab_geometry_uncached(model, slab, exact=True, progress=progress)
    print("OK", round(time.time() - t))
except Exception as error:
    print("ERR", round(time.time() - t), str(error)[:300])
