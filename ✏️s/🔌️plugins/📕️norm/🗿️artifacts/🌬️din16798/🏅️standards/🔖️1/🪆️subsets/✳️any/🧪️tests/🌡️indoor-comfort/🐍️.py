"""Independent oracle of the DIN EN 16798-1 comfort and ventilation closed forms. `--write` regenerates the committed vectors; without arguments it recomputes every vector and fails when the committed numbers differ. The PMV is solved twice (ISO 7730 Annex D fixed point and a bisection of the clothing heat balance) and both must agree."""
import json
import math
import pathlib
import sys

VECTORS = pathlib.Path(__file__).resolve().parent.parent / "🧫️fixtures" / "🌡️comfort-vectors" / "🔣️.json"

HALF_BAND = {1: 2.0, 2: 3.0, 3: 4.0, 4: 5.0}
PER_PERSON = {1: 10.0, 2: 7.0, 3: 4.0, 4: 2.5}
AREA_RATE = {"very_low": (0.5, 0.35, 0.2, 0.15), "low": (1.0, 0.7, 0.4, 0.3), "non_low": (2.0, 1.4, 0.8, 0.6)}
CO2_ABOVE = {1: 550.0, 2: 800.0, 3: 1350.0, 4: 1350.0}
ABSOLUTE = {"de": {"residential": 1200.0, "classroom": 800.0, "other": 900.0}, "en": {"residential": 1500.0, "classroom": 1000.0, "other": 1000.0}}


def pmv_fixed_point(ta, vel, rh, met, clo):
    pa = rh * 10 * math.exp(16.6536 - 4030.183 / (ta + 235))
    icl, m = 0.155 * clo, met * 58.15
    fcl = 1 + 1.29 * icl if icl <= 0.078 else 1.05 + 0.645 * icl
    hcf, taa = 12.1 * math.sqrt(vel), ta + 273
    p1 = icl * fcl
    p2, p3, p4 = p1 * 3.96, p1 * 100, p1 * taa
    p5 = 308.7 - 0.028 * m + p2 * (taa / 100) ** 4
    xn = (taa + (35.5 - ta) / (3.5 * icl + 0.1)) / 100
    xf = xn * 2
    hc = hcf
    for _ in range(150):
        if abs(xn - xf) <= 0.00015:
            break
        xf = (xf + xn) / 2
        hc = max(hcf, 2.38 * abs(100 * xf - taa) ** 0.25)
        xn = (p5 + p4 * hc - p2 * xf**4) / (100 + p3 * hc)
    tcl = 100 * xn - 273
    losses = 3.05e-3 * (5733 - 6.99 * m - pa) + (0.42 * (m - 58.15) if m > 58.15 else 0) + 1.7e-5 * m * (5867 - pa) + 0.0014 * m * (34 - ta) + 3.96 * fcl * (xn**4 - (taa / 100) ** 4) + fcl * hc * (tcl - ta)
    return (0.303 * math.exp(-0.036 * m) + 0.028) * (m - losses)


def pmv_bisection(ta, vel, rh, met, clo):
    pa = rh * 10 * math.exp(16.6536 - 4030.183 / (ta + 235))
    icl, m = 0.155 * clo, met * 58.15
    fcl = 1 + 1.29 * icl if icl <= 0.078 else 1.05 + 0.645 * icl
    hcf = 12.1 * math.sqrt(vel)

    def residual(tcl):
        hc = max(hcf, 2.38 * abs(tcl - ta) ** 0.25)
        rad = 3.96e-8 * fcl * ((tcl + 273) ** 4 - (ta + 273) ** 4)
        return tcl - (35.7 - 0.028 * m - icl * (rad + fcl * hc * (tcl - ta)))

    low, high = ta - 20.0, ta + 40.0
    for _ in range(200):
        mid = (low + high) / 2
        if residual(low) * residual(mid) <= 0:
            high = mid
        else:
            low = mid
    tcl = (low + high) / 2
    hc = max(hcf, 2.38 * abs(tcl - ta) ** 0.25)
    losses = 3.05e-3 * (5733 - 6.99 * m - pa) + (0.42 * (m - 58.15) if m > 58.15 else 0) + 1.7e-5 * m * (5867 - pa) + 0.0014 * m * (34 - ta) + 3.96e-8 * fcl * ((tcl + 273) ** 4 - (ta + 273) ** 4) + fcl * hc * (tcl - ta)
    return (0.303 * math.exp(-0.036 * m) + 0.028) * (m - losses)


