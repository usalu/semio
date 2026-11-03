import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
A = Art("en1997")
D = A.any + "/🚪️io/🪶️sqlite/📸️snapshot"
sub(D + "/🗄️.sql", "CREATE TABLE en1997_placement (", "CREATE TABLE en1997_placement (")
s = rd(D + "/🗄️.sql").rstrip("\n") + """
CREATE TABLE en1997_case (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1997_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, label_en TEXT NOT NULL, label_de TEXT NOT NULL, entry_id TEXT NOT NULL
);
CREATE TABLE en1997_case_number (
 id INTEGER PRIMARY KEY, case_id INTEGER NOT NULL REFERENCES en1997_case(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, value REAL,
 value_ieee754_bits INTEGER NOT NULL, value_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_case_text (
 id INTEGER PRIMARY KEY, case_id INTEGER NOT NULL REFERENCES en1997_case(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, value TEXT NOT NULL
);
"""
wr(D + "/🗄️.sql", s)
p = D + "/🦀️.rs"
sub(p, "LayerWater, Placement};", "LayerWater, Placement, CaseRow, CaseNumber, CaseText};")
sub(p, "const PLACEMENT:&[FloatColumn]", "const CASENUMBER:&[FloatColumn]=&[FloatColumn::Binary64(4)];\nconst PLACEMENT:&[FloatColumn]")
w = '  for(n,q)in self.placements.iter().enumerate(){p.insert_float("en1997_placement",&[Cell::Integer(1),ordinal(n)?,Cell::Text(&q.id),Cell::Real(q.x),Cell::Real(q.y),Cell::Real(q.z),Cell::Real(q.yaw)],PLACEMENT)?;}\n'
sub(p, w, w + '  for(n,v)in self.cases.iter().enumerate(){let id=p.insert_float("en1997_case",&[Cell::Integer(1),ordinal(n)?,Cell::Text(&v.id),Cell::Text(&v.label_en),Cell::Text(&v.label_de),Cell::Text(&v.entry_id)],&[])?;for(k,a)in v.numbers.iter().enumerate(){p.insert_float("en1997_case_number",&[Cell::Integer(id),ordinal(k)?,Cell::Text(&a.id),Cell::Real(a.value)],CASENUMBER)?;}for(k,a)in v.texts.iter().enumerate(){p.insert_float("en1997_case_text",&[Cell::Integer(id),ordinal(k)?,Cell::Text(&a.id),Cell::Text(&a.value)],&[])?;}}\n')
sub(p, "sqlite_native::schema(En1997Snapshot::SQLITE_SCHEMA,17,96,c)", "sqlite_native::schema(En1997Snapshot::SQLITE_SCHEMA,20,96,c)")
sub(p, "self.layer_waters.len(),self.placements.len()]{admit_rows", "self.layer_waters.len(),self.placements.len(),self.cases.len()]{admit_rows")
sub(p, "for v in &self.lab_tests{admit_rows(&mut b,&mut rows,v.points.len())?;}", "for v in &self.lab_tests{admit_rows(&mut b,&mut rows,v.points.len())?;}for v in &self.cases{admit_rows(&mut b,&mut rows,v.numbers.len())?;admit_rows(&mut b,&mut rows,v.texts.len())?;}")
sub(p, "for v in &self.placements{b.add(8192)?;b.repeated(v.id.len(),24)?;}b.finish()}", "for v in &self.placements{b.add(8192)?;b.repeated(v.id.len(),24)?;}for v in &self.cases{b.add(4096)?;for s in[&v.id,&v.label_en,&v.label_de,&v.entry_id]{b.repeated(s.len(),24)?;}for a in &v.numbers{b.add(2048)?;b.repeated(a.id.len(),24)?;}for a in &v.texts{b.add(2048)?;for s in[&a.id,&a.value]{b.repeated(s.len(),24)?;}}}b.finish()}")
r = '  let mut placements=Vec::new();'
i = rd(p).index(r)
s = rd(p)
rd_cases = '  let case_rows=entities(d,"en1997_case",7,&[],c)?;let mut case_numbers=groups(entities(d,"en1997_case_number",5,CASENUMBER,c)?,&case_rows,c)?;let mut case_texts=groups(entities(d,"en1997_case_text",5,&[],c)?,&case_rows,c)?;\n  let mut cases=Vec::new();for v in ordered(case_rows.into_values(),1,c)?{checkpoint(c,cases.len(),0)?;let mut numbers=Vec::new();for a in ordered(case_numbers.remove(&v.rowid).unwrap_or_default(),v.rowid,c)?{numbers.push(CaseNumber{id:text(a,3,c)?,value:a.real(4)?});}let mut texts=Vec::new();for a in ordered(case_texts.remove(&v.rowid).unwrap_or_default(),v.rowid,c)?{texts.push(CaseText{id:text(a,3,c)?,value:text(a,4,c)?});}cases.push(CaseRow{id:text(v,3,c)?,label_en:text(v,4,c)?,label_de:text(v,5,c)?,entry_id:text(v,6,c)?,numbers,texts});}\n'
s = s[:i] + rd_cases + s[i:]
assert s.count("layer_waters,placements};") == 1
s = s.replace("layer_waters,placements};", "layer_waters,placements,cases};")
wr(p, s)
q = D + "/🛂️admission/🦀️.rs"
sub(q, "for id in 7..=18{", "for id in 7..=19{")
sub(q, "for(id,nested)in[(8,6),(9,12),(13,13),(16,7)]{for owner in n::list(n::field(root,id)?)?{rows=n::add(rows,n::list(n::field(n::record(owner)?,nested)?)?.len())?}}", "for(id,nested)in[(8,6),(9,12),(13,13),(16,7),(19,4),(19,5)]{for owner in n::list(n::field(root,id)?)?{rows=n::add(rows,n::list(n::field(n::record(owner)?,nested)?)?.len())?}}")
sub(q, " Ok(())\n}\n/// 🛂️ 17 authored", ' for(index,c)in n::list(n::field(root,19)?)?.iter().enumerate(){let c=n::record(c)?;let parent=n::entity(out,"en1997_case",1,index,&n::cells(c,[Text(0),Text(1),Text(2),Text(3)])?,&[])?;\n for(index,r)in n::list(n::field(c,4)?)?.iter().enumerate(){n::entity(out,"en1997_case_number",parent,index,&n::cells(n::record(r)?,[Text(0),Real(1)])?,CASENUMBER)?;}\n for(index,r)in n::list(n::field(c,5)?)?.iter().enumerate(){n::entity(out,"en1997_case_text",parent,index,&n::cells(n::record(r)?,[Text(0),Text(1)])?,&[])?;}}\nOk(())\n}\n/// 🛂️ 20 authored')
sub(q, "En1997Snapshot::SQLITE_SCHEMA,17,96,count(root)?,write)", "En1997Snapshot::SQLITE_SCHEMA,20,96,count(root)?,write)")
print(len(LOG))
