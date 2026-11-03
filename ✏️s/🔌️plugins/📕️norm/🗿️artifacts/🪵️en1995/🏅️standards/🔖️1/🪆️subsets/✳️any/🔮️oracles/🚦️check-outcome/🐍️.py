"""🚦️ Independent EN 1995 document oracle for the check-outcome case: a committed snapshot (JSON on stdin or a path) to the utilisation of every check family of the spec-trace `check` rows (`--json`, default: `{"checks":[{"id","utilization"}]}`; `--self-check`: rectangle section constants against `sectionproperties`, the buckling factor k_c against `scipy.optimize.brentq`, the simple-beam deflection against PyNite). It wraps `../⚖️compliance/🐍️.py` (members 6.1.x, 6.3.x, 7.2, 7.3, fire, bridges, connections 8.x, floors) and adds the document's design cases (`cases`, id `en1995.t-nn.<outcome>.<case id>`): T-14 biaxial bending, T-18 net section (5.2), T-30 tapered edge at the governing section of the moment line (6.38, 6.39, both edge states, `l` and `load_case` required), T-31 apex bending (6.41), T-32 apex tension perpendicular (6.50), T-33 apex reinforcement force, T-45 notch shear (6.60) and T-46 reinforced notch force (NA-DE NCI). Rows without a utilisation (unknown strength class) are dropped, as the Rust reports them not applicable. Written from EN 1995-1-1 and DIN EN 1995-1-1/NA, not from the Rust."""

import importlib.util
import json
import math
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
BASE_PATH = HERE.parent / "⚖️compliance" / "🐍️.py"


def base():
    spec = importlib.util.spec_from_file_location("en1995_compliance", BASE_PATH)
    module = importlib.util.module_from_spec(spec)
    sys.modules["en1995_compliance"] = module
    spec.loader.exec_module(module)
    count_reads(module)
    return module


UNVERIFIED_READS = ("t-97.", "t-122.prestress_min.", "t-122.friction.", "t-122.plate_bearing.", "6.1.6.bending.", "6.1.7.shear.", "6.2.3.tension.", "6.3.2.compression.", "6.3.2.c.combined.", "6.1.5.c90.", "7.2.wfin.", "7.2.floor-creep.", ".johansen.", "2.uls.bending.")
READS = [0]


class CountingDict(dict):
    def __getitem__(self, key):
        READS[0] += 1
        return super().__getitem__(key)


def count_reads(module):
    """🔎 Counts every k_mod and k_def read: the registry rows `ndp.en1995.kmod.*` and `ndp.en1995.kdef.*` are unverified (no independent source), so a check that reads them ends in `UnverifiedBasis`."""
    original = module.k_mod

    def counted(*args, **kwargs):
        READS[0] += 1
        return original(*args, **kwargs)

    module.k_mod = counted
    module.K_DEF_PRODUCT = CountingDict(module.K_DEF_PRODUCT)


def kp(alpha, hr):
    t = math.tan(alpha)
    return 0.2 * t + (0.25 - 1.5 * t + 2.6 * t * t) * hr + (2.1 * t - 4.0 * t * t) * hr * hr


def gamma_method(n, slip):
    """🧩 Annex B of a two or three part mechanically jointed beam (part 2 is the web, gamma_2 = 1; parts 1 and 3 are connected with the spacing s_i and the slip modulus `slip` per row, span l): the gamma factors, the distances a_i and (EI)_ef, written from the clause formulas."""
    parts = [(n["a1"], n["i1"], n["e1"], n["h1"], n["s1"]), (n["a2"], n["i2"], n["e2"], n["h2"], 1.0)]
    if "a3" in n:
        parts.append((n["a3"], n["i3"], n["e3"], n["h3"], n["s3"]))
    gamma = [1.0 / (1.0 + math.pi ** 2 * e * a * s / (slip * n["l"] ** 2)) if i != 1 else 1.0 for i, (a, _, e, _, s) in enumerate(parts)]
    ea = [g * e * a for g, (a, _, e, _, _) in zip(gamma, parts)]
    a2 = (ea[0] * (parts[0][3] + parts[1][3]) - (ea[2] * (parts[1][3] + parts[2][3]) if len(parts) == 3 else 0.0)) / (2.0 * sum(ea))
    dist = [(parts[0][3] + parts[1][3]) / 2.0 - a2, a2] + ([(parts[1][3] + parts[2][3]) / 2.0 + a2] if len(parts) == 3 else [])
    ei = sum(e * i + g * e * a * d * d for g, (a, i, e, _, _), d in zip(gamma, parts, dist))
    return parts, gamma, dist, ei


