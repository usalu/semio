"""Adds the `Storey` entity to the EN 1993 snapshot model, its five twins, the diff layer and the schema (idempotent). Usage: python3 -I en1993-storey-model-gen.py."""
import json, pathlib, re, sys

HERE = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
from schema_json import dump_schema

exec(compile((HERE / "en1993-storey-spec.py").read_text(encoding="utf8"), "spec", "exec"))
A = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any")
SCHEMA = A / "🧬️schema"
RS = {"String": "String", "f64": "f64", "u32": "u32"}
PB = {"String": "string", "f64": "double", "u32": "uint32"}
GQ = {"String": "String", "f64": "Float", "u32": "Int"}
TS = {"String": "string", "f64": "number", "u32": "number"}
WIRE = {"String": "normWireString", "f64": "normWireNumber", "u32": "normWireNumber"}
rd = lambda p: p.read_text(encoding="utf8")
wr = lambda p, s: p.write_text(s, encoding="utf8")


def sub1(s, pattern, repl, flags=0):
    out, n = re.subn(pattern, repl, s, count=1, flags=flags)
    assert n == 1, f"no match: {pattern[:100]}"
    return out


def rust_default(fd):
    v = fd["default"]
    if fd["ty"] == "String":
        return 'String::new()'
    return str(int(v)) if fd["ty"] == "u32" else repr(float(v))


def model():
    p = SCHEMA / "📸️snapshot/🦀️.rs"
    s = rd(p)
    if "pub struct Storey" in s:
        return
    m = re.search(r"(/// 🏗️ Crane runway local web check.*?\n)((?:#\[[^\n]*\n)+)pub struct CraneRunway", s, re.S)
    attrs = m.group(2)
    body = "".join(f"    pub {fd['name']}: {RS[fd['ty']]},\n" for fd in FIELDS)
    struct = f"/// 🪜 Storey of a frame: sway stability of EN 1993-1-1 5.2 and 5.3 (α_cr, imperfection, bracing).\n{attrs}pub struct Storey {{\n{body}}}\n\n"
    s = s.replace("/// 🗿️ The computed-compliance artifact", struct + "/// 🗿️ The computed-compliance artifact", 1)
    s = sub1(s, r"(    #\[state\(artifact\)\]\n    pub crane_runways: Vec<CraneRunway>,\n)", lambda m: m.group(1) + "    #[dsl(table)]\n    #[state(artifact)]\n    pub storeys: Vec<Storey>,\n")
    s = s.replace("            }],\n        }\n    }\n\n    /// ❌️ Non-compliant", "            }],\n            storeys: vec![],\n        }\n    }\n\n    /// ❌️ Non-compliant", 1)
    assert "storeys: vec![]," in s
    row = "".join(f"            row.{fd['name']} = {json.dumps(SHOW[fd['name']]) + '.into()' if fd['ty'] == 'String' else repr(float(SHOW[fd['name']])) if fd['ty'] == 'f64' else str(int(SHOW[fd['name']]))};\n" for fd in FIELDS)
    lit = ", ".join(f"{fd['name']}: " + (json.dumps(SHOW[fd['name']]) + ".into()" if fd["ty"] == "String" else repr(float(SHOW[fd["name"]])) if fd["ty"] == "f64" else str(int(SHOW[fd["name"]]))) for fd in FIELDS)
    show = f"        doc.storeys.push(Storey {{ {lit} }});\n"
    s = sub1(s, r"(pub fn special_parts_showcase\(\) -> Self \{\n        let mut doc = Self::compliant_heb240_frame\(\);\n)", lambda m: m.group(1) + show)
    wr(p, s)


