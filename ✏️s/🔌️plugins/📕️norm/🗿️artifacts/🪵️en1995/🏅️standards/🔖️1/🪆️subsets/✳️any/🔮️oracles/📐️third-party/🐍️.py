"""📐️ Third-party oracles of the EN 1995 catalogue: sectionproperties (MIT) for the part constants and the rigid composite of the mechanically jointed beam T-90 and for the exact residual section of the reduced-properties method T-106, PyNiteFEA (MIT) for the floor of T-63 and the footbridge of T-120.

Usage with the repo interpreter and `-I`:
  python -I 🐍️.py            print {"cases": [{"entry", "label", "case", "values"}]} for the built-in cases
  python -I 🐍️.py --check    additionally assert the closed-form bounds and exit 1 on a violation
Units follow the kernel: N and mm for T-90 and T-106, N, m and kg for T-63 and T-120. The divisors 200, 125 and 330 of T-106 are unverified registry values (`ndp.en1995.fire.reduced-*`)."""

import json
import math
import sys
from pathlib import Path

from Pynite import FEModel3D
from scipy.integrate import solve_ivp
from sectionproperties.analysis import Section
from sectionproperties.pre import Material
from sectionproperties.pre.library import rectangular_section

COMBO = "Combo 1"
REGISTRY = next(parent / "⚖️compliance" / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json" for parent in Path(__file__).resolve().parents if (parent / "⚖️compliance" / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json").is_file())


def bracing_row(row):
    """📖 Common value of the registry row `ndp.en1995.bracing.<row>`: the row the Rust kernel reads (unverified)."""
    entries = {e["id"]: e for e in json.loads(REGISTRY.read_text(encoding="utf-8"))["entries"]}
    return float(entries[f"ndp.en1995.bracing.{row}"]["recommended"])
K_REDUCED = (("k_mod_fi_bending", 200.0), ("k_mod_fi_compression", 125.0), ("k_mod_fi_tension", 330.0))


def rectangle(b, h, x, y, e=None):
    """🟫️ Rectangle b × h with its lower left corner at (x, y); with `e` it carries a material of that modulus and the analysis is composite."""
    material = None if e is None else Material(name=f"E{e:g}", elastic_modulus=e, poissons_ratio=0.0, yield_strength=1.0, density=1.0e-9, color="tan")
    geometry = rectangular_section(d=h, b=b) if material is None else rectangular_section(d=h, b=b, material=material)
    return geometry.shift_section(x_offset=x, y_offset=y)


def solved(geometry, mesh):
    """🧮️ Mesh with triangles of about `mesh`² mm² and run the geometric analysis."""
    geometry.create_mesh(mesh_sizes=[mesh * mesh])
    section = Section(geometry)
    section.calculate_geometric_properties()
    return section


def stiffness(parts, offsets):
    """📊️ (EA, EI) about the centroid of the parts `(b, h, e)` whose lower left corners sit at `offsets` (x, y), from the composite analysis."""
    geometry = None
    for (b, h, e), (x, y) in zip(parts, offsets):
        shape = rectangle(b, h, x, y, e)
        geometry = shape if geometry is None else geometry + shape
    section = solved(geometry, 5.0)
    return section.get_ea(), section.get_eic()[0]


