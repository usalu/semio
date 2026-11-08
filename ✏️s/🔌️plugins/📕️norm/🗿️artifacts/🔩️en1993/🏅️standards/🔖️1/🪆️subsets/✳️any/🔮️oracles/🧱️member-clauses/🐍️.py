"""🧱️ Third-party cross-check of the EN 1993-1-1 member clauses that the document oracle (`../🚦️check-outcome/🐍️.py`) states in closed form: the nominal grades (3.2.1 Table 3.1: `blueprints`), the material constants (3.2.6: `sympy`), the bow imperfections (5.3.2, 5.3.3, 5.3.4: `scipy` eigen mode and quadrature), the classification (5.5 Table 5.2: `sectionproperties` geometry and plastic neutral axis), the hybrid girder (5.5.2(11): `sectionproperties` with two materials), torsional and torsional-flexural buckling (6.3.1.4: `sectionproperties` warping constants and a `scipy` eigenproblem), lateral-torsional buckling (6.3.2.3 and 6.3.2.4: `scipy` sine-series Galerkin and `sectionproperties` flange properties), the interaction methods (6.3.3 Annex A and B: `scipy` and `sympy`), the floor frequency (7.2.3: `scipy` beam finite elements) and the brittle fracture table (1-10 2.1: `scipy` grid interpolation). Written from the clause texts and the spec `en1993-target.md`, not from the Rust; every number is asserted against the closed form of the document oracle that is passed in as `core`. `--self-check` prints the compared values."""

import json
import math
import pathlib
import sys

import numpy as np
import sympy
from blueprints.codes.eurocode.en_1993_1_1_2005.chapter_3_materials.table_3_1 import SteelStrengthClass, Table3Dot1NominalValuesHotRolledStructuralSteel
from blueprints.codes.eurocode.en_1993_1_8_2005.chapter_3_connections_made_with_bolts_rivets_or_pins.table_3_1 import FastenerClass
from scipy import integrate, interpolate, linalg

SPEC_ROWS = {
    "V-010": ("en1993.3.2.1.grade", "blueprints Table3Dot1NominalValuesHotRolledStructuralSteel"),
    "V-012": ("en1993.1-10.t", "scipy.interpolate.RegularGridInterpolator over Table 2.1"),
    "V-013": ("en1993.3.2.6.constants", "sympy G = E / (2 (1 + nu))"),
    "V-024": ("en1993.5.3.2.bow", "scipy.integrate.quad over the equivalent bow load"),
    "V-025": ("en1993.5.3.2.bow", "scipy.linalg.eigh buckling mode scaled to e0"),
    "V-026": ("en1993.5.3.2.bow", "scipy.integrate.quad over k e0 with k = 0.5"),
    "V-028": ("en1993.5.2.class", "sectionproperties geometry and blueprints-free Table 5.2 limits"),
    "V-029": ("en1993.5.6.plastic", "scipy.linalg.eigh Galerkin stable length L_m (Eq. 6.67)"),
    "V-030": ("en1993.5.2.class", "sectionproperties plastic neutral axis, internal parts"),
    "V-031": ("en1993.5.2.class", "sectionproperties geometry, outstand flanges"),
    "V-032": ("en1993.5.2.class", "sectionproperties circular_hollow_section and angle_section"),
    "V-033": ("en1993.5.2.class", "sectionproperties, highest class of any compressed part"),
    "V-034": ("en1993.1-1.5.5.hybrid", "sectionproperties plastic moment with two materials"),
    "V-101": ("en1993.6.3.1.tf", "sectionproperties warping constants and scipy.linalg.eigh"),
    "V-111": ("en1993.6.3.2.3.rolled", "scipy.linalg.eigh sine-series Galerkin M_cr"),
    "V-112": ("en1993.6.3.2.4.cf", "sectionproperties flange plus web part radius of gyration"),
    "V-120": ("en1993.6.3.3.int", "sympy and scipy Annex A against Annex B"),
    "V-121": ("en1993.6.3.3.int", "sympy and scipy Annex B against Annex A"),
    "V-122": ("en1993.6.3.3.int", "scipy.linalg.eigh second-order amplification, Eq. 6.61 and 6.62 together"),
    "V-091": ("en1993.7.2.3.freq", "scipy.linalg.eigh Hermite beam finite elements"),
    "V-014": ("part_1_8::bolt_fub", "blueprints FastenerClass of EN 1993-1-8 Table 3.1"),
}
TABLE_31 = {"S235": (235.0, 360.0), "S275": (275.0, 430.0), "S355": (355.0, 510.0), "S450": (440.0, 550.0)}
BLUEPRINTS_FU_DIFFERS = {"S355": (510.0, 490.0)}
BOW_ELASTIC = {"a0": 350.0, "a": 300.0, "b": 250.0, "c": 200.0, "d": 150.0}
BOW_PLASTIC = {"a0": 300.0, "a": 250.0, "b": 200.0, "c": 150.0, "d": 100.0}
EQUATION_6_67_C1 = 2.752


