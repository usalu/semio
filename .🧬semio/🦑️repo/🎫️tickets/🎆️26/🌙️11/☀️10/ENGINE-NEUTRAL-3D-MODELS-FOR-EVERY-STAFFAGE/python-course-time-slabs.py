import importlib, sys, time
sys.path.insert(0, "/Users/ueli/Documents/semio")
core = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
model = core.load_model()
start = time.time()
def progress(event):
    print("[DEBUG]", round(time.time() - start, 1), event, flush=True)
for tag in sys.argv[1:]:
    start = time.time()
    core._slab_geometry_uncached(model, core._find(model, tag), exact=True, progress=progress)
    print(tag, round(time.time() - start, 1), flush=True)
