#!/usr/bin/env python3
"""Adds the typed snapshot field `fireLoadBasis` (enum occupancyTable | userSpecified, default occupancyTable) after `siloHopperAngleDeg` and its edit leaf
`change-fire-load-basis` (clone of `change-fire-mode`) through every EN 1991 layer: model, artifact, outline, diff, json / ts twins, sqlite (sql, rs, ts, admission,
contract, tests), assets, examples, committed fixtures, leaf registration. Idempotent. Input script (recipe of en1991-add-tandem-x.py)."""
import copy, glob, json, os, re, sqlite3

ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any"
SCH = f"{ROOT}/🧬️schema"
MUT = f"{SCH}/🧬️mutations"
FIX = f"{ROOT}/🧪️tests/🧫️fixtures"
SQ = f"{ROOT}/🚪️io/🪶️sqlite/📸️snapshot"
KIND, DIR, P, S = "change-fire-load-basis", "🧯change-fire-load-basis", "ChangeFireLoadBasis", "change_fire_load_basis"
SRC_KIND, SRC_DIR = "change-fire-mode", "🔥change-fire-mode"
OWNER = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
VARIANTS = ["occupancyTable", "userSpecified"]


def rd(p):
    return open(p, encoding="utf-8").read()


def wr(p, s):
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w", encoding="utf-8").write(s)


def sub(path, old, new, count=1, marker=None):
    s = rd(path)
    if (marker or new) in s:
        return
    assert s.count(old) == count, (path, old[:90], s.count(old))
    wr(path, s.replace(old, new))


def after(path, anchor, add, count=1):
    sub(path, anchor, anchor + add, count, marker=add)


def jdump(p, d):
    wr(p, json.dumps(d, indent=2, ensure_ascii=False) + "\n")


def model():
    m = f"{SCH}/📸️snapshot/🦀️.rs"
    after(m, "    pub silo_hopper_angle_deg: f64,\n", "    #[state(artifact)]\n    pub fire_load_basis: crate::FireLoadBasis,\n")
    after(m, "            silo_hopper_angle_deg: 60.0,\n", "            fire_load_basis: crate::FireLoadBasis::OccupancyTable,\n")
    enum = (
        "/// 🧯 Basis of the characteristic fire load density q_f,k (EN 1991-1-2, Annex E): the 80 % fractile of the occupancy row of Table E.4, or the value the user specifies.\n"
        "#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]\n"
        "#[canonical_json(owner = semio_framework_pack_json)]\n"
        '#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]\n'
        '#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]\n'
        '#[value(rename_all = "camelCase")]\n'
        "pub enum FireLoadBasis {\n    OccupancyTable,\n    UserSpecified,\n}\n\n"
    )
    sub(m, "/// 🏗️ Structure kind — building vs bridge", enum + "/// 🏗️ Structure kind — building vs bridge", marker="pub enum FireLoadBasis")
    a = f"{SCH}/🦀️.rs"
    after(a, "    pub silo_hopper_angle_deg: f64,\n", "    #[state(artifact)]\n    pub fire_load_basis: semio_s_artifact_norm_en1991_model::FireLoadBasis,\n")
    after(a, "            silo_hopper_angle_deg: self.silo_hopper_angle_deg,\n", "            fire_load_basis: self.fire_load_basis,\n")
    after(a, "            silo_hopper_angle_deg: snapshot.silo_hopper_angle_deg,\n", "            fire_load_basis: snapshot.fire_load_basis,\n")
    d = f"{SCH}/🔺️diff/🦀️.rs"
    after(d, "    pub silo_hopper_angle_deg: Option<f64>,\n", "    #[state(artifact)]\n    pub fire_load_basis: Option<semio_s_artifact_norm_en1991_model::FireLoadBasis>,\n")
    after(d, "            self.silo_hopper_angle_deg = other.silo_hopper_angle_deg;\n        }\n", "        if other.fire_load_basis.is_some() {\n            self.fire_load_basis = other.fire_load_basis;\n        }\n")
    after(d, "            silo_hopper_angle_deg: self.silo_hopper_angle_deg.unwrap_or(base.silo_hopper_angle_deg),\n", "            fire_load_basis: self.fire_load_basis.unwrap_or(base.fire_load_basis),\n")
    after(d, "            && self.silo_hopper_angle_deg.is_none()\n", "            && self.fire_load_basis.is_none()\n")
    after(d, "            silo_hopper_angle_deg: self.silo_hopper_angle_deg.as_ref().map(|_| base.silo_hopper_angle_deg),\n", "            fire_load_basis: self.fire_load_basis.as_ref().map(|_| base.fire_load_basis),\n")
    after(f"{SCH}/💡️inferences/🧾outline/🦀️.rs", '    "silo_hopper_angle_deg",\n', '    "fire_load_basis",\n')
    after(f"{ROOT}/📚️examples/🧬️subjects/🦀️.rs", "    s.fire_load_density_qf = 420.0e6;\n", "    s.fire_load_basis = semio_s_artifact_norm_en1991_model::FireLoadBasis::OccupancyTable;\n")


