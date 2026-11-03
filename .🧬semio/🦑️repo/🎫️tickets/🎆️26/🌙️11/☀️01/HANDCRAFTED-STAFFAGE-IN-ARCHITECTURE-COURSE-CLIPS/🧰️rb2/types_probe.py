import sys, importlib, pickle
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
sys.path.insert(0, "C:/git/semio")
api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
part, cutters, _ = pickle.load(open("C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🗑️generated/rb2/capture.pkl", "rb"))
from collections import Counter
for label, p in (("A", part), ("B0", cutters[0]), ("B1", cutters[1])):
    print(label, Counter(type(c).__name__ for v in p["mesh"]["vertices"] for c in v), Counter(type(c).__name__ for v in api.part_vertices(p) for c in v), "outline" in p)
print("A outline types", Counter(type(c).__name__ for v in part["outline"] for c in v), part["z0"], part["z1"])
import json, importlib.util
from fractions import Fraction
p = "C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🧭️cutter-winding.py"
s = importlib.util.spec_from_file_location("cw", p); m = importlib.util.module_from_spec(s); s.loader.exec_module(m)
J = m.CASE
sA, sB = m._solid(J["stock"], "s"), m._solid(J["cutter"], "c")
print("A mesh equal", part["mesh"]["vertices"] == sA["mesh"]["vertices"], part["mesh"]["faces"] == sA["mesh"]["faces"])
print("B1 mesh equal", cutters[1]["mesh"]["vertices"] == sB["mesh"]["vertices"], cutters[1]["mesh"]["faces"] == sB["mesh"]["faces"])
vol = lambda r: float(api._native_parts_exact_volume(r))
print("pickleA+pickleB", vol(api._reference_material_parts(part, [cutters[1]])))
print("jsonA+pickleB", vol(api._reference_material_parts(sA, [cutters[1]])))
print("pickleA+jsonB", vol(api._reference_material_parts(part, [sB])))
print("jsonA+jsonB", vol(api._reference_material_parts(sA, [sB])))
print("pickleA+B0+B1", vol(api._reference_material_parts(part, cutters)))
print("jsonA+B0+B1", vol(api._reference_material_parts(sA, cutters)))
