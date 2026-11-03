import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from padd import *

A = Art("🔩️en1993", "En1993", "frame_bars", "frameBars", "FrameBar")
Q = "/🚪️io/🪶️sqlite/📸️snapshot"
TB = "en1993_placement"


def other_layers():
    i = "/🧬️schema/💡️inferences"
    edit(A.path(i + "/🦀️.rs"), lambda t: t if '"frameBars", "placements"' in t else sub1(t, r'"frameNodes", "frameBars"\]', '"frameNodes", "frameBars", "placements"]'))
    edit(A.path(i + "/🧾outline/🦀️.rs"), lambda t: t if "snapshot.placements.len()" in t else sub1(clone_line(t, r'^\s+"frameBars",$', A, attrs=False), r"(\n\s+\+ snapshot\.frame_bars\.len\(\))", lambda m: m.group(1) + m.group(1).replace("frame_bars", "placements")))
    edit(A.path(i + "/⚖️checks/🧰️structure-helpers/🦀️.rs"), lambda t: clone_line(t, r'^\s+push_duplicate_ids\(report, annex, "frameBars"', A, attrs=False))
    rows = [
        '    ("placements", FieldSpec::new("Placements", "Platzierungen")),',
        '    ("placements[].id", FieldSpec::new("Placed entity id", "Platzierte Element-Id")),',
        '    ("placements[].x", FieldSpec::quantity("Placement x", "Platzierung x", UnitId::Metre)),',
        '    ("placements[].y", FieldSpec::quantity("Placement y", "Platzierung y", UnitId::Metre)),',
        '    ("placements[].z", FieldSpec::quantity("Placement z", "Platzierung z", UnitId::Metre)),',
        '    ("placements[].yaw", FieldSpec::quantity("Placement yaw about Z", "Platzierung Gierwinkel um Z", UnitId::Radian)),',
    ]
    edit(A.path("/✏️editor/🏷️field-meta/🦀️.rs"), lambda t: append_after_last(t, r'^\s+\("frameBars\[\]\.\w+", ', rows))
    b = "/🚪️io/💾️binary/📸️snapshot"
    n = "/🚪️io/📝️text/📸️snapshot"
    for p in (b + "/🦀️.rs", n + "/🦀️.rs"):
        edit(A.path(p), lambda t: t if "FrameBar, Placement" in t else t.replace("FrameBar,", "FrameBar, Placement,"))
    def bts(t):
        if "interface Placement" in t:
            return t
        t = sub1(t, r"(export interface FrameBar \{[^\n]*\n)", r"\1export interface Placement { id: string; x: number; y: number; z: number; yaw: number; }\n")
        return sub1(t, r"frameBars: FrameBar\[\]; \}", "frameBars: FrameBar[]; placements: Placement[]; }")
    edit(A.path(b + "/🟦️.ts"), bts)
    def tgq(t):
        if "type Placement" in t:
            return t
        t = sub1(t, r"(type FrameBar \{[^\n]*\n)", r"\1type Placement { id: String! x: Float! y: Float! z: Float! yaw: Float! }\n")
        return sub1(t, r"frameBars: \[FrameBar!\]! \}", "frameBars: [FrameBar!]! placements: [Placement!]! }")
    edit(A.path(n + "/🔗️.graphql"), tgq)


