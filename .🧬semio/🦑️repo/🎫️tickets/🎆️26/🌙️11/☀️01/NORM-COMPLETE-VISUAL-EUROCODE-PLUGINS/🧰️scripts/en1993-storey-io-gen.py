"""Adds the `Storey` table to the EN 1993 SQLite snapshot codecs (SQL, Rust, TypeScript, admission, semantic contract). Idempotent. Usage: python3 -I en1993-storey-io-gen.py."""
import json, pathlib, re

HERE = pathlib.Path(__file__).resolve().parent
exec(compile((HERE / "en1993-storey-spec.py").read_text(encoding="utf8"), "spec", "exec"))
A = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any")
S = A / "🚪️io/🪶️sqlite/📸️snapshot"
rd = lambda p: p.read_text(encoding="utf8")
wr = lambda p, s: p.write_text(s, encoding="utf8")


def sub1(s, pattern, repl, flags=0):
    out, n = re.subn(pattern, repl, s, count=1, flags=flags)
    assert n == 1, f"no match: {pattern[:100]}"
    return out


NAMES = [fd["name"] for fd in FIELDS]
K0 = 3
IDX = {n: i + 4 for i, n in enumerate(NAMES[1:])}
IDX["id"] = 3
FLOATS = [IDX[fd["name"]] for fd in FIELDS if fd["ty"] == "f64"]
WIDTH = 4 + len(FIELDS) - 1


def sql():
    p = S / "🗄️.sql"
    s = rd(p)
    if "en1993_storey" in s:
        return
    cols = ""
    ieee = ""
    for fd in FIELDS[1:]:
        n = fd["name"]
        if fd["ty"] == "String":
            cols += f",{n} TEXT NOT NULL"
        elif fd["ty"] == "u32":
            cols += f",{n} INTEGER NOT NULL CHECK({n}>=0 AND {n}<=4294967295)"
        else:
            cols += f",{n} REAL"
            ieee += f",{n}_ieee754_bits INTEGER NOT NULL,{n}_ieee754_class TEXT NOT NULL"
    table = f"CREATE TABLE en1993_storey(\n id INTEGER PRIMARY KEY,document_id INTEGER NOT NULL REFERENCES en1993_document(id),ordinal INTEGER NOT NULL CHECK(ordinal>=0),logical_id TEXT NOT NULL{cols},\n {ieee[1:]}\n);\n"
    wr(p, s.rstrip("\n") + "\n" + table)


def rust():
    p = S / "🦀️.rs"
    s = rd(p)
    if "STOREY" in s:
        return
    s = sub1(s, r"(use semio_s_artifact_norm_en1993_model::\{[^}]*CraneRunway, )", lambda m: m.group(1) + "Storey, ")
    s = sub1(s, r"(const CRANE:&\[FloatColumn\]=[^\n]*\n)", lambda m: m.group(1) + "const STOREY:&[FloatColumn]=&[" + ",".join(f"FloatColumn::Binary64({i})" for i in FLOATS) + "];\n")
    cells = ",".join(f"Cell::Text(&v.{n})" if fd["ty"] == "String" else f"Cell::Integer(i64::from(v.{n}))" if fd["ty"] == "u32" else f"Cell::Real(v.{n})" for fd in FIELDS for n in [fd["name"]])
    ins = f'  for(n,v)in self.storeys.iter().enumerate(){{p.insert_float("en1993_storey",&[Cell::Integer(1),ordinal(n)?,{cells}],STOREY)?;}}\n'
    s = sub1(s, r'(  for\(n,v\)in self\.crane_runways\.iter\(\)\.enumerate\(\)\{.*?write_forces\(p,"en1993_crane_action",id,&v\.actions\)\?;\})(Ok\(\(\)\))', lambda m: m.group(1) + "\n" + ins.rstrip("\n") + m.group(2), re.S)
    assert s.count("self.crane_runways.len()") >= 2
    s = s.replace("self.crane_runways.len()]", "self.crane_runways.len(),self.storeys.len()]")
    strs = ",".join(f"&v.{fd['name']}" for fd in FIELDS if fd["ty"] == "String")
    s = sub1(s, r"(for v in &self\.crane_runways\{admit_strings\(&mut b,&\[[^\]]*\],\d+\)\?;admit_forces\(&mut b,&mut rows,&v\.actions\)\?;\})", lambda m: m.group(1) + f"for v in &self.storeys{{admit_strings(&mut b,&[{strs}],32768)?;}}")
    s = sub1(s, r'(let crane_rows=entities\(d,"en1993_crane_runway",\d+,CRANE,c\)\?;)', lambda m: m.group(1) + f'let storey_rows=entities(d,"en1993_storey",{WIDTH},STOREY,c)?;')
    read = ",".join((f"{fd['name']}:text(v,{IDX[fd['name']]},c)?" if fd["ty"] == "String" else f"{fd['name']}:uint(v,{IDX[fd['name']]})?" if fd["ty"] == "u32" else f"{fd['name']}:v.real({IDX[fd['name']]})?") for fd in FIELDS)
    s = sub1(s, r"(  c\.checkpoint\(SqliteSnapshotPhase::ReconstructSnapshot,1,1\)\?;Ok\(Self\{[^\n]*?crane_runways)(\}\))", lambda m: f"  let mut storeys=Vec::new();for v in ordered(storey_rows.into_values(),1,c)?{{storeys.push(Storey{{{read}}});}}\n" + m.group(1) + ",storeys" + m.group(2))
    wr(p, s)


