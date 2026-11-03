"""🧪️ EN 1991 wave 3 oracle cases (`third-party-v3-extended.json`): placeholder expectations for the bridge wind force, the structural factor, vortex shedding, impact, internal explosion, bridge temperature actions, LM3 special vehicles and the roof step drift; bless with the harness (`oracle-en1991-v3-bless.sh`).

Usage: /home/deck/semio/.venv/bin/python -I oracle-en1991-v3-gen.py
"""

import json
from pathlib import Path

ROOT = Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm")
FOLDER = ROOT / "🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔮️oracle-cases"
SCHEMA = ROOT / "🔮️oracles/🏃️harness/🧬️schema/🔣️.json"
SY, NU = ("norm-eurocode-sympy", "sympy"), ("norm-eurocode-scipy-numerics", "numerical")
EQ, NUM = "eurocode-quantities-v1", "numerical-v1"
UNVERIFIED = "registry values recalled from the standard and unverified (ndp.en1991.*)"


def ref(lib):
    return {"oracle": lib[0], "adapter": lib[1], "kind": "independent-formulation"}


def case(slug, check, clause, standard, inputs, expected, refs, annex="en"):
    return {"id": f"en1991.{slug}", "check": check, "citation": {"standard": standard, "edition": "2005", "clause": clause, "annex": annex}, "inputs": inputs, "expected": {k: {"value": 0, "unit": v[0], "profile": v[1]} for k, v in expected.items()}, "references": refs}


WIND = {"open_add": 0.3, "road_add": 2.0, "rail_add": 4.0, "r_narrow": 0.5, "r_wide": 4.0, "cfx_narrow": 2.4, "cfx_wide": 1.3, "cfz": 0.9, "longitudinal": 0.25}
cases = []
for slug, over in (("open-parapet-wide", {"b": 12.0, "d": 2.0, "parapet_kind": 1, "d1": 1.1, "traffic_kind": 0}), ("solid-parapet-road-traffic", {"b": 9.0, "d": 1.6, "parapet_kind": 2, "d1": 1.1, "traffic_kind": 1}), ("narrow-deck-rail", {"b": 4.0, "d": 3.0, "parapet_kind": 0, "d1": 0.0, "traffic_kind": 2})):
    cases.append(case(f"8.bridge-wind-force-{slug}", "en1991.8.bridge-wind-force", f"EN 1991-1-4 8.3.1 Table 8.1 and Fig. 8.3, 8.3.2 Eq. 8.1, 8.3.3, 8.3.4; {UNVERIFIED}", "EN 1991-1-4", {"q_p": 1250.0} | WIND | over, {"d_tot": ("m", EQ), "c_fx0": ("1", EQ), "F_x": ("N/m", EQ), "F_z": ("N/m", EQ), "F_y": ("N/m", EQ)}, [ref(NU), ref(SY)]))
for slug, over in (("tower-60m", {"h": 60.0, "b": 8.0, "z0": 0.3, "z_min": 5.0, "v_b": 25.0, "delta": 0.05}), ("chimney-120m", {"h": 120.0, "b": 6.0, "z0": 0.05, "z_min": 2.0, "v_b": 27.5, "delta": 0.03}), ("concrete-building-90m", {"h": 90.0, "b": 30.0, "z0": 1.0, "z_min": 10.0, "v_b": 24.0, "delta": 0.1})):
    cases.append(case(f"6.structural-factor-{slug}", "en1991.6.structural-factor", "EN 1991-1-4 6.3.1 Eq. 6.1, Annex B.1 to B.4 Eq. B.1, B.3, B.4, B.7, B.8 with n_1 = 46/h", "EN 1991-1-4", {"n1": 46.0 / over["h"], "L_t": 300.0, "z_t": 200.0, "z0_II": 0.05, "nu_min": 0.08, "T": 600.0} | over, {"z_s": ("m", EQ), "L": ("m", EQ), "B2": ("1", EQ), "S_L": ("1", EQ), "R2": ("1", EQ), "nu": ("Hz", EQ), "k_p": ("1", EQ), "c_s_c_d": ("1", EQ)}, [ref(NU)]))
for slug, over in (("steel-chimney", {"b": 3.0, "n1": 0.8, "St": 0.18, "delta_s": 0.012, "m_e": 800.0, "v_m": 30.0}), ("slender-mast", {"b": 0.6, "n1": 2.5, "St": 0.18, "delta_s": 0.02, "m_e": 60.0, "v_m": 22.0})):
    cases.append(case(f"6.vortex-shedding-{slug}", "en1991.6.vortex-shedding", f"EN 1991-1-4 Annex E.1.3.1 Eq. E.1 and E.2, E.1.3.3 Eq. E.4; {UNVERIFIED}", "EN 1991-1-4", {"rho": 1.25, "onset": 1.25, "nu": 1.5e-5} | over, {"v_crit": ("m/s", EQ), "Sc": ("1", EQ), "v_onset": ("m/s", EQ), "Re": ("1", EQ)}, [ref(NU), ref(SY)]))
