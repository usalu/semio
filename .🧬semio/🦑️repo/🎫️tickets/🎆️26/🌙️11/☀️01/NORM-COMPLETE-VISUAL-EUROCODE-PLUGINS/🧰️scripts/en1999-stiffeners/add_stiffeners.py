"""EN 1999 wave 2: adds TransverseStiffener as a snapshot collection after RingStiffener through model, root, five snapshot twins, diff lanes, field-meta, SQLite, fixtures. Idempotent per stage. Honours EN1999_ANY. Usage: EN1999_ANY=<subset root> python -I add_stiffeners.py"""
import copy, json, pathlib, re, sys
HERE = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(HERE.parent / "en1999-wave2"))
sys.path.insert(0, str(HERE.parent / "leaf-kinds"))
from spec_st import *
import s1_model as S1
import diff as D, model as M
from common import compact_json

URL = "https://json.schemas.assets.semio-tech.com/s/norm/en1999/snapshot.json"
MODEL = "semio_s_artifact_norm_en1999_model"
EMOJI = {e.field: e.emoji for e in NEW}
P = PREV


def rd(p):
    return pathlib.Path(p).read_text(encoding="utf8")


def wr(p, t):
    pathlib.Path(p).write_text(t, encoding="utf8")


def after(t, anchor, add, count=1):
    assert anchor in t, "anchor missing: " + anchor[:140]
    return t.replace(anchor, anchor + add, count)


def sub(t, old, new, count=1):
    assert old in t, "anchor missing: " + old[:140]
    return t.replace(old, new, count)


def between(t, a, b):
    i = t.index(a)
    return t[i:t.index(b, i)]


def each(fn):
    return "".join(fn(e) for e in NEW)


def model_rs():
    p = SNAP / "🦀️.rs"
    t = rd(p)
    if "pub struct TransverseStiffener" in t:
        return
    t = sub(t, "//#endregion 🔖️SubjectEntities", "".join(S1.rust_struct(e) for e in NEW) + "//#endregion 🔖️SubjectEntities")
    t = after(t, "    #[state(artifact)]\n    #[dsl(table)]\n    pub ring_stiffeners: Vec<RingStiffener>,\n", each(lambda e: f"    #[state(artifact)]\n    #[dsl(table)]\n    pub {e.field}: Vec<{e.struct}>,\n"))
    t = sub(t, "            ring_stiffeners: Vec::new(),\n", "            ring_stiffeners: Vec::new(),\n" + each(lambda e: f"            {e.field}: Vec::new(),\n"), t.count("            ring_stiffeners: Vec::new(),\n"))
    wr(p, t)


def root_rs():
    p = SCHEMA / "🦀️.rs"
    t = rd(p)
    if "TransverseStiffener" in t:
        return
    t = sub(t, "PatchLoad, RingStiffener,", "PatchLoad, RingStiffener, " + ", ".join(e.struct for e in NEW) + ",")
    t = after(t, "    #[state(artifact)]\n    pub ring_stiffeners: Vec<RingStiffener>,\n", each(lambda e: f"    #[state(artifact)]\n    pub {e.field}: Vec<{e.struct}>,\n"))
    t = after(t, "            ring_stiffeners: self.ring_stiffeners.clone(),\n", each(lambda e: f"            {e.field}: self.{e.field}.clone(),\n"))
    t = after(t, "            ring_stiffeners: snapshot.ring_stiffeners,\n", each(lambda e: f"            {e.field}: snapshot.{e.field},\n"))
    wr(p, t)


def snapshot_json():
    p = SNAP / "🔣️.json"
    d = json.loads(rd(p))
    if "TransverseStiffener" in d["$defs"]:
        return
    for e in NEW:
        d["required"].append(e.camel)
        d["properties"][e.camel] = {"type": "array", "items": {"$ref": f"#/$defs/{e.struct}"}, "x-semio-state": "artifact"}
        d["$defs"][e.struct] = S1.json_def(e)
    wr(p, json.dumps(d, indent=2, ensure_ascii=False) + "\n")


def root_json():
    p = SCHEMA / "🔣️.json"
    d = json.loads(rd(p))
    if "transverseStiffeners" in d["properties"]:
        return
    for e in NEW:
        d["required"].append(e.camel)
        props = {"id": {"type": "string"}, **{f.camel: {"type": f.json} for f in e.fields}}
        d["properties"][e.camel] = {"type": "array", "items": {"type": "object", "additionalProperties": False, "required": list(props), "properties": props}, "x-semio-state": "artifact"}
    wr(p, json.dumps(d, indent=2, ensure_ascii=False) + "\n")


