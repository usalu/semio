"""🔮️ Third-party oracle for the mechanics kernel: PyNiteFEA, anastruct and scipy reproduce the kernel's reference cases.

Prints `key value` lines. Every case is defined here independently of the Rust sources; Pynite works Y-up, so the
Rust Z-up geometry maps to Pynite as (x, y, z) -> (x, z, -y).
"""
import math
import warnings

import numpy as np
from scipy import integrate, linalg

E, G, RHO = 210e9, 81e9, 7850.0
A, IY, IZ, J = 0.01, 8e-5, 8e-5, 1e-4
OUT = []
warnings.simplefilter("ignore")


def emit(key, value):
    OUT.append(f"{key} {float(value)!r}")


def pynite_model(nodes, members, supports):
    from Pynite import FEModel3D

    m = FEModel3D()
    m.add_material("S", E, G, 0.3, RHO)
    m.add_section("X", A, IY, IZ, J)
    for name, (x, y, z) in nodes.items():
        m.add_node(name, x, z, -y)
    for name, (a, b) in members.items():
        m.add_member(name, a, b, "S", "X")
    for name, flags in supports.items():
        m.def_support(name, *flags)
    return m


def beam_and_frame():
    from Pynite import FEModel3D

    length, w = 6.0, 12e3
    m = pynite_model({"a": (0, 0, 0), "b": (length, 0, 0)}, {"m": ("a", "b")}, {"a": (True, True, True, True, False, False), "b": (False, True, True, False, False, False)})
    m.add_member_dist_load("m", "FY", -w, -w)
    m.analyze_linear(check_stability=False)
    emit("pynite_ss_udl_mid", abs(m.members["m"].deflection("dy", length / 2)))
    emit("pynite_ss_udl_reaction", m.nodes["a"].RxnFY["Combo 1"])

    h, span, p = 4.0, 6.0, 20e3
    m = pynite_model({"a": (0, 0, 0), "b": (0, 0, h), "c": (span, 0, h), "d": (span, 0, 0)}, {"l": ("a", "b"), "r": ("b", "c"), "s": ("c", "d")}, {"a": (True,) * 6, "d": (True,) * 6})
    m.add_node_load("b", "FX", p)
    m.analyze_linear(check_stability=False)
    emit("pynite_portal_sway", m.nodes["b"].DX["Combo 1"])
    emit("pynite_portal_base_moment", abs(m.nodes["a"].RxnMZ["Combo 1"]))

    l1, l2, fz, fx = 3.0, 2.0, 10e3, 4e3
    m = pynite_model({"a": (0, 0, 0), "b": (l1, 0, 0), "c": (l1, l2, 0)}, {"m1": ("a", "b"), "m2": ("b", "c")}, {"a": (True,) * 6})
    m.add_node_load("c", "FY", -fz)
    m.add_node_load("c", "FX", fx)
    m.analyze_linear(check_stability=False)
    n = m.nodes["c"]
    emit("pynite_lframe_ux", n.DX["Combo 1"])
    emit("pynite_lframe_uy", -n.DZ["Combo 1"])
    emit("pynite_lframe_uz", n.DY["Combo 1"])

    from anastruct import SystemElements

    s = SystemElements()
    s.add_element([[0, 0], [length, 0]], EA=E * A, EI=E * IZ)
    s.add_support_hinged(1)
    s.add_support_roll(2)
    s.q_load(q=-w, element_id=1)
    s.solve()
    emit("anastruct_ss_udl_mid_node_check", abs(s.get_node_displacements(node_id=2)["uy"]))
    s = SystemElements()
    for i in range(8):
        s.add_element([[length * i / 8, 0], [length * (i + 1) / 8, 0]], EA=E * A, EI=E * IZ)
    s.add_support_hinged(1)
    s.add_support_roll(9)
    s.q_load(q=-w, element_id=list(range(1, 9)))
    s.solve()
    emit("anastruct_ss_udl_mid", abs(s.get_node_displacements(node_id=5)["uy"]))

    s = SystemElements()
    s.add_element([[0, 0], [0, h]], EA=E * A, EI=E * IZ)
    s.add_element([[0, h], [span, h]], EA=E * A, EI=E * IZ)
    s.add_element([[span, h], [span, 0]], EA=E * A, EI=E * IZ)
    s.add_support_fixed([1, 4])
    s.point_load(node_id=2, Fx=p)
    s.solve()
    emit("anastruct_portal_sway", s.get_node_displacements(node_id=2)["ux"])


def element_matrices(n_el, length):
    from Pynite import FEModel3D

    m = FEModel3D()
    m.add_material("S", E, G, 0.3, RHO)
    m.add_section("X", A, IY, IZ, J)
    for i in range(n_el + 1):
        m.add_node(f"n{i}", length * i / n_el, 0, 0)
    for i in range(n_el):
        m.add_member(f"m{i}", f"n{i}", f"n{i + 1}", "S", "X")
    m.def_support("n0", True, True, True, True, False, False)
    m.add_node_load(f"n{n_el}", "FY", 1.0)
    m.analyze_linear(check_stability=False)
    size = 6 * (n_el + 1)
    K = np.zeros((size, size))
    Kg = np.zeros((size, size))
    for i in range(n_el):
        mem = next(iter(m.members[f"m{i}"].sub_members.values()))
        sl = slice(6 * i, 6 * i + 12)
        K[sl, sl] += np.array(mem.ke())
        Kg[sl, sl] += np.array(mem.kg(1.0))
    return K, Kg, size