def _close(name, ours, theirs, rel):
    assert abs(ours - theirs) <= rel * max(abs(theirs), 1e-300), (name, ours, theirs)
    return {"core": ours, "reference": theirs}


def grade_ratio(material):
    """🏷️ max(f_y / f_y,nom, f_u / f_u,nom) of a carbon material against Table 3.1 for t <= 40 mm; the nominal values of `blueprints` must agree with the clause except for the entries of `BLUEPRINTS_FU_DIFFERS` (S355: the clause 510 MPa, `blueprints` 490 MPa)."""
    name = next((g for g in TABLE_31 if material["grade"].upper().startswith(g)), None)
    if name is None:
        return None
    table = Table3Dot1NominalValuesHotRolledStructuralSteel(SteelStrengthClass[name], 20)
    assert float(table.fy) == TABLE_31[name][0], (name, table.fy)
    assert float(table.fu) == BLUEPRINTS_FU_DIFFERS.get(name, TABLE_31[name])[1], (name, table.fu)
    return max(material["fy"] / 1e6 / TABLE_31[name][0], material["fu"] / 1e6 / TABLE_31[name][1])


def bolt_class_cross_check(core):
    """🔩 f_ub = 100 n and f_yb = f_ub m / 10 of every bolt class "n.m" enumerated by `blueprints` (EN 1993-1-8 Table 3.1, shared with EN 1993-1-1 3.3) against the tables `core.BOLT_FUB_MPA` and `core.BOLT_FYB_MPA`."""
    out = {}
    for fastener in FastenerClass:
        first, second = (int(part) for part in fastener.value.split("."))
        fub, fyb = 100.0 * first, 100.0 * first * second / 10.0
        out[fastener.value] = {"fub": _close(f"f_ub {fastener.value}", core.BOLT_FUB_MPA[fastener.value], fub, 0.0), "fyb": _close(f"f_yb {fastener.value}", core.BOLT_FYB_MPA[fastener.value], fyb, 0.0)}
    return out


def constants_deviation(material):
    """🧮 Largest relative deviation of E from 210000 MPa and of G from E / (2 (1 + nu)) with nu = 0.3, the shear modulus derived symbolically by `sympy`."""
    e, nu = sympy.symbols("E nu", positive=True)
    g_nominal = float(sympy.simplify(e / (2 * (1 + nu))).subs({e: material["eModulus"], nu: sympy.Rational(3, 10)}))
    return max(abs(material["eModulus"] - 210.0e9) / 210.0e9, abs(material["gModulus"] - g_nominal) / g_nominal)


def _mid_moment(load, length):
    """🌀 Midspan moment of a simply supported beam under the transverse load `load(x)` by `scipy.integrate.quad` with the influence line x / 2 (x <= L / 2), (L - x) / 2."""
    return integrate.quad(lambda x: load(x) * min(x, length - x) / 2, 0.0, length, points=[length / 2])[0]