def snapshot_ts():
    p = SNAP / "🟦️.ts"
    t = rd(p)
    if "export interface TransverseStiffener" in t:
        return
    t = after(t, "  ringStiffeners: RingStiffener[];\n", each(lambda e: f"  /** {e.emoji} @state artifact */\n  {e.camel}: {e.struct}[];\n"))
    i = t.index("export interface RingStiffener {")
    j = t.index("}\n\n", i) + 3
    t = t[:j] + each(S1.ts_iface) + t[j:]
    a = "ringStiffeners: normWireRequired(normWireArray(normWireRef(() => parseRingStiffener)))"
    assert a in t
    t = t.replace(a, a + ", " + ", ".join(f"{e.camel}: normWireRequired(normWireArray(normWireRef(() => parse{e.struct})))" for e in NEW), 1)
    k = t.index("export const parseRingStiffener:")
    k = t.index("\n", k) + 1
    t = t[:k] + each(S1.ts_parser) + t[k:]
    wr(p, t)


def inline_ts(e):
    return "{ id: string; " + " ".join(f"{f.camel}: {f.ts};" for f in e.fields) + " }"


def root_ts():
    p = SCHEMA / "🟦️.ts"
    t = rd(p)
    if "transverseStiffeners:" in t:
        return
    anchor = "  ringStiffeners: { id: string; shellId: string; x: number; depth: number; thickness: number; internal: boolean; }[];\n"
    t = after(t, anchor, each(lambda e: f"  /** {e.emoji} @state artifact */\n  {e.camel}: {inline_ts(e)}[];\n"))
    reader = lambda e: f"{e.camel}: normWireRequired(normWireArray(normWireObject<{inline_ts(e)}>({{ " + ", ".join(["id: normWireRequired(normWireString)"] + [f"{f.camel}: normWireRequired({f.wire})" for f in e.fields]) + " })))"
    tail = "internal: normWireRequired(normWireBoolean) }))) });"
    assert t.count(tail) == 1
    t = t.replace(tail, "internal: normWireRequired(normWireBoolean) }))), " + ", ".join(reader(e) for e in NEW) + " });", 1)
    wr(p, t)


def graphql():
    for p in (SCHEMA / "🔗️.graphql", SNAP / "🔗️.graphql"):
        t = rd(p)
        if "TransverseStiffener" in t:
            continue
        t = after(t, "  ringStiffeners: [RingStiffener!]! @state(artifact)\n", each(lambda e: f"  {e.camel}: [{e.struct}!]! @state(artifact)\n"))
        i = t.index("type RingStiffener {")
        i = t.index("\n", i) + 1
        t = t[:i] + each(S1.gql_type) + t[i:]
        wr(p, t)


def proto():
    for p in (SCHEMA / "🛰️.proto", SNAP / "🛰️.proto"):
        t = rd(p)
        if "TransverseStiffener" in t:
            continue
        m = re.search(r"  repeated RingStiffener ring_stiffeners = (\d+);\n", t)
        t = after(t, m.group(0), "".join(f"  repeated {e.struct} {e.field} = {int(m.group(1)) + 1 + n};\n" for n, e in enumerate(NEW)))
        i = t.index("message RingStiffener {")
        i = t.index("\n", i) + 1
        t = t[:i] + each(S1.proto_msg) + t[i:]
        wr(p, t)


