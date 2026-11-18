import sys; sys.path.insert(0, "."); from importlib import import_module
lib = import_module("spread-lib")
spec = [a.split(":") for a in sys.argv[2:]]
d = lib.mv.D("🐕️animals/horse")
for rev, view, sc, ids in ((a[0], a[1], int(a[2]), a[3].split(",")) for a in spec):
    for sid in ids:
        try: lib.restore_shape(d, "395993ad470" if rev == "old" else "3cde99d423d", sc, view, sid)
        except KeyError: print("skip", sc, view, sid)
d.commit()