def t90(case):
    """🧩️ T-90: E_iA_i and E_iI_i of every part and the rigid composite (EI) from sectionproperties; γ_i, a_i of Annex B (B.5) and (EI)_ef from those constants; the deflection of a PyNite beam carrying (EI)_ef under the line load q_d."""
    n = case["num"]
    keys = [k for k in (1, 2, 3) if f"b{k}" in n]
    parts = {k: (n[f"b{k}"], n[f"h{k}"], n[f"e{k}"]) for k in keys}
    width = max(p[0] for p in parts.values())
    single = {k: stiffness([parts[k]], [(0.0, 0.0)]) for k in keys}
    for k in keys:
        assert abs(single[k][0] / parts[k][2] - n[f"a{k}"]) <= 1.0e-6 * n[f"a{k}"], f"a{k}"
    y, offsets = 0.0, {}
    for k in reversed(keys):
        offsets[k] = ((width - parts[k][0]) / 2.0, y)
        y += parts[k][1]
    rigid = stiffness([parts[k] for k in keys], [offsets[k] for k in keys])[1]
    gamma = {k: 1.0 if k == 2 else 1.0 / (1.0 + math.pi**2 * single[k][0] * n[f"s{k}"] / (n["k"] * n["l"] ** 2)) for k in keys}
    ea = {k: gamma[k] * single[k][0] for k in keys}
    gap = lambda k: (parts[k][1] + parts[2][1]) / 2.0
    a2 = (ea[1] * 2.0 * gap(1) - (ea[3] * 2.0 * gap(3) if 3 in keys else 0.0)) / (2.0 * sum(ea.values()))
    lever = {1: gap(1) - a2, 2: a2, 3: gap(3) + a2 if 3 in keys else 0.0}
    ei_ef = sum(single[k][1] + ea[k] * lever[k] ** 2 for k in keys)
    out = {"ei_ef": ei_ef, "ei_rigid": rigid, "ei_separate": sum(single[k][1] for k in keys), "slip_efficiency": ei_ef / rigid, "gamma_1": gamma[1], "a_2": a2}
    if 3 in keys:
        out["gamma_3"] = gamma[3]
    if "q_d" in n:
        model, nodes = FEModel3D(), 10
        model.add_material("T", 1.0, 0.4, 0.4, 1.0e-9)
        model.add_section("S", 1.0, ei_ef, ei_ef, 1.0)
        for j in range(nodes + 1):
            model.add_node(f"n{j}", n["l"] * j / nodes, 0.0, 0.0)
        for j in range(nodes):
            model.add_member(f"m{j}", f"n{j}", f"n{j + 1}", "T", "S")
            model.add_member_dist_load(f"m{j}", "Fy", -n["q_d"], -n["q_d"])
        model.def_support("n0", True, True, True, True, False, False)
        model.def_support(f"n{nodes}", False, True, True, True, False, False)
        model.analyze_linear(check_stability=False)
        out["deflection"] = -model.nodes[f"n{nodes // 2}"].DY[COMBO]
    return out


def t106(case):
    """🔥️ T-106: the exact residual section of the notional char depth d_char,n = β_n·t (the section of the method, not the d_ef section of T-101) from sectionproperties, p/A_r of its exposed faces and k_mod,fi = 1 - min(t/20, 1)·(p/A_r)/divisor."""
    n = case["num"]
    left, right, top, bottom = n.get("face_left", 1.0), n.get("face_right", 1.0), n.get("face_top", 0.0), n.get("face_bottom", 1.0)
    d = n["beta_n"] * n["t_req"]
    br, hr = n["b"] - d * (left + right), n["h"] - d * (top + bottom)
    section = solved(rectangle(br, hr, d * left, d * bottom), max(min(br, hr) / 16.0, 1.0))
    area = section.get_area()
    ratio = ((left + right) * hr + (top + bottom) * br) / area * 1.0e3
    w_el = min(abs(z) for z in section.get_z()[:2])
    out = {"b_r": br, "h_r": hr, "area": area, "i_y": section.get_ic()[0], "w_el": w_el, "ratio": ratio}
    for key, divisor in K_REDUCED:
        out[key] = max(1.0 - min(n["t_req"] / 20.0, 1.0) * ratio / divisor, 0.0)
    if "m_fi" in n:
        out["sigma_m"] = n["m_fi"] / w_el
    return out


def beam_modal(span, ei, mass, cells=40, iterations=6):
    """🌊️ Fundamental angular frequency and mode shape of a simply supported beam by inverse iteration on PyNite static solves: the load `mass·φ` of a mode estimate deflects into the next estimate and the Rayleigh quotient converges to ω₁²."""
    model = FEModel3D()
    model.add_material("T", 1.0, 0.4, 0.4, 1.0e-9)
    model.add_section("S", 1.0, ei, ei, 1.0)
    for j in range(cells + 1):
        model.add_node(f"n{j}", span * j / cells, 0.0, 0.0)
    for j in range(cells):
        model.add_member(f"m{j}", f"n{j}", f"n{j + 1}", "T", "S")
    model.def_support("n0", True, True, True, True, False, False)
    model.def_support(f"n{cells}", False, True, True, True, False, False)
    model.add_load_combo(COMBO, {"Case 1": 1.0})
    phi = [math.sin(math.pi * j / cells) for j in range(cells + 1)]
    lumped = mass * span / cells
    omega2 = 0.0
    for _ in range(iterations):
        for j in range(1, cells):
            model.nodes[f"n{j}"].NodeLoads.clear()
            model.add_node_load(f"n{j}", "FY", lumped * phi[j], "Case 1")
        model.analyze_linear(check_stability=False)
        y = [0.0] + [model.nodes[f"n{j}"].DY[COMBO] for j in range(1, cells)] + [0.0]
        omega2 = sum(phi[j] ** 2 for j in range(1, cells)) / sum(phi[j] * y[j] for j in range(1, cells))
        peak = max(y, key=abs)
        phi = [v / peak for v in y]
    return math.sqrt(omega2), phi


