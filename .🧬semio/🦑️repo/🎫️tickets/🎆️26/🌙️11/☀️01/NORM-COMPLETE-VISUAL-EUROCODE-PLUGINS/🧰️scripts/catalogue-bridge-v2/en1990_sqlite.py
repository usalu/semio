import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys, re; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
A = Art("en1990")
B = A.any + "/🚪️io/🪶️sqlite/📸️snapshot"
SQL = '''CREATE TABLE en1990_case(id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1990_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, label_en TEXT NOT NULL, label_de TEXT NOT NULL, entry_id TEXT NOT NULL);
CREATE TABLE en1990_case_number(id INTEGER PRIMARY KEY, case_id INTEGER NOT NULL REFERENCES en1990_case(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, value REAL, value_ieee754_bits INTEGER NOT NULL, value_ieee754_class TEXT NOT NULL);
CREATE TABLE en1990_case_text(id INTEGER PRIMARY KEY, case_id INTEGER NOT NULL REFERENCES en1990_case(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, value TEXT NOT NULL);
'''
p = B + "/🗄️.sql"
s = rd(p)
wr(p, s.rstrip("\n") + "\n" + SQL)
sql = rd(p)
tot = sum(len(st.encode()) + len(st[len("CREATE TABLE "):].split("(")[0].strip()) for st in [x.strip() for x in sql.split(";") if x.strip()])
widths = {"en1990_case": 7, "en1990_case_number": 7, "en1990_case_text": 5}
p = B + "/🦀️.rs"
sub(p, "TestSeries, Placement, VariableAction};", "TestSeries, Placement, VariableAction, CaseRow, CaseNumber, CaseText};")
sub(p, "const PLACEMENT_FLOATS:", "const CASE_NUMBER_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4)];\nconst PLACEMENT_FLOATS:")
sub(p, "Cell::Real(placement.yaw)], PLACEMENT_FLOATS)?;\n        }\n", '''Cell::Real(placement.yaw)], PLACEMENT_FLOATS)?;
        }
        for (position, case) in self.cases.iter().enumerate() {
            let id = out.insert_float("en1990_case", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&case.id), Cell::Text(&case.label_en), Cell::Text(&case.label_de), Cell::Text(&case.entry_id)], &[])?;
            for (index, number) in case.numbers.iter().enumerate() { out.insert_float("en1990_case_number", &[Cell::Integer(id), ordinal(index)?, Cell::Text(&number.id), Cell::Real(number.value)], CASE_NUMBER_FLOATS)?; }
            for (index, text) in case.texts.iter().enumerate() { out.insert_float("en1990_case_text", &[Cell::Integer(id), ordinal(index)?, Cell::Text(&text.id), Cell::Text(&text.value)], &[])?; }
        }
''')
sub(p, "sqlite_native::schema(En1990Snapshot::SQLITE_SCHEMA,13,57,control)", "sqlite_native::schema(En1990Snapshot::SQLITE_SCHEMA,16,76,control)")
sub(p, "self.test_series.len(),self.placements.len()]{rows=add(rows,size)?}", "self.test_series.len(),self.placements.len(),self.cases.len()]{rows=add(rows,size)?}for case in &self.cases{rows=add(rows,case.numbers.len())?;rows=add(rows,case.texts.len())?}")
sub(p, "        for placement in &self.placements { bound.add(4096)?; bound.repeated(placement.id.len(), 24)?; }\n", '''        for placement in &self.placements { bound.add(4096)?; bound.repeated(placement.id.len(), 24)?; }
        for case in &self.cases { bound.add(4096)?; for value in [&case.id, &case.label_en, &case.label_de, &case.entry_id] { bound.repeated(value.len(), 24)?; } for number in &case.numbers { bound.add(2048)?; bound.repeated(number.id.len(), 24)?; } for text in &case.texts { bound.add(2048)?; for value in [&text.id, &text.value] { bound.repeated(value.len(), 24)?; } } }
''')
sub(p, '        let placement_rows = entities(database, "en1990_placement", 8, PLACEMENT_FLOATS, control)?;\n', '''        let placement_rows = entities(database, "en1990_placement", 8, PLACEMENT_FLOATS, control)?;
        let case_rows = entities(database, "en1990_case", 7, &[], control)?;
        let mut case_numbers = BTreeMap::<i64, Vec<FloatRow<'_>>>::new();
        for row in entities(database, "en1990_case_number", 7, CASE_NUMBER_FLOATS, control)?.into_values() { let parent = row.integer(1)?; if !case_rows.contains_key(&parent) { return Err(invalid("EN1990 case number has an unknown case")); } case_numbers.entry(parent).or_default().push(row); }
        let mut case_texts = BTreeMap::<i64, Vec<FloatRow<'_>>>::new();
        for row in entities(database, "en1990_case_text", 5, &[], control)?.into_values() { let parent = row.integer(1)?; if !case_rows.contains_key(&parent) { return Err(invalid("EN1990 case text has an unknown case")); } case_texts.entry(parent).or_default().push(row); }
''')
sub(p, "        let result = Self { annex: match", '''        let mut cases = Vec::new();
        for row in ordered(case_rows.values().copied(), 2, control)? {
            checkpoint(control, cases.len(), 0)?;
            if row.integer(1)? != 1 { return Err(invalid("EN1990 case has an unknown document")); }
            let mut numbers = Vec::new();
            for number in ordered(case_numbers.remove(&row.rowid).unwrap_or_default().into_iter(), 2, control)? { numbers.push(CaseNumber { id: reconstruct_text(control, number.text(3)?)?, value: number.real(4)? }); }
            let mut texts = Vec::new();
            for text in ordered(case_texts.remove(&row.rowid).unwrap_or_default().into_iter(), 2, control)? { texts.push(CaseText { id: reconstruct_text(control, text.text(3)?)?, value: reconstruct_text(control, text.text(4)?)? }); }
            cases.push(CaseRow { id: reconstruct_text(control, row.text(3)?)?, label_en: reconstruct_text(control, row.text(4)?)?, label_de: reconstruct_text(control, row.text(5)?)?, entry_id: reconstruct_text(control, row.text(6)?)?, numbers, texts });
        }
        let result = Self { annex: match''')
