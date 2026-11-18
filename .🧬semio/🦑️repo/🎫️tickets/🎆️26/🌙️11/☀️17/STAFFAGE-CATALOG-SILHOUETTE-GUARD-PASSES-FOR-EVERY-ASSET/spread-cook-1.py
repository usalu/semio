import sys; sys.path.insert(0, "."); from importlib import import_module
lib = import_module("spread-lib")
d = lib.mv.D("👤️humans/person-cooking")
lib.shift(d, 200, "plan", "head", float(sys.argv[1]), float(sys.argv[2]))
d.commit()
