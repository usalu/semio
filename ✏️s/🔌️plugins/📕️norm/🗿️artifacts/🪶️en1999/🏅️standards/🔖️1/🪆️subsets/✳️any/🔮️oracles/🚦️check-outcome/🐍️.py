"""🚦️ Independent EN 1999 document oracle for the check-outcome case: a committed snapshot (JSON on stdin or a path) to the utilisation of every check family of the spec-trace rows A-02 to A-80 (`--json`, default: `{"checks":[{"id","utilization"[,"status"]}]}`; `--self-check`: section constants against `sectionproperties`, the Ayrton-Perry root against `scipy.optimize.brentq`, the simple-beam deflection against PyNite). It wraps the second implementation `../⚖️compliance/🐍️.py` (class, resistances 6.2, buckling 6.3, patch loading, bolts, welds, fatigue, fire, sheeting, shells, deflection) and adds what that one does not report: the HAZ extent of each weld run (A-02, `en1999.6.1.6.haz.<run>`, reach over element width, status `warning` instead of `fail` above 1) and the welded-member buckling row (A-21, `en1999.6.3.1.haz.<member>`, the worst axis of the buckling rows with the HAZ factors kappa and omega_x). A run's `hazExtent` overrides the table of 6.1.6.3 in the strips as well. Written from EN 1999-1-1, not from the Rust."""

import importlib.util
import json
import math
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
BASE_PATH = HERE.parent / "⚖️compliance" / "🐍️.py"


def _limit_rule():
    for parent in pathlib.Path(__file__).resolve().parents:
        path = parent / "🔮️oracles" / "🏃️harness" / "📏️tolerance" / "🐍️.py"
        if path.is_file():
            spec = importlib.util.spec_from_file_location("norm_limit_rule", path)
            module = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(module)
            return module
    raise FileNotFoundError("oracle tolerance module")


LIMIT = _limit_rule()


def base():
    spec = importlib.util.spec_from_file_location("en1999_compliance", BASE_PATH)
    module = importlib.util.module_from_spec(spec)
    sys.modules["en1999_compliance"] = module
    spec.loader.exec_module(module)
    original_apply, original_width = module.apply_runs, module.haz_width_for

    def apply_runs(member, section, runs):
        m, s = original_apply(member, section, runs)
        for run in runs:
            if run.get("hazExtent", 0.0) > 0:
                for el in s.get("elements", []):
                    if el["id"] == run["elementId"]:
                        el["hazOverride"] = max(el.get("hazOverride", 0.0), run["hazExtent"])
        return m, s

    module.apply_runs = apply_runs
    module.haz_width_for = lambda el: el["hazOverride"] if el.get("hazOverride") else original_width(el)
    return module


def haz_rows(doc, module):
    """🧵 A-02: reach of the HAZ of each weld run (both sides of a longitudinal run, the full width of a transverse one) over the element width."""
    sections = {s["id"]: s for s in doc.get("sections", [])}
    members = {m["id"]: m for m in doc.get("members", [])}
    rows = []
    for run in doc.get("weldRuns", []):
        member = members.get(run["memberId"])
        section = sections.get(member["sectionId"]) if member else None
        element = next((e for e in section.get("elements", []) if e["id"] == run["elementId"]), None) if section else None
        if element is None or run["process"] not in ("mig", "tig"):
            continue
        b_haz = run["hazExtent"] if run["hazExtent"] > 0 else module.haz_width(element["thickness"]) * (1.5 if run["process"] == "tig" else 1.0) * (1.33 if run["tJunction"] else 1.0)
        reach = element["width"] if run["direction"] == "transverse" else 2 * b_haz
        u = reach / max(element["width"], 1e-9)
        rows.append({"id": f"en1999.6.1.6.haz.{run['id']}", "utilization": u, "status": LIMIT.limit_status(u)})
    return rows


def edge_rows(doc, module):
    """🪝 6.7.7: a flat-bar lip d x t_s on an outstand b x t is a Winkler strut on the cantilever strip K = E t^3 / (4 (1 - nu^2) b^3) (tip stiffness 3 E I_p / b^3 of the strip per metre, I_p = t^3 / (12 (1 - nu^2))): A_s = d t_s, I_s = t_s d^3 / 12, sigma_cr,s = 2 sqrt(K E I_s) / A_s, utilisation (f_o / sigma_cr,s) / lambda_s,max^2 with the registry row for lambda_s,max; E = 70 GPa, nu = 0.3."""
    e_mod, nu = 70e9, 0.3
    limit = module.NDP["ndp.en1999.stiffener.edge-slenderness-limit"]["recommended"]
    sections = {s["id"]: s for s in doc.get("sections", [])}
    materials = {m["id"]: m for m in doc.get("materials", [])}
    rows = []
    for member in doc.get("members", []):
        section = sections.get(member["sectionId"])
        material = materials.get(member.get("materialId"))
        alloy = module.alloy_for_section(material["designation"], section) if section and material else None
        if alloy is None:
            continue
        for el in section.get("elements", []):
            if not (el.get("edgeStiffenerDepth", 0) > 0 and el.get("edgeStiffenerThickness", 0) > 0 and el.get("outstand") and el["width"] > 0 and el["thickness"] > 0):
                continue
            k = e_mod * el["thickness"] ** 3 / (4.0 * (1.0 - nu * nu) * el["width"] ** 3)
            a_s, i_s = el["edgeStiffenerDepth"] * el["edgeStiffenerThickness"], el["edgeStiffenerThickness"] * el["edgeStiffenerDepth"] ** 3 / 12.0
            sigma_cr = 2.0 * math.sqrt(k * e_mod * i_s) / a_s
            u = alloy["fo"] / sigma_cr / limit ** 2
            rows.append({"id": f"en1999.6.7.7.edge.{member['id']}.{el['id']}", "utilization": u, "status": "unverifiedBasis" if LIMIT.within_limit(u) else "fail"})
    return rows


