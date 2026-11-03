"""Stage 4: SQLite contract of WeldRun and SectionVertex (sql, rust, ts, admission, tests, fixtures). Idempotent."""
import json, sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from spec import *

TBL = {"weld_runs": ("en1999_weld_run", "WELD_RUN", "w"), "outline_vertices": ("en1999_section_vertex", "VERTEX", "v")}
# column layouts: (rust expr suffix, kind) per field after the 4 base columns
SQLCOL = {
    "weld_runs": "member_reference TEXT NOT NULL, element_reference TEXT NOT NULL, run_start REAL, run_length REAL, across_offset REAL, throat REAL, process TEXT NOT NULL, t_junction INTEGER NOT NULL CHECK(t_junction IN(0,1)), direction TEXT NOT NULL, haz_extent REAL",
    "outline_vertices": "section_reference TEXT NOT NULL, ring TEXT NOT NULL, element_reference TEXT NOT NULL, y REAL, z REAL",
}
REALS = {"weld_runs": ["run_start", "run_length", "across_offset", "throat", "haz_extent"], "outline_vertices": ["y", "z"]}
FLOAT_IDX = {"weld_runs": [6, 7, 8, 9, 13], "outline_vertices": [7, 8]}
COUNT = {"weld_runs": 14, "outline_vertices": 9}


def rw(p, fn):
    p = pathlib.Path(p)
    t = p.read_text(encoding="utf8")
    n = fn(t)
    if n != t:
        p.write_text(n, encoding="utf8")


def sql(t):
    if "en1999_weld_run" in t:
        return t
    base = "id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1999_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL"
    for e in ENTS:
        name = TBL[e.field][0]
        ieee = ",\n".join(f" {c}_ieee754_bits INTEGER NOT NULL, {c}_ieee754_class TEXT NOT NULL" for c in REALS[e.field])
        t = t.rstrip("\n") + f"\nCREATE TABLE {name} (\n {base}, {SQLCOL[e.field]},\n{ieee}\n);\n"
    return t


