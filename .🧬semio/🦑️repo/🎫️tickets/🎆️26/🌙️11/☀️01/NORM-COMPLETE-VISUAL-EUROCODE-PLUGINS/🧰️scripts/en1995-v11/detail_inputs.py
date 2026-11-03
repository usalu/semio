"""Declares the detail inputs of the EN 1995 catalogue entries T-95 (truss layout) and T-96 (bracing layout) in the vocabulary (JSON and Rust in step), lists them (plus the evaluator's `n_braced`) in the entry inputs, and gives the demo values via `EXTRA` of values_en1995.py. Idempotent. Run with python3 -I detail_inputs.py, then demo_gen.py en1995 and geometry_gen.py."""
import json, pathlib, re, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent.parent / "demo-values"))
from dv_lib import *
vp = vocab_path("en1995"); rp = vp.parent / "🦀️.rs"
v = json.loads(vp.read_text(encoding="utf-8")); rs = rp.read_text(encoding="utf-8")
MM = ("mm", "Millimetre", "Millimetre")
NUM = [
    ("T-95", "span", "Truss span", "Fachwerkspannweite", MM),
    ("T-95", "height", "Truss height", "Fachwerkhöhe", MM),
    ("T-95", "bays", "Number of bays", "Anzahl der Felder", ("1", "One", "One")),
    ("T-95", "chord_b", "Chord width", "Gurtbreite", MM),
    ("T-95", "chord_h", "Chord depth", "Gurthöhe", MM),
    ("T-95", "web_b", "Web member width", "Füllstabbreite", MM),
    ("T-95", "web_h", "Web member depth", "Füllstabhöhe", MM),
    ("T-96", "spacing", "Spacing of the stabilised members", "Abstand der ausgesteiften Bauteile", MM),
    ("T-96", "member_b", "Stabilised member width", "Breite des ausgesteiften Bauteils", MM),
    ("T-96", "member_h", "Stabilised member depth", "Höhe des ausgesteiften Bauteils", MM),
    ("T-96", "brace_b", "Bracing member width", "Aussteifungsstabbreite", MM),
    ("T-96", "brace_h", "Bracing member depth", "Aussteifungsstabhöhe", MM),
]
KINDS = {
    "T-95": ("Truss type", "Fachwerkart", "warren", [("warren", "Warren", "Warren"), ("pratt", "Pratt", "Pratt"), ("howe", "Howe", "Howe"), ("king-post", "King post", "Hängewerk")]),
    "T-96": ("Bracing type", "Aussteifungsart", "plan-bracing", [("plan-bracing", "Plan bracing", "Dachverband"), ("bridging", "Bridging", "Queraussteifung"), ("elastic-restraint", "Elastic restraint", "Elastische Halterung")]),
}
q = lambda s: json.dumps(s, ensure_ascii=False)
have = {(r["key"], r.get("scope", "")) for r in v["inputs"]}
anchor = '    InputSpec::number("n_braced"'
lines = []
for scope, key, en, de, (unit, uid, rust) in NUM:
    if (key, scope) in have:
        continue
    row = {"key": key, "scope": scope, "kind": "number", "en": en, "de": de, "unit": unit, "unitId": uid}
    spec = f'InputSpec::number({q(key)}, {q(en)}, {q(de)}, UnitId::{rust})'
    if key == "bays":
        row["min"], row["max"] = 2.0, 24.0
        spec += ".range(2.0, 24.0)"
    v["inputs"].append(row)
    lines.append(f'    {spec}.scope({q(scope)}),\n')
for scope, (en, de, example, choices) in KINDS.items():
    if ("kind", scope) in have:
        continue
    v["inputs"].append({"key": "kind", "scope": scope, "kind": "choice", "en": en, "de": de, "choices": [{"value": a, "en": b, "de": c} for a, b, c in choices], "example": example})
    cs = ", ".join(f'InputChoice::new({q(a)}, {q(b)}, {q(c)})' for a, b, c in choices)
    lines.append(f'    InputSpec::choice("kind", {q(en)}, {q(de)}, &[{cs}]).example_text({q(example)}).scope({q(scope)}),\n')
if lines:
    i = rs.index(anchor); j = rs.index("\n", i) + 1
    rs = rs[:j] + "".join(lines) + rs[j:]
v["inputs"].sort(key=lambda x: (x["key"], x.get("scope", "")))
vp.write_text(json.dumps(v, indent=2, ensure_ascii=False) + "\n", encoding="utf-8"); rp.write_text(rs, encoding="utf-8")
comp = art("en1995") / "🧬️schema/⚖️kernel/🧩️components/🦀️.rs"
t = comp.read_text(encoding="utf-8")
new = {
    "T-95": '"n_braced", "n_d", "l_m", "product", "q_rd", "kind", "span", "height", "bays", "chord_b", "chord_h", "web_b", "web_h"',
    "T-96": '"n_braced", "n_d", "l_m", "f_rd", "k_br", "kind", "spacing", "member_b", "member_h", "brace_b", "brace_h"',
}
old = {"T-95": '"n_d", "l_m", "product", "q_rd"', "T-96": '"n_d", "l_m", "f_rd", "k_br"'}
for e in new:
    t = t.replace(f'id: "{e}",', f'id: "{e}",', 1)
    pat = re.compile(r'(Entry \{ id: "%s",[^\n]*?inputs: &\[)%s(\])' % (e, re.escape(old[e])))
    t = pat.sub(lambda m: m.group(1) + new[e] + m.group(2), t)
comp.write_text(t, encoding="utf-8")
vals = TICKET / "🧰️scripts/demo-values/values_en1995.py"
s = vals.read_text(encoding="utf-8")
if '"T-95": {' not in s:
    extra = '    "T-95": {"span": 6000.0, "height": 900.0, "bays": 6.0, "chord_b": 100.0, "chord_h": 200.0, "web_b": 80.0, "web_h": 120.0},\n    "T-96": {"spacing": 800.0, "member_b": 100.0, "member_h": 200.0, "brace_b": 60.0, "brace_h": 120.0},\n'
    s = s.replace('EXTRA = {\n', 'EXTRA = {\n' + extra, 1)
    vals.write_text(s, encoding="utf-8")
print("patched", len(lines), "vocabulary rows")
