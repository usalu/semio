"""Applies the EN 1996 wave 2 entities to the SQLite snapshot layer (ddl, rust codec, admission). Idempotent."""
import os, re
HERE = os.path.dirname(os.path.abspath(__file__))
exec(compile(open(os.path.join(HERE, "en1996_spec.py"), encoding="utf-8").read(), "en1996_spec.py", "exec"))
SQ = f"{IO}/🪶️sqlite/📸️snapshot"
TABLE = {r["key"]: f"en1996_{r['key']}" for r in ROWS}
CONST = {r["key"]: f"{r['key'].upper()}_FLOATS" for r in ROWS}
REC0 = 33


def choice_fn(kind):
    return {"TieKind": "tie_kind", "BarKind": "bar_kind", "FrostClass": "frost_class"}[kind]


def sql_col(f):
    if f["kind"] == "f64":
        return f"{f['name']} REAL"
    vs = ",".join(f"'{v}'" for v in ENUMS[f["kind"]]["variants"])
    return f"{f['name']} TEXT NOT NULL CHECK ({f['name']} IN ({vs}))"


def pair(f):
    return f"{f['name']}_ieee754_bits INTEGER NOT NULL, {f['name']}_ieee754_class TEXT NOT NULL"


def patch_sql():
    p = f"{SQ}/🗄️.sql"
    s = rd(p)
    if "en1996_pier" in s:
        return
    wall_data = ",\n".join("  " + sql_col(x) for x in SCALARS)
    s = s.replace("  is_basement INTEGER NOT NULL CHECK (is_basement IN (0,1)),\n", "  is_basement INTEGER NOT NULL CHECK (is_basement IN (0,1)),\n" + wall_data + ",\n", 1)
    s = s.replace("  phi_infinity_ieee754_bits INTEGER NOT NULL, phi_infinity_ieee754_class TEXT NOT NULL\n);", "  phi_infinity_ieee754_bits INTEGER NOT NULL, phi_infinity_ieee754_class TEXT NOT NULL,\n" + ",\n".join("  " + pair(x) for x in SCALARS if x["kind"] == "f64") + "\n);", 1)
    s = s.replace("  width_m REAL, height_m REAL, sill_height_m REAL,\n", "  width_m REAL, height_m REAL, sill_height_m REAL, position_m REAL,\n", 1)
    s = s.replace("  sill_height_m_ieee754_bits INTEGER NOT NULL, sill_height_m_ieee754_class TEXT NOT NULL\n);", "  sill_height_m_ieee754_bits INTEGER NOT NULL, sill_height_m_ieee754_class TEXT NOT NULL,\n  position_m_ieee754_bits INTEGER NOT NULL, position_m_ieee754_class TEXT NOT NULL\n);", 1)
    s = s.replace("  force_n REAL, bearing_area_m2 REAL, bearing_length_m REAL,\n", "  force_n REAL, bearing_area_m2 REAL, bearing_length_m REAL, distance_a1_m REAL, position_m REAL,\n", 1)
    s = s.replace("  bearing_length_m_ieee754_bits INTEGER NOT NULL, bearing_length_m_ieee754_class TEXT NOT NULL\n);", "  bearing_length_m_ieee754_bits INTEGER NOT NULL, bearing_length_m_ieee754_class TEXT NOT NULL,\n  distance_a1_m_ieee754_bits INTEGER NOT NULL, distance_a1_m_ieee754_class TEXT NOT NULL,\n  position_m_ieee754_bits INTEGER NOT NULL, position_m_ieee754_class TEXT NOT NULL\n);", 1)
    for r in ROWS:
        cols = ",\n  ".join(sql_col(f) for f in r["fields"])
        pairs = ",\n  ".join(pair(f) for f in r["fields"] if f["kind"] == "f64")
        s = s.rstrip("\n") + f"\nCREATE TABLE {TABLE[r['key']]} (\n  id INTEGER PRIMARY KEY,\n  wall_id INTEGER NOT NULL REFERENCES en1996_wall(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  logical_id TEXT NOT NULL,\n  {cols},\n  {pairs}\n);\n"
    wr(p, s)


def floats(idxs):
    return "&[" + ",".join(f"FloatColumn::Binary64({i})" for i in idxs) + "]"


def row_float_idx(r):
    return [4 + i for i, f in enumerate(r["fields"]) if f["kind"] == "f64"]


def cells_rs(r, v):
    out = []
    for f in r["fields"]:
        out.append(f"Cell::Real({v}.{f['name']})" if f["kind"] == "f64" else f"Cell::Text({choice_fn(f['kind'])}({v}.{f['name']}))")
    return ",".join(out)