def ppd(pmv):
    return 100 - 95 * math.exp(-0.03353 * pmv**4 - 0.2179 * pmv**2)


def vectors():
    pmv_rows = []
    for ta, rh, vel, met, clo in [(24.5, 45, 0.1, 1.2, 0.5), (22.0, 45, 0.1, 1.2, 1.0), (26.0, 60, 0.1, 1.2, 0.5), (28.5, 75, 0.1, 1.2, 0.5), (20.0, 30, 0.2, 1.6, 0.7), (23.0, 50, 0.05, 1.0, 0.0), (22.0, 75, 0.1, 1.2, 1.0)]:
        a, b = pmv_fixed_point(ta, vel, rh, met, clo), pmv_bisection(ta, vel, rh, met, clo)
        if abs(a - b) > 5e-3:
            raise SystemExit(f"the two PMV solutions differ at {(ta, rh, vel, met, clo)}: {a} vs {b}")
        pmv_rows.append({"t_op": ta, "rh": rh, "v": vel, "met": met, "clo": clo, "pmv": a, "ppd": ppd(a)})
    adaptive = [{"theta_rm": t, "category": c, "lower": 0.33 * t + 18.8 - HALF_BAND[c], "centre": 0.33 * t + 18.8, "upper": 0.33 * t + 18.8 + HALF_BAND[c]} for t in (10.0, 15.0, 22.5, 30.0) for c in (1, 2, 3, 4)]
    ventilation = []
    for occupants, area, pollution in [(20, 200.0, "low"), (3, 80.0, "very_low"), (45, 120.0, "non_low")]:
        for c in (1, 2, 3, 4):
            ventilation.append({"occupants": occupants, "area": area, "category": c, "pollution": pollution, "persons": occupants * PER_PERSON[c] * 3.6, "building": area * AREA_RATE[pollution][c - 1] * 3.6})
    co2 = []
    for annex in ("de", "en"):
        for usage in ("office", "residential", "classroom"):
            for c in (1, 2, 3, 4):
                above = CO2_ABOVE[c]
                if annex == "de":
                    above = min(ABSOLUTE["de"][usage if usage != "office" else "other"] - 400.0, above)
                co2.append({"annex": annex, "usage": usage, "category": c, "outdoor": 400.0, "limit_above": above})
    table_d1 = [{"t_op": 22.0, "rh": 60, "v": 0.1, "met": 1.2, "clo": 0.5, "pmv": -0.75}, {"t_op": 27.0, "rh": 60, "v": 0.1, "met": 1.2, "clo": 0.5, "pmv": 0.77}, {"t_op": 19.0, "rh": 40, "v": 0.1, "met": 1.2, "clo": 1.0, "pmv": -0.60}, {"t_op": 23.5, "rh": 60, "v": 0.1, "met": 1.2, "clo": 0.5, "pmv": -0.30}]
    for row in table_d1:
        if abs(pmv_fixed_point(row["t_op"], row["v"], row["rh"], row["met"], row["clo"]) - row["pmv"]) > 0.011:
            raise SystemExit(f"the PMV disagrees with the published ISO 7730 table at {row}")
    return {"iso7730_table_d1": table_d1, "pmv": pmv_rows, "adaptive": adaptive, "ventilation": ventilation, "co2": co2}


def main():
    computed = vectors()
    if "--write" in sys.argv:
        VECTORS.parent.mkdir(parents=True, exist_ok=True)
        VECTORS.write_text(json.dumps(computed, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
        return 0
    committed = json.loads(VECTORS.read_text(encoding="utf8"))
    ok = json.dumps(committed, sort_keys=True) == json.dumps(json.loads(json.dumps(computed)), sort_keys=True)
    print(json.dumps({"ok": ok}))
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