def jointed_rows(n):
    """🧩 `fibre_<i>`, `web_shear`, `fastener_<i>` and `deflection` outcomes of T-90 (Annex B: sigma_i = gamma_i E_i a_i M / (EI)_ef, sigma_m,i = 0.5 E_i h_i M / (EI)_ef, B.3 summation per part, F_i = gamma_i E_i A_i a_i s_i V / (EI)_ef, tau_2 = 0.5 E_2 (h_2 / 2 + a_2)^2 V / (EI)_ef, w = 5 q l^4 / (384 (EI)_ef) against l / 300)."""
    parts, gamma, dist, ei = gamma_method(n, n["k"])
    m, v, out = n["m_d"], n.get("v_d", 0.0), {}
    for i, (a, _, e, h, s) in enumerate(parts):
        if f"fd_a{i + 1}" in n and a > 0.0:
            out[f"fibre_{i + 1}"] = abs(gamma[i] * e * dist[i] * m / ei) / n[f"fd_a{i + 1}"] + abs(0.5 * e * h * m / ei) / n.get(f"fd_m{i + 1}", math.inf)
        if i != 1 and f"r_d{i + 1}" in n:
            out[f"fastener_{i + 1}"] = abs(gamma[i] * e * a * dist[i] * s * v / ei) / n[f"r_d{i + 1}"]
    if "fv_d_2" in n:
        out["web_shear"] = 0.5 * parts[1][2] * (parts[1][3] / 2.0 + dist[1]) ** 2 * v / ei / n["fv_d_2"]
    if "q_d" in n:
        out["deflection"] = 5.0 * n["q_d"] * n["l"] ** 4 / (384.0 * ei) / (n["l"] / n.get("limit_divisor", 300.0))
    return out


def stabilising_load(o, annex, n_braced, n_d, length_m):
    """🪢 q_d = k_l n N_d / (k_f3 l) in N/m with k_l = min(1, sqrt(15 / l)) (EN 1995-1-1 9.2.5.3(1), Expressions 9.37, 9.38; the 15 m and k_f3 are registry rows)."""
    return min(1.0, math.sqrt(o.ndp("bracing.kl-reference-length", annex) / length_m)) * n_braced * n_d / (o.ndp("bracing.stabilising-divisor", annex) * length_m)


