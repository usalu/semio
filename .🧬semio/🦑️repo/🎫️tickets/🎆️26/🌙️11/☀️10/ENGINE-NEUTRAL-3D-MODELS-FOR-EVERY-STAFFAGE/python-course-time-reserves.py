import importlib, sys, time
sys.path.insert(0, "/Users/ueli/Documents/semio")
core = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
model = core.load_model()
tag, limit = sys.argv[1], int(sys.argv[2])
start = time.time()
marks = {}
class Done(Exception): pass
def progress(event):
    if event.get("phase") == "finish-reservations":
        marks[event["completed"]] = time.time() - start
        if event["completed"] >= limit: raise Done
try:
    core._slab_geometry_uncached(model, core._find(model, tag), exact=True, progress=progress)
except Done:
    pass
print("[DEBUG]", {k: round(v, 1) for k, v in marks.items() if k % 25 == 0 or k == limit}, flush=True)