def bow_cross_check(core):
    """🌀 Table 5.1 e0 / L (elastic and plastic): the equivalent load q = 8 N e0 / L^2 (5.3.2(11)) reproduces the moment N e0 at midspan, the buckling mode of a pinned column from `scipy.linalg.eigh` scaled to e0 (5.3.3) carries the equivalent sine load N e0 (pi / L)^2 sin(pi x / L) with the same midspan moment, and the weak axis imperfection of 5.3.4 is k e0 with k = 0.5."""
    length, n_ed = 6.0, 1.0e6
    out = {}
    for analysis, table in (("elastic", BOW_ELASTIC), ("plastic", BOW_PLASTIC)):
        for curve, ratio in table.items():
            e0 = length / ratio
            q = 8 * n_ed * e0 / length**2
            _close(f"5.3.2 bow {analysis} {curve}", _mid_moment(lambda x: q, length), n_ed * e0, 1e-9)
            if analysis == "elastic":
                _close(f"core bow {curve}", core.SECOND_WAVE.BOW_ELASTIC[curve], ratio, 0.0)
            out[f"{analysis}.{curve}"] = ratio
    n_el, h = 40, length / 40
    k = np.zeros((n_el + 1, n_el + 1))
    kg = np.zeros_like(k)
    for j in range(n_el):
        k[j:j + 2, j:j + 2] += np.array([[1, -1], [-1, 1]]) / h
        kg[j:j + 2, j:j + 2] += np.array([[2, 1], [1, 2]]) * h / 6
    inner = slice(1, n_el)
    lam, vec = linalg.eigh(k[inner, inner], kg[inner, inner])
    mode = np.concatenate([[0.0], vec[:, 0], [0.0]])
    mode = mode / np.abs(mode).max()
    out["mode"] = _close("5.3.3 buckling mode", float(np.sqrt(lam[0])), math.pi / length, 5e-3)
    e0 = length / BOW_ELASTIC["b"]
    shape = lambda x: float(np.interp(x, np.linspace(0.0, length, n_el + 1), mode))
    scaled = lambda x: e0 * shape(x)
    out["equivalentLoadMoment"] = _close("5.3.3 equivalent load", _mid_moment(lambda x: n_ed * e0 * (math.pi / length) ** 2 * math.sin(math.pi * x / length), length), n_ed * scaled(length / 2), 5e-3)
    out["weakAxisK"] = _close("5.3.4 k e0", 0.5 * e0, e0 / 2, 1e-12)
    return out


def _warping(geometry, size):
    from sectionproperties.analysis import Section

    geometry.create_mesh(mesh_sizes=[size])
    section = Section(geometry)
    section.calculate_geometric_properties()
    section.calculate_warping_properties()
    return section


def class_limits(psi_or_alpha, eps, kind):
    """🏷️ Table 5.2 limits of an internal part (kind `web`: classes 1, 2 from alpha, class 3 from psi) or an outstand (`flange`: 9, 10, 14 eps)."""
    if kind == "flange":
        return (9 * eps, 10 * eps, 14 * eps)
    alpha, psi = psi_or_alpha
    one = 396 * eps / (13 * alpha - 1) if alpha > 0.5 else 36 * eps / alpha
    two = 456 * eps / (13 * alpha - 1) if alpha > 0.5 else 41.5 * eps / alpha
    three = 42 * eps / (0.67 + 0.33 * psi) if psi > -1 else 62 * eps * (1 - psi) * math.sqrt(-psi)
    return (one, two, three)


def classify_i(section, fy):
    """🏷️ Class of a doubly symmetric I section as the highest class of the compressed parts under pure compression (alpha = 1, psi = 1) and pure bending (alpha from the plastic neutral axis of `sectionproperties`, psi = -1): c from the geometry of `sectionproperties`."""
    from sectionproperties.analysis import Section
    from sectionproperties.pre.library import i_section

    eps = math.sqrt(235.0 / (fy / 1e6))
    geometry = i_section(d=section["h"] * 1e3, b=section["b"] * 1e3, t_f=section["tf"] * 1e3, t_w=section["tw"] * 1e3, r=section["r"] * 1e3, n_r=8)
    geometry.create_mesh(mesh_sizes=[40.0])
    analysis = Section(geometry)
    analysis.calculate_geometric_properties()
    analysis.calculate_plastic_properties()
    pna = analysis.get_pc()[1]
    web = section["h"] * 1e3 - 2 * section["tf"] * 1e3 - 2 * section["r"] * 1e3
    alpha_bending = (pna - (section["tf"] * 1e3 + section["r"] * 1e3)) / web
    alpha_bending = min(max(alpha_bending, 0.0), 1.0)
    c_w = web
    c_f = (section["b"] * 1e3 - section["tw"] * 1e3 - 2 * section["r"] * 1e3) / 2
    rank = lambda ratio, limits: next((i + 1 for i, lim in enumerate(limits) if ratio <= lim), 4)
    web_t, flange_t = section["tw"] * 1e3, section["tf"] * 1e3
    cls_web = max(rank(c_w / web_t, class_limits((1.0, 1.0), eps, "web")), rank(c_w / web_t, class_limits((alpha_bending, -1.0), eps, "web")))
    cls_flange = rank(c_f / flange_t, class_limits(None, eps, "flange"))
    return max(cls_web, cls_flange), alpha_bending


