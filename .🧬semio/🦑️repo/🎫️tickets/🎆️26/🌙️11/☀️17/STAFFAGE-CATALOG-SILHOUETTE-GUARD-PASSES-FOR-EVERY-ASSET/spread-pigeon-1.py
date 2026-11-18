import sys; sys.path.insert(0, "."); from importlib import import_module
lib = import_module("spread-lib")
d = lib.mv.D("🐕️animals/pigeon")
for sid, ys in (("near-toes", (0, 1)), ("far-toes", (0, 1))):
    for i in range(4):
        pass
for sid in ("near-toes", "far-toes"):
    pts = d.val("asset", "representations", "50", "geometry", "front")
    s = [x for x in pts if x["id"] == sid][0]["points"]
    for i, p in enumerate(s):
        if p[1] < -0.009: lib.pt(d, 50, "front", sid, i, 1, -0.0125)
d.commit()
