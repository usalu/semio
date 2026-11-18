import sys; sys.path.insert(0, "."); from importlib import import_module
lib = import_module("spread-lib")
d = lib.mv.D("🚜️vehicles/city-bicycle")
for sid in ("sweep-l", "sweep-r"): lib.restore_shape(d, "HEAD", 200, "plan", sid)
d.commit()
lim = 0.27
d = lib.mv.D("🚜️vehicles/city-bicycle")
for sid, sg in (("sweep-l", 1), ("sweep-r", -1)):
    pts = [s for s in d.val("asset", "representations", "200", "geometry", "plan") if s["id"] == sid][0]["points"]
    for i, p in enumerate(pts):
        if abs(p[1]) > lim: lib.pt(d, 200, "plan", sid, i, 1, sg * lim)
d.commit()