def build_rs():
    p = f"{SQ}/🦀️.rs"
    s = rd(p)
    if "PIER_FLOATS" in s:
        return
    s = s.replace("WallOpening, ConcentratedLoad};", "WallOpening, ConcentratedLoad, " + ", ".join(r["struct"] for r in ROWS) + ", TieKind, BarKind, FrostClass};", 1)
    s = s.replace("FloatColumn::Binary64(32)];", "FloatColumn::Binary64(32)," + ",".join(f"FloatColumn::Binary64({i})" for i in range(35, 45)) + "];", 1)
    s = s.replace("const OPENING_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6)];", "const OPENING_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7)];\n" + "\n".join(f"const {CONST[r['key']]}: &[FloatColumn] = {floats(row_float_idx(r))};" for r in ROWS), 1)
    s = s.replace("const CONCENTRATED_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6)];", "const CONCENTRATED_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8)];", 1)
    ch = ""
    for n, e in ENUMS.items():
        fn = choice_fn(n)
        ch += f"choice!({fn},read_{fn},{n},{{" + ",".join(f'{n}::{v}=>"{v}"' for v in e["variants"]) + "});\n"
    s = s.replace("\nfn checkpoint(", "\n" + ch + "\nfn checkpoint(", 1)
    scalar_cells = ",".join(f"Cell::Text({choice_fn(x['kind'])}(wall.{x['name']}))" if x["kind"] != "f64" else f"Cell::Real(wall.{x['name']})" for x in SCALARS)
    s = s.replace("flag(wall.is_basement)],WALL_FLOATS)?;", "flag(wall.is_basement)," + scalar_cells + "],WALL_FLOATS)?;", 1)
    s = s.replace("Cell::Real(opening.sill_height_m)]", "Cell::Real(opening.sill_height_m),Cell::Real(opening.position_m)]", 1)
    s = s.replace("Cell::Real(load.bearing_length_m)]", "Cell::Real(load.bearing_length_m),Cell::Real(load.distance_a1_m),Cell::Real(load.position_m)]", 1)
    loops = ""
    for r in ROWS:
        loops += f'            for(position,row)in wall.{r["coll"]}.iter().enumerate(){{out.insert_float("{TABLE[r["key"]]}",&[Cell::Integer(id),ordinal(position)?,Cell::Text(&row.id),{cells_rs(r, "row")}],{CONST[r["key"]]})?;}}\n'
    anchor = re.search(r"            for\(position,opening\)in wall\.openings\.iter\(\)\.enumerate\(\)\{[^\n]*\n", s)
    s = s[: anchor.end()] + loops + s[anchor.end():]
    s = s.replace("(En1996Snapshot::SQLITE_SCHEMA,5,70,control)", "(En1996Snapshot::SQLITE_SCHEMA,12,101,control)", 1)
    s = s.replace('entities(database,"en1996_wall",34,WALL_FLOATS,control)', 'entities(database,"en1996_wall",45,WALL_FLOATS,control)', 1)
    s = s.replace('entities(database,"en1996_opening",7,OPENING_FLOATS,control)', 'entities(database,"en1996_opening",8,OPENING_FLOATS,control)', 1)
    s = s.replace('entities(database,"en1996_concentrated_load",7,CONCENTRATED_FLOATS,control)', 'entities(database,"en1996_concentrated_load",9,CONCENTRATED_FLOATS,control)', 1)
    groups_rs = "".join(f'let mut g_{r["coll"]}=groups(entities(database,"{TABLE[r["key"]]}",{4 + len(r["fields"])},{CONST[r["key"]]},control)?,&wall_rows,control)?;' for r in ROWS)
    s = s.replace("let mut cases=groups(load_rows,&wall_rows,control)?;", "let mut cases=groups(load_rows,&wall_rows,control)?;" + groups_rs, 1)
    s = s.replace("sill_height_m:opening.real(6)?}", "sill_height_m:opening.real(6)?,position_m:opening.real(7)?}", 1)
    s = s.replace("bearing_length_m:load.real(6)?}", "bearing_length_m:load.real(6)?,distance_a1_m:load.real(7)?,position_m:load.real(8)?}", 1)
    reads = ""
    for r in ROWS:
        fl = ",".join(f"{f['name']}:" + (f"r.real({4 + i})?" if f["kind"] == "f64" else f"read_{choice_fn(f['kind'])}(r.text({4 + i})?)?") for i, f in enumerate(r["fields"]))
        reads += f"            let mut v_{r['coll']}=Vec::new();for r in ordered(g_{r['coll']}.remove(&row.rowid).unwrap_or_default().into_iter(),row.rowid,control)?{{checkpoint(control,v_{r['coll']}.len(),0)?;v_{r['coll']}.push({r['struct']}{{id:reconstruct_text(control,r.text(3)?)?,{fl}}});}}\n"
    anchor = re.search(r"            checkpoint\(control,walls\.len\(\),0\)\?;let mut wall_openings=Vec::new\(\);[^\n]*\n", s)
    s = s[: anchor.end()] + reads + s[anchor.end():]
    sc = ",".join(f"{x['name']}:" + (f"row.real({35 + [y['name'] for y in SCALARS if y['kind'] == 'f64'].index(x['name'])})?" if x["kind"] == "f64" else "read_frost_class(row.text(34)?)?") for x in SCALARS)
    s = s.replace("is_basement:read_flag(row,33)?,load_cases}", "is_basement:read_flag(row,33)?," + sc + "," + ",".join(f"{r['coll']}:v_{r['coll']}" for r in ROWS) + ",load_cases}", 1)
    s = s.replace("rows=add(add(rows,wall.openings.len())?,wall.load_cases.len())?;", "rows=add(add(rows,wall.openings.len())?,wall.load_cases.len())?;" + "".join(f"rows=add(rows,wall.{r['coll']}.len())?;" for r in ROWS), 1)
    s = s.replace("for opening in &wall.openings{bound.add(4096)?;bound.repeated(opening.id.len(),24)?;}", "for opening in &wall.openings{bound.add(4096)?;bound.repeated(opening.id.len(),24)?;}" + "".join(f"for row in &wall.{r['coll']}{{bound.add(4096)?;bound.repeated(row.id.len(),24)?;}}" for r in ROWS), 1)
    wr(p, s)


