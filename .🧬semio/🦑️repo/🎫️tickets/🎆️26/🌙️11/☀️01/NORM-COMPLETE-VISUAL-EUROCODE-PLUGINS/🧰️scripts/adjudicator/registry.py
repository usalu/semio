"""Adjudicator wave 2: NDP registry rows decided from the clause text. Idempotent. Run: python -I registry.py, then regenerate the tables with the ticket codegen."""
import json, pathlib
REG = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json")
d = json.loads(REG.read_text(encoding="utf-8"))
rows = {e["id"]: e for e in d["entries"]}
SPEC = "adjudicator wave 2 (oracle vs Rust disagreement)"
EN95 = "adjudicator 2026-11-03: EN column read in BS EN 1995-1-1:2004+A1:2008 (9.2.5.2(3) Expression 9.35, 9.2.5.3(1) Expressions 9.37 and 9.38, Table 9.2, recommended values underlined: k_f,1 50, k_f,2 80, k_f,3 30, ranges 50-80, 80-100, 30-80). DE column NOT verified: the DIN EN 1995-1-1/NA text was not available and the column repeats the recommended value; an earlier recollection of NA values 30 (solid) and 50 (glulam, LVL) has no source and contradicts the structure of the EN (k_f,3 is one factor of q_d, the product dependence sits in k_f,1 and k_f,2). Kernel and oracle read this row."
def patch(i, **kw):
    e = rows[i]
    for k, v in kw.items():
        if k == "locator": e["source"]["locator"] = v
        else: e[k] = v
patch("ndp.en1995.bracing.stabilising-divisor", clause="EN 1995-1-1 9.2.5.3(1)", locator="EN 1995-1-1 9.2.5.3(1)", symbol="k_f,3 of the stabilising load q_d = k_l n N_d / (k_f,3 l) (9.37)", asWritten="k_f,3 = 30 (recommended, range 30 to 80); q_d = k_l N_d / (k_f,3 l) with N_d the sum of the member forces n N_d; unverified for DE", specRef=SPEC, evidence=EN95)
patch("ndp.en1995.bracing.kl-reference-length", clause="EN 1995-1-1 9.2.5.3(1)", locator="EN 1995-1-1 9.2.5.3(1)", symbol="reference length of k_l = min(1, sqrt(15 / l)) (9.38)", asWritten="k_l = min(1, sqrt(15 / l)) with the overall span l in m (9.38, printed constant 15, not an NDP); unverified for DE", specRef=SPEC, evidence=EN95)
patch("ndp.en1995.bracing.force-divisor-solid", clause="EN 1995-1-1 9.2.5.2(3)", locator="EN 1995-1-1 9.2.5.2(3)", symbol="k_f,1 of the stabilising force F_d = N_d / k_f,1 for solid timber (9.35)", asWritten="k_f,1 = 50 (recommended, range 50 to 80); unverified for DE", specRef=SPEC, evidence=EN95)
patch("ndp.en1995.bracing.force-divisor-engineered", clause="EN 1995-1-1 9.2.5.2(3)", locator="EN 1995-1-1 9.2.5.2(3)", symbol="k_f,2 of the stabilising force F_d = N_d / k_f,2 for glued laminated timber and LVL (9.35)", asWritten="k_f,2 = 80 (recommended, range 80 to 100); unverified for DE", specRef=SPEC, evidence=EN95)
patch("ndp.en1995.bracing.deflection-limit-divisor", clause="EN 1995-1-1 9.2.5.3(2)", locator="EN 1995-1-1 9.2.5.3(2)", symbol="bracing deflection limit l / divisor", asWritten="horizontal deflection of the bracing system under q_d and external load not above l/500 (9.2.5.3(2), printed constant, not an NDP); unverified for DE", specRef=SPEC, evidence=EN95.split(" DE column")[0] + " 9.2.5.3(2) prints l/500.")
patch("ndp.en1991.e17.blast", asWritten="Mills (1987) blast overpressure fit p = 1772/Z^3 - 114/Z^2 + 108/Z kPa, Z = R/W^(1/3), and Annex D gas explosion formulas | Mills-Explosionsdruck und Anhang D Gasexplosion", specRef=SPEC, evidence="adjudicator 2026-11-03: EN 1991-1-7 prints no free-air blast overpressure formula (Annex D is the internal gas explosion; the catalogue entry E17.10 is editorial, the spec says its Annex E is not used). Rust and oracle both use the Mills fit and agree to 1e-11. The independent Kinney-Graham fit (scipy.constants.atm scaled, oracle `kinney_graham`) gives 8.4 percent less at W = 100 kg, R = 30 m (18 828 Pa against 20 544 Pa): model uncertainty of the empirical fit, not a calculation error. No printed value exists to verify; every use stays an unverified basis.")
def new(i, clause, symbol, rec, unit, note, part):
    if i in rows: return
    rows[i] = {"id": i, "code": "EN 1991", "part": part, "clause": clause, "symbol": symbol, "recommended": float(rec), "naDe": float(rec), "unit": unit, "source": {"document": "DIN EN 1991/NA", "edition": "to be recorded at verification", "locator": clause}, "status": "unverified", "specRef": SPEC, "asWritten": note, "evidence": ""}
AAC = "Table 2.1 of EN 1991-4 as recalled from secondary sources (national annexes quote Class 1 below 100 t and Class 3 above 10 000 t, Class 2 for the rest, so both boundary values are Class 2); the spec line 346 writes the boundaries inclusive (AAC1 <= 100 t, AAC2 <= 10 000 t). Printed table not available: unverified, kernel and oracle read this row."
new("ndp.en1991.4.aac-class1-below", "EN 1991-4 2.2 Table 2.1", "silo capacity below which the action assessment class is 1", 100, "t", "Action assessment class 1: capacity < 100 t. " + AAC, "4")
new("ndp.en1991.4.aac-class3-above", "EN 1991-4 2.2 Table 2.1", "silo capacity above which the action assessment class is 3", 10000, "t", "Action assessment class 3: capacity > 10 000 t. " + AAC, "4")
FB = "EN 1990 A2.4.3.2(1) asks for the comfort verification when the fundamental frequency is below 5 Hz (vertical) or 2.5 Hz (horizontal and torsional); the walking excitation ranges named by the spec for EN 1991-2 5.7 (vertical 1.7 to 2.1 Hz, horizontal 0.9 to 1.1 Hz) are a different quantity and are not used. Jogging 2.0 to 3.5 Hz from the spec line 326. Unverified against the print: kernel and oracle read this row."
new("ndp.en1991.2.footbridge-band-vertical-max", "EN 1990 A2.4.3.2(1)", "upper bound of the vertical natural frequency band needing the comfort check", 5.0, "Hz", FB, "2")
new("ndp.en1991.2.footbridge-band-horizontal-max", "EN 1990 A2.4.3.2(1)", "upper bound of the horizontal and torsional natural frequency band needing the comfort check", 2.5, "Hz", FB, "2")
new("ndp.en1991.2.footbridge-band-jogging-min", "EN 1991-2 5.7", "lower bound of the jogging excitation band", 2.0, "Hz", FB, "2")
new("ndp.en1991.2.footbridge-band-jogging-max", "EN 1991-2 5.7", "upper bound of the jogging excitation band", 3.5, "Hz", FB, "2")
d["entries"] = sorted(rows.values(), key=lambda e: e["id"])
REG.write_text(json.dumps(d, ensure_ascii=False, indent=1), encoding="utf-8")
print(len(rows))
