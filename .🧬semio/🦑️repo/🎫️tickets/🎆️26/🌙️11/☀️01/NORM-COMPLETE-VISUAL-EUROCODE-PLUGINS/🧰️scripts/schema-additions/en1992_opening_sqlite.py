"""en1992 `members[].openings[]` SQLite layer: table `en1992_opening` (appended to the DDL), rust write/read, ts write/read, native cells (member shape 38, collection field 37, width 16), contract fixture (table list, tableWidths). Idempotent."""
import json, re
Q = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/"
rd = lambda p: open(Q + p, encoding="utf8").read()


def sub(path, old, new, count=1):
    text = rd(path)
    if new in text:
        return print("present", path, old[:40])
    assert text.count(old) == count, (path, old[:70], text.count(old))
    open(Q + path, "w", encoding="utf8").write(text.replace(old, new))


sql = rd("🗄️.sql")
if "en1992_opening" not in sql:
    open(Q + "🗄️.sql", "w", encoding="utf8").write(sql.rstrip("\n") + """
CREATE TABLE en1992_opening (
 id INTEGER PRIMARY KEY, member_id INTEGER NOT NULL REFERENCES en1992_member(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, x REAL, y REAL, width REAL, depth REAL,
 x_ieee754_bits INTEGER NOT NULL, x_ieee754_class TEXT NOT NULL,
 y_ieee754_bits INTEGER NOT NULL, y_ieee754_class TEXT NOT NULL,
 width_ieee754_bits INTEGER NOT NULL, width_ieee754_class TEXT NOT NULL,
 depth_ieee754_bits INTEGER NOT NULL, depth_ieee754_class TEXT NOT NULL
);
""")
r = "🦀️.rs"
sub(r, "TendonPoint, StudRing,", "TendonPoint, StudRing, Opening,")
sub(r, "const STUD:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(6)];", "const STUD:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(6)];\nconst OPENING:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7)];")
sub(r, "Cell::Real(a.diameter)],STUD)?;}\n", "Cell::Real(a.diameter)],STUD)?;}\n   for(k,a)in v.openings.iter().enumerate(){p.insert_float(\"en1992_opening\",&[Cell::Integer(id),ordinal(k)?,Cell::Text(&a.id),Cell::Real(a.x),Cell::Real(a.y),Cell::Real(a.width),Cell::Real(a.depth)],OPENING)?;}\n")
sub(r, 'let mut stud_rows=groups(entities(d,"en1992_stud_ring",7,STUD,c)?,&member_rows,c)?;', 'let mut stud_rows=groups(entities(d,"en1992_stud_ring",7,STUD,c)?,&member_rows,c)?;let mut opening_rows=groups(entities(d,"en1992_opening",8,OPENING,c)?,&member_rows,c)?;')
sub(r, "   let stirrups=stirrups.remove(&v.rowid)", "   let mut openings=Vec::new();for a in ordered(opening_rows.remove(&v.rowid).unwrap_or_default(),v.rowid,c)?{openings.push(Opening{id:text(a,3,c)?,x:a.real(4)?,y:a.real(5)?,width:a.real(6)?,depth:a.real(7)?});}\n   let stirrups=stirrups.remove(&v.rowid)")
sub(r, "tendon_points,stud_rings", "tendon_points,stud_rings,openings")
t = "🟦️.ts"
sub(t, "export type En1992SqliteStudRing=", "export type En1992SqliteOpening={id:string;x:Binary64;y:Binary64;width:Binary64;depth:Binary64};\nexport type En1992SqliteStudRing=")
sub(t, "studRings:En1992SqliteStudRing[]};", "studRings:En1992SqliteStudRing[];openings:En1992SqliteOpening[]};")
sub(t, '"tendonPoints","studRings"', '"tendonPoints","studRings","openings"')
sub(t, "collection(v.studRings);", "collection(v.studRings);collection(v.openings);")
sub(t, "const STUD_RING:readonly Ieee754Column[]=[4,6].map(index=>({index,width:64}));", "const STUD_RING:readonly Ieee754Column[]=[4,6].map(index=>({index,width:64}));\nconst OPENING:readonly Ieee754Column[]=[4,5,6,7].map(index=>({index,width:64}));")
tx = rd(t)
w = re.search(r'(  for\(let k=0;k<v\.studRings\.length;k\+\+\)\{.*?\}\n)', tx)
assert w
if "en1992_opening" not in tx:
    tx = tx.replace(w.group(1), w.group(1) + '  for(let k=0;k<v.openings.length;k++){const a=v.openings[k]!;fields(a,["id","x","y","width","depth"]);await write(out,"en1992_opening",[id,BigInt(k),a.id,a.x,a.y,a.width,a.depth],OPENING,o)}\n')
    tx = tx.replace("studRows=await groups(await entities(t[16]!,7,STUD_RING,o),memberRows,o),", "studRows=await groups(await entities(t[16]!,7,STUD_RING,o),memberRows,o),openingRows=await groups(await entities(t[21]!,8,OPENING,o),memberRows,o),")
    tx = tx.replace("studRings:En1992SqliteStudRing[]=[];", "studRings:En1992SqliteStudRing[]=[],openings:En1992SqliteOpening[]=[];")
    tx = re.sub(r'(  for\(const a of await ordered\(studRows\.get\(v\.rowid\)\?\?\[\],v\.rowid,o\)\)studRings\.push\(\{[^\n]*\}\);\n)', lambda m: m.group(1) + "  for(const a of await ordered(openingRows.get(v.rowid)??[],v.rowid,o))openings.push({id:artifactSqliteText(a,3),x:readBinary64(a,4,OPENING),y:readBinary64(a,5,OPENING),width:readBinary64(a,6,OPENING),depth:readBinary64(a,7,OPENING)});\n", tx)
    tx = tx.replace("tendonPoints,studRings,longitudinal", "tendonPoints,studRings,openings,longitudinal")
    open(Q + t, "w", encoding="utf8").write(tx)
c = "📏️cells/🦀️.rs"
sub(c, '("en1992_member_action",31),', '("en1992_member_action",31),("en1992_opening",16),')
sub(c, "shape(value,37)?", "shape(value,38)?")
sub(c, '   collection(field(value,36)?,4,"en1992_stud_ring",&[(0,Text),(1,Float),(2,UInt),(3,Float)],&mut c,n)?;\n', '   collection(field(value,36)?,4,"en1992_stud_ring",&[(0,Text),(1,Float),(2,UInt),(3,Float)],&mut c,n)?;\n   collection(field(value,37)?,5,"en1992_opening",&[(0,Text),(1,Float),(2,Float),(3,Float),(4,Float)],&mut c,n)?;\n')
fx = "🧪️tests/🧫️fixtures/🔣️.json"
data = json.loads(rd(fx))
tables = data["tables"] if "tables" in data else None
changed = False
for key, val in data.items():
    if isinstance(val, list) and "en1992_case_text" in val and "en1992_opening" not in val:
        val.append("en1992_opening")
        changed = True
widths = data["semanticCells"]["tableWidths"]
if not any(n == "en1992_opening" for n, _ in widths):
    widths.append(["en1992_opening", 16])
    data["semanticCells"]["tableWidths"] = sorted(widths)
    changed = True
if changed:
    open(Q + fx, "w", encoding="utf8").write(json.dumps(data, indent=2, ensure_ascii=False) + "\n")
    print("fixture rewritten (indent 2)")