for slug, over in (("car", {"m": 1500.0, "v": 15.0, "k": 300000.0}), ("lorry", {"m": 30000.0, "v": 12.5, "k": 300000.0}), ("ship", {"m": 2.0e6, "v": 3.0, "k": 5.0e6})):
    cases.append(case(f"7.impact-force-{slug}", "en1991.7.impact-force", "EN 1991-1-7 Annex C.1 and C.2: F = v sqrt(k m) from the energy balance of a linear spring", "EN 1991-1-7", over, {"F": ("N", NUM), "E": ("J", EQ)}, [ref(NU), ref(SY)]))
for slug, over in (("small-vent", {"p_v": 5.0, "V": 100.0, "a_v": 2.0}), ("large-vent", {"p_v": 5.0, "V": 100.0, "a_v": 40.0}), ("strong-vent-panel", {"p_v": 10.0, "V": 60.0, "a_v": 3.0})):
    cases.append(case(f"7.gas-explosion-{slug}", "en1991.7.gas-explosion", f"EN 1991-1-7 Annex D.1 and D.2; {UNVERIFIED}", "EN 1991-1-7", {"base": 3.0, "coefficient": 0.04} | over, {"p_d1": ("kPa", EQ), "p_d2": ("kPa", EQ), "p_d": ("kPa", EQ), "ratio": ("1", EQ)}, [ref(NU), ref(SY)]))
for slug, over in (("steel-deck", {"shift_min": -3.0, "shift_max": 16.0, "dtm_heat": 18.0, "dtm_cool": 13.0, "ksur_heat": 1.0, "ksur_cool": 1.0, "alpha": 12e-6, "h": 1.5, "L": 60.0}), ("composite-deck", {"shift_min": 4.0, "shift_max": 4.0, "dtm_heat": 15.0, "dtm_cool": 18.0, "ksur_heat": 1.0, "ksur_cool": 1.0, "alpha": 10e-6, "h": 2.2, "L": 90.0}), ("concrete-box", {"shift_min": 8.0, "shift_max": 2.0, "dtm_heat": 10.0, "dtm_cool": 5.0, "ksur_heat": 1.0, "ksur_cool": 1.0, "alpha": 10e-6, "h": 3.0, "L": 120.0})):
    cases.append(case(f"5.bridge-temperature-{slug}", "en1991.5.bridge-temperature", f"EN 1991-1-5 6.1.3 Fig. 6.1, 6.1.4 Table 6.1, 6.1.5; {UNVERIFIED}", "EN 1991-1-5", {"t_min": -24.0, "t_max": 37.0, "t_0": 10.0, "omega_n": 0.35, "omega_m": 0.75} | over, {"dt_con": ("K", EQ), "dt_exp": ("K", EQ), "kappa": ("1", NUM), "theta": ("rad", NUM), "movement": ("m", EQ), "combined": ("K", EQ)}, [ref(NU)]))
for designation in ("600/150", "1800/200", "3000/200", "3600/200"):
    cases.append(case(f"2.special-vehicle-{designation.replace('/', '-')}", "en1991.2.special-vehicle", f"EN 1991-2 Annex A standard special vehicles total/axle line in kN; {UNVERIFIED}", "EN 1991-2", {"designation": designation, "spacing": 1.5}, {"total": ("N", EQ), "axle": ("N", EQ), "lines": ("1", EQ), "length": ("m", EQ), "sum": ("N", EQ)}, [ref(NU)]))
for slug, over in (("high-step", {"h": 1.5, "b1": 8.0, "b2": 12.0, "s_k": 1100.0, "pitch_upper": 25.0}), ("low-step", {"h": 0.5, "b1": 2.0, "b2": 3.0, "s_k": 1000.0, "pitch_upper": 10.0}), ("flat-upper", {"h": 2.0, "b1": 10.0, "b2": 14.0, "s_k": 800.0, "pitch_upper": 0.0})):
    cases.append(case(f"snow.roof-step-drift-{slug}", "en1991.snow.roof-step-drift", f"EN 1991-1-3 5.3.6 Eq. 5.5 to 5.7; {UNVERIFIED}", "EN 1991-1-3", {"gamma": 2000.0, "cap": 4.0, "share": 0.5} | over, {"mu_w": ("1", EQ), "mu_s": ("1", EQ), "l_s": ("m", EQ)}, [ref(NU), ref(SY)]))
rel = Path(*([".."] * len(FOLDER.relative_to(ROOT).parts))) / SCHEMA.relative_to(ROOT)
document = {"$schema": str(rel), "schemaVersion": 1, "artifact": "en1991", "annex": "en", "cases": cases}
(FOLDER / "third-party-v3-extended.json").write_text(json.dumps(document, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
print(len(cases), "cases")
