import sys; sys.path.insert(0, "."); from importlib import import_module
lib = import_module("spread-lib")
d = lib.mv.D("👤️humans/jogger")
for a in sys.argv[1:]:
    sc, view, sid = a.split(":"); lib.reinsert(d, "HEAD", int(sc), view, sid)
d.commit()
