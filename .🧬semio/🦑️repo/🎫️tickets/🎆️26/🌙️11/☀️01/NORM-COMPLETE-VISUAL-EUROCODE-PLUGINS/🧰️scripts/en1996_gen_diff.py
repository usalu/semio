"""Applies the EN 1996 wave 2 entities to the diff (rust macros, ts, json). Idempotent."""
import os, re
HERE = os.path.dirname(os.path.abspath(__file__))
exec(compile(open(os.path.join(HERE, "en1996_spec.py"), encoding="utf-8").read(), "en1996_spec.py", "exec"))
M = "semio_s_artifact_norm_en1996_model"
CFG = "    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]\n    #[cfg_attr(test, serde(rename_all = \"camelCase\"))]\n"


def rty(f):
    return "f64" if f["kind"] == "f64" else f"{M}::{f['kind']}"


def patch_rs():
    p = f"{DIFF}/🦀️.rs"
    s = rd(p)
    if "En1996PierPatch" in s:
        return
    s = s.replace("{ set { width_m: f64, height_m: f64, sill_height_m: f64 } }", "{ set { width_m: f64, height_m: f64, sill_height_m: f64, position_m: f64 } }", 1)
    s = s.replace("{ set { force_n: f64, bearing_area_m2: f64, bearing_length_m: f64 } }", "{ set { force_n: f64, bearing_area_m2: f64, bearing_length_m: f64, distance_a1_m: f64, position_m: f64 } }", 1)
    blocks = ""
    for r in ROWS:
        pre, st = r["pre"], r["struct"]
        sets = ", ".join(f"{f['name']}: {rty(f)}" for f in r["fields"])
        blocks += f"""protocol::row_patch! {{
{CFG}    /// 🩹 Sparse field patch of one `{st}`.
    pub En1996{pre}Patch of {M}::{st} {{ set {{ {sets} }} }}
}}

/// 🔑️ Key extractor of the foreign rows of `En1996{pre}Delta` (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1996{pre}DeltaKey;

impl protocol::list_delta::KeyOf<{M}::{st}> for En1996{pre}DeltaKey {{
    type Key = String;
    fn key_of(row: &{M}::{st}) -> String {{
        row.id.clone()
    }}
}}

protocol::list_delta! {{
{CFG}    /// 📋️ Positional row delta of one `{st}` list.
    pub En1996{pre}Delta {{ removal: En1996{pre}Removal, insertion: En1996{pre}Insertion, relocation: En1996{pre}Relocation, modification: En1996{pre}Modification, row: {M}::{st}, patch: En1996{pre}Patch, list: Vec<{M}::{st}>, key: String = by En1996{pre}DeltaKey }}
}}

"""
    s = s.replace("//#endregion 🔖️Rows", blocks + "//#endregion 🔖️Rows", 1)
    sets = ", ".join(f"{x['name']}: {rty(x)}" for x in SCALARS)
    nest = ", ".join(f"{r['coll']}: En1996{r['pre']}Delta" for r in ROWS)
    s = s.replace("is_basement: bool } nest { openings: En1996OpeningDelta, load_cases: En1996LoadCaseDelta } }", f"is_basement: bool, {sets} }} nest {{ openings: En1996OpeningDelta, load_cases: En1996LoadCaseDelta, {nest} }} }}", 1)
    assert "En1996PierDelta, " in s or "piers: En1996PierDelta" in s
    wr(p, s)


def ts_t(f):
    return "number" if f["kind"] == "f64" else f"En1996{f['kind']}"


def ts_p(f):
    return "normWireNumber" if f["kind"] == "f64" else f"parseEn1996{f['kind']}"


