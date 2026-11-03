import sys, re
sys.path.insert(0, ".")
from spec import *
Q = S + "/🚪️io/🪶️sqlite/📸️snapshot"

class T:
    def __init__(self, e, table, parent_table, const, var, nested_of=None):
        self.e, self.table, self.parent, self.const, self.var = e, table, parent_table, const, var
        self.cols = [("id", "s")] + [(f.name, f.kind) for f in e.fields]  # id -> logical_id
        self.children = []

tabs = []
for e in ENTITIES:
    t = T(e, "en1997_" + e.coll[:-1], "document", e.struct.upper(), e.coll)
    tabs.append(t)
    if e.nested:
        n = e.nested[2]
        c = T(n, "en1997_" + n.struct_snake if hasattr(n, "struct_snake") else "en1997_" + re.sub(r"(?<!^)([A-Z])", r"_\1", n.struct).lower(), t.table, n.struct.upper(), e.nested[0])
        c.parent_ent = t
        t.children.append(c)
        tabs.append(c)
for t in tabs:
    t.name = t.table.replace("en1997_", "")

def sql_table(t):
    pk = "document_id INTEGER NOT NULL REFERENCES en1997_document(id)" if t.parent == "document" else f"{t.parent.replace('en1997_', '')}_id INTEGER NOT NULL REFERENCES {t.parent}(id)"
    cols = []
    for n, k in t.cols:
        cn = "logical_id" if n == "id" else n
        if k == "s": cols.append(f"{cn} TEXT NOT NULL")
        elif k == "u": cols.append(f"{cn} INTEGER NOT NULL CHECK({cn} BETWEEN 0 AND 4294967295)")
        else: cols.append(f"{cn} REAL")
    ieee = [f" {n}_ieee754_bits INTEGER NOT NULL, {n}_ieee754_class TEXT NOT NULL" for n, k in t.cols if k == "n"]
    return f"CREATE TABLE {t.table} (\n id INTEGER PRIMARY KEY, {pk}, ordinal INTEGER NOT NULL CHECK(ordinal>=0), " + ", ".join(cols) + ",\n" + ",\n".join(ieee) + "\n);\n"

def floats(t):
    # index: 0 id,1 parent,2 ordinal,3 logical,4.. data in order after logical_id (cols[0] is logical)
    idx = []
    for i, (n, k) in enumerate(t.cols):
        if k == "n": idx.append(3 + i)
    return idx

p = Q + "/🗄️.sql"
s = open(p, encoding="utf8").read()
if "en1997_anchor" not in s:
    s = s.rstrip("\n") + "\n" + "".join(sql_table(t) for t in tabs)
    open(p, "w", encoding="utf8").write(s)
SQL_LEN_NEW = s