def t63(case):
    """🌊️ T-63: f₁ of a simply supported floor strip with (EI)_l and mass per square metre from the PyNite iteration."""
    n = case["num"]
    return {"f1": beam_modal(n["span_m"], n["ei_l"], n["mass_kg_m2"])[0] / (2.0 * math.pi)}


def t120(case):
    """🌉️ T-120: f₁ of the girder from the PyNite iteration and the single-pedestrian vertical acceleration k/(M ζ) with the modal mass μL/2 of EN 1995-2 7.3 (k = 200 N up to 2.5 Hz, else 100 N)."""
    n = case["num"]
    f1 = beam_modal(n["span_m"], n["ei_nm2"], n["mass_kg_m"])[0] / (2.0 * math.pi)
    return {"f1": f1, "a_single": (200.0 if f1 <= 2.5 else 100.0) / (n["mass_kg_m"] * n["span_m"] / 2.0 * n.get("zeta", 0.015))}


def floor_mode(case, nx=8, ny=8, iterations=8):
    """🏢️ Orthotropic simply supported floor as a PyNite grillage (joists (EI)_l along x per metre width, distribution (EI)_b along y, no torsion): f₁, the mode shape φ (max 1) on the nx+1 × ny+1 grid and the resonant acceleration a(x,y) = F·|φ(x,y)|·φ_f/(2 ζ M) of a harmonic footfall force F at the antinode with the modal mass M = Σ m φ² A."""
    n = case["num"]
    lx, ly, mass = n["span_m"], n["width_m"], n["mass_kg_m2"]
    model = FEModel3D()
    model.add_material("T", 1.0, 0.4, 0.4, 1.0e-9)
    model.add_section("L", 1.0, n["ei_l"] * ly / ny, n["ei_l"] * ly / ny, 1.0e-6)
    model.add_section("B", 1.0, n["ei_b"] * lx / nx, n["ei_b"] * lx / nx, 1.0e-6)
    for i in range(nx + 1):
        for j in range(ny + 1):
            model.add_node(f"p{i}_{j}", lx * i / nx, 0.0, ly * j / ny)
    for i in range(nx):
        for j in range(ny + 1):
            model.add_member(f"x{i}_{j}", f"p{i}_{j}", f"p{i + 1}_{j}", "T", "L")
    for i in range(nx + 1):
        for j in range(ny):
            model.add_member(f"z{i}_{j}", f"p{i}_{j}", f"p{i}_{j + 1}", "T", "B")
    for i in range(nx + 1):
        for j in range(ny + 1):
            if i in (0, nx) or j in (0, ny):
                model.def_support(f"p{i}_{j}", (i, j) in ((0, 0), (0, ny)), True, (i, j) in ((0, 0), (nx, 0)), False, False, False)
    model.add_load_combo(COMBO, {"Case 1": 1.0})
    keys = [(i, j) for i in range(nx + 1) for j in range(ny + 1)]
    lumped = mass * lx * ly / (nx * ny)
    phi = {(i, j): math.sin(math.pi * i / nx) * math.sin(math.pi * j / ny) for i, j in keys}
    omega2 = 0.0
    for _ in range(iterations):
        for i in range(1, nx):
            for j in range(1, ny):
                model.nodes[f"p{i}_{j}"].NodeLoads.clear()
                model.add_node_load(f"p{i}_{j}", "FY", lumped * phi[(i, j)], "Case 1")
        model.analyze_linear(check_stability=False)
        y = {k: model.nodes[f"p{k[0]}_{k[1]}"].DY[COMBO] for k in keys}
        omega2 = sum(phi[k] ** 2 for k in keys) / sum(phi[k] * y[k] for k in keys)
        peak = max(y.values(), key=abs)
        phi = {k: v / peak for k, v in y.items()}
    modal = sum(lumped * phi[k] ** 2 for k in keys)
    zeta, force = n.get("zeta", 0.03), n.get("force_n", 700.0)
    amplitude = force / (2.0 * zeta * modal)
    shape = [[phi[(i, j)] for j in range(ny + 1)] for i in range(nx + 1)]
    return {"f1": math.sqrt(omega2) / (2.0 * math.pi), "modal_mass": modal, "a_peak": amplitude, "phi": shape}


def strip_deflection(q, span, ei, nodes=20):
    """📏️ Midspan deflection of a simply supported strip of width 1 m under the line load `q` (N/m) with the stiffness `ei` (N·m²) from PyNite."""
    model = FEModel3D()
    model.add_material("T", 1.0, 0.4, 0.4, 1.0e-9)
    model.add_section("S", 1.0, ei, ei, 1.0)
    for j in range(nodes + 1):
        model.add_node(f"n{j}", span * j / nodes, 0.0, 0.0)
    for j in range(nodes):
        model.add_member(f"m{j}", f"n{j}", f"n{j + 1}", "T", "S")
        model.add_member_dist_load(f"m{j}", "Fy", -q, -q)
    model.def_support("n0", True, True, True, True, False, False)
    model.def_support(f"n{nodes}", False, True, True, True, False, False)
    model.add_load_combo(COMBO, {"Case 1": 1.0})
    model.analyze_linear(check_stability=False)
    return -model.nodes[f"n{nodes // 2}"].DY[COMBO]