def patch_ts():
    p = f"{DIFF}/🟦️.ts"
    s = rd(p)
    if "En1996PierPatch" in s:
        return
    s = s.replace("  sillHeightM: number | null;\n}", "  sillHeightM: number | null;\n  positionM: number | null;\n}", 1)
    s = s.replace("  bearingLengthM: number | null;\n}", "  bearingLengthM: number | null;\n  distanceA1M: number | null;\n  positionM: number | null;\n}", 1)
    s = s.replace("{ widthM: normWireDefault(normWireNullable(normWireNumber), () => null), heightM: normWireDefault(normWireNullable(normWireNumber), () => null), sillHeightM: normWireDefault(normWireNullable(normWireNumber), () => null) }", "{ widthM: normWireDefault(normWireNullable(normWireNumber), () => null), heightM: normWireDefault(normWireNullable(normWireNumber), () => null), sillHeightM: normWireDefault(normWireNullable(normWireNumber), () => null), positionM: normWireDefault(normWireNullable(normWireNumber), () => null) }", 1)
    s = s.replace("bearingLengthM: normWireDefault(normWireNullable(normWireNumber), () => null) }", "bearingLengthM: normWireDefault(normWireNullable(normWireNumber), () => null), distanceA1M: normWireDefault(normWireNullable(normWireNumber), () => null), positionM: normWireDefault(normWireNullable(normWireNumber), () => null) }", 1)
    ifaces, parsers, snap_t, snap_p = "", "", set(), set()
    for r in ROWS:
        pre, st = r["pre"], r["struct"]
        ifaces += f"export interface En1996{pre}Patch {{\n" + "".join(f"  {f['camel']}: {ts_t(f)} | null;\n" for f in r["fields"]) + "}\n\n"
        ifaces += f"export interface En1996{pre}Removal {{\n  id: string;\n  index: number;\n}}\n\nexport interface En1996{pre}Insertion {{\n  index: number;\n  row: {st};\n}}\n\nexport interface En1996{pre}Relocation {{\n  id: string;\n  from: number;\n  to: number;\n}}\n\nexport interface En1996{pre}Modification {{\n  id: string;\n  patch: En1996{pre}Patch;\n}}\n\nexport interface En1996{pre}Delta {{\n  removed: En1996{pre}Removal[];\n  inserted: En1996{pre}Insertion[];\n  moved: En1996{pre}Relocation[];\n  modified: En1996{pre}Modification[];\n}}\n\n"
        parsers += f"export const parseEn1996{pre}Patch: NormWireReader<En1996{pre}Patch> = normWireObject<En1996{pre}Patch>({{ " + ", ".join(f"{f['camel']}: normWireDefault(normWireNullable({ts_p(f)}), () => null)" for f in r["fields"]) + " });\n"
        parsers += f'export const parseEn1996{pre}Removal: NormWireReader<En1996{pre}Removal> = normWireObject<En1996{pre}Removal>({{ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {{"minimum":0}})) }});\n'
        parsers += f'export const parseEn1996{pre}Insertion: NormWireReader<En1996{pre}Insertion> = normWireObject<En1996{pre}Insertion>({{ index: normWireRequired(normWireRange(normWireInteger, {{"minimum":0}})), row: normWireRequired(parse{st}) }});\n'
        parsers += f'export const parseEn1996{pre}Relocation: NormWireReader<En1996{pre}Relocation> = normWireObject<En1996{pre}Relocation>({{ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {{"minimum":0}})), to: normWireRequired(normWireRange(normWireInteger, {{"minimum":0}})) }});\n'
        parsers += f"export const parseEn1996{pre}Modification: NormWireReader<En1996{pre}Modification> = normWireObject<En1996{pre}Modification>({{ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1996{pre}Patch)) }});\n"
        parsers += f"export const parseEn1996{pre}Delta: NormWireReader<En1996{pre}Delta> = normWireObject<En1996{pre}Delta>({{ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1996{pre}Removal))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1996{pre}Insertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1996{pre}Relocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1996{pre}Modification))) }});\n"
        snap_t.add(st)
        snap_p.add("parse" + st)
        for f in r["fields"]:
            if f["kind"] != "f64":
                snap_t.add(f"En1996{f['kind']}")
                snap_p.add(f"parseEn1996{f['kind']}")
    for x in SCALARS:
        if x["kind"] != "f64":
            snap_t.add(f"En1996{x['kind']}")
            snap_p.add(f"parseEn1996{x['kind']}")
    s = s.replace("export interface En1996WallPatch {", ifaces + "export interface En1996WallPatch {", 1)
    s = s.replace("  isBasement: boolean | null;\n  openings?: En1996OpeningDelta;\n  loadCases?: En1996LoadCaseDelta;\n}", "  isBasement: boolean | null;\n" + "".join(f"  {x['camel']}: {ts_t(x)} | null;\n" for x in SCALARS) + "  openings?: En1996OpeningDelta;\n  loadCases?: En1996LoadCaseDelta;\n" + "".join(f"  {r['coll']}?: En1996{r['pre']}Delta;\n" for r in ROWS) + "}", 1)
    lines = s.split("\n")
    for i, l in enumerate(lines):
        if l.startswith("export const parseEn1996WallPatch:"):
            l = l.replace("isBasement: normWireDefault(normWireNullable(normWireBoolean), () => null), ", "isBasement: normWireDefault(normWireNullable(normWireBoolean), () => null), " + "".join(f"{x['camel']}: normWireDefault(normWireNullable({ts_p(x)}), () => null), " for x in SCALARS), 1)
            l = l.replace("loadCases: normWireOptional(normWireRef(() => parseEn1996LoadCaseDelta)) }", "loadCases: normWireOptional(normWireRef(() => parseEn1996LoadCaseDelta))" + "".join(f", {r['coll']}: normWireOptional(normWireRef(() => parseEn1996{r['pre']}Delta))" for r in ROWS) + " }", 1)
            lines[i] = parsers + l
    s = "\n".join(lines)
    m = re.search(r"^import \{ type ConcentratedLoad,[^\n]*\} from \"\.\./📸️snapshot/🟦️\.ts\";", s, re.M)
    old = m.group(0)
    names_t = set(re.findall(r"type (\w+)", old))
    names_p = set(re.findall(r"(?<!type )\b(parse\w+)", old.split("{", 1)[1].split("}")[0]))
    names_t |= snap_t
    names_p |= snap_p
    items = sorted([n for n in names_t] + [n for n in names_p], key=lambda x: x.lower())
    new = "import { " + ", ".join(f"type {n}" if n in names_t else n for n in items) + ' } from "../📸️snapshot/🟦️.ts";'
    s = s.replace(old, new, 1)
    wr(p, s)


