import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from padd import *

A = Art("🫨️en1998", "En1998", "towers", "towers", "Tower", model="En1998Tower", new_model="En1998Placement")


def schema_layers():
    s = "/🧬️schema"
    layer_model_struct(A, A.path(s + "/📸️snapshot/🦀️.rs"))
    edit(A.path(s + "/📸️snapshot/🦀️.rs"), lambda t: clone_line(t, r"^\s+(pub )?towers: (Vec<En1998Tower>,|vec!\[\],|Vec::new\(\),)", A))
    edit(A.path(s + "/🦀️.rs"), lambda t: clone_line(t, r"^\s+(pub )?towers: ", A))
    for f in ("/🟦️.ts", "/📸️snapshot/🟦️.ts"):
        ts_iface(A, A.path(s + f))
        ts_root_field(A, A.path(s + f))
    graphql_snapshot(A, A.path(s + "/📸️snapshot/🔗️.graphql"))
    proto_snapshot(A, A.path(s + "/📸️snapshot/🛰️.proto"))
    for f in ("/🔣️.json", "/📸️snapshot/🔣️.json"):
        json_snapshot(A, A.path(s + f))


def sqlite_layers():
    q = "/🚪️io/🪶️sqlite/📸️snapshot"
    TB = "en1998_placement"
    cols = ["id INTEGER PRIMARY KEY", "document_id INTEGER NOT NULL REFERENCES en1998_document(id)", "ordinal INTEGER NOT NULL CHECK(ordinal>=0)", "logical_id TEXT NOT NULL", "x REAL", "y REAL", "z REAL", "yaw REAL"] + sum([["%s_ieee754_bits INTEGER NOT NULL" % f, "%s_ieee754_class TEXT NOT NULL" % f] for f in FIELDS], [])
    ddl = "CREATE TABLE %s (%s);" % (TB, ",".join(cols))
    edit(A.path(q + "/🗄️.sql"), lambda t: t if TB in t else t.rstrip("\n") + "\n" + ddl + "\n")

    def rs(t):
        if "PLACEMENT" in t:
            return t
        t = sub1(t, r"En1998RetainingWall, En1998Tower\};", "En1998RetainingWall, En1998Tower, En1998Placement};")
        t = sub1(t, r"(const TOWER:[^\n]*\n)", r"\1const PLACEMENT:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7)];\n")
        t = sub1(t, r"(variables\(out,\"en1998_tower_variable\",parent,&a\.variables\)\?;\}\n)", lambda m: m.group(1) + '  for(position,a)in self.placements.iter().enumerate(){out.insert_float("en1998_placement",&[Cell::Integer(1),ordinal(position)?,Cell::Text(&a.id),Cell::Real(a.x),Cell::Real(a.y),Cell::Real(a.z),Cell::Real(a.yaw)],PLACEMENT)?;}\n')
        t = sub1(t, r"self\.towers\.len\(\)\]\{rows=add", "self.towers.len(),self.placements.len()]{rows=add")
        t = sub1(t, r"bound_variables\(&mut bound,&a\.variables\)\?;\}bound\.finish\(\)", "bound_variables(&mut bound,&a.variables)?;}for a in &self.placements{bound.add(4096)?;bound.repeated(a.id.len(),24)?;}bound.finish()")
        t = sub1(t, r"(let tv_rows=entities\(database,\"en1998_tower_variable\",6,VARIABLE,control\)\?;)", r'\1let placement_rows=entities(database,"en1998_placement",8,PLACEMENT,control)?;')
        t = sub1(t, r"(let tv=groups\(&tv_rows,&tower_rows,control\)\?;)", r"\1let placements=groups(&placement_rows,&documents,control)?;")
        t = sub1(t, r"towers:Vec::new\(\)\};", "towers:Vec::new(),placements:Vec::new()};")
        t = sub1(t, r"(variables:read_variables\(siblings\(&tv,a\.rowid\),control\)\?\}\);\}\n)", lambda m: m.group(1) + '  for a in siblings(&placements,1){checkpoint(control,snapshot.placements.len(),placement_rows.len())?;snapshot.placements.push(En1998Placement{id:reconstruct_text(control,a.text(3)?)?,x:a.real(4)?,y:a.real(5)?,z:a.real(6)?,yaw:a.real(7)?});}\n')
        return t
    edit(A.path(q + "/🦀️.rs"), rs)

    def ts(t):
        if "PLACEMENT" in t:
            return t
        t = sub1(t, r"En1998RetainingWall,En1998Tower\} from", "En1998RetainingWall,En1998Tower,En1998Placement} from")
        t = sub1(t, r'\|"towers">&\{', '|"towers"|"placements">&{')
        t = sub1(t, r"towers:En1998SqliteTower\[\]\};", "towers:En1998SqliteTower[];placements:Exact<En1998Placement>[]};")
        t = sub1(t, r"(const TOWER:[^\n]*\n)", r"\1const PLACEMENT:readonly Ieee754Column[]=[4,5,6,7].map(index=>({index,width:64}));\n")
        t = sub1(t, r"await variables\(out,\"en1998_tower_variable\",parent,a\.variables,options\)\}return out\.finish\(\);", 'await variables(out,"en1998_tower_variable",parent,a.variables,options)}\n for(let index=0;index<snapshot.placements.length;index++){const a=snapshot.placements[index]!;await write(out,"en1998_placement",[1n,BigInt(index),a.id,a.x,a.y,a.z,a.yaw],PLACEMENT,options)}return out.finish();')
        t = sub1(t, r"(tvRows=await entities\(tables\[15\]!,6,VARIABLE,options\);)", r"\1const placementRows=await entities(tables[16]!,8,PLACEMENT,options);")
        t = sub1(t, r"(,tv=await groups\(tvRows,towerRows,options\);)", r"\1const placementGroups=await groups(placementRows,documents,options);")
        t = sub1(t, r"towers:\[\]\};", "towers:[],placements:[]};")
        t = sub1(t, r"(variables:await readVariables\(tv\.get\(a\.rowid\)\?\?\[\],options\)\}\)\}\n)", lambda m: m.group(1) + ' for(const a of placementGroups.get(1n)??[]){await checkpoint(options,snapshot.placements.length,placementRows.size);snapshot.placements.push({id:artifactSqliteText(a,3),x:readBinary64(a,4,PLACEMENT),y:readBinary64(a,5,PLACEMENT),z:readBinary64(a,6,PLACEMENT),yaw:readBinary64(a,7,PLACEMENT)})}\n')
        return t
    edit(A.path(q + "/🟦️.ts"), ts)

    def rt(t):
        if "placements" in t:
            return t
        t = sub1(t, r"(\n for x in &mut s\.towers\{[^\n]*\}\n)", lambda m: m.group(1) + " for x in &mut s.placements{x.x=v;x.y=v;x.z=v;x.yaw=v;}\n")
        t = sub1(t, r"empty\.towers\.clear\(\);assert_eq!\(database\(&empty\)\.tables\.len\(\),16\)", "empty.towers.clear();empty.placements.clear();assert_eq!(database(&empty).tables.len(),17)")
        return t
    edit(A.path(q + "/🧪️tests/🦀️.rs"), rt)

    def tt(t):
        if "placements" in t:
            return t
        t = sub1(t, r"towers:\[\]\},database=", "towers:[],placements:[]},database=")
        t = sub1(t, r"database\.tables\.length\)\.toBe\(16\)", "database.tables.length).toBe(17)")
        t = sub1(t, r'(variables:x\.variables\.map\(a=>floats\(a,\["qkN"\]\)\)\}\)\))\}\}', r'\1,placements:fixture.placements.map(x=>floats(x,["x","y","z","yaw"]))}}')
        return t
    edit(A.path(q + "/🧪️tests/🟦️.ts"), tt)


