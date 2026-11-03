"""🕸️ Oracle of the general planar frame eigenvalue alpha_cr (EN 1993-1-1 5.2.1) for the portal showcase and a cantilever column: an independent Euler-Bernoulli beam-column assembly (3 dof per node, cubic Hermite stiffness and consistent geometric stiffness of the static axial forces) solved with scipy.linalg.eigh as (-Kg) phi = mu K phi, alpha = 1/mu, plus the closed forms of the cantilever (pi^2 E I / (4 L^2)) and of the fixed-base portal (stability functions, root finding with scipy.optimize.brentq). Run with `python3 -I` from the project virtual environment."""
import json
import math

import numpy as np
from scipy.linalg import eigh
from scipy.optimize import brentq

E = 210e9
A = 0.0106
I = 1.126e-4
SEGMENTS = 4


def stiffness(length):
    ea, ei = E * A / length, E * I
    k = np.zeros((6, 6))
    k[np.ix_([0, 3], [0, 3])] = ea * np.array([[1, -1], [-1, 1]])
    b = ei / length**3 * np.array([[12, 6 * length, -12, 6 * length], [6 * length, 4 * length**2, -6 * length, 2 * length**2], [-12, -6 * length, 12, -6 * length], [6 * length, 2 * length**2, -6 * length, 4 * length**2]])
    k[np.ix_([1, 2, 4, 5], [1, 2, 4, 5])] = b
    return k


def geometric(length, n):
    g = np.zeros((6, 6))
    g[np.ix_([1, 2, 4, 5], [1, 2, 4, 5])] = n / (30 * length) * np.array([[36, 3 * length, -36, 3 * length], [3 * length, 4 * length**2, -3 * length, -length**2], [-36, -3 * length, 36, -3 * length], [3 * length, -length**2, -3 * length, 4 * length**2]])
    return g


def rotation(c, s):
    r = np.zeros((6, 6))
    for o in (0, 3):
        r[o:o + 3, o:o + 3] = [[c, s, 0], [-s, c, 0], [0, 0, 1]]
    return r


def frame_alpha(nodes, bars, fixed, loads):
    """🧾️ nodes: {id: (x, z)}, bars: [(a, b)], fixed: {id: (ux, uz, rot)} booleans, loads: {id: (fx, fz)}; returns the sorted positive alpha_cr values."""
    pts, links = dict(nodes), []
    for a, b in bars:
        (xa, za), (xb, zb) = nodes[a], nodes[b]
        chain = [a] + [f"{a}-{b}@{k}" for k in range(1, SEGMENTS)] + [b]
        for k in range(1, SEGMENTS):
            pts[chain[k]] = (xa + (xb - xa) * k / SEGMENTS, za + (zb - za) * k / SEGMENTS)
        links += list(zip(chain[:-1], chain[1:]))
    ids = list(pts)
    index = {n: i for i, n in enumerate(ids)}
    size = 3 * len(ids)
    K = np.zeros((size, size))
    elements = []
    for a, b in links:
        (xa, za), (xb, zb) = pts[a], pts[b]
        length = math.hypot(xb - xa, zb - za)
        r = rotation((xb - xa) / length, (zb - za) / length)
        dofs = [3 * index[a] + i for i in range(3)] + [3 * index[b] + i for i in range(3)]
        K[np.ix_(dofs, dofs)] += r.T @ stiffness(length) @ r
        elements.append((dofs, length, r))
    F = np.zeros(size)
    for n, (fx, fz) in loads.items():
        F[3 * index[n]] += fx
        F[3 * index[n] + 1] += fz
    held = [3 * index[n] + i for n, flags in fixed.items() for i, f in enumerate(flags) if f]
    free = [d for d in range(size) if d not in held]
    u = np.zeros(size)
    u[free] = np.linalg.solve(K[np.ix_(free, free)], F[free])
    G = np.zeros((size, size))
    for dofs, length, r in elements:
        local = r @ u[dofs]
        n = E * A / length * (local[3] - local[0])
        G[np.ix_(dofs, dofs)] += r.T @ geometric(length, n) @ r
    mu = eigh(-G[np.ix_(free, free)], K[np.ix_(free, free)], eigvals_only=True)
    return sorted(1.0 / m for m in mu if m > 1e-12)


