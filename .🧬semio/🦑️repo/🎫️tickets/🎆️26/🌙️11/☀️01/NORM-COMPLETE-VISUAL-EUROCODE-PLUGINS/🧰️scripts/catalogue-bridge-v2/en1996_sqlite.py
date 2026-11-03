import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys, re, json; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
A = Art("en1996")
B = A.any + "/🚪️io/🪶️sqlite/📸️snapshot"
SQL = '''CREATE TABLE en1996_case (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES en1996_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  logical_id TEXT NOT NULL, label_en TEXT NOT NULL, label_de TEXT NOT NULL, entry_id TEXT NOT NULL
);
CREATE TABLE en1996_case_number (
  id INTEGER PRIMARY KEY,
  case_id INTEGER NOT NULL REFERENCES en1996_case(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  logical_id TEXT NOT NULL,
  value REAL,
  value_ieee754_bits INTEGER NOT NULL, value_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1996_case_text (
  id INTEGER PRIMARY KEY,
  case_id INTEGER NOT NULL REFERENCES en1996_case(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  logical_id TEXT NOT NULL, value TEXT NOT NULL
);
'''
p = B + "/🗄️.sql"
wr(p, rd(p).rstrip("\n") + "\n" + SQL)
sql = rd(p)
tot = sum(len(st.encode()) + len(st[len("CREATE TABLE "):].split("(")[0].strip()) for st in [x.strip() for x in sql.split(";") if x.strip()])
p = B + "/🦀️.rs"
s = rd(p)
s = s.replace("use semio_s_artifact_norm_en1996_model::{MasonryClass, MasonryWall, Placement,", "use semio_s_artifact_norm_en1996_model::{CaseNumber, CaseRow, CaseText, MasonryClass, MasonryWall, Placement,", 1)
assert "CaseRow" in s
s = s.replace("const PLACEMENT_FLOATS:", "const CASE_NUMBER_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4)];\nconst PLACEMENT_FLOATS:", 1)
a = "        Ok(())\n    }\n    fn admit_sqlite_values"
assert s.count(a) == 1
s = s.replace(a, '''        for(position,case)in self.cases.iter().enumerate(){
            let id=out.insert_float("en1996_case",&[Cell::Integer(1),ordinal(position)?,Cell::Text(&case.id),Cell::Text(&case.label_en),Cell::Text(&case.label_de),Cell::Text(&case.entry_id)],&[])?;
            for(position,number)in case.numbers.iter().enumerate(){out.insert_float("en1996_case_number",&[Cell::Integer(id),ordinal(position)?,Cell::Text(&number.id),Cell::Real(number.value)],CASE_NUMBER_FLOATS)?;}
            for(position,text)in case.texts.iter().enumerate(){out.insert_float("en1996_case_text",&[Cell::Integer(id),ordinal(position)?,Cell::Text(&text.id),Cell::Text(&text.value)],&[])?;}
        }
''' + a, 1)
a = "SQLITE_SCHEMA,13,101,control)"
assert a in s
s = s.replace(a, "SQLITE_SCHEMA,16,101,control)", 1)
a = "let mut rows=add(add(1,self.walls.len())?,self.placements.len())?;control.check_rows(rows)?;"
assert a in s
s = s.replace(a, "let mut rows=add(add(add(1,self.walls.len())?,self.placements.len())?,self.cases.len())?;for case in &self.cases{rows=add(add(rows,case.numbers.len())?,case.texts.len())?}control.check_rows(rows)?;", 1)
a = "        for placement in &self.placements{bound.add(4096)?;bound.repeated(placement.id.len(),24)?;}\n"
assert s.count(a) == 1
s = s.replace(a, a + "        for case in &self.cases{bound.add(4096)?;for text in [&case.id,&case.label_en,&case.label_de,&case.entry_id]{bound.repeated(text.len(),24)?;}for number in &case.numbers{bound.add(2048)?;bound.repeated(number.id.len(),24)?;}for text in &case.texts{bound.add(2048)?;for value in [&text.id,&text.value]{bound.repeated(value.len(),24)?;}}}\n", 1)
a = 'let placement_rows=entities(database,"en1996_placement",8,PLACEMENT_FLOATS,control)?;'
assert a in s
s = s.replace(a, a + 'let case_rows=entities(database,"en1996_case",7,&[],control)?;let mut case_numbers=groups(entities(database,"en1996_case_number",5,CASE_NUMBER_FLOATS,control)?,&case_rows,control)?;let mut case_texts=groups(entities(database,"en1996_case_text",5,&[],control)?,&case_rows,control)?;', 1)
a = "        let snapshot=Self{annex:read_annex"
assert a in s
s = s.replace(a, '''        let mut design_cases=Vec::new();for row in ordered(case_rows.into_values(),1,control)?{
            checkpoint(control,design_cases.len(),0)?;let mut numbers=Vec::new();for number in ordered(case_numbers.remove(&row.rowid).unwrap_or_default().into_iter(),row.rowid,control)?{numbers.push(CaseNumber{id:reconstruct_text(control,number.text(3)?)?,value:number.real(4)?});}
            let mut texts=Vec::new();for text in ordered(case_texts.remove(&row.rowid).unwrap_or_default().into_iter(),row.rowid,control)?{texts.push(CaseText{id:reconstruct_text(control,text.text(3)?)?,value:reconstruct_text(control,text.text(4)?)?});}
            design_cases.push(CaseRow{id:reconstruct_text(control,row.text(3)?)?,label_en:reconstruct_text(control,row.text(4)?)?,label_de:reconstruct_text(control,row.text(5)?)?,entry_id:reconstruct_text(control,row.text(6)?)?,numbers,texts});
        }
''' + a, 1)
a = ",walls,placements};control.checkpoint"
assert s.count(a) == 1
s = s.replace(a, ",walls,placements,cases:design_cases};control.checkpoint", 1)
wr(p, s)
p = B + "/🛂️admission/🦀️.rs"
s = rd(p)
a = "for case in cases{rows=n::add(rows,n::list(n::field(n::record(case)?,11)?)?.len())?}}Ok(rows)}"
assert s.count(a) == 1
s = s.replace(a, "for case in cases{rows=n::add(rows,n::list(n::field(n::record(case)?,11)?)?.len())?}}let design=n::list(n::field(root,6)?)?;rows=n::add(rows,design.len())?;for case in design{let case=n::record(case)?;rows=n::add(rows,n::list(n::field(case,4)?)?.len())?;rows=n::add(rows,n::list(n::field(case,5)?)?.len())?}Ok(rows)}", 1)
a = 'PLACEMENT_FLOATS)?;}Ok(())'
assert s.count(a) == 1
s = s.replace(a, '''PLACEMENT_FLOATS)?;}
 for(index,c)in n::list(n::field(root,6)?)?.iter().enumerate(){let c=n::record(c)?;let parent=n::entity(out,"en1996_case",1,index,&n::cells(c,[Text(0),Text(1),Text(2),Text(3)])?,&[])?;
 for(i,r)in n::list(n::field(c,4)?)?.iter().enumerate(){n::entity(out,"en1996_case_number",parent,i,&n::cells(n::record(r)?,[Text(0),Real(1)])?,CASE_NUMBER_FLOATS)?;}
 for(i,r)in n::list(n::field(c,5)?)?.iter().enumerate(){n::entity(out,"en1996_case_text",parent,i,&n::cells(n::record(r)?,[Text(0),Text(1)])?,&[])?;}}
 Ok(())''', 1)
