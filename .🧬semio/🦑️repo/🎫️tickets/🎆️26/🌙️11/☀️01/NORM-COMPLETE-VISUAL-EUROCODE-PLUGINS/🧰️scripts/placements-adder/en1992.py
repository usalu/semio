import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from padd import *

A = Art("🏛️en1992", "En1992", "anchors", "anchors", "Anchor")
S = "/🧬️schema"
INLINE = ("placement_x", "placement_y", "placement_z", "placement_yaw")


def strip_model():
    p = A.path(S + "/📸️snapshot/🦀️.rs")
    edit(p, lambda t: re.sub(r"^\s*(pub )?placement_(x|y|z|yaw)(: [^\n]*)?,\n", "", t, flags=re.M))


def strip_twins():
    def ts(t):
        t = re.sub(r"^  placement(X|Y|Z|Yaw): number;\n", "", t, flags=re.M)
        return re.sub(r", placement(X|Y|Z|Yaw): normWireRequired\(normWireNumber\)", "", t)
    for f in ("/🟦️.ts", "/📸️snapshot/🟦️.ts"):
        edit(A.path(S + f), ts)
    for f in ("/🔗️.graphql", "/📸️snapshot/🔗️.graphql"):
        edit(A.path(S + f), lambda t: re.sub(r"^  placement(X|Y|Z|Yaw): Float!\n", "", t, flags=re.M))

    def proto(t):
        t = re.sub(r"^ double placement_(x|y|z|yaw) = \d+;\n", "", t, flags=re.M)
        t = t.replace(" double placement_x = 12; double placement_y = 13; double placement_z = 14; double placement_yaw = 15;", "")

        def renum(m):
            n = [0]

            def f(mm):
                n[0] += 1
                return "= %d;" % n[0]
            return re.sub(r"= \d+;", f, m.group(0))
        return re.sub(r"^message RcMember \{\n.*?\n\}\n", renum, t, flags=re.S | re.M)
    for f in ("/🛰️.proto", "/📸️snapshot/🛰️.proto"):
        edit(A.path(S + f), proto)

    def js(t):
        d = json.loads(t)
        form, tail = json_form(t)
        for k, v in d["$defs"].items():
            if k in ("RcMember", "Anchor"):
                for f in ("placementX", "placementY", "placementZ", "placementYaw"):
                    v["properties"].pop(f, None)
                v["required"] = [r for r in v["required"] if not r.startswith("placement")]
        return dumps_with(form, d) + tail
    for f in ("/🔣️.json", "/📸️snapshot/🔣️.json"):
        edit(A.path(S + f), js)


def proto_line(t):
    if "message Placement " in t:
        return t
    t = re.sub(r"^(message Anchor \{[^\n]*\}\n)", r"\1message Placement { string id = 1; double x = 2; double y = 3; double z = 4; double yaw = 5; }\n", t, flags=re.M)
    return t.replace("repeated Anchor anchors = 10; }", "repeated Anchor anchors = 10; repeated Placement placements = 11; }")


def add_collection():
    layer_model_struct(A, A.path(S + "/📸️snapshot/🦀️.rs"))
    edit(A.path(S + "/📸️snapshot/🦀️.rs"), lambda t: clone_line(t, r"^\s+(pub )?%s: (Vec<%s>,|vec!\[\],|Vec::new\(\),)" % (A.snake, A.model), A))
    edit(A.path(S + "/🦀️.rs"), lambda t: clone_line(t, r"^\s+(pub )?%s: " % A.snake, A))
    for f in ("/🟦️.ts", "/📸️snapshot/🟦️.ts"):
        ts_iface(A, A.path(S + f))
        ts_root_field(A, A.path(S + f))
    for f in ("/🔗️.graphql", "/📸️snapshot/🔗️.graphql"):
        graphql_snapshot(A, A.path(S + f))
    for f in ("/🛰️.proto", "/📸️snapshot/🛰️.proto"):
        edit(A.path(S + f), proto_line)
    for f in ("/🔣️.json", "/📸️snapshot/🔣️.json"):
        json_snapshot(A, A.path(S + f))
    literal_sweep(A)
    multiline_literal_sweep(A)


