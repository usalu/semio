"""EN 1999 wave 5: adds FatigueCategory, PatchLoad and RingStiffener as snapshot collections through model, root, five snapshot twins, diff lanes, field-meta, SQLite, fixtures. Idempotent per stage. Usage: python -I add_entities.py"""
import copy, json, pathlib, re, sys
HERE = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(HERE.parent / "en1999-wave2"))
sys.path.insert(0, str(HERE.parent / "leaf-kinds"))
from spec5 import *
import s1_model as S1
import diff as D, model as M

PREV = SECTION_VERTEX
EMOJI = {e.field: e.emoji for e in NEW}


def rd(p):
    return pathlib.Path(p).read_text(encoding="utf8")


def wr(p, t):
    pathlib.Path(p).write_text(t, encoding="utf8")


def fresh(t, e):
    return e.struct in t


def model_rs():
    p = SNAP / "🦀️.rs"
    t = rd(p)
    if "pub struct FatigueCategory" in t:
        return
    anchor = "//#endregion 🔖️SubjectEntities"
    t = t.replace(anchor, "".join(S1.rust_struct(e) for e in NEW) + anchor)
    old = "    #[state(artifact)]\n    #[dsl(table)]\n    pub outline_vertices: Vec<SectionVertex>,\n"
    assert old in t
    t = t.replace(old, old + "".join(f"    #[state(artifact)]\n    #[dsl(table)]\n    pub {e.field}: Vec<{e.struct}>,\n" for e in NEW), 1)
    t = t.replace("            outline_vertices: Vec::new(),\n", "            outline_vertices: Vec::new(),\n" + "".join(f"            {e.field}: Vec::new(),\n" for e in NEW))
    wr(p, t)


def root_rs():
    p = SCHEMA / "🦀️.rs"
    t = rd(p)
    if "FatigueCategory" in t:
        return
    t = t.replace("WeldRun, SectionVertex,", "WeldRun, SectionVertex, " + ", ".join(e.struct for e in NEW) + ",", 1)
    old = "    #[state(artifact)]\n    pub outline_vertices: Vec<SectionVertex>,\n"
    assert old in t
    t = t.replace(old, old + "".join(f"    #[state(artifact)]\n    pub {e.field}: Vec<{e.struct}>,\n" for e in NEW), 1)
    a = "            outline_vertices: self.outline_vertices.clone(),\n"
    assert a in t
    t = t.replace(a, a + "".join(f"            {e.field}: self.{e.field}.clone(),\n" for e in NEW), 1)
    a = "            outline_vertices: snapshot.outline_vertices,\n"
    assert a in t
    t = t.replace(a, a + "".join(f"            {e.field}: snapshot.{e.field},\n" for e in NEW), 1)
    wr(p, t)


def snapshot_json():
    p = SNAP / "🔣️.json"
    d = json.loads(rd(p))
    if "FatigueCategory" in d["$defs"]:
        return
    for e in NEW:
        d["required"].append(e.camel)
        d["properties"][e.camel] = {"type": "array", "items": {"$ref": f"#/$defs/{e.struct}"}, "x-semio-state": "artifact"}
        d["$defs"][e.struct] = S1.json_def(e)
    wr(p, json.dumps(d, indent=2, ensure_ascii=False) + "\n")


def root_json():
    p = SCHEMA / "🔣️.json"
    d = json.loads(rd(p))
    if "fatigueCategories" in d["properties"]:
        return
    for e in NEW:
        d["required"].append(e.camel)
        props = {"id": {"type": "string"}, **{f.camel: {"type": f.json} for f in e.fields}}
        d["properties"][e.camel] = {"type": "array", "items": {"type": "object", "additionalProperties": False, "required": list(props), "properties": props}, "x-semio-state": "artifact"}
    wr(p, json.dumps(d, indent=2, ensure_ascii=False) + "\n")


