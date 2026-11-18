import sys; sys.path.insert(0, "."); from importlib import import_module
lib = import_module("spread-lib")
REV = "395993ad470"
plan = sys.argv[1].split(",") if sys.argv[1] else []
front = sys.argv[2].split(",") if sys.argv[2] else []
scales = [int(x) for x in sys.argv[3].split(",")]
d = lib.mv.D("🐕️animals/horse")
for sc in scales:
    for view, ids in (("plan", plan), ("front", front)):
        for sid in ids:
            try: lib.restore_shape(d, REV, sc, view, sid)
            except KeyError as e: print("skip", sc, view, sid)
d.commit()
