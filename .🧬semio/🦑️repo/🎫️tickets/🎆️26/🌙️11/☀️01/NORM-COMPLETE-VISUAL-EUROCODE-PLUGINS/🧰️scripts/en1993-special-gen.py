"""Appends the SPEC fields of en1993-special-spec.py to every site of the EN 1993 artifact (schema-first, idempotent per entity).
Usage: python3 -I en1993-special-gen.py [--check]. Sites: snapshot model + twins, diff patches + twins, SQLite/binary/text IO, field-meta, fixtures, assets, DSL texts."""
import json, pathlib, re, sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from schema_json import dump_schema

HERE = pathlib.Path(__file__).resolve().parent
SPEC_FILE = next((a for a in sys.argv[1:] if not a.startswith("--")), "en1993-special-spec.py")
exec(compile((HERE / SPEC_FILE).read_text(encoding="utf8"), "spec", "exec"))
TAG = globals().get("TAG", "SPECIAL")
A = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any")
CHECK = "--check" in sys.argv
SCHEMA = A / "🧬️schema"
camel = lambda s: re.sub(r"_([a-z0-9])", lambda m: m.group(1).upper(), s)
kebab = lambda s: s.replace("_", "-")
touched = set()


BUFFER = {}


def rd(p):
    return BUFFER[p] if p in BUFFER else p.read_text(encoding="utf8")


def wr(p, s):
    BUFFER[p] = s
    touched.add(str(p))


def flush():
    for p, s in BUFFER.items():
        if not CHECK and (not p.exists() or p.read_text(encoding="utf8") != s):
            p.write_text(s, encoding="utf8")


def rust_default(fd, key="default"):
    v = fd[key]
    if fd["ty"] == "String":
        return "String::new()" if v == "" else f'"{v}".into()'
    if fd["ty"] == "u32":
        return str(int(v))
    return repr(float(v))


def json_default(fd):
    v = fd["default"]
    return v if fd["ty"] == "String" else (int(v) if fd["ty"] == "u32" else float(v))


def is_float(fd):
    return fd["ty"] == "f64"


def sub1(s, pattern, repl, flags=0, count=1):
    out, n = re.subn(pattern, repl, s, count=count, flags=flags)
    assert n >= 1, f"no match: {pattern[:90]}"
    return out


def struct_done(text, ent):
    first = SPEC[ent]["fields"][0]["name"]
    m = re.search(r"pub struct %s \{(.*?)\n\}" % ent, text, re.S)
    return bool(m) and f"pub {first}:" in m.group(1)


def model_rust(p, ent):
    s = rd(p)
    if struct_done(s, ent):
        return
    add = "".join(f"    pub {fd['name']}: {fd['ty']},\n" for fd in SPEC[ent]["fields"])
    s = sub1(s, r"(pub struct %s \{.*?\n)(\})" % ent, lambda m: m.group(1) + add + m.group(2), re.S)
    wr(p, s)


def fix_literals(p, ents):
    s = rd(p)
    for ent in ents:
        fds = SPEC[ent]["fields"]
        pat = re.compile(r"((?<!struct )(?<!for )(?<!impl )(?<!model::)\b%s \{)(\n[ \t]*| )" % ent)
        def rep(m):
            after = s[m.end():m.end() + 400]
            if re.match(r"[^}]*?\b%s:" % fds[0]["name"], after):
                return m.group(0)
            sep = m.group(2)
            if sep.startswith("\n"):
                ind = sep[1:]
                return m.group(1) + "".join(f"\n{ind}{fd['name']}: {rust_default(fd)}," for fd in fds) + sep
            return m.group(1) + " " + ", ".join(f"{fd['name']}: {rust_default(fd)}" for fd in fds) + "," + sep
        if pat.search(s):
            s = pat.sub(rep, s)
    wr(p, s)