def snapshot_ts():
    p = SNAP / "🟦️.ts"
    t = rd(p)
    if "export interface FatigueCategory" in t:
        return
    a = "  outlineVertices: SectionVertex[];\n}"
    assert a in t
    t = t.replace(a, "  outlineVertices: SectionVertex[];\n" + "".join(f"  /** {e.emoji} @state artifact */\n  {e.camel}: {e.struct}[];\n" for e in NEW) + "}", 1)
    i = t.index("export interface SectionVertex {")
    j = t.index("}\n\n", i) + 3
    t = t[:j] + "".join(S1.ts_iface(e) for e in NEW) + t[j:]
    a = "outlineVertices: normWireRequired(normWireArray(normWireRef(() => parseSectionVertex))) });"
    assert a in t
    t = t.replace(a, a[:-4] + ", " + ", ".join(f"{e.camel}: normWireRequired(normWireArray(normWireRef(() => parse{e.struct})))" for e in NEW) + " });", 1)
    k = t.index("export const parseSectionVertex:")
    k = t.index("\n", k) + 1
    t = t[:k] + "".join(S1.ts_parser(e) for e in NEW) + t[k:]
    wr(p, t)


def inline_ts(e):
    return "{ id: string; " + " ".join(f"{f.camel}: {f.ts};" for f in e.fields) + " }"


def root_ts():
    p = SCHEMA / "🟦️.ts"
    t = rd(p)
    if "fatigueCategories:" in t:
        return
    a = "  outlineVertices: { id: string; sectionId: string; ring: string; elementId: string; y: number; z: number; }[];\n}"
    assert a in t
    t = t.replace(a, a[:-1] + "".join(f"  /** {e.emoji} @state artifact */\n  {e.camel}: {inline_ts(e)}[];\n" for e in NEW) + "}", 1)
    reader = lambda e: f"{e.camel}: normWireRequired(normWireArray(normWireObject<{inline_ts(e)}>({{ " + ", ".join(["id: normWireRequired(normWireString)"] + [f"{f.camel}: normWireRequired({f.wire})" for f in e.fields]) + " })))"
    tail = "z: normWireRequired(normWireNumber) }))) });"
    assert t.count(tail) == 1
    t = t.replace(tail, "z: normWireRequired(normWireNumber) }))), " + ", ".join(reader(e) for e in NEW) + " });", 1)
    wr(p, t)


def graphql():
    for p in (SCHEMA / "🔗️.graphql", SNAP / "🔗️.graphql"):
        t = rd(p)
        if "FatigueCategory" in t:
            continue
        a = "  outlineVertices: [SectionVertex!]! @state(artifact)\n"
        assert a in t
        t = t.replace(a, a + "".join(f"  {e.camel}: [{e.struct}!]! @state(artifact)\n" for e in NEW))
        i = t.index("type SectionVertex {")
        i = t.index("\n", i) + 1
        t = t[:i] + "".join(S1.gql_type(e) for e in NEW) + t[i:]
        wr(p, t)


def proto():
    for p in (SCHEMA / "🛰️.proto", SNAP / "🛰️.proto"):
        t = rd(p)
        if "FatigueCategory" in t:
            continue
        a = "  repeated SectionVertex outline_vertices = 12;\n"
        assert a in t
        t = t.replace(a, a + "".join(f"  repeated {e.struct} {e.field} = {13 + n};\n" for n, e in enumerate(NEW)))
        i = t.index("message SectionVertex {")
        i = t.index("\n", i) + 1
        t = t[:i] + "".join(S1.proto_msg(e) for e in NEW) + t[i:]
        wr(p, t)


URL = "https://json.schemas.assets.semio-tech.com/s/norm/en1999/snapshot.json"
MODEL = "semio_s_artifact_norm_en1999_model"
V = SECTION_VERTEX


def clone(text, ent):
    return text.replace("OutlineVertices", ent.lane).replace("SectionVertex", ent.struct).replace("outline_vertices", ent.field).replace("outlineVertices", ent.camel)


def between(t, a, b):
    i = t.index(a)
    return t[i:t.index(b, i)]