def entity_rows(doc, o):
    """🧱 Utilisations of the typed diaphragms (T-94: wall racking by method A or B 9.2.4, hold-down F h / b, diaphragm shear flow F_d / l 9.2.3), trusses (T-95: q_d against the bracing capacity) and bracings (T-96: sum F_d = q_d l against the resistance and F / k_br against l / divisor) as `en1995.t-9x.<outcome>.<collection>:<id>:T-9x`."""
    annex = o.annex_of(doc)
    rows = []
    for d in doc.get("diaphragms") or []:
        name, key = d["id"], f"diaphragms:{d['id']}:T-94"
        if d["kind"] == "wall":
            if min(d["panelWidthM"], d["wallHeightM"], d["fastenerSpacingM"]) <= 0 or d["panels"] == 0:
                continue
            b, h, spacing = d["panelWidthM"] * 1e3, d["wallHeightM"] * 1e3, d["fastenerSpacingM"] * 1e3
            c_i = 1.0 if b >= h / 2.0 else b / (h / 2.0)
            k = 1.0 if d["method"] == "method-a" else math.prod(o.ndp(f"diaphragm.method-b-k-{x}", annex) for x in ("d", "q", "n", "op"))
            resistance = k * d["panels"] * d["fastenerCapacityN"] * b * c_i / spacing
            reads = d["method"] != "method-a"
            rows.append({"id": f"en1995.t-94.racking.{key}", "utilization": d["shearEdN"] / resistance, "reads": reads})
            if d["holdDownN"] > 0:
                rows.append({"id": f"en1995.t-94.hold_down.{key}", "utilization": max(d["shearEdN"] * h / b, 0.0) / d["holdDownN"], "reads": False})
        elif d["diaphragmLengthM"] > 0 and d["shearResistanceNPerM"] > 0:
            rows.append({"id": f"en1995.t-94.diaphragm_shear.{key}", "utilization": d["diaphragmForceN"] / (d["diaphragmLengthM"] * 1e3) / (d["shearResistanceNPerM"] / 1e3), "reads": False})
    for t in doc.get("trusses") or []:
        if min(t["spanM"], t["heightM"], t["restraintLengthM"]) > 0 and t["bays"] >= 2 and t["bracingCapacityNPerM"] > 0:
            q_d = stabilising_load(o, annex, 1.0, t["chordForceN"], t["restraintLengthM"])
            rows.append({"id": f"en1995.t-95.bracing_capacity.trusses:{t['id']}:T-95", "utilization": q_d / t["bracingCapacityNPerM"], "reads": True})
    for b in doc.get("bracings") or []:
        if b["lengthM"] > 0 and b["stabilisedCount"] > 0 and b["stiffnessNPerM"] > 0 and any(m["id"] == b["memberId"] for m in doc["members"]):
            force = stabilising_load(o, annex, b["stabilisedCount"], b["compressionForceN"], b["lengthM"]) * b["lengthM"]
            key = f"bracings:{b['id']}:T-96"
            rows.append({"id": f"en1995.t-96.bracing_force.{key}", "utilization": force / b["resistanceN"], "reads": True})
            rows.append({"id": f"en1995.t-96.bracing_deflection.{key}", "utilization": force / (b["stiffnessNPerM"] / 1e3) / (b["lengthM"] * 1e3 / o.ndp("bracing.deflection-limit-divisor", annex)), "reads": True})
    return rows


def case_rows(doc, o):
    """🗂️ Design cases to `en1995.<entry>.<outcome>.<case id>` utilisations (N, mm, N/mm2)."""
    annex = o.annex_of(doc)
    rows = []
    for item in doc.get("cases") or []:
        reads = READS[0]
        n = {x["id"]: x["value"] for x in item.get("numbers", [])}
        t = {x["id"]: x["value"] for x in item.get("texts", [])}
        case = {"annex": annex, "num": n, "text": t}
        entry = item["entryId"]
        row = o.props(t.get("class", ""))
        if row is None:
            continue
        key = entry.lower()
        fd = lambda fk: o._fd(case, row, fk) / o.MPA
        prod = row["product"]

        def add(name, value):
            rows.append({"id": f"en1995.{key}.{name}.{item['id']}", "utilization": value, "reads": READS[0] > reads})

        if entry == "T-14":
            for name, value in o.cat_t14(case).items():
                add(name, value)
        elif entry == "T-18":
            holes = n.get("hole_d", 0.0) * n.get("hole_count", 0.0)
            a_net = n["b"] * n["h"] - holes * n["b"]
            dim = n.get("kh_dim") or (max(n["b"], n["h"]) if prod == "solid" else n["b"])
            add("net_section", n["n_t"] / a_net / (o.k_h(dim / 1000.0, prod, row["rho"]) * fd(row["ft0"])))
            if "hole_d" in n:
                add("hole_limit", n["hole_d"] / (0.15 * n["h"]))
        elif entry == "T-30":
            _, tapered, parallel = o._taper_governing(case)
            add("tapered_edge", tapered)
            add("parallel_edge", parallel)
            add("taper_angle", n["alpha_deg"] / 10.0)
        elif entry == "T-31":
            for name, value in o.cat_t31(case).items():
                add(name, value)
        elif entry in ("T-32", "T-33"):
            alpha = math.radians(n.get("alpha_ap_deg", 0.0))
            hr = n["h_ap"] / (n["r_in"] + 0.5 * n["h_ap"])
            sd = kp(alpha, hr) * 6 * n["m_ap"] / (n["b"] * n["h_ap"] ** 2)
            if entry == "T-32":
                cambered = t.get("shape", "double_tapered") == "pitched_cambered"
                zone = n.get("v_apex") or n["b"] * n["h_ap"] ** 2 * (1 - 0.25 * math.tan(alpha))
                volume = min(zone, 2 / 3 * n["v_beam"])
                if cambered:
                    sd -= 0.6 * n.get("p_d", 0.0) / n["b"]
                capacity = (1.7 if cambered else 1.4) * (1e7 / volume) ** 0.2 * fd(row["ft90"])
                add("apex_tension90", sd / capacity)
                if "tau_d" in n:
                    add("apex_shear_tension", n["tau_d"] / fd(row["fv"]) + sd / capacity)
            else:
                kmod = n.get("kmod") or o.k_mod(n.get("sc", 1), o.duration_rank(t.get("duration", "medium")))
                add("apex_reinforcement", 0.5 * sd * n["b"] * n["h_ap"] / (n["n_rods"] * n["r_ax_k"] * kmod / n.get("gamma_m_conn", 1.3)))
        elif entry == "T-90":
            for name, value in jointed_rows(n).items():
                add(name, value)
        elif entry == "T-45":
            add("notch_shear", o.cat_t45(case)["notch_shear"])
        elif entry == "T-46":
            alpha = n["h_ef"] / n["h"]
            kmod = n.get("kmod") or o.k_mod(n.get("sc", 1), o.duration_rank(t.get("duration", "medium")))
            ft = 1.3 * n["v_d"] * (3 * (1 - alpha) ** 2 - 2 * (1 - alpha) ** 3)
            add("notch_reinforcement", ft / (n["n_rods"] * n["r_ax_k"] * kmod / n.get("gamma_m_conn", 1.3)))
    return rows