def rest_layers():
    rows = [
        '    ("placements", m!("Placements", "Platzierungen")),',
        '    ("placements[].id", m!("Placed entity id", "Platzierte Element-Id")),',
        '    ("placements[].x", m!("Placement x", "Platzierung x", UnitId::Metre)),',
        '    ("placements[].y", m!("Placement y", "Platzierung y", UnitId::Metre)),',
        '    ("placements[].z", m!("Placement z", "Platzierung z", UnitId::Metre)),',
        '    ("placements[].yaw", m!("Placement yaw about Z", "Platzierung Gierwinkel um Z", UnitId::Radian)),',
    ]
    edit(A.path("/🏷️field-meta/🦀️.rs"), lambda t: append_after_last(t, r'^\s+\("towers\[\]\.variables\[\]\.qkN"', rows))
    i = "/🧬️schema/💡️inferences"

    def outline(t):
        if "placements" in t:
            return t
        t = sub1(t, r'(\n    "towers",\n)', r'\1    "placements",\n')
        return sub1(t, r"\+ snapshot\.towers\.len\(\)\) as u32", "+ snapshot.towers.len()\n            + snapshot.placements.len()) as u32")
    edit(A.path(i + "/🧾outline/🦀️.rs"), outline)
    edit(A.path(i + "/⚖️part-evaluators/🦀️.rs"), lambda t: t if '"placements"' in t else clone_line(t, r'^\s+push_duplicate_ids\(report, annex, "towers", ', A, attrs=False, rn=lambda s: s.replace("towers", "placements").replace("t.id", "t.id").replace("document.placements", "document.placements")))


if __name__ == "__main__":
    schema_layers()
    diff_layers(A)
    sqlite_layers()
    rest_layers()
    fixtures(A)