def ts():
    p = S / "🟦️.ts"
    s = rd(p)
    if "En1993SqliteStorey" in s:
        return
    tsty = lambda fd: "string" if fd["ty"] == "String" else "number" if fd["ty"] == "u32" else "Binary64"
    typ = "export type En1993SqliteStorey={" + ";".join(f"{camel(fd['name'])}:{tsty(fd)}" for fd in FIELDS) + "};\n"
    s = sub1(s, r"(export type En1993SqliteCrane=\{[^\n]*\n)", lambda m: m.group(1) + typ)
    s = s.replace("craneRunways:En1993SqliteCrane[]};", "craneRunways:En1993SqliteCrane[];storeys:En1993SqliteStorey[]};", 1)
    s = sub1(s, r"(const CRANE:readonly Ieee754Column\[\]=[^\n]*\n)", lambda m: m.group(1) + "const STOREY:readonly Ieee754Column[]=[" + ",".join(map(str, FLOATS)) + "].map(index=>({index,width:64}));\n")
    s = s.replace('"piles","craneRunways"]);for(const v of[', '"piles","craneRunways","storeys"]);for(const v of[', 1).replace("s.piles,s.craneRunways])collection(v)", "s.piles,s.craneRunways,s.storeys])collection(v)", 1)
    names = ",".join(f'"{camel(fd["name"])}"' for fd in FIELDS)
    cells = ",".join(f"v.{camel(fd['name'])}" if fd["ty"] != "u32" else f"uint(v.{camel(fd['name'])})" for fd in FIELDS)
    w = f' for(let n=0;n<s.storeys.length;n++){{const v=s.storeys[n]!;fields(v,[{names}]);await write(out,"en1993_storey",[1n,BigInt(n),{cells}],STOREY,o)}}\n'
    s = sub1(s, r'( for\(let n=0;n<s\.craneRunways\.length;n\+\+\)\{.*?await writeForces\(out,"en1993_crane_action",id,v\.actions,o\)\}\n)', lambda m: m.group(1) + w, re.S)
    s = sub1(s, r"(crane=await entities\(t\[23\]!,\d+,CRANE,o\);)", lambda m: m.group(1) + f"const storey=await entities(t[25]!,{WIDTH},STOREY,o);")
    s = s.replace("craneRunways:[]};", "craneRunways:[],storeys:[]};", 1)
    rdr = ",".join((f"{camel(fd['name'])}:artifactSqliteText(v,{IDX[fd['name']]})" if fd["ty"] == "String" else f"{camel(fd['name'])}:readUint(v,{IDX[fd['name']]})" if fd["ty"] == "u32" else f"{camel(fd['name'])}:readBinary64(v,{IDX[fd['name']]},STOREY)") for fd in FIELDS)
    r = f" for(const v of await ordered(storey.values(),1n,o))result.storeys.push({{{rdr}}});\n"
    s = sub1(s, r"( for\(const v of await ordered\(crane\.values\(\),1n,o\)\)result\.craneRunways\.push\(\{.*?\}\);\n)", lambda m: m.group(1) + r, re.S)
    wr(p, s)


def admission():
    p = S / "🛂️admission/🦀️.rs"
    s = rd(p)
    if "en1993_storey" in s:
        return
    cells = ",".join(f"Text({i})" if fd["ty"] == "String" else f"Unsigned({i},u32::MAX as u64)" if fd["ty"] == "u32" else f"Real({i})" for i, fd in enumerate(FIELDS) for _ in [0])
    line = f' for(index,v)in n::list(n::field(root,17)?)?.iter().enumerate(){{n::entity(out,"en1993_storey",1,index,&n::cells(n::record(v)?,[{cells}])?,STOREY)?;}}\n'
    s = sub1(s, r'(forces\(out,"en1993_crane_action",parent,v,7\)\?;\})(Ok\(\(\)\))', lambda m: m.group(1) + "\n" + line.rstrip("\n") + m.group(2))
    s = s.replace("for id in 1..=16{", "for id in 1..=17{", 1).replace("Twenty-five authored tables", "Twenty-six authored tables", 1).replace("En1993Snapshot::SQLITE_SCHEMA,25,57,", "En1993Snapshot::SQLITE_SCHEMA,26,57,", 1)
    wr(p, s)


def contract():
    p = S / "🧪️tests/🧫️fixtures/🎛️semantic.json"
    d = json.loads(rd(p))
    if "en1993_storey" not in d["tableWidths"]:
        d["tableWidths"]["en1993_storey"] = WIDTH + 1 + len(FLOATS) * 2
        d["tableWidths"] = dict(sorted(d["tableWidths"].items()))
        wr(p, json.dumps(d, indent=2, ensure_ascii=False) + "\n")
    p = S / "🧪️tests/🟦️.ts"
    s = rd(p)
    if "en1993_storey" not in s:
        s = s.replace('"en1993_silo_shell":22,', f'"en1993_silo_shell":22,"en1993_storey":{WIDTH + 1 + len(FLOATS) * 2},', 1).replace("craneRunways:[]};const d=await project(s,{maxRows:1})", "craneRunways:[],storeys:[]};const d=await project(s,{maxRows:1})", 1).replace("source.craneRunways=[];}", "source.craneRunways=[];source.storeys=[];}", 1).replace("maxTables:25", "maxTables:26", 1)
        wr(p, s)
    p = S / "🧪️tests/🦀️.rs"
    s = rd(p)
    if "source.storeys.clear()" not in s:
        s = s.replace("source.crane_runways.clear();}", "source.crane_runways.clear();source.storeys.clear();}", 1)
        wr(p, s)


sql()
rust()
ts()
admission()
contract()
print("storey io gen done")