def build_admission():
    p = f"{SQ}/🛂️admission/🦀️.rs"
    s = rd(p)
    if "en1996_pier" in s:
        return
    s = s.replace("//! 🪨️ Borrowed masonry walls, openings and load cells before typed ownership.", "//! 🪨️ Borrowed masonry walls, openings, member rows and load cells before typed ownership.", 1)
    s = s.replace("rows=n::add(rows,n::list(n::field(wall,8)?)?.len())?;", "rows=n::add(rows,n::list(n::field(wall,8)?)?.len())?;" + "".join(f"rows=n::add(rows,n::list(n::field(wall,{REC0 + i})?)?.len())?;" for i, r in enumerate(ROWS)), 1)
    scal = ",".join(f"Enumeration({40 + i},&[" + ",".join(f'"{v}"' for v in ENUMS[x["kind"]]["variants"]) + "])" if x["kind"] != "f64" else f"Real({40 + i})" for i, x in enumerate(SCALARS))
    s = s.replace("Boolean(31)])?,WALL_FLOATS)?;", "Boolean(31)," + scal + "])?,WALL_FLOATS)?;", 1)
    s = s.replace("[Text(0),Real(1),Real(2),Real(3)])?,OPENING_FLOATS)", "[Text(0),Real(1),Real(2),Real(3),Real(4)])?,OPENING_FLOATS)", 1)
    s = s.replace("[Text(0),Real(1),Real(2),Real(3)])?,CONCENTRATED_FLOATS)", "[Text(0),Real(1),Real(2),Real(3),Real(4),Real(5)])?,CONCENTRATED_FLOATS)", 1)
    loops = ""
    for i, r in enumerate(ROWS):
        cols = ["Text(0)"]
        for j, f in enumerate(r["fields"], 1):
            cols.append(f"Real({j})" if f["kind"] == "f64" else f"Enumeration({j},&[" + ",".join(f'"{v}"' for v in ENUMS[f["kind"]]["variants"]) + "])")
        loops += f' for(index,row)in n::list(n::field(wall,{REC0 + i})?)?.iter().enumerate(){{n::entity(out,"{TABLE[r["key"]]}",parent,index,&n::cells(n::record(row)?,[{",".join(cols)}])?,{CONST[r["key"]]})?;}}\n'
    anchor = re.search(r' for\(index,opening\)in n::list\(n::field\(wall,8\)\?\)\?\.iter\(\)\.enumerate\(\)\{[^\n]*\n', s)
    s = s[: anchor.end()] + loops + s[anchor.end():]
    s = s.replace("/// 🛂️ Five authored SQL tables", "/// 🛂️ Twelve authored SQL tables", 1)
    s = s.replace("En1996Snapshot::SQLITE_SCHEMA,5,70,count(root)?,write)", "En1996Snapshot::SQLITE_SCHEMA,12,101,count(root)?,write)", 1)
    wr(p, s)


patch_sql()
build_rs()
build_admission()
print("sqlite done")