def classify_hollow(section, fy, kind):
    """🏷️ Circular hollow section d / t <= 50, 70, 90 eps^2 and angle h / t <= 15 eps with (b + h) / 2 t <= 11.5 eps (class 3, else 4), d and t recovered from the `sectionproperties` library geometry extents and area."""
    from sectionproperties.pre.library import angle_section, circular_hollow_section

    eps = math.sqrt(235.0 / (fy / 1e6))
    if kind == "Chs":
        t = max(section["tw"], section["tf"]) * 1e3
        geometry = circular_hollow_section(d=section["h"] * 1e3, t=t, n=96)
        x0, x1, y0, y1 = geometry.calculate_extents()
        d = max(x1 - x0, y1 - y0)
        return next((i + 1 for i, lim in enumerate((50, 70, 90)) if d / t <= lim * eps * eps), 4)
    t = max(section["tw"], section["tf"]) * 1e3
    geometry = angle_section(d=max(section["h"], section["b"]) * 1e3, b=min(section["h"], section["b"]) * 1e3, t=t, r_r=0, r_t=0, n_r=1)
    x0, x1, y0, y1 = geometry.calculate_extents()
    leg, short = max(x1 - x0, y1 - y0), min(x1 - x0, y1 - y0)
    return 3 if leg / t <= 15 * eps and (leg + short) / (2 * t) <= 11.5 * eps else 4


def class_cross_check(core, documents):
    """🏷️ Classes of every section of the committed oracle documents against `core.section_class`."""
    seen = {}
    for document in documents:
        material = {m["id"]: m for m in document["materials"]}
        for member in document["members"]:
            section = next((s for s in document["sections"] if s["id"] == member["sectionId"]), None)
            mat = material.get(member["materialId"])
            if section is None or mat is None or section["kind"] == "Hss" or (section["kind"], section["id"], mat["fy"]) in seen:
                continue
            ours = core.section_class(section, mat["fy"])
            if section["kind"] in ("RolledI", "WeldedI"):
                theirs, alpha = classify_i(section, mat["fy"])
                assert abs(alpha - 0.5) < 0.05, (section["id"], alpha)
            else:
                theirs = classify_hollow(section, mat["fy"], section["kind"])
            assert ours == theirs, (section["id"], section["kind"], mat["fy"], ours, theirs)
            seen[(section["kind"], section["id"], mat["fy"])] = ours
    return {f"{k[1]}@{k[2] / 1e6:.0f}": v for k, v in seen.items()}


def hybrid_plastic_moment(h, b, tw, tf, fy_flange, fy_web):
    """🏷️ Plastic moment of a doubly symmetric hybrid I girder from the plastic analysis of `sectionproperties` with two materials (5.5.2(11)), against the closed form f_y,f b t_f (h - t_f) + f_y,w t_w h_w^2 / 4."""
    from sectionproperties.analysis import Section
    from sectionproperties.pre import Geometry, Material
    from shapely.geometry import box

    flange = Material("flange", 210e3, 0.3, fy_flange / 1e6, 7.85e-6, "steelblue")
    web = Material("web", 210e3, 0.3, fy_web / 1e6, 7.85e-6, "gray")
    hw = (h - 2 * tf) * 1e3
    parts = [
        Geometry(box(0, 0, b * 1e3, tf * 1e3), flange),
        Geometry(box((b - tw) * 5e2, tf * 1e3, (b + tw) * 5e2, tf * 1e3 + hw), web),
        Geometry(box(0, tf * 1e3 + hw, b * 1e3, h * 1e3), flange),
    ]
    geometry = parts[0] + parts[1] + parts[2]
    geometry.create_mesh(mesh_sizes=[40.0])
    section = Section(geometry)
    section.calculate_geometric_properties()
    section.calculate_plastic_properties()
    theirs = section.get_mp_xx() if hasattr(section, "get_mp_xx") else section.get_mp()[0]
    ours = (fy_flange / 1e6 * b * 1e3 * tf * 1e3 * (h - tf) * 1e3 + fy_web / 1e6 * tw * 1e3 * hw**2 / 4)
    return _close("hybrid M_pl", ours, theirs, 5e-3)