def rs(t):
    if "WELD_RUN" in t:
        return t
    t = t.replace("AluminiumShell, Placement}", "AluminiumShell, Placement, WeldRun, SectionVertex}", 1)
    cols = lambda f: ",".join(f"FloatColumn::Binary64({i})" for i in FLOAT_IDX[f])
    t = t.replace("const PLACEMENT:", f"const WELD_RUN:&[FloatColumn]=&[{cols('weld_runs')}];\nconst VERTEX:&[FloatColumn]=&[{cols('outline_vertices')}];\nconst PLACEMENT:", 1)
    w = '  for(n,w)in self.weld_runs.iter().enumerate(){out.insert_float("en1999_weld_run",&[Cell::Integer(1),ordinal(n)?,Cell::Text(&w.id),Cell::Text(&w.member_id),Cell::Text(&w.element_id),Cell::Real(w.start),Cell::Real(w.length),Cell::Real(w.offset),Cell::Real(w.throat),Cell::Text(&w.process),flag(w.t_junction),Cell::Text(&w.direction),Cell::Real(w.haz_extent)],WELD_RUN)?;}\n'
    v = '  for(n,v)in self.outline_vertices.iter().enumerate(){out.insert_float("en1999_section_vertex",&[Cell::Integer(1),ordinal(n)?,Cell::Text(&v.id),Cell::Text(&v.section_id),Cell::Text(&v.ring),Cell::Text(&v.element_id),Cell::Real(v.y),Cell::Real(v.z)],VERTEX)?;}\n'
    anchor = "        Ok(())\n    }\n    fn admit_sqlite_values"
    assert anchor in t
    t = t.replace(anchor, w + v + anchor, 1)
    t = t.replace("schema(En1999Snapshot::SQLITE_SCHEMA,16,36,control)", "schema(En1999Snapshot::SQLITE_SCHEMA,19,36,control)", 1)
    t = t.replace("self.shells.len(),self.placements.len()]{add(n)?;}", "self.shells.len(),self.placements.len(),self.weld_runs.len(),self.outline_vertices.len()]{add(n)?;}", 1)
    t = t.replace("admit_rows(&mut b,&mut rows,self.placements.len())?;", "admit_rows(&mut b,&mut rows,self.placements.len())?;admit_rows(&mut b,&mut rows,self.weld_runs.len())?;admit_rows(&mut b,&mut rows,self.outline_vertices.len())?;", 1)
    t = t.replace("  for p in &self.placements{b.add(8192)?;b.repeated(p.id.len(),24)?;}\n", "  for p in &self.placements{b.add(8192)?;b.repeated(p.id.len(),24)?;}\n  for w in &self.weld_runs{b.add(12288)?;for t in [&w.id,&w.member_id,&w.element_id,&w.process,&w.direction]{b.repeated(t.len(),24)?;}}\n  for v in &self.outline_vertices{b.add(8192)?;for t in [&v.id,&v.section_id,&v.ring,&v.element_id]{b.repeated(t.len(),24)?;}}\n", 1)
    rdr = '  let mut weld_runs=Vec::new();for r in ordered(entities(d,"en1999_weld_run",14,WELD_RUN,c)?.into_values(),1,c)?{checkpoint(c,weld_runs.len(),0)?;weld_runs.push(WeldRun{id:text(r,3,c)?,member_id:text(r,4,c)?,element_id:text(r,5,c)?,start:r.real(6)?,length:r.real(7)?,offset:r.real(8)?,throat:r.real(9)?,process:text(r,10,c)?,t_junction:read_flag(r,11)?,direction:text(r,12,c)?,haz_extent:r.real(13)?});}\n  let mut outline_vertices=Vec::new();for r in ordered(entities(d,"en1999_section_vertex",9,VERTEX,c)?.into_values(),1,c)?{checkpoint(c,outline_vertices.len(),0)?;outline_vertices.push(SectionVertex{id:text(r,3,c)?,section_id:text(r,4,c)?,ring:text(r,5,c)?,element_id:text(r,6,c)?,y:r.real(7)?,z:r.real(8)?});}\n'
    a2 = "  c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;Ok(Self{"
    assert a2 in t
    t = t.replace(a2, rdr + a2, 1)
    t = t.replace("cold_formed,shells,placements})", "cold_formed,shells,placements,weld_runs,outline_vertices})", 1)
    return t


def admission(t):
    if "en1999_weld_run" in t:
        return t
    t = t.replace("for id in 1..=9{", "for id in 1..=11{", 1)
    new = ' for(index,w)in n::list(n::field(root,10)?)?.iter().enumerate(){n::entity(out,"en1999_weld_run",1,index,&n::cells(n::record(w)?,[Text(0),Text(1),Text(2),Real(3),Real(4),Real(5),Real(6),Text(7),Boolean(8),Text(9),Real(10)])?,WELD_RUN)?;}\n for(index,v)in n::list(n::field(root,11)?)?.iter().enumerate(){n::entity(out,"en1999_section_vertex",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Text(2),Real(3),Real(4)])?,VERTEX)?;}\n'
    old = 'PLACEMENT)?;}Ok(())\n}'
    assert old in t
    t = t.replace(old, 'PLACEMENT)?;}\n' + new + ' Ok(())\n}', 1)
    t = t.replace("Seventeen authored tables", "Nineteen authored tables").replace("SQLITE_SCHEMA,17,36,", "SQLITE_SCHEMA,19,36,")
    return t


