#!/usr/bin/env python3
"""One-shot (typed enums v4): en1997 call sites that take the typed enums through `&str`. The model enums get `as_str` (the legacy camelCase token) and `Display`; the lines the generator lists as residual are rewritten to `.as_str()`. Reruns are no-ops."""
import pathlib, re, sys
A = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/"
ROOT = pathlib.Path(A)
ENUMS = {
    "SituationChoice": ["persistent", "transient", "accidental"],
    "ApproachChoice": ["da1str", "da1geo", "da2", "da2star", "da3", "geo3"],
    "SoilType": ["sand", "gravel", "clay", "silt", "fill"],
    "PileType": ["bored", "driven", "cfa"],
    "EarthPressureMode": ["active", "atRest", "increasedActive"],
    "WallMovement": ["free", "rigid", "propped"],
    "TendonType": ["strand", "bar"],
    "EmbeddedWallKind": ["freeEarth", "cantilever"],
    "LabTestKind": ["oedometer", "triaxial", "directShear"],
}
def variant(tok):
    return tok[:1].upper() + tok[1:]
SNAP = ROOT / "🧬️schema/📸️snapshot/🦀️.rs"
t = SNAP.read_text(encoding="utf-8")
if "impl SituationChoice" not in t:
    add = ""
    for e, toks in ENUMS.items():
        arms = "".join(f'            Self::{variant(k)} => "{k}",\n' for k in toks)
        add += f"""
impl {e} {{
    /// 🔤 Camel-case wire token of the variant, for the string-keyed registry and case parameters.
    pub const fn as_str(self) -> &'static str {{
        match self {{
{arms}        }}
    }}
}}

impl std::fmt::Display for {e} {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        f.write_str(self.as_str())
    }}
}}
"""
    t = t.rstrip("\n") + "\n" + add
t = t.replace('design_situation: "".into(),', "design_situation: SituationChoice::Persistent,")
SNAP.write_text(t, encoding="utf-8")

log = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8")
FIELDS = r"design_situation|design_approach|soil_type|pile_type|earth_pressure_mode|wall_movement|tendon|kind"
n = 0
for m in re.finditer(r"^\s*residual: (.+?): (\d+): ", log, re.M):
    rel, ln = m.group(1), int(m.group(2))
    if "mutations/" in rel or rel.endswith(".py") or rel == "🧬️schema/🦀️.rs":
        continue
    p = ROOT / rel
    lines = p.read_text(encoding="utf-8").split("\n")
    old = lines[ln - 1]
    new = re.sub(r"&(\w+(?:\.\w+)*)\.(%s)\b(?![\w(]|\.as_str)" % FIELDS, r"\1.\2.as_str()", old)
    new = re.sub(r"\b(\w+(?:\.\w+)*)\.(design_situation|design_approach)\.clone\(\)", r"\1.\2.as_str().to_string()", new)
    new = re.sub(r"match (\w+(?:\.\w+)*)\.(earth_pressure_mode) \{", r"match \1.\2.as_str() {", new)
    if new != old:
        lines[ln - 1] = new
        p.write_text("\n".join(lines), encoding="utf-8")
        n += 1
CK = ROOT / "🧬️schema/💡️inferences/⚖️checks/🪨️layer-and-duplicate-id-helpers/🦀️.rs"
t = CK.read_text(encoding="utf-8")
t = t.replace('''    if !lc.design_situation.trim().is_empty() {
        lc.design_situation.clone()
    } else {
        doc.design_situation.clone()
    }''', '''    let _ = doc;
    lc.design_situation.as_str().to_string()''')
CK.write_text(t, encoding="utf-8")
print("lines rewritten", n)
