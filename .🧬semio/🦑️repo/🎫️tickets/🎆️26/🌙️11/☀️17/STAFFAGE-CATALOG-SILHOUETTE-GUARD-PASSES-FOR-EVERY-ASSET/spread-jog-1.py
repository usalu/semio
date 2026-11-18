import sys; sys.path.insert(0, "."); from importlib import import_module
lib = import_module("spread-lib")
d = lib.mv.D("👤️humans/jogger")
for sc in (50, 100): lib.restore_shape(d, "395993ad470", sc, "plan", "shank-r")
for sc, sid in ((50, "fit-shank-r-50-2"), (50, "fit-shank-r-50-3"), (100, "fit-shank-r-100-3"), (200, "fit-shank-r-200-1"), (500, "fit-thigh-l-500-1")): lib.drop(d, sc, "plan", sid)
d.commit()
