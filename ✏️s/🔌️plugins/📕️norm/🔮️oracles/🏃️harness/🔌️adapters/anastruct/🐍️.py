"""🏗️ anastruct adapter: independent 2D linear frame solver (GPL-3.0, test-only, never linked into shipped code); SI in, sagging moments positive in `M_sag`, hogging in `M_hog`."""

from anastruct import SystemElements

LIBRARY = "anastruct"


def continuous_beam(i):
    """🌉️ Beam over supports at span ends with uniform load `w` (N/m, downward) and a downward midspan force `P_mid` per span; `fixed_start` clamps the left end. Two elements per span."""
    s = SystemElements(EA=i["E"] * i["A"], EI=i["E"] * i["I"])
    spans = i["spans"]
    x, elements = 0.0, []
    for length in spans:
        first = s.add_element([[x, 0.0], [x + length / 2.0, 0.0]])
        second = s.add_element([[x + length / 2.0, 0.0], [x + length, 0.0]])
        elements.append((first, second))
        x += length
    support_nodes = [s.element_map[1].node_id1] + [s.element_map[e[1]].node_id2 for e in elements]
    (s.add_support_fixed if i.get("fixed_start") else s.add_support_hinged)(support_nodes[0])
    for node in support_nodes[1:]:
        s.add_support_roll(node)
    for k, (first, second) in enumerate(elements):
        s.q_load(q=-i["w"][k], element_id=[first, second])
        if i.get("P_mid", [0.0] * len(spans))[k]:
            s.point_load(node_id=s.element_map[first].node_id2, Fy=-i["P_mid"][k])
    s.solve()
    out = {f"R_{k}": -float(s.get_node_results_system(node)["Fy"]) for k, node in enumerate(support_nodes)}
    for k, pair in enumerate(elements):
        results = [s.get_element_results(e) for e in pair]
        out[f"M_sag_{k}"] = max(0.0, max(-float(r["Mmin"]) for r in results))
        out[f"M_hog_{k}"] = max(0.0, max(float(r["Mmax"]) for r in results))
        out[f"delta_{k}"] = max(0.0, max(float(r["wtotmin"]) for r in results))
    return out


def portal_frame(i):
    """🏠️ Fixed-base portal with equal columns, a rigid-joint beam and a horizontal force `H` at the left beam-column joint; returns the sway and the base reactions."""
    s = SystemElements()
    s.add_element([[0.0, 0.0], [0.0, i["h"]]], EA=i["E"] * i["A"], EI=i["E"] * i["I_col"])
    s.add_element([[0.0, i["h"]], [i["span"], i["h"]]], EA=i["E"] * i["A"], EI=i["E"] * i["I_beam"])
    s.add_element([[i["span"], i["h"]], [i["span"], 0.0]], EA=i["E"] * i["A"], EI=i["E"] * i["I_col"])
    s.add_support_fixed([1, 4])
    s.point_load(node_id=2, Fx=i["H"])
    s.solve()
    left, right = s.get_node_results_system(1), s.get_node_results_system(4)
    return {"sway": float(s.get_node_displacements(node_id=2)["ux"]), "M_base_left": abs(float(left["Tz"])), "M_base_right": abs(float(right["Tz"])), "H_base_left": abs(float(left["Fx"]))}


def beam_span_envelope(i):
    """🪜️ EN 1990 6.4.3.2 arrangements of the permanent load per span (gamma_G or gamma_G,inf, as individual actions) times the variable load `gamma_Q q` on every subset of spans; extremes over all 4^n linear analyses (`R_min_0` is the smallest reaction of the first support over all arrangements)."""
    spans = i["spans"]
    n = len(spans)
    runs = []
    for p in range(2**n):
        for v in range(2**n):
            w = [(i["gamma_G"] if p >> k & 1 else i["gamma_G_inf"]) * i["g"] + (i["gamma_Q"] * i["q"] if v >> k & 1 else 0.0) for k in range(n)]
            runs.append(continuous_beam(i | {"w": w}))
    out = {f"M_sag_{k}": max(r[f"M_sag_{k}"] for r in runs) for k in range(n)}
    out["M_hog_support"] = max(r[f"M_hog_{k}"] for r in runs for k in range(n))
    out["R_max_support"] = max(r["R_1"] for r in runs)
    out["R_min_0"] = min(r["R_0"] for r in runs)
    return out


