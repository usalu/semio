import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from padd import *

def placement_def():
    pd = {"title": "Placement", "type": "object", "additionalProperties": False, "required": ["id"] + FIELDS, "properties": {"id": {"type": "string", "x-semio-ui": {"widget": "reference", "label": {"en": "Placed entity", "de": "Platziertes Element"}, "ref": {"kind": "entity"}}}}}
    for f, en, de, unit, disp, fac, prec, step in [("x", "Origin x", "Ursprung x", "m", "m", 1, 3, 0.1), ("y", "Origin y", "Ursprung y", "m", "m", 1, 3, 0.1), ("z", "Origin z", "Ursprung z", "m", "m", 1, 3, 0.1), ("yaw", "Yaw about Z", "Gierwinkel um Z", "rad", "°", 57.29577951308232, 1, 0.0174532925)]:
        pd["properties"][f] = {"type": "number", "x-semio-ui": {"label": {"en": en, "de": de}, "widget": "stepper", "unit": unit, "displayUnit": disp, "displayFactor": fac, "precision": prec, "step": step}}
    pd["x-semio-formats"] = ["🔣️jsonschema", "🟦️typescript"]
    return pd


def diff_json_text(path):
    def go(t):
        if '"placements"' in t:
            return t
        t = sub1(t, r'(\n    "roofSurfaces": \{\n      "\$ref": "#/\$defs/En1991RoofSurfaceDelta",\n      "x-semio-state": "artifact"\n    \})', lambda m: m.group(1) + "," + m.group(1).replace("roofSurfaces", "placements").replace("RoofSurface", "Placement"))
        lines = t.split("\n")
        idx = [i for i, l in enumerate(lines) if re.match(r'    "En1991RoofSurface(Patch|Removal|Insertion|Relocation|Modification|Delta)": ', l)]
        assert len(idx) == 6, idx
        number = {"anyOf": [{"type": "number"}, {"type": "null"}]}
        new = []
        for i in idx:
            l = A.rn(lines[i].rstrip(","))
            if "Patch" in l.split(":")[0]:
                key, value = l.strip().split(": ", 1)
                d = json.loads(value)
                d["properties"] = {f: number for f in FIELDS}
                l = "    " + key + ": " + json.dumps(d, separators=(",", ":"), ensure_ascii=False)
            new.append(l)
        last = idx[-1]
        lines[last] = lines[last].rstrip(",") + ","
        lines[last + 1:last + 1] = [x + "," for x in new[:-1]] + [new[-1] + ("," if lines[last].endswith(",") and not lines[last + 1].startswith("  }") else "")]
        return "\n".join(lines)
    edit(path, go)


def snapshot_json_text(path):
    def go(t):
        if '"placements"' in t:
            return t
        t = sub1(t, r'(\n    "roofSurfaces",)', r'\1\n    "placements",')
        t = sub1(t, r'(\n    "roofSurfaces": \{\n      "type": "array",\n      "items": \{\n        "\$ref": "#/\$defs/RoofSurface"\n      \},\n      "x-semio-state": "artifact"\n    \},)', lambda m: m.group(1) + m.group(1).replace("roofSurfaces", "placements").replace("RoofSurface", "Placement"))
        body = json.dumps(placement_def(), indent=2, ensure_ascii=False).replace("\n", "\n    ")
        m = re.search(r'\n    "RoofSurface": \{\n.*?\n    \}(?=\n  \}\n\}|,\n)', t, re.S)
        return t[:m.end()] + ',\n    "Placement": ' + body + t[m.end():]
    edit(path, go)


A = Art("🏋️en1991", "En1991", "roof_surfaces", "roofSurfaces", "RoofSurface")
S = "/🧬️schema"
Q = "/🚪️io/🪶️sqlite/📸️snapshot"
TB = "en1991_placement"


def opaque_proto(path):
    def go(t):
        if "placements" in t:
            return t
        m = re.search(r"^(\s+)string roof_surfaces = (\d+);\n", t, re.M)
        n = max(int(x) for x in re.findall(r"= (\d+);", t)) + 1
        return t[:m.end()] + "%sstring placements = %d;\n" % (m.group(1), n) + t[m.end():]
    edit(path, go)


def opaque_graphql(path):
    edit(path, lambda t: t if "placements:" in t else clone_line(t, r"^\s+roofSurfaces: \[JSON!\]", A, attrs=False))