def crippling_rows(doc, module):
    """🧱 1-4 6.1.7: the web-crippling coefficients alpha, c_r, c_a, c_l, c_phi of the end and interior positions are null in the registry, so a patch load on a cold-formed sheet is `unverifiedBasis` without a resistance (utilisation 0); a registry with values would need the resistance formula and raises instead of guessing."""
    sheets = {c["id"]: c for c in doc.get("coldFormed", [])}
    materials = {m["id"]: m for m in doc.get("materials", [])}
    rows = []
    for load in doc.get("patchLoads", []):
        sheet = sheets.get(load["subjectId"])
        material = materials.get(sheet["materialId"]) if sheet else None
        if material is None or module.alloy(material["designation"], sheet["thickness"]) is None:
            continue
        position = "end" if load["position"] == "end" else "interior"
        if any(module.NDP[f"ndp.en1999.web-crippling.{position}.{key}"]["recommended"] is not None for key in ("alpha", "c-r", "c-a", "c-l", "c-phi")):
            raise NotImplementedError("the web-crippling coefficients have values; the resistance of 6.1.7 is not written yet")
        rows.append({"id": f"en1999.1-4.web-crippling.{load['id']}", "utilization": 0.0, "status": "unverifiedBasis"})
    return rows


def evaluate(doc):
    module = base()
    rows = module.evaluate(doc)["checks"]
    by_id = {r["id"]: r["utilization"] for r in rows}
    sections = {s["id"]: s for s in doc.get("sections", [])}
    runs = doc.get("weldRuns", [])
    for member in doc.get("members", []):
        section = sections.get(member["sectionId"])
        if section is None or section["kind"] == "free":
            continue
        welded = any(e.get("welded") for e in section.get("elements", [])) or any(r["memberId"] == member["id"] for r in runs) or member.get("transverseWeld")
        keys = [f"en1999.6.3.1.fb.{member['id']}", f"en1999.6.3.1.fb.z.{member['id']}"]
        if welded and all(k in by_id for k in keys):
            rows.append({"id": f"en1999.6.3.1.haz.{member['id']}", "utilization": max(by_id[k] for k in keys)})
    unverified = ("en1999.6.7.5.web.", "en1999.6.7.6.web-")
    for row in rows:
        if row["id"].startswith(unverified):
            row["status"] = ("unverifiedBasis" if LIMIT.within_limit(row["utilization"]) else "fail")
    return {"checks": rows + haz_rows(doc, module) + edge_rows(doc, module) + crippling_rows(doc, module)}


