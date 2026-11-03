"""en1992 `members[].studRows[]` SQLite layer: table `en1992_stud_row` replaces `en1992_stud_ring` (angle, count, spacing_first, spacing_radial, diameter; 4 ieee columns), rust and ts write/read, native cells (width 17), contract fixture (table list, tableWidths). Idempotent."""
import json, re
Q = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/"
def rw(path, fn):
    t = open(Q + path, encoding="utf8").read()
    u = fn(t)
    if u != t:
        open(Q + path, "w", encoding="utf8").write(u); print("edited", path)
def sql(t):
    a = t.index("CREATE TABLE en1992_stud_ring ("); b = t.index(");\n", a) + 3
    new = """CREATE TABLE en1992_stud_row (
 id INTEGER PRIMARY KEY, member_id INTEGER NOT NULL REFERENCES en1992_member(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, angle REAL, count INTEGER NOT NULL CHECK(count BETWEEN 0 AND 4294967295), spacing_first REAL, spacing_radial REAL, diameter REAL,
 angle_ieee754_bits INTEGER NOT NULL, angle_ieee754_class TEXT NOT NULL,
 spacing_first_ieee754_bits INTEGER NOT NULL, spacing_first_ieee754_class TEXT NOT NULL,
 spacing_radial_ieee754_bits INTEGER NOT NULL, spacing_radial_ieee754_class TEXT NOT NULL,
 diameter_ieee754_bits INTEGER NOT NULL, diameter_ieee754_class TEXT NOT NULL
);
"""
    return t[:a] + new + t[b:]
rw("🗄️.sql", sql)
def rs(t):
    t = t.replace("StudRing, Opening,", "StudRow, Opening,")
    t = t.replace("const STUD:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(6)];", "const STUD:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8)];")
    t = t.replace('for(k,a)in v.stud_rings.iter().enumerate(){p.insert_float("en1992_stud_ring",&[Cell::Integer(id),ordinal(k)?,Cell::Text(&a.id),Cell::Real(a.radius),Cell::Integer(i64::from(a.count)),Cell::Real(a.diameter)],STUD)?;}', 'for(k,a)in v.stud_rows.iter().enumerate(){p.insert_float("en1992_stud_row",&[Cell::Integer(id),ordinal(k)?,Cell::Text(&a.id),Cell::Real(a.angle),Cell::Integer(i64::from(a.count)),Cell::Real(a.spacing_first),Cell::Real(a.spacing_radial),Cell::Real(a.diameter)],STUD)?;}')
    t = t.replace('entities(d,"en1992_stud_ring",7,STUD,c)', 'entities(d,"en1992_stud_row",9,STUD,c)')
    t = t.replace("let mut stud_rings=Vec::new();", "let mut stud_rows_out=Vec::new();").replace("stud_rings.push(StudRing{id:text(a,3,c)?,radius:a.real(4)?,count:u32_value(a,5)?,diameter:a.real(6)?});", "stud_rows_out.push(StudRow{id:text(a,3,c)?,angle:a.real(4)?,count:u32_value(a,5)?,spacing_first:a.real(6)?,spacing_radial:a.real(7)?,diameter:a.real(8)?});")
    return t.replace("tendon_points,stud_rings,openings", "tendon_points,stud_rows:stud_rows_out,openings")
rw("🦀️.rs", rs)
def ts(t):
    t = t.replace("export type En1992SqliteStudRing={id:string;radius:Binary64;count:number;diameter:Binary64};", "export type En1992SqliteStudRow={id:string;angle:Binary64;count:number;spacingFirst:Binary64;spacingRadial:Binary64;diameter:Binary64};")
    t = t.replace("studRings:En1992SqliteStudRing[]", "studRows:En1992SqliteStudRow[]")
    t = t.replace("const STUD_RING:readonly Ieee754Column[]=[4,6]", "const STUD_ROW:readonly Ieee754Column[]=[4,6,7,8]").replace("STUD_RING", "STUD_ROW")
    t = t.replace('"studRings"', '"studRows"').replace("collection(v.studRings)", "collection(v.studRows)")
    t = re.sub(r"for\(let k=0;k<v\.studRings\.length;k\+\+\)\{const a=v\.studRings\[k\]!;fields\(a,\[[^\]]*\]\);await write\(out,\"en1992_stud_ring\",\[[^\]]*\],STUD_ROW,o\)\}", 'for(let k=0;k<v.studRows.length;k++){const a=v.studRows[k]!;fields(a,["id","angle","count","spacingFirst","spacingRadial","diameter"]);await write(out,"en1992_stud_row",[id,BigInt(k),a.id,a.angle,unsigned(a.count),a.spacingFirst,a.spacingRadial,a.diameter],STUD_ROW,o)}', t)
    t = t.replace("entities(t[16]!,7,STUD_ROW,o)", "entities(t[16]!,9,STUD_ROW,o)")
    t = t.replace("studRings.push({id:artifactSqliteText(a,3),radius:readBinary64(a,4,STUD_ROW),count:readUnsigned(a,5),diameter:readBinary64(a,6,STUD_ROW)})", "studRows.push({id:artifactSqliteText(a,3),angle:readBinary64(a,4,STUD_ROW),count:readUnsigned(a,5),spacingFirst:readBinary64(a,6,STUD_ROW),spacingRadial:readBinary64(a,7,STUD_ROW),diameter:readBinary64(a,8,STUD_ROW)})")
    t = t.replace("studRings:En1992SqliteStudRow[]=[]", "studRows:En1992SqliteStudRow[]=[]").replace("studRings:En1992SqliteStudRing[]=[]", "studRows:En1992SqliteStudRow[]=[]")
    return t.replace("tendonPoints,studRings,openings", "tendonPoints,studRows,openings")
rw("🟦️.ts", ts)
def cells(t):
    t = t.replace('("en1992_stud_ring",11)', '("en1992_stud_row",17)')
    return t.replace('collection(field(value,36)?,4,"en1992_stud_ring",&[(0,Text),(1,Float),(2,UInt),(3,Float)]', 'collection(field(value,36)?,6,"en1992_stud_row",&[(0,Text),(1,Float),(2,UInt),(3,Float),(4,Float),(5,Float)]')
rw("📏️cells/🦀️.rs", cells)
def fx(t):
    d = json.loads(t)
    for k, v in d.items():
        if isinstance(v, list) and "en1992_stud_ring" in v:
            d[k] = ["en1992_stud_row" if x == "en1992_stud_ring" else x for x in v]
    d["semanticCells"]["tableWidths"] = sorted([["en1992_stud_row", 17] if n == "en1992_stud_ring" else [n, w] for n, w in d["semanticCells"]["tableWidths"]])
    return json.dumps(d, indent=2, ensure_ascii=False) + "\n"
rw("🧪️tests/🧫️fixtures/🔣️.json", fx)
