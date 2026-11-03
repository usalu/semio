import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from padd import *

A = Art("⚖️en1990", "En1990", "test_series", "testSeries", "TestSeries")


def snapshot_layers():
    s = "/🧬️schema"
    layer_model_struct(A, A.path(s + "/📸️snapshot/🦀️.rs"))
    edit(A.path(s + "/📸️snapshot/🦀️.rs"), lambda t: clone_line(t, r"^\s+(pub )?test_series: (Vec<TestSeries>,|vec!\[\],)", A))
    edit(A.path(s + "/🦀️.rs"), lambda t: clone_line(t, r"^\s+(pub )?test_series: ", A))
    for f in ("/🟦️.ts", "/📸️snapshot/🟦️.ts"):
        ts_iface(A, A.path(s + f))
        ts_root_field(A, A.path(s + f))
    graphql_snapshot(A, A.path(s + "/🔗️.graphql"))
    graphql_snapshot(A, A.path(s + "/📸️snapshot/🔗️.graphql"))
    proto_snapshot(A, A.path(s + "/🛰️.proto"))
    proto_snapshot(A, A.path(s + "/📸️snapshot/🛰️.proto"))
    json_snapshot(A, A.path(s + "/🔣️.json"))
    json_snapshot(A, A.path(s + "/📸️snapshot/🔣️.json"))
    edit(A.path(s + "/🧪parse-en1990-artifact.bun.ts"), lambda t: clone_line(t, r"^\s+testSeries: \[\],", A))


def other_layers():
    field_meta_fieldspec(A, A.path("/✏️editor/🏷️field-meta/🦀️.rs"), A.path("/✏️editor/🏷️field-meta/🧪️tests/🔬️unit/🦀️.rs"))
    i = "/🧬️schema/💡️inferences"
    edit(A.path(i + "/🦀️.rs"), lambda t: clone_line(t, r'^\s+"testSeries",$', A, attrs=False))
    edit(A.path(i + "/🧾outline/🦀️.rs"), lambda t: sub1(clone_line(t, r'^\s+"testSeries",$', A, attrs=False), r"\+ snapshot\.test_series\.len\(\)\)", "+ snapshot.test_series.len() + snapshot.placements.len())") if "snapshot.placements.len()" not in t else t)
    edit(A.path(i + "/⛓️referential-integrity/🦀️.rs"), lambda t: clone_line(t, r'^\s+push_duplicate_ids\(report, document, "testSeries"', A, attrs=False))