def buckling():
    length, n_el = 5.0, 4
    K, Kg, size = element_matrices(n_el, length)
    fixed = {0, 1, 2, 3, 6 * n_el + 1, 6 * n_el + 2}
    free = [i for i in range(size) if i not in fixed]
    vals = linalg.eigh(Kg[np.ix_(free, free)], K[np.ix_(free, free)], eigvals_only=True)
    emit("pynite_pinned_column_alpha_4el", sorted(1.0 / v for v in vals if v > 1e-12)[0])
    emit("euler_pinned", math.pi**2 * E * IZ / length**2)


def modal():
    length, n_el = 6.0, 16
    K, _, size = element_matrices(n_el, length)
    h = length / n_el
    c = RHO * A * h / 420.0
    plane = np.array([[156, 22 * h, 54, -13 * h], [22 * h, 4 * h * h, 13 * h, -3 * h * h], [54, 13 * h, 156, -22 * h], [-13 * h, -3 * h * h, -22 * h, 4 * h * h]]) * c
    M = np.zeros((size, size))
    for i in range(n_el):
        for a, ia in enumerate([1, 5, 7, 11]):
            for b, ib in enumerate([1, 5, 7, 11]):
                M[6 * i + ia, 6 * i + ib] += plane[a, b]
    fixed = {0, 1, 2, 3, 6 * n_el + 1, 6 * n_el + 2}
    free = [i for i in range(size) if i not in fixed and any(M[i, :] != 0)]
    vals = np.sort(linalg.eigh(K[np.ix_(free, free)], M[np.ix_(free, free)], eigvals_only=True))
    freqs = np.sqrt(vals) / (2 * math.pi)
    emit("pynite_k_scipy_ss_modal_f1", float(freqs[0]))
    emit("pynite_k_scipy_ss_modal_f2", float(freqs[1]))
    emit("closed_form_ss_modal_f1", math.pi / (2 * length**2) * math.sqrt(E * IZ / (RHO * A)))
    emit("closed_form_ss_modal_f2", 4 * math.pi / (2 * length**2) * math.sqrt(E * IZ / (RHO * A)))


def ltb_galerkin(case, length, iz, iw, j, moment, line, points, height, terms=14):
    sin = lambda i, x: math.sin(i * math.pi * x / length)
    d2 = lambda i, x: -((i * math.pi / length) ** 2) * sin(i, x)
    d1 = lambda i, x: (i * math.pi / length) * math.cos(i * math.pi * x / length)
    Kv = np.zeros((terms, terms))
    Kf = np.zeros((terms, terms))
    Gc = np.zeros((terms, terms))
    Gq = np.zeros((terms, terms))
    pts = sorted({0.0, length, *[p for p, _ in points]})
    q = lambda x: line(x)
    for a in range(1, terms + 1):
        for b in range(1, terms + 1):
            Kv[a - 1, b - 1] = E * iz * integrate.quad(lambda x: d2(a, x) * d2(b, x), 0, length)[0]
            Kf[a - 1, b - 1] = integrate.quad(lambda x: E * iw * d2(a, x) * d2(b, x) + G * j * d1(a, x) * d1(b, x), 0, length)[0]
            Gc[a - 1, b - 1] = integrate.quad(lambda x: moment(x) * d2(a, x) * sin(b, x), 0, length, points=pts[1:-1] or None, limit=200)[0]
            Gq[a - 1, b - 1] = -height * (integrate.quad(lambda x: q(x) * sin(a, x) * sin(b, x), 0, length, limit=200)[0] + sum(p * sin(a, x0) * sin(b, x0) for x0, p in points))
    Kfull = np.block([[Kv, np.zeros((terms, terms))], [np.zeros((terms, terms)), Kf]])
    Gfull = np.block([[np.zeros((terms, terms)), Gc], [Gc.T, Gq]])
    vals = linalg.eigh(-Gfull, Kfull, eigvals_only=True)
    pos = sorted(1.0 / v for v in vals if v > 1e-12)
    return pos[0]


def ltb():
    length = 5.0
    iz, iw, j = 6.04e-6, 1.26e-7, 2.01e-7
    cases = {
        "uniform": (lambda x: 1.0, lambda x: 0.0, [], 0.0),
        "udl_shear_centre": (lambda x: x * (length - x) / 2.0, lambda x: 1.0, [], 0.0),
        "udl_top_flange": (lambda x: x * (length - x) / 2.0, lambda x: 1.0, [], 0.15),
        "point_shear_centre": (lambda x: min(x, length - x) / 2.0, lambda x: 0.0, [(length / 2, 1.0)], 0.0),
        "point_top_flange": (lambda x: min(x, length - x) / 2.0, lambda x: 0.0, [(length / 2, 1.0)], 0.15),
    }
    for name, (moment, line, points, height) in cases.items():
        lam = ltb_galerkin(name, length, iz, iw, j, moment, line, points, height)
        emit(f"ltb_{name}_alpha", lam)
    mcr = math.pi / length * math.sqrt(E * iz * G * j) * math.sqrt(1 + math.pi**2 * E * iw / (length**2 * G * j))
    emit("ltb_uniform_closed_form", mcr)


if __name__ == "__main__":
    beam_and_frame()
    buckling()
    modal()
    ltb()
    print("\n".join(OUT))
