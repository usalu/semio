"""🌍️ Third-party EN 1997 kernel oracle: design bearing capacity, wall sliding with Rankine earth pressure, consolidation settlement, the resistance-settlement of a pile and a simplified-Bishop slip circle, with groundhog and scipy against the in-repo second implementation. The partial factors are NEVER fixture inputs: they are read from the shared NDP registry rows (`ndp.en1997.{a1,a2,m1,m2,r1,r2,r3,de.*}`, `⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json`) of the design approach, annex and situation the case names, with the same set composition as the Rust resolver; every one of those rows is still `unverified` there, so the numbers are a basis for tests, not National Annex values. The case file `../🧪️tests/🧫️fixtures/🌍️geotechnics/🔣️.json` is an oracle case file (schema `🔮️oracles/🏃️harness/🧬️schema/🔣️.json`: SI inputs, blessed expected quantities). `python -I 🐍️.py <cases.json> --json` prints `{"results":[{"id","check","values"}]}`; `--self-check` asserts the blessed values, the pass/fail suffix of every case id and the cross-checks against the in-repo library; `--bless <cases.json>` rewrites the expected values from this oracle. Bearing: Nq from groundhog, N_gamma = 2 (Nq - 1) tan(phi) (Annex D; groundhog Vesic minus 4 tan(phi)), strip footing, vertical centred load, no depth factor. See [groundhog](https://github.com/snakesonabrain/groundhog)."""
from __future__ import annotations

import importlib.util
import json
import math
import pathlib
import sys
from unittest import mock

import numpy as np
from scipy import integrate, optimize

HERE = pathlib.Path(__file__).resolve().parent

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

COMPLIANCE = HERE.parent / "⚖️compliance"
NDP = next(parent / "⚖️compliance" / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json" for parent in HERE.parents if (parent / "⚖️compliance" / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json").is_file())
SITUATIONS = {"persistent": "p", "transient": "t", "accidental": "a"}
SETS = {"da1str": ("a1", "m1", "r1"), "da1c1": ("a1", "m1", "r1"), "da1geo": ("a2", "m2", "r1"), "da1c2": ("a2", "m2", "r1"), "da2": ("a1", "m1", "r2"), "da3": ("a1", "m2", "r3"), "geo3": ("a2", "m2", "r3")}


def groundhog():
    for _ in range(60):
        try:
            from groundhog.excavations.basic import earthpressurecoefficients_rankine, earthpressurecoefficients_poncelet
            from groundhog.shallowfoundations.capacity import nq_frictionangle_sand, ngamma_frictionangle_vesic
            from groundhog.shallowfoundations.settlement import primaryconsolidationsettlement_nc

            return earthpressurecoefficients_rankine, nq_frictionangle_sand, ngamma_frictionangle_vesic, primaryconsolidationsettlement_nc, earthpressurecoefficients_poncelet
        except ModuleNotFoundError as error:
            sys.modules[error.name] = mock.MagicMock()
            for known in [m for m in sys.modules if m.startswith("groundhog")]:
                del sys.modules[known]
    raise RuntimeError("groundhog cannot be imported")


def compliance(file: str, name: str):
    spec = importlib.util.spec_from_file_location(name, COMPLIANCE / file)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def registry_rows():
    return {row["id"]: row for row in json.loads(NDP.read_text(encoding="utf8"))["entries"]}