def diff_rust(t):
    if "FatigueCategoryKey" in t:
        return t
    key = between(t, "/// 🔑️ Key extractor of the foreign `SectionVertex` rows", "\nuse protocol::list_delta::Keyed as _;")
    t = t.replace(key, key + "".join(clone(key, e) for e in NEW), 1)
    s = t.rindex("/// 🩹️ Sparse patch of the", 0, t.index("pub struct En1999OutlineVerticesPatch"))
    e_ = t.index("\n}\n", t.index("pub En1999OutlineVerticesRows {", s)) + 3
    lane = t[s:e_]
    t = t.replace(lane, lane + "".join(clone(lane, e) for e in NEW), 1)
    a = "    pub outline_vertices: Option<En1999OutlineVerticesRows>,\n"
    t = t.replace(a, a + "".join(f"    #[state(artifact)]\n    pub {e.field}: Option<En1999{e.lane}Rows>,\n" for e in NEW), 1)
    ab = between(t, "        if let Some(theirs) = other.outline_vertices {", "    }\n\n    fn apply(")
    t = t.replace(ab, ab + "".join(clone(ab, e) for e in NEW), 1)
    ap = between(t, "        if let Some(rows) = &self.outline_vertices {", "        *unpublished = next;")
    t = t.replace(ap, ap + "".join(clone(ap, e) for e in NEW), 1)
    a = ".outline_vertices.as_ref().map_or(true, |rows| rows.is_empty())"
    assert t.count(a) == 1
    t = t.replace(a, a + "".join(f" && self.{e.field}.as_ref().map_or(true, |rows| rows.is_empty())" for e in NEW), 1)
    inv = "            outline_vertices: self.outline_vertices.as_ref().map(|rows| rows.inverse(&base.outline_vertices)).filter(|rows| !rows.is_empty()),\n"
    assert inv in t
    t = t.replace(inv, inv + "".join(clone(inv, e) for e in NEW), 1)
    return t


def diff_ts(t):
    if "FatigueCategories" in t:
        return t
    ifaces = between(t, "export interface En1999OutlineVerticesPatch {", "export const parseEn1999Diff:")
    t = t.replace(ifaces, ifaces + "".join(clone(ifaces, e) for e in NEW), 1)
    i = t.index("export const parseEn1999OutlineVerticesInserted:")
    j = t.index("export const parseEn1999OutlineVerticesPatch:")
    j = t.index("\n", j) + 1
    block = t[i:j]
    t = t[:j] + "".join(clone(block, e) for e in NEW) + t[j:]
    a = "  outlineVertices: En1999OutlineVerticesRows | null;\n}"
    assert a in t
    t = t.replace(a, a[:-1] + "".join(f"  /** {e.emoji} @state artifact */\n  {e.camel}: En1999{e.lane}Rows | null;\n" for e in NEW) + "}", 1)
    a = "outlineVertices: normWireDefault(normWireNullable(normWireRef(() => parseEn1999OutlineVerticesRows)), () => null) });"
    assert a in t
    t = t.replace(a, a[:-4] + ", " + ", ".join(f"{e.camel}: normWireDefault(normWireNullable(normWireRef(() => parseEn1999{e.lane}Rows)), () => null)" for e in NEW) + " });", 1)
    t = t.replace("type SectionVertex, parsePlacement, parseWeldRun, parseSectionVertex,", "type SectionVertex, " + ", ".join(f"type {e.struct}" for e in NEW) + ", parsePlacement, parseWeldRun, parseSectionVertex, " + ", ".join(f"parse{e.struct}" for e in NEW) + ",", 1)
    return t


def diff_gql(t):
    if "FatigueCategories" in t:
        return t
    blk = between(t, "type En1999OutlineVerticesPatch {", "type En1999Diff {")
    t = t.replace(blk, blk + "".join(clone(blk, e) for e in NEW), 1)
    a = "  outlineVertices: En1999OutlineVerticesRows @state(class: ARTIFACT)\n"
    assert a in t
    t = t.replace(a, a + "".join(f"  {e.camel}: En1999{e.lane}Rows @state(class: ARTIFACT)\n" for e in NEW), 1)
    return t


def diff_proto(t):
    if "FatigueCategories" in t:
        return t
    blk = between(t, "message En1999OutlineVerticesPatch {", "message En1999Diff {")
    t = t.replace(blk, blk + "".join(clone(blk, e) for e in NEW), 1)
    a = "  optional En1999OutlineVerticesRows outline_vertices = 11;\n"
    assert a in t
    t = t.replace(a, a + "".join(f"  optional En1999{e.lane}Rows {e.field} = {12 + n};\n" for n, e in enumerate(NEW)), 1)
    return t


