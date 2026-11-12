import importlib, sys, time
sys.path.insert(0, "/Users/ueli/Documents/semio")
core = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
model = core.load_model()
for slab in [item for item in model["elements"] if item["class"] == "IfcSlab"]:
    start = time.time()
    status = []
    core._slab_geometry_entry(model, slab, exact=True, progress=lambda event: status.append(event) if event.get("phase") == "slab-cache" else None)
    print("[DEBUG]", slab["tag"], round(time.time() - start, 1), [event["status"] for event in status], flush=True)