def factors(annex: str, approach: str, situation: str) -> dict:
    """⚖️ The partial factors of the registry rows of one annex, design approach and situation, composed like the Rust `DesignApproach::resolve`: action, material and resistance sets, DIN 1054 rows for the German DA2*, DA3 and GEO-3."""
    rows = registry_rows()
    key = approach.lower().replace("-", "").replace("_", "").replace(" ", "")
    key = {"da2s": "da2star", "da2*": "da2star", "geo2": "da2"}.get(key, key)
    k = SITUATIONS[situation.lower()]

    def value(row_id: str) -> float:
        row = rows["ndp.en1997." + row_id]
        found = row["naDe"] if annex == "de" else row["recommended"]
        assert found is not None, (annex, row_id)
        return float(found)

    if annex == "de" and key in ("da2", "da2star"):
        return dict(gamma_g=value(f"de.act.{k}.g"), gamma_phi=value("m1.phi"), gamma_c=value("m1.c"), gamma_r_v=value(f"de.res.{k}.rv"), gamma_r_h=value("de.res.rh"))
    if key in ("da3", "geo3") and annex == "de":
        soil = value(f"de.soil.{k}.f")
        gamma_g = value("a2.g") if key == "geo3" else value(f"de.act.{k}.g")
        return dict(gamma_g=gamma_g, gamma_phi=soil, gamma_c=soil, gamma_r_v=value("r3.rv"), gamma_r_h=value("r3.rh"))
    action, material, resistance = SETS["da2" if key == "da2star" else key]
    if annex == "en" and k == "a":
        action, material, resistance = "a-acc", "m1", "r1"
    return dict(gamma_g=value(f"{action}.g"), gamma_phi=value(f"{material}.phi"), gamma_c=value(f"{material}.c"), gamma_r_v=value(f"{resistance}.rv"), gamma_r_h=value(f"{resistance}.rh"))


def bearing(i):
    _, nq_of, ngamma_of, _, _ = groundhog()
    f = factors(i["annex"], i["approach"], i["situation"])
    phi = math.degrees(math.atan(math.tan(math.radians(i["phi"])) / f["gamma_phi"]))
    nq = nq_of(phi)["Nq [-]"]
    ng = ngamma_of(phi)["Ngamma [-]"] - 4 * math.tan(math.radians(phi))
    nc = (nq - 1) / math.tan(math.radians(phi))
    q = i["gamma_above"] * i["D"]
    qu = i["c"] / f["gamma_c"] * nc + q * nq + 0.5 * i["gamma_below"] * i["B"] * ng
    r_d = qu * i["B"] / f["gamma_r_v"]
    v_d = f["gamma_g"] * i["V_k"]
    return {"gamma_g": f["gamma_g"], "gamma_phi": f["gamma_phi"], "gamma_r_v": f["gamma_r_v"], "r_d": r_d, "utilization": v_d / r_d}


def wall_sliding(i):
    rankine, _, _, _, _ = groundhog()
    f = factors(i["annex"], i["approach"], i["situation"])
    phi = math.degrees(math.atan(math.tan(math.radians(i["phi"])) / f["gamma_phi"]))
    ka = rankine(phi_eff=phi, wall_angle=0.0, top_angle=0.0)["KaR [-]"]
    thrust, _ = integrate.quad(lambda z: ka * (i["gamma"] * z + i["q"]), 0.0, i["H"])
    resisting = i["W"] * math.tan(math.radians(i["delta_base"])) / f["gamma_phi"] / f["gamma_r_h"]
    return {"gamma_g": f["gamma_g"], "gamma_phi": f["gamma_phi"], "gamma_r_h": f["gamma_r_h"], "k_a": ka, "thrust_k": thrust, "utilization": f["gamma_g"] * thrust / resisting}


def stress_below_rectangle_centre(q, b, l, z):
    """📐️ Boussinesq point-load integration over a uniform rectangle under its centre: four corner quadrants of 3 q z^3 / (2 pi) * dA / (x^2 + y^2 + z^2)^(5/2)."""
    quarter, _ = integrate.dblquad(lambda y, x: (x * x + y * y + z * z) ** -2.5, 0.0, b / 2.0, 0.0, l / 2.0, epsabs=1e-13, epsrel=1e-12)
    return 4.0 * 3.0 * q * z ** 3 / (2.0 * math.pi) * quarter


def settlement(i):
    _, _, _, nc_settlement, _ = groundhog()
    z = i["z_top"] + i["thickness"] / 2
    d_sigma = stress_below_rectangle_centre(i["q"], i["B"], i["L"], z)
    s = nc_settlement(initial_height=i["thickness"], initial_voidratio=i["e0"], initial_effective_stress=i["sigma_v0"], effective_stress_increase=d_sigma, compression_index=i["C_c"])["delta z [m]"]
    return {"delta_sigma": d_sigma, "s": s, "utilization": s / i["limit"]}