def diff_json(text):
    d = json.loads(text)
    defs = d["$defs"]
    if "En1999FatigueCategoriesRows" not in defs:
        for e in NEW:
            for suffix in ("Inserted", "Modified", "Moved", "Removed", "Rows", "Patch"):
                defs[f"En1999{e.lane}{suffix}"] = json.loads(clone(json.dumps(defs[f"En1999OutlineVertices{suffix}"], ensure_ascii=False), e))
            d["properties"][e.camel] = json.loads(clone(json.dumps(d["properties"]["outlineVertices"], ensure_ascii=False), e))
    return d


def diff_all():
    DIFF_ = DIFF
    paths = {"rs": DIFF_ / "🦀️.rs", "ts": DIFF_ / "🟦️.ts", "gql": DIFF_ / "🔗️.graphql", "proto": DIFF_ / "🛰️.proto", "json": DIFF_ / "🔣️.json"}
    before = {k: rd(p) for k, p in paths.items()}
    if "FatigueCategoryKey" in before["rs"] and "En1999FatigueCategoriesRows" in before["json"]:
        return
    rs_t, ts_t, gql_t, proto_t = diff_rust(before["rs"]), diff_ts(before["ts"]), diff_gql(before["gql"]), diff_proto(before["proto"])
    jd = diff_json(before["json"])
    ents = [M.Ent(e.struct.lower(), e.emoji, [(e.field, e.struct, e.field + "_id")], e.id_en, e.id_de, slug=e.struct.lower()) for e in NEW]
    leaves, _ = M.read_entities(str(SNAP / "🔣️.json"), ents, set())
    spec = D.specs("En1999", ents, leaves, MODEL)
    rs_t = D.patch_rust(rs_t, "En1999", spec, MODEL, "MembersActions", None)
    for s in spec.values():
        jd["$defs"][s.full] = D.json_patch(s, URL, "En1999")
    ts_t = D.patch_ts(ts_t, "En1999", spec, "MembersActions")
    gql_t = D.patch_blocks(gql_t, "En1999", spec, "MembersActions", "gql")
    proto_t = D.patch_blocks(proto_t, "En1999", spec, "MembersActions", "proto")
    for k, t in (("rs", rs_t), ("ts", ts_t), ("gql", gql_t), ("proto", proto_t), ("json", json.dumps(jd, indent=2, ensure_ascii=False) + "\n")):
        paths[k].write_text(t, encoding="utf8")


FM = ART / "✏️editor/🏷️field-meta/🦀️.rs"
JS = SCHEMA / "🎲️field-metadata.json"
KIND = {"m": ("length", "m"), "rad": ("angle", "rad"), "N": ("force", "N")}


def unit_of(f):
    if f.ty != "f64":
        return "None", None
    u = f.ui.get("unit", "")
    return {"m": ("Some(UnitId::Metre)", "m"), "rad": ("Some(UnitId::Radian)", "rad"), "N": ("Some(UnitId::Newton)", "N"), "": ("None", None)}[u]