def twins():
    enum_inline = '      "type": "string",\n      "enum": [\n        "occupancyTable",\n        "userSpecified"\n      ],\n      "x-semio-state": "artifact"\n    },\n'
    for rel in ["🔣️.json", "📸️snapshot/🔣️.json"]:
        p = f"{SCH}/{rel}"
        after(p, '    "siloHopperAngleDeg",\n', '    "fireLoadBasis",\n')
        blk = '    "siloHopperAngleDeg": {\n      "type": "number",\n      "format": "double",\n      "x-semio-state": "artifact"\n    },\n'
        after(p, blk, '    "fireLoadBasis": {\n' + enum_inline)
    p = f"{SCH}/📸️snapshot/🔣️.json"
    defs = (
        '    "FireLoadBasis": {\n      "title": "FireLoadBasis",\n      "type": "string",\n      "enum": [\n        "occupancyTable",\n        "userSpecified"\n      ],\n'
        '      "x-semio-ui": {\n        "options": {\n          "occupancyTable": {\n            "en": "Occupancy table (Table E.4, 80 % fractile)",\n            "de": "Nutzungstabelle (Tabelle E.4, 80-%-Fraktile)"\n          },\n'
        '          "userSpecified": {\n            "en": "User-specified q_f,k",\n            "de": "Vorgegebenes q_f,k"\n          }\n        }\n      },\n'
        '      "x-semio-formats": [\n        "🔣️jsonschema"\n      ]\n    },\n'
    )
    sub(p, '    "BridgeLane": {\n      "title": "BridgeLane",', defs + '    "BridgeLane": {\n      "title": "BridgeLane",', marker='"FireLoadBasis": {')
    p = f"{SCH}/🔺️diff/🔣️.json"
    blk = '    "siloHopperAngleDeg": {\n      "anyOf": [\n        {\n          "$ref": "https://json.schemas.assets.semio-tech.com/s/norm/en1991/snapshot.json#/properties/siloHopperAngleDeg"\n        },\n        {\n          "type": "null"\n        }\n      ],\n      "x-semio-state": "artifact"\n    },\n'
    after(p, blk, blk.replace("siloHopperAngleDeg", "fireLoadBasis"))
    lit = 'normWireLiteral("occupancyTable", "userSpecified")'
    for rel, req in [("🟦️.ts", f"normWireRequired({lit})"), ("📸️snapshot/🟦️.ts", f"normWireRequired({lit})"), ("🔺️diff/🟦️.ts", f"normWireDefault(normWireNullable({lit}), () => null)")]:
        p = f"{SCH}/{rel}"
        sub(p, "siloHopperAngleDeg: " + ("normWireRequired(normWireNumber), floors:" if "diff" not in rel else "normWireDefault(normWireNullable(normWireNumber), () => null), floors:"), "siloHopperAngleDeg: " + ("normWireRequired(normWireNumber), " if "diff" not in rel else "normWireDefault(normWireNullable(normWireNumber), () => null), ") + f"fireLoadBasis: {req}, floors:", marker="fireLoadBasis: ")
        nullable = "" if "diff" not in rel else " | null"
        ty = '"occupancyTable" | "userSpecified"' if not nullable else '("occupancyTable" | "userSpecified") | null'
        sub(p, f"  siloHopperAngleDeg: number{nullable};\n", f"  siloHopperAngleDeg: number{nullable};\n  fireLoadBasis: {ty};\n")
    p = f"{SCH}/📸️snapshot/🟦️.ts"
    after(p, 'export type En1991FireMode = "none" | "nominal" | "parametric";\n', '\nexport type En1991FireLoadBasis = "occupancyTable" | "userSpecified";\n')
    after(p, 'export const parseEn1991FireMode: NormWireReader<En1991FireMode> = normWireLiteral("none", "nominal", "parametric");\n', f"export const parseEn1991FireLoadBasis: NormWireReader<En1991FireLoadBasis> = {lit};\n")


