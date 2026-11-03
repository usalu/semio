"""Blessed third-party values of the v12 row checks (sympy symbolic evaluation, numpy numerics); writes third-party-v12-rows.json."""
import json, pathlib, numpy as np, sympy as sp
OUT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔮️oracle-cases/third-party-v12-rows.json")
REFS = [{"oracle": "norm-eurocode-scipy-numerics", "adapter": "numerical", "kind": "third-party-library"}]
def quantity(value, unit): return {"value": float(value), "unit": unit, "profile": "eurocode-quantities-v1"}
def case(cid, check, std, clause, inputs, expected):
    return {"id": cid, "check": check, "citation": {"standard": std, "edition": "2003", "clause": clause, "annex": "en"}, "inputs": inputs, "expected": expected, "references": REFS}
cases = []
L, b = sp.symbols("L b", positive=True)
for length, width in ((30.0, 3.0), (10.0, 2.0), (1000.0, 4.0)):
    q = float(np.clip(float((2000 + 120000 / (L + 30)).subs(L, length)), 2500.0, 5000.0))
    h = max(0.10 * q * length * width, 10000.0)
    cases.append(case(f"en1991.2.footbridge-load-L-{length:g}-b-{width:g}".replace(".", "_"), "en1991.2.footbridge-load", "EN 1991-2", "EN 1991-2 5.3.2.1 q_fk = 2.0 + 120/(L + 30) kN/m2 within 2.5 to 5.0 and 5.4 Q_flk = max(10 % of q_fk L b, 10 kN); constants unverified (registry)", {"L": length, "b": width}, {"q_fk": quantity(q, "Pa"), "Q_flk": quantity(h, "N")}))
for l_phi in (4.0, 25.0, 100.0):
    r = np.sqrt(l_phi) - 0.2
    cases.append(case(f"en1991.2.rail-dynamic-L-{l_phi:g}".replace(".", "_"), "en1991.2.rail-dynamic", "EN 1991-2", "EN 1991-2 6.4.5.2 Phi_2 = 1.44/(sqrt(L_Phi) - 0.2) + 0.82 in 1.00 to 1.67 and Phi_3 = 2.16/(sqrt(L_Phi) - 0.2) + 0.73 in 1.00 to 2.00", {"L_phi": l_phi}, {"Phi_2": quantity(np.clip(1.44 / r + 0.82, 1.0, 1.67), "1"), "Phi_3": quantity(np.clip(2.16 / r + 0.73, 1.0, 2.0), "1")}))
phi = sp.symbols("phi", positive=True)
for deg in (20.0, 30.0, 40.0):
    cases.append(case(f"en1991.4.bulk-k-mean-phi-{deg:g}".replace(".", "_"), "en1991.4.bulk-k-mean", "EN 1991-4", "EN 1991-4 Annex E K_m = 1.1 (1 - sin phi_i); factor unverified (registry)", {"phi_i_deg": deg}, {"K_m": quantity(float((sp.Rational(11, 10) * (1 - sp.sin(phi))).subs(phi, sp.rad(deg))), "1")}))
m, e, f = sp.symbols("m e f", positive=True)
for mass, ecc, freq in ((1000.0, 0.002, 25.0), (250.0, 0.0005, 50.0)):
    cases.append(case(f"en1991.3.machine-force-m-{mass:g}-f-{freq:g}".replace(".", "_"), "en1991.3.machine-force", "EN 1991-3", "EN 1991-3 5 F_dyn = m e omega^2 with omega = 2 pi f (spec line E3.07, unverified)", {"m": mass, "e": ecc, "f": freq}, {"F_dyn": quantity(float((m * e * (2 * sp.pi * f) ** 2).subs({m: mass, e: ecc, f: freq})), "N")}))
for a, w, tp, ts in ((0.4, 0.4, 0.1, 0.3), (0.2, 0.6, 0.0, 0.25)):
    ex = a + 2 * (tp + ts / 2); ey = w + 2 * (tp + ts / 2)
    cases.append(case(f"en1991.2.dispersal-footprint-{a:g}-{w:g}-{tp:g}-{ts:g}".replace(".", "_"), "en1991.2.dispersal-footprint", "EN 1991-2", "EN 1991-2 4.3.6 spreading at 45 degrees through pavement and half the slab", {"a": a, "b": w, "t_p": tp, "t_s": ts}, {"A_foot": quantity(ex * ey, "m2")}))
OUT.write_text(json.dumps({"$schema": "../../../../../../../../../🔮️oracles/🏃️harness/🧬️schema/🔣️.json", "schemaVersion": 1, "artifact": "en1991", "annex": "en", "cases": cases}, ensure_ascii=False, indent=2) + "\n")
print(len(cases))