def diff_rust(t):
    if "TransverseStiffenerKey" in t:
        return t
    key = between(t, "/// 🔑️ Key extractor of the foreign `RingStiffener` rows", "\n\nuse protocol::list_delta::Keyed as _;")
    t = t.replace(key, key + "".join("\n" + clone_prev(key, e) for e in NEW), 1)
    s = t.rindex("/// 🩹️ Sparse patch of the", 0, t.index("pub struct En1999RingStiffenersPatch"))
    e_ = t.index("\n}\n", t.index("pub En1999RingStiffenersRows {", s)) + 3
    lane = t[s:e_]
    t = t.replace(lane, lane + "".join(clone_prev(lane, e) for e in NEW), 1)
    t = after(t, "    pub ring_stiffeners: Option<En1999RingStiffenersRows>,\n", each(lambda e: f"    #[state(artifact)]\n    pub {e.field}: Option<En1999{e.lane}Rows>,\n"))
    ab = between(t, "        if let Some(theirs) = other.ring_stiffeners {", "    }\n\n    fn apply(")
    t = t.replace(ab, ab + each(lambda e: clone_prev(ab, e)), 1)
    ap = between(t, "        if let Some(rows) = &self.ring_stiffeners {", "        *unpublished = next;")
    t = t.replace(ap, ap + each(lambda e: clone_prev(ap, e)), 1)
    a = ".ring_stiffeners.as_ref().map_or(true, |rows| rows.is_empty())"
    assert t.count(a) == 1
    t = t.replace(a, a + "".join(f" && self.{e.field}.as_ref().map_or(true, |rows| rows.is_empty())" for e in NEW), 1)
    inv = "            ring_stiffeners: self.ring_stiffeners.as_ref().map(|rows| rows.inverse(&base.ring_stiffeners)).filter(|rows| !rows.is_empty()),\n"
    t = after(t, inv, each(lambda e: clone_prev(inv, e)))
    return t


def diff_ts(t):
    if "TransverseStiffeners" in t:
        return t
    ifaces = between(t, "export interface En1999RingStiffenersPatch {", "export const parseEn1999Diff:")
    t = t.replace(ifaces, ifaces + each(lambda e: clone_prev(ifaces, e)), 1)
    i = t.index("export const parseEn1999RingStiffenersInserted:")
    j = t.index("export const parseEn1999RingStiffenersPatch:")
    j = t.index("\n", j) + 1
    block = t[i:j]
    t = t[:j] + each(lambda e: clone_prev(block, e)) + t[j:]
    a = "  ringStiffeners: En1999RingStiffenersRows | null;\n"
    t = after(t, a, each(lambda e: f"  /** {e.emoji} @state artifact */\n  {e.camel}: En1999{e.lane}Rows | null;\n"))
    a = "ringStiffeners: normWireDefault(normWireNullable(normWireRef(() => parseEn1999RingStiffenersRows)), () => null)"
    t = after(t, a, "".join(f", {e.camel}: normWireDefault(normWireNullable(normWireRef(() => parseEn1999{e.lane}Rows)), () => null)" for e in NEW))
    t = sub(t, "type RingStiffener, parsePlacement,", "type RingStiffener, " + ", ".join(f"type {e.struct}" for e in NEW) + ", parsePlacement,")
    t = sub(t, "parseRingStiffener,", "parseRingStiffener, " + ", ".join(f"parse{e.struct}" for e in NEW) + ",")
    return t


def diff_gql(t):
    if "TransverseStiffeners" in t:
        return t
    blk = between(t, "type En1999RingStiffenersPatch {", "type En1999Diff {")
    t = t.replace(blk, blk + each(lambda e: clone_prev(blk, e)), 1)
    return after(t, "  ringStiffeners: En1999RingStiffenersRows @state(class: ARTIFACT)\n", each(lambda e: f"  {e.camel}: En1999{e.lane}Rows @state(class: ARTIFACT)\n"))


def diff_proto(t):
    if "TransverseStiffeners" in t:
        return t
    blk = between(t, "message En1999RingStiffenersPatch {", "message En1999Diff {")
    t = t.replace(blk, blk + each(lambda e: clone_prev(blk, e)), 1)
    m = re.search(r"  optional En1999RingStiffenersRows ring_stiffeners = (\d+);\n", t)
    return after(t, m.group(0), "".join(f"  optional En1999{e.lane}Rows {e.field} = {int(m.group(1)) + 1 + n};\n" for n, e in enumerate(NEW)))


def diff_json(text):
    d = json.loads(text)
    defs = d["$defs"]
    if f"En1999{NEW[0].lane}Rows" not in defs:
        for e in NEW:
            for suffix in ("Inserted", "Modified", "Moved", "Removed", "Rows", "Patch"):
                defs[f"En1999{e.lane}{suffix}"] = json.loads(clone_prev(json.dumps(defs[f"En1999{P['lane']}{suffix}"], ensure_ascii=False), e))
            d["properties"][e.camel] = json.loads(clone_prev(json.dumps(d["properties"][P["camel"]], ensure_ascii=False), e))
    return d