def strip_fixtures():
    n = 0
    rx = re.compile(r',[ \t]*(?:\n[ \t]*)?"placement(?:X|Y|Z|Yaw)"[ \t]*:[ \t]*(?:null|-?[0-9][0-9.eE+-]*)')
    for dp, ds, fs in os.walk(A.root):
        ds[:] = [d for d in ds if d not in ("target", "node_modules", "🧬️schema", "🎬️scene")]
        for f in fs:
            if not f.endswith(".json"):
                continue
            p = os.path.join(dp, f)
            t = rd(p)
            if '"placement' not in t:
                continue
            nt = rx.sub("", t)
            json.loads(nt)
            assert not re.search(r'"placement(X|Y|Z|Yaw)"', nt), p
            wr(p, nt)
            n += 1
    print("  fixtures stripped:", n)


def pad_fixtures():
    n = 0
    pat = re.compile(r'"anchors"\s*:\s*')
    for dp, ds, fs in os.walk(A.root):
        ds[:] = [d for d in ds if d not in ("target", "node_modules", "🧬️schema", "🎬️scene")]
        for f in fs:
            if not f.endswith(".json"):
                continue
            p = os.path.join(dp, f)
            t = rd(p)
            if '"anchors"' not in t:
                continue
            out, pos = [], 0
            for m in pat.finditer(t):
                if m.start() < pos:
                    continue
                s = m.end()
                e = value_end(t, s)
                if re.match(r'\s*,\s*"placements"', t[e:e + 40]):
                    continue
                val = t[s:e].lstrip()
                new = "[]" if val.startswith("[") else "null"
                ls = t.rfind("\n", 0, m.start()) + 1
                indent = t[ls:m.start()] if t[ls:m.start()].strip() == "" else None
                sep = (",\n" + indent) if indent is not None else ", "
                colon = ": " if ": " in t[m.start():s] else ":"
                out.append(t[pos:e])
                out.append('%s"placements"%s%s' % (sep, colon, new))
                pos = e
            out.append(t[pos:])
            nt = "".join(out)
            if nt != t:
                json.loads(nt)
                wr(p, nt)
                n += 1
    print("  fixtures padded:", n)


Q = "/🚪️io/🪶️sqlite/📸️snapshot"
IEEE = "".join(" %s_ieee754_bits INTEGER NOT NULL, %s_ieee754_class TEXT NOT NULL,\n" % (f, f) for f in FIELDS)[:-2] + "\n"
TABLE = ("CREATE TABLE en1992_placement (\n id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1992_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, x REAL, y REAL, z REAL, yaw REAL,\n" + IEEE + ");\n")
PL4 = "Cell::Real(v.placement_x),Cell::Real(v.placement_y),Cell::Real(v.placement_z),Cell::Real(v.placement_yaw)"