# Rust codec
p = Q + "/🦀️.rs"
s = open(p, encoding="utf8").read()
if "ANCHOR:" not in s:
    s = s.replace("use semio_s_artifact_norm_en1997_model::{SoilLayer, SpreadFoundation, FoundationLoadCase, Pile, PileTestProfile, RetainingWall, Slope, UpliftCase};", "use semio_s_artifact_norm_en1997_model::{SoilLayer, SpreadFoundation, FoundationLoadCase, Pile, PileTestProfile, RetainingWall, Slope, UpliftCase, Anchor, AnchorTest, EmbeddedWall, PileGroup, LabTest, LabPoint, LayerWater};")
    consts = "".join(f"const {t.const}:&[FloatColumn]=&[{','.join(f'FloatColumn::Binary64({i})' for i in floats(t))}];\n" for t in tabs)
    s = s.replace("fn ordinal(n:usize)", consts + "fn ordinal(n:usize)", 1)
    def cell(k, expr):
        return f"Cell::Text(&{expr})" if k == "s" else (f"Cell::Integer(i64::from({expr}))" if k == "u" else f"Cell::Real({expr})")
    writes = ""
    for t in tabs:
        if t.parent == "document":
            row = f"Cell::Integer(1),ordinal(n)?,Cell::Text(&v.id)," + ",".join(cell(k, f"v.{n}") for n, k in t.cols[1:])
            kids = t.children
            if kids:
                c = kids[0]
                crow = "Cell::Integer(id),ordinal(k)?,Cell::Text(&a.id)," + ",".join(cell(k, f"a.{n}") for n, k in c.cols[1:])
                writes += f'  for(n,v)in self.{t.var}.iter().enumerate(){{let id=p.insert_float("{t.table}",&[{row}],{t.const})?;for(k,a)in v.{c.var}.iter().enumerate(){{p.insert_float("{c.table}",&[{crow}],{c.const})?;}}}}\n'
            else:
                writes += f'  for(n,v)in self.{t.var}.iter().enumerate(){{p.insert_float("{t.table}",&[{row}],{t.const})?;}}\n'
    marker = "  Ok(())\n }\n fn admit_sqlite_values"
    assert marker in s
    s = s.replace(marker, writes + marker, 1)
    # schema counts
    s = s.replace("En1997Snapshot::SQLITE_SCHEMA,9,42,c)", f"En1997Snapshot::SQLITE_SCHEMA,{9 + len(tabs)},{42 + sum(len(t.cols) for t in tabs)},c)")
    # reader
    def rd(k, n, i, v):
        if k == "s": return f"{n}:text({v},{i},c)?"
        if k == "u": return f"{n}:u32::try_from({v}.integer({i})?).map_err(|e| invalid(e.to_string()))?"
        return f"{n}:{v}.real({i})?"
    reads = ""
    for t in tabs:
        if t.parent != "document": continue
        cnt = 4 + len(t.cols) - 1
        fields = ",".join(["id:text(v,3,c)?"] + [rd(k, n, 4 + i, "v") for i, (n, k) in enumerate(t.cols[1:])])
        if t.children:
            c = t.children[0]
            ccnt = 4 + len(c.cols) - 1
            cf = ",".join(["id:text(a,3,c)?"] + [rd(k, n, 4 + i, "a") for i, (n, k) in enumerate(c.cols[1:])])
            reads += f'  let {t.var}_rows=entities(d,"{t.table}",{cnt},{t.const},c)?;let mut {c.var}_{t.var}=groups(entities(d,"{c.table}",{ccnt},{c.const},c)?,&{t.var}_rows,c)?;\n'
            reads += f'  let mut {t.var}=Vec::new();for v in ordered({t.var}_rows.into_values(),1,c)?{{let mut {c.var}=Vec::new();for a in ordered({c.var}_{t.var}.remove(&v.rowid).unwrap_or_default(),v.rowid,c)?{{checkpoint(c,{c.var}.len(),0)?;{c.var}.push({c.e.struct}{{{cf}}});}}{t.var}.push({t.e.struct}{{{fields},{c.var}}});}}\n'
        else:
            reads += f'  let mut {t.var}=Vec::new();for v in ordered(entities(d,"{t.table}",{cnt},{t.const},c)?.into_values(),1,c)?{{checkpoint(c,{t.var}.len(),0)?;{t.var}.push({t.e.struct}{{{fields}}});}}\n'
    marker = "  let result=Self{structure_id:"
    assert marker in s
    s = s.replace(marker, reads + marker, 1)
    s = s.replace("slopes,uplift_cases};c.checkpoint", "slopes,uplift_cases," + ",".join(e.coll for e in ENTITIES) + "};c.checkpoint")
    # preflight
    pre_lists = ",".join(f"self.{e.coll}.len()" for e in ENTITIES)
    s = s.replace("self.uplift_cases.len()]{admit_rows(&mut b,&mut rows,count)?;}", "self.uplift_cases.len(),"+pre_lists+"]{admit_rows(&mut b,&mut rows,count)?;}")
    s = s.replace("for v in &self.piles{admit_rows(&mut b,&mut rows,v.test_profiles.len())?;}", "for v in &self.piles{admit_rows(&mut b,&mut rows,v.test_profiles.len())?;}for v in &self.anchors{admit_rows(&mut b,&mut rows,v.tests.len())?;}for v in &self.lab_tests{admit_rows(&mut b,&mut rows,v.points.len())?;}")
    bounds = "".join(f"for v in &self.{e.coll}{{b.add(32768)?;b.repeated(v.id.len(),24)?;" + "".join(f"b.repeated(v.{f.name}.len(),24)?;" for f in e.fields if f.kind == "s") + (f"for a in &v.{e.nested[0]}{{b.add(8192)?;b.repeated(a.id.len(),24)?;}}" if e.nested else "") + "}" for e in ENTITIES)
    s = s.replace("for v in &self.uplift_cases{b.add(16384)?;b.repeated(v.id.len(),24)?;}b.finish()", "for v in &self.uplift_cases{b.add(16384)?;b.repeated(v.id.len(),24)?;}" + bounds + "b.finish()")
    open(p, "w", encoding="utf8").write(s)

# admission
p = Q + "/🛂️admission/🦀️.rs"
s = open(p, encoding="utf8").read()
if "en1997_anchor" not in s:
    s = s.replace("for id in 7..=12{", "for id in 7..=17{")
    s = s.replace("for(id,nested)in[(8,6),(9,12)]{", "for(id,nested)in[(8,6),(9,12),(13,13),(16,7)]{")
    def col(k, i):
        return f"Text({i})" if k == "s" else (f"Unsigned({i},u32::MAX as u64)" if k == "u" else f"Real({i})")
    w = ""
    for ent_i, e in enumerate(ENTITIES):
        fid = 13 + ent_i
        t = next(x for x in tabs if x.e is e)
        cells = ",".join(col(k, i) for i, (n, k) in enumerate(t.cols))
        if e.nested:
            c = t.children[0]
            ccells = ",".join(col(k, i) for i, (n, k) in enumerate(c.cols))
            nid = len(t.cols)
            w += f' for(index,v)in n::list(n::field(root,{fid})?)?.iter().enumerate(){{let v=n::record(v)?;let parent=n::entity(out,"{t.table}",1,index,&n::cells(v,[{cells}])?,{t.const})?;\n for(index,a)in n::list(n::field(v,{nid})?)?.iter().enumerate(){{n::entity(out,"{c.table}",parent,index,&n::cells(n::record(a)?,[{ccells}])?,{c.const})?;}}}}\n'
        else:
            w += f' for(index,v)in n::list(n::field(root,{fid})?)?.iter().enumerate(){{n::entity(out,"{t.table}",1,index,&n::cells(n::record(v)?,[{cells}])?,{t.const})?;}}\n'
    s = s.replace("Ok(())\n}\n/// 🛂️ Nine", w + "Ok(())\n}\n/// 🛂️ Nine", 1) if "Ok(())\n}\n/// 🛂️ Nine" in s else s
    s = s.replace("write)}", "write)}") 
    s = s.replace("SQLITE_SCHEMA,9,42,count(root)?,write)", f"SQLITE_SCHEMA,{9 + len(tabs)},{42 + sum(len(t.cols) for t in tabs)},count(root)?,write)")
    s = s.replace("/// 🛂️ Nine authored physical tables", f"/// 🛂️ {9 + len(tabs)} authored physical tables")
    open(p, "w", encoding="utf8").write(s)
print(len(tabs), "tables")
