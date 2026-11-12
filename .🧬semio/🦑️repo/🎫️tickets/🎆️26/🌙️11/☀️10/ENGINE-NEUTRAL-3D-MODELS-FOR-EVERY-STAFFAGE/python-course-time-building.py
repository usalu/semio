import importlib, sys, time
sys.path.insert(0, "/Users/ueli/Documents/semio")
core = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
start = time.time()
core.building_geometry()
print("[DEBUG] building_geometry cold", round(time.time() - start, 1), flush=True)
start = time.time()
core.building_geometry()
print("[DEBUG] building_geometry second call in the same process", round(time.time() - start, 3), flush=True)
