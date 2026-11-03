import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from padd import *

A = Art("🧩️en1994", "En1994", "slabs", "slabs", "CompositeSlab")


def schema_layers():
    s = "/🧬️schema"
    layer_model_struct(A, A.path(s + "/📸️snapshot/🦀️.rs"))
    edit(A.path(s + "/📸️snapshot/🦀️.rs"), lambda t: clone_line(t, r"^\s+(pub )?slabs: (Vec<CompositeSlab>,|vec!\[\],|Vec::new\(\),)", A))
    edit(A.path(s + "/🦀️.rs"), lambda t: clone_line(t, r"^\s+(pub )?slabs: ", A))
    for f in ("/🟦️.ts", "/📸️snapshot/🟦️.ts"):
        ts_iface(A, A.path(s + f))
        ts_root_field(A, A.path(s + f))
    for f in ("/🔗️.graphql", "/📸️snapshot/🔗️.graphql"):
        graphql_snapshot(A, A.path(s + f))
    proto_snapshot(A, A.path(s + "/📸️snapshot/🛰️.proto"))
    for f in ("/🔣️.json", "/📸️snapshot/🔣️.json"):
        json_snapshot(A, A.path(s + f))



DIFF = "/🧬️schema/🔺️diff"
SNAP = "https://json.schemas.assets.semio-tech.com/s/norm/en1994/snapshot.json#/$defs/Placement"


def diff_rust():
    def lane(t):
        if "placement_rows" in t:
            return t
        t = sub1(t, r'(pub use slab_rows::\*;\n)', r'\1\n#[path = "📍️placement-rows/🦀️.rs"]\nmod placement_rows;\npub use placement_rows::*;\n')
        t = sub1(t, r"(    pub slabs: Option<En1994SlabsRows>,\n)", r"\1    #[state(artifact)]\n    pub placements: Option<En1994PlacementsRows>,\n")
        return sub1(t, r'include_str!\("🧱️slab-rows/🦀️.rs"\), ', 'include_str!("🧱️slab-rows/🦀️.rs"), include_str!("📍️placement-rows/🦀️.rs"), ')
    edit(A.path(DIFF + "/🦀️.rs"), lane)

    def apply(t):
        if "placements" in t:
            return t
        t = sub1(t, r"(        if let Some\(theirs\) = other\.slabs \{\n.*?\n        \}\n)", lambda m: m.group(1) + m.group(1).replace("slabs", "placements"), flags=re.S)
        t = sub1(t, r"(        if let Some\(rows\) = &self\.slabs \{\n[^\n]*\n        \}\n)", lambda m: m.group(1) + m.group(1).replace("slabs", "placements"))
        t = sub1(t, r"(\.slabs\.as_ref\(\)\.map_or\(true, \|rows\| rows\.is_empty\(\)\))", r"\1 && self.placements.as_ref().map_or(true, |rows| rows.is_empty())")
        t = sub1(t, r"(            slabs: self\.slabs[^\n]*\n)", lambda m: m.group(1) + m.group(1).replace("slabs", "placements"))
        return t
    edit(A.path(DIFF + "/🔁️apply-and-inverse/🦀️.rs"), apply)


def diff_graphql():
    def go(t):
        if "En1994PlacementsRows" in t:
            return t
        blocks = "type En1994PlacementsPatch {\n  index: Int!\n  x: Float\n  y: Float\n  z: Float\n  yaw: Float\n}\n\ntype En1994PlacementsInserted {\n  index: Int!\n  row: Placement!\n}\n\ntype En1994PlacementsRows {\n  removed: [Int!]!\n  inserted: [En1994PlacementsInserted!]!\n  modified: [En1994PlacementsPatch!]!\n}\n\n"
        t = sub1(t, r"(type En1994SlabsRows \{\n.*?\n\}\n\n)", lambda m: m.group(1) + blocks, flags=re.S)
        return sub1(t, r"(  slabs: En1994SlabsRows @state\(class: ARTIFACT\)\n)", r"\1  placements: En1994PlacementsRows @state(class: ARTIFACT)\n")
    edit(A.path(DIFF + "/🔗️.graphql"), go)