def diff_all():
    paths = {"rs": DIFF / "🦀️.rs", "ts": DIFF / "🟦️.ts", "gql": DIFF / "🔗️.graphql", "proto": DIFF / "🛰️.proto", "json": DIFF / "🔣️.json"}
    before = {k: rd(p) for k, p in paths.items()}
    if "TransverseStiffenerKey" in before["rs"] and f"En1999{NEW[0].lane}Rows" in before["json"]:
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
    for k, t in (("rs", rs_t), ("ts", ts_t), ("gql", gql_t), ("proto", proto_t), ("json", compact_json(jd))):
        paths[k].write_text(t, encoding="utf8")


FM = ART / "✏️editor/🏷️field-meta/🦀️.rs"
JS = SCHEMA / "🎲️field-metadata.json"


def unit_of(f):
    if f.ty != "f64":
        return "None"
    return {"m": "Some(UnitId::Metre)", "rad": "Some(UnitId::Radian)", "N": "Some(UnitId::Newton)", "": "None"}[f.ui.get("unit", "")]


def field_meta():
    t = rd(FM)
    if "transverseStiffeners[]" not in t:
        anchor = '    ("ringStiffeners[].internal", '
        i = t.index(anchor)
        i = t.index("\n", i) + 1
        rows = ""
        for e in NEW:
            for f in e.all_fields:
                ch = choice_of(e, f)
                rows += f'    ("{e.camel}[].{f.camel}", m("{f.en}", "{f.de}", {unit_of(f)}, {"Some(" + ch + ")" if ch else "None"})),\n'
        t = t[:i] + rows + t[i:]
        consts = ""
        for name, vals in CHOICE_VALUES.items():
            consts += f"const {name}: &[NormFieldChoice] = &[\n" + "".join(f'    choice("{v}", "{en}", "{de}"),\n' for v, en, de in vals) + "];\n"
        t = sub(t, "const FABRICATION_CLASS:", consts + "const FABRICATION_CLASS:")
        wr(FM, t)
    d = json.loads(rd(JS))
    if not any(f["path"].startswith("transverseStiffeners[]") for f in d["fields"]):
        for e in NEW:
            for f in e.fields:
                if f.ty == "f64" and f.ui.get("unit") in ("m", "rad"):
                    k = {"m": ("length", "m"), "rad": ("angle", "rad")}[f.ui["unit"]]
                    d["fields"].append({"path": f"{e.camel}[].{f.camel}", "label": {"en": f.en, "de": f.de}, "unit": k[1], "kind": k[0]})
                elif f.ty == "str" and choice_of(e, f):
                    d["fields"].append({"path": f"{e.camel}[].{f.camel}", "label": {"en": f.en, "de": f.de}, "kind": "string", "choices": [v for v, *_ in CHOICE_VALUES[choice_of(e, f)]]})
        wr(JS, json.dumps(d, indent=2, ensure_ascii=False) + "\n")


def rw(p, fn):
    p = pathlib.Path(p)
    t = p.read_text(encoding="utf8")
    n = fn(t)
    if n != t:
        p.write_text(n, encoding="utf8")


def sql_sql(t):
    if "en1999_transverse_stiffener" in t:
        return t
    base = "id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1999_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL"
    for e in NEW:
        cols = ", ".join(f"{sql_name(f)} " + ("TEXT NOT NULL" if f.ty == "str" else "REAL" if f.ty == "f64" else f"INTEGER NOT NULL CHECK({f.snake} IN(0,1))") for f in e.fields)
        ieee = ",\n".join(f" {f.snake}_ieee754_bits INTEGER NOT NULL, {f.snake}_ieee754_class TEXT NOT NULL" for f in reals(e))
        t = t.rstrip("\n") + f"\nCREATE TABLE {TABLE[e.field]} (\n {base}, {cols},\n{ieee}\n);\n"
    return t


def rust_cell(f, v):
    return {"str": f"Cell::Text(&{v}.{f.snake})", "f64": f"Cell::Real({v}.{f.snake})", "bool": f"flag({v}.{f.snake})"}[f.ty]


def schema_tables(t):
    m = re.search(r"SQLITE_SCHEMA,(\d+),36", t)
    return int(m.group(1))


