import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from padd import *

A = Art("🪵️en1995", "En1995", "connections", "connections", "Connection", model="TimberConnection")
S = "/🧬️schema"
Q = "/🚪️io/🪶️sqlite/📸️snapshot"
TB = "en1995_placement"


def opaque_root_ts(path):
    def go(t):
        if "placements:" in t:
            return t
        t = clone_line(t, r"^\s+connections: ", A, fix=emoji_fix)
        def p(m):
            line = m.group(0)
            s, e = entry_span(line, "connections")
            return line[:e] + ", " + A.rn(line[s:e]) + line[e:]
        return re.sub(r"^export const parseEn1995Artifact: .*$", p, t, flags=re.M)
    edit(path, go)


def opaque_graphql(path):
    edit(path, lambda t: t if "placements:" in t else sub1(t, r"(\n  cases: \[JSON!\]!)", r"\1\n  placements: [JSON!]!"))


def opaque_proto(path):
    edit(path, lambda t: t if "placements" in t else sub1(t, r"(\n  repeated bytes cases = (\d+);)", lambda m: m.group(1) + "\n  repeated bytes placements = %d;" % (int(m.group(2)) + 1)))


def opaque_json(path):
    def go(t):
        data = json.loads(t)
        if "placements" in data["properties"]:
            return t
        form, tail = json_form(t)
        data["properties"]["placements"] = json.loads(json.dumps(data["properties"]["cases"]))
        data["required"].append("placements")
        return dumps_with(form, data) + tail
    edit(path, go)


def diff_opaque_proto(path):
    def go(t):
        if "placements" in t:
            return t
        n = max(int(x) for x in re.findall(r"= (\d+);", t)) + 1
        return sub1(t, r"(  // @state artifact\n  bytes cases = \d+;\n)", lambda m: m.group(1) + "  // @state artifact\n  bytes placements = %d;\n" % n)
    edit(path, go)


def relocate_after_cases(path, root_kind):
    def go(t):
        m = re.search(r"((?:    #\[[^\n]*\n)+)    pub placements: [^\n]*\n", t)
        c = re.search(r"    pub cases: [^\n]*\n", t)
        if not m or not c or m.start() > c.start():
            return t
        block = m.group(0)
        t = t[:m.start()] + t[m.end():]
        c = re.search(r"    pub cases: [^\n]*\n", t)
        return t[:c.end()] + block + t[c.end():]
    edit(path, go)


def schema_layers():
    opaque_root_ts(A.path(S + "/🟦️.ts"))
    layer_model_struct(A, A.path(S + "/📸️snapshot/🦀️.rs"))
    edit(A.path(S + "/📸️snapshot/🦀️.rs"), lambda t: t if "pub placements:" in t else clone_line(t, r"^\s+(pub )?connections: (Vec<TimberConnection>,|vec!\[\],|Vec::new\(\),)", A))
    edit(A.path(S + "/🦀️.rs"), lambda t: t if "pub placements:" in t else clone_line(t, r"^\s+(pub )?connections: ", A))
    relocate_after_cases(A.path(S + "/📸️snapshot/🦀️.rs"), "snapshot")
    relocate_after_cases(A.path(S + "/🦀️.rs"), "root")
    ts_iface(A, A.path(S + "/📸️snapshot/🟦️.ts"))
    ts_root_field(A, A.path(S + "/📸️snapshot/🟦️.ts"))
    for f in ("/🔗️.graphql", "/📸️snapshot/🔗️.graphql"):
        opaque_graphql(A.path(S + f))
    for f in ("/🛰️.proto", "/📸️snapshot/🛰️.proto"):
        opaque_proto(A.path(S + f))
    opaque_json(A.path(S + "/🔣️.json"))
    json_snapshot(A, A.path(S + "/📸️snapshot/🔣️.json"))
    diff_layers(A)
    diff_opaque_proto(A.path(S + "/🔺️diff/🛰️.proto"))

    def diff_fix(t):
        t = t.replace(" nest { actions: En1995PlacementActionDelta }", "")
        if "placements: self.placements.commit_onto" in t:
            return t
        return sub1(t, r"(            cases: self\.cases\.commit_onto\(&base\.cases, capability\)\.map_err\(\|error\| error\.under\(\[\"cases\"\]\)\)\?,\n)", lambda m: m.group(1) + '            placements: self.placements.commit_onto(&base.placements, capability).map_err(|error| error.under(["placements"]))?,\n')
    edit(A.path(S + "/🔺️diff/🦀️.rs"), diff_fix)
    edit(A.path(S + "/🧬️schema/x"), lambda t: t) if False else None

    def root_conv(t):
        if "placements: self.placements" in t:
            return t
        t = sub1(t, r"cases: self\.cases\.clone\(\) \}", "cases: self.cases.clone(), placements: self.placements.clone() }")
        return sub1(t, r"cases: snapshot\.cases \}", "cases: snapshot.cases, placements: snapshot.placements }")
    edit(A.path(S + "/🦀️.rs"), root_conv)
    edit(A.path(S + "/📸️snapshot/🦀️.rs"), lambda t: t if "cases: Vec::new(), placements" in t else sub1(t, r"cases: Vec::new\(\) \}", "cases: Vec::new(), placements: Vec::new() }"))
    literal_sweep(A)
    multiline_literal_sweep(A)
    fixtures(A)


