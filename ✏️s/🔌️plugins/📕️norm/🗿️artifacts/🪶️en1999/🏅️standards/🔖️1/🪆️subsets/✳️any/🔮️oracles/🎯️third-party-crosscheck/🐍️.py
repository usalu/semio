#!/usr/bin/env python3
"""🎯️ Third-party cross-check of the EN 1999 thin-walled quantities of the compliance oracle (EN 1999-1-4 and 1-5): the spec rows A-02, A-21, A-22, A-60, A-62, A-70 and A-72 of `🧭️spec-trace` against anastruct, sectionproperties, scipy and sympy. Run with the repository venv: `python -I 🐍️.py`; exit code 1 on any disagreement."""
from __future__ import annotations

import importlib.util
import json
import math
import sys
from pathlib import Path

import sympy
from anastruct import SystemElements
from scipy import optimize
from sectionproperties.analysis import Section
from sectionproperties.pre import Geometry
from sectionproperties.pre.library import i_section
from shapely.geometry import LineString, box
import numpy

HERE = Path(__file__).resolve().parent
_SPEC = importlib.util.spec_from_file_location("compliance_oracle", HERE.parent / "⚖️compliance" / "🐍️.py")
ORACLE = importlib.util.module_from_spec(_SPEC)
sys.modules["compliance_oracle"] = ORACLE
_SPEC.loader.exec_module(ORACLE)

E, G = ORACLE.E, ORACLE.G
SPEC_ROWS = {
    "A-60": {"checks": ["en1999.1-4.local"], "clause": "EN 1999-1-1 6.1.4 / 1-4", "library": "scipy.optimize.brentq + minimize_scalar", "function": "local_class_limits"},
    "A-62": {"checks": ["en1999.1-4.defl"], "clause": "EN 1999-1-1 7.2 / 1-4", "library": "anastruct + sectionproperties", "function": "deflection_and_rib"},
    "A-70": {"checks": ["en1999.1-5.buckle"], "clause": "EN 1999-1-5 5", "library": "scipy.optimize.minimize_scalar + sympy", "function": "axial_shell_constant"},
    "A-71": {"checks": ["en1999.1-5.regime"], "clause": "EN 1999-1-5 5.2 length regimes", "library": "scipy.optimize.brentq", "function": "shell_regimes"},
    "A-02": {"checks": ["en1999.6.1.6.haz"], "clause": "EN 1999-1-1 6.1.6.3 (b_haz table and reduced area)", "library": "sympy.Piecewise + shapely", "function": "haz_extent"},
    "A-21": {"checks": ["en1999.6.3.1.haz"], "clause": "EN 1999-1-1 6.3.1.1 (kappa, omega_x, chi)", "library": "scipy.optimize.brentq + numpy.interp + sectionproperties + shapely", "function": "member_haz"},
    "A-22": {"checks": ["en1999.6.3.2.ltb"], "clause": "EN 1999-1-1 6.3.2 (M_cr, chi_LT, M_b,Rd)", "library": "scipy.optimize.brentq + sectionproperties", "function": "lateral_torsional"},
    "A-72": {"checks": ["en1999.1-5.buckle"], "clause": "EN 1999-1-5 5 (HAZ rho_o,haz)", "library": "scipy.optimize.minimize_scalar + sympy", "function": "haz_shell_buckling"},
}


def close(name: str, ours: float, theirs: float, tol: float) -> dict[str, float | str]:
    rel = abs(ours - theirs) / abs(theirs)
    assert rel <= tol, f"{name}: {ours} vs {theirs} (rel {rel:.2e} > {tol})"
    return {"name": name, "ours": ours, "theirs": theirs, "rel": rel}


