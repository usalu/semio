"""EN 1999 v5 references of the three new rules, third-party: ring stiffener composite section (sectionproperties + shapely), transverse web load fixed point (scipy brentq) and category Miner damage with the critical stress-range scale (scipy brentq). Prints the numbers asserted by the Rust unit tests; `--selftest` checks the closed forms. Usage: /home/deck/semio/.venv/bin/python -I wave5_ref.py [--selftest]"""
import math, sys
from scipy.optimize import brentq
from shapely.geometry import box
from sectionproperties.pre.geometry import Geometry
from sectionproperties.analysis import Section

E = 70_000e6


def ring_closed(shell_t, b_e, depth, bar_t):
    a_s, a_b = b_e * shell_t, depth * bar_t
    z_b = 0.5 * shell_t + 0.5 * depth
    z_c = a_b * z_b / (a_s + a_b)
    inertia = b_e * shell_t**3 / 12 + a_s * z_c**2 + bar_t * depth**3 / 12 + a_b * (z_b - z_c) ** 2
    return a_s + a_b, inertia, z_c


def ring_sectionproperties(shell_t, b_e, depth, bar_t):
    strip = box(-b_e / 2, -shell_t / 2, b_e / 2, shell_t / 2)
    bar = box(-bar_t / 2, shell_t / 2, bar_t / 2, shell_t / 2 + depth)
    union = strip.union(bar)
    geo = Geometry(geom=union)
    geo.create_mesh(mesh_sizes=[(b_e * shell_t) / 400])
    sec = Section(geo)
    sec.calculate_geometric_properties()
    area = sec.get_area()
    (_, cy) = sec.get_c()
    ixx = sec.get_ic()[0]
    return area, ixx, cy


def transverse(h_w, t_w, t_f, b_f, f_o, s_s, a, k=(6.0, 2.0, 0.9, 0.5, 0.02), gamma=1.1):
    k0, k1, c, chi0, m2f = k
    k_f = k0 + k1 * (h_w / a) ** 2 if a > 0 else k0
    m1 = f_o * b_f / (f_o * t_w)
    f_cr = c * k_f * E * t_w**3 / h_w

    def l_of(m2):
        l = s_s + 2 * t_f * (1 + math.sqrt(m1 + m2))
        return min(l, a) if a > 0 else l

    def lam(l):
        return math.sqrt(l * t_w * f_o / f_cr)

    l0 = l_of(0.0)
    if lam(l0) > 0.5:
        l = brentq(lambda x: x - l_of(m2f * (h_w / t_f) ** 2), 1e-9, 1e3)
    else:
        l = l0
    chi = min(chi0 / lam(l), 1.0)
    return dict(k_f=k_f, l_y=l, f_cr=f_cr, lam=lam(l), chi=chi, l_eff=chi * l, f_rd=f_o * chi * l * t_w / gamma)


NC, ND, NL = 2e6, 5e6, 1e8


def n_allow(ds, dc, m1, m2):
    dd = dc * (NC / ND) ** (1 / m1)
    dl = dd * (ND / NL) ** (1 / m2)
    if ds >= dd:
        return NC * (dc / ds) ** m1
    if ds < dl:
        return math.inf
    return ND * (dd / ds) ** m2


def miner(blocks, dc, m1, m2, gmf=1.0):
    return sum(n / n_allow(ds * gmf, dc, m1, m2) for ds, n in blocks if not math.isinf(n_allow(ds * gmf, dc, m1, m2)))


def scale_for(blocks, dc, m1, m2, target=0.98):
    return brentq(lambda s: miner([(ds * s, n) for ds, n in blocks], dc, m1, m2) - target, 1e-6, 1.0)


if __name__ == "__main__":
    ring = (0.010, 0.78 * math.sqrt(0.5 * 0.010), 0.060, 0.006)
    area, inertia, z_c = ring_closed(*ring)
    print("ring closed form", area, inertia, z_c)
    sp_area, sp_i, sp_c = ring_sectionproperties(*ring)
    print("ring sectionproperties", sp_area, sp_i, sp_c)
    t = transverse(0.120, 0.006, 0.010, 0.080, 260e6, 0.05, 0.0)
    print("transverse", t)
    blocks = [(120e6, 1.0e6), (70e6, 3.0e6), (40e6, 5.0e6)]
    print("miner", miner(blocks, 71e6, 4.3, 6.3), "scale", scale_for(blocks, 71e6, 4.3, 6.3))
    if "--selftest" in sys.argv:
        assert abs(sp_area - area) / area < 1e-6
        assert abs(sp_i - inertia) / inertia < 5e-3, (sp_i, inertia)
        assert t["chi"] <= 1.0 and t["f_rd"] > 0
        print("selftest ok")