def ts_iface(p, ents):
    s = rd(p)
    for ent in ents:
        fds = SPEC[ent]["fields"]
        m = re.search(r"export interface %s \{(.*?)\n\}" % ent, s, re.S)
        if m and f"{camel(fds[0]['name'])}:" in m.group(1):
            continue
        tsty = lambda fd: "string" if fd["ty"] == "String" else "number"
        if m:
            add = "".join(f"\n  {camel(fd['name'])}: {tsty(fd)};" for fd in fds)
            s = s[:m.end(1)] + add + s[m.end(1):]
            pm = re.search(r"(export const parse%s: NormWireReader<%s> = normWireObject<%s>\(\{)(.*?)(\}\);)" % (ent, ent, ent), s, re.S)
            padd = "".join(f", {camel(fd['name'])}: normWireRequired({'normWireString' if fd['ty'] == 'String' else 'normWireNumber'})" for fd in fds)
            cut = pm.start(2) + len(pm.group(2).rstrip())
            s = s[:cut] + padd + s[cut:]
    wr(p, s)


def snapshot_twins():
    for sub in (SCHEMA, SCHEMA / "📸️snapshot"):
        p = sub / "🟦️.ts"
        ts_iface(p, SPEC)
        p = sub / "🛰️.proto"
        s = rd(p)
        for ent, d in SPEC.items():
            m = re.search(r"message %s \{(.*?)\}" % ent, s)
            if f" {d['fields'][0]['name']} =" in m.group(1):
                continue
            n = max(int(x) for x in re.findall(r"= (\d+);", m.group(1)))
            add = ""
            for fd in d["fields"]:
                n += 1
                pt = {"f64": "double", "u32": "uint32", "String": "string"}[fd["ty"]]
                add += f" {pt} {fd['name']} = {n};"
            s = s[:m.end(1)].rstrip() + add + " " + s[m.end(1):]
        wr(p, s)
        for gql in {sub / "🔗️.graphql", A / "🚪️io/📝️text/📸️snapshot/🔗️.graphql"}:
            s = rd(gql)
            for ent, d in SPEC.items():
                m = re.search(r"type %s \{(.*?)\}" % ent, s)
                if f" {camel(d['fields'][0]['name'])}:" in m.group(1):
                    continue
                add = "".join(f" {camel(fd['name'])}: {'String' if fd['ty'] == 'String' else 'Float' if fd['ty'] == 'f64' else 'Int'}!" for fd in d["fields"])
                s = s[:m.end(1)].rstrip() + add + " " + s[m.end(1):]
            wr(gql, s)
        wrap_json_schema(sub / "🔣️.json")
    s = rd(A / "🚪️io/💾️binary/📸️snapshot/🟦️.ts")
    for ent, d in SPEC.items():
        m = re.search(r"export interface %s \{(.*?)\}" % ent, s)
        if f" {camel(d['fields'][0]['name'])}:" in m.group(1):
            continue
        add = "".join(f" {camel(fd['name'])}: {'string' if fd['ty'] == 'String' else 'number'};" for fd in d["fields"])
        s = s[:m.end(1)].rstrip() + add + " " + s[m.end(1):]
    wr(A / "🚪️io/💾️binary/📸️snapshot/🟦️.ts", s)


def ui_for(fd):
    u = U[fd["unit"]]
    ui = {"label": {"en": fd["en"], "de": fd["de"]}}
    if fd["ty"] == "String":
        return ui
    ui["widget"] = "stepper"
    if fd["ty"] == "u32":
        ui.update({"step": 1, "precision": 0})
        return ui
    if u[1]:
        ui["unit"] = u[1]
    if u[2]:
        ui["displayUnit"] = u[2]
        ui["displayFactor"] = u[3]
    ui["precision"] = u[4]
    ui["step"] = u[5]
    return ui


def schema_prop(fd):
    ui = ui_for(fd)
    if fd["ty"] == "String":
        return {"type": "string", "x-semio-ui": ui}
    if fd["ty"] == "u32":
        return {"type": "integer", "minimum": 0, "x-semio-ui": ui, "maximum": 4294967295}
    return {"type": "number", "x-semio-ui": ui}


def wrap_json_schema(p):
    d = json.loads(rd(p))
    defs = d["$defs"]
    for ent, spec in SPEC.items():
        node = defs[ent]
        if camel(spec["fields"][0]["name"]) in node["properties"]:
            continue
        for fd in spec["fields"]:
            node["required"].append(camel(fd["name"]))
            node["properties"][camel(fd["name"])] = schema_prop(fd)
    wr(p, dump_schema(d))