def sqlite_all():
    def sql(t):
        if "en1992_placement" in t:
            return t
        t = re.sub(r" placement_x REAL, placement_y REAL, placement_z REAL, placement_yaw REAL,", "", t)
        t = re.sub(r"^ placement_\w+_ieee754_bits INTEGER NOT NULL, placement_\w+_ieee754_class TEXT NOT NULL,?\n", "", t, flags=re.M)
        t = re.sub(r",\n\);", "\n);", t)
        return t.rstrip("\n") + "\n" + TABLE
    edit(A.path(Q + "/🗄️.sql"), sql)

    def rs(t):
        if "en1992_placement" in t:
            return t
        t = t.replace("FloatColumn::Binary64(29),FloatColumn::Binary64(30),FloatColumn::Binary64(31),FloatColumn::Binary64(32),FloatColumn::Binary64(33)];", "FloatColumn::Binary64(29)];")
        t = t.replace("FloatColumn::Binary64(12),FloatColumn::Binary64(13),FloatColumn::Binary64(14),FloatColumn::Binary64(15),FloatColumn::Binary64(16)];", "FloatColumn::Binary64(12)];")
        t = sub1(t, r"(const STUD:[^\n]*\n)", r"\1const PLACEMENT:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7)];\n")
        t = t.replace("," + PL4 + "]", "]")
        assert PL4 not in t
        t = re.sub(r",placement_x:v\.real\(30\)\?,placement_y:v\.real\(31\)\?,placement_z:v\.real\(32\)\?,placement_yaw:v\.real\(33\)\?", "", t)
        t = re.sub(r",placement_x:v\.real\(13\)\?,placement_y:v\.real\(14\)\?,placement_z:v\.real\(15\)\?,placement_yaw:v\.real\(16\)\?", "", t)
        t = t.replace('entities(d,"en1992_member",34,MEMBER,c)', 'entities(d,"en1992_member",30,MEMBER,c)').replace('entities(d,"en1992_anchor",17,ANCHOR,c)', 'entities(d,"en1992_anchor",13,ANCHOR,c)')
        t = sub1(t, r"(let anchor_rows=entities\(d,\"en1992_anchor\",13,ANCHOR,c\)\?;)", r'\1let placement_rows=entities(d,"en1992_placement",8,PLACEMENT,c)?;')
        t = sub1(t, r"(\n  let result=Self\{)", lambda m: '\n  let mut placements=Vec::new();for v in ordered(placement_rows.into_values(),1,c)?{placements.push(Placement{id:text(v,3,c)?,x:v.real(4)?,y:v.real(5)?,z:v.real(6)?,yaw:v.real(7)?});}' + m.group(1))
        t = sub1(t, r"(,anchors)(\}|,)", r"\1,placements\2") if re.search(r"Self\{[^\n]*,anchors[,}]", t) else t
        t = sub1(t, r"LoadCaseActions, Anchor,", "LoadCaseActions, Anchor, Placement,")
        t = sub1(t, r"(  for\(n,v\)in s\.anchors\.iter\(\)\.enumerate\(\)\{[^\n]*\n)", lambda m: m.group(1) + '  for(n,v)in s.placements.iter().enumerate(){p.insert_float("en1992_placement",&[Cell::Integer(1),ordinal(n)?,Cell::Text(&v.id),Cell::Real(v.x),Cell::Real(v.y),Cell::Real(v.z),Cell::Real(v.yaw)],PLACEMENT)?;}\n')
        return t
    edit(A.path(Q + "/🦀️.rs"), rs)


