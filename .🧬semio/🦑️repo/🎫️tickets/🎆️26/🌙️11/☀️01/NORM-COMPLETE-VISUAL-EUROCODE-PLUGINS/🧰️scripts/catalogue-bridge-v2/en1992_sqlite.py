import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys, re, json; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
A = Art("en1992")
B = A.any + "/🚪️io/🪶️sqlite/📸️snapshot"
SQL = '''CREATE TABLE en1992_case (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1992_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 logical_id TEXT NOT NULL, label_en TEXT NOT NULL, label_de TEXT NOT NULL, entry_id TEXT NOT NULL
);
CREATE TABLE en1992_case_number (
 id INTEGER PRIMARY KEY, case_id INTEGER NOT NULL REFERENCES en1992_case(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 logical_id TEXT NOT NULL, value REAL,
 value_ieee754_bits INTEGER NOT NULL, value_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1992_case_text (
 id INTEGER PRIMARY KEY, case_id INTEGER NOT NULL REFERENCES en1992_case(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 logical_id TEXT NOT NULL, value TEXT NOT NULL
);
'''
p = B + "/🗄️.sql"
wr(p, rd(p).rstrip("\n") + "\n" + SQL)
p = B + "/🦀️.rs"
s = rd(p)
a = "use crate::{ConcreteGrade,"
assert s.count(a) == 1
s = s.replace(a, "use crate::{CaseNumber, CaseRow, CaseText, ConcreteGrade,", 1)
a = "type Entities<'a>="
s = s.replace(a, "const CASE_NUMBER:&[FloatColumn]=&[FloatColumn::Binary64(4)];\n" + a, 1)
a = '  for(n,v)in s.placements.iter().enumerate(){p.insert_float("en1992_placement",'
assert s.count(a) == 1
i = s.index(a)
j = s.index("\n", i)
s = s[:j + 1] + '''  for(n,k)in s.cases.iter().enumerate(){let id=p.insert_float("en1992_case",&[Cell::Integer(1),ordinal(n)?,Cell::Text(&k.id),Cell::Text(&k.label_en),Cell::Text(&k.label_de),Cell::Text(&k.entry_id)],&[])?;for(m,v)in k.numbers.iter().enumerate(){p.insert_float("en1992_case_number",&[Cell::Integer(id),ordinal(m)?,Cell::Text(&v.id),Cell::Real(v.value)],CASE_NUMBER)?;}for(m,v)in k.texts.iter().enumerate(){p.insert_float("en1992_case_text",&[Cell::Integer(id),ordinal(m)?,Cell::Text(&v.id),Cell::Text(&v.value)],&[])?;}}
''' + s[j + 1:]
a = 'let placement_rows=entities(d,"en1992_placement",8,PLACEMENT,c)?;'
assert s.count(a) == 1
s = s.replace(a, a + '\n  let case_rows=entities(d,"en1992_case",7,&[],c)?;let mut case_numbers=groups(entities(d,"en1992_case_number",5,CASE_NUMBER,c)?,&case_rows,c)?;let mut case_texts=groups(entities(d,"en1992_case_text",5,&[],c)?,&case_rows,c)?;', 1)
a = "  let result=Self{annex:read_annex"
assert s.count(a) == 1
s = s.replace(a, '''  let mut cases=Vec::new();for k in ordered(case_rows.into_values(),1,c)?{checkpoint(c,cases.len(),0)?;let mut numbers=Vec::new();for v in ordered(case_numbers.remove(&k.rowid).unwrap_or_default(),k.rowid,c)?{numbers.push(CaseNumber{id:text(v,3,c)?,value:v.real(4)?});}let mut texts=Vec::new();for v in ordered(case_texts.remove(&k.rowid).unwrap_or_default(),k.rowid,c)?{texts.push(CaseText{id:text(v,3,c)?,value:text(v,4,c)?});}cases.push(CaseRow{id:text(k,3,c)?,label_en:text(k,4,c)?,label_de:text(k,5,c)?,entry_id:text(k,6,c)?,numbers,texts});}
''' + a, 1)
a = "anchors,placements};c.checkpoint"
assert s.count(a) == 1
s = s.replace(a, "anchors,placements,cases};c.checkpoint", 1)
wr(p, s)
# cells
p = B + "/📏️cells/🦀️.rs"
s = rd(p)
a = '("en1992_bar_point",14),'
assert a in s
s = s.replace(a, a + '("en1992_case",7),("en1992_case_number",7),("en1992_case_text",5),', 1)
a = "shape(source,11)?;"
assert s.count(a) == 1
s = s.replace(a, "shape(source,12)?;", 1)
a = '  collection(field(source,10)?,5,"en1992_placement",&[(0,Text),(1,Float),(2,Float),(3,Float),(4,Float)],&mut c,n)?;\n'
assert s.count(a) == 1
s = s.replace(a, a + '''  for value in items(field(source,11)?)?{
   let value=record(value)?;shape(value,6)?;c.row("en1992_case",add(24,fields(value,&[(0,Text),(1,Text),(2,Text),(3,Text)],n)?)?,n)?;
   collection(field(value,4)?,2,"en1992_case_number",&[(0,Text),(1,Float)],&mut c,n)?;
   collection(field(value,5)?,2,"en1992_case_text",&[(0,Text),(1,Text)],&mut c,n)?;
  }
''', 1)
wr(p, s)
# ts
p = B + "/🟦️.ts"
s = rd(p)
a = "export type En1992SqlitePlacement="
s = s.replace(a, "export type En1992SqliteCaseNumber={id:string;value:Binary64};\nexport type En1992SqliteCaseText={id:string;value:string};\nexport type En1992SqliteCase={id:string;labelEn:string;labelDe:string;entryId:string;numbers:En1992SqliteCaseNumber[];texts:En1992SqliteCaseText[]};\n" + a, 1)
s = s.replace("placements:En1992SqlitePlacement[]};", "placements:En1992SqlitePlacement[];cases:En1992SqliteCase[]};", 1)
s = s.replace("const PLACEMENT:", "const CASE_NUMBER:readonly Ieee754Column[]=[{index:4,width:64}];\nconst PLACEMENT:", 1)
assert "CASE_NUMBER" in s
s = s.replace('"members","anchors","placements"]);for(const v of[s.concreteGrades,s.reinforcementGrades,s.prestressSteels,s.members,s.anchors,s.placements])collection(v);', '"members","anchors","placements","cases"]);for(const v of[s.concreteGrades,s.reinforcementGrades,s.prestressSteels,s.members,s.anchors,s.placements,s.cases])collection(v);', 1)
assert '"placements","cases"' in s
a = ' for(let n=0;n<s.placements.length;n++){const v=s.placements[n]!;'
i = s.index(a)
j = s.index("\n", i)
s = s[:j + 1] + ''' for(let n=0;n<s.cases.length;n++){const k=s.cases[n]!;fields(k,["id","labelEn","labelDe","entryId","numbers","texts"]);const id=await write(out,"en1992_case",[1n,BigInt(n),k.id,k.labelEn,k.labelDe,k.entryId],[],o);
  for(let m=0;m<k.numbers.length;m++){const v=k.numbers[m]!;fields(v,["id","value"]);await write(out,"en1992_case_number",[id,BigInt(m),v.id,v.value],CASE_NUMBER,o)}
  for(let m=0;m<k.texts.length;m++){const v=k.texts[m]!;fields(v,["id","value"]);await write(out,"en1992_case_text",[id,BigInt(m),v.id,v.value],[],o)}}
''' + s[j + 1:]
a = " const placements:En1992SqlitePlacement[]=[];"
assert s.count(a) == 1
s = s.replace(a, ''' const caseRows=await entities(t[18]!,7,[],o),caseNumbers=await groups(await entities(t[19]!,5,CASE_NUMBER,o),caseRows,o),caseTexts=await groups(await entities(t[20]!,5,[],o),caseRows,o);
 const cases:En1992SqliteCase[]=[];for(const k of await ordered(caseRows.values(),1n,o)){const numbers:En1992SqliteCaseNumber[]=[],texts:En1992SqliteCaseText[]=[];for(const v of await ordered(caseNumbers.get(k.rowid)??[],k.rowid,o))numbers.push({id:artifactSqliteText(v,3),value:readBinary64(v,4,CASE_NUMBER)});for(const v of await ordered(caseTexts.get(k.rowid)??[],k.rowid,o))texts.push({id:artifactSqliteText(v,3),value:artifactSqliteText(v,4)});cases.push({id:artifactSqliteText(k,3),labelEn:artifactSqliteText(k,4),labelDe:artifactSqliteText(k,5),entryId:artifactSqliteText(k,6),numbers,texts})}
''' + a, 1)
a = "prestressSteels,members,anchors,placements};"
assert s.count(a) == 1
s = s.replace(a, "prestressSteels,members,anchors,placements,cases};", 1)
wr(p, s)
# tests + fixture
p = B + "/🧪️tests/🦀️.rs"
s = rd(p)
assert "assert_eq!(d.tables.len(),18);" in s
wr(p, s.replace("assert_eq!(d.tables.len(),18);", "assert_eq!(d.tables.len(),21);", 1))
fx = B + "/🧪️tests/🧫️fixtures/🔣️.json"
j = json.loads(rd(fx))
j["snapshot"]["cases"] = []
j["tableNames"] += ["en1992_case", "en1992_case_number", "en1992_case_text"]
j["semanticCells"]["tableWidths"] = sorted(j["semanticCells"]["tableWidths"] + [["en1992_case", 7], ["en1992_case_number", 7], ["en1992_case_text", 5]])
wr(fx, json.dumps(j, indent=2, ensure_ascii=False) + "\n")
print(len(LOG))
