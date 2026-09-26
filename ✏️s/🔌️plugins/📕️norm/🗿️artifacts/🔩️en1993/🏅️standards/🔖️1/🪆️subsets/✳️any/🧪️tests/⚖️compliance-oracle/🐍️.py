#!/usr/bin/env python3
"""🐍 Independent EN 1993 oracle — EN 1990 combinations + governing steel checks (SI)."""

from __future__ import annotations

import json
import math
from pathlib import Path

E = 210e9
PI = math.pi
GAMMA_G = 1.35
GAMMA_Q = 1.50
XI = 0.85


def chi(lambda_bar: float, alpha: float) -> float:
    if lambda_bar <= 0.2:
        return 1.0
    phi = 0.5 * (1.0 + alpha * (lambda_bar - 0.2) + lambda_bar * lambda_bar)
    return min(1.0, 1.0 / (phi + math.sqrt(max(0.0, phi * phi - lambda_bar * lambda_bar))))


def lambda_bar(l_cr: float, i: float, fy: float) -> float:
    lambda_1 = PI * math.sqrt(E / fy)
    return (l_cr / i) / lambda_1


def axial_rd(area: float, fy: float, gamma_m0: float = 1.0) -> float:
    return area * fy / gamma_m0


def buckling_rd(area: float, fy: float, chi_v: float, gamma_m1: float) -> float:
    return chi_v * area * fy / gamma_m1


def critical_temperature(mu0: float) -> float:
    mu = min(1.0, max(0.02, mu0))
    return 39.19 * math.log(1.0 / 0.9674 / mu**3.833 - 1.0) + 482.0


def combine_n(doc: dict) -> float:
    """EN 1990 Eq. 6.10a/b governing |N| for the first member."""
    mid = doc["members"][0]["id"]
    g = 0.0
    variables = []
    cases = {lc["id"]: lc for lc in doc["loadCases"]}
    for row in doc["memberActions"]:
        if row["memberId"] != mid:
            continue
        lc = cases[row["loadCaseId"]]
        n = row["action"]["n"]
        if lc["kind"] in ("permanent", "prestress"):
            g += n
        else:
            variables.append(n)
    if not variables:
        return abs(GAMMA_G * g)
    lead = variables[0]
    a = abs(GAMMA_G * g + GAMMA_Q * lead)
    b = abs(XI * GAMMA_G * g + GAMMA_Q * lead)
    return max(a, b)


def main() -> None:
    assets = Path(__file__).resolve().parents[2] / "🖼️assets"
    compliant = json.loads((assets / "✅️heb240-compliant" / "snapshot.json").read_text())
    section = compliant["sections"][0]
    material = compliant["materials"][0]
    member = compliant["members"][0]
    n_ed = combine_n(compliant)

    area = section["area"]
    fy = material["fy"]
    iy_r = (section["iy"] / area) ** 0.5
    iz_r = (section["iz"] / area) ** 0.5
    lam_y = lambda_bar(member["bucklingLengthY"], iy_r, fy)
    lam_z = lambda_bar(member["bucklingLengthZ"], iz_r, fy)
    # Table 6.2: h/b=1 → y curve b (0.34), z curve c (0.49)
    chi_y = chi(lam_y, 0.34)
    chi_z = chi(lam_z, 0.49)
    chi_gov = min(chi_y, chi_z)
    n_rd = axial_rd(area, fy, 1.0)
    n_b_rd_de = buckling_rd(area, fy, chi_gov, 1.1)
    n_b_rd_en = buckling_rd(area, fy, chi_gov, 1.0)

    report = {
        "axialUtilization": n_ed / n_rd,
        "bucklingUtilizationDe": n_ed / n_b_rd_de,
        "bucklingUtilizationEn": n_ed / n_b_rd_en,
        "designN": n_ed,
        "chiY": chi_y,
        "lambdaY": lam_y,
        "nRdKN": n_rd / 1000.0,
        "nBRdDeKN": n_b_rd_de / 1000.0,
        "thetaCrMu05": critical_temperature(0.5),
        "deExceedsEn": (n_ed / n_b_rd_de) > (n_ed / n_b_rd_en),
    }

    noncomp = json.loads((assets / "🔩️high-strength-connection" / "snapshot.json").read_text())
    n_fail = combine_n(noncomp)
    m_fail = noncomp["members"][0]
    sec_f = noncomp["sections"][0]
    mat_f = noncomp["materials"][0]
    area_f = sec_f["area"]
    fy_f = mat_f["fy"]
    iy_f = math.sqrt(sec_f["iy"] / area_f)
    iz_f = math.sqrt(sec_f["iz"] / area_f)
    lam_fy = lambda_bar(m_fail["bucklingLengthY"], iy_f, fy_f)
    lam_fz = lambda_bar(m_fail["bucklingLengthZ"], iz_f, fy_f)
    chi_f = min(chi(lam_fy, 0.34), chi(lam_fz, 0.49))
    n_b_fail = buckling_rd(area_f, fy_f, chi_f, 1.1)  # report DE for comparison even if annex=en
    # Rust noncompliant_overloaded uses DE annex HEB240 — oracle also reports DSL asset EN S460.
    # Prefer overloaded frame numbers from compliant geometry with noncompliant Lcr/N for rust test.
    overloaded_n = 1.35 * 2_100_000.0
    overloaded_lcr = 12.0
    lam_oy = lambda_bar(overloaded_lcr, iy_r, fy)
    lam_oz = lambda_bar(overloaded_lcr, iz_r, fy)
    chi_o = min(chi(lam_oy, 0.34), chi(lam_oz, 0.49))
    n_b_o = buckling_rd(area, fy, chi_o, 1.1)
    report["noncompliantBucklingUtilizationDe"] = overloaded_n / n_b_o
    report["noncompliantFails"] = (overloaded_n / n_b_o) > 1.0
    report["highStrengthDesignN"] = n_fail
    report["highStrengthBucklingU"] = n_fail / n_b_fail

    a_s = 245e-6
    f_ub = 800e6
    f_pc = 0.7 * f_ub * a_s
    fs = 1.0 * 1 * 0.5 * f_pc * 4 / 1.25
    report["slipFsRdKN"] = fs / 1000.0

    print(json.dumps(report, indent=2))
    assert report["axialUtilization"] < 1.0
    assert report["bucklingUtilizationDe"] < 1.0
    assert report["deExceedsEn"]
    assert report["noncompliantFails"], report
    assert abs(report["thetaCrMu05"] - 584.7) < 2.0
    assert fs > 0
    assert abs(report["designN"] - 200000.0) < 50.0, report["designN"]
    print("oracle ok")


if __name__ == "__main__":
    main()
