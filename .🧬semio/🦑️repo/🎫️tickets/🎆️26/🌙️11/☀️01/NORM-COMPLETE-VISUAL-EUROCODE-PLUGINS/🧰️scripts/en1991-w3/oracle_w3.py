"""Blessed third-party values of the wave 3 row checks (sympy symbolic evaluation, numpy numerics); writes third-party-w3-rows.json."""
import json, pathlib, numpy as np, sympy as sp
OUT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔮️oracle-cases/third-party-w3-rows.json")
REFS = [{"oracle": "norm-eurocode-scipy-numerics", "adapter": "numerical", "kind": "third-party-library"}]
def quantity(value, unit): return {"value": float(value), "unit": unit, "profile": "eurocode-quantities-v1"}
def case(cid, check, std, clause, inputs, expected):
    return {"id": cid, "check": check, "citation": {"standard": std, "edition": "2003", "clause": clause, "annex": "en"}, "inputs": inputs, "expected": expected, "references": REFS}
cases = []
Q = {1: 300e3, 2: 200e3, 3: 100e3}; U = {1: 9000.0, 2: 2500.0, 3: 2500.0}
for lane in (1, 2, 3):
    spacings = sp.Matrix([sp.Rational(12, 10), 6, sp.Rational(12, 10)])
    cases.append(case(f"en1991.2.fatigue-load-model-lane-{lane}", "en1991.2.fatigue-load-model", "EN 1991-2", "EN 1991-2 4.6.2 FLM1 = 0.7 Q_ik and 0.3 q_ik of LM1 (lane value, alpha = 1), 4.6.4 FLM3 four axles of 120 kN at 1.2, 6.0, 1.2 m; factors and axle data unverified (registry, spec line E2.10)", {"lane": float(lane)}, {"axle_flm1": quantity(sp.Rational(7, 10) * Q[lane], "N"), "udl_flm1": quantity(sp.Rational(3, 10) * U[lane], "Pa"), "axle_flm3": quantity(120e3, "N"), "length_flm3": quantity(sum(spacings), "m")}))
for soffit in (0.0, 1.0):
    expected = {"F_dx": quantity(500e3 if soffit else 1000e3, "N")}
    if not soffit: expected["F_dy"] = quantity(500e3, "N")
    cases.append(case(f"en1991.2.support-collision-{'soffit' if soffit else 'deck'}", "en1991.2.support-collision", "EN 1991-2", "EN 1991-2 4.7.3.1 collision forces on a support at deck level (F_dx 1000 kN, F_dy 500 kN) and on the soffit (F_dx 500 kN); the spec marks the values [V], unverified (registry)", {"soffit": soffit}, expected))
D, Qw, H, r = sp.symbols("D Q H r", positive=True)
for d, q, h, dist in ((2.0, 5.0e6, 3.0, 1.0), (1.0, 2.0e6, 2.5, 0.2), (3.0, 1.0e7, 4.0, 6.0)):
    subs = {D: d, Qw: q, H: h, r: dist}
    q_h = Qw / (sp.Float(1.11e6) * H ** sp.Rational(5, 2)); q_d = Qw / (sp.Float(1.11e6) * D ** sp.Rational(5, 2))
    l_h = (H * (sp.Float(2.9) * q_h ** sp.Float(0.33) - 1)).subs(subs)
    qd_value = float(q_d.subs(subs))
    zp = (2.4 * D * (q_d ** sp.Rational(2, 5) - q_d ** sp.Rational(2, 3)) if qd_value < 1.0 else 2.4 * D * (1 - q_d ** sp.Rational(2, 5))).subs(subs)
    y = float((dist + h + zp) / (l_h + h + zp))
    flux = 100e3 if y <= 0.3 else 136.3e3 - 121e3 * y if y < 1.0 else 15e3 * y ** -3.7
    cases.append(case(f"en1991.1-2.local-fire-hasemi-D-{d:g}-Q-{q:g}-H-{h:g}-r-{dist:g}".replace(".", "_"), "en1991.1-2.local-fire-hasemi", "EN 1991-1-2", "EN 1991-1-2 Annex C L_f = -1.02 D + 0.0148 Q^0.4, horizontal flame length L_h = H (2.9 Q_H*^0.33 - 1), virtual source z', Hasemi flux; the constants beyond the spec line E12.05 are recalled and unverified (registry)", {"D": d, "Q": q, "H": h, "r": dist}, {"L_f": quantity(-1.02 * d + 0.0148 * q ** 0.4, "m"), "L_h": quantity(float(l_h), "m"), "z_prime": quantity(float(zp), "m"), "flux": quantity(flux, "W/m2")}))