def stability(k):
    s = (k * math.sin(k) - k * k * math.cos(k)) / (2 - 2 * math.cos(k) - k * math.sin(k))
    c = (k * k - k * math.sin(k)) / (k * math.sin(k) - k * k * math.cos(k))
    return s, c


def portal_closed_form(h, span, p):
    kappa = 6 * h / span

    def f(k):
        s, c = stability(k)
        return s * (1 + c) * (s * (1 + c) / (s + kappa) - 2) + k * k

    grid = np.linspace(1e-3, math.pi - 1e-3, 4000)
    for a, b in zip(grid[:-1], grid[1:]):
        if f(a) < 0 <= f(b):
            k = brentq(f, a, b)
            return k * k * E * I / (h * h * p)
    raise ValueError("no root")


def pynite_second_order(nodes, bars, supports, node_loads, bar_loads):
    """🧾️ Third-party second-order oracle: PyNite `analyze_PDelta` (P-Delta) on the same frame with every bar split into SEGMENTS members (so P-delta of the bar is resolved), nodes {id: (x, z)} in the PyNite x-y plane, supports {id: (dx, dy, rz)}, node_loads {id: (fx, fz)}, bar_loads {(a, b): q} local transverse N/m; returns the displacement of every node and the peak |M| per bar."""
    from Pynite import FEModel3D

    m = FEModel3D()
    m.add_material("S", E, E / 2.6, 0.3, 7850.0)
    m.add_section("X", A, I, I, 1.0e-6)
    for n, (x, z) in nodes.items():
        m.add_node(n, x, z, 0.0)
    chains = {}
    for a, b in bars:
        (xa, za), (xb, zb) = nodes[a], nodes[b]
        chain = [a] + [f"{a}-{b}@{k}" for k in range(1, SEGMENTS)] + [b]
        for k in range(1, SEGMENTS):
            m.add_node(chain[k], xa + (xb - xa) * k / SEGMENTS, za + (zb - za) * k / SEGMENTS, 0.0)
        chains[(a, b)] = chain
        for k, (i, j) in enumerate(zip(chain[:-1], chain[1:])):
            m.add_member(f"{a}-{b}#{k}", i, j, "S", "X")
            if (a, b) in bar_loads:
                m.add_member_dist_load(f"{a}-{b}#{k}", "Fy", bar_loads[(a, b)], bar_loads[(a, b)])
    for n, (dx, dy, rz) in supports.items():
        m.def_support(n, dx, dy, True, True, True, rz)
    for n, (fx, fz) in node_loads.items():
        m.add_node_load(n, "FX", fx)
        m.add_node_load(n, "FY", fz)
    m.analyze_PDelta(check_stability=False)
    moments = {}
    for (a, b), chain in chains.items():
        parts = [m.members[f"{a}-{b}#{k}"] for k in range(SEGMENTS)]
        moments[(a, b)] = max(max(abs(p.min_moment("Mz", "Combo 1")), abs(p.max_moment("Mz", "Combo 1"))) for p in parts)
    return {n: (float(m.nodes[n].DX["Combo 1"]), float(m.nodes[n].DY["Combo 1"])) for n in nodes}, moments


def sway_phi(h, columns):
    """📐️ EN 1993-1-1 5.3.2(3): phi = phi_0 alpha_h alpha_m with phi_0 = 1/200, 2/3 <= alpha_h = 2/sqrt(h) <= 1, alpha_m = sqrt(0.5 (1 + 1/m))."""
    return 0.005 * min(1.0, max(2.0 / 3.0, 2.0 / math.sqrt(h))) * math.sqrt(0.5 * (1.0 + 1.0 / columns))


