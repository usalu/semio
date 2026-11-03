import sys, importlib.util
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
sys.path.insert(0, "C:/git/semio")
p = "C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🧭️cutter-winding.py"
s = importlib.util.spec_from_file_location("cw", p); m = importlib.util.module_from_spec(s); s.loader.exec_module(m)
m.test_inward_wound_sliver_removes_only_its_own_volume(); print("PASS")
import pickle
part, cutters, _ = pickle.load(open("C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🗑️generated/rb2/capture.pkl", "rb"))
stock, cutter = m._solid(m.CASE["stock"], "s"), m._solid(m.CASE["cutter"], "c")
print("pickle faces", m.api.part_faces(cutters[1]) == cutter["mesh"]["faces"], "keys", list(part), list(cutters[1]))
print("pickle route", float(m.api._native_parts_exact_volume(m.api._reference_material_parts(part, cutters[1:]))))
print("json route", float(m.api._native_parts_exact_volume(m.api._reference_material_parts(stock, [cutter]))))
print("stock vs part verts", m.api.part_vertices(part)[:2], stock["mesh"]["vertices"][:2])