def bishop_factor(i, f, slices=400):
    h, beta = i["H"], math.radians(i["beta"])
    run = h / math.tan(beta)
    xc, yc, r = i["xc"], i["yc"], i["r"]
    tan_phi, c = math.tan(math.radians(i["phi"])) / f["gamma_phi"], i["c"] / f["gamma_c"]
    gamma = i["gamma"]
    ground = lambda x: np.where(x <= 0, h, np.where(x >= run, 0.0, h - x * math.tan(beta)))
    xs = np.linspace(xc - r, xc + r, 20001)
    base = yc - np.sqrt(np.maximum(r * r - (xs - xc) ** 2, 0.0))
    inside = xs[(np.abs(xs - xc) < r) & (base < ground(xs))]
    if inside.size == 0:
        return None
    xa, xb = inside.min(), inside.max()
    dx = (xb - xa) / slices
    x = xa + (np.arange(slices) + 0.5) * dx
    yb = yc - np.sqrt(np.maximum(r * r - (x - xc) ** 2, 0.0))
    height = np.maximum(ground(x) - yb, 0.0)
    w = gamma * height * dx
    alpha = np.arcsin(np.clip((xc - x) / r, -1.0, 1.0))
    drive = float(np.sum(w * np.sin(alpha)))
    if drive <= 1e-9:
        return None
    resist = lambda ff: float(np.sum((c * dx + w * tan_phi) / (np.cos(alpha) + np.sin(alpha) * tan_phi / ff)))
    residual = lambda ff: ff * drive - resist(ff)
    low = max(0.2, float(np.max(-np.tan(alpha))) * tan_phi * 1.0001 + 1e-9)
    if residual(low) * residual(50.0) > 0.0:
        return None
    return optimize.brentq(residual, low, 50.0, xtol=1e-12)


def slope(i):
    f = factors(i["annex"], i["approach"], i["situation"])
    factor = bishop_factor(i, f)
    return {"gamma_phi": f["gamma_phi"], "gamma_c": f["gamma_c"], "factor": factor, "utilization": 1.0 / factor}


def din4017(i):
    """🏛️ DIN 4017 bearing resistance of a rectangular footing under an inclined, eccentric load: N_d0 = Prandtl N_q (groundhog), N_b0 = (N_d0 - 1) tan(phi), shape and inclination factors of the standard written independently of the Rust kernel."""
    _, nq_of, _, _, _ = groundhog()
    f = factors(i["annex"], i["approach"], i["situation"])
    phi = math.atan(math.tan(math.radians(i["phi"])) / f["gamma_phi"])
    v = f["gamma_g"] * i["V_k"]
    hb = f["gamma_g"] * i["H_k"]
    e = f["gamma_g"] * i["M_k"] / v
    b, l = i["B"] - 2.0 * e, i["L"]
    nd = nq_of(math.degrees(phi))["Nq [-]"]
    nb, ratio = (nd - 1.0) * math.tan(phi), b / l
    sd, sb = 1.0 + ratio * math.sin(phi), 1.0 - 0.3 * ratio
    sc = (sd * nd - 1.0) / (nd - 1.0)
    tan_delta = hb / v
    m = (2.0 + ratio) / (1.0 + ratio)
    id_, ib = (1.0 - 0.7 * tan_delta) ** (m + 1.0), (1.0 - tan_delta) ** (m + 1.0)
    ic = (id_ * nd - 1.0) / (nd - 1.0)
    c = i["c"] / f["gamma_c"]
    per_area = c * ((nd - 1.0) / math.tan(phi)) * sc * ic + i["gamma_above"] * i["D"] * nd * sd * id_ + i["gamma_below"] * b * nb * sb * ib
    r_d = per_area * b * l / f["gamma_r_v"]
    return {"gamma_g": f["gamma_g"], "gamma_phi": f["gamma_phi"], "gamma_r_v": f["gamma_r_v"], "r_d": r_d, "utilization": v / r_d}