s, b, a = sp.symbols("s b alpha", positive=True)
for load, length, angle in ((680.0, 6.0, 30.0), (1250.0, 4.5, 20.0), (900.0, 10.0, 45.0)):
    cases.append(case(f"en1991.1-3.snow-guard-{load:g}-{length:g}-{angle:g}".replace(".", "_"), "en1991.1-3.snow-guard", "EN 1991-1-3", "EN 1991-1-3 5.3.8 and 6.3 snow guard line force F_s = s b sin(alpha)", {"s": load, "b": length, "alpha_deg": angle}, {"F_s": quantity(float((s * b * sp.sin(a)).subs({s: load, b: length, a: sp.rad(angle)})), "N/m")}))
dm, b3 = sp.symbols("dm beta3", real=True)
for fraction, device, beta in ((0.4, "grab", 0.5), (0.25, "magnet", 1.0), (0.0, "grab", 0.5)):
    cases.append(case(f"en1991.3.hoist-release-{device}-{fraction:g}".replace(".", "_"), "en1991.3.hoist-release", "EN 1991-3", "EN 1991-3 2.5 phi_3 = 1 - (dm/m)(1 + beta_3) for the sudden release of a payload fraction; beta_3 of grab 0.5 and magnet 1.0 are unverified registry rows (registry rows)", {"dm_over_m": fraction, "beta_3": beta}, {"phi_3": quantity(float((1 - dm * (1 + b3)).subs({dm: fraction, b3: beta})), "1")}))
for alpha in (1.0, 1.21):
    q_ii = float(sp.Rational(14, 10) * sp.Float(alpha) * 80000)
    cases.append(case(f"en1991.2.rail-derailment-load-alpha-{alpha:g}".replace(".", "_"), "en1991.2.rail-derailment-load", "EN 1991-2", "EN 1991-2 6.7.1 derailment line loads (design situation I 45 kN/m per the spec line E2.19; situation II as 1.4 alpha q_vk, loaded length 20 m, offset at most 1.5 s of the track gauge s = 1.435 m; every value an unverified registry row)", {"alpha": alpha}, {"q_i": quantity(45000, "N/m"), "q_ii": quantity(q_ii, "N/m"), "length": quantity(20, "m"), "offset_limit": quantity(float(sp.Rational(3, 2) * sp.Rational(1435, 1000)), "m")}))
span_s, alpha_s = sp.symbols("span alpha", positive=True)
for span, alpha in ((30.0, 1.21), (100.0, 1.0), (400.0, 1.33)):
    traction = sp.Min(sp.Float(33000) * span_s, 1000000)
    braking = sp.Min(sp.Float(20000) * span_s, 6000000)
    long = float(sp.Max(traction, braking).subs(span_s, span))
    cases.append(case(f"en1991.2.rail-group-bases-{span:g}-{alpha:g}".replace(".", "_"), "en1991.2.rail-group-bases", "EN 1991-2", "EN 1991-2 Table 6.11 groups on the characteristic bases of the spec line E2.17: the larger of traction min(33 kN/m L, 1000 kN) and braking min(20 kN/m L, 6000 kN), and the nosing force alpha 100 kN (unverified registry rows)", {"span": span, "alpha": alpha}, {"F_long": quantity(long, "N"), "F_nosing": quantity(float(alpha_s.subs(alpha_s, alpha) * 100000), "N")}))
kst_s, pm_s, v_s, ps_s, pr_s = sp.symbols("K_St p_max V p_stat p_red", positive=True)
vent = (sp.Float(3.264e-5) * pm_s * kst_s * pr_s ** sp.Float(-0.569) + sp.Float(0.27) * (ps_s - sp.Float(0.1)) * pr_s ** sp.Rational(-1, 2)) * v_s ** sp.Float(0.753)
for kst, pm, volume, ps, pr in ((150.0, 8.0, 100.0, 0.2, 0.5), (150.0, 8.0, 100.0, 0.2, 1.0), (200.0, 9.0, 500.0, 0.1, 1.5)):
    cases.append(case(f"en1991.4.dust-vent-area-{kst:g}-{pm:g}-{volume:g}-{ps:g}-{pr:g}".replace(".", "_"), "en1991.4.dust-vent-area", "EN 1991-4", "EN 14491 vent area after Bartknecht A = [3.264e-5 p_max K_St p_red^-0.569 + 0.27 (p_stat - 0.1) p_red^-0.5] V^0.753, pressures in bar (recalled; the spec line E4.07 states no value; unverified registry rows)", {"K_St": kst, "p_max_bar": pm, "V": volume, "p_stat_bar": ps, "p_red_bar": pr}, {"A_v": quantity(float(vent.subs({kst_s: kst, pm_s: pm, v_s: volume, ps_s: ps, pr_s: pr})), "m2")}))
OUT.write_text(json.dumps({"$schema": "../../../../../../../../../🔮️oracles/🏃️harness/🧬️schema/🔣️.json", "schemaVersion": 1, "artifact": "en1991", "annex": "en", "cases": cases}, ensure_ascii=False, indent=2) + "\n")
print(len(cases))
