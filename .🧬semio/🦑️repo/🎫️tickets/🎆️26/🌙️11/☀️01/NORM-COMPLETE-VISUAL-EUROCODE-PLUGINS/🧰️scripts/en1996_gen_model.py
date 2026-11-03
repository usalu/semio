"""Applies the EN 1996 wave 2 entities to the snapshot model (rust), its twins (ts, json, proto, graphql) and the diff (rust, ts, json). Idempotent via markers."""
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
exec(compile(open(os.path.join(HERE, "en1996_spec.py"), encoding="utf-8").read(), "en1996_spec.py", "exec"))

DERIVE_REC = "#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]\n#[canonical_json(owner = semio_framework_pack_json)]\n#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]\n#[cfg_attr(test, serde(rename_all = \"camelCase\"))]\n#[value(rename_all = \"camelCase\")]\n"
DERIVE_ENUM = "#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]\n#[canonical_json(owner = semio_framework_pack_json)]\n#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]\n"
RUST_UNIT = {"m": "m", "N": "N", "m2": "m2", "Pa": "Pa"}
MARK = "//#region 🏛️Wave2Entities"


def rust_field(f, pub=True):
    attr = f'    #[dsl(unit = "{RUST_UNIT[f["unit"]]}")]\n' if f["kind"] == "f64" and f["unit"] in RUST_UNIT else ""
    ty = "f64" if f["kind"] == "f64" else f["kind"]
    return f"{attr}    pub {f['name']}: {ty},\n"


def model_blocks():
    out = MARK + "\n"
    for name, e in ENUMS.items():
        out += f"/// {e['doc']}\n{DERIVE_ENUM}pub enum {name} {{\n" + "".join(f"    {v},\n" for v in e["variants"]) + "}\n\n"
    for r in ROWS:
        out += f"/// {r['doc']}\n{DERIVE_REC}pub struct {r['struct']} {{\n    pub id: String,\n" + "".join(rust_field(f) for f in r["fields"]) + "}\n\n"
    out += "//#endregion 🏛️Wave2Entities\n\n"
    return out


WALL_FIELDS = "".join(f"    #[dsl(table)]\n    pub {r['coll']}: Vec<{r['struct']}>,\n" for r in ROWS) + "".join(rust_field(s) for s in SCALARS)


def wall_defaults(indent):
    parts = [f"{r['coll']}: vec![]," for r in ROWS]
    for s in SCALARS:
        v = s["default"]
        parts.append(f"{s['name']}: " + (f"FrostClass::{v}," if s["kind"] == "FrostClass" else f"{float(v)!r},"))
    return "".join(f"{indent}{p}\n" for p in parts)