def sqlite_rs(t):
    if "const STIFFENER" in t:
        return t
    n_tables = schema_tables(t)
    t = sub(t, "PatchLoad, RingStiffener}", "PatchLoad, RingStiffener, " + ", ".join(e.struct for e in NEW) + "}")
    consts = "".join(f"const {CONST[e.field]}:&[FloatColumn]=&[{','.join(f'FloatColumn::Binary64({i})' for i in real_cols(e))}];\n" for e in NEW)
    t = sub(t, "const VERTEX:", consts + "const VERTEX:")
    writers = ""
    for e in NEW:
        cells = ",".join(["Cell::Integer(1)", "ordinal(n)?", "Cell::Text(&x.id)"] + [rust_cell(f, "x") for f in e.fields])
        writers += f'  for(n,x)in self.{e.field}.iter().enumerate(){{out.insert_float("{TABLE[e.field]}",&[{cells}],{CONST[e.field]})?;}}\n'
    anchor = "        Ok(())\n    }\n    fn admit_sqlite_values"
    t = sub(t, anchor, writers + anchor)
    t = sub(t, f"SQLITE_SCHEMA,{n_tables},36,control", f"SQLITE_SCHEMA,{n_tables + len(NEW)},36,control")
    t = sub(t, "self.ring_stiffeners.len()]{add(n)?;}", "self.ring_stiffeners.len()," + ",".join(f"self.{e.field}.len()" for e in NEW) + "]{add(n)?;}")
    t = sub(t, "admit_rows(&mut b,&mut rows,self.ring_stiffeners.len())?;", "admit_rows(&mut b,&mut rows,self.ring_stiffeners.len())?;" + "".join(f"admit_rows(&mut b,&mut rows,self.{e.field}.len())?;" for e in NEW))
    sizes = ""
    for e in NEW:
        strs = ["x.id"] + [f"x.{f.snake}" for f in e.fields if f.ty == "str"]
        sizes += f"  for x in &self.{e.field}{{b.add(8192)?;for t in [{','.join('&' + s for s in strs)}]{{b.repeated(t.len(),24)?;}}}}\n"
    t = after(t, "  for x in &self.ring_stiffeners{b.add(8192)?;for t in [&x.id,&x.shell_id]{b.repeated(t.len(),24)?;}}\n", sizes)
    rdr = ""
    for e in NEW:
        rd_ = lambda f, i: {"str": f"text(r,{i},c)?", "f64": f"r.real({i})?", "bool": f"read_flag(r,{i})?"}[f.ty]
        init = ",".join(["id:text(r,3,c)?"] + [f"{f.snake}:{rd_(f, 4 + k)}" for k, f in enumerate(e.fields)])
        rdr += f'  let mut {e.field}=Vec::new();for r in ordered(entities(d,"{TABLE[e.field]}",{4 + len(e.fields)},{CONST[e.field]},c)?.into_values(),1,c)?{{checkpoint(c,{e.field}.len(),0)?;{e.field}.push({e.struct}{{{init}}});}}\n'
    t = sub(t, "  c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;Ok(Self{", rdr + "  c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;Ok(Self{")
    t = sub(t, "patch_loads,ring_stiffeners})", "patch_loads,ring_stiffeners," + ",".join(e.field for e in NEW) + "})")
    return t


def cell_kind(f, i):
    return {"str": f"Text({i})", "f64": f"Real({i})", "bool": f"Boolean({i})"}[f.ty]


def sqlite_admission(t):
    if "en1999_transverse_stiffener" in t:
        return t
    n_tables = schema_tables(t)
    m = re.search(r"for id in 1\.\.=(\d+)\{", t)
    last = int(m.group(1))
    t = sub(t, m.group(0), f"for id in 1..={last + len(NEW)}{{")
    new = ""
    for n, e in enumerate(NEW):
        spec = ",".join(["Text(0)"] + [cell_kind(f, k + 1) for k, f in enumerate(e.fields)])
        new += f' for(index,x)in n::list(n::field(root,{last + 1 + n})?)?.iter().enumerate(){{n::entity(out,"{TABLE[e.field]}",1,index,&n::cells(n::record(x)?,[{spec}])?,{CONST[e.field]})?;}}\n'
    anchor = ',RING)?;}\n'
    assert t.count(anchor) == 1
    t = t.replace(anchor, anchor + new, 1)
    return sub(t, f"SQLITE_SCHEMA,{n_tables},36,", f"SQLITE_SCHEMA,{n_tables + len(NEW)},36,")