def ts_snapshot(p, parse_name):
    s = rd(p)
    if "interface Storey" in s:
        return
    iface = "export interface Storey {\n" + "".join(f"  {camel(fd['name'])}: {TS[fd['ty']]};\n" for fd in FIELDS) + "}\n\n"
    s = sub1(s, r"(export interface CraneRunway \{.*?\n\}\n\n)", lambda m: m.group(1) + iface, re.S)
    s = sub1(s, r"(  craneRunways: CraneRunway\[\];\n)", lambda m: m.group(1) + "  /** 🪜 @state artifact */\n  storeys: Storey[];\n")
    parse = "export const parseStorey: NormWireReader<Storey> = normWireObject<Storey>({ " + ", ".join(f"{camel(fd['name'])}: normWireRequired({WIRE[fd['ty']]})" for fd in FIELDS) + " });\n"
    s = sub1(s, r"(export const parseCraneRunway: [^\n]*\n)", lambda m: m.group(1) + parse)
    s = sub1(s, r"(craneRunways: normWireRequired\(normWireArray\(normWireRef\(\(\) => parseCraneRunway\)\)\))", lambda m: m.group(1) + ", storeys: normWireRequired(normWireArray(normWireRef(() => parseStorey)))")
    wr(p, s)


def proto_snapshot(p):
    s = rd(p)
    if "message Storey" in s:
        return
    msg = "message Storey { " + " ".join(f"{PB[fd['ty']]} {fd['name']} = {i + 1};" for i, fd in enumerate(FIELDS)) + " }\n"
    s = sub1(s, r"(message CraneRunway \{[^\n]*\n)", lambda m: m.group(1) + msg)
    s = sub1(s, r"(  repeated CraneRunway crane_runways = 17;\n)", lambda m: m.group(1) + "  repeated Storey storeys = 18;\n")
    wr(p, s)


def graphql_snapshot(p):
    s = rd(p)
    if "type Storey" in s:
        return
    t = "type Storey { " + " ".join(f"{camel(fd['name'])}: {GQ[fd['ty']]}!" for fd in FIELDS) + " }\n"
    s = sub1(s, r"(type CraneRunway \{[^\n]*\n)", lambda m: m.group(1) + t)
    s = sub1(s, r"craneRunways: \[CraneRunway!\]!", "craneRunways: [CraneRunway!]! storeys: [Storey!]!")
    wr(p, s)


def ui_for(fd):
    u = U[fd["unit"]]
    ui = {"label": {"en": fd["en"], "de": fd["de"]}}
    if fd["ty"] == "u32":
        ui.update({"widget": "stepper", "step": 1, "precision": 0})
        return ui
    ui["widget"] = "stepper"
    if u[1]:
        ui["unit"] = u[1]
    if u[2]:
        ui["displayUnit"] = u[2]
        ui["displayFactor"] = u[3]
    ui["precision"] = u[4]
    ui["step"] = u[5]
    return ui


def schema_prop(fd):
    if fd["ty"] == "String":
        return {"type": "string"}
    if fd["ty"] == "u32":
        return {"type": "integer", "minimum": 0, "x-semio-ui": ui_for(fd), "maximum": 4294967295}
    return {"type": "number", "x-semio-ui": ui_for(fd)}


def json_snapshot(p):
    d = json.loads(rd(p))
    if "Storey" in d["$defs"]:
        return
    d["required"].append("storeys")
    props = {}
    for k, v in d["properties"].items():
        props[k] = v
    props["storeys"] = {"type": "array", "items": {"$ref": "#/$defs/Storey"}, "x-semio-state": "artifact"}
    d["properties"] = props
    defs = {}
    for k, v in d["$defs"].items():
        defs[k] = v
        if k == "CraneRunway":
            defs["Storey"] = {"title": "Storey", "type": "object", "additionalProperties": False, "required": [camel(fd["name"]) for fd in FIELDS], "properties": {camel(fd["name"]): schema_prop(fd) for fd in FIELDS}}
    d["$defs"] = defs
    wr(p, dump_schema(d))


def snapshot_twins():
    for sub in (SCHEMA, SCHEMA / "📸️snapshot"):
        ts_snapshot(sub / "🟦️.ts", "")
        proto_snapshot(sub / "🛰️.proto")
        graphql_snapshot(sub / "🔗️.graphql")
        json_snapshot(sub / "🔣️.json")
    graphql_snapshot(A / "🚪️io/📝️text/📸️snapshot/🔗️.graphql")