def patch_model():
    p = f"{SNAP}/🦀️.rs"
    s = rd(p)
    if MARK in s:
        return
    s = s.replace("/// 🪟 Opening in a masonry wall — SI base units.", model_blocks() + "/// 🪟 Opening in a masonry wall — SI base units.", 1)
    s = s.replace("    #[dsl(unit = \"m\")]\n    pub sill_height_m: f64,\n}", "    #[dsl(unit = \"m\")]\n    pub sill_height_m: f64,\n    #[dsl(unit = \"m\")]\n    pub position_m: f64,\n}", 1)
    s = s.replace("    #[dsl(unit = \"m\")]\n    pub bearing_length_m: f64,\n}", "    #[dsl(unit = \"m\")]\n    pub bearing_length_m: f64,\n    /// 📏️ Distance a1 from the wall end to the nearest edge of the bearing area [m] (EN 1996-1-1 §6.1.3).\n    #[dsl(unit = \"m\")]\n    pub distance_a1_m: f64,\n    #[dsl(unit = \"m\")]\n    pub position_m: f64,\n}", 1)
    s = s.replace("    #[dsl(table)]\n    pub load_cases: Vec<WallLoadCase>,\n}", "    #[dsl(table)]\n    pub load_cases: Vec<WallLoadCase>,\n" + WALL_FIELDS + "}", 1)
    s = s.replace("                is_basement: false,\n", "                is_basement: false,\n" + wall_defaults("                "))
    s = s.replace("                        bearing_length_m: 0.10,\n", "                        bearing_length_m: 0.10,\n                        distance_a1_m: 0.0,\n                        position_m: 1.0,\n")
    s = s.replace("            bearing_length_m: 0.20,\n", "            bearing_length_m: 0.20,\n            distance_a1_m: 0.5,\n            position_m: 2.5,\n")
    s = s.replace("                sill_height_m: 0.90,\n", "                sill_height_m: 0.90,\n                position_m: 0.80,\n")
    s = s.replace("                sill_height_m: 0.0,\n", "                sill_height_m: 0.0,\n                position_m: 3.40,\n")
    wall_example = '''
    /// 🧭 Scope example: wall with openings, lintels, piers, ties, tendon, confining column, flange wall and bars (EN 1996-1-1 §5.5, §6.6, §6.7, §8.2, §8.5, §8.7).
    pub fn members_wall_example() -> Self {
        let mut doc = Self::opening_wall_example();
        let wall = &mut doc.walls[0];
        wall.id = "wall-members".into();
        wall.label_en = "Wall with lintels, piers, ties, tendon and confining column".into();
        wall.label_de = "Wand mit Stürzen, Pfeilern, Ankern, Spannglied und Einfassungsstütze".into();
        wall.lintels = vec![
            WallLintel { id: "lintel-1".into(), position_m: 0.60, span_m: 1.40, depth_m: 0.25, bearing_length_m: 0.25 },
            WallLintel { id: "lintel-2".into(), position_m: 3.20, span_m: 1.20, depth_m: 0.25, bearing_length_m: 0.25 },
        ];
        wall.piers = vec![WallPier { id: "pier-1".into(), position_m: 2.0, width_m: 1.4 }];
        wall.ties = vec![
            WallTie { id: "tie-1".into(), kind: TieKind::WallTie, position_m: 1.0, level_m: 1.0, capacity_n: 1_500.0 },
            WallTie { id: "anchor-1".into(), kind: TieKind::Anchor, position_m: 2.5, level_m: 2.75, capacity_n: 12_000.0 },
            WallTie { id: "ring-1".into(), kind: TieKind::RingTie, position_m: 2.5, level_m: 2.75, capacity_n: 30_000.0 },
        ];
        wall.prestress = vec![WallPrestress { id: "tendon-1".into(), force_n: 90_000.0, area_m2: 2.0e-4, f_pk_pa: 1.77e9, position_m: 2.5, eccentricity_m: 0.05, loss_ratio: 0.2 }];
        wall.confined = vec![ConfinedElement { id: "column-1".into(), position_m: 4.9, width_m: 0.25, depth_m: 0.24, as_m2: 6.0e-4 }];
        wall.flanges = vec![ShearFlange { id: "flange-1".into(), length_m: 1.2, thickness_m: 0.24, position_m: 0.0 }];
        wall.bars = vec![
            WallBar { id: "bar-shear".into(), kind: BarKind::Shear, diameter_m: 0.008, length_m: 0.5, spacing_m: 0.4 },
            WallBar { id: "bar-anchor".into(), kind: BarKind::Anchorage, diameter_m: 0.012, length_m: 0.6, spacing_m: 0.4 },
        ];
        wall.frost_class = FrostClass::Moderate;
        wall.cover_m = 0.035;
        wall.soil_height_m = 0.0;
        wall.isotherm_depth_m = 0.04;
        wall.plan_x_m = 1.0;
        wall.plan_y_m = 2.0;
        wall.in_plane_shear_n = 15_000.0;
        wall.in_plane_moment_nm = 20_000.0;
        doc
    }
'''
    s = s.replace("    /// 🏗️ Scope example: basement wall", wall_example.lstrip("\n") + "\n    /// 🏗️ Scope example: basement wall", 1)
    wr(p, s)


def wire_fields(r_or_none):
    pass


def ts_ty(f):
    return "number" if f["kind"] == "f64" else f"En1996{f['kind']}"


def ts_parse(f):
    return "normWireNumber" if f["kind"] == "f64" else f"normWireRef(() => parseEn1996{f['kind']})"