def sqlite():
    p = f"{SQ}/🗄️.sql"
    sub(p, "silo_hopper_angle_deg REAL,\n", "silo_hopper_angle_deg REAL, fire_load_basis TEXT NOT NULL CHECK(fire_load_basis IN ('occupancyTable','userSpecified')),\n", marker="fire_load_basis TEXT")
    r = f"{SQ}/🦀️.rs"
    sub(r, "AccidentalImpact, FireMode,", "AccidentalImpact, FireLoadBasis, FireMode,")
    sub(r, 'entities(database, "en1991_document", 78,', 'entities(database, "en1991_document", 79,')
    sub(r, "Cell::Real(self.silo_hopper_angle_deg)\n", 'Cell::Real(self.silo_hopper_angle_deg), Cell::Text(match self.fire_load_basis { FireLoadBasis::OccupancyTable => "occupancyTable", FireLoadBasis::UserSpecified => "userSpecified" })\n')
    sub(r, "silo_hopper_angle_deg: document.real(77)?, floors,", 'silo_hopper_angle_deg: document.real(77)?, fire_load_basis: match document.text(78)? { "occupancyTable" => FireLoadBasis::OccupancyTable, "userSpecified" => FireLoadBasis::UserSpecified, _ => return Err(invalid("EN1991 fire load basis is not owned")) }, floors,')
    t = f"{SQ}/🟦️.ts"
    sub(t, "function fireMode(value:unknown)", 'function fireLoadBasis(value:unknown):En1991SqliteSnapshot["fireLoadBasis"]{if(value!=="occupancyTable"&&value!=="userSpecified")throw Error("EN1991 fire load basis is not owned");return value}\nfunction fireMode(value:unknown)', marker="function fireLoadBasis")
    sub(t, "s.siloHopperAngleDeg],DOCUMENT_FLOATS", "s.siloHopperAngleDeg,fireLoadBasis(s.fireLoadBasis)],DOCUMENT_FLOATS")
    sub(t, "siloHopperAngleDeg:readBinary64(d,77,DOCUMENT_FLOATS),floors", "siloHopperAngleDeg:readBinary64(d,77,DOCUMENT_FLOATS),fireLoadBasis:fireLoadBasis(artifactSqliteText(d,78)),floors")
    sub(t, "entities(tables[0]!,78,DOCUMENT_FLOATS", "entities(tables[0]!,79,DOCUMENT_FLOATS")
    a = f"{SQ}/🛂️admission/🦀️.rs"
    s = rd(a)
    if "Enumeration(77,&[\"occupancyTable\"" not in s:
        s = s.replace("for id in 77..=92{", "for id in 78..=93{")
        s = re.sub(r"n::field\(root,(\d+)\)", lambda m: f"n::field(root,{int(m.group(1)) + 1})" if int(m.group(1)) >= 77 else m.group(0), s)
        s = s.replace("Real(76)])?,DOCUMENT_FLOATS)", 'Real(76),Enumeration(77,&["occupancyTable","userSpecified"])])?,DOCUMENT_FLOATS)')
        s = s.replace("including the full 188-column", "including the full 189-column").replace("SQLITE_SCHEMA,21,188,", "SQLITE_SCHEMA,21,189,")
        assert 'Enumeration(77,&["occupancyTable"' in s and "21,189," in s
        wr(a, s)
    tr = f"{SQ}/🧪️tests/🦀️.rs"
    sub(tr, 'for field in ["annex", "fireMode", "fireCurve", "structureKind"] {', 'for field in ["annex", "fireMode", "fireCurve", "structureKind", "fireLoadBasis"] {')
    tt = f"{SQ}/🧪️tests/🟦️.ts"
    s = rd(tt)
    if "fireLoadBasis" not in s:
        s = s.replace('["annex","fireMode","fireCurve","structureKind"]', '["annex","fireMode","fireCurve","structureKind","fireLoadBasis"]')
        s = s.replace('case"structureKind":expected.structureKind=value as En1991SqliteSnapshot["structureKind"]}', 'case"structureKind":expected.structureKind=value as En1991SqliteSnapshot["structureKind"];break;case"fireLoadBasis":expected.fireLoadBasis=value as En1991SqliteSnapshot["fireLoadBasis"]}')
        s = s.replace('"UPDATE en1991_document SET fire_curve=\'unknown\'",', '"UPDATE en1991_document SET fire_curve=\'unknown\'","UPDATE en1991_document SET fire_load_basis=\'unknown\'",')
        s = s.replace('annex:"En",fireMode:"parametric",', 'annex:"En",fireLoadBasis:"occupancyTable",fireMode:"parametric",')
        wr(tt, s)
    ch = f"{SQ}/🧪️tests/🧫️fixtures/🛂️choices.json"
    c = json.loads(rd(ch))
    c["fireLoadBasis"] = VARIANTS
    wr(ch, json.dumps(c, ensure_ascii=False, separators=(",", ":")) + "\n")
    sql = rd(p)
    con = sqlite3.connect(":memory:")
    con.executescript(sql)
    rows = con.execute("select name, sql from sqlite_schema where type='table' order by name").fetchall()
    widths = {n: len(con.execute(f"pragma table_info({n})").fetchall()) for n, _ in rows}
    extent = {"schemaBytes": sum(len(n.encode()) + len(q.encode()) for n, q in rows), "tableWidths": dict(sorted(widths.items()))}
    sj = f"{SQ}/🧪️tests/🧫️fixtures/🎛️semantic.json"
    d = json.loads(rd(sj))
    d["schemaBytes"], d["tableWidths"] = extent["schemaBytes"], extent["tableWidths"]
    wr(sj, json.dumps(d, indent=2, ensure_ascii=False) + "\n")
    s = rd(tt)
    s = re.sub(r"(expect\(normSemanticContract\[\"schemaBytes\"\]\)\.toEqual\()\d+", lambda m: m.group(1) + str(extent["schemaBytes"]), s)
    s = re.sub(r'(expect\(normSemanticContract\["tableWidths"\]\)\.toEqual\()\{[^}]*\}', lambda m: m.group(1) + json.dumps(extent["tableWidths"], separators=(",", ":")), s)
    wr(tt, s)
    print("sqlite contract:", extent["schemaBytes"], extent["tableWidths"]["en1991_document"])


