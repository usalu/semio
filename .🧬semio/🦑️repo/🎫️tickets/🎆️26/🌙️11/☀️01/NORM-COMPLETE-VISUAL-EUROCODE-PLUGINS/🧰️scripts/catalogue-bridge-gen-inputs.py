import json, re, sys, pathlib
HERE = pathlib.Path(__file__).resolve().parent
OUT = pathlib.Path(sys.argv[1])
SOURCE = pathlib.Path(sys.argv[2]) if len(sys.argv) > 2 else HERE / "catalogue-bridge-en1995-inputs.tsv"
ARTIFACT = sys.argv[3] if len(sys.argv) > 3 else "s.norm.en1995"
UNITS = {"Pa": "Pascal", "kPa": "Kilopascal", "kN": "Kilonewton", "Nm": "NewtonMetre", "kNm": "KilonewtonMetre", "N_m3": "NewtonPerCubicMetre", "m2": "SquareMetre", "m3": "CubicMetre", "m4": "MetreFourth", "m_s": "MetrePerSecond", "m3_s": "CubicMetrePerSecond", "s": "Second", "min": "Minute", "kg": "Kilogram", "kg_m3": "KilogramPerCubicMetre", "kg_m2": "KilogramPerSquareMetre", "N_m2": "Pascal", "mm_ts": "MillimetrePerSecond", "rad": "Radian", "Nmm_": "NewtonMillimetre", "m_s2": "MetrePerSecondSquared", "Hz": "Hertz", "h": "Hour", "t": "Tonne", "d": "Day", "MJ_m2": "MegajoulePerSquareMetre", "W": "Watt", "kg_m": "KilogramPerMetre", "km_h": "KilometrePerHour", "a": "Year", "C": "Celsius", "K": "Kelvin", "mm": "Millimetre", "m": "Metre", "N": "Newton", "MPa": "NewtonPerSquareMillimetre", "Nmm": "NewtonMillimetre", "mm2": "SquareMillimetre", "mm3": "CubicMillimetre", "mm4": "MillimetreFourth", "deg": "Degree", "1": "One", "kgm": "KilogramPerMetre", "kgm2": "KilogramPerSquareMetre", "kgm3": "KilogramPerCubicMetre", "N_m": "NewtonPerMetre", "N_mm": "NewtonPerMillimetre", "Hz": "Hertz", "min": "Minute", "per_m": "PerMetre", "pct": "Percent", "mm2_m": "SquareMillimetrePerMetre", "W_m2K": "WattPerSquareMetreKelvin", "J_m2sK": "JoulePerSquareMetreRootSecondKelvin"}
SYMBOLS = {"h": "h","t": "t", "d": "d", "MJ_m2": "MJ/m²", "W": "W", "kg_m": "kg/m","km_h": "km/h", "a": "a", "m_s2": "m/s²","Pa": "Pa", "kPa": "kPa", "kN": "kN", "Nm": "N·m", "kNm": "kN·m", "N_m3": "N/m³", "m2": "m²", "m3": "m³", "m4": "m⁴", "m_s": "m/s", "m3_s": "m³/s", "s": "s", "kg": "kg", "kg_m3": "kg/m³", "kg_m2": "kg/m²", "N_m2": "Pa", "mm_ts": "mm/s", "rad": "rad", "Nmm_": "N·mm", "m_s2": "m/s²", "C": "°C", "K": "K", "mm": "mm", "m": "m", "N": "N", "MPa": "N/mm²", "Nmm": "N·mm", "mm2": "mm²", "mm3": "mm³", "mm4": "mm⁴", "deg": "°", "1": "1", "kgm": "kg/m", "kgm2": "kg/m²", "kgm3": "kg/m³", "N_m": "N/m", "N_mm": "N/mm", "Hz": "Hz", "min": "min", "per_m": "1/m", "pct": "%", "mm2_m": "mm²/m", "W_m2K": "W/(m²·K)", "J_m2sK": "J/(m²·s½·K)"}
CLASSES = ["C14", "C16", "C18", "C20", "C22", "C24", "C27", "C30", "C35", "C40", "C45", "C50", "D18", "D24", "D30", "D35", "D40", "D50", "D60", "D70", "GL20h", "GL22h", "GL24h", "GL26h", "GL28h", "GL30h", "GL32h", "GL20c", "GL22c", "GL24c", "GL26c", "GL28c", "GL30c", "GL32c", "LVL32", "CLT100"]
def pairs(*items):
    return [{"value": v, "en": e, "de": d} for v, e, d in items]