def diff_twins():
    D = SCHEMA / "🔺️diff"
    s = rd(D / "🦀️.rs")
    for ent, spec in SPEC.items():
        pm = re.search(r"pub %s of semio_s_artifact_norm_en1993_model::%s \{ set \{(.*?)\} \}" % (spec["patch"], ent), s)
        if f" {spec['fields'][0]['name']}:" in pm.group(1):
            continue
        add = "".join(f", {fd['name']}: {fd['ty']}" for fd in spec["fields"])
        s = s[:pm.end(1)] + add + s[pm.end(1):]
    wr(D / "🦀️.rs", s)
    s = rd(D / "🔗️.graphql")
    for ent, spec in SPEC.items():
        m = re.search(r"type %s \{\n(.*?)\n\}" % spec["patch"], s, re.S)
        if f"  {camel(spec['fields'][0]['name'])}:" in m.group(1):
            continue
        add = "".join(f"\n  {camel(fd['name'])}: {'String' if fd['ty'] == 'String' else 'Float' if fd['ty'] == 'f64' else 'Int'}" for fd in spec["fields"])
        s = s[:m.end(1)] + add + s[m.end(1):]
    wr(D / "🔗️.graphql", s)
    s = rd(D / "🛰️.proto")
    for ent, spec in SPEC.items():
        m = re.search(r"message %s \{\n(.*?)\n\}" % spec["patch"], s, re.S)
        if f" {spec['fields'][0]['name']} =" in m.group(1):
            continue
        n = max(int(x) for x in re.findall(r"= (\d+);", m.group(1)))
        add = ""
        for fd in spec["fields"]:
            n += 1
            add += f"\n  optional {({'f64': 'double', 'u32': 'uint32', 'String': 'string'})[fd['ty']]} {fd['name']} = {n};"
        s = s[:m.end(1)] + add + s[m.end(1):]
    wr(D / "🛰️.proto", s)
    s = rd(D / "🟦️.ts")
    for ent, spec in SPEC.items():
        pn = spec["patch"]
        m = re.search(r"export interface %s \{(.*?)\}" % pn, s)
        if f" {camel(spec['fields'][0]['name'])}:" in m.group(1):
            continue
        t = lambda fd: "string" if fd["ty"] == "String" else "number"
        add = "".join(f" {camel(fd['name'])}: ({t(fd)}) | null;" for fd in spec["fields"])
        s = s[:m.end(1)].rstrip() + add + " " + s[m.end(1):]
        pm = re.search(r"(export const parse%s: NormWireReader<%s> = normWireObject<%s>\(\{)(.*?)( \}\);)" % (pn, pn, pn), s, re.S)
        padd = "".join(f", {camel(fd['name'])}: normWireDefault(normWireNullable({'normWireString' if fd['ty'] == 'String' else 'normWireNumber'}), () => null)" for fd in spec["fields"])
        cut = pm.start(2) + len(pm.group(2).rstrip())
        s = s[:cut] + padd + s[cut:]
    wr(D / "🟦️.ts", s)
    p = D / "🔣️.json"
    d = json.loads(rd(p))
    for ent, spec in SPEC.items():
        node = d["$defs"][spec["patch"]]["properties"]
        if camel(spec["fields"][0]["name"]) in node:
            continue
        for fd in spec["fields"]:
            ty = {"f64": "number", "u32": "integer", "String": "string"}[fd["ty"]]
            node[camel(fd["name"])] = {"anyOf": [{"type": ty}, {"type": "null"}]}
    wr(p, dump_schema(d))


