"""🏗️ PyNiteFEA adapter: independent 3D finite element frame solver (MIT) used for 2D beams and portals in the X-Y plane; SI in, sagging moments positive in the reported `M_sag`, hogging in `M_hog`."""

from Pynite import FEModel3D

LIBRARY = "PyNiteFEA"
COMBO = "Combo 1"
ROLL = (False, True, True, True, False, False)
PIN = (True, True, True, True, False, False)
FIXED = (True, True, True, True, True, True)


def model(i, section=None):
    """🧱️ Empty model with a steel-like material and one section; the shear modulus and density are irrelevant to slender-beam results."""
    m = FEModel3D()
    m.add_material("S", i["E"], i["E"] / 2.6, 0.3, 7850.0)
    m.add_section("X", i["A"], i["I"], i["I"], 1.0e-6)
    return m


def continuous_beam(i):
    """🌉️ Beam over supports at span ends with uniform load `w` (N/m, downward) and a downward midspan force `P_mid` per span; `fixed_start` clamps the left end. `mesh` members per span (even, default 2) so the maximum deflection of unequal spans is sampled on a fine mesh."""
    m = model(i)
    spans = i["spans"]
    count = int(i.get("mesh", 2))
    xs = [0.0]
    for length in spans:
        xs.append(xs[-1] + length)
    for k, x in enumerate(xs):
        m.add_node(f"s{k}", x, 0.0, 0.0)
    for k, length in enumerate(spans):
        names = [f"s{k}"] + [f"c{k}_{j}" for j in range(1, count)] + [f"s{k + 1}"]
        for j in range(1, count):
            m.add_node(names[j], xs[k] + length * j / count, 0.0, 0.0)
        for j in range(count):
            m.add_member(f"m{k}_{j}", names[j], names[j + 1], "S", "X")
            m.add_member_dist_load(f"m{k}_{j}", "Fy", -i["w"][k], -i["w"][k])
        if i.get("P_mid", [0.0] * len(spans))[k]:
            m.add_node_load(names[count // 2], "FY", -i["P_mid"][k])
    for k in range(len(xs)):
        m.def_support(f"s{k}", *(FIXED if k == 0 and i.get("fixed_start") else PIN if k == 0 else ROLL))
    m.analyze_linear(check_stability=False)
    out = {f"R_{k}": float(m.nodes[f"s{k}"].RxnFY[COMBO]) for k in range(len(xs))}
    for k in range(len(spans)):
        members = [m.members[f"m{k}_{h}"] for h in range(count)]
        out[f"M_sag_{k}"] = max(0.0, max(-mem.min_moment("Mz", COMBO) for mem in members))
        out[f"M_hog_{k}"] = max(0.0, max(mem.max_moment("Mz", COMBO) for mem in members))
        out[f"delta_{k}"] = max(0.0, max(-mem.min_deflection("dy", COMBO) for mem in members))
    return out


def portal_frame(i):
    """🏠️ Fixed-base portal with equal columns, a rigid-joint beam and a horizontal force `H` at the left beam-column joint; returns the sway and the base reactions."""
    m = FEModel3D()
    m.add_material("S", i["E"], i["E"] / 2.6, 0.3, 7850.0)
    m.add_section("C", i["A"], i["I_col"], i["I_col"], 1.0e-6)
    m.add_section("B", i["A"], i["I_beam"], i["I_beam"], 1.0e-6)
    for name, x, y in (("a", 0.0, 0.0), ("b", 0.0, i["h"]), ("c", i["span"], i["h"]), ("d", i["span"], 0.0)):
        m.add_node(name, x, y, 0.0)
    for name, a, b, sec in (("col1", "a", "b", "C"), ("beam", "b", "c", "B"), ("col2", "d", "c", "C")):
        m.add_member(name, a, b, "S", sec)
    for node in ("a", "d"):
        m.def_support(node, *FIXED)
    m.add_node_load("b", "FX", i["H"])
    m.analyze_linear(check_stability=False)
    return {"sway": float(m.nodes["b"].DX[COMBO]), "M_base_left": abs(float(m.nodes["a"].RxnMZ[COMBO])), "M_base_right": abs(float(m.nodes["d"].RxnMZ[COMBO])), "H_base_left": abs(float(m.nodes["a"].RxnFX[COMBO]))}


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
    m = FEModel3D()
    m.add_material("S", i["E"], i["E"] / 2.6, 0.3, 7850.0)
    m.add_section("C", i["A"], i["I_col"], i["I_col"], 1.0e-6)
    m.add_section("B", i["A"], i["I_beam"], i["I_beam"], 1.0e-6)
    heights = [0.0]
    for h in i["h"]:
        heights.append(heights[-1] + h)
    for j, y in enumerate(heights):
        m.add_node(f"l{j}", 0.0, y, 0.0)
        m.add_node(f"r{j}", i["span"], y, 0.0)
    for j in range(len(i["h"])):
        for side in "lr":
            m.add_member(f"c{side}{j}", f"{side}{j}", f"{side}{j + 1}", "S", "C")
        m.add_member(f"b{j + 1}", f"l{j + 1}", f"r{j + 1}", "S", "B")
        m.add_node_load(f"l{j + 1}", "FX", i["F"][j])
    for side in "lr":
        m.def_support(f"{side}0", *FIXED)
    m.analyze_linear(check_stability=False)
    u = [0.0] + [float(m.nodes[f"l{j + 1}"].DX[COMBO]) for j in range(len(i["h"]))]
    out = {f"u_{j}": u[j] for j in range(1, len(u))}
    out |= {f"drift_{j}": u[j] - u[j - 1] for j in range(1, len(u))}
    out["V_base"] = abs(float(m.nodes["l0"].RxnFX[COMBO])) + abs(float(m.nodes["r0"].RxnFX[COMBO]))
    out["M_base_left"], out["M_base_right"] = abs(float(m.nodes["l0"].RxnMZ[COMBO])), abs(float(m.nodes["r0"].RxnMZ[COMBO]))
    return out


def tandem(i):
    """🚛️ EN 1991-2 4.3.2 LM1 tandem system: two equal axle loads `Q` (N) at spacing `s` crossing a simply supported span; the position of the first axle is swept on a grid and the extremes of bending moment and support reaction are reported (an axle off the span is dropped)."""
    span, q, s, step = i["span"], i["Q"], i["s"], i["step"]
    best_m, best_r = 0.0, 0.0
    for k in range(int(round((span + s) / step)) + 1):
        a = -s + k * step
        axles = [x for x in (a, a + s) if -1.0e-9 <= x <= span + 1.0e-9]
        m = model(i)
        m.add_node("a", 0.0, 0.0, 0.0)
        m.add_node("b", span, 0.0, 0.0)
        m.add_member("beam", "a", "b", "S", "X")
        m.def_support("a", *PIN)
        m.def_support("b", *ROLL)
        for x in axles:
            m.add_member_pt_load("beam", "Fy", -q, min(max(x, 0.0), span))
        if not axles:
            continue
        m.analyze_linear(check_stability=False)
        best_m = max(best_m, -m.members["beam"].min_moment("Mz", COMBO), m.members["beam"].max_moment("Mz", COMBO))
        best_r = max(best_r, float(m.nodes["a"].RxnFY[COMBO]), float(m.nodes["b"].RxnFY[COMBO]))
    return {"M_max": best_m, "R_max": best_r}


def tandem_section_envelope(i):
    """🌈️ EN 1991-2 4.3.2 LM1 tandem system sweeping a simply supported span: the largest bending moment and the largest and smallest shear force at the section `x`, taken from the library over the tandem positions on a grid (an axle off the span is dropped; the shear is read an epsilon left and right of the section so a load standing on it counts on its worse side)."""
    span, q, s, step, x = i["span"], i["Q"], i["s"], i["step"], i["x"]
    best_m, v_max, v_min = 0.0, 0.0, 0.0
    for k in range(int(round((span + s) / step)) + 1):
        a = -s + k * step
        axles = [p for p in (a, a + s) if 1.0e-9 <= p <= span - 1.0e-9]
        if not axles:
            continue
        m = model(i)
        m.add_node("a", 0.0, 0.0, 0.0)
        m.add_node("b", span, 0.0, 0.0)
        m.add_member("beam", "a", "b", "S", "X")
        m.def_support("a", *PIN)
        m.def_support("b", *ROLL)
        for p in axles:
            m.add_member_pt_load("beam", "Fy", -q, p)
        m.analyze_linear(check_stability=False)
        beam = m.members["beam"]
        best_m = max(best_m, abs(float(beam.moment("Mz", x, COMBO))))
        for side in (-1.0e-6, 1.0e-6):
            v = float(beam.shear("Fy", min(max(x + side, 0.0), span), COMBO))
            v_max, v_min = max(v_max, v), min(v_min, v)
    return {"M_env": best_m, "V_max": v_max, "V_min": v_min}


def axle_group_mid_moment(i):
    """🚆️ EN 1991-2 6.3.2 LM71 on a simply supported span: `n` axle loads `Q` (N) at spacing `s` centred at mid-span plus the distributed load `q` (N/m) on the span outside the group length (n - 1) s / 2 + `lead` on both sides; the library reports the mid-span moment."""
    span, q_axle, s, n, udl, lead = i["span"], i["Q"], i["s"], int(i["n"]), i["q"], i["lead"]
    mid, group = span / 2.0, (int(i["n"]) - 1) / 2.0 * i["s"] + i["lead"]
    m = model(i)
    m.add_node("a", 0.0, 0.0, 0.0)
    m.add_node("b", span, 0.0, 0.0)
    m.add_member("beam", "a", "b", "S", "X")
    m.def_support("a", *PIN)
    m.def_support("b", *ROLL)
    for j in range(n):
        m.add_member_pt_load("beam", "Fy", -q_axle, mid + (j - (n - 1) / 2.0) * s)
    m.add_member_dist_load("beam", "Fy", -udl, -udl, 0.0, mid - group)
    m.add_member_dist_load("beam", "Fy", -udl, -udl, mid + group, span)
    m.analyze_linear(check_stability=False)
    return {"M_mid": abs(float(m.members["beam"].moment("Mz", mid, COMBO)))}


CHECKS = {
    "mechanics.rail-lm71-mid-moment": axle_group_mid_moment,
    "mechanics.beam-continuous": continuous_beam,
    "mechanics.portal-sway": portal_frame,
    "mechanics.beam-span-envelope": beam_span_envelope,
    "mechanics.lateral-frame": lateral_frame,
    "mechanics.lm1-tandem": tandem,
    "mechanics.tandem-section-envelope": tandem_section_envelope,
}


def storey_alpha_cr(i):
    """🏢️ EN 1993-1-1 5.2.1(4)B Eq. 5.2 alpha_cr = (H_Ed / V_Ed)(h / delta_H,Ed) with the first-order storey drift of the fixed-base portal solved by the library under H."""
    sway = portal_frame(i)["sway"]
    return {"alpha_cr_formula": i["H"] / i["V"] * i["h"] / sway, "sway": sway}


def anchor_elongation(i):
    """⚓️ Elastic elongation of the free length of a ground anchor under the design force: one member of axial stiffness E A and length l_free fixed at the wall end and restrained transversally at the head."""
    m = FEModel3D()
    m.add_material("S", i["E"], i["E"] / 2.6, 0.3, 7850.0)
    m.add_section("T", i["A_t"], 1.0, 1.0, 1.0)
    m.add_node("a", 0.0, 0.0, 0.0)
    m.add_node("b", i["l_free"], 0.0, 0.0)
    m.add_member("anchor", "a", "b", "S", "T")
    m.def_support("a", *FIXED)
    m.def_support("b", False, True, True, True, True, True)
    m.add_node_load("b", "FX", i["a_d"])
    m.analyze_linear(check_stability=False)
    return {"delta": float(m.nodes["b"].DX[COMBO]), "N": abs(float(m.members["anchor"].max_axial(COMBO)))}


def pin_bending_moment(i):
    """📌️ Pin in double shear as a simply supported member of length 2 (b / 2 + c + a) with partial uniform loads (inner plate down, outer plates up, self-equilibrated); the mid-span moment is the largest."""
    f, a, b, c = i["F"], i["a"], i["b"], i["c"]
    half = b / 2.0 + c + a
    length = 2.0 * half
    m = FEModel3D()
    m.add_material("S", 2.1e11, 8.1e10, 0.3, 7850.0)
    m.add_section("P", 0.01, 1.0e-4, 1.0e-4, 1.0e-4)
    m.add_node("a", 0.0, 0.0, 0.0)
    m.add_node("b", length, 0.0, 0.0)
    m.add_member("pin", "a", "b", "S", "P")
    m.def_support("a", *PIN)
    m.def_support("b", *ROLL)
    m.add_member_dist_load("pin", "Fy", f / (2.0 * a), f / (2.0 * a), 0.0, a, "D")
    m.add_member_dist_load("pin", "Fy", -f / b, -f / b, a + c, a + c + b, "D")
    m.add_member_dist_load("pin", "Fy", f / (2.0 * a), f / (2.0 * a), a + 2.0 * c + b, length, "D")
    m.add_load_combo(COMBO, {"D": 1.0})
    m.analyze_linear(check_stability=False)
    pin = m.members["pin"]
    return {"M_Ed": max(abs(pin.min_moment("Mz", COMBO)), abs(pin.max_moment("Mz", COMBO)))}


CHECKS |= {
    "en1993.5.2.alpha-cr-storey": storey_alpha_cr,
    "en1997.8.anchor-elongation": anchor_elongation,
    "en1999.8.5.pin-bending-moment": pin_bending_moment,
}


def storey_alpha_cr_second_order(i):
    """🏢️ EN 1993-1-1 5.2.1(3) alpha_cr from the second-order amplification of the sway: PyNite `analyze_PDelta` with the vertical loads V / 2 on each column head gives delta_II, and delta_II / delta_I = 1 / (1 - 1 / alpha_cr)."""
    def sway(second_order):
        m = FEModel3D()
        m.add_material("S", i["E"], i["E"] / 2.6, 0.3, 7850.0)
        m.add_section("C", i["A"], i["I_col"], i["I_col"], 1.0e-6)
        m.add_section("B", i["A"], i["I_beam"], i["I_beam"], 1.0e-6)
        for name, x, y in (("a", 0.0, 0.0), ("b", 0.0, i["h"]), ("c", i["span"], i["h"]), ("d", i["span"], 0.0)):
            m.add_node(name, x, y, 0.0)
        for name, a, b, sec in (("col1", "a", "b", "C"), ("beam", "b", "c", "B"), ("col2", "d", "c", "C")):
            m.add_member(name, a, b, "S", sec)
        for node in ("a", "d"):
            m.def_support(node, *FIXED)
        m.add_node_load("b", "FX", i["H"])
        m.add_node_load("b", "FY", -i["V"] / 2.0)
        m.add_node_load("c", "FY", -i["V"] / 2.0)
        m.analyze_PDelta() if second_order else m.analyze_linear(check_stability=False)
        return float(m.nodes["b"].DX[COMBO])

    first, second = sway(False), sway(True)
    return {"alpha_cr_eigen": 1.0 / (1.0 - first / second)}


CHECKS |= {"en1993.5.2.alpha-cr-eigen": storey_alpha_cr_second_order}