def conversions(t):
    if "placements: self.placements" in t:
        return t
    t = sub1(t, r"(            roof_surfaces: self\.roof_surfaces\.clone\(\),\n)", lambda m: m.group(1) + "            placements: self.placements.clone(),\n")
    return sub1(t, r"(            roof_surfaces: snapshot\.roof_surfaces\.clone\(\),\n)", lambda m: m.group(1) + "            placements: snapshot.placements.clone(),\n")


def schema_layers():
    s = S
    layer_model_struct(A, A.path(s + "/📸️snapshot/🦀️.rs"))
    edit(A.path(s + "/📸️snapshot/🦀️.rs"), lambda t: t if "pub placements:" in t else clone_line(t, r"^\s+pub roof_surfaces: Vec<RoofSurface>,", A))
    edit(A.path(s + "/🦀️.rs"), lambda t: t if "pub placements:" in t else clone_line(t, r"^\s+pub roof_surfaces: ", A))
    edit(A.path(s + "/🦀️.rs"), conversions)
    for f in ("/🟦️.ts", "/📸️snapshot/🟦️.ts"):
        ts_iface(A, A.path(s + f))
        ts_root_field(A, A.path(s + f))
    for f in ("/🔗️.graphql", "/📸️snapshot/🔗️.graphql", "/🔺️diff/🔗️.graphql"):
        opaque_graphql(A.path(s + f))
    for f in ("/🛰️.proto", "/📸️snapshot/🛰️.proto", "/🔺️diff/🛰️.proto"):
        opaque_proto(A.path(s + f))
    json_snapshot(A, A.path(s + "/🔣️.json"))
    snapshot_json_text(A.path(s + "/📸️snapshot/🔣️.json"))
    d = s + "/🔺️diff"
    diff_rs(A, A.path(d + "/🦀️.rs"))
    diff_ts(A, A.path(d + "/🟦️.ts"))
    diff_ts_imports(A, A.path(d + "/🟦️.ts"))
    diff_json_text(A.path(d + "/🔣️.json"))
    literal_sweep(A)
    multiline_literal_sweep(A)
    fixtures(A)


def rest_layers():
    def default_literal(t):
        if re.search(r"^placements: vec!\[\],$", t, re.M):
            return t
        return sub1(t, r"^(roof_surfaces: vec!\[\],\n)", lambda m: m.group(1) + "placements: vec![],\n", re.M)
    edit(A.path(S + "/📸️snapshot/🦀️.rs"), default_literal)

    def commit(t):
        if "placements: self.placements.commit_onto" in t:
            return t
        return sub1(t, r"(            roof_surfaces: self\.roof_surfaces\.commit_onto\(&base\.roof_surfaces, capability\)\.map_err\(\|error\| error\.under\(\[\"roof_surfaces\"\]\)\)\?,\n)", lambda m: m.group(1) + '            placements: self.placements.commit_onto(&base.placements, capability).map_err(|error| error.under(["placements"]))?,\n')
    edit(A.path(S + "/🔺️diff/🦀️.rs"), commit)
    i = S + "/💡️inferences"

    def outline(t):
        if '"placements"' in t:
            return t
        t = sub1(t, r'(    "roof_surfaces",\n)\];', lambda m: m.group(1) + '    "placements",\n];')
        return sub1(t, r"\+ snapshot\.roof_surfaces\.len\(\)\) as u32", "+ snapshot.roof_surfaces.len()\n            + snapshot.placements.len()) as u32")
    edit(A.path(i + "/🧾outline/🦀️.rs"), outline)
    edit(A.path(i + "/🦀️.rs"), lambda t: t if '"placements"' in t else clone_line(t, r'^\s+push_duplicate_ids\(report, document, "roofSurfaces"', A, attrs=False, rn=lambda s: s.replace("roofSurfaces", "placements").replace("roof_surfaces", "placements").replace("|r| r.id", "|p| p.id")))
    rows = ['        "placements" => FieldSpec::new("Placements", "Platzierungen"),', '        "placements[].id" => FieldSpec::new("Placed entity id", "Platzierte Element-Id"),'] + ['        "placements[].%s" => FieldSpec::quantity("Placement %s", "Platzierung %s", UnitId::Metre),' % (f, f, f) for f in "xyz"] + ['        "placements[].yaw" => FieldSpec::quantity("Placement yaw about Z", "Platzierung Gierwinkel um Z", UnitId::Radian),']
    edit(A.path("/✏️editor/🏷️field-meta/🦀️.rs"), lambda t: t if '"placements"' in t else append_after_last(t, r'^\s+"roofSurfaces\[\]\.extent" =>', rows))