SQL_TABLE = {"SteelMaterial": "material", "SteelMember": "member", "ColdFormedMember": "cold_formed_member", "PlatedPanel": "plated_panel", "SiloShell": "silo_shell", "SteelJoint": "joint", "TensionComponent": "tension_component", "BridgeFatigue": "bridge_fatigue", "TowerLeg": "tower_leg", "SteelPile": "pile", "CraneRunway": "crane"}
SQL_TABLE["CraneRunway"] = "crane_runway"
SQL_CONST = {"SteelMaterial": "MATERIAL", "SteelMember": "MEMBER", "ColdFormedMember": "COLD", "PlatedPanel": "PLATE", "SiloShell": "SILO", "SteelJoint": "JOINT", "TensionComponent": "TENSION", "BridgeFatigue": "BRIDGE", "TowerLeg": "TOWER", "SteelPile": "PILE", "CraneRunway": "CRANE"}
TS_TYPE = {"SteelMaterial": "Material", "SteelMember": "Member", "ColdFormedMember": "Cold", "PlatedPanel": "Plate", "SiloShell": "Silo", "SteelJoint": "Joint", "TensionComponent": "Tension", "BridgeFatigue": "Bridge", "TowerLeg": "Tower", "SteelPile": "Pile", "CraneRunway": "Crane"}
VAR = {"SteelMaterial": "materials", "SteelMember": "members", "ColdFormedMember": "cold_formed_members", "PlatedPanel": "plated_panels", "SiloShell": "silo_shells", "SteelJoint": "joints", "TensionComponent": "tension_components", "BridgeFatigue": "bridge_fatigue", "TowerLeg": "tower_legs", "SteelPile": "piles", "CraneRunway": "crane_runways"}
SQL_TABLE["BuiltUpMember"] = "built_up_member"
SQL_CONST["BuiltUpMember"] = "BUILT_UP_MEMBER"
TS_TYPE["BuiltUpMember"] = "BuiltUp"
VAR["BuiltUpMember"] = "built_up_members"
STR_BOUND = {}