def spiral_kp(phi_deg, delta_deg):
    """🧱️ Passive coefficient of a logarithmic-spiral failure surface behind a vertical wall with level ground: minimum of the moment-equilibrium coefficient over the pole (xo, yo) with scipy Nelder-Mead from a coarse multi-start, wedge area and moment by dense polygon; the wall friction is limited to 2/3 phi."""
    delta = math.radians(min(abs(delta_deg), abs(phi_deg) * 2.0 / 3.0))
    tan_phi = math.tan(math.radians(phi_deg))
    if delta < 1e-12:
        return (1.0 + math.sin(math.radians(phi_deg))) / (1.0 - math.sin(math.radians(phi_deg)))

    def wedge(xo, yo):
        r_b, th0 = math.hypot(xo, yo), math.atan2(-yo, -xo)
        track = lambda th: (xo + r_b * np.exp(th * tan_phi) * np.cos(th0 + th), yo + r_b * np.exp(th * tan_phi) * np.sin(th0 + th))
        grid = np.linspace(0.0, 6.0, 1201)
        xs, ys = track(grid)
        hit = np.nonzero(ys >= 1.0)[0]
        if hit.size == 0 or hit[0] == 0 or np.any(xs[: hit[0] + 1] < -1e-9):
            return None
        te = optimize.brentq(lambda t: float(track(t)[1]) - 1.0, grid[hit[0] - 1], grid[hit[0]], xtol=1e-14)
        px, py = track(te * np.arange(1, 2001) / 2000.0)
        py[-1] = 1.0
        x = np.concatenate(([0.0, 0.0], px, [0.0]))
        y = np.concatenate(([1.0, 0.0], py, [1.0]))
        x1, x2, y1, y2 = x[:-1] - xo, x[1:] - xo, y[:-1], y[1:]
        return float(np.sum((x1 * x1 + x1 * x2 + x2 * x2) / 6.0 * (y2 - y1)))

    def objective(p):
        xo, yo = p
        if yo < 0.05:
            return 1e6
        moment = wedge(xo, yo)
        den = yo * math.cos(delta) + xo * math.sin(delta) - math.cos(delta) / 3.0
        return 1e6 if moment is None or den <= 1e-9 else 2.0 * moment / den

    starts = [(-3.0, 1.5), (-1.0, 2.5), (1.0, 1.0)]
    results = [optimize.minimize(objective, start, method="Nelder-Mead", options={"xatol": 1e-9, "fatol": 1e-12, "maxiter": 600}) for start in starts]
    return min(r.fun for r in results)


def passive_curved(i):
    rankine, _, _, _, poncelet = groundhog()
    kp_rankine = rankine(phi_eff=i["phi"], wall_angle=0.0, top_angle=0.0)["KpR [-]"]
    values = {"kp_rankine": float(kp_rankine), "kp": float(spiral_kp(i["phi"], i["delta"]))}
    if i["delta"] >= 15.0:
        values["kp_coulomb"] = float(poncelet(phi_eff=i["phi"], interface_friction_angle=i["delta"], wall_angle=0.0, top_angle=0.0)["KpC [-]"])
    return values


def secondary_settlement(i):
    return {"s_sec": float(i["c_alpha"] * i["h"] / (1.0 + i["e0"]) * np.log10(i["t2"] / i["t1"]))}


def negative_friction(i):
    f = factors(i["annex"], i["approach"], i["situation"])
    drag = math.pi * i["d"] * i["beta"] * i["sv"] * i["h"]
    values = {"gamma_g": f["gamma_g"], "f_neg_k": drag, "f_neg_d": f["gamma_g"] * drag}
    if "f_cd" in i:
        values["downdrag"] = (i["f_cd"] + values["f_neg_d"]) / i["r_c_d"]
    return values


