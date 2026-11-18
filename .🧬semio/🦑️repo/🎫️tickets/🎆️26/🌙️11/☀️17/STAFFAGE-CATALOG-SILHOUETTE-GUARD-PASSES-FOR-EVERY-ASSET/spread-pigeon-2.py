import sys; sys.path.insert(0, "."); from importlib import import_module
lib = import_module("spread-lib")
d = lib.mv.D("🐕️animals/pigeon")
for sid, x0, x1, y0 in (("near-toes", -0.0223, 0.0471, -0.00996), ("far-toes", -0.0385, 0.0306, -0.01074)):
    for i in (0, 1):
        lib.pt(d, 50, "front", sid, i, 1, y0)
    for i, x in ((0, x0), (1, x1), (2, x1), (3, x0)):
        lib.pt(d, 50, "front", sid, i, 0, x)
d.commit()