def taper(case, segments=24):
    """📐️ T-30 and T-31 stiffness: a simply supported beam with linearly varying depth, bending only, as `segments` PyNite members whose second moment of area I = b h³/12 of the mid-segment depth comes from sectionproperties; the midspan deflection under the line load q (N/mm) in mm, with E in N/mm²."""
    n = case["num"]
    model = FEModel3D()
    model.add_material("T", n["e"], n["e"] / 20.0, 0.4, 1.0e-9)
    for j in range(segments + 1):
        model.add_node(f"n{j}", n["l"] * j / segments, 0.0, 0.0)
    inertia = []
    for j in range(segments):
        h = n["h_a"] + (n["h_b"] - n["h_a"]) * (j + 0.5) / segments
        section = solved(rectangle(n["b"], h, 0.0, 0.0), max(min(n["b"], h) / 6.0, 4.0))
        inertia.append(section.get_ic()[0])
        model.add_section(f"S{j}", section.get_area(), inertia[-1], inertia[-1], 1.0)
        model.add_member(f"m{j}", f"n{j}", f"n{j + 1}", "T", f"S{j}")
        model.add_member_dist_load(f"m{j}", "Fy", -n["q"], -n["q"])
    model.def_support("n0", True, True, True, True, False, False)
    model.def_support(f"n{segments}", False, True, True, True, False, False)
    model.add_load_combo(COMBO, {"Case 1": 1.0})
    model.analyze_linear(check_stability=False)
    return {"deflection": -model.nodes[f"n{segments // 2}"].DY[COMBO], "i_mid": inertia[segments // 2], "i_small": inertia[0], "i_large": inertia[-1]}


def hole(case):
    """🕳️ T-18: the net section at a hole of diameter d through the member width, a rectangle b × (h − d), as the area A_net and the elastic modulus about its own centroid from sectionproperties, next to the gross section."""
    n = case["num"]
    net = solved(rectangle(n["b"], n["h"] - n["hole_d"], 0.0, 0.0), 5.0)
    gross = solved(rectangle(n["b"], n["h"], 0.0, 0.0), 5.0)
    return {"a_net": net.get_area(), "a_gross": gross.get_area(), "i_net": net.get_ic()[0], "w_net": min(abs(z) for z in net.get_z()[:2])}


def creep(case):
    """⏳️ Creep deflection time curve: φ(t) from the ODE φ' = (k_def − φ)/τ by scipy `solve_ivp` (the closed form is k_def (1 − e^(−t/τ))), the instantaneous deflections of the permanent and the variable load from the PyNite strip, and w(t) = w_G (1 + φ) + w_Q (1 + ψ₂ φ) at the listed ages in years."""
    n = case["num"]
    ages = [0.0, 1.0, 2.0, 5.0, 10.0, 20.0, 50.0]
    solution = solve_ivp(lambda t, phi: [(n["k_def"] - phi[0]) / n["tau"]], (0.0, ages[-1]), [0.0], t_eval=ages, rtol=1.0e-10, atol=1.0e-12)
    w_g, w_q = strip_deflection(n["g"], n["span_m"], n["ei_l"]), strip_deflection(n["q"], n["span_m"], n["ei_l"])
    phi = [float(v) for v in solution.y[0]]
    return {"ages": ages, "phi": phi, "w_g": w_g, "w_q": w_q, "w": [w_g * (1.0 + f) + w_q * (1.0 + n["psi_2"] * f) for f in phi]}


def strip(case):
    """📏️ Floor strip deflection under an area load: 5 q ℓ⁴/(384 (EI)_ℓ) from the PyNite strip of width 1 m."""
    n = case["num"]
    return {"deflection": strip_deflection(n["q"], n["span_m"], n["ei_l"])}


def clt(case):
    """🧱️ T-97: the layered CLT section of width `b` mm from sectionproperties, one rectangle per layer from the bottom, the modulus `e` for a layer along the span and a vanishing modulus for a cross layer (the γ-method counts only the longitudinal layers): the rigid composite (EI) per mm width (all γ_i = 1, the upper bound of the γ-method) and the sum of the own second moments E_i I_i of the longitudinal layers (the lower bound γ_i = 0)."""
    n = case["num"]
    layers = [(n[f"t{k}"], n[f"o{k}"] > 0.5) for k in range(1, int(n["n_layers"]) + 1)]
    parts, offsets, z = [], [], 0.0
    for thickness, along in layers:
        parts.append((n["b"], thickness, n["e"] if along else 1.0e-6))
        offsets.append((0.0, z))
        z += thickness
    _, rigid = stiffness(parts, offsets)
    own = 0.0
    for thickness, along in layers:
        if along:
            own += stiffness([(n["b"], thickness, n["e"])], [(0.0, 0.0)])[1]
    return {"ei_rigid": rigid / n["b"], "ei_separate": own / n["b"], "total": z}


def truss_layout(kind, span, height, bays):
    """🔺️ Joints (x, z) and members (from, to) of a parallel-chord truss written independently of the Rust `truss_layout`: Warren (top joints at the bay centres, diagonals alternate) or Pratt (verticals everywhere, diagonals fall towards midspan)."""
    dx = span / bays
    joints = [(i * dx, 0.0) for i in range(bays + 1)]
    members = [(i, i + 1) for i in range(bays)]
    if kind == "warren":
        joints += [((i + 0.5) * dx, height) for i in range(bays)]
        top = lambda i: bays + 1 + i
        members += [(top(i), top(i + 1)) for i in range(bays - 1)]
        for i in range(bays):
            members += [(i, top(i)), (top(i), i + 1)]
    else:
        joints += [(i * dx, height) for i in range(bays + 1)]
        top = lambda i: bays + 1 + i
        members += [(top(i), top(i + 1)) for i in range(bays)] + [(i, top(i)) for i in range(bays + 1)]
        for i in range(bays):
            members.append((top(i), i + 1) if 2 * i < bays else (i, top(i + 1)))
    return joints, members


def truss_forces(span, height, bays, q, kind="warren", ea=1.0e9):
    """🔺️ Axial forces of a pin-jointed truss under the nodal load of a line load `q` (N/m) on the top joints, from PyNite (members with released ends, planar). Returns the largest top-chord force (PyNite reports compression of the top chord of a gravity-loaded truss as positive), the abscissa-moment reference `M/H` of the same bay from statics and the support reaction."""
    joints, members = truss_layout(kind, span, height, bays)
    model = FEModel3D()
    model.add_material("T", 1.0, 0.4, 0.4, 1.0e-9)
    model.add_section("S", ea, 1.0e3, 1.0e3, 1.0)
    for k, (x, z) in enumerate(joints):
        model.add_node(f"j{k}", x, 0.0, z)
    for k, (a, b) in enumerate(members):
        model.add_member(f"m{k}", f"j{a}", f"j{b}", "T", "S")
        model.def_releases(f"m{k}", Ryi=True, Rzi=True, Ryj=True, Rzj=True)
    for k in range(len(joints)):
        model.def_support(f"j{k}", False, True, False, True, True, True)
    model.def_support("j0", True, True, True, True, True, True)
    model.def_support(f"j{bays}", False, True, True, True, True, True)
    tops = [k for k, (x, z) in enumerate(joints) if z > 0.0]
    dx = span / bays
    loads = {k: q * dx / (2.0 if joints[k][0] in (0.0, span) else 1.0) for k in tops}
    for k, p in loads.items():
        model.add_node_load(f"j{k}", "FZ", -p)
    model.add_load_combo(COMBO, {"Case 1": 1.0})
    model.analyze_linear(check_stability=False)
    axial = {k: model.members[f"m{k}"].max_axial(COMBO) for k in range(len(members))}
    top_chord = [k for k, (a, b) in enumerate(members) if joints[a][1] > 0.0 and joints[b][1] > 0.0]
    n_top = max(axial[k] for k in top_chord)
    reaction = model.nodes["j0"].RxnFZ[COMBO]
    total = sum(loads.values())
    def moment(x):
        return reaction * x - sum(p * (x - joints[k][0]) for k, p in loads.items() if joints[k][0] < x)
    k_best = max(top_chord, key=lambda k: axial[k])
    a, b = members[k_best]
    xs = [joints[j][0] for j in (a, b)]
    centre = [j for j, (x, z) in enumerate(joints) if z == 0.0 and min(xs) - 1e-9 <= x <= max(xs) + 1e-9]
    ref = max(moment(joints[j][0]) for j in centre) / height if centre else moment(sum(xs) / 2.0) / height
    return {"n_chord": n_top, "n_statics": ref, "reaction": reaction, "total": total, "joints": float(len(joints)), "members": float(len(members))}


def bracing_spring(n_braced, n_d, length_m, k_br, ref_length=None, divisor=None, deflection_divisor=None):
    """✖️ T-96 (EN 1995-1-1 9.2.5.3(1), (2)): the line load q_d = k_l n N_d/(k_f,3 l) (N/m) on a restraint line of length l whose bracing system has the lateral stiffness k_br (N/mm, force of the resultant q_d l per deflection, the definition of a system stiffness; the clause names no spring layout) realised as one spring k_br/2 at each end, from PyNite: the sum of the spring reactions must equal ΣF_d = q_d l, the deflection δ = F_d/k_br and the limit l/500. Constants from the registry rows `bracing.*`."""
    ref_length, divisor, deflection_divisor = ref_length or bracing_row("kl-reference-length"), divisor or bracing_row("stabilising-divisor"), deflection_divisor or bracing_row("deflection-limit-divisor")
    q = min(1.0, math.sqrt(ref_length / length_m)) * n_braced * n_d / (divisor * length_m)
    model = FEModel3D()
    model.add_material("T", 1.0, 0.4, 0.4, 1.0e-9)
    model.add_section("S", 1.0e12, 1.0e12, 1.0e12, 1.0e12)
    model.add_node("a", 0.0, 0.0, 0.0)
    model.add_node("b", length_m, 0.0, 0.0)
    model.add_member("m", "a", "b", "T", "S")
    model.add_member_dist_load("m", "Fy", q, q)
    model.def_support("a", True, False, True, True, True, False)
    model.def_support("b", False, False, True, True, True, False)
    k = k_br * 1.0e3 / 2.0
    model.def_support_spring("a", "DY", k)
    model.def_support_spring("b", "DY", k)
    model.add_load_combo(COMBO, {"Case 1": 1.0})
    model.analyze_linear(check_stability=False)
    sums = abs(model.nodes["a"].RxnFY[COMBO]) + abs(model.nodes["b"].RxnFY[COMBO])
    return {"q_d": q, "f_d": q * length_m, "reaction_sum": sums, "delta_mm": abs(model.nodes["a"].DY[COMBO]) * 1.0e3, "limit_mm": length_m * 1.0e3 / deflection_divisor}


def deck_bar(sigma_p_0, spacing_mm, depth_mm, area_mm2, width_m, e_bar=200000.0):
    """🌉️ T-122: the prestress bar of a stress-laminated deck as a PyNite tension member (N, mm): the jacking force P = σ_p,0 s h of the tributary timber area acts at the free end, the library returns the support reaction, the member axial force and the elongation over the deck width, against the closed forms P, P/A and P L/(E A)."""
    force = sigma_p_0 * spacing_mm * depth_mm
    length = width_m * 1.0e3
    model = FEModel3D()
    model.add_material("S", e_bar, e_bar / 2.6, 0.3, 7.85e-9)
    model.add_section("B", area_mm2, 1.0e6, 1.0e6, 1.0e6)
    model.add_node("a", 0.0, 0.0, 0.0)
    model.add_node("b", length, 0.0, 0.0)
    model.add_member("m", "a", "b", "S", "B")
    model.def_support("a", True, True, True, True, True, True)
    model.def_support("b", False, True, True, True, True, True)
    model.add_node_load("b", "FX", force, case="Case 1")
    model.add_load_combo(COMBO, {"Case 1": 1.0})
    model.analyze_linear(check_stability=False)
    axial = abs(float(model.members["m"].max_axial(COMBO)))
    return {"force": force, "reaction": abs(float(model.nodes["a"].RxnFX[COMBO])), "axial": axial, "stress": axial / area_mm2, "elongation_mm": float(model.nodes["b"].DX[COMBO]), "closed_elongation_mm": force * length / (e_bar * area_mm2)}


def t122(case):
    """🌉️ T-122: bar force, bar stress and elongation of one prestress bar from PyNite."""
    n = case["num"]
    return deck_bar(n["sigma_p_0"], n["bar_spacing"], n["deck_depth"], n["bar_area"], n["width_m"])


def t95(case):
    """🔺️ T-95: chord compression of a pin-jointed truss under a line load from PyNite against the statics reference M/H of the governing bay."""
    n = case["num"]
    return truss_forces(n["span_m"], n["height_m"], int(n["bays"]), n["q"], case.get("kind", "warren"))


def t96(case):
    """✖️ T-96: bracing force and stiffness deflection of a restraint line from PyNite springs."""
    n = case["num"]
    return bracing_spring(n["n_braced"], n["n_d"], n["l_m"], n["k_br"])


def cases():
    """📚️ The built-in cases: a T-beam and a three-part box of T-90, two residual sections of T-106, a floor strip, a footbridge girder and an orthotropic floor."""
    t_beam = {"b1": 200.0, "h1": 40.0, "e1": 11000.0, "a1": 8000.0, "s1": 100.0, "b2": 60.0, "h2": 240.0, "e2": 11000.0, "a2": 14400.0, "k": 2000.0, "l": 5000.0, "q_d": 3.0}
    box = {"b1": 150.0, "h1": 30.0, "e1": 11000.0, "a1": 4500.0, "s1": 80.0, "b2": 40.0, "h2": 200.0, "e2": 11000.0, "a2": 8000.0, "b3": 150.0, "h3": 30.0, "e3": 11000.0, "a3": 4500.0, "s3": 80.0, "k": 1500.0, "l": 4000.0}
    return [
        {"entry": "T-90", "label": "t-beam", "case": {"num": t_beam}},
        {"entry": "T-90", "label": "box-three-parts", "case": {"num": box}},
        {"entry": "T-106", "label": "three-sided", "case": {"num": {"b": 200.0, "h": 400.0, "t_req": 60.0, "beta_n": 0.7, "m_fi": 60.0e6}}},
        {"entry": "T-106", "label": "early", "case": {"num": {"b": 160.0, "h": 360.0, "t_req": 10.0, "beta_n": 0.8}}},
        {"entry": "T-63", "label": "strip", "case": {"num": {"span_m": 4.5, "ei_l": 1.2e6, "mass_kg_m2": 160.0}}},
        {"entry": "T-120", "label": "girder", "case": {"num": {"span_m": 24.0, "ei_nm2": 3.0e8, "mass_kg_m": 450.0, "zeta": 0.015}}},
        {"entry": "T-63", "label": "floor-mode", "case": {"num": {"span_m": 4.5, "width_m": 4.0, "ei_l": 1.2e6, "ei_b": 6.0e4, "mass_kg_m2": 160.0, "zeta": 0.03, "force_n": 700.0}}},
        {"entry": "T-30", "label": "taper", "case": {"num": {"b": 120.0, "h_a": 300.0, "h_b": 500.0, "l": 6000.0, "q": 5.0, "e": 11000.0}}},
        {"entry": "T-18", "label": "hole", "case": {"num": {"b": 120.0, "h": 360.0, "hole_d": 50.0}}},
        {"entry": "T-61", "label": "creep", "case": {"num": {"span_m": 4.0, "ei_l": 1.0e6, "g": 1000.0, "q": 2000.0, "psi_2": 0.3, "k_def": 0.6, "tau": 5.0}}},
        {"entry": "T-63", "label": "strip-deflection", "case": {"num": {"span_m": 4.0, "ei_l": 1.0e6, "q": 3000.0}}},
        {"entry": "T-95", "label": "warren-truss", "case": {"kind": "warren", "num": {"span_m": 12.0, "height_m": 1.5, "bays": 6.0, "q": 4000.0}}},
        {"entry": "T-95", "label": "pratt-truss", "case": {"kind": "pratt", "num": {"span_m": 12.0, "height_m": 1.5, "bays": 6.0, "q": 4000.0}}},
        {"entry": "T-122", "label": "deck-bar", "case": {"num": {"sigma_p_0": 1.0, "bar_spacing": 800.0, "deck_depth": 250.0, "bar_area": 804.247719318987, "width_m": 4.0}}},
        {"entry": "T-96", "label": "bracing-spring", "case": {"num": {"n_braced": 4.0, "n_d": 5000.0, "l_m": 6.0, "k_br": 500.0}}},
        {"entry": "T-97", "label": "clt-three-layer", "case": {"num": {"b": 1000.0, "e": 11000.0, "g_r": 50.0, "l": 4000.0, "n_layers": 3.0, "t1": 30.0, "o1": 1.0, "t2": 40.0, "o2": 0.0, "t3": 30.0, "o3": 1.0}}},
        {"entry": "T-97", "label": "clt-five-layer", "case": {"num": {"b": 1000.0, "e": 11000.0, "g_r": 50.0, "l": 5000.0, "n_layers": 5.0, "t1": 40.0, "o1": 1.0, "t2": 40.0, "o2": 0.0, "t3": 40.0, "o3": 1.0, "t4": 40.0, "o4": 0.0, "t5": 40.0, "o5": 1.0}}},
    ]


ENTRIES = {("T-122", None): t122, ("T-95", None): t95, ("T-96", None): t96, ("T-90", None): t90, ("T-106", None): t106, ("T-63", "strip"): t63, ("T-120", None): t120, ("T-63", "floor-mode"): floor_mode, ("T-30", None): taper, ("T-18", None): hole, ("T-61", None): creep, ("T-63", "strip-deflection"): strip, ("T-97", None): clt}


def run(item):
    """🧮️ Evaluate one case with the library behind its entry."""
    return (ENTRIES.get((item["entry"], item["label"])) or ENTRIES[(item["entry"], None)])(item["case"])


def check(items):
    """🔬️ Closed-form bounds every library result must respect."""
    by = {i["label"]: i["values"] for i in items}
    for label in ("t-beam", "box-three-parts"):
        v = by[label]
        assert v["ei_separate"] < v["ei_ef"] < v["ei_rigid"], label
    s = cases()[4]["case"]["num"]
    closed = math.pi / (2 * s["span_m"] ** 2) * math.sqrt(s["ei_l"] / s["mass_kg_m2"])
    assert abs(by["strip"]["f1"] - closed) < 1.0e-3 * closed
    f = cases()[6]["case"]["num"]
    plate = math.pi / 2.0 * math.sqrt(f["ei_l"] / f["mass_kg_m2"]) * math.sqrt(1 / f["span_m"] ** 4 + f["ei_b"] / f["ei_l"] / f["width_m"] ** 4)
    assert abs(by["floor-mode"]["f1"] - plate) < 1.0e-3 * plate
    n = cases()[7]["case"]["num"]
    average = (n["h_a"] + n["h_b"]) / 2.0
    prismatic = 5.0 * n["q"] * n["l"] ** 4 / (384.0 * n["e"] * n["b"] * average**3 / 12.0)
    assert by["taper"]["i_small"] < by["taper"]["i_mid"] < by["taper"]["i_large"]
    assert 0.5 * prismatic < by["taper"]["deflection"] < 1.5 * prismatic
    h = by["hole"]
    assert abs(h["a_net"] - 120.0 * 310.0) < 1e-6 * h["a_net"] and abs(h["w_net"] - 120.0 * 310.0**2 / 6.0) < 1e-6 * h["w_net"]
    c = by["creep"]
    k = cases()[9]["case"]["num"]
    assert all(abs(f - k["k_def"] * (1.0 - math.exp(-a / k["tau"]))) < 1e-7 for f, a in zip(c["phi"], c["ages"]))
    assert abs(by["strip-deflection"]["deflection"] - 5.0 * 3000.0 * 4.0**4 / (384.0 * 1.0e6)) < 1e-9
    for label in ("clt-three-layer", "clt-five-layer"):
        v, num = by[label], next(i for i in cases() if i["label"] == label)["case"]["num"]
        assert 0.0 < v["ei_separate"] < v["ei_rigid"], label
        assert abs(v["total"] - sum(num[f"t{k}"] for k in range(1, int(num["n_layers"]) + 1))) < 1e-9, label
    three = by["clt-three-layer"]
    rigid = 11000.0 * (2.0 * 30.0**3 / 12.0 + 2.0 * 30.0 * 35.0**2)
    assert abs(three["ei_rigid"] - rigid) < 1e-6 * rigid
    for label in ("warren-truss", "pratt-truss"):
        v = by[label]
        assert abs(v["n_chord"] - v["n_statics"]) < 1.0e-6 * v["n_statics"], label
        assert abs(v["reaction"] - v["total"] / 2.0) < 1.0e-6 * v["total"], label
    b = by["bracing-spring"]
    assert abs(b["reaction_sum"] - b["f_d"]) < 1.0e-6 * b["f_d"] and abs(b["delta_mm"] - b["f_d"] / 500.0) < 1.0e-6 * b["delta_mm"]
    d = by["deck-bar"]
    assert abs(d["reaction"] - d["force"]) < 1.0e-6 * d["force"] and abs(d["axial"] - d["force"]) < 1.0e-6 * d["force"], "the bar carries the jacking force"
    assert abs(d["force"] - 200000.0) < 1e-9 and abs(d["stress"] - 200000.0 / 804.247719318987) < 1.0e-6 * d["stress"]
    assert abs(d["elongation_mm"] - d["closed_elongation_mm"]) < 1.0e-6 * d["closed_elongation_mm"]
    t = by["three-sided"]
    assert abs(t["b_r"] - 116.0) < 1e-9 and abs(t["h_r"] - 358.0) < 1e-9 and abs(t["area"] - 116.0 * 358.0) < 1e-6 * t["area"]
    assert abs(t["i_y"] - 116.0 * 358.0**3 / 12.0) < 1e-6 * t["i_y"] and abs(t["w_el"] - 116.0 * 358.0**2 / 6.0) < 1e-6 * t["w_el"]


def main():
    """🚪️ Print the evaluated built-in cases; `--check` asserts the bounds first."""
    items = [{**item, "values": run(item)} for item in cases()]
    if "--check" in sys.argv:
        check(items)
    print(json.dumps({"cases": items}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