def field_meta():
    t = rd(FM)
    if "fatigueCategories[]" not in t:
        anchor = '    ("outlineVertices[].z", '
        i = t.index(anchor)
        i = t.index("\n", i) + 1
        rows = ""
        for e in NEW:
            for f in e.all_fields:
                unit, _ = unit_of(f)
                ch = CHOICES.get(f.snake) if e is not RING_STIFFENER else None
                choices = f"Some({ch})" if ch else "None"
                rows += f'    ("{e.camel}[].{f.camel}", m("{f.en}", "{f.de}", {unit}, {choices})),\n'
        t = t[:i] + rows + t[i:]
        consts = ""
        for name, vals in CHOICE_VALUES.items():
            consts += f"const {name}: &[NormFieldChoice] = &[\n" + "".join(f'    choice("{v}", "{en}", "{de}"),\n' for v, en, de in vals) + "];\n"
        a = "const FABRICATION_CLASS:"
        t = t.replace(a, consts + a, 1)
        wr(FM, t)
    d = json.loads(rd(JS))
    if not any(f["path"].startswith("fatigueCategories[]") for f in d["fields"]):
        for e in NEW:
            for f in e.fields:
                if f.ty == "f64":
                    unit, _ = unit_of(f)
                    if f.ui["unit"] in ("m", "rad"):
                        k = {"m": ("length", "m"), "rad": ("angle", "rad")}[f.ui["unit"]]
                        d["fields"].append({"path": f"{e.camel}[].{f.camel}", "label": {"en": f.en, "de": f.de}, "unit": k[1], "kind": k[0]})
                elif f.ty == "str" and f.snake in CHOICES:
                    d["fields"].append({"path": f"{e.camel}[].{f.camel}", "label": {"en": f.en, "de": f.de}, "kind": "string", "choices": [v for v, *_ in CHOICE_VALUES[CHOICES[f.snake]]]})
        wr(JS, json.dumps(d, indent=2, ensure_ascii=False) + "\n")


SQLD = SQL


def rw(p, fn):
    p = pathlib.Path(p)
    t = p.read_text(encoding="utf8")
    n = fn(t)
    if n != t:
        p.write_text(n, encoding="utf8")


def sql_sql(t):
    if "en1999_fatigue_category" in t:
        return t
    base = "id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1999_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL"
    for e in NEW:
        cols = ", ".join(f"{sql_name(f)} " + ("TEXT NOT NULL" if f.ty == "str" else "REAL" if f.ty == "f64" else f"INTEGER NOT NULL CHECK({f.snake} IN(0,1))") for f in e.fields)
        ieee = ",\n".join(f" {f.snake}_ieee754_bits INTEGER NOT NULL, {f.snake}_ieee754_class TEXT NOT NULL" for f in reals(e))
        t = t.rstrip("\n") + f"\nCREATE TABLE {TABLE[e.field]} (\n {base}, {cols},\n{ieee}\n);\n"
    return t


def rust_cell(f, v):
    return {"str": f"Cell::Text(&{v}.{f.snake})", "f64": f"Cell::Real({v}.{f.snake})", "bool": f"flag({v}.{f.snake})"}[f.ty]


def sqlite_rs(t):
    if "const CATEGORY" in t:
        return t
    t = t.replace("AluminiumShell, Placement, WeldRun, SectionVertex}", "AluminiumShell, Placement, WeldRun, SectionVertex, " + ", ".join(e.struct for e in NEW) + "}", 1)
    consts = "".join(f"const {CONST[e.field]}:&[FloatColumn]=&[{','.join(f'FloatColumn::Binary64({i})' for i in real_cols(e))}];\n" for e in NEW)
    t = t.replace("const VERTEX:", consts + "const VERTEX:", 1)
    writers = ""
    for e in NEW:
        cells = ",".join(["Cell::Integer(1)", "ordinal(n)?", "Cell::Text(&x.id)"] + [rust_cell(f, "x") for f in e.fields])
        writers += f'  for(n,x)in self.{e.field}.iter().enumerate(){{out.insert_float("{TABLE[e.field]}",&[{cells}],{CONST[e.field]})?;}}\n'
    anchor = "        Ok(())\n    }\n    fn admit_sqlite_values"
    assert anchor in t
    t = t.replace(anchor, writers + anchor, 1)
    t = t.replace("SQLITE_SCHEMA,19,36,control", "SQLITE_SCHEMA,22,36,control", 1)
    t = t.replace("self.outline_vertices.len()]{add(n)?;}", "self.outline_vertices.len()," + ",".join(f"self.{e.field}.len()" for e in NEW) + "]{add(n)?;}", 1)
    t = t.replace("admit_rows(&mut b,&mut rows,self.outline_vertices.len())?;", "admit_rows(&mut b,&mut rows,self.outline_vertices.len())?;" + "".join(f"admit_rows(&mut b,&mut rows,self.{e.field}.len())?;" for e in NEW), 1)
    sizes = ""
    for e in NEW:
        strs = ["x.id"] + [f"x.{f.snake}" for f in e.fields if f.ty == "str"]
        sizes += f"  for x in &self.{e.field}{{b.add(8192)?;for t in [{','.join('&' + s for s in strs)}]{{b.repeated(t.len(),24)?;}}}}\n"
    a = "  for v in &self.outline_vertices{b.add(8192)?;for t in [&v.id,&v.section_id,&v.ring,&v.element_id]{b.repeated(t.len(),24)?;}}\n"
    assert a in t
    t = t.replace(a, a + sizes, 1)
    rdr = ""
    for e in NEW:
        def rd_(f, i):
            return {"str": f"text(r,{i},c)?", "f64": f"r.real({i})?", "bool": f"read_flag(r,{i})?"}[f.ty]
        init = ",".join(["id:text(r,3,c)?"] + [f"{f.snake}:{rd_(f, 4 + k)}" for k, f in enumerate(e.fields)])
        rdr += f'  let mut {e.field}=Vec::new();for r in ordered(entities(d,"{TABLE[e.field]}",{4 + len(e.fields)},{CONST[e.field]},c)?.into_values(),1,c)?{{checkpoint(c,{e.field}.len(),0)?;{e.field}.push({e.struct}{{{init}}});}}\n'
    a = "  c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;Ok(Self{"
    assert a in t
    t = t.replace(a, rdr + a, 1)
    t = t.replace("placements,weld_runs,outline_vertices})", "placements,weld_runs,outline_vertices," + ",".join(e.field for e in NEW) + "})", 1)
    return t


