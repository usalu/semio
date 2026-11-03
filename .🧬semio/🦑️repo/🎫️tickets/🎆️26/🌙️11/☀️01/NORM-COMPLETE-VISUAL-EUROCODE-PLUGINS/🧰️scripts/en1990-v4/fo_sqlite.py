"""Layer 3: SQLite (DDL, Rust and TS codecs, admission, tests, contract) of `factorOverrides`; also repairs the admission cell lists of members and bridge_sls that missed the v3/v4 float columns, and the table/column limits."""
import sqlite3
from fo_lib import *

TB = TABLE
BASE = "id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1990_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL"
DDL = "CREATE TABLE %s(%s, factor TEXT NOT NULL, target TEXT NOT NULL, value REAL, justification TEXT NOT NULL, value_ieee754_bits INTEGER NOT NULL, value_ieee754_class TEXT NOT NULL);" % (TB, BASE)


def extent(sql):
    db = sqlite3.connect(":memory:")
    db.executescript(sql)
    rows = db.execute("SELECT name, sql FROM sqlite_schema WHERE type='table' ORDER BY name").fetchall()
    widths = {n: len(db.execute('PRAGMA table_info("%s")' % n).fetchall()) for n, _ in rows}
    return sum(len(n.encode()) + len(s.encode()) for n, s in rows), widths


def sql():
    def go(t):
        return t if TB in t else t.rstrip("\n") + "\n" + DDL + "\n"
    edit(SQ + "/🗄️.sql", go)


def rs():
    def go(t):
        if "FACTOR_OVERRIDE_FLOATS" in t:
            return t
        t = put(t, "TestSeries, Placement, VariableAction,", "TestSeries, Placement, FactorOverride, VariableAction,")
        t = put(t, "const PLACEMENT_FLOATS: &[FloatColumn] =", "const FACTOR_OVERRIDE_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(6)];\nconst PLACEMENT_FLOATS: &[FloatColumn] =")
        write = '        for (position, row) in self.factor_overrides.iter().enumerate() {\n            out.insert_float("%s", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Text(&row.factor), Cell::Text(&row.target), Cell::Real(row.value), Cell::Text(&row.justification)], FACTOR_OVERRIDE_FLOATS)?;\n        }\n' % TB
        t = put(t, "        Ok(())\n }\n fn admit_sqlite_values", write + "        Ok(())\n }\n fn admit_sqlite_values")
        t = sub1(t, r"SQLITE_SCHEMA,16,\d+,control\)", "SQLITE_SCHEMA,17,96,control)")
        t = put(t, "self.placements.len(),self.cases.len()]{rows=add(rows,size)?}", "self.placements.len(),self.cases.len(),self.factor_overrides.len()]{rows=add(rows,size)?}")
        t = put(t, "        for placement in &self.placements { bound.add(4096)?; bound.repeated(placement.id.len(), 24)?; }\n", "        for placement in &self.placements { bound.add(4096)?; bound.repeated(placement.id.len(), 24)?; }\n        for row in &self.factor_overrides { bound.add(4096)?; for value in [&row.id, &row.factor, &row.target, &row.justification] { bound.repeated(value.len(), 24)?; } }\n")
        t = put(t, '        let case_rows = entities(database, "en1990_case", 7, &[], control)?;\n', '        let override_rows = entities(database, "%s", 8, FACTOR_OVERRIDE_FLOATS, control)?;\n        let case_rows = entities(database, "en1990_case", 7, &[], control)?;\n' % TB)
        read = '        let mut factor_overrides = Vec::new();\n        for row in ordered(override_rows.values().copied(), 2, control)? {\n            checkpoint(control, factor_overrides.len(), 0)?;\n            if row.integer(1)? != 1 { return Err(invalid("EN1990 factor override has an unknown document")); }\n            factor_overrides.push(FactorOverride { id: reconstruct_text(control, row.text(3)?)?, factor: reconstruct_text(control, row.text(4)?)?, target: reconstruct_text(control, row.text(5)?)?, value: row.real(6)?, justification: reconstruct_text(control, row.text(7)?)? });\n        }\n'
        t = put(t, "        let result = Self { annex:", read + "        let result = Self { annex:")
        return put(t, "test_series, placements, cases };", "test_series, placements, cases, factor_overrides };")
    edit(SQ + "/🦀️.rs", go)


def ts():
    def go(t):
        if "FACTOR_OVERRIDE_FLOATS" in t:
            return t
        t = put(t, "En1990Placement as Placement,", "En1990Placement as Placement,En1990FactorOverride as FactorOverride,")
        t = put(t, 'export type En1990SqliteCaseNumber=', 'export type En1990SqliteFactorOverride=Omit<FactorOverride,"value">&{value:Binary64};\nexport type En1990SqliteCaseNumber=')
        t = put(t, '|"placements"|"cases">&{', '|"placements"|"cases"|"factorOverrides">&{')
        t = put(t, "cases:En1990SqliteCase[]};", "cases:En1990SqliteCase[];factorOverrides:En1990SqliteFactorOverride[]};")
        t = put(t, "const PLACEMENT_FLOATS:readonly Ieee754Column[]=", "const FACTOR_OVERRIDE_FLOATS:readonly Ieee754Column[]=[{index:6,width:64}];\nconst PLACEMENT_FLOATS:readonly Ieee754Column[]=")
        w = ' for(let position=0;position<snapshot.factorOverrides.length;position++){const row=snapshot.factorOverrides[position]!;await write(out,"%s",BigInt(position+1),[1n,BigInt(position),row.id,row.factor,row.target,row.value,row.justification],FACTOR_OVERRIDE_FLOATS,options)}\n' % TB
        t = put(t, " return out.finish();\n}", w + " return out.finish();\n}")
        t = put(t, "const caseTextRows=await entities(tables[15]!,5,[],options);", "const caseTextRows=await entities(tables[15]!,5,[],options);const overrideRows=await entities(tables[16]!,8,FACTOR_OVERRIDE_FLOATS,options);")
        r = " const factorOverrides:En1990SqliteFactorOverride[]=[];for(const row of await ordered([...overrideRows.values()],options)){await checkpoint(options,factorOverrides.length,0);factorOverrides.push({id:artifactSqliteText(row,3),factor:artifactSqliteText(row,4),target:artifactSqliteText(row,5),value:readBinary64(row,6,FACTOR_OVERRIDE_FLOATS),justification:artifactSqliteText(row,7)})}\n"
        t = put(t, " const annex=artifactSqliteText(document,1);if(", r + " const annex=artifactSqliteText(document,1);if(")
        return put(t, "testSeries,placements,cases};", "testSeries,placements,cases,factorOverrides};")
    edit(SQ + "/🟦️.ts", go)


