import sys; sys.path.insert(0, "."); from importlib import import_module
lib = import_module("spread-lib")
d = lib.mv.D("👤️humans/jogger")
for a in sys.argv[1:]:
    op, sc, view, sid = a.split(":")
    if op == "old": lib.restore_shape(d, "395993ad470", int(sc), view, sid)
    elif op == "drop": lib.drop(d, int(sc), view, sid)
d.commit()