def cell_kind(f, i):
    return {"str": f"Text({i})", "f64": f"Real({i})", "bool": f"Boolean({i})"}[f.ty]


def sqlite_admission(t):
    if "en1999_fatigue_category" in t:
        return t
    t = t.replace("for id in 1..=11{", "for id in 1..=14{", 1)
    new = ""
    for n, e in enumerate(NEW):
        spec = ",".join(["Text(0)"] + [cell_kind(f, k + 1) for k, f in enumerate(e.fields)])
        new += f' for(index,x)in n::list(n::field(root,{12 + n})?)?.iter().enumerate(){{n::entity(out,"{TABLE[e.field]}",1,index,&n::cells(n::record(x)?,[{spec}])?,{CONST[e.field]})?;}}\n'
    a = "VERTEX)?;}\n"
    assert t.count(a) == 1
    t = t.replace(a, a + new, 1)
    t = t.replace("SQLITE_SCHEMA,19,36,", "SQLITE_SCHEMA,22,36,")
    return t


def sqlite_ts(t):
    if "CATEGORY" in t:
        return t
    t = t.replace("Placement,WeldRun,SectionVertex}from", "Placement,WeldRun,SectionVertex," + ",".join(e.struct for e in NEW) + "}from", 1)
    types = ""
    for e in NEW:
        names = "|".join(f'"{f.camel}"' for f in reals(e))
        wrapped = ";".join(f"{f.camel}:Binary64" for f in reals(e))
        types += f"export type En1999Sqlite{e.struct}=Omit<{e.struct},{names}>&{{{wrapped}}};\n"
    t = t.replace("export type En1999SqliteSnapshot=", types + "export type En1999SqliteSnapshot=", 1)
    t = t.replace("outlineVertices:En1999SqliteVertex[]}", "outlineVertices:En1999SqliteVertex[];" + ";".join(f"{e.camel}:En1999Sqlite{e.struct}[]" for e in NEW) + "}", 1)
    consts = "".join(f"const {CONST[e.field]}:readonly Ieee754Column[]=[{','.join(str(i) for i in real_cols(e))}].map(index=>({{index,width:64}}));\n" for e in NEW)
    t = t.replace("const VERTEX:", consts + "const VERTEX:", 1)
    names = '"outlineVertices"]);for(const values of[s.materials'
    t = t.replace('"weldRuns","outlineVertices"]);for(const values of[', '"weldRuns","outlineVertices",' + ",".join(f'"{e.camel}"' for e in NEW) + ']);for(const values of[', 1)
    t = t.replace("s.weldRuns,s.outlineVertices])collection(values);", "s.weldRuns,s.outlineVertices," + ",".join(f"s.{e.camel}" for e in NEW) + "])collection(values);", 1)
    w = ""
    for e in NEW:
        fl = ",".join(['"id"'] + [f'"{f.camel}"' for f in e.fields])
        vals = ",".join(["1n", "BigInt(n)", "v.id"] + [f"flag(v.{f.camel})" if f.ty == "bool" else f"v.{f.camel}" for f in e.fields])
        w += f' for(let n=0;n<s.{e.camel}.length;n++){{const v=s.{e.camel}[n]!;fields(v,[{fl}]);await write(out,"{TABLE[e.field]}",[{vals}],{CONST[e.field]},o)}}\n'
    a = " return out.finish();"
    assert a in t
    t = t.replace(a, w + a, 1)
    t = t.replace("outlineVertices:En1999SqliteVertex[]=[];", "outlineVertices:En1999SqliteVertex[]=[]," + ",".join(f"{e.camel}:En1999Sqlite{e.struct}[]=[]" for e in NEW) + ";", 1)
    rdr = ""
    for n, e in enumerate(NEW):
        def rv(f, i):
            return {"str": f"artifactSqliteText(r,{i})", "f64": f"readBinary64(r,{i},{CONST[e.field]})", "bool": f"artifactSqliteBoolean(r,{i})"}[f.ty]
        init = ",".join(["id:artifactSqliteText(r,3)"] + [f"{f.camel}:{rv(f, 4 + k)}" for k, f in enumerate(e.fields)])
        rdr += f" for(const r of await ordered((await entities(t[{19 + n}]!,{4 + len(e.fields)},{CONST[e.field]},o)).values(),1n,o)){{await checkpoint(o,{e.camel}.length);{e.camel}.push({{{init}}})}}\n"
    a = ' await artifactSqliteCheckpoint(o,"reconstructSnapshot",1,1);return{'
    assert a in t
    t = t.replace(a, rdr + a, 1)
    t = t.replace("weldRuns,outlineVertices};", "weldRuns,outlineVertices," + ",".join(e.camel for e in NEW) + "};", 1)
    return t


