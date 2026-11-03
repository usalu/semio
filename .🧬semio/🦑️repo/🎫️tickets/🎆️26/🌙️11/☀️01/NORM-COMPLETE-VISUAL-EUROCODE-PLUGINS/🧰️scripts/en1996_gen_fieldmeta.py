"""Adds the EN 1996 wave 2 fields (EN + DE) to the editor field-meta table. Idempotent."""
import os
HERE = os.path.dirname(os.path.abspath(__file__))
exec(compile(open(os.path.join(HERE, "en1996_spec.py"), encoding="utf-8").read(), "en1996_spec.py", "exec"))
p = f"{SUB}/✏️editor/🏷️field-meta/🦀️.rs"
s = rd(p)
if "TIE_KINDS" in s:
    raise SystemExit("present")
UNIT = {"m": "UnitId::Metre", "N": "UnitId::Newton", "m2": "UnitId::SquareMetre", "Pa": "UnitId::Pascal", "deg": "UnitId::Degree", "kg/m3": "UnitId::KilogramPerCubicMetre", "Nm": "UnitId::NewtonMetre", "1": "UnitId::One"}
CH = {"TieKind": "TIE_KINDS", "BarKind": "BAR_KINDS", "FrostClass": "FROST_CLASSES"}
consts = ""
for n, e in ENUMS.items():
    consts += f"const {CH[n]}: &[NormFieldChoice] = &[\n" + "".join(f'    choice("{v[:1].lower() + v[1:]}", "{e["en"][i]}", "{e["de"][i]}"),\n' for i, v in enumerate(e["variants"])) + "];\n"
s = s.replace("const TABLE: &[(&str, FieldSpec)] = &[", consts + "\nconst TABLE: &[(&str, FieldSpec)] = &[", 1)


def spec(f):
    if f["kind"] != "f64":
        return f'FieldSpec::choice("{f["en"]}", "{f["de"]}", {CH[f["kind"]]})'
    if f["unit"] in ("1",):
        return f'FieldSpec::new("{f["en"]}", "{f["de"]}")'
    return f'FieldSpec::quantity("{f["en"]}", "{f["de"]}", {UNIT[f["unit"]]})'


rows = ""
rows += '    ("walls[].openings[].positionM", ' + spec(OPENING_POS) + "),\n"
rows += '    ("walls[].loadCases[].concentrated[].distanceA1M", ' + spec(CONC_A1) + "),\n"
rows += '    ("walls[].loadCases[].concentrated[].positionM", ' + spec(CONC_POS) + "),\n"
for r in ROWS:
    rows += f'    ("walls[].{r["coll"]}", FieldSpec::new("{r["noun_en"]}s", "{r["noun_de"]}")),\n'
    rows += f'    ("walls[].{r["coll"]}[].id", FieldSpec::new("{r["noun_en"]} id", "{r["noun_de"]}-Id")),\n'
    for f in r["fields"]:
        rows += f'    ("walls[].{r["coll"]}[].{f["camel"]}", {spec(f)}),\n'
for x in SCALARS:
    rows += f'    ("walls[].{x["camel"]}", {spec(x)}),\n'
s = s.replace('    ("walls[].loadCases[].concentrated[].bearingLengthM", FieldSpec::quantity("Bearing length", "Auflagerlänge", UnitId::Metre)),\n', '    ("walls[].loadCases[].concentrated[].bearingLengthM", FieldSpec::quantity("Bearing length", "Auflagerlänge", UnitId::Metre)),\n' + rows, 1)
wr(p, s)
print("fieldmeta done")