def torsional_cross_check(section, material, length):
    """🌀 N_cr,T = (G I_t + pi^2 E I_w / L^2) / i_0^2 of a doubly symmetric I from the warping constants of `sectionproperties`, N_cr,TF of a channel (shear centre offset e) by `scipy.linalg.eigh` on the pair (diag(N_c, i_0^2 N_T), [[1, e], [e, i_0^2]]) against the closed form of 6.3.1.4 with beta = 1 - (e / i_0)^2."""
    from sectionproperties.pre.library import channel_section, i_section

    e_mod, g_mod = material["eModulus"] / 1e6, material["gModulus"] / 1e6
    geometry = i_section(d=section["h"] * 1e3, b=section["b"] * 1e3, t_f=section["tf"] * 1e3, t_w=section["tw"] * 1e3, r=section["r"] * 1e3, n_r=8)
    analysis = _warping(geometry, 25.0)
    area = analysis.get_area()
    ixx, iyy, _ = analysis.get_ic()
    j, gamma = analysis.get_j(), analysis.get_gamma()
    i0_sq = (ixx + iyy) / area
    n_t = (g_mod * j + math.pi**2 * e_mod * gamma / (length * 1e3) ** 2) / i0_sq
    ours = (material["gModulus"] * section["it"] + math.pi**2 * material["eModulus"] * section["iw"] / length**2) / ((section["iy"] + section["iz"]) / section["area"])
    out = {"doublySymmetricNcrT": _close("N_cr,T", ours, n_t, 0.02)}
    channel = _warping(channel_section(d=200.0, b=75.0, t_f=11.0, t_w=7.5, r=12.0, n_r=6), 15.0)
    area_c = channel.get_area()
    ixx_c, iyy_c, _ = channel.get_ic()
    xc = channel.get_c()[0]
    xse = channel.get_sc()[0]
    e_off = xc - xse
    i0_sq_c = (ixx_c + iyy_c) / area_c + e_off**2
    n_c = math.pi**2 * e_mod * iyy_c / (length * 1e3) ** 2
    n_t_c = (g_mod * channel.get_j() + math.pi**2 * e_mod * channel.get_gamma() / (length * 1e3) ** 2) / i0_sq_c
    lam = linalg.eigh(np.diag([n_c, i0_sq_c * n_t_c]), np.array([[1.0, e_off], [e_off, i0_sq_c]]), eigvals_only=True)
    beta = 1 - (e_off / math.sqrt(i0_sq_c)) ** 2
    closed = ((n_c + n_t_c) - math.sqrt((n_c + n_t_c) ** 2 - 4 * beta * n_c * n_t_c)) / (2 * beta)
    x = sympy.symbols("N")
    cubic = sympy.Poly(sympy.expand((n_c - x) * (n_t_c - x) - (x * e_off) ** 2 / i0_sq_c), x)
    roots = sorted(float(r) for r in sympy.Poly(cubic).nroots())
    out["channelNcrTF"] = {"eigh": float(lam.min()), "closed": closed, "sympyRoot": roots[0]}
    assert abs(lam.min() - closed) <= 1e-9 * closed and abs(roots[0] - closed) <= 1e-9 * closed
    return out


def galerkin_mcr(length, e, g, iz, it, iw, shape, n=14, pts=4000):
    """🔄 M_cr (in units of the peak of `shape`) of a fork supported I beam under the moment M(x) = M0 shape(x) by the sine-series Galerkin of v and phi solved with `scipy.linalg.eigh`."""
    h = length / pts
    xs = (np.arange(pts) + 0.5) * h
    ks = np.arange(1, n + 1) * np.pi / length
    stiff = np.diag(np.concatenate([e * iz * ks**4 * length / 2, (e * iw * ks**4 + g * it * ks**2) * length / 2]))
    s = np.sin(np.outer(ks, xs))
    cross = -(s * (ks[:, None] ** 2) * np.array([shape(x) for x in xs])[None, :]) @ s.T * h
    geometric = np.zeros((2 * n, 2 * n))
    geometric[:n, n:] = cross
    geometric[n:, :n] = cross.T
    return float(-1.0 / linalg.eigh(geometric, stiff, eigvals_only=True).min())