def rest_layers():
    i = S + "/💡️inferences"
    edit(A.path(i + "/🧾outline/🦀️.rs"), lambda t: t if '"placements"' in t else sub1(sub1(t, r'"cases"\]', '"cases", "placements"]'), r"snapshot\.cases\.len\(\)\)", "snapshot.cases.len() + snapshot.placements.len())"))
    t_ = S + "/⚖️timber/🏗️structure-and-members/🦀️.rs"

    def integrity(t):
        if "placements" in t:
            return t
        t = sub1(t, r"connections: &\[TimberConnection\]\) \{", "connections: &[TimberConnection], placements: &[Placement]) {")
        t = sub1(t, r"(        &\|id\| format!\(\"connections\[id=\{id\}\]\.id\"\),\n    \);\n)", lambda m: m.group(1) + '    push_duplicate_ids(report, annex, "placements", &placements.iter().map(|p| p.id.clone()).collect::<Vec<_>>(), &|id| format!("placements[id={id}].id"));\n')
        return sub1(t, r"push_referential_integrity\(&mut report, annex, members, connections\);", "push_referential_integrity(&mut report, annex, members, connections, &document.placements);")
    edit(A.path(t_), integrity)
    rows = ['    ("placements", FieldSpec::new("Placements", "Platzierungen")),', '    ("placements[].id", FieldSpec::new("Placed entity id", "Platzierte Element-Id")),'] + ['    ("placements[].%s", FieldSpec::quantity("Placement %s", "Platzierung %s", UnitId::Metre)),' % (f, f, f) for f in "xyz"] + ['    ("placements[].yaw", FieldSpec::quantity("Placement yaw about Z", "Platzierung Gierwinkel um Z", UnitId::Radian)),']
    edit(A.path("/✏️editor/🏷️field-meta/🦀️.rs"), lambda t: t if '"placements"' in t else append_after_last(t, r'^\s+\("cases\[\]\.texts\[\]\.value", ', rows))


