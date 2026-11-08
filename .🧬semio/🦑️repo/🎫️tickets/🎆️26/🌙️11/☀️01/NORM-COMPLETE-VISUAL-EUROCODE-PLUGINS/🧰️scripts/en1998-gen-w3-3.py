import json, math, sys
import numpy as np
from scipy.linalg import eigh

OUT = sys.argv[1]
G = 9.80665


def sd(ag, S, TB, TC, TD, q, T, beta=0.2):
    if T <= TB:
        return ag * S * (2 / 3 + T / TB * (2.5 / q - 2 / 3))
    if T <= TC:
        return ag * S * 2.5 / q
    if T <= TD:
        return max(ag * S * 2.5 / q * TC / T, beta * ag)
    return max(ag * S * 2.5 / q * TC * TD / T**2, beta * ag)


def se(ag, S, TB, TC, TD, T):
    return sd(ag, S, TB, TC, TD, 1.0, T, 0.0) if False else (
        ag * S * (1 + T / TB * 1.5) if T <= TB else ag * S * 2.5 if T <= TC else ag * S * 2.5 * TC / T if T <= TD else ag * S * 2.5 * TC * TD / T**2)


def modal(m, k, spec, xi):
    n = len(m)
    K = np.zeros((n, n))
    for i in range(n):
        K[i, i] += k[i]
        if i + 1 < n:
            K[i, i] += k[i + 1]
            K[i, i + 1] -= k[i + 1]
            K[i + 1, i] -= k[i + 1]
    M = np.diag(m)
    w2, phi = eigh(K, M)
    w = np.sqrt(w2)
    T = 2 * math.pi / w
    meff = [(phi[:, j] @ M @ np.ones(n)) ** 2 / (phi[:, j] @ M @ phi[:, j]) for j in range(n)]
    F = [meff[j] * sd(*spec, T[j]) for j in range(n)]
    srss = math.sqrt(sum(f * f for f in F))
    cqc = 0.0
    for i in range(n):
        for j in range(n):
            r = w[j] / w[i]
            rho = 8 * xi**2 * (1 + r) * r**1.5 / ((1 - r * r) ** 2 + 4 * xi**2 * r * (1 + r) ** 2)
            cqc += rho * F[i] * F[j]
    return F, srss, math.sqrt(cqc)


def q(v, unit, profile):
    return {"value": float(v), "unit": unit, "profile": profile}


cases = []
ref_scipy = [{"oracle": "norm-eurocode-scipy-numerics", "adapter": "numerical", "kind": "independent-formulation"}]
for name, m, k, spec, xi in [
    ("three-storey-type1-b", [100000.0, 100000.0, 80000.0], [60e6, 50e6, 40e6], (1.5, 1.2, 0.15, 0.5, 2.0, 3.0), 0.05),
    ("four-storey-soft-ground", [120000.0, 110000.0, 110000.0, 90000.0], [90e6, 80e6, 70e6, 55e6], (2.4, 1.35, 0.2, 0.6, 2.0, 2.0), 0.05),
    ("two-storey-low-q", [200000.0, 150000.0], [120e6, 80e6], (1.0, 1.0, 0.15, 0.4, 2.0, 1.5), 0.03),
]:
    F, srss, cqc = modal(np.array(m), np.array(k), spec, xi)
    exp = {f"F_b_{j + 1}": q(F[j] , "N", "numerical-v1") for j in range(len(m))}
    exp["V_srss"] = q(srss, "N", "numerical-v1")
    exp["V_cqc"] = q(cqc, "N", "numerical-v1")
    a_g, S, TB, TC, TD, qf = spec
    cases.append({"id": f"en1998.4.3.3.3.w3-3-modal-base-shear-{name}", "check": "en1998.4.3.3.3.modal-base-shear",
                  "citation": {"standard": "EN 1998-1", "edition": "2004", "clause": "4.3.3.3 modal response spectrum analysis, F_b,j = M_eff,j S_d(T_j), 4.3.3.3.2 SRSS and CQC combination"},
                  "inputs": {"m": m, "k": k, "a_g": a_g, "S": S, "T_B": TB, "T_C": TC, "T_D": TD, "q": qf, "xi": xi},
                  "expected": exp, "references": ref_scipy})
for name, spec, T in [
    ("q-1-5-rising", (1.5, 1.15, 0.15, 0.5, 2.0, 1.5), 0.08),
    ("q-4-0-plateau", (2.0, 1.0, 0.2, 0.6, 2.0, 4.0), 0.4),
    ("q-1-0-constant-velocity", (1.2, 1.4, 0.2, 0.8, 2.0, 1.0), 1.5),
    ("q-3-0-constant-displacement", (1.0, 1.2, 0.15, 0.5, 2.0, 3.0), 3.2),
    ("q-5-0-lower-bound-floor", (1.0, 1.2, 0.15, 0.5, 2.0, 5.0), 3.8),
]:
    a_g, S, TB, TC, TD, qf = spec
    cases.append({"id": f"en1998.3.2.2.5.w3-3-design-spectrum-{name}", "check": "en1998.3.2.2.5.design-spectrum",
                  "citation": {"standard": "EN 1998-1", "edition": "2004", "clause": "3.2.2.2 Eq. 3.2 to 3.5 elastic spectrum, 3.2.2.5 Eq. 3.12 to 3.15 design spectrum with lower bound beta = 0.2"},
                  "inputs": {"a_g": a_g, "S": S, "T_B": TB, "T_C": TC, "T_D": TD, "q": qf, "beta": 0.2, "T": T},
                  "expected": {"S_d": q(sd(a_g, S, TB, TC, TD, qf, T), "m/s^2", "eurocode-quantities-v1"), "S_e": q(se(a_g, S, TB, TC, TD, T), "m/s^2", "eurocode-quantities-v1")},
                  "references": ref_scipy})
doc = {"$schema": "../../../../../../../../../🔮️oracles/🏃️harness/🧬️schema/🔣️.json", "schemaVersion": 1, "artifact": "en1998", "annex": "de", "cases": cases}
open(OUT, "w", encoding="utf-8").write(json.dumps(doc, indent=2, ensure_ascii=False) + "\n")