s = s.replace("SQLITE_SCHEMA,13,101,", "SQLITE_SCHEMA,16,101,").replace("Thirteen authored", "Sixteen authored")
wr(p, s)
# ts
p = B + "/🟦️.ts"
s = rd(p)
s = s.replace("MasonryWall,Placement,WallOpening,", "MasonryWall,Placement,CaseRow,CaseNumber,CaseText,WallOpening,", 1)
a = "export type En1996SqlitePlacement="
s = s.replace(a, 'export type En1996SqliteCaseNumber=Omit<CaseNumber,"value">&{value:Binary64};\nexport type En1996SqliteCase=Omit<CaseRow,"numbers">&{numbers:En1996SqliteCaseNumber[]};\n' + a, 1)
s = s.replace('|"walls"|"placements">', '|"walls"|"placements"|"cases">', 1)
s = s.replace("placements:En1996SqlitePlacement[]};", "placements:En1996SqlitePlacement[];cases:En1996SqliteCase[]};", 1)
s = s.replace("const PLACEMENT_FLOATS:", "const CASE_NUMBER_FLOATS:readonly Ieee754Column[]=[4].map(index=>({index,width:64}));\nconst PLACEMENT_FLOATS:", 1)
a = " return out.finish();\n}"
assert s.count(a) == 1
s = s.replace(a, ''' for(let position=0;position<snapshot.cases.length;position++){
  const k=snapshot.cases[position]!,caseId=await write(out,"en1996_case",[1n,BigInt(position),k.id,k.labelEn,k.labelDe,k.entryId],[],options);
  for(let index=0;index<k.numbers.length;index++){const n=k.numbers[index]!;await write(out,"en1996_case_number",[caseId,BigInt(index),n.id,n.value],CASE_NUMBER_FLOATS,options)}
  for(let index=0;index<k.texts.length;index++){const t=k.texts[index]!;await write(out,"en1996_case_text",[caseId,BigInt(index),t.id,t.value],[],options)}
 }
 return out.finish();
}''', 1)
a = " const walls:En1996SqliteWall[]=[];"
assert s.count(a) == 1
s = s.replace(a, ''' const designRows=await entities(tables[13]!,7,[],options),designNumbers=await groups(await entities(tables[14]!,5,CASE_NUMBER_FLOATS,options),designRows,options),designTexts=await groups(await entities(tables[15]!,5,[],options),designRows,options);
 const cases:En1996SqliteCase[]=[];for(const r of await ordered(designRows.values(),1n,options)){
  await checkpoint(options,cases.length,0);const numbers:En1996SqliteCaseNumber[]=[];for(const n of await ordered(designNumbers.get(r.rowid)??[],r.rowid,options))numbers.push({id:artifactSqliteText(n,3),value:readBinary64(n,4,CASE_NUMBER_FLOATS)});
  const texts:CaseText[]=[];for(const t of await ordered(designTexts.get(r.rowid)??[],r.rowid,options))texts.push({id:artifactSqliteText(t,3),value:artifactSqliteText(t,4)});
  cases.push({id:artifactSqliteText(r,3),labelEn:artifactSqliteText(r,4),labelDe:artifactSqliteText(r,5),entryId:artifactSqliteText(r,6),numbers,texts});
 }
''' + a, 1)
a = "storeys:readUnsigned(d,4,4294967295n),walls,placements};"
assert s.count(a) == 1
s = s.replace(a, "storeys:readUnsigned(d,4,4294967295n),walls,placements,cases};", 1)
wr(p, s)
# tests
p = B + "/🧪️tests/🦀️.rs"
s = rd(p)
assert '("tables",13),("columns",101)' in s
wr(p, s.replace('("tables",13),("columns",101)', '("tables",16),("columns",101)'))
p = B + "/🧪️tests/🟦️.ts"
s = rd(p)
s = s.replace("maxTables:13,maxColumns:101", "maxTables:16,maxColumns:101").replace("maxTables:11", "maxTables:14").replace("toEqual(11759)", "toEqual(%d)" % tot)
s = s.replace('"en1996_bar":14,', '"en1996_bar":14,"en1996_case":7,"en1996_case_number":7,"en1996_case_text":5,', 1)
wr(p, s)
sem = B + "/🧪️tests/🧫️fixtures/🎛️semantic.json"
j = json.loads(rd(sem)); j["schemaBytes"] = tot; j["tableWidths"].update({"en1996_case": 7, "en1996_case_number": 7, "en1996_case_text": 5}); j["tableWidths"] = dict(sorted(j["tableWidths"].items()))
wr(sem, json.dumps(j, indent=2, ensure_ascii=False) + "\n")
print("schemaBytes", tot, len(LOG))
