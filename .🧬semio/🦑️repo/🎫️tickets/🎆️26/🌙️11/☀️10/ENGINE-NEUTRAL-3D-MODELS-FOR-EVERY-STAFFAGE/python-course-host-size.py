import importlib, sys
sys.path.insert(0, "/Users/ueli/Documents/semio")
core = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
model = core.load_model()
original = core._finish_reservations
def wrapper(model, slab, parts, *args, **kwargs):
    print("[DEBUG] parts", len(parts), "finish_voids", len(slab["geometry"]["finish_voids"]), flush=True)
    cells = [cell for part in parts for cell in core._native_material_cells({**part, "material": part.get("material") or "x", "role": "finish"})]
    print("[DEBUG] host cells", len(cells), flush=True)
    raise SystemExit
core._finish_reservations = wrapper
core._slab_geometry_uncached(model, core._find(model, "SL-R"), exact=True)