def reinforced_soil(i):
    t_max = i["k"] * i["sigma_v"] * i["s_v"]
    return {"t_max": t_max, "l_e": t_max / (2.0 * i["b"] * i["f_star"] * i["sigma_v"])}


def single_well(i):
    drawdown = i["h_aquifer"] - i["h_well"]
    radius = 3000.0 * drawdown * math.sqrt(i["k"])
    if i["aquifer"] == "confined":
        q = 2.0 * math.pi * i["k"] * i["thickness"] * drawdown / math.log(radius / i["r_well"])
    else:
        q = math.pi * i["k"] * (i["h_aquifer"] ** 2 - i["h_well"] ** 2) / math.log(radius / i["r_well"])
    return {"radius": radius, "q": q}


def correlation_factors_dynamic(n):
    """🎯️ xi5 and xi6 of EN 1997-1 Table A.11 for `n` dynamic tests from groundhog's `select_correlation_factors` (the third-party table, not the registry rows the kernel reads)."""
    for _ in range(60):
        try:
            from groundhog.standards.eurocode7.factors import Eurocode7_factoring_STR_GEO

            picked = Eurocode7_factoring_STR_GEO().select_correlation_factors("Dynamic load test", n)
            return float(picked["ksi_5"]), float(picked["ksi_6"])
        except ModuleNotFoundError as error:
            sys.modules[error.name] = mock.MagicMock()
            for known in [m for m in sys.modules if m.startswith("groundhog")]:
                del sys.modules[known]
    raise RuntimeError("groundhog cannot be imported")


def pile_dynamic(i):
    """🪵️ Dynamic-test pile resistance, EN 1997-1 7.6.2.4 (7.10): R_c,k = min{R_c,m,mean / xi5; R_c,m,min / xi6}, R_c,d = R_c,k / gamma_t with gamma_t of the registry row of the resistance set; the Hiley-type driving formula R = eta W h / (s + 0.5 s_e) beside it; xi5, xi6 by groundhog (the series mean and lowest value are the inputs)."""
    xi5, xi6 = correlation_factors_dynamic(int(i["n"]))
    key = i["approach"].lower().replace("-", "").replace("_", "")
    resistance = SETS[key][2]
    gamma_t = registry_rows()["ndp.en1997." + resistance + ".t"]["recommended"]
    r_k = min(i["rm_mean"] / xi5, i["rm_min"] / xi6)
    r_d = r_k / gamma_t
    values = {"xi5": xi5, "xi6": xi6, "gamma_t": gamma_t, "r_c_k": r_k, "r_c_d": r_d, "r_driving": i["eta"] * i["W"] * i["h"] / (i["s"] + 0.5 * i["s_e"])}
    if "f_cd" in i:
        values["compression"] = i["f_cd"] / r_d
    return values


def pile_settlement(i):
    """🪵️ Resistance-settlement of one pile at its serviceability load: shaft and base resistance mobilise linearly up to the reference settlements s_ref_s and s_ref_b (brentq on the total mobilised resistance), plus the elastic shortening (N - R_s,mob / 2) L / (E_c A) with E_c from the registry row `ndp.en1997.7.6.4.concrete-modulus`."""
    ea = i["E_c"] * math.pi * i["D"] ** 2 / 4.0
    total = lambda s: i["rs_max"] * min(s / i["s_ref_s"], 1.0) + i["rb_max"] * min(s / i["s_ref_b"], 1.0)
    s_mob = optimize.brentq(lambda s: total(s) - i["N"], 0.0, max(i["s_ref_s"], i["s_ref_b"]), xtol=1e-14)
    rs_mob = i["rs_max"] * min(s_mob / i["s_ref_s"], 1.0)
    s_el = (i["N"] - 0.5 * rs_mob) * i["L"] / ea
    return {"s_mobilised": s_mob, "s_elastic": s_el, "s_total": s_mob + s_el, "utilization": (s_mob + s_el) / i["s_lim"]}