sub(p, "reference_period_years: document.real(9)?", "reference_period_years: document.real(9)?")
sub(p, "test_series, placements };", "test_series, placements, cases };")
# admission
p = B + "/🛂️admission/🦀️.rs"
s = rd(p)
a = 'PLACEMENT_FLOATS)?;}Ok(())'
assert s.count(a) == 1
wr(p, s.replace(a, 'PLACEMENT_FLOATS)?;}\n for(index,c)in n::list(n::field(root,24)?)?.iter().enumerate(){let c=n::record(c)?;let parent=n::entity(out,"en1990_case",1,index,&n::cells(c,[Text(0),Text(1),Text(2),Text(3)])?,&[])?;\n for(i,r)in n::list(n::field(c,4)?)?.iter().enumerate(){n::entity(out,"en1990_case_number",parent,i,&n::cells(n::record(r)?,[Text(0),Real(1)])?,CASE_NUMBER_FLOATS)?;}\n for(i,r)in n::list(n::field(c,5)?)?.iter().enumerate(){n::entity(out,"en1990_case_text",parent,i,&n::cells(n::record(r)?,[Text(0),Text(1)])?,&[])?;}}\n Ok(())'))
print("schemaBytes", tot)
import json
sem = B + "/🧪️tests/🧫️fixtures/🎛️semantic.json"
j = json.loads(rd(sem)); j["schemaBytes"] = tot; j["tableWidths"].update(widths); j["tableWidths"] = dict(sorted(j["tableWidths"].items()))
wr(sem, json.dumps(j, indent=2, ensure_ascii=False) + "\n")
print(open(B + "/🛂️admission/🦀️.rs").read()[:900])