def lateral_frame(i):
    """🏢️ Fixed-base single-bay multi-storey frame under horizontal floor forces `F` (N) at the left joints: floor displacements, storey drifts, total base shear and the base moments of both columns."""
    s = SystemElements()
    heights = [0.0]
    for h in i["h"]:
        heights.append(heights[-1] + h)
    ea = i["E"] * i["A"]
    for j in range(len(i["h"])):
        for x in (0.0, i["span"]):
            s.add_element([[x, heights[j]], [x, heights[j + 1]]], EA=ea, EI=i["E"] * i["I_col"])
        s.add_element([[0.0, heights[j + 1]], [i["span"], heights[j + 1]]], EA=ea, EI=i["E"] * i["I_beam"])
    node = lambda x, y: s.find_node_id([x, y])
    s.add_support_fixed([node(0.0, 0.0), node(i["span"], 0.0)])
    for j, f in enumerate(i["F"]):
        s.point_load(node_id=node(0.0, heights[j + 1]), Fx=f)
    s.solve()
    u = [0.0] + [float(s.get_node_displacements(node_id=node(0.0, heights[j + 1]))["ux"]) for j in range(len(i["h"]))]
    left, right = s.get_node_results_system(node(0.0, 0.0)), s.get_node_results_system(node(i["span"], 0.0))
    out = {f"u_{j}": u[j] for j in range(1, len(u))}
    out |= {f"drift_{j}": u[j] - u[j - 1] for j in range(1, len(u))}
    out["V_base"] = abs(float(left["Fx"])) + abs(float(right["Fx"]))
    out["M_base_left"], out["M_base_right"] = abs(float(left["Tz"])), abs(float(right["Tz"]))
    return out


def tandem(i):
    """🚛️ EN 1991-2 4.3.2 LM1 tandem system: two equal axle loads `Q` (N) at spacing `s` crossing a simply supported span meshed at `step`; the position of the first axle is swept on the mesh and the extremes of bending moment and support reaction are reported (an axle off the span is dropped)."""
    span, q, s, step = i["span"], i["Q"], i["s"], i["step"]
    count = int(round(span / step))
    best_m, best_r = 0.0, 0.0
    for k in range(int(round((span + s) / step)) + 1):
        a = -s + k * step
        loaded = [round(x / step) for x in (a, a + s) if -1.0e-9 <= x <= span + 1.0e-9]
        if not loaded:
            continue
        system = SystemElements(EA=i["E"] * i["A"], EI=i["E"] * i["I"])
        for e in range(count):
            system.add_element([[e * step, 0.0], [(e + 1) * step, 0.0]])
        system.add_support_hinged(1)
        system.add_support_roll(count + 1)
        for index in loaded:
            system.point_load(node_id=index + 1, Fy=-q)
        system.solve()
        results = [system.get_element_results(e) for e in range(1, count + 1)]
        best_m = max(best_m, max(-float(r["Mmin"]) for r in results), max(float(r["Mmax"]) for r in results))
        best_r = max(best_r, -float(system.get_node_results_system(1)["Fy"]), -float(system.get_node_results_system(count + 1)["Fy"]))
    return {"M_max": best_m, "R_max": best_r}


def axle_group_mid_moment(i):
    """🚆️ EN 1991-2 6.3.2 LM71 on a simply supported span meshed at `step`: `n` axle loads `Q` (N) at spacing `s` centred at mid-span plus the distributed load `q` (N/m) on the elements outside the group length (n - 1) s / 2 + `lead`; by symmetry the maximum moment of the element results is the mid-span moment."""
    span, step, n = i["span"], i["step"], int(i["n"])
    mid, group = span / 2.0, (n - 1) / 2.0 * i["s"] + i["lead"]
    count = int(round(span / step))
    system = SystemElements(EA=i["E"] * i["A"], EI=i["E"] * i["I"])
    for e in range(count):
        system.add_element([[e * step, 0.0], [(e + 1) * step, 0.0]])
    system.add_support_hinged(1)
    system.add_support_roll(count + 1)
    for j in range(n):
        system.point_load(node_id=int(round((mid + (j - (n - 1) / 2.0) * i["s"]) / step)) + 1, Fy=-i["Q"])
    outside = [e + 1 for e in range(count) if (e + 1) * step <= mid - group + 1.0e-9 or e * step >= mid + group - 1.0e-9]
    system.q_load(q=-i["q"], element_id=outside)
    system.solve()
    results = [system.get_element_results(e) for e in range(1, count + 1)]
    return {"M_mid": max(max(-float(r["Mmin"]) for r in results), max(float(r["Mmax"]) for r in results))}