def fixtures():
    count = 0
    pat = re.compile(r'("siloHopperAngleDeg"\s*:\s*[^,\n}\]]+,)(\s*)')
    for f in glob.glob(f"{FIX}/**/*.json", recursive=True) + glob.glob(f"{ROOT}/🔮️oracles/**/*.json", recursive=True) + glob.glob(f"{ROOT}/🚪️io/**/🧫️fixtures/**/*.json", recursive=True):
        if "/🧬️mutations/" + DIR + "/" in f:
            continue
        s = rd(f)
        if '"fireLoadBasis"' in s or '"siloHopperAngleDeg"' not in s:
            continue
        value = "null" if "🔺️diff" in f or "/diff" in f else '"occupancyTable"'
        out, n = pat.subn(lambda m: f'{m.group(1)}{m.group(2)}"fireLoadBasis": {value},{m.group(2)}', s, count=1)
        if n:
            json.loads(out)
            wr(f, out)
            count += 1
    print("fixtures updated:", count)


def assets():
    for f in glob.glob(f"{ROOT}/🖼️assets/**/🗣️.dsl.semio", recursive=True):
        s = rd(f)
        if "fire-load-basis=" in s:
            continue
        out, n = re.subn(r"(silo-hopper-angle-deg=[0-9.]+)", r"\1 fire-load-basis=occupancyTable", s, count=1)
        assert n == 1, f
        wr(f, out)


