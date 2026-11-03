import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys, re, json; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
A = Art("en1991")
B = A.any + "/🚪️io/🪶️sqlite/📸️snapshot"
SQL = '''CREATE TABLE en1991_case(id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1991_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, label_en TEXT NOT NULL, label_de TEXT NOT NULL, entry_id TEXT NOT NULL);
CREATE TABLE en1991_case_number(id INTEGER PRIMARY KEY, case_id INTEGER NOT NULL REFERENCES en1991_case(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, value REAL, value_ieee754_bits INTEGER NOT NULL, value_ieee754_class TEXT NOT NULL);
CREATE TABLE en1991_case_text(id INTEGER PRIMARY KEY, case_id INTEGER NOT NULL REFERENCES en1991_case(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, value TEXT NOT NULL);
'''
p = B + "/🗄️.sql"
wr(p, rd(p).rstrip("\n") + "\n" + SQL)
sql = rd(p)
tot = sum(len(st.encode()) + len(st[len("CREATE TABLE "):].split("(")[0].strip()) for st in [x.strip() for x in sql.split(";") if x.strip()])
n_old = sql.count("CREATE TABLE") - 3
p = B + "/🦀️.rs"
s = rd(p)
s = s.replace("use semio_s_artifact_norm_en1991_model::{AccidentalCase,", "use semio_s_artifact_norm_en1991_model::{CaseNumber, CaseRow, CaseText, AccidentalCase,", 1)
assert "CaseRow" in s
s = s.replace("const THERMAL_SECTION_FLOATS:", "const CASE_NUMBER_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4)];\nconst THERMAL_SECTION_FLOATS:", 1)
a = "        for (position, row) in self.thermal_sections.iter().enumerate() { out.insert_float("
assert s.count(a) == 1
s = s.replace(a, '''        for (position, case) in self.cases.iter().enumerate() {
            let id = out.insert_float("en1991_case", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&case.id), Cell::Text(&case.label_en), Cell::Text(&case.label_de), Cell::Text(&case.entry_id)], &[])?;
            for (index, number) in case.numbers.iter().enumerate() { out.insert_float("en1991_case_number", &[Cell::Integer(id), ordinal(index)?, Cell::Text(&number.id), Cell::Real(number.value)], CASE_NUMBER_FLOATS)?; }
            for (index, text) in case.texts.iter().enumerate() { out.insert_float("en1991_case_text", &[Cell::Integer(id), ordinal(index)?, Cell::Text(&text.id), Cell::Text(&text.value)], &[])?; }
        }
''' + a, 1)
a = "self.internal_explosions.len(),self.thermal_sections.len()]{rows=add(rows,size)?}"
assert s.count(a) == 1
s = s.replace(a, "self.internal_explosions.len(),self.thermal_sections.len(),self.cases.len()]{rows=add(rows,size)?}for case in &self.cases{rows=add(add(rows,case.numbers.len())?,case.texts.len())?}", 1)
s = s.replace("SQLITE_SCHEMA,18,188", "SQLITE_SCHEMA,21,188")
a = "        for row in &self.thermal_sections { bound.add(10240)?; bound.repeated(row.id.len(), 24)?; }\n"
assert s.count(a) == 1
s = s.replace(a, a + "        for case in &self.cases { bound.add(4096)?; for value in [&case.id, &case.label_en, &case.label_de, &case.entry_id] { bound.repeated(value.len(), 24)?; } for number in &case.numbers { bound.add(2048)?; bound.repeated(number.id.len(), 24)?; } for text in &case.texts { bound.add(2048)?; for value in [&text.id, &text.value] { bound.repeated(value.len(), 24)?; } } }\n", 1)
a = '        let thermal_section_rows = entities(database, "en1991_thermal_section", 10, THERMAL_SECTION_FLOATS, control)?;\n'
assert s.count(a) == 1
s = s.replace(a, a + '''        let design_rows = entities(database, "en1991_case", 7, &[], control)?;
        let mut design_numbers = child_groups(entities(database, "en1991_case_number", 5, CASE_NUMBER_FLOATS, control)?, &design_rows, control)?;
        let mut design_texts = child_groups(entities(database, "en1991_case_text", 5, &[], control)?, &design_rows, control)?;
''', 1)
a = "        let result = Self {\n"
assert s.count(a) == 1
s = s.replace(a, '''        let mut cases = Vec::new();
        for row in ordered(design_rows.into_values(), 1, control)? {
            checkpoint(control, cases.len(), 0)?;
            let mut numbers = Vec::new();
            for number in ordered(design_numbers.remove(&row.rowid).unwrap_or_default().into_iter(), row.rowid, control)? { numbers.push(CaseNumber { id: reconstruct_text(control, number.text(3)?)?, value: number.real(4)? }); }
            let mut texts = Vec::new();
            for text in ordered(design_texts.remove(&row.rowid).unwrap_or_default().into_iter(), row.rowid, control)? { texts.push(CaseText { id: reconstruct_text(control, text.text(3)?)?, value: reconstruct_text(control, text.text(4)?)? }); }
            cases.push(CaseRow { id: reconstruct_text(control, row.text(3)?)?, label_en: reconstruct_text(control, row.text(4)?)?, label_de: reconstruct_text(control, row.text(5)?)?, entry_id: reconstruct_text(control, row.text(6)?)?, numbers, texts });
        }
''' + a, 1)
a = "            thermal_sections,\n        };"
assert s.count(a) == 1
s = s.replace(a, "            thermal_sections,\n            cases,\n        };", 1)
wr(p, s)
# admission
p = B + "/🛂️admission/🦀️.rs"
s = rd(p)
a = "for id in 77..=91{rows=n::add(rows,n::list(n::field(root,id)?)?.len())?}"
assert s.count(a) == 1
s = s.replace(a, "for id in 77..=92{rows=n::add(rows,n::list(n::field(root,id)?)?.len())?}for case in n::list(n::field(root,92)?)?{let case=n::record(case)?;for id in 4..=5{rows=n::add(rows,n::list(n::field(case,id)?)?.len())?}}", 1)
a = "THERMAL_SECTION_FLOATS)?;}\n Ok(())"
assert s.count(a) == 1, s.count(a)
s = s.replace(a, '''THERMAL_SECTION_FLOATS)?;}
 for(index,c)in n::list(n::field(root,92)?)?.iter().enumerate(){let c=n::record(c)?;let parent=n::entity(out,"en1991_case",1,index,&n::cells(c,[Text(0),Text(1),Text(2),Text(3)])?,&[])?;
 for(i,r)in n::list(n::field(c,4)?)?.iter().enumerate(){n::entity(out,"en1991_case_number",parent,i,&n::cells(n::record(r)?,[Text(0),Real(1)])?,CASE_NUMBER_FLOATS)?;}
 for(i,r)in n::list(n::field(c,5)?)?.iter().enumerate(){n::entity(out,"en1991_case_text",parent,i,&n::cells(n::record(r)?,[Text(0),Text(1)])?,&[])?;}}
 Ok(())''', 1)