def admission():
    def go(t):
        if TB in t:
            return t
        t = put(t, "for id in 17..=24{rows=n::add", "for id in 17..=25{rows=n::add")
        reals_m = ",".join("Real(%d)" % i for i in range(20, 33))
        t = put(t, "Real(18),Real(19)])?,MEMBER_FLOATS", "Real(18),Real(19),%s])?,MEMBER_FLOATS" % reals_m)
        extra = "".join(",Cell::Real(n::real(n::field(b,%d)?)?)" % i for i in range(8, 16))
        t = put(t, "Cell::Real(n::real(n::field(b,7)?)?)],BRIDGE_FLOATS", "Cell::Real(n::real(n::field(b,7)?)?)%s],BRIDGE_FLOATS" % extra)
        row = ' for(index,o)in n::list(n::field(root,25)?)?.iter().enumerate(){n::entity(out,"%s",1,index,&n::cells(n::record(o)?,[Text(0),Text(1),Text(2),Real(3),Text(4)])?,FACTOR_OVERRIDE_FLOATS)?;}\n' % TB
        t = put(t, " Ok(())\n}\n/// 🛂️ Sixteen SQL tables", row + " Ok(())\n}\n/// 🛂️ Seventeen SQL tables")
        t = t.replace("SQLITE_SCHEMA,16,57", "SQLITE_SCHEMA,17,96")
        return t
    edit(SQ + "/🛂️admission/🦀️.rs", go)


def tests():
    def rs(t):
        if "factor_overrides" in t:
            return t
        t = put(t, "    for placement in &mut snapshot.placements { placement.x = value; placement.y = value; placement.z = value; placement.yaw = value; }\n", "    for placement in &mut snapshot.placements { placement.x = value; placement.y = value; placement.z = value; placement.yaw = value; }\n    for row in &mut snapshot.factor_overrides { row.value = value; }\n")
        t = put(t, "empty.placements.clear();", "empty.placements.clear(); empty.factor_overrides.clear();")
        t = put(t, "source.placements.clear();", "source.placements.clear();source.factor_overrides.clear();")
        t = t.replace("tables.len(), 15)", "tables.len(), 17)").replace("max_tables:13,max_columns:57", "max_tables:17,max_columns:96").replace('("tables",16),("columns",57)', '("tables",17),("columns",96)')
        return t
    edit(SQ + "/🧪️tests/🦀️.rs", rs)

    def tt(t):
        if "factorOverrides" in t:
            return t
        t = put(t, "yaw:binary64(x.yaw)}))};", "yaw:binary64(x.yaw)})),factorOverrides:f.factorOverrides.map(x=>({...x,value:binary64(x.value)}))};")
        t = put(t, " for(const x of s.placements){x.x=value;x.y=value;x.z=value;x.yaw=value}\n", " for(const x of s.placements){x.x=value;x.y=value;x.z=value;x.yaw=value}\n for(const x of s.factorOverrides)x.value=value\n")
        t = put(t, "testSeries:[],placements:[]};", "testSeries:[],placements:[],cases:[],factorOverrides:[]};") if "testSeries:[],placements:[],cases" not in t else t
        t = put(t, "source.placements=[];", "source.placements=[];source.factorOverrides=[];")
        t = t.replace("maxTables:16,maxColumns:57", "maxTables:17,maxColumns:96").replace("tables.length).toBe(16)", "tables.length).toBe(17)")
        return t
    edit(SQ + "/🧪️tests/🟦️.ts", tt)


def contract():
    old_sql = rd(SQ + "/🗄️.sql")
    size, widths = extent(old_sql)
    p = SQ + "/🧪️tests/🧫️fixtures/🎛️semantic.json"
    d = json.loads(rd(p))
    d["schemaBytes"] = size
    d["tableWidths"] = dict(sorted(widths.items()))
    wr(p, json.dumps(d, indent=2, ensure_ascii=False) + "\n")
    tp = SQ + "/🧪️tests/🟦️.ts"
    t = rd(tp)
    t = re.sub(r'(normSemanticContract\["schemaBytes"\]\)\.toEqual\()\d+\)', lambda m: m.group(1) + str(size) + ")", t)
    t = re.sub(r'(normSemanticContract\["tableWidths"\]\)\.toEqual\()\{[^}]*\}\)', lambda m: m.group(1) + json.dumps(d["tableWidths"], separators=(",", ":")) + ")", t)
    wr(tp, t)
    print("  contract", size, max(widths.values()), len(widths))


def main():
    sql()
    rs()
    ts()
    admission()
    tests()
    contract()


if __name__ == "__main__":
    main()