def ts(t):
    if "WELD_RUN" in t:
        return t
    t = t.replace("AluminiumShell,Placement}from", "AluminiumShell,Placement,WeldRun,SectionVertex}from", 1)
    t = t.replace("export type En1999SqliteSnapshot=", 'export type En1999SqlitePlacement=Omit<Placement,"x"|"y"|"z"|"yaw">&{x:Binary64;y:Binary64;z:Binary64;yaw:Binary64};\nexport type En1999SqliteWeldRun=Omit<WeldRun,"start"|"length"|"offset"|"throat"|"hazExtent">&{start:Binary64;length:Binary64;offset:Binary64;throat:Binary64;hazExtent:Binary64};\nexport type En1999SqliteVertex=Omit<SectionVertex,"y"|"z">&{y:Binary64;z:Binary64};\nexport type En1999SqliteSnapshot=', 1)
    t = t.replace("shells:En1999SqliteShell[];placements:Placement[]}", "shells:En1999SqliteShell[];placements:En1999SqlitePlacement[];weldRuns:En1999SqliteWeldRun[];outlineVertices:En1999SqliteVertex[]}", 1)
    t = t.replace("const PLACEMENT:", "const WELD_RUN:readonly Ieee754Column[]=[6,7,8,9,13].map(index=>({index,width:64}));\nconst VERTEX:readonly Ieee754Column[]=[7,8].map(index=>({index,width:64}));\nconst PLACEMENT:", 1)
    t = t.replace('"shells","placements"]);for(const values of[s.materials,s.sections,s.members,s.connections,s.fireScenarios,s.fatigueDetails,s.coldFormed,s.shells,s.placements])', '"shells","placements","weldRuns","outlineVertices"]);for(const values of[s.materials,s.sections,s.members,s.connections,s.fireScenarios,s.fatigueDetails,s.coldFormed,s.shells,s.placements,s.weldRuns,s.outlineVertices])', 1)
    w = ' for(let n=0;n<s.weldRuns.length;n++){const v=s.weldRuns[n]!;fields(v,["id","memberId","elementId","start","length","offset","throat","process","tJunction","direction","hazExtent"]);await write(out,"en1999_weld_run",[1n,BigInt(n),v.id,v.memberId,v.elementId,v.start,v.length,v.offset,v.throat,v.process,flag(v.tJunction),v.direction,v.hazExtent],WELD_RUN,o)}\n for(let n=0;n<s.outlineVertices.length;n++){const v=s.outlineVertices[n]!;fields(v,["id","sectionId","ring","elementId","y","z"]);await write(out,"en1999_section_vertex",[1n,BigInt(n),v.id,v.sectionId,v.ring,v.elementId,v.y,v.z],VERTEX,o)}\n'
    a = " return out.finish();"
    assert a in t
    t = t.replace(a, w + a, 1)
    t = t.replace("shells:En1999SqliteShell[]=[],placements:Placement[]=[];", "shells:En1999SqliteShell[]=[],placements:En1999SqlitePlacement[]=[],weldRuns:En1999SqliteWeldRun[]=[],outlineVertices:En1999SqliteVertex[]=[];", 1)
    rdr = ' for(const r of await ordered((await entities(t[17]!,14,WELD_RUN,o)).values(),1n,o)){await checkpoint(o,weldRuns.length);weldRuns.push({id:artifactSqliteText(r,3),memberId:artifactSqliteText(r,4),elementId:artifactSqliteText(r,5),start:readBinary64(r,6,WELD_RUN),length:readBinary64(r,7,WELD_RUN),offset:readBinary64(r,8,WELD_RUN),throat:readBinary64(r,9,WELD_RUN),process:artifactSqliteText(r,10),tJunction:artifactSqliteBoolean(r,11),direction:artifactSqliteText(r,12),hazExtent:readBinary64(r,13,WELD_RUN)})}\n for(const r of await ordered((await entities(t[18]!,9,VERTEX,o)).values(),1n,o)){await checkpoint(o,outlineVertices.length);outlineVertices.push({id:artifactSqliteText(r,3),sectionId:artifactSqliteText(r,4),ring:artifactSqliteText(r,5),elementId:artifactSqliteText(r,6),y:readBinary64(r,7,VERTEX),z:readBinary64(r,8,VERTEX)})}\n'
    a = ' await artifactSqliteCheckpoint(o,"reconstructSnapshot",1,1);return{'
    assert a in t
    t = t.replace(a, rdr + a, 1)
    t = t.replace("coldFormed,shells,placements};", "coldFormed,shells,placements,weldRuns,outlineVertices};", 1)
    return t


