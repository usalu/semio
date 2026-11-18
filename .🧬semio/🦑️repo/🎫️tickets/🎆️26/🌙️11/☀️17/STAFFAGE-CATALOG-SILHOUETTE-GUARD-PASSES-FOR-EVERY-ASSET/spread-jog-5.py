import sys; sys.path.insert(0, "."); from importlib import import_module
lib = import_module("spread-lib")
for a in sys.argv[1:]:
    op, rev, sc, view, sid = a.split(":")
    d = lib.mv.D("👤️humans/jogger")
    if op == "restore": lib.restore_shape(d, rev, int(sc), view, sid)
    elif op == "reinsert": lib.reinsert(d, rev, int(sc), view, sid)
    elif op == "drop": lib.drop(d, int(sc), view, sid)
    d.commit()
