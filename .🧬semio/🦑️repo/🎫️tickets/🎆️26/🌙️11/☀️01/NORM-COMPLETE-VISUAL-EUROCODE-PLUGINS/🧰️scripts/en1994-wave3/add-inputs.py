import json, pathlib, sys
ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📇️catalogue/📚️inputs")
N = lambda key, en, de, symbol, unit, lo, hi, example, confidence="high", note="", lever=False: dict(key=key, scope="", kind="number", en=en, de=de, confidence=confidence, **({"note": note} if note else {}), unit=symbol, unitId=unit, min=lo, max=hi, example=example, **({"lever": True} if lever else {}))
ROWS = [
    N("psi_vib", "Share of the variable load acting as vibrating mass ψ", "Anteil der veränderlichen Last als schwingende Masse ψ", "1", "One", 0.0, 1.0, 0.3, "low", "Value fixed by the National Annex or the project; the example is a placeholder."),
    N("f_min", "Required lowest natural frequency fmin", "Geforderte tiefste Eigenfrequenz fmin", "Hz", "Hertz", 0.5, 20.0, 3.0, "low", "Limit fixed by the National Annex of EN 1990 A1.4.4 or the project; the example is a placeholder.", True),
    N("l_ltb", "Length between lateral restraints of the bottom flange L", "Länge zwischen seitlichen Halterungen des Untergurts L", "m", "Metre", 0.5, 30.0, 6.0),
    N("i2_slab", "Cracked second moment of the slab per metre I2", "Gerissenes Flächenmoment 2. Grades der Platte je Meter I2", "m⁴/m", "MetreFourth", 1e-09, 0.01, 4e-05, "medium", "Unit m⁴ per metre of the beam length."),
    N("a_beam", "Beam spacing a", "Trägerabstand a", "m", "Metre", 0.5, 20.0, 3.0),
    N("k_alpha", "Frame factor α (2 edge beam, 3 interior beam)", "Rahmenbeiwert α (2 Randträger, 3 Innenträger)", "1", "One", 1.0, 4.0, 2.0),
    dict(key="steel_kind", scope="", kind="choice", en="Production of the steel section", de="Herstellung des Stahlprofils", confidence="high", choices=[dict(value="rolled", en="Rolled section", de="Walzprofil"), dict(value="welded", en="Welded section", de="Geschweißtes Profil")], example="rolled"),
    N("phi_t", "Creep coefficient φ(t,t0)", "Kriechzahl φ(t,t0)", "1", "One", 0.0, 6.0, 2.5, "low", "Value of the project or EN 1992-1-1 3.1.4; the example is a placeholder."),
    N("psi_l", "Creep multiplier ψL of the permanent load", "Kriechbeiwert ψL der ständigen Last", "1", "One", 0.4, 2.0, 1.1, "medium", "EN 1994-1-1 5.4.2.2 (1.1 permanent loads, 0.55 shrinkage)."),
    N("eps_cs", "Free shrinkage strain εcs", "Freie Schwinddehnung εcs", "1", "One", 0.0, 0.002, 0.0003, "low", "Value of the project or EN 1992-1-1 3.1.4; the example is a placeholder."),
    N("am_v", "Section factor of the steel Ap/V (1/m)", "Profilfaktor des Stahls Ap/V (1/m)", "1", "One", 10.0, 500.0, 150.0),
    N("d_p", "Thickness of the fire protection dp", "Dicke der Brandschutzbekleidung dp", "m", "Metre", 0.001, 0.1, 0.02),
    N("lambda_p", "Thermal conductivity of the protection λp", "Wärmeleitfähigkeit der Bekleidung λp", "W/(m·K)", "WattPerMetreKelvin", 0.01, 1.0, 0.12, "low", "Material data of the protection product; the example is a placeholder."),
    N("c_p", "Specific heat of the protection cp (J/(kg·K))", "Spezifische Wärme der Bekleidung cp (J/(kg·K))", "1", "One", 100.0, 5000.0, 1000.0, "low", "Material data of the protection product; the example is a placeholder."),
    N("rho_p", "Density of the protection ρp", "Rohdichte der Bekleidung ρp", "kg/m³", "KilogramPerCubicMetre", 50.0, 3000.0, 800.0, "low", "Material data of the protection product; the example is a placeholder."),
    N("t_fire", "Fire duration t", "Branddauer t", "s", "Second", 60.0, 14400.0, 3600.0),
    N("mu0", "Degree of utilisation μ0 in fire", "Ausnutzungsgrad μ0 im Brandfall", "1", "One", 0.013, 0.99, 0.5),
]
def q(s): return json.dumps(s, ensure_ascii=False)
def num(x): return repr(float(x))
doc = json.loads((ROOT / "🔣️.json").read_text(encoding="utf-8"))
have = {r["key"] for r in doc["inputs"]}
fresh = [r for r in ROWS if r["key"] not in have]
doc["inputs"] += fresh
(ROOT / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
lines = []
for r in fresh:
    if r["kind"] == "number":
        s = f'InputSpec::number({q(r["key"])}, {q(r["en"])}, {q(r["de"])}, UnitId::{r["unitId"]}).range({num(r["min"])}, {num(r["max"])}).example({num(r["example"])})'
        if r.get("lever"): s += ".lever()"
    else:
        cs = ", ".join(f'InputChoice::new({q(c["value"])}, {q(c["en"])}, {q(c["de"])})' for c in r["choices"])
        s = f'InputSpec::choice({q(r["key"])}, {q(r["en"])}, {q(r["de"])}, &[{cs}]).example_text({q(r["example"])})'
    lines.append(f"    {s},")
rs = (ROOT / "🦀️.rs").read_text(encoding="utf-8")
assert rs.rstrip().endswith("];")
rs = rs.rstrip()[:-2] + "\n".join(lines) + "\n];\n"
(ROOT / "🦀️.rs").write_text(rs, encoding="utf-8")
print(len(fresh), "rows added")