def second_order_cases():
    """🧾️ Imperfect second-order frames: (1) the fixed-base portal under 200 kN per eaves node with the equivalent sway forces phi N (phi from h = 4 m, m = 2), (2) a pinned bar of 8 m with 1000 kN and the bow load 8 N e0 / L^2 for e0 = L/250 (curve b, elastic), also against the closed form M = q/k^2 (sec(k L / 2) - 1)."""
    h_load = sway_phi(4.0, 2) * 200e3
    nodes = {"a": (0.0, 0.0), "b": (0.0, 4.0), "c": (6.0, 4.0), "d": (6.0, 0.0)}
    shifts, moments = pynite_second_order(nodes, [("a", "b"), ("b", "c"), ("d", "c")], {"a": (True, True, True), "d": (True, True, True)}, {"b": (h_load, -200e3), "c": (h_load, -200e3)}, {})
    length, n_ed = 8.0, 1.0e6
    q = 8.0 * n_ed * (length / 250.0) / length**2
    _, bow = pynite_second_order({"a": (0.0, 0.0), "b": (length, 0.0)}, [("a", "b")], {"a": (True, True, False), "b": (False, True, False)}, {"b": (-n_ed, 0.0)}, {("a", "b"): q})
    k = math.sqrt(n_ed / (E * I))
    return {
        "swayForce": h_load,
        "portalSwayShift": shifts["b"][0],
        "portalColumnMoment": moments[("a", "b")],
        "bowLoad": q,
        "bowMoment": bow[("a", "b")],
        "bowMomentClosedForm": q / k**2 * (1.0 / math.cos(k * length / 2.0) - 1.0),
    }


def semi_rigid_portal(spring, lateral=0.0, vertical=-200e3):
    """🔗️ Fixed-base portal (h = 4 m, span 6 m) whose beam ends are joined to the columns by rotational springs of stiffness `spring` (N m/rad): one extra rotation dof per beam end (exact spring, no connector element), static sway of `lateral` per eaves node and sway alpha_cr of the vertical eaves loads. Returns (sway of node b, lowest alpha_cr)."""
    nodes = {"a": (0.0, 0.0), "b": (0.0, 4.0), "c": (6.0, 4.0), "d": (6.0, 0.0)}
    bars = [("a", "b"), ("b", "c"), ("d", "c")]
    ids = list(nodes)
    index = {n: i for i, n in enumerate(ids)}
    ghost = {("b", 1): 3 * len(ids), ("c", 1): 3 * len(ids) + 1}
    size = 3 * len(ids) + 2
    K = np.zeros((size, size))
    elements = []
    for k, (a, b) in enumerate(bars):
        (xa, za), (xb, zb) = nodes[a], nodes[b]
        length = math.hypot(xb - xa, zb - za)
        r = rotation((xb - xa) / length, (zb - za) / length)
        dofs = [3 * index[a] + i for i in range(3)] + [3 * index[b] + i for i in range(3)]
        if k == 1:
            dofs[2], dofs[5] = ghost[("b", 1)], ghost[("c", 1)]
        K[np.ix_(dofs, dofs)] += r.T @ stiffness(length) @ r
        elements.append((dofs, length, r))
    for end, extra in (("b", ghost[("b", 1)]), ("c", ghost[("c", 1)])):
        main = 3 * index[end] + 2
        K[np.ix_([main, extra], [main, extra])] += spring * np.array([[1.0, -1.0], [-1.0, 1.0]])
    F = np.zeros(size)
    for n in ("b", "c"):
        F[3 * index[n]] += lateral
        F[3 * index[n] + 1] += vertical
    held = [3 * index[n] + i for n in ("a", "d") for i in range(3)]
    free = [d for d in range(size) if d not in held]
    u = np.zeros(size)
    u[free] = np.linalg.solve(K[np.ix_(free, free)], F[free])
    G = np.zeros((size, size))
    for dofs, length, r in elements:
        local = r @ u[dofs]
        G[np.ix_(dofs, dofs)] += r.T @ geometric(length, E * A / length * (local[3] - local[0])) @ r
    mu = eigh(-G[np.ix_(free, free)], K[np.ix_(free, free)], eigvals_only=True)
    return float(u[3 * index["b"]]), sorted(1.0 / m for m in mu if m > 1e-12)[0]