def self_check():
    """📐 Section constants, the buckling root and the beam deflection against the libraries."""
    from sectionproperties.analysis import Section
    from sectionproperties.pre.library import i_section
    import numpy as np
    from scipy.optimize import brentq

    module = base()
    s = dict(kind="extrudedI", height=0.14, width=0.07, flangeThickness=0.01, webThickness=0.008, outerDiameter=0.0, elements=[])
    geo = module.geometry(s)
    section = Section(i_section(d=0.14, b=0.07, t_f=0.01, t_w=0.008, r=0.0, n_r=1).create_mesh(mesh_sizes=[1e-5]))
    section.calculate_geometric_properties()
    section.calculate_warping_properties()
    area, (ixx, iyy, _) = section.get_area(), section.get_ic()
    j, gamma = section.get_j(), section.get_gamma()
    result = {"area": abs(geo["a"] / area - 1), "iy": abs(geo["iy"] / ixx - 1), "iz": abs(geo["iz"] / iyy - 1), "it": abs(geo["it"] / j - 1), "iw": abs(geo["iw"] / gamma - 1)}
    assert max(result["area"], result["iy"], result["iz"]) < 1e-9 and result["it"] < 0.06 and result["iw"] < 0.02, result
    for lam, alpha, lam0 in [(0.5, 0.2, 0.1), (1.2, 0.2, 0.1), (2.0, 0.32, 0.0), (0.15, 0.2, 0.1)]:
        k = alpha * (lam - lam0)
        root = brentq(lambda x: (1 - x) * (1 - x * lam * lam) - k * x, 1e-9, 1.0) if lam > lam0 else 1.0
        got = module.chi_ap(lam, alpha, lam0)
        result[f"chi_{lam}"] = abs(got - root)
        assert abs(got - root) < 1e-9 or root == 1.0 and got == 1.0, (lam, got, root)
    from sectionproperties.pre import Geometry
    from shapely.geometry import box

    r, t, depth, bar_t = 0.5, 0.01, 0.06, 0.01
    b_e = module.ndp("ring-stiffener.effective-width-factor") * math.sqrt(r * t)
    ring = Section(Geometry(box(-b_e / 2, -t / 2, b_e / 2, t / 2).union(box(-bar_t / 2, t / 2, bar_t / 2, t / 2 + depth))).create_mesh(mesh_sizes=[2e-6]))
    ring.calculate_geometric_properties()
    a_s, a_b = b_e * t, depth * bar_t
    z_b = 0.5 * t + 0.5 * depth
    z_c = a_b * z_b / (a_s + a_b)
    i_ring = b_e * t ** 3 / 12 + a_s * z_c ** 2 + bar_t * depth ** 3 / 12 + a_b * (z_b - z_c) ** 2
    w_cr = min((n * n - 1) * module.E * ring.get_ic()[0] / r ** 3 for n in range(2, 12))
    result["ring_inertia"] = abs(i_ring / ring.get_ic()[0] - 1)
    result["ring_w_cr"] = abs(3 * module.E * i_ring / r ** 3 / w_cr - 1)
    assert result["ring_inertia"] < 1e-3 and result["ring_w_cr"] < 1e-3, result
    from shapely.geometry import LineString
    from scipy.integrate import solve_ivp

    p_, top, bot, h_, t_ = 0.2, 0.06, 0.06, 0.04, 0.001
    proj = (p_ - top - bot) / 2
    outline = LineString([(0.0, 0.0), (bot, 0.0), (bot + proj, h_), (bot + proj + top, h_), (bot + 2 * proj + top, 0.0)]).buffer(t_ / 2, cap_style=2, join_style=2)
    rib_section = Section(Geometry(outline).create_mesh(mesh_sizes=[(t_ / 3) ** 2]))
    rib_section.calculate_geometric_properties()
    area_r, zc_r, i_r, _, _ = module.rib_sections((p_, top, bot, h_), t_, [1.0, 1.0, 1.0], 1500)
    result["rib_area"], result["rib_zc"], result["rib_inertia"] = abs(area_r / rib_section.get_area() - 1), abs(zc_r / rib_section.get_c()[1] - 1), abs(i_r / rib_section.get_ic()[0] - 1)
    assert result["rib_area"] < 1e-6 and result["rib_zc"] < 1e-3 and result["rib_inertia"] < 1e-3, result
    ins = dict(thickness=0.02, conductivity=0.2, specificHeat=1200.0, density=800.0)
    k1, k2 = module.ndp("fire-protected-denominator"), module.ndp("fire-protected-exponent")

    def rate(t, y):
        ca = 0.41 * y[0] + 903
        phi = ins["specificHeat"] * ins["density"] / (ca * 2700) * ins["thickness"] * 200.0
        g_dot = 345 * 8 / 60 / ((8 * t / 60 + 1) * math.log(10))
        return [max(ins["conductivity"] / ins["thickness"] * 200.0 / (ca * 2700) * (module.gas(t) - y[0]) / (1 + phi / k1) - (math.exp(phi / k2) - 1) * g_dot, 0.0)]

    ode = solve_ivp(rate, (0, 900), [20.0], t_eval=[900], max_step=1.0, rtol=1e-8).y[0][0]
    result["protected_fire"] = abs((module.protected_temperature(200.0, 900.0, 20.0, ins) - 20.0) / (ode - 20.0) - 1)
    assert result["protected_fire"] < 0.02, result
    from Pynite import FEModel3D

    length, e, m_ed = 6.0, 70e9, 40e3
    model = FEModel3D()
    model.add_material("Al", e, 27e9, 0.3, 2700)
    for name, x in (("A", 0.0), ("B", length)):
        model.add_node(name, x, 0, 0)
    model.add_section("Sec", geo["a"], geo["iy"], geo["iy"], geo["it"])
    model.add_member("M", "A", "B", "Al", "Sec")
    model.def_support("A", True, True, True, True, False, False)
    model.def_support("B", False, True, True, False, False, False)
    model.add_member_dist_load("M", "Fy", -8 * m_ed / length ** 2, -8 * m_ed / length ** 2)
    model.analyze()
    theirs = abs(model.members["M"].deflection("dy", 0.5 * length))
    ours = 5 / 48 * m_ed * length ** 2 / (e * geo["iy"])
    result["deflection"] = abs(ours / theirs - 1)
    assert result["deflection"] < 1e-3, result
    return result


if __name__ == "__main__":
    if "--self-check" in sys.argv:
        print(json.dumps(self_check()))
        sys.exit(0)
    paths = [a for a in sys.argv[1:] if not a.startswith("--")]
    document = json.load(open(paths[0], encoding="utf8")) if paths else json.load(sys.stdin)
    print(json.dumps(evaluate(document)))
