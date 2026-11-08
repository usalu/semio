#!/usr/bin/env python3
"""EN 1998: adds the SQLite tables en1998_isolation_system, en1998_pier_base and en1998_critical_region to the hand-written snapshot codecs (sql, Rust, TS, TS test) behind the slope table, mirroring it field by field (idempotent). Usage: python3 -I en1998-sqlite-new-collections.py"""
import re

D = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/"
CRF = "section_width_m section_depth_m clear_length_m hooped_length_m core_width_m core_depth_m axial_force_n concrete_design_strength_mpa steel_design_yield_mpa hoop_design_yield_mpa longitudinal_steel_area_m2 longitudinal_bar_diameter_m longitudinal_bar_max_diameter_m hoop_diameter_m hoop_spacing_m hoop_legs_x hoop_legs_y restrained_bar_count restrained_bar_spacing_m".split()
TABLES = [
    dict(name="en1998_isolation_system", const="ISOLATION_SYSTEM", ty="En1998IsolationSystem", field="isolation_systems", var="isolation_system", ts="isolationSystems", texts=["bridge_id", "building_id"], floats=["strength_n", "stiffness_n_per_m", "yield_displacement_m", "capacity_m"]),
    dict(name="en1998_pier_base", const="PIER_BASE", ty="En1998PierBase", field="pier_bases", var="pier_base", ts="pierBases", texts=["bridge_id", "pier_id"], floats=["radius_m", "soil_vs_m_s", "soil_unit_weight_n_m3", "poisson_ratio"]),
    dict(name="en1998_critical_region", const="CRITICAL_REGION", ty="En1998CriticalRegion", field="critical_regions", var="critical_region", ts="criticalRegions", texts=["building_id", "member_id"], floats=CRF),
]
camel = lambda s: (lambda a: a[0] + "".join(x[:1].upper() + x[1:] for x in a[1:]))(s.split("_"))
for t in TABLES:
    t["cols"] = 4 + len(t["texts"]) + len(t["floats"])
    t["fi"] = list(range(4 + len(t["texts"]), t["cols"]))


def rd(n): return open(D + n, encoding="utf-8").read()
def wr(n, s): open(D + n, "w", encoding="utf-8").write(s)
def once(s, old, new):
    assert s.count(old) == 1, (old[:80], s.count(old))
    return s.replace(old, new)


sql = rd("🗄️.sql")
if "en1998_isolation_system" not in sql:
    lines = []
    for t in TABLES:
        cols = ["id INTEGER PRIMARY KEY", "document_id INTEGER NOT NULL REFERENCES en1998_document(id)", "ordinal INTEGER NOT NULL CHECK(ordinal>=0)", "logical_id TEXT NOT NULL"] + [f"{x} TEXT NOT NULL" for x in t["texts"]] + [f"{x} REAL" for x in t["floats"]]
        for x in t["floats"]:
            cols += [f"{x}_ieee754_bits INTEGER NOT NULL", f"{x}_ieee754_class TEXT NOT NULL"]
        lines.append(f"CREATE TABLE {t['name']} ({','.join(cols)});")
    wr("🗄️.sql", sql.rstrip("\n") + "\n" + "\n".join(lines) + ("\n" if sql.endswith("\n") else ""))

rs = rd("🦀️.rs")
if "en1998_isolation_system" not in rs:
    rs = once(rs, "En1998Slope, ", "En1998Slope, " + ", ".join(t["ty"] for t in TABLES) + ", ")
    consts = "".join(f"const {t['const']}:&[FloatColumn]=&[{','.join(f'FloatColumn::Binary64({i})' for i in t['fi'])}];\n" for t in TABLES)
    m = re.search(r"const SLOPE:&\[FloatColumn\]=[^\n]*\n", rs)
    rs = rs[:m.end()] + consts + rs[m.end():]
    enc = ""
    for t in TABLES:
        cells = ",".join(["Cell::Integer(1)", "ordinal(position)?", "Cell::Text(&a.id)"] + [f"Cell::Text(&a.{x})" for x in t["texts"]] + [f"Cell::Real(a.{x})" for x in t["floats"]])
        enc += f"  for(position,a)in self.{t['field']}.iter().enumerate(){{out.insert_float(\"{t['name']}\",&[{cells}],{t['const']})?;}}\n"
    m = re.search(r"  for\(position,a\)in self\.slopes\.iter\(\)\.enumerate\(\)\{[^\n]*\n", rs)
    rs = rs[:m.end()] + enc + rs[m.end():]
    rs = once(rs, "self.slopes.len(),", "self.slopes.len()," + ",".join(f"self.{t['field']}.len()" for t in TABLES) + ",")
    rs = once(rs, 'let slope_rows=entities(database,"en1998_slope",13,SLOPE,control)?;', 'let slope_rows=entities(database,"en1998_slope",13,SLOPE,control)?;' + "".join(f'let {t["var"]}_rows=entities(database,"{t["name"]}",{t["cols"]},{t["const"]},control)?;' for t in TABLES))
    rs = once(rs, "let slopes=groups(&slope_rows,&documents,control)?;", "let slopes=groups(&slope_rows,&documents,control)?;" + "".join(f"let {t['field']}=groups(&{t['var']}_rows,&documents,control)?;" for t in TABLES))
    rs = once(rs, "slopes:Vec::new(),", "slopes:Vec::new()," + "".join(f"{t['field']}:Vec::new()," for t in TABLES))
    dec = ""
    for t in TABLES:
        fs = [f"id:reconstruct_text(control,a.text(3)?)?"] + [f"{x}:reconstruct_text(control,a.text({4 + i})?)?" for i, x in enumerate(t["texts"])] + [f"{x}:a.real({i})?" for x, i in zip(t["floats"], t["fi"])]
        dec += f"  for a in siblings(&{t['field']},1){{checkpoint(control,snapshot.{t['field']}.len(),{t['var']}_rows.len())?;snapshot.{t['field']}.push({t['ty']}{{{','.join(fs)}}});}}\n"
    m = re.search(r"  for a in siblings\(&slopes,1\)\{[^\n]*\n", rs)
    rs = rs[:m.end()] + dec + rs[m.end():]
    wr("🦀️.rs", rs)