def sqlite_io():
    S = A / "🚪️io/🪶️sqlite/📸️snapshot"
    sql = rd(S / "🗄️.sql")
    rs = rd(S / "🦀️.rs")
    ts = rd(S / "🟦️.ts")
    tst = rd(S / "🧪️tests/🦀️.rs")
    n_new_floats = 0
    word_tested = set()
    for ent, spec in SPEC.items():
        if spec.get("sqlite") is False:
            continue
        fds = spec["fields"]
        tab = "en1993_" + SQL_TABLE[ent]
        m = re.search(r"CREATE TABLE %s\(\n(.*?)\n(.*?)\n\);" % tab, sql, re.S)
        if re.search(r"\b%s\b" % fds[0]["name"], m.group(1)):
            continue
        k0 = int(re.search(r'entities\(d,"%s",(\d+),%s,c\)' % (tab, SQL_CONST[ent]), rs).group(1))
        cols = ""
        ieee = ""
        for fd in fds:
            n = fd["name"]
            if fd["ty"] == "f64":
                cols += f",{n} REAL"
                ieee += f",{n}_ieee754_bits INTEGER NOT NULL,{n}_ieee754_class TEXT NOT NULL"
            elif fd["ty"] == "u32":
                cols += f",{n} INTEGER NOT NULL CHECK({n}>=0 AND {n}<=4294967295)"
            else:
                cols += f",{n} TEXT NOT NULL"
        l1 = m.group(1).rstrip()
        assert l1.endswith(",")
        l1 = l1[:-1] + cols + ","
        sql = sql[:m.start(1)] + l1 + "\n" + m.group(2) + ieee + sql[m.end(2):]
        const = SQL_CONST[ent]
        cells_rs = ""
        cells_ts = ""
        read_rs = ""
        read_ts = ""
        names = ""
        floats = []
        for i, fd in enumerate(fds):
            col = k0 + i
            n, c = fd["name"], camel(fd["name"])
            if fd["ty"] == "f64":
                cells_rs += f",Cell::Real(v.{n})"
                cells_ts += f",v.{c}"
                read_rs += f"{n}:v.real({col})?,"
                read_ts += f"{c}:readBinary64(v,{col},{const}),"
                floats.append(col)
            elif fd["ty"] == "u32":
                cells_rs += f",Cell::Integer(i64::from(v.{n}))"
                cells_ts += f",uint(v.{c})"
                read_rs += f"{n}:uint(v,{col})?,"
                read_ts += f"{c}:readUint(v,{col}),"
            else:
                cells_rs += f",Cell::Text(&v.{n})"
                cells_ts += f",v.{c}"
                read_rs += f"{n}:text(v,{col},c)?,"
                read_ts += f"{c}:artifactSqliteText(v,{col}),"
            names += f',"{c}"'
        rs = sub1(rs, r"(const %s:&\[FloatColumn\]=&\[.*?)(\];)" % const, lambda m: m.group(1) + "".join(f",FloatColumn::Binary64({x})" for x in floats) + m.group(2))
        rs = sub1(rs, r"(insert_float\(\"%s\",&\[.*?)(\],%s\)\?)" % (tab, const), lambda m: m.group(1) + cells_rs + m.group(2))
        rs = sub1(rs, r'(entities\(d,"%s",)(\d+)(,%s,c\))' % (tab, const), lambda m: f"{m.group(1)}{k0 + len(fds)}{m.group(3)}")
        rs = sub1(rs, r"(%s\.push\(%s\{)" % (VAR[ent], ent), lambda m: m.group(1) + read_rs)
        strs = "".join(f",&v.{fd['name']}" for fd in fds if fd["ty"] == "String")
        if strs:
            rs = sub1(rs, r"(for v in &self\.%s\{admit_strings\(&mut b,&\[)(.*?)(\],\d+\)\?)" % VAR[ent], lambda m: m.group(1) + m.group(2) + strs + m.group(3))
        tt = "En1993Sqlite" + TS_TYPE[ent]
        ts = sub1(ts, r"(export type %s=\{.*?)(\};)" % tt, lambda m: m.group(1) + "".join(f";{camel(fd['name'])}:{'string' if fd['ty'] == 'String' else 'number' if fd['ty'] == 'u32' else 'Binary64'}" for fd in fds) + m.group(2))
        ts = sub1(ts, r"(const %s:readonly Ieee754Column\[\]=\[[0-9,]*)(\])" % const, lambda m: m.group(1) + "".join(f",{x}" for x in floats) + m.group(2))
        ts = sub1(ts, r'(const v=s\.%s\[n\]!;fields\(v,\[[^\]]*)(\]\))' % camel(VAR[ent]), lambda m: m.group(1) + names + m.group(2))
        ts = sub1(ts, r'(await write\(out,"%s",\[[^\]]*)(\],%s,o\))' % (tab, const), lambda m: m.group(1) + cells_ts + m.group(2))
        ts = sub1(ts, r"(entities\(t\[\d+\]!,)(\d+)(,%s,o\))" % const, lambda m: f"{m.group(1)}{k0 + len(fds)}{m.group(3)}")
        ts = sub1(ts, r"(result\.%s\.push\(\{)" % camel(VAR[ent]), lambda m: m.group(1) + read_ts)
        fl = "".join(f"v.{fd['name']}=w;" for fd in fds if fd["ty"] == "f64")
        if re.search(r"for v in &mut s\.%s\{" % VAR[ent], tst):
            tst = sub1(tst, r"(for v in &mut s\.%s\{)" % VAR[ent], lambda m: m.group(1) + fl)
            word_tested.add(ent)
            n_new_floats += len(floats)
    old = int(re.search(r"count!==(\d+)", tst).group(1)) if n_new_floats else 0
    if n_new_floats:
        fx = json.loads(rd(S / "🧪️tests/🧫️fixtures/🔣️.json"))["snapshot"]
        extra = sum(len(fx.get(camel(VAR[e]), [])) * sum(1 for fd in SPEC[e]["fields"] if fd["ty"] == "f64") for e in SPEC if e in word_tested)
        new = old + extra
        tst = tst.replace(f"count!=={old}", f"count!=={new}")
        tst = re.sub(r"all_\w+_fields", f"all_{new}_fields", tst)
        sql_test = tst
    wr(S / "🗄️.sql", sql)
    wr(S / "🦀️.rs", rs)
    wr(S / "🟦️.ts", ts)
    wr(S / "🧪️tests/🦀️.rs", tst)


def field_meta():
    p = A / "✏️editor/🏷️field-meta/🦀️.rs"
    s = rd(p)
    first = next(iter(SPEC.values()))
    if f'"{first["collection"]}[].{camel(first["fields"][0]["name"])}"' in s:
        return
    consts = ""
    for name, rows in CH.items():
        consts += f"const {name}: &[NormFieldChoice] = &[\n" + "".join(f'    choice("{v}", "{en}", "{de}"),\n' for v, en, de in rows) + "];\n"
    s = sub1(s, r"(const JOINT_KIND: &\[NormFieldChoice\] = &\[\n.*?)(\];\n)", lambda m: m.group(1) + '    choice("endPlate", "End plate", "Stirnplatte"),\n    choice("basePlate", "Base plate", "Fußplatte"),\n' + m.group(2), re.S)
    s = s.replace("const ANNEX:", consts + "\nconst ANNEX:", 1)
    rows = ""
    for ent, spec in SPEC.items():
        for fd in spec["fields"]:
            path = f"{spec['collection']}[].{camel(fd['name'])}"
            if fd["choices"]:
                rows += f'    ("{path}", FieldSpec::choice("{fd["en"]}", "{fd["de"]}", {fd["choices"]})),\n'
            elif U[fd["unit"]][0]:
                rows += f'    ("{path}", FieldSpec::quantity("{fd["en"]}", "{fd["de"]}", UnitId::{U[fd["unit"]][0]})),\n'
            else:
                rows += f'    ("{path}", FieldSpec::new("{fd["en"]}", "{fd["de"]}")),\n'
    s = sub1(s, r"(\n\];\n\n/// 🏷️ Exact)", lambda m: "\n" + rows.rstrip("\n") + m.group(1))
    wr(p, s)