def tests_rs(t):
    if "fatigue_categories" in t:
        return t
    t = t.replace("assert_eq!(d.tables.len(),19)", "assert_eq!(d.tables.len(),22)", 1)
    t = t.replace("for v in &mut s.outline_vertices{v.y=w;v.z=w;}s}", "for v in &mut s.outline_vertices{v.y=w;v.z=w;}for v in &mut s.fatigue_categories{v.location_x=w;v.m1=w;v.m2=w;}for v in &mut s.patch_loads{v.x=w;v.bearing_length=w;v.web_height=w;v.web_angle=w;v.corner_radius=w;v.load=w;}for v in &mut s.ring_stiffeners{v.x=w;v.depth=w;v.thickness=w;}s}", 1)
    t = t.replace("source.outline_vertices.clear();}", "source.outline_vertices.clear();source.fatigue_categories.clear();source.patch_loads.clear();source.ring_stiffeners.clear();}", 1)
    t = t.replace("max_tables:19,max_columns:36", "max_tables:22,max_columns:36").replace('("tables",19)', '("tables",22)').replace("max_tables:18,..limits", "max_tables:21,..limits")
    return t


def tests_ts(t):
    if "fatigueCategories" in t:
        return t
    t = t.replace("outlineVertices:f.outlineVertices.map(r=>({...r,y:binary64(r.y),z:binary64(r.z)}))}}", "outlineVertices:f.outlineVertices.map(r=>({...r,y:binary64(r.y),z:binary64(r.z)})),fatigueCategories:f.fatigueCategories.map(r=>({...r,locationX:binary64(r.locationX),m1:binary64(r.m1),m2:binary64(r.m2)})),patchLoads:f.patchLoads.map(r=>({...r,x:binary64(r.x),bearingLength:binary64(r.bearingLength),webHeight:binary64(r.webHeight),webAngle:binary64(r.webAngle),cornerRadius:binary64(r.cornerRadius),load:binary64(r.load)})),ringStiffeners:f.ringStiffeners.map(r=>({...r,x:binary64(r.x),depth:binary64(r.depth),thickness:binary64(r.thickness)}))}}", 1)
    t = t.replace("for(const v of s.outlineVertices)Object.assign(v,{y:w,z:w});", "for(const v of s.outlineVertices)Object.assign(v,{y:w,z:w});for(const v of s.fatigueCategories)Object.assign(v,{locationX:w,m1:w,m2:w});for(const v of s.patchLoads)Object.assign(v,{x:w,bearingLength:w,webHeight:w,webAngle:w,cornerRadius:w,load:w});for(const v of s.ringStiffeners)Object.assign(v,{x:w,depth:w,thickness:w});", 1)
    t = t.replace("source.outlineVertices=[];}", "source.outlineVertices=[];source.fatigueCategories=[];source.patchLoads=[];source.ringStiffeners=[];}")
    t = t.replace("maxTables:19,", "maxTables:22,").replace("maxTables:18}", "maxTables:21}")
    return t