def tests_rs(t):
    if "weld_runs" in t:
        return t
    t = t.replace("assert_eq!(d.tables.len(),16)", "assert_eq!(d.tables.len(),19)", 1)
    t = t.replace("for v in &mut s.placements{v.x=w;v.y=w;v.z=w;v.yaw=w;}s}", "for v in &mut s.placements{v.x=w;v.y=w;v.z=w;v.yaw=w;}for v in &mut s.weld_runs{v.start=w;v.length=w;v.offset=w;v.throat=w;v.haz_extent=w;}for v in &mut s.outline_vertices{v.y=w;v.z=w;}s}", 1)
    t = t.replace("source.placements.clear();}", "source.placements.clear();source.weld_runs.clear();source.outline_vertices.clear();}", 1)
    t = t.replace("max_tables:17,max_columns:36", "max_tables:19,max_columns:36").replace('("tables",17)', '("tables",19)').replace("max_tables:16,..limits", "max_tables:18,..limits")
    return t


def tests_ts(t):
    if "weldRuns" in t:
        return t
    t = t.replace("yaw:binary64(r.yaw)}))}}", "yaw:binary64(r.yaw)})),weldRuns:f.weldRuns.map(r=>({...r,start:binary64(r.start),length:binary64(r.length),offset:binary64(r.offset),throat:binary64(r.throat),hazExtent:binary64(r.hazExtent)})),outlineVertices:f.outlineVertices.map(r=>({...r,y:binary64(r.y),z:binary64(r.z)}))}}", 1)
    t = t.replace("for(const v of s.placements)Object.assign(v,{x:w,y:w,z:w,yaw:w});", "for(const v of s.placements)Object.assign(v,{x:w,y:w,z:w,yaw:w});for(const v of s.weldRuns)Object.assign(v,{start:w,length:w,offset:w,throat:w,hazExtent:w});for(const v of s.outlineVertices)Object.assign(v,{y:w,z:w});")
    t = t.replace("source.placements=[];}", "source.placements=[];source.weldRuns=[];source.outlineVertices=[];}")
    t = t.replace("maxTables:17,", "maxTables:19,").replace("maxTables:16}", "maxTables:18}")
    return t


def fixture():
    p = SQL / "🧪️tests/🧫️fixtures/🔣️.json"
    d = json.loads(p.read_text(encoding="utf8"))
    s = d["snapshot"]
    if "weldRuns" not in s:
        s["weldRuns"] = [{"id": "weld 日本", "memberId": "member", "elementId": "web", "start": 0.5, "length": 1.25, "offset": 0.02, "throat": 0.004, "process": "tig", "tJunction": True, "direction": "longitudinal", "hazExtent": 0.0}]
        s["outlineVertices"] = [{"id": "v1", "sectionId": "section", "ring": "outer", "elementId": "web", "y": 0.0, "z": 0.0}, {"id": "v2", "sectionId": "section", "ring": "outer", "elementId": "", "y": 0.1, "z": 0.0}, {"id": "v3", "sectionId": "section", "ring": "hole 日本", "elementId": "", "y": 0.05, "z": 0.02}]
        d["tableNames"] += ["en1999_weld_run", "en1999_section_vertex"]
        p.write_text(json.dumps(d, indent=2, ensure_ascii=False) + "\n", encoding="utf8")


if __name__ == "__main__":
    rw(SQL / "🗄️.sql", sql); rw(SQL / "🦀️.rs", rs); rw(SQL / "🛂️admission/🦀️.rs", admission); rw(SQL / "🟦️.ts", ts)
    rw(SQL / "🧪️tests/🦀️.rs", tests_rs); rw(SQL / "🧪️tests/🟦️.ts", tests_ts); fixture()
    print("stage 4 done")