def beam_deflection(length: float, ei: float, load: float, elements: int = 12) -> float:
    system = SystemElements(EA=1e12, EI=ei)
    for i in range(elements):
        system.add_element([[length * i / elements, 0], [length * (i + 1) / elements, 0]])
    system.add_support_hinged(1)
    system.add_support_roll(elements + 1)
    system.q_load(q=-load, element_id=list(range(1, elements + 1)))
    system.solve()
    return abs(system.get_node_displacements(node_id=elements // 2 + 1)["uy"])


def rib_inertia(pitch: float, top: float, bottom: float, height: float, thickness: float) -> float:
    run = (pitch - top - bottom) / 2
    line = LineString([(-pitch / 2, 0), (-pitch / 2 + bottom / 2, 0), (-top / 2, height), (top / 2, height), (pitch / 2 - bottom / 2, 0), (pitch / 2, 0)])
    outline = line.buffer(thickness / 2, cap_style="flat", join_style="mitre")
    assert abs(run - (pitch / 2 - bottom / 2 - top / 2)) < 1e-12
    geometry = Geometry(outline)
    geometry.create_mesh(mesh_sizes=[thickness * thickness / 4])
    section = Section(geometry=geometry, time_info=False)
    section.calculate_geometric_properties()
    return section.get_ic()[0]


def deflection_and_rib() -> list[dict]:
    rows = []
    length, ei, load = 3.0, E * 2.0e-7, 1500.0
    ours = 5 * load * length ** 4 / (384 * ei)
    rows.append(close("A-62 5wL^4/(384EI) vs anastruct simply supported UDL", ours, beam_deflection(length, ei, load), 1e-6))
    pitch, top, bottom, height, t = 0.2, 0.06, 0.05, 0.04, 0.0012
    _, _, inertia, _, _ = ORACLE.rib_sections((pitch, top, bottom, height), t, [1.0, 1.0, 1.0])
    rows.append(close("A-62 rib second moment per pitch vs sectionproperties", inertia, rib_inertia(pitch, top, bottom, height, t), 2e-3))
    return rows


def local_class_limits() -> list[dict]:
    rows = []
    for heat_treated in (True, False):
        for outstand in (False, True):
            for welded in (False, True):
                limit = ORACLE.limits(heat_treated, outstand, welded)[2]
                c1, c2 = ORACLE.consts(heat_treated, outstand, welded)
                root = optimize.brentq(lambda beta: c1 / beta - c2 / beta ** 2 - 1.0, 2 * c2 / c1, limit * 3)
                rows.append(close(f"A-60 class 3 limit = root of C1/b - C2/b^2 = 1 (a={heat_treated}, outstand={outstand}, welded={welded})", float(limit), root, 3e-2))
    k_long = optimize.minimize_scalar(lambda x: (x + 1 / x) ** 2, bounds=(0.2, 5.0), method="bounded", options={"xatol": 1e-12})
    rows.append(close("A-60 plate buckling coefficient k = min (m b/a + a/(m b))^2", 4.0, k_long.fun, 1e-9))
    return rows


def axial_shell_constant(nu: float = 0.3) -> list[dict]:
    rows = []
    r, t = 1.0, 0.004
    d = E * t ** 3 / (12 * (1 - nu ** 2))
    load = lambda k: d * k ** 2 + E * t / (r ** 2 * k ** 2)
    found = optimize.minimize_scalar(lambda lk: load(math.exp(lk)), bounds=(-3.0, 8.0), method="bounded", options={"xatol": 1e-12})
    constant = found.fun / t / (E * t / r)
    rows.append(close("A-70 axisymmetric Donnell minimum 0.605 over the axial wave number (scipy)", 0.605, constant, 1e-3))
    k, dd, ee, tt, rr = sympy.symbols("k D E t r", positive=True)
    symbolic = sympy.simplify(sympy.minimum(dd * k ** 2 + ee * tt / (rr ** 2 * k ** 2), k, sympy.Interval.open(0, sympy.oo)))
    exact = float(symbolic.subs({dd: d, ee: E, tt: t, rr: r}))
    rows.append(close("A-70 closed minimum 2 sqrt(D E t)/r (sympy)", found.fun, exact, 1e-9))
    length = 0.6 * math.sqrt(r * t) * 1.7 * 5
    rows.append(close("A-70 sigma_x,cr of the oracle in the plateau range", ORACLE.sigma_x_cr(r, t, length), found.fun / t, 1e-3))
    return rows


def shell_utilisation(radius: float, thickness: float, length: float, welded: bool) -> float:
    action = {"id": "N", "kind": "permanent", "category": "selfWeight", "source": "external", "gKLine": 0.0, "qKLine": 0.0, "nK": 1.0e6, "vYK": 0.0, "vZK": 0.0, "mYK": 0.0, "mZK": 0.0, "torsionK": 0.0, "localLoadK": 0.0}
    shell = {"id": "s", "materialId": "m", "radius": radius, "thickness": thickness, "length": length, "welded": welded, "fabricationClass": "b", "actions": [action]}
    document = {"annex": "en", "reliabilityClass": "RC2", "materials": [{"id": "m", "designation": "aw6082-t6", "productForm": "extrudedProfile"}], "shells": [shell]}
    checks = ORACLE.evaluate(document)["checks"]
    return next(c["utilization"] for c in checks if c["id"] == "en1999.1-5.buckle.s")


def haz_shell_buckling() -> list[dict]:
    rows = []
    rho_o = ORACLE.alloy("aw6082-t6", 0.1)["rho_o"]
    stocky = (shell_utilisation(0.5, 0.1, 0.8, False), shell_utilisation(0.5, 0.1, 0.8, True))
    rows.append(close("A-72 plastic range: utilisation ratio welded/unwelded = 1/rho_o,haz (plain factor)", stocky[1] / stocky[0], 1 / rho_o, 1e-9))
    slender = (shell_utilisation(0.5, 0.0005, 0.9, False), shell_utilisation(0.5, 0.0005, 0.9, True))
    rows.append(close("A-72 elastic range: utilisation independent of rho_o,haz", slender[1], slender[0], 1e-9))
    return rows


def shell_regimes() -> list[dict]:
    rows = []
    short_limit = optimize.brentq(lambda w: 1.36 - 1.83 / w + 2.07 / w ** 2 - 1.0, 1.0, 3.0)
    rows.append(close("A-71 short-medium boundary w = 1.7 as the root of C_x = 1", 1.7, short_limit, 1e-3))
    for ratio in (50.0, 200.0, 800.0):
        long_limit = optimize.brentq(lambda w: 1 + 0.2 * (1 - 2 * w / ratio) - 1.0, 1.0, ratio)
        rows.append(close(f"A-71 medium-long boundary w = 0.5 r/t at r/t = {ratio:.0f}", 0.5 * ratio, long_limit, 1e-9))
    radius, thickness = 1.0, 0.004
    for factor, label in ((0.5, "short"), (10.0, "medium"), (0.75 * radius / thickness, "long")):
        length = factor * math.sqrt(radius * thickness)
        w = length / math.sqrt(radius * thickness)
        cx = 1.36 - 1.83 / w + 2.07 / w ** 2 if w < 1.7 else 1.0 if w <= 0.5 * radius / thickness else max(1 + 0.2 * (1 - 2 * w / (radius / thickness)), 0.6)
        rows.append(close(f"A-71 sigma_x,cr regime {label}", ORACLE.sigma_x_cr(radius, thickness, length), 0.605 * E * max(cx, 0.1) * thickness / radius, 1e-12))
    return rows


FO, RHO_O = 260e6, 0.48
H, B, TF, TW = 0.14, 0.07, 0.01, 0.008
GAMMA = ORACLE.GG * ORACLE.K_FI["RC2"]


def element(name: str, width: float, thickness: float, outstand: bool, welded: bool, position: float = 0.005) -> dict:
    return {"id": name, "width": width, "thickness": thickness, "outstand": outstand, "welded": welded, "weldPosition": position, "weldProcess": "mig", "weldTJunction": False, "weldDirection": "longitudinal", "edgeStiffenerDepth": 0.0, "edgeStiffenerThickness": 0.0}


def i_member_document(welded: bool, member: dict, action: dict) -> dict:
    flange, web = (B - TW) / 2, H - 2 * TF
    section = {"id": "s", "kind": "extrudedI", "height": H, "width": B, "flangeThickness": TF, "webThickness": TW, "outerDiameter": 0.0, "elements": [element("flange-top", flange, TF, True, welded), element("flange-bot", flange, TF, True, welded), element("web", web, TW, False, welded)]}
    base = {"id": "m", "sectionId": "s", "materialId": "m", "length": 4.0, "support": "simplySupported", "bucklingLengthY": 4.0, "bucklingLengthZ": 4.0, "bucklingLengthT": 4.0, "ltbLength": 4.0, "c1": 1.0, "restrainedLtb": False, "bearingLength": 0.0, "transverseWeld": False, "transverseWeldOffset": 0.0}
    act = {"id": "G", "kind": "permanent", "category": "selfWeight", "source": "external", "gKLine": 0.0, "qKLine": 0.0, "nK": 0.0, "vYK": 0.0, "vZK": 0.0, "mYK": 0.0, "mZK": 0.0, "torsionK": 0.0, "localLoadK": 0.0}
    return {"annex": "en", "reliabilityClass": "RC2", "materials": [{"id": "m", "designation": "aw6082-t6", "productForm": "extrudedProfile"}], "sections": [section], "members": [{**base, **member, "actions": [{**act, **action}]}]}


def utilisation(document: dict, check: str) -> float:
    return next(c["utilization"] for c in ORACLE.evaluate(document)["checks"] if c["id"] == f"{check}.m")


def extruded_i():
    geometry = i_section(d=H, b=B, t_f=TF, t_w=TW, r=0.0, n_r=1)
    geometry.create_mesh(mesh_sizes=[TF * TW / 6])
    section = Section(geometry=geometry, time_info=False)
    section.calculate_geometric_properties()
    section.calculate_warping_properties()
    section.calculate_plastic_properties()
    return section


def haz_extent() -> list[dict]:
    rows = []
    t = sympy.symbols("t", positive=True)
    table = sympy.Piecewise((0.020, t <= 0.006), (0.030, t <= 0.012), (0.035, t <= 0.025), (0.040, True))
    for thickness, process, junction in ((0.004, "mig", False), (0.006, "mig", False), (0.0061, "mig", False), (0.012, "mig", False), (0.02, "mig", False), (0.025, "mig", False), (0.03, "mig", False), (0.01, "tig", False), (0.01, "mig", True), (0.01, "tig", True)):
        element_ = element("e", 0.1, thickness, True, True) | {"weldProcess": process, "weldTJunction": junction}
        theirs = float(table.subs(t, thickness)) * (1.5 if process == "tig" else 1.0) * (1.33 if junction else 1.0)
        rows.append({**close(f"A-02 en1999.6.1.6.haz b_haz t={thickness} {process} T={junction} vs sympy Piecewise", ORACLE.haz_width_for(element_), theirs, 1e-12), "element": element_})
    flange, web = (B - TW) / 2, H - 2 * TF
    section = i_member_document(True, {}, {})["sections"][0]
    bh = float(table.subs(t, TF))
    zones = [box(0, 0, flange, TF).intersection(box(0.005 - bh, 0, 0.005 + bh, TF)).area] * 4
    web_zone = box(0, 0, TW, web).intersection(box(0, 0, TW, 0.005 + float(table.subs(t, TW)))).area
    reduced = 2 * (2 * flange + TW) * TF + web * TW - (1 - RHO_O) * (sum(zones) + web_zone)
    ours = ORACLE.props(section, {"fo": FO, "a": True, "eta": 1.0, "local": False, "haz": RHO_O})["a"]
    rows.append(close("A-02 en1999.6.1.6.haz reduced area A - (1-rho_o,haz) A_haz vs shapely strip intersection", ours, reduced, 1e-3))
    return rows


def chi_ayrton_perry(lam: float, alpha: float, lam0: float) -> float:
    if lam <= lam0:
        return 1.0
    return optimize.brentq(lambda x: (1 - x) * (1 - x * lam ** 2) - alpha * (lam - lam0) * x, 1e-9, 1.0)


def member_haz() -> list[dict]:
    rows = []
    reference = extruded_i()
    area = reference.get_area()
    iy, iz = reference.get_ic()[0], reference.get_ic()[1]
    length, load = 4.0, 1.0e4
    action = {"nK": -load}
    for lam in (0.1, 0.5, 0.9, 1.5):
        k0 = 0.8
        rows.append(close(f"A-21 en1999.6.3.1.haz kappa at lambda={lam} vs numpy.interp", ORACLE.kappa(k0, lam), float(numpy.interp(lam, [0.2, 1.2], [k0, 1.0])), 1e-12))
    flange = (B - TW) / 2
    bh_flange, bh_web = 0.030, 0.030
    zone = 4 * min(flange, 0.005 + bh_flange) * TF + (0.005 + bh_web) * TW
    k0 = (area - (1 - RHO_O) * zone) / area
    for buckling in (0.5, 1.0):
        for label, inertia, key in (("y", iy, "en1999.6.3.1.fb"), ("z", iz, "en1999.6.3.1.fb.z")):
            lam = math.sqrt(area * FO / (math.pi ** 2 * E * inertia / buckling ** 2))
            chi = chi_ayrton_perry(lam, 0.20, 0.10)
            kappa = k0 + (1 - k0) * float(numpy.interp(lam, [0.2, 1.2], [0.0, 1.0]))
            lengths = {"bucklingLengthY": buckling, "bucklingLengthZ": buckling, "bucklingLengthT": buckling}
            nb = kappa * chi * area * FO / ORACLE.GM1
            welded_document = i_member_document(True, lengths, action)
            rows.append({**close(f"A-21 en1999.6.3.1.haz welded kappa*chi N_b,Rd Lc={buckling} axis {label}: utilisation", utilisation(welded_document, key), GAMMA * load / nb, 5e-2), "document": welded_document, "check_id": f"{key}.m", "k0": k0, "kappa": kappa, "chi": chi, "lambda": lam})
            offset = buckling / 4
            omega = 1.0 - (1.0 - min(RHO_O / chi, 1.0)) * abs(math.sin(math.pi * offset / buckling))
            nb = omega * chi * area * FO / ORACLE.GM1
            document = i_member_document(False, {**lengths, "transverseWeld": True, "transverseWeldOffset": offset}, action)
            rows.append({**close(f"A-21 en1999.6.3.1.haz transverse weld omega_x Lc={buckling} axis {label}: utilisation", utilisation(document, key), GAMMA * load / nb, 5e-2), "omega": omega, "chi": chi, "lambda": lam, "document": document, "check_id": f"{key}.m"})
    return rows


def lateral_torsional() -> list[dict]:
    rows = []
    reference = extruded_i()
    iz = reference.get_ic()[1]
    torsion, warping = reference.get_j(), reference.get_gamma()
    modulus_el, modulus_pl = reference.get_z()[0], reference.get_s()[0]
    shape = min(max(modulus_pl / modulus_el, 1.0), 1.25)
    for length, c1 in ((2.0, 1.0), (4.0, 1.0), (6.0, 1.132)):
        k = math.pi / length
        determinant = lambda m: E * iz * k ** 4 * (E * warping * k ** 4 + G * torsion * k ** 2) - (m * k ** 2) ** 2
        mcr_unit = optimize.brentq(determinant, 1.0, 1.0e9) * c1
        closed = c1 * math.pi ** 2 * E * iz / length ** 2 * math.sqrt(warping / iz + length ** 2 * G * torsion / (math.pi ** 2 * E * iz))
        rows.append(close(f"A-22 en1999.6.3.2.ltb M_cr L={length} brentq of the Vlasov determinant vs closed form", mcr_unit, closed, 1e-9))
        lam = math.sqrt(shape * modulus_el * FO / mcr_unit)
        chi = chi_ayrton_perry(lam, 0.10, 0.6)
        mb = chi * shape * modulus_el * FO / ORACLE.GM1
        moment = 4.0e3
        document = i_member_document(False, {"ltbLength": length, "c1": c1}, {"mYK": moment})
        rows.append({**close(f"A-22 en1999.6.3.2.ltb utilisation M_Ed/M_b,Rd L={length}", utilisation(document, "en1999.6.3.2.ltb"), GAMMA * moment / mb, 5e-2), "mcr_third_party": mcr_unit, "chi": chi, "lambda": lam, "document": document, "check_id": "en1999.6.3.2.ltb.m"})
    return rows


def main() -> None:
    rows = []
    for function in (deflection_and_rib, local_class_limits, axial_shell_constant, haz_shell_buckling, shell_regimes, haz_extent, member_haz, lateral_torsional):
        rows += function()
    json.dump({"spec_rows": SPEC_ROWS, "rows": rows}, sys.stdout, ensure_ascii=False)
    print()


if __name__ == "__main__":
    main()