KINDS = {"en1997.G.secondary-settlement": secondary_settlement, "en1997.G.negative-skin-friction": negative_friction, "en1997.G.reinforced-soil": reinforced_soil, "en1997.G.single-well": single_well, "en1997.G.bearing-din4017": din4017, "en1997.G.passive-curved": passive_curved, "en1997.G.pile-resistance-settlement": pile_settlement, "en1997.G.pile-dynamic-tests": pile_dynamic, "en1997.G.bearing-design": bearing, "en1997.G.wall-sliding-design": wall_sliding, "en1997.G.settlement-primary": settlement, "en1997.G.slope-bishop": slope}
UNITS = {"gamma_g": "1", "gamma_phi": "1", "gamma_c": "1", "gamma_r_v": "1", "gamma_r_h": "1", "r_d": "N/m", "k_a": "1", "thrust_k": "N/m", "delta_sigma": "Pa", "s": "m", "s_mobilised": "m", "s_elastic": "m", "s_total": "m", "factor": "1", "utilization": "1", "s_sec": "m", "f_neg_k": "N", "f_neg_d": "N", "t_max": "N", "l_e": "m", "radius": "m", "q": "m3/s", "kp": "1", "kp_rankine": "1", "kp_coulomb": "1", "downdrag": "1", "xi5": "1", "xi6": "1", "gamma_t": "1", "r_c_k": "N", "r_c_d": "N", "r_driving": "N", "compression": "1"}
TABLE_EXACT = {"gamma_g", "gamma_phi", "gamma_c", "gamma_r_v", "gamma_r_h", "xi5", "xi6", "gamma_t"}


def profile(check, name):
    if name in TABLE_EXACT:
        return {"profile": "table-exact-v1"}
    if check == "en1997.G.passive-curved" and name == "kp":
        return {"profile": "numerical-v1", "tolerance": 5e-3}
    if check == "en1997.G.slope-bishop":
        return {"profile": "numerical-v1", "tolerance": 1e-2}
    return {"profile": "numerical-v1"}


def evaluate(document):
    return [{"id": case["id"], "check": case["check"], "values": KINDS[case["check"]](case["inputs"])} for case in document["cases"]]


def bless(document):
    for case, row in zip(document["cases"], evaluate(document)):
        case["expected"] = {name: {"value": value, "unit": UNITS[name], **profile(case["check"], name)} for name, value in row["values"].items()}
    return document