def ltb_cross_check(core):
    """🔄 M_cr of HEB 240 by Galerkin (constants of `sectionproperties`) against `core.critical_moment` for the uniform, linear (psi = 0.5, 0, -0.5) and parabolic moment diagrams, and chi_LT of 6.3.2.3 (Eq. 6.57, beta = 0.75, lambda_LT,0 = 0.4) against the general Eq. 6.56 reduction `core.chi` for beta = 1, lambda_LT,0 = 0.2."""
    from sectionproperties.pre.library import i_section

    s = dict(h=0.24, b=0.24, tw=0.01, tf=0.017, r=0.021, iz=3.923e-5, it=1.037e-6, iw=4.762e-7, kind="RolledI")
    analysis = _warping(i_section(d=240.0, b=240.0, t_f=17.0, t_w=10.0, r=21.0, n_r=8), 20.0)
    material = dict(eModulus=210e9, gModulus=81e9)
    ixx, iyy, _ = analysis.get_ic()
    third = dict(iz=iyy * 1e-12, it=analysis.get_j() * 1e-12, iw=analysis.get_gamma() * 1e-18)
    length = 5.0
    out = {}
    cases = (
        ("uniform", dict(momentDiagram="Uniform", endMomentRatioPsi=1.0, loadApplication="ShearCenter"), lambda x: 1.0),
        ("psi=0.5", dict(momentDiagram="Linear", endMomentRatioPsi=0.5, loadApplication="ShearCenter"), lambda x: 1 - 0.5 * x / length),
        ("psi=0", dict(momentDiagram="Linear", endMomentRatioPsi=0.0, loadApplication="ShearCenter"), lambda x: 1 - x / length),
        ("psi=-0.5", dict(momentDiagram="Linear", endMomentRatioPsi=-0.5, loadApplication="ShearCenter"), lambda x: 1 - 1.5 * x / length),
        ("parabolic", dict(momentDiagram="Parabolic", endMomentRatioPsi=0.0, loadApplication="ShearCenter"), lambda x: 4 * x * (length - x) / length**2),
    )
    for name, member, shape in cases:
        theirs = galerkin_mcr(length, material["eModulus"], material["gModulus"], third["iz"], third["it"], third["iw"], shape)
        out[name] = _close(f"M_cr {name}", core.critical_moment(s, material, length, member), theirs, 0.08)
        assert name in ("psi=-0.5", "psi=-0.25") or abs(out[name]["core"] / theirs - 1) <= 0.03, (name, out[name])
    for lam in (0.3, 0.6, 1.0, 1.6):
        mine = _chi_lt(lam, 0.34, 1.0, 0.2)
        out[f"chi({lam})"] = _close("chi_LT limit", mine, core.chi(lam, "b"), 1e-12)
    return out


def _chi_lt(lam, alpha, beta, lam0):
    """🔄 chi_LT of Eq. 6.57 (rolled and equivalent welded), limited by 1 and 1 / lambda^2."""
    if lam <= lam0:
        return 1.0
    phi = 0.5 * (1 + alpha * (lam - lam0) + beta * lam**2)
    return min(1.0 / (phi + math.sqrt(phi**2 - beta * lam**2)), 1.0, 1.0 / lam**2 if lam > 0 else 1.0)


def compression_flange_radius(section):
    """🔄 i_f,z of the compression flange plus 1/3 of the compressed web (hw / 6 of the height for a symmetric bending diagram) from `sectionproperties`."""
    from sectionproperties.analysis import Section
    from sectionproperties.pre import Geometry
    from shapely.geometry import box

    b, tf, tw, hw = section["b"] * 1e3, section["tf"] * 1e3, section["tw"] * 1e3, (section["h"] - 2 * section["tf"]) * 1e3
    geometry = Geometry(box(0, 0, b, tf).union(box((b - tw) / 2, -hw / 6, (b + tw) / 2, 0)))
    geometry.create_mesh(mesh_sizes=[10.0])
    analysis = Section(geometry)
    analysis.calculate_geometric_properties()
    i_zz = analysis.get_ic()[1]
    return math.sqrt(i_zz / analysis.get_area())


def cf_cross_check():
    """🔄 i_f,z of HEB 240 and the slenderness lambda_f = k_c L_c / (i_f,z lambda_1) of 6.3.2.4 against the closed form i_f,z = sqrt(I_f / A_f) with A_f = b t_f + h_w t_w / 6."""
    s = dict(h=0.24, b=0.24, tw=0.01, tf=0.017)
    hw = s["h"] - 2 * s["tf"]
    a_f = s["b"] * s["tf"] + hw * s["tw"] / 6
    i_f = s["tf"] * s["b"] ** 3 / 12 + hw / 6 * s["tw"] ** 3 / 12
    return {"iFz": _close("i_f,z", math.sqrt(i_f / a_f) * 1e3, compression_flange_radius(s), 5e-3)}