def sqlite_ts(t):
    if "STIFFENER" in t:
        return t
    t = sub(t, "PatchLoad,RingStiffener}from", "PatchLoad,RingStiffener," + ",".join(e.struct for e in NEW) + "}from")
    types = ""
    for e in NEW:
        names = "|".join(f'"{f.camel}"' for f in reals(e))
        wrapped = ";".join(f"{f.camel}:Binary64" for f in reals(e))
        types += f"export type En1999Sqlite{e.struct}=Omit<{e.struct},{names}>&{{{wrapped}}};\n"
    t = sub(t, "export type En1999SqliteSnapshot=", types + "export type En1999SqliteSnapshot=")
    t = sub(t, "ringStiffeners:En1999SqliteRingStiffener[]};", "ringStiffeners:En1999SqliteRingStiffener[];" + ";".join(f"{e.camel}:En1999Sqlite{e.struct}[]" for e in NEW) + "};")
    consts = "".join(f"const {CONST[e.field]}:readonly Ieee754Column[]=[{','.join(str(i) for i in real_cols(e))}].map(index=>({{index,width:64}}));\n" for e in NEW)
    t = sub(t, "const VERTEX:", consts + "const VERTEX:")
    t = sub(t, '"patchLoads","ringStiffeners"]);', '"patchLoads","ringStiffeners",' + ",".join(f'"{e.camel}"' for e in NEW) + "]);")
    t = sub(t, "s.patchLoads,s.ringStiffeners])collection(values);", "s.patchLoads,s.ringStiffeners," + ",".join(f"s.{e.camel}" for e in NEW) + "])collection(values);")
    w = ""
    for e in NEW:
        fl = ",".join(['"id"'] + [f'"{f.camel}"' for f in e.fields])
        vals = ",".join(["1n", "BigInt(n)", "v.id"] + [f"flag(v.{f.camel})" if f.ty == "bool" else f"v.{f.camel}" for f in e.fields])
        w += f' for(let n=0;n<s.{e.camel}.length;n++){{const v=s.{e.camel}[n]!;fields(v,[{fl}]);await write(out,"{TABLE[e.field]}",[{vals}],{CONST[e.field]},o)}}\n'
    t = sub(t, " return out.finish();", w + " return out.finish();")
    t = sub(t, "ringStiffeners:En1999SqliteRingStiffener[]=[];", "ringStiffeners:En1999SqliteRingStiffener[]=[]," + ",".join(f"{e.camel}:En1999Sqlite{e.struct}[]=[]" for e in NEW) + ";")
    ix = int(re.search(r"entities\(t\[(\d+)\]!,9,RING,o\)", t).group(1)) + 1
    rdr = ""
    for n, e in enumerate(NEW):
        rv = lambda f, i: {"str": f"artifactSqliteText(r,{i})", "f64": f"readBinary64(r,{i},{CONST[e.field]})", "bool": f"artifactSqliteBoolean(r,{i})"}[f.ty]
        init = ",".join(["id:artifactSqliteText(r,3)"] + [f"{f.camel}:{rv(f, 4 + k)}" for k, f in enumerate(e.fields)])
        rdr += f" for(const r of await ordered((await entities(t[{ix + n}]!,{4 + len(e.fields)},{CONST[e.field]},o)).values(),1n,o)){{await checkpoint(o,{e.camel}.length);{e.camel}.push({{{init}}})}}\n"
    t = sub(t, ' await artifactSqliteCheckpoint(o,"reconstructSnapshot",1,1);return{', rdr + ' await artifactSqliteCheckpoint(o,"reconstructSnapshot",1,1);return{')
    return sub(t, "patchLoads,ringStiffeners};", "patchLoads,ringStiffeners," + ",".join(e.camel for e in NEW) + "};")