def patch_snapshot_ts():
    p = f"{SNAP}/🟦️.ts"
    s = rd(p)
    if "interface WallPier" in s:
        return
    s = re.sub(r"  walls: \(\{.*?\}\)\[\];", "  walls: MasonryWall[];", s)
    s = re.sub(r"walls: normWireRequired\(normWireArray\(normWireObject<\{.*?\}>\(\{.*?\}\)\)\) \}\);\n", "walls: normWireRequired(normWireArray(normWireRef(() => parseMasonryWall))) });\n", s, flags=re.S)
    s = s.replace("  bearingLengthM: number;\n}", "  bearingLengthM: number;\n  distanceA1M: number;\n  positionM: number;\n}", 1)
    s = s.replace("  sillHeightM: number;\n}", "  sillHeightM: number;\n  positionM: number;\n}", 1)
    s = s.replace("  loadCases: WallLoadCase[];\n}", "  loadCases: WallLoadCase[];\n" + "".join(f"  {r['coll']}: {r['struct']}[];\n" for r in ROWS) + "".join(f"  {x['camel']}: {ts_ty(x)};\n" for x in SCALARS) + "}", 1)
    ifaces = ""
    parsers = ""
    for r in ROWS:
        ifaces += f"export interface {r['struct']} {{\n  id: string;\n" + "".join(f"  {f['camel']}: {ts_ty(f)};\n" for f in r["fields"]) + "}\n\n"
        parsers += f"export const parse{r['struct']}: NormWireReader<{r['struct']}> = normWireObject<{r['struct']}>({{ id: normWireRequired(normWireString), " + ", ".join(f"{f['camel']}: normWireRequired({ts_parse(f)})" for f in r["fields"]) + " });\n"
    s = s.replace("export interface MasonryWall {", ifaces + "export interface MasonryWall {", 1)
    types = "".join(f"export type En1996{n} = " + " | ".join(f'"{v}"' for v in e["variants"]) + ";\n\n" for n, e in ENUMS.items())
    s = s.replace("export type En1996AnnexChoice", types + "export type En1996AnnexChoice", 1)
    lines = s.split("\n")
    for i, l in enumerate(lines):
        if l.startswith("export const parseWallOpening"):
            lines[i] = l.replace("sillHeightM: normWireRequired(normWireNumber) }", "sillHeightM: normWireRequired(normWireNumber), positionM: normWireRequired(normWireNumber) }")
        elif l.startswith("export const parseConcentratedLoad"):
            lines[i] = l.replace("bearingLengthM: normWireRequired(normWireNumber) }", "bearingLengthM: normWireRequired(normWireNumber), distanceA1M: normWireRequired(normWireNumber), positionM: normWireRequired(normWireNumber) }")
        elif l.startswith("export const parseWallLoadCase"):
            pass
        elif l.startswith("export const parseMasonryWall"):
            add = "".join(f", {r['coll']}: normWireRequired(normWireArray(normWireRef(() => parse{r['struct']})))" for r in ROWS) + "".join(f", {x['camel']}: normWireRequired({ts_parse(x)})" for x in SCALARS)
            lines[i] = l.replace("loadCases: normWireRequired(normWireArray(normWireRef(() => parseWallLoadCase))) }", "loadCases: normWireRequired(normWireArray(normWireRef(() => parseWallLoadCase)))" + add + " }")
        elif l.startswith("export const parseWallOpening") is False and l.startswith("export const parseEn1996AnnexChoice"):
            lines[i] = parsers + "".join(f"export const parseEn1996{n}: NormWireReader<En1996{n}> = normWireLiteral(" + ", ".join(f'"{v}"' for v in e["variants"]) + ");\n" for n, e in ENUMS.items()) + l
    wr(p, "\n".join(lines))


def ui(f):
    u = {}
    if f["kind"] == "f64":
        u = {"widget": "stepper", "label": {"en": f["en"], "de": f["de"]}, "unit": f["unit"]}
        if f.get("disp"):
            u["displayUnit"], u["displayFactor"] = f["disp"], f["factor"]
        u["step"], u["precision"] = f["step"], f["prec"]
    else:
        u = {"label": {"en": f["en"], "de": f["de"]}}
    return u


def prop(f):
    if f["kind"] == "f64":
        d = {"type": "number"}
    else:
        d = {"$ref": f"#/$defs/{f['kind']}"}
    d["x-semio-ui"] = ui(f)
    return d