def diff_proto():
    def go(t):
        if "En1994PlacementsRows" in t:
            return t
        blocks = "message En1994PlacementsPatch {\n  uint64 index = 1;\n  optional double x = 2;\n  optional double y = 3;\n  optional double z = 4;\n  optional double yaw = 5;\n}\n\nmessage En1994PlacementsInserted {\n  uint64 index = 1;\n}\n\nmessage En1994PlacementsRows {\n  repeated uint64 removed = 1;\n  repeated En1994PlacementsInserted inserted = 2;\n  repeated En1994PlacementsPatch modified = 3;\n}\n\n"
        t = sub1(t, r"(message En1994SlabsRows \{\n.*?\n\}\n\n)", lambda m: m.group(1) + blocks, flags=re.S)
        m = re.search(r"message En1994Diff \{\n(.*?)\n\}", t, re.S)
        n = max(int(x) for x in re.findall(r"= (\d+);", m.group(1))) + 1
        return sub1(t, r"(  optional En1994SlabsRows slabs = \d+;\n)", r"\1  optional En1994PlacementsRows placements = %d;\n" % n)
    edit(A.path(DIFF + "/🛰️.proto"), go)


def diff_ts():
    def go(t):
        if "En1994PlacementsRows" in t:
            return t
        t = sub1(t, r"type CompositeSlab, ", "type CompositeSlab, type Placement, ")
        t = sub1(t, r"parseCompositeSlab, ", "parseCompositeSlab, parsePlacement, ")
        ifaces = ("export interface En1994PlacementsPatch {\n  index: number;\n  x: number | null;\n  y: number | null;\n  z: number | null;\n  yaw: number | null;\n}\n\n"
                  "export interface En1994PlacementsInserted {\n  index: number;\n  row: Placement;\n}\n\n"
                  "export interface En1994PlacementsRows {\n  removed: number[];\n  inserted: En1994PlacementsInserted[];\n  modified: En1994PlacementsPatch[];\n}\n\n")
        t = sub1(t, r"(export interface En1994SlabsRows \{\n.*?\n\}\n\n)", lambda m: m.group(1) + ifaces, flags=re.S)
        t = sub1(t, r"(  slabs: En1994SlabsRows \| null;\n)", r"\1  /** 📍️ @state artifact */\n  placements: En1994PlacementsRows | null;\n")
        idx = 'index: normWireRequired(normWireRange(normWireInteger, {"minimum":0}))'
        nul = lambda f: "%s: normWireRequired(normWireNullable(normWireNumber))" % f
        parsers = (
            "export const parseEn1994PlacementsPatch: NormWireReader<En1994PlacementsPatch> = normWireObject<En1994PlacementsPatch>({ %s, %s });\n" % (idx, ", ".join(nul(f) for f in FIELDS)) +
            "export const parseEn1994PlacementsInserted: NormWireReader<En1994PlacementsInserted> = normWireObject<En1994PlacementsInserted>({ %s, row: normWireRequired(parsePlacement) });\n" % idx +
            'export const parseEn1994PlacementsRows: NormWireReader<En1994PlacementsRows> = normWireObject<En1994PlacementsRows>({ removed: normWireRequired(normWireArray(normWireRange(normWireInteger, {"minimum":0}))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1994PlacementsInserted))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1994PlacementsPatch))) });\n')
        t = sub1(t, r"(export const parseEn1994SlabsRows: [^\n]*\n)", lambda m: m.group(1) + parsers)
        return sub1(t, r"(slabs: normWireDefault\(normWireNullable\(normWireRef\(\(\) => parseEn1994SlabsRows\)\), \(\) => null\),)", r"\1 placements: normWireDefault(normWireNullable(normWireRef(() => parseEn1994PlacementsRows)), () => null),")
    edit(A.path(DIFF + "/🟦️.ts"), go)


