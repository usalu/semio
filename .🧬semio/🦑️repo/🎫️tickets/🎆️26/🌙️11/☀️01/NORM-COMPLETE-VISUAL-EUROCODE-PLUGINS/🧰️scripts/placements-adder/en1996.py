import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from padd import *

A = Art("🪨️en1996", "En1996", "walls", "walls", "Wall", model="MasonryWall")


def inline_root_ts(path):
    inline = "{ id: string; x: number; y: number; z: number; yaw: number; }"
    reader = "normWireObject<%s>({ id: normWireRequired(normWireString), x: normWireRequired(normWireNumber), y: normWireRequired(normWireNumber), z: normWireRequired(normWireNumber), yaw: normWireRequired(normWireNumber) })" % inline
    def go(t):
        if "placements:" in t:
            return t
        t = sub1(t, r"(\n  walls: [^\n]*\n)", lambda m: m.group(1) + "  placements: %s[];\n" % inline)
        return sub1(t, r"(export const parseEn1996Snapshot: [^\n]*)( \}\);)$", lambda m: m.group(1) + ", placements: normWireRequired(normWireArray(%s))" % reader + m.group(2), re.M)
    edit(path, go)


def sql_multiline(table):
    cols = ["id INTEGER PRIMARY KEY", "document_id INTEGER NOT NULL REFERENCES en1996_document(id)", "ordinal INTEGER NOT NULL CHECK(ordinal>=0)", "logical_id TEXT NOT NULL", "x REAL", "y REAL", "z REAL", "yaw REAL"] + ["%s_ieee754_bits INTEGER NOT NULL, %s_ieee754_class TEXT NOT NULL" % (f, f) for f in FIELDS]
    return "CREATE TABLE %s (\n  %s\n);\n" % (table, ",\n  ".join(cols))


def sqlite_layers():
    q = "/🚪️io/🪶️sqlite/📸️snapshot"
    TB = "en1996_placement"
    edit(A.path(q + "/🗄️.sql"), lambda t: t if TB in t else t.rstrip("\n") + "\n" + sql_multiline(TB))

    def rs(t):
        if "PLACEMENT_FLOATS" in t:
            return t
        t = sub1(t, r"\{MasonryClass, MasonryWall,", "{MasonryClass, MasonryWall, Placement,")
        t = sub1(t, r"(const WALL_FLOATS[^\n]*\n)", r"\1const PLACEMENT_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7)];\n")
        t = sub1(t, r"(\n        Ok\(\(\)\)\n    \}\n)", lambda m: '\n        for(position,placement)in self.placements.iter().enumerate(){out.insert_float("en1996_placement",&[Cell::Integer(1),ordinal(position)?,Cell::Text(&placement.id),Cell::Real(placement.x),Cell::Real(placement.y),Cell::Real(placement.z),Cell::Real(placement.yaw)],PLACEMENT_FLOATS)?;}' + m.group(1), count=1)
        t = sub1(t, r"let mut rows=add\(1,self\.walls\.len\(\)\)\?;", "let mut rows=add(add(1,self.walls.len())?,self.placements.len())?;")
        t = sub1(t, r"(        for wall in &self\.walls\{[^\n]*\n)", lambda m: m.group(1) + "        for placement in &self.placements{bound.add(4096)?;bound.repeated(placement.id.len(),24)?;}\n")
        t = sub1(t, r"(let wall_rows=entities\(database,\"en1996_wall\",45,WALL_FLOATS,control\)\?;)", r'\1let placement_rows=entities(database,"en1996_placement",8,PLACEMENT_FLOATS,control)?;')
        t = sub1(t, r"(\n        let snapshot=Self\{)", lambda m: '\n        let mut placements=Vec::new();for row in ordered(placement_rows.into_values(),1,control)?{checkpoint(control,placements.len(),0)?;placements.push(Placement{id:reconstruct_text(control,row.text(3)?)?,x:row.real(4)?,y:row.real(5)?,z:row.real(6)?,yaw:row.real(7)?});}' + m.group(1))
        t = sub1(t, r"([,{ ])walls\};", r"\1walls,placements};")
        t = sub1(t, r"SQLITE_SCHEMA,12,101", "SQLITE_SCHEMA,13,101")
        return t
    edit(A.path(q + "/🦀️.rs"), rs)

    def ts(t):
        if "PLACEMENT_FLOATS" in t:
            return t
        t = sub1(t, r"import type\{En1996Snapshot,MasonryWall,", "import type{En1996Snapshot,MasonryWall,Placement,")
        t = sub1(t, r'(export type En1996SqliteSnapshot=Omit<En1996Snapshot,[^\n]*?)\|"walls">&\{', r'\1|"walls"|"placements">\&{')
        t = sub1(t, r"walls:En1996SqliteWall\[\]\}", "walls:En1996SqliteWall[];placements:En1996SqlitePlacement[]}")
        t = sub1(t, r"(export type En1996SqliteSnapshot=)", 'export type En1996SqlitePlacement=Omit<Placement,"x"|"y"|"z"|"yaw">&{x:Binary64;y:Binary64;z:Binary64;yaw:Binary64};\n\\1')
        t = sub1(t, r"(const WALL_FLOATS[^\n]*\n)", r"\1const PLACEMENT_FLOATS:readonly Ieee754Column[]=[4,5,6,7].map(index=>({index,width:64}));\n")
        t = sub1(t, r"\n return out\.finish\(\);", '\n for(let position=0;position<snapshot.placements.length;position++){const p=snapshot.placements[position]!;await write(out,"en1996_placement",[1n,BigInt(position),p.id,p.x,p.y,p.z,p.yaw],PLACEMENT_FLOATS,options)}\n return out.finish();')
        t = sub1(t, r"(\n const walls:En1996SqliteWall\[\]=\[\];)", lambda m: '\n const placementRows=await entities(tables[12]!,8,PLACEMENT_FLOATS,options);const placements:En1996SqlitePlacement[]=[];for(const r of await ordered(placementRows.values(),1n,options)){await checkpoint(options,placements.length,0);placements.push({id:artifactSqliteText(r,3),x:readBinary64(r,4,PLACEMENT_FLOATS),y:readBinary64(r,5,PLACEMENT_FLOATS),z:readBinary64(r,6,PLACEMENT_FLOATS),yaw:readBinary64(r,7,PLACEMENT_FLOATS)})}' + m.group(1))
        t = sub1(t, r",walls\};await artifactSqliteCheckpoint", ",walls,placements};await artifactSqliteCheckpoint")
        return t
    edit(A.path(q + "/🟦️.ts"), ts)