def patch_snapshot_json():
    p = f"{SNAP}/🔣️.json"
    d = jload(p)
    defs = d["$defs"]
    if "WallPier" in defs:
        return
    d["properties"]["walls"]["items"] = {"$ref": "#/$defs/MasonryWall"}
    pos = F("position_m", "Opening position x", "Öffnungslage x", sample=2.0)
    defs["WallOpening"]["properties"]["positionM"] = prop(pos)
    defs["WallOpening"]["required"].append("positionM")
    defs["ConcentratedLoad"]["properties"]["distanceA1M"] = prop(CONC_A1)
    defs["ConcentratedLoad"]["properties"]["positionM"] = prop(CONC_POS)
    defs["ConcentratedLoad"]["required"] += ["distanceA1M", "positionM"]
    for n, e in ENUMS.items():
        defs[n] = {"title": n, "type": "string", "enum": e["variants"], "x-semio-ui": {"options": {v: {"en": e["en"][i], "de": e["de"][i]} for i, v in enumerate(e["variants"])}}, "x-semio-formats": ["🔣️jsonschema"]}
    for r in ROWS:
        props = {"id": {"type": "string"}}
        props.update({f["camel"]: prop(f) for f in r["fields"]})
        defs[r["struct"]] = {"title": r["struct"], "type": "object", "additionalProperties": False, "required": ["id"] + [f["camel"] for f in r["fields"]], "properties": props, "x-semio-formats": ["🔣️jsonschema", "🟦️typescript"]}
        defs["MasonryWall"]["properties"][r["coll"]] = {"type": "array", "items": {"$ref": f"#/$defs/{r['struct']}"}, "x-semio-ui": {"label": {"en": r["noun_en"] + "s", "de": r["noun_de"]}}}
        defs["MasonryWall"]["required"].append(r["coll"])
    for s in SCALARS:
        defs["MasonryWall"]["properties"][s["camel"]] = prop(s)
        defs["MasonryWall"]["required"].append(s["camel"])
    jdump(p, d)


def patch_snapshot_proto_graphql():
    p = f"{SNAP}/🛰️.proto"
    s = rd(p)
    if "message WallPier" in s:
        return
    msgs = ""
    for r in ROWS:
        msgs += f"message {r['struct']} {{\n  string id = 1;\n" + "".join(f"  {'double' if f['kind'] == 'f64' else 'string'} {f['name']} = {i};\n" for i, f in enumerate(r["fields"], 2)) + "}\n"
    msgs += "message WallOpening {\n  string id = 1;\n  double width_m = 2;\n  double height_m = 3;\n  double sill_height_m = 4;\n  double position_m = 5;\n}\n"
    msgs += "message ConcentratedLoad {\n  string id = 1;\n  double force_n = 2;\n  double bearing_area_m2 = 3;\n  double bearing_length_m = 4;\n  double distance_a1_m = 5;\n  double position_m = 6;\n}\n"
    wall_add = "  double density_kg_m3 = 29;\n  double phi_infinity = 30;\n  bool is_basement = 31;\n"
    n = 32
    for r in ROWS:
        wall_add += f"  repeated {r['struct']} {r['coll']} = {n};\n"
        n += 1
    for x in SCALARS:
        wall_add += f"  {'double' if x['kind'] == 'f64' else 'string'} {x['name']} = {n};\n"
        n += 1
    s = s.replace("  double mu = 28;\n}", "  double mu = 28;\n" + wall_add + "}", 1)
    s = s.replace("message En1996Wall {", msgs + "message En1996Wall {", 1)
    wr(p, s)
    p = f"{SNAP}/🔗️.graphql"
    s = rd(p)
    add = "  densityKgM3: Float!\n  phiInfinity: Float!\n  isBasement: Boolean!\n" + "".join(f"  {r['coll']}: [JSON!]!\n" for r in ROWS) + "".join(f"  {x['camel']}: {'Float' if x['kind'] == 'f64' else 'String'}!\n" for x in SCALARS)
    s = s.replace("  mu: Float!\n  loadCases: [JSON!]!\n}", "  mu: Float!\n  loadCases: [JSON!]!\n" + add + "}", 1)
    wr(p, s)


def patch_snapshot_all():
    patch_model()
    patch_snapshot_ts()
    patch_snapshot_json()
    patch_snapshot_proto_graphql()


patch_snapshot_all()
print("snapshot done")
