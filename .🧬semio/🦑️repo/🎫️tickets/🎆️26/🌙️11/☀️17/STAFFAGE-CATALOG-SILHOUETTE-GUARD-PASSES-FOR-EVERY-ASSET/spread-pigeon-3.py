import sys; sys.path.insert(0, "."); from importlib import import_module
lib = import_module("spread-lib")
a = float(sys.argv[1]); b = float(sys.argv[2])
d = lib.mv.D("🐕️animals/pigeon")
for sid, x0, x1 in (("near-toes", -0.00063 - a, 0.02538 + b), ("far-toes", -0.01693 - a, 0.00898 + b)):
    for i, x in ((0, x0), (1, x1), (2, x1), (3, x0)):
        lib.pt(d, 50, "front", sid, i, 0, x)
d.commit()