def sqlite_layers():
    sqltxt = ("CREATE TABLE en1995_placement (\n  id INTEGER PRIMARY KEY,\n  document_id INTEGER NOT NULL REFERENCES en1995_document(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  logical_id TEXT NOT NULL,\n  x REAL, y REAL, z REAL, yaw REAL,\n" +
              ",\n".join("  %s_ieee754_bits INTEGER NOT NULL, %s_ieee754_class TEXT NOT NULL" % (f, f) for f in FIELDS) + "\n);\n")
    edit(A.path(Q + "/🗄️.sql"), lambda t: t if TB in t else t.rstrip("\n") + "\n" + sqltxt)

    def rs(t):
        if "PLACEMENT_FLOATS" in t:
            return t
        t = sub1(t, r"CaseText, MemberRole", "CaseText, Placement, MemberRole")
        t = sub1(t, r"(const CONNECTION_ACTION_FLOATS[^\n]*\n)", r"\1const PLACEMENT_FLOATS:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7)];\n")
        t = sub1(t, r"(\n        Ok\(\(\)\)\n    \}\n    fn admit_sqlite_values)", lambda m: '\n        for(position,placement)in self.placements.iter().enumerate(){out.insert_float("en1995_placement",&[Cell::Integer(1),ordinal(position)?,Cell::Text(&placement.id),Cell::Real(placement.x),Cell::Real(placement.y),Cell::Real(placement.z),Cell::Real(placement.yaw)],PLACEMENT_FLOATS)?;}' + m.group(1))
        t = sub1(t, r"SQLITE_SCHEMA,8,70,control", "SQLITE_SCHEMA,9,70,control")
        t = sub1(t, r"\[self\.members\.len\(\),self\.connections\.len\(\),self\.cases\.len\(\)\]", "[self.members.len(),self.connections.len(),self.cases.len(),self.placements.len()]")
        t = sub1(t, r"(\n        bound\.finish\(\))", lambda m: "\n        for placement in &self.placements{bound.add(4096)?;bound.repeated(placement.id.len(),24)?;}" + m.group(1))
        t = sub1(t, r"(\n        let uls_equation=reconstruct_text)", lambda m: '\n        let mut placements=Vec::new();for row in ordered(entities(database,"en1995_placement",8,PLACEMENT_FLOATS,control)?.into_values(),1,control)?{checkpoint(control,placements.len(),0)?;placements.push(Placement{id:reconstruct_text(control,row.text(3)?)?,x:row.real(4)?,y:row.real(5)?,z:row.real(6)?,yaw:row.real(7)?});}' + m.group(1))
        t = sub1(t, r"uls_equation,cases\};control\.checkpoint", "uls_equation,cases,placements};control.checkpoint")
        return t
    edit(A.path(Q + "/🦀️.rs"), rs)

    def ts(t):
        if "PLACEMENT_FLOATS" in t:
            return t
        t = sub1(t, r"CaseRow,CaseNumber,CaseText\}from", "CaseRow,CaseNumber,CaseText,Placement}from")
        t = sub1(t, r"(export type En1995SqliteCase=[^\n]*\n)", lambda m: m.group(1) + 'export type En1995SqlitePlacement=Omit<Placement,"x"|"y"|"z"|"yaw">&{x:Binary64;y:Binary64;z:Binary64;yaw:Binary64};\n')
        t = sub1(t, r'"connections"\|"cases">&\{', '"connections"|"cases"|"placements">&{')
        t = sub1(t, r"cases:En1995SqliteCase\[\]\};", "cases:En1995SqliteCase[];placements:En1995SqlitePlacement[]};")
        t = sub1(t, r"(const CONNECTION_ACTION_FLOATS[^\n]*\n)", r"\1const PLACEMENT_FLOATS:readonly Ieee754Column[]=[4,5,6,7].map(index=>({index,width:64}));\n")
        t = sub1(t, r"(\n return out\.finish\(\);)", lambda m: '\n for(let position=0;position<snapshot.placements.length;position++){const p=snapshot.placements[position]!;await write(out,"en1995_placement",[1n,BigInt(position),p.id,p.x,p.y,p.z,p.yaw],PLACEMENT_FLOATS,options)}' + m.group(1))
        t = sub1(t, r"(\n const result:En1995SqliteSnapshot=)", lambda m: '\n const placementRows=await entities(tables[8]!,8,PLACEMENT_FLOATS,options),placements:En1995SqlitePlacement[]=[];for(const r of await ordered(placementRows.values(),1n,options)){await checkpoint(options,placements.length,0);placements.push({id:artifactSqliteText(r,3),x:readBinary64(r,4,PLACEMENT_FLOATS),y:readBinary64(r,5,PLACEMENT_FLOATS),z:readBinary64(r,6,PLACEMENT_FLOATS),yaw:readBinary64(r,7,PLACEMENT_FLOATS)})}' + m.group(1))
        t = sub1(t, r"members,connections,cases\};await", "members,connections,cases,placements};await")
        return t
    edit(A.path(Q + "/🟦️.ts"), ts)

    def adm(t):
        if "en1995_placement" in t:
            return t
        t = sub1(t, r"(rows=n::add\(rows,n::list\(n::field\(o,5\)\?\)\?\.len\(\)\)\?\})Ok\(rows\)", lambda m: m.group(1) + "rows=n::add(rows,n::list(n::field(root,5)?)?.len())?;Ok(rows)")
        t = sub1(t, r"(\n)( Ok\(\(\)\)\n\})", lambda m: m.group(1) + ' for(index,p)in n::list(n::field(root,5)?)?.iter().enumerate(){n::entity(out,"en1995_placement",1,index,&n::cells(n::record(p)?,[Text(0),Real(1),Real(2),Real(3),Real(4)])?,PLACEMENT_FLOATS)?;}\n' + m.group(2))
        t = sub1(t, r"SQLITE_SCHEMA,8,70", "SQLITE_SCHEMA,9,70")
        return t.replace("Exact five-table", "Exact nine-table")
    edit(A.path(Q + "/🛂️admission/🦀️.rs"), adm)

    def rtests(t):
        if "placements" in t:
            return t
        t = sub1(t, r"(    for c in &mut snapshot\.connections\{[^\n]*\n)", lambda m: m.group(1) + "    for p in &mut snapshot.placements{p.x=value;p.y=value;p.z=value;p.yaw=value;}\n")
        t = sub1(t, r'\("tables",8\)', '("tables",9)')
        t = sub1(t, r"database\(&empty\)\.tables\.len\(\),8", "database(&empty).tables.len(),9")
        t = sub1(t, r"empty\.cases\.clear\(\);", "empty.cases.clear();empty.placements.clear();")
        return t
    edit(A.path(Q + "/🧪️tests/🦀️.rs"), rtests)

    def ttests(t):
        if "placements" in t:
            return t
        t = sub1(t, r"ulsEquation:f\.ulsEquation,", "ulsEquation:f.ulsEquation,placements:f.placements.map(p=>({...p,x:binary64(p.x),y:binary64(p.y),z:binary64(p.z),yaw:binary64(p.yaw)})),")
        t = sub1(t, r"( for\(const c of s\.connections\)\{[^\n]*\n)", lambda m: m.group(1) + " for(const p of s.placements){p.x=v;p.y=v;p.z=v;p.yaw=v}\n")
        t = sub1(t, r"database\.tables\.length\)\.toBe\(5\)", "database.tables.length).toBe(9)")
        return t
    edit(A.path(Q + "/🧪️tests/🟦️.ts"), ttests)
    sqlite_extent(A, A.path(Q + "/🗄️.sql"), A.path(Q + "/🧪️tests/🧫️fixtures/🎛️semantic.json"), A.path(Q + "/🧪️tests/🟦️.ts"), None, 8, TB)


if __name__ == "__main__":
    schema_layers()
    rest_layers()
    sqlite_layers()