def pynite_semi_rigid_sway(spring, lateral, connector=0.01):
    """🧩 Third-party semi-rigid oracle: PyNite first-order sway of the same portal with each beam end joined to the column node by a short connector member of length `connector` and bending stiffness E I = spring * connector (rotational spring k = E I / l, shear-free limit), the beam starting at the connector end."""
    from Pynite import FEModel3D

    m = FEModel3D()
    m.add_material("S", E, E / 2.6, 0.3, 7850.0)
    m.add_section("X", A, I, I, 1.0e-6)
    m.add_section("C", 100.0 * A, spring * connector / E, spring * connector / E, 1.0e-6)
    for n, (x, y) in {"a": (0.0, 0.0), "b": (0.0, 4.0), "c": (6.0, 4.0), "d": (6.0, 0.0), "b2": (connector, 4.0), "c2": (6.0 - connector, 4.0)}.items():
        m.add_node(n, x, y, 0.0)
    m.add_member("col1", "a", "b", "S", "X")
    m.add_member("col2", "d", "c", "S", "X")
    m.add_member("con1", "b", "b2", "S", "C")
    m.add_member("con2", "c2", "c", "S", "C")
    m.add_member("beam", "b2", "c2", "S", "X")
    for n in ("a", "d"):
        m.def_support(n, True, True, True, True, True, True)
    for n in ("b", "c"):
        m.add_node_load(n, "FX", lateral)
    m.analyze(check_statics=False)
    return float(m.nodes["b"].DX["Combo 1"])


def semi_rigid_cases():
    """🧾️ Semi-rigid beam ends S in {1e4, 2e7, 1e12} N m/rad: exact-spring alpha_cr and sway shift (10 kN per eaves node) against PyNite connector-member sway."""
    out = {}
    for name, spring in (("soft", 1.0e4), ("medium", 2.0e7), ("stiff", 1.0e12)):
        out[f"semiRigid{name.title()}AlphaCr"] = semi_rigid_portal(spring)[1]
        out[f"semiRigid{name.title()}Sway"] = semi_rigid_portal(spring, lateral=10e3, vertical=0.0)[0]
        out[f"semiRigid{name.title()}PyNiteSway"] = pynite_semi_rigid_sway(spring, 10e3)
    return out


def first_order_course_cases():
    """🧾️ First-order internal forces of the demo portal (20 kN horizontal at the left eaves, 200 kN down at both eaves, fixed bases) from two third-party solvers: PyNite (linear `analyze`) and anastruct, as the largest |M|, |V| and |N| of the left column and of the beam; the Rust kernel course (`BarForces::first_profile`) must reproduce them."""
    from Pynite import FEModel3D
    from anastruct import SystemElements

    nodes = {"a": (0.0, 0.0), "b": (0.0, 4.0), "c": (6.0, 4.0), "d": (6.0, 0.0)}
    bars = {"column": ("a", "b"), "beam": ("b", "c")}
    m = FEModel3D()
    m.add_material("S", E, E / 2.6, 0.3, 7850.0)
    m.add_section("X", A, I, I, 1.0e-6)
    for n, (x, z) in nodes.items():
        m.add_node(n, x, z, 0.0)
    for name, (i, j) in bars.items():
        m.add_member(name, i, j, "S", "X")
    for n in ("a", "d"):
        m.def_support(n, True, True, True, True, True, True)
    for n in ("b", "c"):
        m.def_support(n, False, False, True, True, True, False)
    m.add_member("right", "d", "c", "S", "X")
    m.add_node_load("b", "FX", 20e3)
    m.add_node_load("b", "FY", -200e3)
    m.add_node_load("c", "FY", -200e3)
    m.analyze(check_statics=False)
    out = {}
    for name in ("column", "beam"):
        p = m.members[name]
        out[f"{name}MomentPyNite"] = max(abs(p.min_moment("Mz", "Combo 1")), abs(p.max_moment("Mz", "Combo 1")))
        out[f"{name}ShearPyNite"] = max(abs(p.min_shear("Fy", "Combo 1")), abs(p.max_shear("Fy", "Combo 1")))
        out[f"{name}AxialPyNite"] = max(abs(p.min_axial("Combo 1")), abs(p.max_axial("Combo 1")))
    s = SystemElements()
    ids = {}
    for name, (i, j) in (("column", ("a", "b")), ("beam", ("b", "c")), ("right", ("d", "c"))):
        ids[name] = s.add_element(location=[list(nodes[i]), list(nodes[j])], EA=E * A, EI=E * I)
    s.add_support_fixed(node_id=s.find_node_id(list(nodes["a"])))
    s.add_support_fixed(node_id=s.find_node_id(list(nodes["d"])))
    s.point_load(node_id=s.find_node_id(list(nodes["b"])), Fx=-20e3, Fy=200e3)
    s.point_load(node_id=s.find_node_id(list(nodes["c"])), Fy=200e3)
    s.solve()
    for name in ("column", "beam"):
        r = s.get_element_results(element_id=ids[name])
        out[f"{name}MomentAnastruct"] = max(abs(r["Mmin"]), abs(r["Mmax"]))
        out[f"{name}ShearAnastruct"] = max(abs(r["Qmin"]), abs(r["Qmax"]))
        out[f"{name}AxialAnastruct"] = max(abs(r["Nmin"]), abs(r["Nmax"]))
    return out