def diff_json():
    def go(t):
        data = json.loads(t)
        form, tail = json_form(t)
        if "placements" in data["properties"]:
            return t
        idx = {"type": "integer", "minimum": 0}
        patch = {"type": "object", "additionalProperties": False, "required": ["index"] + FIELDS, "properties": {"index": idx, **{f: {"anyOf": [{"$ref": SNAP + "/properties/" + f}, {"type": "null"}]} for f in FIELDS}}}
        ins = {"type": "object", "additionalProperties": False, "required": ["index", "row"], "properties": {"index": idx, "row": {"$ref": SNAP}}}
        rows = {"type": "object", "additionalProperties": False, "required": ["removed", "inserted", "modified"], "properties": {"removed": {"type": "array", "items": idx}, "inserted": {"type": "array", "items": {"$ref": "#/$defs/En1994PlacementsInserted"}}, "modified": {"type": "array", "items": {"$ref": "#/$defs/En1994PlacementsPatch"}}}}
        defs = {}
        for k, v in data["$defs"].items():
            defs[k] = v
            if k == "En1994SlabsRows":
                defs["En1994PlacementsPatch"], defs["En1994PlacementsInserted"], defs["En1994PlacementsRows"] = patch, ins, rows
        data["$defs"] = defs
        props = {}
        for k, v in data["properties"].items():
            props[k] = v
            if k == "slabs":
                props["placements"] = {"anyOf": [{"$ref": "#/$defs/En1994PlacementsRows"}, {"type": "null"}], "x-semio-state": "artifact"}
        data["properties"] = props
        return dumps_with(form, data) + tail
    edit(A.path(DIFF + "/🔣️.json"), go)


def diff_layers_1994():
    diff_rust()
    diff_graphql()
    diff_proto()
    diff_ts()
    diff_json()