def sqlite_ts():
    def ts(t):
        if "en1992_placement" in t:
            return t
        t = re.sub(r"placement(?:X|Y|Z|Yaw):Binary64;", "", t)
        t = re.sub(r',"placement(?:X|Y|Z|Yaw)"', "", t)
        t = re.sub(r",v\.placement(?:X|Y|Z|Yaw)", "", t)
        t = re.sub(r",placement(?:X|Y|Z|Yaw):readBinary64\(v,\d+,(?:MEMBER|ANCHOR)\)", "", t)
        t = sub1(t, r"\[11,12,13,14,15,17,19,22,23,24,25,26,27,28,29,30,31,32,33\]", "[11,12,13,14,15,17,19,22,23,24,25,26,27,28,29]")
        t = sub1(t, r"\[4,6,7,8,9,10,11,12,13,14,15,16\]", "[4,6,7,8,9,10,11,12]")
        t = sub1(t, r"(const STUD_RING:[^\n]*\n)", r"\1const PLACEMENT:readonly Ieee754Column[]=[4,5,6,7].map(index=>({index,width:64}));\n")
        t = sub1(t, r"(export type En1992SqliteAnchor=[^\n]*\n)", r"\1export type En1992SqlitePlacement={id:string;x:Binary64;y:Binary64;z:Binary64;yaw:Binary64};\n")
        t = sub1(t, r"anchors:En1992SqliteAnchor\[\]\};", "anchors:En1992SqliteAnchor[];placements:En1992SqlitePlacement[]};")
        t = sub1(t, r'"members","anchors"\]\);', '"members","anchors","placements"]);')
        t = sub1(t, r"s\.members,s\.anchors\]\)collection", "s.members,s.anchors,s.placements])collection")
        t = sub1(t, r"(\n for\(let n=0;n<s\.anchors\.length;n\+\+\)\{[^\n]*\n)", lambda m: m.group(1) + ' for(let n=0;n<s.placements.length;n++){const v=s.placements[n]!;fields(v,["id","x","y","z","yaw"]);await write(out,"en1992_placement",[1n,BigInt(n),v.id,v.x,v.y,v.z,v.yaw],PLACEMENT,o)}\n')
        t = t.replace("entities(t[4]!,34,MEMBER,o)", "entities(t[4]!,30,MEMBER,o)").replace("entities(t[11]!,17,ANCHOR,o)", "entities(t[11]!,13,ANCHOR,o)")
        t = sub1(t, r"(const memberRows=await entities)", r"const placementRows=await entities(t[17]!,8,PLACEMENT,o);\n const placements:En1992SqlitePlacement[]=[];\n \1") if False else t
        t = sub1(t, r"(\n await artifactSqliteCheckpoint\(o,\"reconstructSnapshot\",1,1\);return\{)", lambda m: '\n const placements:En1992SqlitePlacement[]=[];for(const v of await ordered((await entities(t[17]!,8,PLACEMENT,o)).values(),1n,o))placements.push({id:artifactSqliteText(v,3),x:readBinary64(v,4,PLACEMENT),y:readBinary64(v,5,PLACEMENT),z:readBinary64(v,6,PLACEMENT),yaw:readBinary64(v,7,PLACEMENT)});' + m.group(1))
        t = sub1(t, r"prestressSteels,members,anchors\};", "prestressSteels,members,anchors,placements};")
        return t
    edit(A.path(Q + "/🟦️.ts"), ts)

    def cells(t):
        if "en1992_placement" in t:
            return t
        t = t.replace('("en1992_anchor",41)', '("en1992_anchor",29)').replace('("en1992_member",72)', '("en1992_member",60)')
        t = t.replace('("en1992_member_action",31),', '("en1992_member_action",31),("en1992_placement",16),')
        t = t.replace("shape(value,41)?;\n   c.row(\"en1992_member\"", "shape(value,37)?;\n   c.row(\"en1992_member\"")
        t = t.replace(",(32,Float),(33,Float),(34,Float),(35,Float),(36,Float)],n)?)?,n)?;", ",(32,Float)],n)?)?,n)?;")
        for a, b in (("field(value,37)?,3,\"en1992_section_point\"", "field(value,33)?,3,\"en1992_section_point\""), ("field(value,38)?,5", "field(value,34)?,5"), ("field(value,39)?,4,\"en1992_tendon_point\"", "field(value,35)?,4,\"en1992_tendon_point\""), ("field(value,40)?,4,\"en1992_stud_ring\"", "field(value,36)?,4,\"en1992_stud_ring\"")):
            t = sub1(t, re.escape(a), b)
        t = sub1(t, r"shape\(value,15\)\?;c\.row\(\"en1992_anchor\"", 'shape(value,11)?;c.row("en1992_anchor"')
        t = t.replace("(8,Float),(10,Float),(11,Float),(12,Float),(13,Float),(14,Float)],n)?)?,n)?;\n   collection(field(value,9)", "(8,Float),(10,Float)],n)?)?,n)?;\n   collection(field(value,9)")
        t = sub1(t, r"shape\(source,10\)\?", "shape(source,11)?")
        t = sub1(t, r"(  n\.checkpoint\(\)\n)", '  collection(field(source,10)?,5,"en1992_placement",&[(0,Text),(1,Float),(2,Float),(3,Float),(4,Float)],&mut c,n)?;\n\\1')
        return t
    edit(A.path(Q + "/📏️cells/🦀️.rs"), cells)


if __name__ == "__main__":
    step = sys.argv[1]
    {"strip_model": strip_model, "strip_twins": strip_twins, "add_collection": add_collection, "strip_fixtures": strip_fixtures, "pad_fixtures": pad_fixtures, "sqlite_all": sqlite_all, "sqlite_ts": sqlite_ts}[step]()