def evaluate(doc):
    o = base()
    rows = [dict(r, reads=any(key in r["id"] for key in UNVERIFIED_READS)) for r in o.evaluate(doc)["checks"] if r["utilization"] is not None]
    return {"checks": rows + case_rows(doc, o) + entity_rows(doc, o)}


def self_check():
    """📐 Rectangle constants, k_c and the beam deflection against the libraries."""
    from scipy.optimize import brentq
    from sectionproperties.analysis import Section
    from sectionproperties.pre.library import rectangular_section
    from Pynite import FEModel3D

    o = base()
    b, h = 0.2, 0.4
    section = Section(rectangular_section(d=h, b=b).create_mesh(mesh_sizes=[1e-4]))
    section.calculate_geometric_properties()
    zxx = section.get_z()[0]
    result = {"area": abs(b * h / section.get_area() - 1), "w_y": abs(b * h * h / 6 / zxx - 1)}
    assert max(result.values()) < 1e-9, result
    for lam in (0.5, 0.9, 1.6):
        kk = 0.5 * (1 + 0.2 * (lam - 0.3) + lam * lam)
        ours = o.k_c(lam, 0.2)
        root = brentq(lambda kc: (1 / kc - kk) ** 2 - (kk * kk - lam * lam), 1e-6, 1.0)
        result[f"k_c_{lam}"] = abs(ours - root)
        assert abs(ours - root) < 1e-9, (lam, ours, root)
    p = o.props("GL28h")
    member = {"bM": b, "hM": h, "spanM": 5.0, "support": "simplySupported"}
    q = 6000.0
    model = FEModel3D()
    model.add_material("T", p["e0"], p["e0"] / 16, 0.3, 450)
    for name, x in (("A", 0.0), ("B", 5.0)):
        model.add_node(name, x, 0, 0)
    model.add_section("S", b * h, h * b ** 3 / 12, b * h ** 3 / 12, 1e-4)
    model.add_member("M", "A", "B", "T", "S")
    model.def_support("A", True, True, True, True, False, False)
    model.def_support("B", False, True, True, False, False, False)
    model.add_member_dist_load("M", "Fy", -q, -q)
    model.analyze()
    theirs = abs(model.members["M"].deflection("dy", 2.5))
    ours = o.deflection(member, p, q, 0.0)
    result["deflection"] = abs(ours / theirs - 1)
    assert result["deflection"] < 5e-3, result
    return result


if __name__ == "__main__":
    if "--self-check" in sys.argv:
        print(json.dumps(self_check()))
        sys.exit(0)
    paths = [a for a in sys.argv[1:] if not a.startswith("--")]
    document = json.load(open(paths[0], encoding="utf8")) if paths else json.load(sys.stdin)
    print(json.dumps(evaluate(document)))