SAME = lambda *v: [(x, x, x) for x in v]
CHOICES = {
    "class": pairs(*SAME(*CLASSES)), "class_2": pairs(*SAME(*CLASSES)),
    "duration": pairs(("permanent", "Permanent", "Ständig"), ("long", "Long-term", "Lang"), ("medium", "Medium-term", "Mittel"), ("short", "Short-term", "Kurz"), ("instantaneous", "Instantaneous", "Sehr kurz")),
    "situation": pairs(("persistent", "Persistent and transient", "Ständig und vorübergehend"), ("accidental", "Accidental", "Außergewöhnlich"), ("fire", "Fire", "Brand")),
    "product": pairs(("solid", "Solid timber", "Vollholz"), ("glulam", "Glulam", "Brettschichtholz"), ("lvl", "LVL", "Furnierschichtholz"), ("clt", "CLT", "Brettsperrholz")),
    "kind": pairs(("nail", "Nail", "Nagel"), ("nail_predrilled", "Predrilled nail", "Vorgebohrter Nagel"), ("nail_square", "Square nail", "Quadratnagel"), ("nail_other", "Other nail", "Sonstiger Nagel"), ("staple", "Staple", "Klammer"), ("screw", "Screw", "Schraube"), ("screw_axial", "Axially loaded screw", "Axial beanspruchte Schraube"), ("bolt", "Bolt", "Bolzen"), ("dowel", "Dowel", "Stabdübel")),
    "support": pairs(("discrete", "Discrete support", "Punktuelle Lagerung"), ("continuous", "Continuous support", "Durchlaufende Lagerung")),
    "members": pairs(("timber", "Timber", "Holz"), ("steel", "Steel", "Stahl")),
    "side_members": pairs(("timber", "Timber", "Holz"), ("steel", "Steel", "Stahl")),
    "panel": pairs(("timber", "Timber", "Holz"), ("plywood", "Plywood", "Sperrholz"), ("osb", "OSB", "OSB"), ("particleboard", "Particleboard", "Spanplatte"), ("fibreboard", "Fibreboard", "Faserplatte")),
    "panel_2": pairs(("timber", "Timber", "Holz"), ("plywood", "Plywood", "Sperrholz"), ("osb", "OSB", "OSB"), ("particleboard", "Particleboard", "Spanplatte"), ("fibreboard", "Fibreboard", "Faserplatte")),
    "wood": pairs(("softwood", "Softwood", "Nadelholz"), ("hardwood", "Hardwood", "Laubholz")),
    "load_case": pairs(("udl_ss", "UDL, simply supported", "Gleichlast, Einfeldträger"), ("udl", "Uniform load", "Gleichlast"), ("point_mid", "Point load at midspan", "Einzellast in Feldmitte"), ("cantilever_udl", "Cantilever, UDL", "Kragarm, Gleichlast"), ("cantilever_point", "Cantilever, point load", "Kragarm, Einzellast")),
    "load_pos": pairs(("centroid", "At the centroid", "Im Schwerpunkt"), ("top", "At the top edge", "Am oberen Rand"), ("bottom", "At the bottom edge", "Am unteren Rand")),
    "comfort": pairs(("normal", "Normal use", "Normale Nutzung"), ("crowd", "Crowd loading", "Menschenansammlung")),
    "crowd_model": pairs(("en1995-2", "EN 1995-2 pedestrian groups", "EN-1995-2-Fußgängergruppen"), ("setra", "Harmonic crowd (Sétra/HIVOSS)", "Harmonische Menschenmenge (Sétra/HIVOSS)")),
    "board": pairs(("F", "Type F board", "Platte Typ F"), ("A", "Type A board", "Platte Typ A")),
    "method": pairs(("A", "Method A", "Verfahren A"), ("B", "Method B", "Verfahren B")),
    "ductility": pairs(("DCL", "Low (DCL)", "Niedrig (DCL)"), ("DCM", "Medium (DCM)", "Mittel (DCM)"), ("DCH", "High (DCH)", "Hoch (DCH)")),
    "shape": pairs(("rect", "Rectangular", "Rechteckig"), ("circle", "Circular", "Kreisförmig"), ("double_tapered", "Double tapered", "Satteldach"), ("pitched_cambered", "Pitched cambered", "Gekrümmter Satteldachträger")),
    "joint": pairs(("nailed", "Nailed", "Genagelt"), ("bolted", "Bolted", "Gebolzt"), ("glued", "Glued", "Geklebt"), ("connectors", "Connectors", "Verbinder")),
    "side": pairs(("support", "Support side", "Auflagerseite"), ("tension", "Tension side", "Zugseite"), ("compression", "Compression side", "Druckseite")),
    "flange_side": pairs(("compression", "Compression flange", "Druckgurt"), ("tension", "Tension flange", "Zuggurt")),
}
if ARTIFACT != "s.norm.en1995":
    CHOICES = {}