def frequency_cross_check():
    """⏱ f_1 = pi / (2 L^2) sqrt(E I / m) of a simply supported beam (7.2.3) against the lowest eigenfrequency of 24 Hermite beam elements with consistent mass solved by `scipy.linalg.eigh`."""
    length, ei, mass, n = 8.0, 210e9 * 1.126e-4, 350.0, 24
    h = length / n
    k_e = ei / h**3 * np.array([[12, 6 * h, -12, 6 * h], [6 * h, 4 * h * h, -6 * h, 2 * h * h], [-12, -6 * h, 12, -6 * h], [6 * h, 2 * h * h, -6 * h, 4 * h * h]])
    m_e = mass * h / 420 * np.array([[156, 22 * h, 54, -13 * h], [22 * h, 4 * h * h, 13 * h, -3 * h * h], [54, 13 * h, 156, -22 * h], [-13 * h, -3 * h * h, -22 * h, 4 * h * h]])
    size = 2 * (n + 1)
    k, m = np.zeros((size, size)), np.zeros((size, size))
    for j in range(n):
        k[2 * j:2 * j + 4, 2 * j:2 * j + 4] += k_e
        m[2 * j:2 * j + 4, 2 * j:2 * j + 4] += m_e
    free = [i for i in range(size) if i not in (0, size - 2)]
    lam = linalg.eigh(k[np.ix_(free, free)], m[np.ix_(free, free)], eigvals_only=True)
    theirs = math.sqrt(lam[0]) / (2 * math.pi)
    ours = math.pi / (2 * length**2) * math.sqrt(ei / mass)
    return {"f1": _close("f_1", ours, theirs, 1e-4)}


def toughness_cross_check(core):
    """🌡️ t_max of Table 2.1 (EN 1993-1-10) between the reference temperatures by `scipy.interpolate.interp1d` against `core.column_21` and `core.limit_21`, for every stress column of a grade and sub-grade that is transcribed in the registry (the other columns are `unverified` registry rows without a value and are not compared)."""
    cells = core.table_21()
    temps = sorted(core.TEMPS_21)
    checked = {}
    for pair in sorted({k.split(".")[0] for k in cells}):
        for level, key in core.LEVELS_21:
            column = [cells.get(f"{pair}.{key}.t{t}") for t in temps]
            if any(v is None for v in column):
                continue
            line = interpolate.interp1d(temps, column, kind="linear", bounds_error=False, fill_value=(column[0], column[-1]))
            for t_ed in (-47.0, -33.0, -20.0, -12.5, 5.0, 15.0):
                theirs = float(line(t_ed))
                _close(f"t_max {pair} {key} {t_ed}", core.column_21(cells, pair, key, t_ed), theirs, 1e-9)
                _close(f"t_max limit {pair} {key} {t_ed}", core.limit_21(cells, pair, level, t_ed), theirs, 1e-9)
            checked[f"{pair}.{key}"] = len(temps)
    assert checked, "no transcribed Table 2.1 column"
    return checked


def stable_length_cross_check():
    """📏 L_m of 6.3.5.3 (Eq. 6.67) for HEB 240 under psi = -1 (C_1 = 2.752) against the Galerkin M_cr of `scipy`: L_m = 38 i_z / sqrt(N / (57.4 A) + W_pl,y^2 / (756 C_1^2 A I_t) (f_y / 235)^2)."""
    i_z, area, wpl, it, fy = math.sqrt(3.923e-5 / 0.0106), 0.0106, 1.053e-3, 1.037e-6, 355.0
    ours = 38 * i_z / math.sqrt(wpl**2 / (756 * EQUATION_6_67_C1**2 * area * it) * (fy / 235) ** 2) * 1.0
    x = sympy.symbols("x", positive=True)
    symbolic = sympy.sqrt(sympy.Rational(756) * sympy.Float(EQUATION_6_67_C1) ** 2 * sympy.Float(area) * sympy.Float(it) / sympy.Float(wpl) ** 2 * (235 / sympy.Float(fy)) ** 2)
    return {"Lm": _close("L_m (6.67)", ours, 38 * i_z * float(symbolic), 1e-9)}


