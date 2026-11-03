"""PyNite cross-check of the simple-span point force at `xM`: M = F a b / L, V = F max(a, b) / L (python -I en1994-point-load-oracle.py)."""
from Pynite import FEModel3D

L, F, A = 8.0, 120e3, 2.0
m = FEModel3D()
m.add_node("A", 0, 0, 0)
m.add_node("B", L, 0, 0)
m.add_material("S", 210e9, 80e9, 0.3, 7850)
m.add_section("sec", 1e-2, 1e-4, 1e-4, 2e-4)
m.add_member("M", "A", "B", "S", "sec")
m.def_support("A", True, True, True, True, False, False)
m.def_support("B", False, True, True, True, False, False)
m.add_member_pt_load("M", "Fy", -F, A)
m.analyze()
member = m.members["M"]
moment = max(abs(member.max_moment("Mz")), abs(member.min_moment("Mz")))
shear = max(abs(member.max_shear("Fy")), abs(member.min_shear("Fy")))
assert abs(moment - F * A * (L - A) / L) < 1e-6 * moment, moment
assert abs(shear - F * max(A, L - A) / L) < 1e-6 * shear, shear
print("PyNite M = %.1f kNm, V = %.1f kN" % (moment / 1e3, shear / 1e3))