def patch_json():
    p = f"{DIFF}/🔣️.json"
    d = jload(p)
    defs = d["$defs"]
    if "En1996PierPatch" in defs:
        return
    sn = lambda path: {"anyOf": [{"$ref": f"{SNAPSHOT_REF}#/$defs/{path}"}, {"type": "null"}]}
    defs["En1996OpeningPatch"]["properties"]["positionM"] = sn("WallOpening/properties/positionM")
    defs["En1996ConcentratedPatch"]["properties"]["distanceA1M"] = sn("ConcentratedLoad/properties/distanceA1M")
    defs["En1996ConcentratedPatch"]["properties"]["positionM"] = sn("ConcentratedLoad/properties/positionM")
    fm = ["🔣️jsonschema", "🟦️typescript"]
    for r in ROWS:
        pre, st = r["pre"], r["struct"]
        defs[f"En1996{pre}Patch"] = {"title": f"En1996{pre}Patch", "type": "object", "additionalProperties": False, "properties": {f["camel"]: sn(f"{st}/properties/{f['camel']}") for f in r["fields"]}, "x-semio-formats": fm}
        defs[f"En1996{pre}Removal"] = {"title": f"En1996{pre}Removal", "type": "object", "additionalProperties": False, "required": ["id", "index"], "properties": {"id": {"type": "string"}, "index": {"type": "integer", "minimum": 0}}, "x-semio-formats": fm}
        defs[f"En1996{pre}Insertion"] = {"title": f"En1996{pre}Insertion", "type": "object", "additionalProperties": False, "required": ["index", "row"], "properties": {"index": {"type": "integer", "minimum": 0}, "row": {"$ref": f"{SNAPSHOT_REF}#/$defs/{st}"}}, "x-semio-formats": fm}
        defs[f"En1996{pre}Relocation"] = {"title": f"En1996{pre}Relocation", "type": "object", "additionalProperties": False, "required": ["id", "from", "to"], "properties": {"id": {"type": "string"}, "from": {"type": "integer", "minimum": 0}, "to": {"type": "integer", "minimum": 0}}, "x-semio-formats": fm}
        defs[f"En1996{pre}Modification"] = {"title": f"En1996{pre}Modification", "type": "object", "additionalProperties": False, "required": ["id", "patch"], "properties": {"id": {"type": "string"}, "patch": {"$ref": f"#/$defs/En1996{pre}Patch"}}, "x-semio-formats": fm}
        defs[f"En1996{pre}Delta"] = {"title": f"En1996{pre}Delta", "type": "object", "additionalProperties": False, "required": ["removed", "inserted", "moved", "modified"], "properties": {k: {"type": "array", "items": {"$ref": f"#/$defs/En1996{pre}{n}"}} for k, n in (("removed", "Removal"), ("inserted", "Insertion"), ("moved", "Relocation"), ("modified", "Modification"))}, "x-semio-formats": fm}
        defs["En1996WallPatch"]["properties"][r["coll"]] = {"$ref": f"#/$defs/En1996{pre}Delta"}
    for x in SCALARS:
        defs["En1996WallPatch"]["properties"][x["camel"]] = sn(f"MasonryWall/properties/{x['camel']}")
    jdump(p, d)


patch_rs()
patch_ts()
patch_json()
print("diff done")