def mutation_tests():
    p = SCHEMA / "🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"
    fix_literals(p, SPEC)


def snapshot_model():
    p = SCHEMA / "📸️snapshot/🦀️.rs"
    for ent in SPEC:
        model_rust(p, ent)
    fix_literals(p, SPEC)


def walk(node, fn):
    if isinstance(node, dict):
        fn(node)
        for v in node.values():
            walk(v, fn)
    elif isinstance(node, list):
        for v in node:
            walk(v, fn)


def old_keys():
    out = {}
    for ent, spec in SPEC.items():
        out[ent] = [camel(fd["name"]) for fd in spec["fields"]]
    return out


def json_fixtures():
    sigs = {}
    base = json.loads(rd(SCHEMA / "📸️snapshot/🔣️.json"))["$defs"]
    for ent, spec in SPEC.items():
        new = [camel(fd["name"]) for fd in spec["fields"]]
        allk = list(base[ent]["properties"].keys())
        old = [k for k in allk if k not in new]
        sigs[ent] = (tuple(old), spec["fields"])
    by_old = {v[0]: (k, v[1]) for k, v in sigs.items()}
    n_files = 0
    for p in A.rglob("*.json"):
        if "🧬️schema" in p.parts and p.name == "🔣️.json" and "🧫️fixtures" not in p.parts:
            continue
        try:
            data = json.loads(rd(p))
        except Exception:
            continue
        changed = [False]

        def fn(d):
            key = tuple(d.keys())
            hit = by_old.get(key)
            if hit:
                for fd in hit[1]:
                    d[camel(fd["name"])] = json_default(fd)
                changed[0] = True
        walk(data, fn)
        if changed[0]:
            raw = rd(p)
            wr(p, json.dumps(data, indent=2, ensure_ascii=False) + ("\n" if raw.endswith("\n") else ""))
            n_files += 1
    print("json files patched:", n_files)


def dsl_texts():
    for p in A.rglob("🗣️.dsl.semio"):
        s = rd(p)
        for ent, spec in SPEC.items():
            coll = kebab(re.sub(r"([A-Z])", lambda m: "-" + m.group(1).lower(), spec["collection"]).lstrip("-"))
            m = re.search(r"^%s \[(.*?)\] \{\n(.*?)\n\}" % re.escape(coll), s, re.S | re.M)
            if not m or kebab(spec["fields"][0]["name"]) + ":" in m.group(1):
                continue
            hdr = m.group(1) + "".join(f" {kebab(fd['name'])}:{ {'f64': 'NUM', 'u32': 'UINT', 'String': 'TEXT'}[fd['ty']] }" for fd in spec["fields"])
            vals = ""
            for fd in spec["fields"]:
                v = json_default(fd)
                vals += " " + (('""' if v == "" else v) if fd["ty"] == "String" else (str(v) if fd["ty"] == "u32" else repr(float(v))))
            rows = "\n".join(r + vals for r in m.group(2).split("\n"))
            s = s[:m.start()] + f"{coll} [{hdr}] {{\n{rows}\n}}" + s[m.end():]
        wr(p, s)


exec(compile((HERE / "en1993-special-showcase.py").read_text(encoding="utf8"), "showcase", "exec"))


def main():
    snapshot_model()
    showcase()
    snapshot_twins()
    diff_twins()
    sqlite_io()
    field_meta()
    mutation_tests()
    leaf_test()
    json_fixtures()
    dsl_texts()
    flush()
    print(("check: would touch " if CHECK else "touched ") + str(len(touched)) + " files")


main()