CHECKS = {
    "mechanics.rail-lm71-mid-moment": axle_group_mid_moment,
    "mechanics.beam-continuous": continuous_beam,
    "mechanics.portal-sway": portal_frame,
    "mechanics.beam-span-envelope": beam_span_envelope,
    "mechanics.lateral-frame": lateral_frame,
    "mechanics.lm1-tandem": tandem,
}


def storey_alpha_cr(i):
    """🏢️ EN 1993-1-1 5.2.1(4)B Eq. 5.2 alpha_cr = (H_Ed / V_Ed)(h / delta_H,Ed) with the first-order storey drift of the fixed-base portal solved by the library under H."""
    sway = portal_frame(i)["sway"]
    return {"alpha_cr_formula": i["H"] / i["V"] * i["h"] / sway, "sway": sway}


def anchor_elongation(i):
    """⚓️ Elastic elongation of the free length of a ground anchor under the design force: one truss element of axial stiffness E A and length l_free, fixed at the wall end."""
    s = SystemElements()
    s.add_truss_element([[0.0, 0.0], [i["l_free"], 0.0]], EA=i["E"] * i["A_t"])
    s.add_support_hinged(1)
    s.add_support_roll(2, direction=2)
    s.point_load(node_id=2, Fx=i["a_d"])
    s.solve()
    return {"delta": float(s.get_node_displacements(node_id=2)["ux"]), "N": float(s.get_element_results(1)["Nmax"])}


def corbel_truss(i):
    """🏛️ EN 1992-1-1 6.5 corbel as a three-bar truss (column bar, tie at the top, inclined strut from the load point to the column foot) solved by the library: load F vertical at distance a, lever arm z, optional horizontal force H at the load point."""
    s = SystemElements()
    s.add_truss_element([[0.0, 0.0], [0.0, i["z"]]], EA=1.0e9)
    s.add_truss_element([[0.0, i["z"]], [i["a"], i["z"]]], EA=1.0e9)
    s.add_truss_element([[i["a"], i["z"]], [0.0, 0.0]], EA=1.0e9)
    s.add_support_hinged(1)
    s.add_support_roll(2, direction=1)
    s.point_load(node_id=3, Fx=i["H"], Fy=-i["F"])
    s.solve()
    forces = [float(s.get_element_results(k)["Nmax"]) for k in (1, 2, 3)]
    return {"F_t": max(forces[1], 0.0), "C": abs(forces[2]), "theta": float(__import__("math").atan2(i["z"], i["a"]))}


def pin_bending_moment(i):
    """📌️ Pin in double shear as a simply supported beam of length 2 (b / 2 + c + a) carrying the bearing pressures as uniform element loads (inner plate down, outer plates up, self-equilibrated): the largest element moment is the mid-span moment."""
    f, a, b, c = i["F"], i["a"], i["b"], i["c"]
    half = b / 2.0 + c + a
    xs = [0.0, a, a + c, a + c + b, a + 2.0 * c + b, 2.0 * half]
    loads = [-f / (2.0 * a), 0.0, f / b, 0.0, -f / (2.0 * a)]
    s = SystemElements(EA=1.0e12, EI=1.0e9)
    for k, q in enumerate(loads):
        s.add_element([[xs[k], 0.0], [xs[k + 1], 0.0]])
        if q:
            s.q_load(q=q, element_id=k + 1)
    s.add_support_hinged(1)
    s.add_support_roll(len(loads) + 1)
    s.point_load(node_id=1, Fy=-1.0e-9)
    s.solve()
    best = max(max(abs(float(r["Mmax"])), abs(float(r["Mmin"]))) for r in (s.get_element_results(e) for e in range(1, len(loads) + 1)))
    return {"M_Ed": best}


CHECKS |= {
    "en1993.5.2.alpha-cr-storey": storey_alpha_cr,
    "en1997.8.anchor-elongation": anchor_elongation,
    "en1992.J.3.corbel-truss": corbel_truss,
    "en1999.8.5.pin-bending-moment": pin_bending_moment,
}