def sqlite_layers():
    cols = "id INTEGER PRIMARY KEY,document_id INTEGER NOT NULL REFERENCES en1993_document(id),ordinal INTEGER NOT NULL CHECK(ordinal>=0),logical_id TEXT NOT NULL,x REAL,y REAL,z REAL,yaw REAL,\n " + ",".join("%s_ieee754_bits INTEGER NOT NULL,%s_ieee754_class TEXT NOT NULL" % (f, f) for f in FIELDS)
    sqltxt = "CREATE TABLE en1993_placement(\n %s\n);\n" % cols
    edit(A.path(Q + "/🗄️.sql"), lambda t: t if TB in t else t.rstrip("\n") + "\n" + sqltxt)

    def rs(t):
        if "PLACEMENT" in t:
            return t
        t = sub1(t, r"FrameBar, ForceAction\};", "FrameBar, Placement, ForceAction};")
        t = sub1(t, r"(const FRAME_BAR:&\[FloatColumn\]=&\[\];\n)", r"\1const PLACEMENT:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7)];\n")
        t = sub1(t, r'(  for\(n,v\)in self\.frame_bars\.iter\(\)\.enumerate\(\)\{[^\n]*?\)\?;\})Ok\(\(\)\)', lambda m: m.group(1) + '\n  for(n,v)in self.placements.iter().enumerate(){p.insert_float("en1993_placement",&[Cell::Integer(1),ordinal(n)?,Cell::Text(&v.id),Cell::Real(v.x),Cell::Real(v.y),Cell::Real(v.z),Cell::Real(v.yaw)],PLACEMENT)?;}Ok(())')
        t = sub1(t, r"self\.frame_bars\.len\(\)\]\{", "self.frame_bars.len(),self.placements.len()]{", count=2)
        t = sub1(t, r"(for v in &self\.frame_bars\{admit_strings[^\n]*?\)\?;\})b\.finish\(\)", lambda m: m.group(1) + "for v in &self.placements{admit_strings(&mut b,&[&v.id],32768)?;}b.finish()")
        t = sub1(t, r'(let frame_bar_rows=entities\(d,"en1993_frame_bar",10,FRAME_BAR,c\)\?;)', r'\1let placement_rows=entities(d,"en1993_placement",8,PLACEMENT,c)?;')
        t = sub1(t, r"(\n  c\.checkpoint\(SqliteSnapshotPhase::ReconstructSnapshot,1,1\)\?;Ok\(Self\{)", lambda m: '\n  let mut placements=Vec::new();for v in ordered(placement_rows.into_values(),1,c)?{placements.push(Placement{id:text(v,3,c)?,x:v.real(4)?,y:v.real(5)?,z:v.real(6)?,yaw:v.real(7)?});}' + m.group(1))
        t = sub1(t, r"frame_nodes,frame_bars\}\)", "frame_nodes,frame_bars,placements})")
        return t.replace("SQLITE_SCHEMA,34,162", "SQLITE_SCHEMA,35,162")
    edit(A.path(Q + "/🦀️.rs"), rs)

    def ts(t):
        if "PLACEMENT" in t:
            return t
        t = sub1(t, r"import type\{LoadCase\}from", "import type{LoadCase,Placement}from")
        t = sub1(t, r"(export type En1993SqliteFrameBar=[^\n]*\n)", lambda m: m.group(1) + 'export type En1993SqlitePlacement=Omit<Placement,"x"|"y"|"z"|"yaw">&{x:Binary64;y:Binary64;z:Binary64;yaw:Binary64};\n')
        t = sub1(t, r"frameBars:En1993SqliteFrameBar\[\]\};", "frameBars:En1993SqliteFrameBar[];placements:En1993SqlitePlacement[]};")
        t = sub1(t, r"(const FRAME_BAR:[^\n]*\n)", r"\1const PLACEMENT:readonly Ieee754Column[]=[4,5,6,7].map(index=>({index,width:64}));\n")
        t = sub1(t, r'"frameNodes","frameBars"\]\);', '"frameNodes","frameBars","placements"]);')
        t = sub1(t, r"s\.frameNodes,s\.frameBars\]\)collection", "s.frameNodes,s.frameBars,s.placements])collection")
        t = sub1(t, r"(\n for\(let n=0;n<s\.frameBars\.length;n\+\+\)\{[^\n]*\n)", lambda m: m.group(1) + ' for(let n=0;n<s.placements.length;n++){const v=s.placements[n]!;fields(v,["id","x","y","z","yaw"]);await write(out,"en1993_placement",[1n,BigInt(n),v.id,v.x,v.y,v.z,v.yaw],PLACEMENT,o)}\n')
        t = sub1(t, r"(const frame_bar=await entities\(t\[33\]!,10,FRAME_BAR,o\);)", r"\1const placement=await entities(t[34]!,8,PLACEMENT,o);")
        t = sub1(t, r"frameNodes:\[\],frameBars:\[\]\};", "frameNodes:[],frameBars:[],placements:[]};")
        t = sub1(t, r"(\n for\(const v of await ordered\(frame_bar\.values\(\),1n,o\)\)result\.frameBars\.push\([^\n]*\n)", lambda m: m.group(1) + ' for(const v of await ordered(placement.values(),1n,o))result.placements.push({id:artifactSqliteText(v,3),x:readBinary64(v,4,PLACEMENT),y:readBinary64(v,5,PLACEMENT),z:readBinary64(v,6,PLACEMENT),yaw:readBinary64(v,7,PLACEMENT)});\n')
        return t
    edit(A.path(Q + "/🟦️.ts"), ts)

    def adm(t):
        if "en1993_placement" in t:
            return t
        t = re.sub(r'(n::entity\(out,"en1993_frame_bar",[^\n]*?FRAME_BAR\)\?;\})Ok\(\(\)\)', lambda m: m.group(1) + '\n for(index,v)in n::list(n::field(root,26)?)?.iter().enumerate(){n::entity(out,"en1993_placement",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Real(3),Real(4)])?,PLACEMENT)?;}Ok(())', t, count=1)
        t = sub1(t, r"for id in 1\.\.=25\{", "for id in 1..=26{")
        t = sub1(t, r"SQLITE_SCHEMA,34,162", "SQLITE_SCHEMA,35,162")
        return t.replace("Thirty-four authored", "Thirty-five authored")
    edit(A.path(Q + "/🛂️admission/🦀️.rs"), adm)

    def rt(t):
        t = t.replace("source.frame_bars.clear();}", "source.frame_bars.clear();source.placements.clear();}")
        return t.replace("tables.len(),34", "tables.len(),35").replace("max_tables:34", "max_tables:35").replace("max_tables:33", "max_tables:34").replace('("tables",34)', '("tables",35)')
    edit(A.path(Q + "/🧪️tests/🦀️.rs"), rt)

    def tt(t):
        t = t.replace("frameBars:[]};const d=await project", "frameBars:[],placements:[]};const d=await project").replace("source.frameBars=[];}", "source.frameBars=[];source.placements=[];}")
        return t.replace("maxTables:34", "maxTables:35").replace("maxTables:33", "maxTables:34")
    edit(A.path(Q + "/🧪️tests/🟦️.ts"), tt)
    sqlite_extent(A, A.path(Q + "/🗄️.sql"), A.path(Q + "/🧪️tests/🧫️fixtures/🎛️semantic.json"), A.path(Q + "/🧪️tests/🟦️.ts"), None, 34, TB)


def table_names():
    p = A.path(Q + "/🧪️tests/🧫️fixtures/🔣️.json")
    names = re.findall(r"CREATE TABLE (\w+)", rd(A.path(Q + "/🗄️.sql")))
    def go(t):
        d = json.loads(t)
        d["tableNames"] = names
        return json.dumps(d, indent=2, ensure_ascii=False) + "\n"
    edit(p, go)


if __name__ == "__main__":
    core(A)
    multiline_literal_sweep(A)
    other_layers()
    sqlite_layers()
    table_names()