def tests_rs(t):
    if "transverse_stiffeners" in t:
        return t
    n = int(re.search(r"assert_eq!\(d\.tables\.len\(\),(\d+)\)", t).group(1))
    t = t.replace(f"assert_eq!(d.tables.len(),{n})", f"assert_eq!(d.tables.len(),{n + 1})")
    t = sub(t, "for v in &mut s.ring_stiffeners{v.x=w;v.depth=w;v.thickness=w;}", "for v in &mut s.ring_stiffeners{v.x=w;v.depth=w;v.thickness=w;}for v in &mut s.transverse_stiffeners{v.position=w;v.width=w;v.thickness=w;}")
    t = sub(t, "source.ring_stiffeners.clear();}", "source.ring_stiffeners.clear();source.transverse_stiffeners.clear();}")
    t = t.replace(f"max_tables:{n}", f"max_tables:{n + 1}").replace(f'("tables",{n})', f'("tables",{n + 1})').replace(f"max_tables:{n - 1},", f"max_tables:{n},")
    t = sub(t, "all_ninety_three_fields", "all_ninety_six_fields")
    t = sub(t, "if(count!==93)throw Error('all93scalars')", "if(count!==96)throw Error('all96scalars')")
    return t


def tests_ts(t):
    if "transverseStiffeners" in t:
        return t
    n = int(re.search(r"maxTables:(\d+),", t).group(1))
    t = sub(t, "ringStiffeners:f.ringStiffeners.map(r=>({...r,x:binary64(r.x),depth:binary64(r.depth),thickness:binary64(r.thickness)}))}}", "ringStiffeners:f.ringStiffeners.map(r=>({...r,x:binary64(r.x),depth:binary64(r.depth),thickness:binary64(r.thickness)})),transverseStiffeners:f.transverseStiffeners.map(r=>({...r,position:binary64(r.position),width:binary64(r.width),thickness:binary64(r.thickness)}))}}")
    t = sub(t, "for(const v of s.ringStiffeners)Object.assign(v,{x:w,depth:w,thickness:w});", "for(const v of s.ringStiffeners)Object.assign(v,{x:w,depth:w,thickness:w});for(const v of s.transverseStiffeners)Object.assign(v,{position:w,width:w,thickness:w});")
    t = sub(t, "source.ringStiffeners=[];}", "source.ringStiffeners=[];source.transverseStiffeners=[];}")
    t = t.replace(f"maxTables:{n},", f"maxTables:{n + 1},").replace(f"maxTables:{n - 1}", f"maxTables:{n}")
    t = sub(t, "ninety-three floating fields", "ninety-six floating fields")
    return t


def sqlite_fixture():
    p = SQL / "🧪️tests/🧫️fixtures/🔣️.json"
    d = json.loads(rd(p))
    s = d["snapshot"]
    if "transverseStiffeners" not in s:
        s.update(copy.deepcopy(SAMPLE))
        d["tableNames"] += [TABLE[e.field] for e in NEW]
        wr(p, json.dumps(d, indent=2, ensure_ascii=False) + "\n")


def pad_fixtures():
    root = ART / "🧪️tests/🧫️fixtures"
    n = 0
    keys = [e.camel for e in NEW]
    for p in root.rglob("*.json"):
        t = p.read_text(encoding="utf8")
        if "ringStiffeners" not in t or "transverseStiffeners" in t:
            continue
        d = json.loads(t)

        def walk(o):
            if isinstance(o, dict):
                if "ringStiffeners" in o:
                    v = o["ringStiffeners"]
                    new = {}
                    for k, x in o.items():
                        new[k] = x
                        if k == "ringStiffeners":
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
        if "ringStiffeners" not in t or "transverseStiffeners" in t:
            continue
        d = json.loads(t)
        if "ringStiffeners" in d:
            new = {}
            for k, x in d.items():
                new[k] = x
                if k == "ringStiffeners":
                    for e in NEW:
                        new[e.camel] = []
            p.write_text(json.dumps(new, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
            n += 1
    return n


if __name__ == "__main__":
    model_rs(); root_rs(); snapshot_json(); root_json(); snapshot_ts(); root_ts(); graphql(); proto()
    diff_all()
    field_meta()
    rw(SQL / "🗄️.sql", sql_sql); rw(SQL / "🦀️.rs", sqlite_rs); rw(SQL / "🛂️admission/🦀️.rs", sqlite_admission); rw(SQL / "🟦️.ts", sqlite_ts)
    rw(SQL / "🧪️tests/🦀️.rs", tests_rs); rw(SQL / "🧪️tests/🟦️.ts", tests_ts); sqlite_fixture()
    print("fixtures", pad_fixtures(), "assets", asset_pad())
    print("stiffener schema done")