def sqlite_layers():
    edit(A.path(Q + "/🗄️.sql"), lambda t: t if TB in t else t.rstrip("\n") + "\n" + sql_table(A, TB) + "\n")

    def rs(t):
        if "PLACEMENT_FLOATS" in t:
            return t
        t = sub1(t, r"ConstructionStage, RoofSurface\};", "ConstructionStage, RoofSurface, Placement};")
        t = sub1(t, r"(const ROOF_SURFACE_FLOATS[^\n]*\n)", r"\1const PLACEMENT_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4), FloatColumn::Binary64(5), FloatColumn::Binary64(6), FloatColumn::Binary64(7)];\n")
        t = sub1(t, r"(        for \(position, row\) in self\.bridge_lanes\.iter\(\)\.enumerate\(\) \{[^\n]*\n)", lambda m: m.group(1) + '        for (position, row) in self.placements.iter().enumerate() { out.insert_float( "en1991_placement", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Real(row.x), Cell::Real(row.y), Cell::Real(row.z), Cell::Real(row.yaw)], PLACEMENT_FLOATS)?; }\n')
        t = sub1(t, r"SQLITE_SCHEMA,\s?11,\s?188", lambda m: m.group(0).replace("11", "12"))
        t = sub1(t, r"self\.roof_surfaces\.len\(\)\]\{rows=add", "self.roof_surfaces.len(),self.placements.len()]{rows=add")
        t = sub1(t, r"(        for row in &self\.roof_surfaces \{[^\n]*\n)", lambda m: m.group(1) + "        for row in &self.placements { bound.add(4096)?; bound.repeated(row.id.len(), 24)?; }\n")
        t = sub1(t, r"(        let construction_stage_rows = [^\n]*\n)", lambda m: m.group(1) + '        let placement_rows = entities(database, "en1991_placement", 8, PLACEMENT_FLOATS, control)?;\n')
        t = sub1(t, r"(        let mut roof_surfaces = Vec::new\(\);\n[^\n]*\n)", lambda m: m.group(1) + '        let mut placements = Vec::new();\n        for row in ordered(placement_rows.into_values(), 1, control)? { checkpoint(control, placements.len(), 0)?; placements.push(Placement { id: reconstruct_text(control, row.text(3)?)?, x: row.real(4)?, y: row.real(5)?, z: row.real(6)?, yaw: row.real(7)? }); }\n')
        t = sub1(t, r"(            roof_surfaces,\n)(        \};)", lambda m: m.group(1) + "            placements,\n" + m.group(2))
        return t
    edit(A.path(Q + "/🦀️.rs"), rs)

    def ts(t):
        if "PLACEMENT_FLOATS" in t:
            return t
        t = sub1(t, r"ConstructionStage,RoofSurface\}from", "ConstructionStage,RoofSurface,Placement}from")
        t = sub1(t, r"(export type En1991SqliteRoofSurface=[^\n]*\n)", lambda m: m.group(1) + 'export type En1991SqlitePlacement=Omit<Placement,"x"|"y"|"z"|"yaw">&{x:Binary64;y:Binary64;z:Binary64;yaw:Binary64};\n')
        t = sub1(t, r'\|"roofSurfaces"\|"accidentalCases">&\{', '|"roofSurfaces"|"placements"|"accidentalCases">&{')
        t = sub1(t, r"bridgeLanes:En1991SqliteBridgeLane\[\];", "bridgeLanes:En1991SqliteBridgeLane[];placements:En1991SqlitePlacement[];")
        t = sub1(t, r"(const ROOF_SURFACE_FLOATS[^\n]*\n)", r"\1const PLACEMENT_FLOATS:readonly Ieee754Column[]=[4,5,6,7].map(index=>({index,width:64}));\n")
        t = sub1(t, r"( for\(let position=0;position<s\.roofSurfaces\.length;position\+\+\)\{[^\n]*\n)", lambda m: m.group(1) + ' for(let position=0;position<s.placements.length;position++){const p=s.placements[position]!;await write(out,"en1991_placement",BigInt(position+1),[1n,BigInt(position),p.id,p.x,p.y,p.z,p.yaw],PLACEMENT_FLOATS,options)}\n')
        t = sub1(t, r"( const roofSurfaceRows=[^\n]*\n)", lambda m: m.group(1) + ' const placementRows=await entities(tables[11]!,8,PLACEMENT_FLOATS,options);\n')
        t = sub1(t, r"( const roofSurfaces:En1991SqliteRoofSurface\[\]=\[\];[^\n]*\n)", lambda m: m.group(1) + ' const placements:En1991SqlitePlacement[]=[];for(const r of await ordered(placementRows.values(),1n,options)){await checkpoint(options,placements.length,0);placements.push({id:artifactSqliteText(r,3),x:readBinary64(r,4,PLACEMENT_FLOATS),y:readBinary64(r,5,PLACEMENT_FLOATS),z:readBinary64(r,6,PLACEMENT_FLOATS),yaw:readBinary64(r,7,PLACEMENT_FLOATS)})}\n')
        t = sub1(t, r"constructionStages,roofSurfaces\};", "constructionStages,roofSurfaces,placements};")
        return t
    edit(A.path(Q + "/🟦️.ts"), ts)

    def adm(t):
        if "en1991_placement" in t:
            return t
        t = sub1(t, r"for id in 77\.\.=84\{", "for id in 77..=85{")
        t = sub1(t, r"(\n)( Ok\(\(\)\)\n\})", lambda m: m.group(1) + ' for(index,v)in n::list(n::field(root,85)?)?.iter().enumerate(){n::entity(out,"en1991_placement",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Real(3),Real(4)])?,PLACEMENT_FLOATS)?;}\n' + m.group(2))
        t = sub1(t, r"SQLITE_SCHEMA,11,188", "SQLITE_SCHEMA,12,188")
        return t.replace("Eleven authored", "Twelve authored")
    edit(A.path(Q + "/🛂️admission/🦀️.rs"), adm)

    def rtests(t):
        if "placements" in t:
            return t
        t = sub1(t, r"(    for row in &mut snapshot\.roof_surfaces \{[^\n]*\n)", lambda m: m.group(1) + "    for row in &mut snapshot.placements { row.x = value; row.y = value; row.z = value; row.yaw = value; }\n")
        t = sub1(t, r"max_tables:11", "max_tables:12")
        t = sub1(t, r"max_tables:10", "max_tables:11")
        t = sub1(t, r'\("tables",11\)', '("tables",12)')
        t = sub1(t, r"tables\.len\(\), 11\)", "tables.len(), 12)")
        t = sub1(t, r"empty\.roof_surfaces\.clear\(\);", "empty.roof_surfaces.clear(); empty.placements.clear();")
        return t
    edit(A.path(Q + "/🧪️tests/🦀️.rs"), rtests)

    def ttests(t):
        if "placements" in t:
            return t
        t = sub1(t, r"(roofSurfaces:f\.roofSurfaces\.map\(w=>\(\{\.\.\.w,pitchDeg:binary64\(w\.pitchDeg\),extent:binary64\(w\.extent\)\}\)\),)", lambda m: m.group(1) + "placements:f.placements.map(w=>({...w,x:binary64(w.x),y:binary64(w.y),z:binary64(w.z),yaw:binary64(w.yaw)})),")
        t = sub1(t, r"(for\(const w of s\.roofSurfaces\)\{w\.pitchDeg=v;w\.extent=v;\})", lambda m: m.group(1) + "for(const w of s.placements){w.x=v;w.y=v;w.z=v;w.yaw=v;}")
        t = sub1(t, r"expect\(database\.tables\.length\)\.toBe\(11\)", "expect(database.tables.length).toBe(12)")
        t = sub1(t, r"maxTables:11", "maxTables:12")
        t = sub1(t, r"maxTables:10", "maxTables:11")
        return t
    edit(A.path(Q + "/🧪️tests/🟦️.ts"), ttests)
    sqlite_extent(A, A.path(Q + "/🗄️.sql"), A.path(Q + "/🧪️tests/🧫️fixtures/🎛️semantic.json"), A.path(Q + "/🧪️tests/🟦️.ts"), None, 11, TB)


if __name__ == "__main__":
    schema_layers()
    rest_layers()
    sqlite_layers()