s = s.replace("SQLITE_SCHEMA,18,188,", "SQLITE_SCHEMA,21,188,").replace("🛂️ 18 authored", "🛂️ 21 authored")
wr(p, s)
# ts
p = B + "/🟦️.ts"
s = rd(p)
s = s.replace("Placement,SpecialVehicle,WindBridge,DynamicElement,ImpactAction,InternalExplosion,ThermalSection}from", "Placement,SpecialVehicle,WindBridge,DynamicElement,ImpactAction,InternalExplosion,ThermalSection,CaseRow,CaseNumber,CaseText}from", 1)
a = "export type En1991SqliteSnapshot="
assert s.count(a) == 1
s = s.replace(a, 'export type En1991SqliteCaseNumber=Omit<CaseNumber,"value">&{value:Binary64};\nexport type En1991SqliteCase=Omit<CaseRow,"numbers">&{numbers:En1991SqliteCaseNumber[]};\n' + a, 1)
s = s.replace('|"thermalSections"', '|"thermalSections"|"cases"', 1) if '|"thermalSections"' in s else s
assert '"cases"' in s
s = s.replace("thermalSections:En1991SqliteThermalSection[];", "thermalSections:En1991SqliteThermalSection[];cases:En1991SqliteCase[];", 1)
s = s.replace("const THERMAL_SECTION_FLOATS:", "const CASE_NUMBER_FLOATS:readonly Ieee754Column[]=[{index:4,width:64}];\nconst THERMAL_SECTION_FLOATS:", 1)
a = " return out.finish();\n}"
assert s.count(a) == 1
s = s.replace(a, ''' let designNumberId=1n,designTextId=1n;
 for(let position=0;position<s.cases.length;position++){
  const k=s.cases[position]!,id=BigInt(position+1);await write(out,"en1991_case",id,[1n,BigInt(position),k.id,k.labelEn,k.labelDe,k.entryId],[],options);
  for(let index=0;index<k.numbers.length;index++){const n=k.numbers[index]!;await write(out,"en1991_case_number",designNumberId++,[id,BigInt(index),n.id,n.value],CASE_NUMBER_FLOATS,options)}
  for(let index=0;index<k.texts.length;index++){const t=k.texts[index]!;await write(out,"en1991_case_text",designTextId++,[id,BigInt(index),t.id,t.value],[],options)}
 }
 return out.finish();
}''', 1)
a = " const thermalSectionRows=await entities(tables[17]!,10,THERMAL_SECTION_FLOATS,options);"
assert s.count(a) == 1
s = s.replace(a, a + "const designRows=await entities(tables[%d]!,7,[],options),designNumberRows=await entities(tables[%d]!,5,CASE_NUMBER_FLOATS,options),designTextRows=await entities(tables[%d]!,5,[],options);" % (n_old, n_old + 1, n_old + 2), 1)
a = " const thermalSections:En1991SqliteThermalSection[]=[];"
assert s.count(a) == 1
s = s.replace(a, ''' const designNumbers=new Map<bigint,SqliteRow[]>(),designTexts=new Map<bigint,SqliteRow[]>();
 for(const row of designNumberRows.values()){const parent=artifactSqliteInteger(row,1);if(!designRows.has(parent))throw Error("EN1991 case number has an unknown case");designNumbers.set(parent,[...(designNumbers.get(parent)??[]),row])}
 for(const row of designTextRows.values()){const parent=artifactSqliteInteger(row,1);if(!designRows.has(parent))throw Error("EN1991 case text has an unknown case");designTexts.set(parent,[...(designTexts.get(parent)??[]),row])}
 const cases:En1991SqliteCase[]=[];for(const r of await ordered(designRows.values(),1n,options)){await checkpoint(options,cases.length,0);const numbers:En1991SqliteCaseNumber[]=[];for(const n of await ordered(designNumbers.get(r.rowid)??[],r.rowid,options))numbers.push({id:artifactSqliteText(n,3),value:readBinary64(n,4,CASE_NUMBER_FLOATS)});const texts:CaseText[]=[];for(const t of await ordered(designTexts.get(r.rowid)??[],r.rowid,options))texts.push({id:artifactSqliteText(t,3),value:artifactSqliteText(t,4)});cases.push({id:artifactSqliteText(r,3),labelEn:artifactSqliteText(r,4),labelDe:artifactSqliteText(r,5),entryId:artifactSqliteText(r,6),numbers,texts})}
''' + a, 1)
a = "internalExplosions,thermalSections};"
assert s.count(a) == 1
s = s.replace(a, "internalExplosions,thermalSections,cases};", 1)
wr(p, s)
# tests
p = B + "/🧪️tests/🦀️.rs"
s = rd(p)
assert '("tables",18),("columns",188)' in s
wr(p, s.replace('("tables",18),("columns",188)', '("tables",21),("columns",188)'))
p = B + "/🧪️tests/🟦️.ts"
s = rd(p)
s = s.replace("maxTables:18,maxColumns:188", "maxTables:21,maxColumns:188").replace("{...limits,maxTables:17}", "{...limits,maxTables:20}").replace("toEqual(18170)", "toEqual(%d)" % tot)
s = s.replace('"en1991_accidental_case":', '"en1991_case":7,"en1991_case_number":7,"en1991_case_text":5,"en1991_accidental_case":', 1)
a = "const f=fixture;return{...f,"
assert s.count(a) == 1
s = s.replace(a, a + "cases:f.cases.map(k=>({...k,numbers:k.numbers.map(n=>({...n,value:binary64(n.value)}))})),", 1)
wr(p, s)
sem = B + "/🧪️tests/🧫️fixtures/🎛️semantic.json"
j = json.loads(rd(sem)); j["schemaBytes"] = tot; j["tableWidths"].update({"en1991_case": 7, "en1991_case_number": 7, "en1991_case_text": 5}); j["tableWidths"] = dict(sorted(j["tableWidths"].items()))
wr(sem, json.dumps(j, indent=2, ensure_ascii=False) + "\n")
print("schemaBytes", tot, "old tables", n_old, len(LOG))