ts = rd("🟦️.ts")
if "en1998_isolation_system" not in ts:
    ts = once(ts, "En1998Slope,", "En1998Slope," + ",".join(t["ty"] for t in TABLES) + ",")
    ts = once(ts, '|"slopes"|', '|"slopes"|' + "|".join(f'"{t["ts"]}"' for t in TABLES) + "|")
    ts = once(ts, "slopes:Exact<En1998Slope>[];", "slopes:Exact<En1998Slope>[];" + "".join(f"{t['ts']}:Exact<{t['ty']}>[];" for t in TABLES))
    consts = "".join(f"const {t['const']}:readonly Ieee754Column[]=[{','.join(map(str, t['fi']))}].map(index=>({{index,width:64}}));\n" for t in TABLES)
    m = re.search(r"const SLOPE:readonly Ieee754Column\[\]=[^\n]*\n", ts)
    ts = ts[:m.end()] + consts + ts[m.end():]
    enc = "".join(f' for(let index=0;index<snapshot.{t["ts"]}.length;index++){{const a=snapshot.{t["ts"]}[index]!;await write(out,"{t["name"]}",[1n,BigInt(index),a.id,{",".join("a." + camel(x) for x in t["texts"] + t["floats"])}],{t["const"]},options)}}' for t in TABLES)
    m = re.search(r'for\(let index=0;index<snapshot\.slopes\.length;index\+\+\)\{const a=snapshot\.slopes\[index\]!;await write\(out,"en1998_slope",[^\n]*?SLOPE,options\)\}', ts)
    ts = ts[:m.end()] + enc + ts[m.end():]
    ts = once(ts, "slopeRows=await entities(tables[24]!,13,SLOPE,options);", "slopeRows=await entities(tables[24]!,13,SLOPE,options);" + "".join(f"const {camel(t['var'])}Rows=await entities(tables[{25 + i}]!,{t['cols']},{t['const']},options);" for i, t in enumerate(TABLES)))
    ts = once(ts, "slopeGroups=await groups(slopeRows,documents,options);", "slopeGroups=await groups(slopeRows,documents,options);" + "".join(f"const {camel(t['var'])}Groups=await groups({camel(t['var'])}Rows,documents,options);" for t in TABLES))
    ts = once(ts, "slopes:[],", "slopes:[]," + "".join(f"{t['ts']}:[]," for t in TABLES))
    dec = ""
    for t in TABLES:
        fs = ["id:artifactSqliteText(a,3)"] + [f"{camel(x)}:artifactSqliteText(a,{4 + i})" for i, x in enumerate(t["texts"])] + [f"{camel(x)}:readBinary64(a,{i},{t['const']})" for x, i in zip(t["floats"], t["fi"])]
        v = camel(t["var"])
        dec += f" for(const a of {v}Groups.get(1n)??[]){{await checkpoint(options,snapshot.{t['ts']}.length,{v}Rows.size);snapshot.{t['ts']}.push({{{','.join(fs)}}})}}"
    m = re.search(r" for\(const a of slopeGroups\.get\(1n\)\?\?\[\]\)\{[^\n]*?circleRadius:readBinary64\(a,12,SLOPE\)\}\)\}", ts)
    ts = ts[:m.end()] + dec + ts[m.end():]
    wr("🟦️.ts", ts)

tt = rd("🧪️tests/🟦️.ts")
if "isolationSystems" not in tt:
    wr("🧪️tests/🟦️.ts", once(tt, "slopes:[],", "slopes:[],isolationSystems:[],pierBases:[],criticalRegions:[],") if tt.count("slopes:[],") == 1 else tt)
print("done")
