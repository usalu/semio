"""Adds the vocabulary rows the registry entries read but the vocabulary lacks, and fixes the kinds of text rows that evaluators read as numbers (found by the demo values). Edits `📚️inputs/🔣️.json` and the generated `📚️inputs/🦀️.rs` in step. Idempotent. Run with python3 -I vocab_patch.py."""
import json, re, sys
sys.path.insert(0, "."); from dv_lib import *
UNIT = {"m": ("Metre", "m"), "1": ("One", "1"), "N": ("Newton", "N"), "Pa": ("Pascal", "Pa"), "deg": ("Degree", "°"), "mm": ("Millimetre", "mm"), "kg/m3": ("KilogramPerCubicMetre", "kg/m³")}
def rs_row(kind, key, en, de, unit, scope):
    return f'    InputSpec::{kind}("{key}", "{en}", "{de}", UnitId::{UNIT[unit][0]})' + (f'.scope("{scope}")' if scope else "") + ",\n"
def add(code, rows, confidence=True):
    vp = vocab_path(code); rp = vp.parent / "🦀️.rs"
    v = json.loads(vp.read_text(encoding="utf-8")); rs = rp.read_text(encoding="utf-8")
    have = {(r["key"], r.get("scope", "")) for r in v["inputs"]}
    new_rs = ""
    for kind, key, en, de, unit, scope in rows:
        if (key, scope) in have: continue
        row = {"key": key, "scope": scope, "kind": kind, "en": en, "de": de}
        if confidence: row["confidence"] = "high"
        row["unit"], row["unitId"] = UNIT[unit][1], UNIT[unit][0]
        v["inputs"].append(row); new_rs += rs_row(kind, key, en, de, unit, scope)
    v["inputs"].sort(key=lambda r: (r["key"], r.get("scope", "")))
    vp.write_text(json.dumps(v, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    if new_rs:
        i = rs.rindex("];")
        rp.write_text(rs[:i] + new_rs + rs[i:], encoding="utf-8")
    print(code, "added", new_rs.count("\n"), "rows")
add("en1997", [
    ("number", "phi_assumed_deg", "Assumed friction angle φ′", "Angesetzter Reibungswinkel φ′", "deg", "GI-17"),
    ("number", "c_assumed", "Assumed cohesion c′", "Angesetzte Kohäsion c′", "Pa", "GI-17"),
    ("series", "ts1", "Settlement of load-test step 1 (series)", "Setzung des Laststufe 1 (Reihe)", "m", "PILE-10"),
    ("series", "tq1", "Load of load-test step 1 (series)", "Last der Laststufe 1 (Reihe)", "N", "PILE-10"),
    ("number", "s_crit_factor", "Failure settlement factor s_g/D", "Grenzsetzungsfaktor s_g/D", "1", "PILE-10"),
    ("series", "p1_h1", "Layer thickness of profile 1 (series)", "Schichtdicke des Profils 1 (Reihe)", "m", "PILE-11"),
    ("series", "p1_qs1", "Unit shaft resistance of profile 1 (series)", "Mantelreibung des Profils 1 (Reihe)", "Pa", "PILE-11"),
    ("number", "p1_qb", "Unit base resistance of profile 1", "Spitzendruck des Profils 1", "Pa", "PILE-11"),
    ("number", "width", "Extrusion width of the slip body", "Breite des Gleitkörpers", "m", "SLOPE-05"),
    ("series", "lv1", "Vertical creep segment 1 (series)", "Senkrechter Sickerwegabschnitt 1 (Reihe)", "m", "HYD-02"),
    ("series", "lh1", "Horizontal creep segment 1 (series)", "Waagerechter Sickerwegabschnitt 1 (Reihe)", "m", "HYD-02"),
    ("number", "i_exit", "Exit gradient i", "Austrittsgefälle i", "1", "HYD-02"),
    ("number", "c_lane", "Required Lane ratio", "Erforderliches Lane-Verhältnis", "1", "HYD-02"),
    ("number", "c_bligh", "Required Bligh ratio", "Erforderliches Bligh-Verhältnis", "1", "HYD-02"),
])
add("en1995", [(("number"), f"o{k}", f"Orientation of layer {k} (1 along the grain, 0 across)", f"Ausrichtung der Lage {k} (1 längs, 0 quer)", "1", "T-97,T-107") for k in (2, 3, 4, 5)] + [("number", f"t{k}", f"Thickness t{k}", f"Dicke t{k}", "mm", "T-97,T-107") for k in (3, 4, 5)], confidence=False)
for code, key, old_kind in (("en1995", "o1", "text"), ("en1996", "density", "text")):
    vp = vocab_path(code); rp = vp.parent / "🦀️.rs"
    v = json.loads(vp.read_text(encoding="utf-8")); rs = rp.read_text(encoding="utf-8")
    unit = "1" if code == "en1995" else "kg/m3"
    for r in v["inputs"]:
        if r["key"] == key and r["kind"] == "text":
            r["kind"] = "number"; r["unit"], r["unitId"] = UNIT[unit][1], UNIT[unit][0]
    vp.write_text(json.dumps(v, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    rs = re.sub(r'InputSpec::text\("%s", ("[^"]*"), ("[^"]*")\)' % key, r'InputSpec::number("%s", \1, \2, UnitId::%s)' % (key, UNIT[unit][0]), rs)
    rp.write_text(rs, encoding="utf-8")
    print(code, "fixed kind of", key)