SAMPLE = {
    "fatigueCategories": [{"id": "fat-site-1", "memberId": "member", "elementId": "web", "detailCategory": "40", "detailType": "butt-weld", "locationX": 1.5, "m1": 3.4, "m2": 5.4}],
    "patchLoads": [{"id": "patch-1", "subjectId": "member", "position": "end", "x": 0.0, "bearingLength": 0.05, "webHeight": 0.0, "webAngle": 0.0, "cornerRadius": 0.004, "load": 12000.0}],
    "ringStiffeners": [{"id": "ring-1", "shellId": "shell", "x": 1.0, "depth": 0.06, "thickness": 0.006, "internal": False}],
}


def sqlite_fixture():
    p = SQL / "🧪️tests/🧫️fixtures/🔣️.json"
    d = json.loads(rd(p))
    s = d["snapshot"]
    if "patchLoads" not in s:
        s.update(copy.deepcopy(SAMPLE))
        d["tableNames"] += [TABLE[e.field] for e in NEW]
        wr(p, json.dumps(d, indent=2, ensure_ascii=False) + "\n")


def pad_fixtures():
    root = ART / "🧪️tests/🧫️fixtures"
    n = 0
    keys = [e.camel for e in NEW]
    for p in root.rglob("*.json"):
        t = p.read_text(encoding="utf8")
        if "outlineVertices" not in t or "fatigueCategories" in t:
            continue
        d = json.loads(t)

        def walk(o):
            if isinstance(o, dict):
                if "outlineVertices" in o:
                    v = o["outlineVertices"]
                    new = {}
                    for k, x in o.items():
                        new[k] = x
                        if k == "outlineVertices":
                            for kk in keys:
                                new[kk] = [] if isinstance(v, list) else None
                    o.clear()
                    o.update(new)
                for x in list(o.values()):
                    walk(x)
            elif isinstance(o, list):
                for x in o:
                    walk(x)
        walk(d)
        p.write_text(json.dumps(d, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
        n += 1
    return n


def asset_pad():
    n = 0
    for p in (ART / "🖼️assets").rglob("*.json"):
        t = p.read_text(encoding="utf8")
        if "outlineVertices" not in t or "fatigueCategories" in t:
            continue
        d = json.loads(t)
        if "outlineVertices" in d:
            for e in NEW:
                d[e.camel] = []
            p.write_text(json.dumps(d, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
            n += 1
    return n


if __name__ == "__main__":
    model_rs(); root_rs(); snapshot_json(); root_json(); snapshot_ts(); root_ts(); graphql(); proto()
    diff_all()
    field_meta()
    rw(SQLD / "🗄️.sql", sql_sql); rw(SQLD / "🦀️.rs", sqlite_rs); rw(SQLD / "🛂️admission/🦀️.rs", sqlite_admission); rw(SQLD / "🟦️.ts", sqlite_ts)
    rw(SQLD / "🧪️tests/🦀️.rs", tests_rs); rw(SQLD / "🧪️tests/🟦️.ts", tests_ts); sqlite_fixture()
    print("fixtures", pad_fixtures(), "assets", asset_pad())
    print("wave5 entities done")