def rest_layers():
    q = "/🚪️io/🪶️sqlite/📸️snapshot"
    def adm(t):
        if "en1996_placement" in t:
            return t
        t = sub1(t, r"let mut rows=n::add\(1,walls\.len\(\)\)\?;", "let mut rows=n::add(n::add(1,walls.len())?,n::list(n::field(root,5)?)?.len())?;")
        t = sub1(t, r"\}\}\}Ok\(\(\)\)\n\}", '}}}\n for(index,p)in n::list(n::field(root,5)?)?.iter().enumerate(){n::entity(out,"en1996_placement",1,index,&n::cells(n::record(p)?,[Text(0),Real(1),Real(2),Real(3),Real(4)])?,PLACEMENT_FLOATS)?;}Ok(())\n}')
        t = sub1(t, r"SQLITE_SCHEMA,12,101", "SQLITE_SCHEMA,13,101")
        return t.replace("Twelve authored", "Thirteen authored")
    edit(A.path(q + "/🛂️admission/🦀️.rs"), adm)
    def rt(t):
        t = t.replace("max_tables:12", "max_tables:13").replace('("tables",12)', '("tables",13)')
        return t
    edit(A.path(q + "/🧪️tests/🦀️.rs"), rt)
    edit(A.path(q + "/🧪️tests/🟦️.ts"), lambda t: t.replace("toBe(12)", "toBe(13)").replace("maxTables:12", "maxTables:13"))
    rows = ['    ("placements", FieldSpec::new("Placements", "Platzierungen")),', '    ("placements[].id", FieldSpec::new("Placed entity id", "Platzierte Element-Id")),'] + ['    ("placements[].%s", FieldSpec::quantity("Placement %s", "Platzierung %s", UnitId::Metre)),' % (f, f, f) for f in "xyz"] + ['    ("placements[].yaw", FieldSpec::quantity("Placement yaw about Z", "Platzierung Gierwinkel um Z", UnitId::Radian)),']
    edit(A.path("/✏️editor/🏷️field-meta/🦀️.rs"), lambda t: append_after_last(t, r'^\s+\("walls\[\]\.\w+", ', rows))


if __name__ == "__main__":
    inline_root_ts(A.path("/🧬️schema/🟦️.ts"))
    core(A, root_ts=("/📸️snapshot/🟦️.ts",), twin_tok="Wall")
    sqlite_layers()
    rest_layers()