def load_case_cases():
    """🧾️ First-order internal forces of the portal under three load cases (permanent 20 kN/m and imposed 15 kN/m uniform on the beam, wind 12 kN at the left eaves; fixed bases, HEB 240) and under one combined load vector with the TEST coefficients 1.0 / 0.5 / 0.25 (multipliers that exercise the solver, not EN 1990 values), as the largest |M| of the left column and of the beam from PyNite (linear `analyze`) and anastruct. The Rust per-case and per-combination results of `frame_envelope` must reproduce them."""
    from Pynite import FEModel3D
    from anastruct import SystemElements

    G, Q, W = 20e3, 15e3, 12e3
    vectors = {"G": (1.0, 0.0, 0.0), "Q": (0.0, 1.0, 0.0), "W": (0.0, 0.0, 1.0), "Combo": (1.0, 0.5, 0.25)}
    nodes = {"a": (0.0, 0.0), "b": (0.0, 4.0), "c": (6.0, 4.0), "d": (6.0, 0.0)}
    out = {}
    for name, (kg, kq, kw) in vectors.items():
        udl, wind = kg * G + kq * Q, kw * W
        m = FEModel3D()
        m.add_material("S", E, E / 2.6, 0.3, 7850.0)
        m.add_section("X", A, I, I, 1.0e-6)
        for n, (x, z) in nodes.items():
            m.add_node(n, x, z, 0.0)
        for member, (i, j) in {"column": ("a", "b"), "beam": ("b", "c"), "right": ("d", "c")}.items():
            m.add_member(member, i, j, "S", "X")
        for n in ("a", "d"):
            m.def_support(n, True, True, True, True, True, True)
        for n in ("b", "c"):
            m.def_support(n, False, False, True, True, True, False)
        m.add_member_dist_load("beam", "Fy", -udl, -udl)
        m.add_node_load("b", "FX", wind)
        m.analyze(check_statics=False)
        for member in ("column", "beam"):
            p = m.members[member]
            out[f"case{name}{member.capitalize()}MomentPyNite"] = max(abs(p.min_moment("Mz", "Combo 1")), abs(p.max_moment("Mz", "Combo 1")))
        s = SystemElements()
        ids = {}
        for member, (i, j) in (("column", ("a", "b")), ("beam", ("b", "c")), ("right", ("d", "c"))):
            ids[member] = s.add_element(location=[list(nodes[i]), list(nodes[j])], EA=E * A, EI=E * I)
        s.add_support_fixed(node_id=s.find_node_id(list(nodes["a"])))
        s.add_support_fixed(node_id=s.find_node_id(list(nodes["d"])))
        s.q_load(q=-udl, element_id=ids["beam"])
        s.point_load(node_id=s.find_node_id(list(nodes["b"])), Fx=wind)
        s.solve()
        for member in ("column", "beam"):
            r = s.get_element_results(element_id=ids[member])
            out[f"case{name}{member.capitalize()}MomentAnastruct"] = max(abs(r["Mmin"]), abs(r["Mmax"]))
    return out