def sqlite_layers():
    q = "/🚪️io/🪶️sqlite/📸️snapshot"
    TB = "en1994_placement"
    cols = ["id INTEGER PRIMARY KEY", "document_id INTEGER NOT NULL REFERENCES en1994_document(id)", "ordinal INTEGER NOT NULL CHECK(ordinal>=0)", "logical_id TEXT NOT NULL", "x REAL", "y REAL", "z REAL", "yaw REAL"] + sum([["%s_ieee754_bits INTEGER NOT NULL" % f, "%s_ieee754_class TEXT NOT NULL" % f] for f in FIELDS], [])
    ddl = "CREATE TABLE %s (%s);" % (TB, ",".join(cols))
    edit(A.path(q + "/🗄️.sql"), lambda t: t if TB in t else t.rstrip("\n") + "\n" + ddl + "\n")

    def rs(t):
        if "PLACEMENT" in t:
            return t
        t = sub1(t, r"CompositeSlab, SteelSection,", "CompositeSlab, Placement, SteelSection,")
        t = sub1(t, r"(const SLAB:[^\n]*\n)", r"\1const PLACEMENT:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7)];\n")
        t = sub1(t, r"(  for\(position,s\)in self\.slabs\.iter\(\)\.enumerate\(\)\{[^\n]*\n)", lambda m: m.group(1) + '  for(position,p)in self.placements.iter().enumerate(){out.insert_float("en1994_placement",&[Cell::Integer(1),ordinal(position)?,Cell::Text(&p.id),Cell::Real(p.x),Cell::Real(p.y),Cell::Real(p.z),Cell::Real(p.yaw)],PLACEMENT)?;}\n')
        t = sub1(t, r'("EN1994 slab row count overflow"\)\)\?\)\?;)', r"\1rows=add(rows,self.placements.len())?;")
        t = sub1(t, r"bound_actions\(&mut bound,&slab\.actions\)\?;\}bound\.finish\(\)", "bound_actions(&mut bound,&slab.actions)?;}for p in &self.placements{bound.add(4096)?;bound.repeated(p.id.len(),24)?;}bound.finish()")
        t = sub1(t, r"(let slab_action_rows=entities\(database,\"en1994_slab_action\",11,ACTION,control\)\?;)", r'\1let placement_rows=entities(database,"en1994_placement",8,PLACEMENT,control)?;')
        t = sub1(t, r"(let slab_actions=groups\(&slab_action_rows,&slab_rows,control\)\?;)", r"\1let placement_groups=groups(&placement_rows,&documents,control)?;")
        t = sub1(t, r"(\n  let snapshot=Self\{)", lambda m: '\n  let mut placements=Vec::new();for p in siblings(&placement_groups,1){checkpoint(control,placements.len(),placement_rows.len())?;placements.push(Placement{id:reconstruct_text(control,p.text(3)?)?,x:p.real(4)?,y:p.real(5)?,z:p.real(6)?,yaw:p.real(7)?});}' + m.group(1))
        t = sub1(t, r"beams,columns,slabs\};control", "beams,columns,slabs,placements};control")
        t = sub1(t, r"SQLITE_SCHEMA,11,39", "SQLITE_SCHEMA,12,39")
        return t
    edit(A.path(q + "/🦀️.rs"), rs)

    def ts(t):
        if "PLACEMENT" in t:
            return t
        t = sub1(t, r"CompositeSlab,SteelSection,", "CompositeSlab,Placement,SteelSection,")
        t = sub1(t, r"(export type En1994SqliteColumnAction=[^\n]*\n)", lambda m: m.group(1) + 'export type En1994SqlitePlacement=Exact<Placement,"x"|"y"|"z"|"yaw">;\n')
        t = sub1(t, r'"columns"\|"slabs">&\{', '"columns"|"slabs"|"placements">&{')
        t = sub1(t, r"slabs:En1994SqliteSlab\[\]\};", "slabs:En1994SqliteSlab[];placements:En1994SqlitePlacement[]};")
        t = sub1(t, r"(const SLAB:[^\n]*\n)", r"\1const PLACEMENT:readonly Ieee754Column[]=[4,5,6,7].map(index=>({index,width:64}));\n")
        t = sub1(t, r'await actions\(out,"en1994_slab_action",parent,s\.actions,options\)\}return out\.finish\(\);', 'await actions(out,"en1994_slab_action",parent,s.actions,options)}\n for(let index=0;index<snapshot.placements.length;index++){const p=snapshot.placements[index]!;await write(out,"en1994_placement",[1n,BigInt(index),p.id,p.x,p.y,p.z,p.yaw],PLACEMENT,options)}return out.finish();')
        t = sub1(t, r"(slabActionRows=await entities\(tables\[10\]!,11,ACTION,options\))", r"\1,placementRows=await entities(tables[11]!,8,PLACEMENT,options)")
        t = sub1(t, r"(slabActions=await groups\(slabActionRows,slabRows,options\);)", r"\1const placementGroups=await groups(placementRows,documents,options);")
        t = sub1(t, r"(\n const snapshot:En1994SqliteSnapshot=\{)", lambda m: '\n const placements:En1994SqlitePlacement[]=[];for(const p of placementGroups.get(1n)??[]){await checkpoint(options,placements.length,placementRows.size);placements.push({id:artifactSqliteText(p,3),x:readBinary64(p,4,PLACEMENT),y:readBinary64(p,5,PLACEMENT),z:readBinary64(p,6,PLACEMENT),yaw:readBinary64(p,7,PLACEMENT)})}' + m.group(1))
        t = sub1(t, r"beams,columns,slabs\};await artifactSqliteCheckpoint", "beams,columns,slabs,placements};await artifactSqliteCheckpoint")
        return t
    edit(A.path(q + "/🟦️.ts"), ts)

    def adm(t):
        if "en1994_placement" in t:
            return t
        t = sub1(t, r"for\(id,actions,nested\)in\[\(3,17,4\),\(4,13,1\),\(5,9,2\)\]\{(.*?)\}Ok\(rows\)\}", lambda m: m.group(0).replace("}Ok(rows)}", "}rows=n::add(rows,n::list(n::field(root,9)?)?.len())?;Ok(rows)}"), flags=re.S)
        t = sub1(t, r"(actions\(out,\"en1994_slab_action\",parent,s,9\)\?;\}Ok\(\(\)\))", lambda m: 'actions(out,"en1994_slab_action",parent,s,9)?;}\n for(index,p)in n::list(n::field(root,9)?)?.iter().enumerate(){n::entity(out,"en1994_placement",1,index,&n::cells(n::record(p)?,[Text(0),Real(1),Real(2),Real(3),Real(4)])?,PLACEMENT)?;}Ok(())')
        t = sub1(t, r"SQLITE_SCHEMA,11,39", "SQLITE_SCHEMA,12,39")
        return t.replace("/// 🛂️ Eleven authored", "/// 🛂️ Twelve authored")
    edit(A.path(q + "/🛂️admission/🦀️.rs"), adm)

    def rt(t):
        if "placements" in t:
            return t
        t = sub1(t, r"(\n for s in &mut s\.slabs\{[^\n]*\n)", lambda m: m.group(1) + " for p in &mut s.placements{p.x=v;p.y=v;p.z=v;p.yaw=v;}\n")
        t = sub1(t, r"slabs\.clear\(\);assert_eq!\(database\(&empty\)\.tables\.len\(\),11\)", "slabs.clear();empty.placements.clear();assert_eq!(database(&empty).tables.len(),12)")
        return t.replace("max_tables:11", "max_tables:12").replace('("tables",11)', '("tables",12)')
    edit(A.path(q + "/🧪️tests/🦀️.rs"), rt)

    def tt(t):
        if "placements" in t:
            return t
        t = sub1(t, r"(\n?)(\bslabs:fixture\.slabs\.map\()", r"placements:fixture.placements.map(p=>floats(p,[\"x\",\"y\",\"z\",\"yaw\"])),\2")
        t = sub1(t, r"columns:\[\],slabs:\[\]\}", "columns:[],slabs:[],placements:[]}")
        t = sub1(t, r"source\.slabs=\[\];", "source.slabs=[];source.placements=[];")
        return t.replace("toBe(11)", "toBe(12)").replace("maxTables:11", "maxTables:12")
    edit(A.path(q + "/🧪️tests/🟦️.ts"), tt)
    sqlite_extent(A, A.path(q + "/🗄️.sql"), A.path(q + "/🧪️tests/🧫️fixtures/🎛️semantic.json"), A.path(q + "/🧪️tests/🟦️.ts"), None, 11, TB)