def sqlite_layers():
    q = "/🚪️io/🪶️sqlite/📸️snapshot"
    TB = "en1990_placement"
    edit(A.path(q + "/🗄️.sql"), lambda t: t if TB in t else t.rstrip("\n") + "\n" + sql_table(A, TB) + "\n")

    def rs(t):
        if "PLACEMENT_FLOATS" in t:
            return t
        t = sub1(t, r"(use semio_s_artifact_norm_en1990_model::\{[^}]*?)TestSeries,", r"\1TestSeries, Placement,")
        t = sub1(t, r"(const TEST_FLOATS[^\n]*\n)", r"\1const PLACEMENT_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4), FloatColumn::Binary64(5), FloatColumn::Binary64(6), FloatColumn::Binary64(7)];\n")
        t = sub1(t, r"(        for \(position, series\) in self\.test_series\.iter\(\)\.enumerate\(\) \{\n[^\n]*\n        \}\n)", lambda m: m.group(1) + '        for (position, placement) in self.placements.iter().enumerate() {\n            out.insert_float( "en1990_placement", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&placement.id), Cell::Real(placement.x), Cell::Real(placement.y), Cell::Real(placement.z), Cell::Real(placement.yaw)], PLACEMENT_FLOATS)?;\n        }\n')
        t = sub1(t, r"self\.test_series\.len\(\)\]\{rows=add", "self.test_series.len(),self.placements.len()]{rows=add")
        t = sub1(t, r"(        for series in &self\.test_series \{[^\n]*\n)", lambda m: m.group(1) + "        for placement in &self.placements { bound.add(4096)?; bound.repeated(placement.id.len(), 24)?; }\n")
        t = sub1(t, r"(        let test_rows = entities\(database, \"en1990_test_series\", 12, TEST_FLOATS, control\)\?;\n)", lambda m: m.group(1) + '        let placement_rows = entities(database, "en1990_placement", 8, PLACEMENT_FLOATS, control)?;\n')
        t = sub1(t, r"(            test_series\.push\(TestSeries \{[^\n]*\n        \}\n)", lambda m: m.group(1) + '        let mut placements = Vec::new();\n        for row in ordered(placement_rows.values().copied(), 2, control)? {\n            checkpoint(control, placements.len(), 0)?;\n            if row.integer(1)? != 1 { return Err(invalid("EN1990 placement has an unknown document")); }\n            placements.push(Placement { id: reconstruct_text(control, row.text(3)?)?, x: row.real(4)?, y: row.real(5)?, z: row.real(6)?, yaw: row.real(7)? });\n        }\n')
        t = sub1(t, r"reliability_analyses, test_series \};", "reliability_analyses, test_series, placements };")
        t = sub1(t, r"SQLITE_SCHEMA,12,57,control", "SQLITE_SCHEMA,13,57,control")
        return t
    edit(A.path(q + "/🦀️.rs"), rs)

    def ts(t):
        if "PLACEMENT_FLOATS" in t:
            return t
        t = sub1(t, r"En1990TestSeries as TestSeries,", "En1990TestSeries as TestSeries,En1990Placement as Placement,")
        t = sub1(t, r"(export type En1990SqliteTestSeries=[^\n]*\n)", lambda m: m.group(1) + 'export type En1990SqlitePlacement=Omit<Placement,"x"|"y"|"z"|"yaw">&{x:Binary64;y:Binary64;z:Binary64;yaw:Binary64};\n')
        t = sub1(t, r'\|"testSeries">&\{', '|"testSeries"|"placements">&{')
        t = sub1(t, r"testSeries:En1990SqliteTestSeries\[\]\};", "testSeries:En1990SqliteTestSeries[];placements:En1990SqlitePlacement[]};")
        t = sub1(t, r"(const TEST_FLOATS[^\n]*\n)", r"\1const PLACEMENT_FLOATS:readonly Ieee754Column[]=[4,5,6,7].map(index=>({index,width:64}));\n")
        t = sub1(t, r"( for\(let position=0;position<snapshot\.testSeries\.length;position\+\+\)\{[^\n]*TEST_FLOATS,options\)\}\n)", lambda m: m.group(1) + ' for(let position=0;position<snapshot.placements.length;position++){const placement=snapshot.placements[position]!;await write(out,"en1990_placement",BigInt(position+1),[1n,BigInt(position),placement.id,placement.x,placement.y,placement.z,placement.yaw],PLACEMENT_FLOATS,options)}\n')
        t = sub1(t, r"(const testRows=await entities\(tables\[11\]!,12,TEST_FLOATS,options\);)", r"\1const placementRows=await entities(tables[12]!,8,PLACEMENT_FLOATS,options);")
        t = sub1(t, r"( const testSeries:En1990SqliteTestSeries\[\]=\[\];[^\n]*required:readBinary64\(row,11,TEST_FLOATS\)\}\)\}\n)", lambda m: m.group(1) + ' const placements:En1990SqlitePlacement[]=[];for(const row of await ordered([...placementRows.values()],options)){await checkpoint(options,placements.length,0);placements.push({id:artifactSqliteText(row,3),x:readBinary64(row,4,PLACEMENT_FLOATS),y:readBinary64(row,5,PLACEMENT_FLOATS),z:readBinary64(row,6,PLACEMENT_FLOATS),yaw:readBinary64(row,7,PLACEMENT_FLOATS)})}\n')
        t = sub1(t, r"reliabilityAnalyses,testSeries\};", "reliabilityAnalyses,testSeries,placements};")
        return t
    edit(A.path(q + "/🟦️.ts"), ts)

    def adm(t):
        if "en1990_placement" in t:
            return t
        t = sub1(t, r"(\n for\(index,s\)in n::list\(n::field\(root,22\)\?\)\?\.iter\(\)\.enumerate\(\)\{[^\n]*?)Ok\(\(\)\)\n", lambda m: m.group(1) + '\n for(index,p)in n::list(n::field(root,23)?)?.iter().enumerate(){n::entity(out,"en1990_placement",1,index,&n::cells(n::record(p)?,[Text(0),Real(1),Real(2),Real(3),Real(4)])?,PLACEMENT_FLOATS)?;}Ok(())\n')
        t = sub1(t, r"for id in 17\.\.=22\{", "for id in 17..=23{")
        t = sub1(t, r"SQLITE_SCHEMA,12,57", "SQLITE_SCHEMA,13,57", count=2)
        t = sub1(t, r"/// 🛂️ Twelve SQL tables", "/// 🛂️ Thirteen SQL tables")
        return t
    edit(A.path(q + "/🛂️admission/🦀️.rs"), adm)

    def rtests(t):
        if "placements" in t:
            return t
        t = sub1(t, r"(    for series in &mut snapshot\.test_series \{[^\n]*\n)", lambda m: m.group(1) + "    for placement in &mut snapshot.placements { placement.x = value; placement.y = value; placement.z = value; placement.yaw = value; }\n")
        t = sub1(t, r"empty\.test_series\.clear\(\);", "empty.test_series.clear(); empty.placements.clear();")
        t = sub1(t, r"source\.test_series\.clear\(\);", "source.test_series.clear();source.placements.clear();")
        t = sub1(t, r"max_tables:12", "max_tables:13")
        t = sub1(t, r'\("tables",12\)', '("tables",13)')
        return t
    edit(A.path(q + "/🧪️tests/🦀️.rs"), rtests)

    def ttests(t):
        if "placements" in t:
            return t
        t = sub1(t, r"(testSeries:f\.testSeries\.map\(x=>\(\{[^\n]*?\}\)\))", lambda m: m.group(1) + ",placements:f.placements.map(x=>({...x,x:binary64(x.x),y:binary64(x.y),z:binary64(x.z),yaw:binary64(x.yaw)}))")
        t = sub1(t, r"( for\(const x of s\.testSeries\)\{[^\n]*\}\n)", lambda m: m.group(1) + " for(const x of s.placements){x.x=value;x.y=value;x.z=value;x.yaw=value}\n")
        t = sub1(t, r"testSeries:\[\]\};", "testSeries:[],placements:[]};")
        t = sub1(t, r"source\.testSeries=\[\];", "source.testSeries=[];source.placements=[];")
        t = sub1(t, r"toBe\(12\)", "toBe(13)")
        t = sub1(t, r"maxTables:12", "maxTables:13")
        return t
    edit(A.path(q + "/🧪️tests/🟦️.ts"), ttests)
    sqlite_extent(A, A.path(q + "/🗄️.sql"), A.path(q + "/🧪️tests/🧫️fixtures/🎛️semantic.json"), A.path(q + "/🧪️tests/🟦️.ts"), None, 12, TB)


if __name__ == "__main__":
    snapshot_layers()
    diff_layers(A)
    sqlite_layers()
    literal_sweep(A)
    other_layers()
    fixtures(A)