rows = []
ALIASES = {}
for line in SOURCE.read_text(encoding="utf-8").splitlines():
    if line.startswith("@alias "):
        key, source = line[len("@alias "):].split("|")
        ALIASES[key] = source
        continue
    if line.startswith("@choice "):
        key, value, en, de = line[len("@choice "):].split("|")
        CHOICES.setdefault(key, []).append({"value": value, "en": en, "de": de})
        continue
for line in SOURCE.read_text(encoding="utf-8").splitlines():
    if not line or line.startswith("#") or line.startswith("@"):
        continue
    f = line.split("|")
    f += [""] * (10 - len(f))
    key, scope, kind, unit, example, flags, en, de, confidence, note = f[:10]
    row = {"key": key, "scope": scope, "kind": {"n": "number", "v": "vector", "s": "series", "c": "choice", "x": "text"}[kind], "en": en, "de": de}
    if confidence:
        row["confidence"] = confidence
    if note:
        row["note"] = note
    if kind in ("n", "v", "s"):
        assert unit in UNITS, (key, unit)
        row["unit"] = SYMBOLS[unit]
        row["unitId"] = UNITS[unit]
        for flag in flags.split(","):
            if flag == "L":
                row["lever"] = True
            elif flag.startswith("r"):
                lo, hi = flag[1:].split(":")
                row["min"], row["max"] = float(lo), float(hi)
        if example:
            row["example"] = float(example)
    elif kind == "c":
        scoped = f"{key}@{scope}"
        name = scoped if scoped in CHOICES or scoped in ALIASES else key
        name = ALIASES.get(name, name)
        assert name in CHOICES, name
        row["choices"] = CHOICES[name]
        if example:
            row["example"] = example
    rows.append(row)
seen = set()
for r in rows:
    k = (r["key"], r["scope"]); assert k not in seen, k; seen.add(k)
OUT.mkdir(parents=True, exist_ok=True)
doc = {"$schema": "https://json.schemas.assets.semio-tech.com/s/norm/compliance/design-case/schema.json", "artifact": ARTIFACT, "inputs": rows}
(OUT / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
def q(s): return json.dumps(s, ensure_ascii=False)
def num(x): return repr(float(x))
out = ["//! 📚️ Typed input vocabulary of the catalogue: unit, bilingual label and choices of every case input key; generated from `./🔣️.json` by `🧰️scripts/catalogue-bridge-gen-inputs.py`. See [`super`].", "", "use semio_s_plugin_norm_compliance::design_case::{InputChoice, InputSpec};", "use semio_s_plugin_norm_compliance::UnitId;", "", "/// 📚️ Every input key the evaluators read; rows with a `scope` apply to the listed catalogue ids only.", "pub static VOCABULARY: &[InputSpec] = &["]
for r in rows:
    if r["kind"] in ("number", "vector", "series"):
        s = f'InputSpec::{r["kind"]}({q(r["key"])}, {q(r["en"])}, {q(r["de"])}, UnitId::{r["unitId"]})'
        if "min" in r: s += f'.range({num(r["min"])}, {num(r["max"])})'
        if "example" in r: s += f'.example({num(r["example"])})'
        if r.get("lever"): s += ".lever()"
    elif r["kind"] == "choice":
        cs = ", ".join(f'InputChoice::new({q(c["value"])}, {q(c["en"])}, {q(c["de"])})' for c in r["choices"])
        s = f'InputSpec::choice({q(r["key"])}, {q(r["en"])}, {q(r["de"])}, &[{cs}])'
        if "example" in r: s += f'.example_text({q(r["example"])})'
    else:
        s = f'InputSpec::text({q(r["key"])}, {q(r["en"])}, {q(r["de"])})'
    if r["scope"]: s += f'.scope({q(r["scope"])})'
    out.append(f"    {s},")
out.append("];")
(OUT / "🦀️.rs").write_text("\n".join(out) + "\n", encoding="utf-8")
print(len(rows), "rows")