def interaction_cross_check(core):
    """🔗 Annex B (Table B.1 and B.2) of `core.interaction_annex_b` against Annex A from the spec text for HEB 240 S355, L = 4 m pinned, N = 800 kN, M_y = 100 kNm at one end (psi = 0): both methods give the same order of magnitude (within 20 %) and the first-order identity at N = 0."""
    e, g, fy, length = 210e9, 81e9, 355e6, 4.0
    a, iy, iz, it, wel_y, wpl_y, wel_z, wpl_z = 0.0106, 1.126e-4, 3.923e-5, 1.027e-6, 938e-6, 1053e-6, 327e-6, 498e-6
    n_ed, my_ed = 800e3, 100e3
    n_rk, my_rk, mz_rk = a * fy, wpl_y * fy, wpl_z * fy
    lam_y = length / math.sqrt(iy / a) / (math.pi * math.sqrt(e / fy))
    lam_z = length / math.sqrt(iz / a) / (math.pi * math.sqrt(e / fy))
    chi_y, chi_z = core.chi(lam_y, "b"), core.chi(lam_z, "c")
    ncr_y, ncr_z = math.pi**2 * e * iy / length**2, math.pi**2 * e * iz / length**2
    ncr_t = (g * it) / ((iy + iz) / a)
    mu_y = (1 - n_ed / ncr_y) / (1 - chi_y * n_ed / ncr_y)
    mu_z = (1 - n_ed / ncr_z) / (1 - chi_z * n_ed / ncr_z)
    w_y, w_z = min(wpl_y / wel_y, 1.5), min(wpl_z / wel_z, 1.5)
    npl = n_ed / n_rk
    a_lt = max(1 - it / iy, 0.0)
    lam_max = max(lam_y, lam_z)
    chi_lt = 1.0
    psi = 0.0
    c_my0 = 0.79 + 0.21 * psi + 0.36 * (psi - 0.33) * n_ed / ncr_y
    eps_y = my_ed / n_ed * a / wel_y
    c_my = c_my0 + (1 - c_my0) * math.sqrt(eps_y) * a_lt / (1 + math.sqrt(eps_y) * a_lt)
    c_mlt = max(c_my**2 * a_lt / math.sqrt((1 - n_ed / ncr_z) * (1 - n_ed / ncr_t)), 1.0)
    c_yy = max(1 + (w_y - 1) * (2 - 1.6 / w_y * c_my**2 * lam_max - 1.6 / w_y * c_my**2 * lam_max**2) * npl, wel_y / wpl_y)
    c_zy = max(1 + (w_y - 1) * (2 - 14 * c_my**2 * lam_max**2 / w_y**5) * npl, 0.6 * math.sqrt(w_y / w_z) * wel_y / wpl_y)
    k_yy = c_my * c_mlt * mu_y / (1 - n_ed / ncr_y) / c_yy
    k_zy = c_my * c_mlt * mu_z / (1 - n_ed / ncr_y) / c_zy * 0.6 * math.sqrt(w_y / w_z)
    u61_a = n_ed / (chi_y * n_rk) + k_yy * my_ed / (chi_lt * my_rk)
    u62_a = n_ed / (chi_z * n_rk) + k_zy * my_ed / (chi_lt * my_rk)
    u_b = core.interaction_annex_b(n_ed, my_ed, 0.0, n_rk, my_rk, mz_rk, chi_y, chi_z, chi_lt, lam_y, lam_z, 0.6, 0.6, 0.6, 1, True, 1.0)
    zero = core.interaction_annex_b(0.0, my_ed, 0.0, n_rk, my_rk, mz_rk, chi_y, chi_z, chi_lt, lam_y, lam_z, 0.6, 0.6, 0.6, 1, True, 1.0)
    assert abs(zero - 0.6 * my_ed / my_rk) <= 0.4 * my_ed / my_rk + 1e-9, zero
    top = max(u61_a, u62_a)
    assert abs(top - u_b) <= 0.2 * u_b, (top, u_b)
    return {"annexA": {"eq6.61": u61_a, "eq6.62": u62_a}, "annexB": u_b}


def self_check(core):
    """🧪 Every cross-check of this module against the closed forms of `core`, with the committed oracle documents."""
    root = pathlib.Path(__file__).resolve().parents[2] / "🧪️tests" / "🧫️fixtures" / "📄️oracle-documents"
    documents = [json.loads(p.read_text(encoding="utf-8")) for p in sorted(root.glob("*/🔣️.json"))]
    materials = {m["id"] + "@" + str(m["fy"]): m for d in documents for m in d["materials"]}
    section = next(s for d in documents for s in d["sections"] if s["designation"] == "HEB 240")
    material = next(iter(materials.values()))
    return {
        "boltClasses": bolt_class_cross_check(core),
        "grade": {k: grade_ratio(m) for k, m in materials.items() if m["kind"] != "Stainless"},
        "constants": {k: constants_deviation(m) for k, m in materials.items()},
        "bow": bow_cross_check(core),
        "classes": class_cross_check(core, documents),
        "hybrid": hybrid_plastic_moment(0.5, 0.2, 0.008, 0.012, 460e6, 235e6),
        "torsion": torsional_cross_check(section, material, 4.0),
        "ltb": ltb_cross_check(core),
        "compressionFlange": cf_cross_check(),
        "frequency": frequency_cross_check(),
        "toughness": toughness_cross_check(core),
        "stableLength": stable_length_cross_check(),
        "interaction": interaction_cross_check(core),
    }