def leaf():
    if os.path.isdir(f"{MUT}/{DIR}"):
        return
    def conv(s):
        return (s.replace("change-fire-mode", KIND).replace("change_fire_mode", S).replace("ChangeFireMode", P).replace("ChangedFireMode", "ChangedFireLoadBasis")
                .replace("new_fire_mode", "new_fire_load_basis").replace("newFireMode", "newFireLoadBasis").replace("fire_mode", "fire_load_basis").replace("fire-mode", "fire-load-basis")
                .replace("semio_s_artifact_norm_en1991_model::FireMode", "semio_s_artifact_norm_en1991_model::FireLoadBasis").replace("FireMode", "FireLoadBasis")
                .replace("Change fire design method", "Change fire load basis").replace("Brandschutzbemessungsverfahren ändern", "Brandlastgrundlage ändern").replace("🔥", "🧯"))
    for rel in ["🦀️.rs", "🔺️diff/🦀️.rs", "↩️inverse/🦀️.rs", "🧬️schema/🟦️.ts"]:
        wr(f"{MUT}/{DIR}/{rel}", conv(rd(f"{MUT}/{SRC_DIR}/{rel}")))
    j = json.loads(rd(f"{MUT}/{SRC_DIR}/🔣️.json"))
    tags = [int(x) for x in re.findall(r"tag=(\d+)", rd(f"{ROOT}/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"))]
    tag = max(tags) + 1
    j.update(owner=f"{OWNER}/{DIR}", semanticKind=KIND, displayName=P, emoji="🧯", aggregateVariant=P, binaryTag=tag)
    jdump(f"{MUT}/{DIR}/🔣️.json", j)
    sj = json.loads(rd(f"{MUT}/{SRC_DIR}/🧬️schema/🔣️.json"))
    sj["$id"] = sj["$id"].replace("change-fire-mode", KIND)
    sj["title"] = P
    sj["required"] = ["newFireLoadBasis"]
    prop = sj["properties"].pop("newFireMode")
    prop["$ref"] = prop["$ref"].replace("$defs/FireMode", "$defs/FireLoadBasis")
    if "x-semio-ui" in prop:
        prop["x-semio-ui"]["label"] = {"en": "Fire load basis", "de": "Brandlastgrundlage"}
    sj["properties"] = {"newFireLoadBasis": prop}
    jdump(f"{MUT}/{DIR}/🧬️schema/🔣️.json", sj)
    mount = f'#[path = "../{DIR}/🦀️.rs"]\npub mod {S};\n'
    sub(f"{MUT}/🔥️fire/🦀️.rs", "//#endregion 🪢️Mounts", mount + "//#endregion 🪢️Mounts", marker=mount)
    agg = f"{MUT}/🦀️.rs"
    after(agg, "use semio_s_artifact_norm_en1991_mut_fire::change_fire_mode;\n", f"use semio_s_artifact_norm_en1991_mut_fire::{S};\n")
    after(agg, "    ChangeFireMode(change_fire_mode::ChangeFireMode),\n", f"    {P}({S}::{P}),\n")
    after(agg, '    "change-fire-mode",\n', f'    "{KIND}",\n')
    after(f"{ROOT}/🚪️io/📝️text/🧬️mutations/🦀️.rs", "use semio_s_artifact_norm_en1991_mut_fire::change_fire_mode;\n", f"use semio_s_artifact_norm_en1991_mut_fire::{S};\n")
    tsx = f"{MUT}/🟦️.ts"
    after(tsx, 'import { type ChangeFireMode, parseChangeFireMode } from "./🔥change-fire-mode/🧬️schema/🟦️.ts";\n', f'import {{ type {P}, parse{P} }} from "./{DIR}/🧬️schema/🟦️.ts";\n')
    after(tsx, "  | { ChangeFireMode: ChangeFireMode }\n", f"  | {{ {P}: {P} }}\n")
    after(tsx, "  ChangeFireMode: parseChangeFireMode,\n", f"  {P}: parse{P},\n")
    aj = json.loads(rd(f"{MUT}/🔣️.json"))
    for key, v in aj.items():
        if isinstance(v, list) and any("ChangeFireMode" in json.dumps(x) for x in v) and not any(P in json.dumps(x) for x in v):
            i = next(i for i, x in enumerate(v) if "ChangeFireMode" in json.dumps(x))
            v.insert(i + 1, json.loads(json.dumps(v[i]).replace("ChangeFireMode", P).replace("change-fire-mode", KIND)))
    jdump(f"{MUT}/🔣️.json", aj)
    after(f"{MUT}/🧭️edit-rules/🦀️.rs", '        SetFieldRule { path: "fireMode", kind: "change-fire-mode", selectors: &[], value: "newFireMode" },\n', f'        SetFieldRule {{ path: "fireLoadBasis", kind: "{KIND}", selectors: &[], value: "newFireLoadBasis" }},\n')
    proto = f"{ROOT}/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
    s = rd(proto)
    if f"record {KIND} " not in s:
        wr(proto, s.rstrip("\n") + f"\nrecord {KIND} tag={tag}\nfield payload bytes\n")
    om = f"{ROOT}/🔮️oracles/🔣️.json"
    o = json.loads(rd(om))
    def walk(node):
        if isinstance(node, dict):
            for v in node.values():
                yield from walk(v)
        elif isinstance(node, list):
            yield node
            for v in node:
                yield from walk(v)
    for lst in list(walk(o)):
        if KIND in json.dumps(lst):
            continue
        for i, x in enumerate(lst):
            if x == SRC_KIND:
                lst.insert(i + 1, KIND)
                break
            if isinstance(x, dict) and x.get("mutationId") == SRC_KIND:
                n = copy.deepcopy(x)
                n.update(mutationId=KIND, sourceMutationDirectoryName=DIR, mutationDirectoryName=DIR)
                lst.insert(i + 1, n)
                break
            if isinstance(x, dict) and x.get("id") == SRC_KIND:
                n = copy.deepcopy(x)
                n["id"] = KIND
                n["productionDispatch"].update(operation=KIND, variant=P)
                lst.insert(i + 1, n)
                break
    jdump(om, o)
    ft = f"{ROOT}/🧪️tests/🏋️mutate-en1991-1/🥒️.feature"
    s = rd(ft)
    if f"| {KIND} " not in s:
        out = []
        for l in s.split("\n"):
            out.append(l)
            if l.strip().startswith(f"| {SRC_KIND} "):
                out.append(l.replace(SRC_KIND, KIND).replace("🔥", "🧯"))
        wr(ft, "\n".join(out))
    src = f"{FIX}/🧬️mutations/{SRC_DIR}/✅apply"
    dst = f"{FIX}/🧬️mutations/{DIR}/✅apply"
    before = json.loads(rd(f"{src}/🔣️.before.json"))
    before["fireLoadBasis"] = "occupancyTable"
    aft = copy.deepcopy(before)
    aft["fireLoadBasis"] = "userSpecified"
    jdump(f"{dst}/🔣️.before.json", before)
    jdump(f"{dst}/🔣️.after.json", aft)
    jdump(f"{dst}/🔣️.mutation.json", {P: {"newFireLoadBasis": "userSpecified"}})
    jdump(f"{dst}/🔣️.outcome.json", {"status": "applied"})
    d = json.loads(rd(f"{src}/🔣️.diff.json"))
    d["fireMode"] = None
    d["fireLoadBasis"] = "userSpecified"
    jdump(f"{dst}/🔣️.diff.json", d)


model()
twins()
sqlite()
leaf()
fixtures()
assets()
print("done")