def sway_imperfection_cases():
    """🧾️ EN 1993-1-1 5.3.2 on the portal (fixed bases, 4 m, 6 m span, 200 kN on each eaves node, H_Ed at the left eaves): phi = phi0 alpha_h alpha_m from the clause formulas computed here, the equivalent forces phi V at both eaves nodes unless H_Ed >= 0.15 V_Ed (V_Ed = 400 kN), and the first-order moments of the portal under the design loads plus these forces from PyNite and anastruct."""
    from Pynite import FEModel3D
    from anastruct import SystemElements

    V, h, span = 200e3, 4.0, 6.0
    phi = (1.0 / 200.0) * max(2.0 / 3.0, min(1.0, 2.0 / math.sqrt(h))) * math.sqrt(0.5 * (1.0 + 1.0 / 2.0))

    def nodes_with_sway(h_ed):
        return 2 if h_ed < 0.15 * 2 * V else 0

    out = {"swayPhi": phi, "swayForce": phi * V, "swayNodesH20": nodes_with_sway(20e3), "swayNodesH59": nodes_with_sway(59e3), "swayNodesH61": nodes_with_sway(61e3)}
    nodes = {"a": (0.0, 0.0), "b": (0.0, h), "c": (span, h), "d": (span, 0.0)}
    fx = {"b": 20e3 + phi * V, "c": phi * V}
    m = FEModel3D()
    m.add_material("S", E, E / 2.6, 0.3, 7850.0)
    m.add_section("X", A, I, I, 1.0e-6)
    for n, (x, z) in nodes.items():
        m.add_node(n, x, z, 0.0)
    for member, (i, j) in {"column": ("a", "b"), "beam": ("b", "c"), "right": ("d", "c")}.items():
        m.add_member(member, i, j, "S", "X")
    for n in ("a", "d"):
        m.def_support(n, True, True, True, True, True, True)
    for n in ("b", "c"):
        m.def_support(n, False, False, True, True, True, False)
        m.add_node_load(n, "FX", fx[n])
        m.add_node_load(n, "FY", -V)
    m.analyze(check_statics=False)
    for member in ("column", "beam"):
        p = m.members[member]
        out[f"sway{member.capitalize()}MomentPyNite"] = max(abs(p.min_moment("Mz", "Combo 1")), abs(p.max_moment("Mz", "Combo 1")))
    s = SystemElements()
    ids = {}
    for member, (i, j) in (("column", ("a", "b")), ("beam", ("b", "c")), ("right", ("d", "c"))):
        ids[member] = s.add_element(location=[list(nodes[i]), list(nodes[j])], EA=E * A, EI=E * I)
    s.add_support_fixed(node_id=s.find_node_id(list(nodes["a"])))
    s.add_support_fixed(node_id=s.find_node_id(list(nodes["d"])))
    for n in ("b", "c"):
        s.point_load(node_id=s.find_node_id(list(nodes[n])), Fx=fx[n], Fy=-V)
    s.solve()
    for member in ("column", "beam"):
        r = s.get_element_results(element_id=ids[member])
        out[f"sway{member.capitalize()}MomentAnastruct"] = max(abs(r["Mmin"]), abs(r["Mmax"]))
    return out


def cases():
    portal_nodes = {"a": (0.0, 0.0), "b": (0.0, 4.0), "c": (6.0, 4.0), "d": (6.0, 0.0)}
    portal = frame_alpha(portal_nodes, [("a", "b"), ("b", "c"), ("d", "c")], {"a": (1, 1, 1), "d": (1, 1, 1)}, {"b": (0.0, -200e3), "c": (0.0, -200e3)})
    column = frame_alpha({"a": (0.0, 0.0), "b": (0.0, 4.0)}, [("a", "b")], {"a": (1, 1, 1)}, {"b": (0.0, -100e3)})
    loaded = frame_alpha(portal_nodes, [("a", "b"), ("b", "c"), ("d", "c")], {"a": (1, 1, 1), "d": (1, 1, 1)}, {"b": (0.0, -20 * 200e3), "c": (0.0, -20 * 200e3)})
    return {
        "portalLowest": portal[0],
        "portalLoaded20": loaded[0],
        "portalClosedForm": portal_closed_form(4.0, 6.0, 200e3),
        "cantileverLowest": column[0],
        "cantileverEuler": math.pi**2 * E * I / (4 * 4.0**2) / 100e3,
    } | second_order_cases() | semi_rigid_cases() | first_order_course_cases() | load_case_cases() | sway_imperfection_cases()


if __name__ == "__main__":
    print(json.dumps(cases(), indent=2))