def self_check(document):
    rows = evaluate(document)
    for case, row in zip(document["cases"], rows):
        for name, expectation in case["expected"].items():
            assert abs(row["values"][name] - expectation["value"]) <= 1e-9 * max(1.0, abs(expectation["value"])), (case["id"], name, row["values"][name], expectation["value"])
        if "utilization" in row["values"]:
            wanted = "pass" if case["id"].endswith("-pass") else "fail"
            assert LIMIT.limit_status(row["values"]["utilization"]) == wanted, (case["id"], row["values"]["utilization"])
    assert {"pass", "fail"} <= {"pass" if r["id"].endswith("-pass") else "fail" for r in rows if r["check"] == "en1997.G.slope-bishop"}
    assert {case["inputs"]["annex"] for case in document["cases"] if "annex" in case["inputs"]} == {"en", "de"}
    geo = compliance("📚️geo.py", "en1997_geo")
    lib = compliance("🐍️.py", "en1997_compliance")
    for case, row in zip(document["cases"], rows):
        i, v = case["inputs"], row["values"]
        if case["check"] == "en1997.G.bearing-design":
            f = factors(i["annex"], i["approach"], i["situation"])
            phi = math.degrees(math.atan(math.tan(math.radians(i["phi"])) / f["gamma_phi"]))
            kernel = geo.bearing_drained(i["B"], 1.0e4, i["D"], 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, phi, i["c"] / f["gamma_c"], i["gamma_below"], i["gamma_above"] * i["D"])
            expected_r = kernel["r_over_a"] * i["B"] / f["gamma_r_v"]
            assert abs(v["r_d"] / expected_r - 1) < 2e-3, (case["id"], v["r_d"], expected_r)
        if case["check"] == "en1997.G.wall-sliding-design":
            phi = math.degrees(math.atan(math.tan(math.radians(i["phi"])) / v["gamma_phi"]))
            assert abs(v["k_a"] / geo.ka_rankine(phi) - 1) < 1e-9
            exact = 0.5 * v["k_a"] * i["gamma"] * i["H"] ** 2 + v["k_a"] * i["q"] * i["H"]
            assert abs(v["thrust_k"] / exact - 1) < 1e-9
        if case["check"] == "en1997.G.settlement-primary":
            z = i["z_top"] + i["thickness"] / 2
            assert abs(v["delta_sigma"] / geo.sigma_z_rect_centre(i["q"], i["B"], i["L"], z) - 1) < 1e-4, case["id"]
        if case["check"] == "en1997.G.bearing-din4017":
            strip = din4017({**i, "L": 1.0e6, "H_k": 0.0, "M_k": 0.0})
            assert abs(strip["r_d"] / (bearing(i)["r_d"] * 1.0e6) - 1) < 1e-4, case["id"]
        if case["check"] == "en1997.G.passive-curved":
            assert v["kp_rankine"] - 1e-9 <= v["kp"] <= v.get("kp_coulomb", float("inf")) + 1e-9, (case["id"], v)
            if i["delta"] == 0.0:
                assert abs(v["kp"] / v["kp_rankine"] - 1) < 1e-12 and abs(v["kp_rankine"] / geo.kp_rankine(i["phi"]) - 1) < 1e-9
        if case["check"] == "en1997.G.slope-bishop":
            layers = [{"depthTop": 0.0, "depthBottom": 1e3, "gamma": i["gamma"], "gammaPrime": i["gamma"] - 9810.0, "phiPrimeDeg": i["phi"], "cohesionEffective": i["c"]}]
            theirs = lib.bishop_fos_circle(layers, i["H"], i["beta"], 14.0, 1e6, v["gamma_phi"], v["gamma_c"], i["xc"], i["yc"], i["r"])
            assert theirs is not None and abs(v["factor"] / theirs - 1) < 0.02, (case["id"], v["factor"], theirs)
            if i["phi"] == 0.0:
                assert abs(v["factor"] / fellenius_undrained(i, v) - 1) < 2e-3, (case["id"], v["factor"])
    return {r["id"]: round(r["values"].get("utilization", r["values"].get("kp", 0.0)), 6) for r in rows}


def fellenius_undrained(i, v):
    """🎯️ phi = 0: F = c R^2 theta / sum(W x_arm), the moment ratio of the arc (equal to Bishop for phi = 0)."""
    h, beta = i["H"], math.radians(i["beta"])
    run = h / math.tan(beta)
    xc, yc, r = i["xc"], i["yc"], i["r"]
    ground = lambda x: h if x <= 0 else 0.0 if x >= run else h - x * math.tan(beta)
    cut = [x for x in np.linspace(xc - r, xc + r, 40001) if yc - math.sqrt(max(r * r - (x - xc) ** 2, 0.0)) < ground(x) and abs(x - xc) < r]
    xa, xb = cut[0], cut[-1]
    arc = r * (math.asin((xc - xa) / r) - math.asin((xc - xb) / r))
    moment, _ = integrate.quad(lambda x: i["gamma"] * (ground(x) - (yc - math.sqrt(max(r * r - (x - xc) ** 2, 0.0)))) * (xc - x), xa, xb, limit=400, points=[0.0, run])
    return i["c"] / v["gamma_c"] * r * arc / moment


if __name__ == "__main__":
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    path = pathlib.Path(args[0]) if args else None
    document = json.loads(path.read_text(encoding="utf8") if path else sys.stdin.read())
    if "--bless" in sys.argv:
        path.write_text(json.dumps(bless(document), ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    elif "--self-check" in sys.argv:
        print(json.dumps(self_check(document)))
    else:
        print(json.dumps({"results": evaluate(document)}, ensure_ascii=False))