def rest_layers():
    rows = [
        '    ("placements", m("Placements", "Platzierungen")),',
        '    ("placements[].id", m("Placed entity id", "Platzierte Element-Id")),',
        '    ("placements[].x", mu("Placement x", "Platzierung x", UnitId::Metre)),',
        '    ("placements[].y", mu("Placement y", "Platzierung y", UnitId::Metre)),',
        '    ("placements[].z", mu("Placement z", "Platzierung z", UnitId::Metre)),',
        '    ("placements[].yaw", mu("Placement yaw about Z", "Platzierung Gierwinkel um Z", UnitId::Radian)),',
    ]
    edit(A.path("/✏️editor/🏷️field-meta/🦀️.rs"), lambda t: append_after_last(t, r'^\s+\("slabs\[\]\.asM2PerM"', rows))
    i = "/🧬️schema/💡️inferences"

    def outline(t):
        if "placements" in t:
            return t
        t = sub1(t, r'(\n    "fatigue_detail",\n)', r'\1    "placements",\n')
        return sub1(t, r"snapshot\.slabs\.len\(\)\) as u32", "snapshot.slabs.len() + snapshot.placements.len()) as u32")
    edit(A.path(i + "/🧾outline/🦀️.rs"), outline)
    edit(A.path("/🧬️schema/📸️snapshot/🦀️.rs"), lambda t: t if "placements: vec![]" in t else sub1(t, r'(            fatigue_detail: "stud_welded"\.into\(\),\n)', r"\1            placements: vec![],\n"))
    import padd
    padd.EMPTY_DELTA = '{"removed": [], "inserted": [], "modified": []}'
    fixtures(A)


if __name__ == "__main__":
    schema_layers()
    diff_layers_1994()
    sqlite_layers()
    rest_layers()