def schema_rs():
    p = SCHEMA / "🦀️.rs"
    s = rd(p)
    if "storeys" in s:
        return
    s = sub1(s, r"(use semio_s_artifact_norm_en1993_model::\{[^}]*SteelPile, SteelSection, )", lambda m: m.group(1) + "Storey, ")
    s = sub1(s, r"(    #\[state\(artifact\)\]\n    pub crane_runways: Vec<CraneRunway>,\n)", lambda m: m.group(1) + "    #[state(artifact)]\n    pub storeys: Vec<Storey>,\n")
    s = sub1(s, r"(            crane_runways: self\.crane_runways\.clone\(\),\n)", lambda m: m.group(1) + "            storeys: self.storeys.clone(),\n")
    s = sub1(s, r"(            crane_runways: snapshot\.crane_runways,\n)", lambda m: m.group(1) + "            storeys: snapshot.storeys,\n")
    wr(p, s)


def diff():
    D = SCHEMA / "🔺️diff"
    p = D / "🦀️.rs"
    s = rd(p)
    if DELTA not in s:
        sets = ", ".join(f"{fd['name']}: {RS[fd['ty']]}" for fd in PATCHF)
        rows = f"""protocol::row_patch! {{
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `Storey`.
    pub {PATCH} of semio_s_artifact_norm_en1993_model::Storey {{ set {{ {sets}}} }}
}}

/// 🔑️ Key extractor of the foreign rows of `{DELTA}` (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct {DELTA}Key;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::Storey> for {DELTA}Key {{
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::Storey) -> String {{
        row.id.clone()
    }}
}}

protocol::list_delta! {{
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `Storey` list.
    pub {DELTA} {{ removal: En1993StoreyRemoval, insertion: En1993StoreyInsertion, relocation: En1993StoreyRelocation, modification: En1993StoreyModification, row: semio_s_artifact_norm_en1993_model::Storey, patch: {PATCH}, list: Vec<semio_s_artifact_norm_en1993_model::Storey>, key: String = by {DELTA}Key }}
}}
"""
        s = sub1(s, r"(//#endregion 🔖️Rows)", lambda m: rows + m.group(1))
        s = sub1(s, r"(    #\[state\(artifact\)\]\n    pub crane_runways: En1993CraneRunwayDelta,\n)", lambda m: m.group(1) + f"    #[state(artifact)]\n    pub storeys: {DELTA},\n")
        s = sub1(s, r"(        self\.crane_runways\.absorb\(other\.crane_runways\);\n)", lambda m: m.group(1) + "        self.storeys.absorb(other.storeys);\n")
        s = sub1(s, r"(        next\.crane_runways = [^\n]*\n)", lambda m: m.group(1) + '        next.storeys = self.storeys.commit_onto(&base.storeys, capability).map_err(|error| error.under(["storeys"]))?;\n')
        s = sub1(s, r"(            && self\.crane_runways\.is_empty\(\)\n)", lambda m: m.group(1) + "            && self.storeys.is_empty()\n")
        s = sub1(s, r"(            crane_runways: self\.crane_runways\.inverse\(&base\.crane_runways\),\n)", lambda m: m.group(1) + "            storeys: self.storeys.inverse(&base.storeys),\n")
        wr(p, s)
    p = D / "🔗️.graphql"
    s = rd(p)
    if DELTA not in s:
        s = sub1(s, r"(  craneRunways: En1993CraneRunwayDelta! @state\(class: ARTIFACT\)\n)", lambda m: m.group(1) + f"  storeys: {DELTA}! @state(class: ARTIFACT)\n")
        blk = re.search(r"type En1993CraneRunwayDelta \{.*?type En1993CraneRunwayPatch \{.*?\n\}\n", s, re.S).group(0)
        new = blk.replace("CraneRunway", "Storey")
        new = re.sub(r"type En1993StoreyPatch \{.*?\n\}\n", lambda m: f"type {PATCH} {{\n" + "".join(f"  {camel(fd['name'])}: {GQ[fd['ty']]}\n" for fd in PATCHF) + "}\n", new, flags=re.S)
        s = s.replace(blk, blk + "\n" + new, 1)
        wr(p, s)
    p = D / "🛰️.proto"
    s = rd(p)
    if DELTA not in s:
        blk = re.search(r"message En1993CraneRunwayRemoval \{.*?message En1993CraneRunwayPatch \{.*?\n\}\n", s, re.S).group(0)
        new = blk.replace("CraneRunway", "Storey")
        new = re.sub(r"message En1993StoreyPatch \{.*?\n\}\n", lambda m: f"message {PATCH} {{\n" + "".join(f"  optional {PB[fd['ty']]} {fd['name']} = {i + 1};\n" for i, fd in enumerate(PATCHF)) + "}\n", new, flags=re.S)
        s = s.replace(blk, blk + "\n" + new, 1)
        s = sub1(s, r"(  En1993CraneRunwayDelta crane_runways = 17;\n)", lambda m: m.group(1) + f"  // @state artifact\n  {DELTA} storeys = 18;\n")
        wr(p, s)
    p = D / "🟦️.ts"
    s = rd(p)
    if DELTA not in s:
        patch_if = f"export interface {PATCH} {{ " + " ".join(f"{camel(fd['name'])}: ({TS[fd['ty']]}) | null;" for fd in PATCHF) + " }\n"
        ifs = re.search(r"export interface En1993CraneRunwayPatch [^\n]*\n(export interface [^\n]*\n){5}", s).group(0)
        new_ifs = ifs.replace("CraneRunway", "Storey").replace("row: Storey;", "row: Storey;")
        new_ifs = re.sub(r"export interface En1993StoreyPatch [^\n]*\n", lambda m: patch_if, new_ifs)
        s = s.replace(ifs, ifs + new_ifs, 1)
        s = sub1(s, r"(  craneRunways: En1993CraneRunwayDelta;\n)", lambda m: m.group(1) + f"  storeys: {DELTA};\n")
        pp = "export const parseEn1993StoreyPatch: NormWireReader<En1993StoreyPatch> = normWireObject<En1993StoreyPatch>({ " + ", ".join(f"{camel(fd['name'])}: normWireDefault(normWireNullable({WIRE[fd['ty']]}), () => null)" for fd in PATCHF) + " });\n"
        parses = re.search(r"export const parseEn1993CraneRunwayPatch[^\n]*\n(export const parseEn1993CraneRunway[^\n]*\n){5}", s).group(0)
        new_parses = parses.replace("CraneRunway", "Storey").replace("parseStorey)", "parseStorey)")
        new_parses = re.sub(r"export const parseEn1993StoreyPatch[^\n]*\n", lambda m: pp, new_parses)
        s = s.replace(parses, parses + new_parses, 1)
        s = sub1(s, r"(craneRunways: normWireDefault\(normWireRef\(\(\) => parseEn1993CraneRunwayDelta\), \(\) => \(\{ removed: \[\], inserted: \[\], moved: \[\], modified: \[\] \}\)\))", lambda m: m.group(1) + ", storeys: normWireDefault(normWireRef(() => parseEn1993StoreyDelta), () => ({ removed: [], inserted: [], moved: [], modified: [] }))")
        s = sub1(s, r"type CraneRunway, ", "type CraneRunway, ")
        s = s.replace("type SteelSection, type TensionComponent,", "type SteelSection, type Storey, type TensionComponent,", 1).replace("parseSteelSection, parseTensionComponent,", "parseSteelSection, parseStorey, parseTensionComponent,", 1)
        wr(p, s)
    p = D / "🔣️.json"
    d = json.loads(rd(p))
    if DELTA not in d["$defs"]:
        props = {}
        for k, v in d["properties"].items():
            props[k] = v
        props["storeys"] = {"anyOf": [{"$ref": f"#/$defs/{DELTA}"}], "x-semio-state": "artifact"}
        d["properties"] = props
        defs = {}
        for k, v in d["$defs"].items():
            defs[k] = v
            if k == "En1993CraneRunwayPatch":
                for kk in ("Delta", "Removal", "Insertion", "Relocation", "Modification", "Patch"):
                    nv = json.loads(json.dumps(d["$defs"]["En1993CraneRunway" + kk]).replace("En1993CraneRunway", "En1993Storey").replace("snapshot.json#/$defs/CraneRunway", "snapshot.json#/$defs/Storey"))
                    if kk == "Patch":
                        nv["properties"] = {camel(fd["name"]): {"anyOf": [{"type": {"String": "string", "f64": "number", "u32": "integer"}[fd["ty"]]}, {"type": "null"}]} for fd in PATCHF}
                    defs["En1993Storey" + kk] = nv
        d["$defs"] = defs
        wr(p, dump_schema(d))


model()
snapshot_twins()
schema_rs()
diff()
print("storey model gen done")
