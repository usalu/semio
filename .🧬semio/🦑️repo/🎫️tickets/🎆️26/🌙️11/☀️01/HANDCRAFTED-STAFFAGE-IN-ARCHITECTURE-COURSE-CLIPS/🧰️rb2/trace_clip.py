import sys, importlib
CAPTURE="C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🗑️generated/rb2/capture.pkl"
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
sys.path.insert(0, "C:/git/semio")
api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
model = api.load_model()
slab = api._find(model, "SL-R")
slab["geometry"]["finish_voids"] = [v for v in slab["geometry"]["finish_voids"] if v["tag"] == "RES-FIN-ROOF-GRAVEL-SL-R-705"]
orig = api._reference_material_parts
vol = api._native_parts_exact_volume
def b(p):
    vs = [tuple(float(c) for c in v) for v in api.part_vertices(p)]
    return tuple(round(f(v[k] for v in vs), 4) for k in range(3) for f in (min, max))
def spy(part, cutters, bounds=(), progress=None, cancel=None):
    r = orig(part, cutters, bounds, progress=progress, cancel=cancel)
    pb = b(part)
    if pb[0] < 8.1 and pb[1] > 8.03 and pb[2] < 13 and pb[3] > 10.7 and pb[4] > 6.4 and pb[5] < 6.7 and (part.get("role") == "weathering_reserve" or len(bounds)):
        vin = float(vol([part]))
        print("CALL role", part.get("role"), "owner", part.get("owner"), "bounds", pb, "cutters", len(cutters), "bnd", len(bounds), "vin", round(vin, 8), "vout", round(float(vol(r)), 8), "cells", len(r), flush=True)
        for c in bounds: print("    bound", b(c), round(float(vol([c])), 8))
        for c in cutters[:3]: print("    cutter", b(c), c.get("owner"), c.get("role"))
    if len(cutters)==2 and abs(float(vol([part]))-0.00838671)<1e-7:
        import pickle
        pickle.dump((part,cutters,r),open(CAPTURE,"wb"))
    return r
api._reference_material_parts = spy
try:
    api._slab_geometry_uncached(model, slab, exact=True)
except Exception as e:
    print("ERR", str(e)[:100])
