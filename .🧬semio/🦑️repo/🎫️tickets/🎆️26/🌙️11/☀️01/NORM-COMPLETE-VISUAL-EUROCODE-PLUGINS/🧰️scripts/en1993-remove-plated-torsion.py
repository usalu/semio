"""Removes the entered PlatedPanel fields `transverse_stiffener_torsion` and `transverse_stiffener_polar` (wave 2 v7: I_t and I_p come from the stiffener outline) from every layer of the EN 1993 artifact: model, twins, diff, SQLite, field-meta, mutation kinds, harnesses, oracle manifest, fixtures, assets and DSL texts. No migration: everything is hand-removed at once. Idempotent. Usage: python3 -I en1993-remove-plated-torsion.py [--check]"""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import json, pathlib, re, shutil, sys

HERE = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
from schema_json import dump_schema

A = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any")
CHECK = "--check" in sys.argv
camel = lambda s: re.sub(r"_([a-z0-9])", lambda m: m.group(1).upper(), s)
FIELDS = ["transverse_stiffener_torsion", "transverse_stiffener_polar"]
KINDS = ["change-plated-trans-stiff-torsion", "change-plated-trans-stiff-polar"]
SQL_INDEX = {"transverse_stiffener_torsion": 33, "transverse_stiffener_polar": 34}
CAMEL = [camel(f) for f in FIELDS]
PASCAL = ["ChangePlatedTransStiffTorsion", "ChangePlatedTransStiffPolar"]
KIND_SNAKE = [k.replace("-", "_") for k in KINDS]
SKIP_DIRS = {"target", "node_modules", "__pycache__"}
SQLITE = A / "🚪️io/🪶️sqlite/📸️snapshot"
touched = {}
deleted = []


def rd(p):
    return touched[p] if p in touched else p.read_text(encoding="utf8", errors="surrogateescape")


def wr(p, s):
    if s != rd(p):
        touched[p] = s


def files(suffixes):
    for p in A.rglob("*"):
        if p.is_file() and p.suffix in suffixes and not (set(p.parts) & SKIP_DIRS) and "🧫️fixtures" not in p.parts:
            yield p


def kind_lines(s):
    keys = KINDS + PASCAL + KIND_SNAKE
    return "".join(l for l in s.splitlines(True) if not any(k in l for k in keys))


def sqlite_plate():
    drop = sorted(SQL_INDEX.values())

    def remap(k):
        return k - sum(1 for d in drop if d < k)

    p = SQLITE / "🗄️.sql"
    s = rd(p)
    for f in FIELDS:
        s = s.replace(f",{f} REAL", "").replace(f",{f}_ieee754_bits INTEGER NOT NULL,{f}_ieee754_class TEXT NOT NULL", "")
    wr(p, s)
    p = SQLITE / "🦀️.rs"
    out = []
    for line in rd(p).splitlines(True):
        if "en1993_plated_panel\",&[" in line and "insert_float" in line:
            for f in FIELDS:
                line = line.replace(f",Cell::Real(v.{f})", "")
        if "plated_panels.push(PlatedPanel{" in line:
            for f in FIELDS:
                line = re.sub(r"%s:v\.real\(\d+\)\?," % f, "", line)
            line = re.sub(r"(v\.real\(|uint\(v,|text\(v,)(\d+)", lambda m: m.group(1) + str(remap(int(m.group(2)))), line)
        if line.startswith("const PLATE:"):
            line = re.sub(r"FloatColumn::Binary64\((\d+)\)", lambda m: "" if int(m.group(1)) in drop else "FloatColumn::Binary64(%d)" % remap(int(m.group(1))), line)
            line = re.sub(r",{2,}", ",", line).replace("[,", "[").replace(",]", "]")
        line = re.sub(r'(entities\(d,"en1993_plated_panel",)(\d+)', lambda m: m.group(1) + str(int(m.group(2)) - len(drop)), line)
        out.append(line)
    wr(p, "".join(out))
    p = SQLITE / "🟦️.ts"
    out = []
    for line in rd(p).splitlines(True):
        if "En1993SqlitePlate=" in line or "s.platedPanels" in line or "result.platedPanels.push" in line or line.startswith("const PLATE:"):
            for c, f in zip(CAMEL, FIELDS):
                line = line.replace(f";{c}:Binary64", "").replace(f',"{c}"', "").replace(f",v.{c}", "")
                line = re.sub(r"%s:readBinary64\(v,\d+,PLATE\)," % c, "", line)
        if "result.platedPanels.push" in line:
            line = re.sub(r"(readBinary64\(v,|readUint\(v,|artifactSqliteText\(v,)(\d+)", lambda m: m.group(1) + str(remap(int(m.group(2)))), line)
        if line.startswith("const PLATE:"):
            nums = [int(x) for x in re.search(r"=\[([0-9,]+)\]", line).group(1).split(",")]
            nums = [remap(n) for n in nums if n not in drop]
            line = re.sub(r"=\[[0-9,]+\]", "=[" + ",".join(map(str, nums)) + "]", line, count=1)
        line = re.sub(r'(entities\(t\[\d+\]!,)(\d+)(,PLATE,o\))', lambda m: m.group(1) + str(int(m.group(2)) - len(drop)) + m.group(3), line)
        out.append(line)
    wr(p, "".join(out))
    p = SQLITE / "🧪️tests/🦀️.rs"
    s = rd(p)
    for f in FIELDS:
        s = s.replace(f"v.{f}=w;", "")
    wr(p, s)


def sqlite_float_count():
    p = SQLITE / "🧪️tests/🦀️.rs"
    s = rd(p)
    fx = json.loads((SQLITE / "🧪️tests/🧫️fixtures/🔣️.json").read_text(encoding="utf8"))["snapshot"]
    rows = len(fx.get("platedPanels", []))
    old = int(re.search(r"count!==(\d+)", s).group(1))
    new = old - rows * len(FIELDS)
    s = s.replace(f"count!=={old}", f"count!=={new}")
    s = re.sub(r"all_\w+_fields", f"all_{new}_fields", s)
    wr(p, s)


def text_rules(p):
    s = rd(p)
    o = s
    s = kind_lines(s)
    for f, c in zip(FIELDS, CAMEL):
        k = f.replace("_", "-")
        s = re.sub(r"^[ \t]*pub %s: f64,[ \t]*\n" % f, "", s, flags=re.M)
        s = re.sub(r"^[ \t]*%s: [^\n]*,[ \t]*\n" % f, "", s, flags=re.M)
        s = re.sub(r"^[ \t]*[a-z_]+\.%s = [^;\n]+;[ \t]*\n" % f, "", s, flags=re.M)
        s = re.sub(r"^[ \t]*(optional )?double %s = \d+;[ \t]*\n" % f, "", s, flags=re.M)
        s = re.sub(r"^[ \t]*%s: (Float!?|number;|\(number\) \| null;)[ \t]*\n" % c, "", s, flags=re.M)
        s = re.sub(r"[ \t]*\b%s: f64,?" % f, "", s)
        s = re.sub(r" ?\bdouble %s = \d+;" % f, "", s)
        s = re.sub(r" ?\b%s: Float!?(?=[ \n}])" % c, "", s)
        s = re.sub(r" ?\b%s: (number|\(number\) \| null);" % c, "", s)
        s = re.sub(r", %s: normWireRequired\(normWireNumber\)" % c, "", s)
        s = re.sub(r", %s: normWireDefault\(normWireNullable\(normWireNumber\), \(\) => null\)" % c, "", s)
        s = re.sub(r"^[ \t]*\(\"platedPanels\", \"[a-z0-9-]+\", \"%s\", 'f'\),[ \t]*\n" % c, "", s, flags=re.M)
        s = re.sub(r"^[ \t]*\(\"platedPanels\[\]\.%s\", [^\n]*\n" % c, "", s, flags=re.M)
    if s != o:
        wr(p, s)


def json_text_remove(s):
    for c in CAMEL:
        s = re.sub(r'(\n[ \t]*)"%s"[ \t]*:[ \t]*(?:[-+0-9.eE]+|null),(?=\s)' % c, "", s)
        s = re.sub(r'"%s"[ \t]*:[ \t]*(?:[-+0-9.eE]+|null),[ \t]*' % c, "", s)
        s = re.sub(r',[ \t]*(\n[ \t]*)?"%s"[ \t]*:[ \t]*(?:[-+0-9.eE]+|null)' % c, "", s)
    return s


def schema_json(p):
    d = json.loads(rd(p))
    defs = d["$defs"]
    for name in ("PlatedPanel", "En1993PlatedPanelPatch"):
        node = defs.get(name)
        if not node:
            continue
        for c in CAMEL:
            node["properties"].pop(c, None)
            if c in node.get("required", []):
                node["required"].remove(c)
    wr(p, dump_schema(d))


def dsl_texts():
    for p in A.rglob("🗣️.dsl.semio"):
        s = rd(p)
        m = re.search(r"^plated-panels \[(.*?)\] \{\n(.*?)\n\}", s, re.S | re.M)
        if not m or "transverse-stiffener-torsion" not in m.group(1):
            continue
        cols = re.findall(r"[a-z0-9-]+:[A-Z]+", m.group(1))
        drop = {i for i, c in enumerate(cols) if c.split(":")[0] in ("transverse-stiffener-torsion", "transverse-stiffener-polar")}
        hdr = " ".join(c for i, c in enumerate(cols) if i not in drop)

        def tokens(row):
            out, i, depth, cur, quote = [], 0, 0, "", False
            for ch in row.strip():
                if ch == '"':
                    quote = not quote
                if not quote and ch in "[{":
                    depth += 1
                if not quote and ch in "]}":
                    depth -= 1
                if ch == " " and depth == 0 and not quote:
                    out.append(cur)
                    cur = ""
                else:
                    cur += ch
            out.append(cur)
            return out
        rows = []
        for r in m.group(2).split("\n"):
            t = tokens(r)
            assert len(t) == len(cols), (p, len(t), len(cols))
            rows.append("  " + " ".join(x for i, x in enumerate(t) if i not in drop))
        wr(p, s[:m.start()] + f"plated-panels [{hdr}] {{\n" + "\n".join(rows) + "\n}" + s[m.end():])


def oracle_json():
    p = A / "🔮️oracles/🔣️.json"
    raw = rd(p)
    d = json.loads(raw)
    drop = set(KINDS)

    def clean(n):
        if isinstance(n, dict):
            return {k: clean(v) for k, v in n.items()}
        if isinstance(n, list):
            return [clean(x) for x in n if not (isinstance(x, str) and x in drop) and not (isinstance(x, dict) and (x.get("mutationId") in drop or x.get("id") in drop))]
        return n
    out = json.dumps(clean(d), indent=2, ensure_ascii=False) + ("\n" if raw.endswith("\n") else "")
    assert json.dumps(d, indent=2, ensure_ascii=False) + ("\n" if raw.endswith("\n") else "") == raw, "oracle manifest does not round-trip"
    wr(p, out)


def protocol():
    p = A / "🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
    s = rd(p)
    for k in KINDS:
        s = re.sub(r"record %s tag=\d+\nfield payload bytes\n" % re.escape(k), "", s)
    wr(p, s)


def fixtures():
    n = 0
    schemas = {A / "🧬️schema/🔣️.json", A / "🧬️schema/📸️snapshot/🔣️.json", A / "🧬️schema/🔺️diff/🔣️.json", A / "🧬️schema/🧬️mutations/🔣️.json", A / "🔮️oracles/🔣️.json"}
    for base in (A,):
        for p in base.rglob("*.json"):
            if set(p.parts) & SKIP_DIRS or p in schemas or any(k in str(p) for k in KINDS) or ("🧬️schema" in p.parts and "🧫️fixtures" not in p.parts):
                continue
            try:
                raw = rd(p)
            except Exception:
                continue
            if not any(c in raw for c in CAMEL):
                continue
            new = json_text_remove(raw)
            json.loads(new)
            wr(p, new)
            n += 1
    print("fixture json files:", n)
    for k in KINDS:
        for d in (A / "🧪️tests/🧫️fixtures/🧬️mutations").glob("*" + k):
            deleted.append(d)


def leaf_dirs():
    for k in KINDS:
        d = A / "🧬️schema/🧬️mutations" / ("🧱️" + k)
        if d.exists():
            deleted.append(d)


def main():
    if FIELDS[0] in rd(SQLITE / "🗄️.sql"):
        sqlite_plate()
        sqlite_float_count()
    for p in files({".rs", ".ts", ".proto", ".graphql", ".feature", ".py", ".semio"}):
        if p == SQLITE / "🦀️.rs" or p == SQLITE / "🟦️.ts" or p == SQLITE / "🧪️tests/🦀️.rs" or p.name == "🗣️.dsl.semio" or p.name == "📡️.protocol.semio":
            continue
        if "🧱️change-plated-trans-stiff-" in str(p):
            continue
        if any(c in rd(p) for c in CAMEL + FIELDS + KINDS + PASCAL + KIND_SNAKE):
            text_rules(p)
    for p in (A / "🧬️schema/🔣️.json", A / "🧬️schema/📸️snapshot/🔣️.json", A / "🧬️schema/🔺️diff/🔣️.json"):
        schema_json(p)
    p = A / "🧬️schema/🧬️mutations/🔣️.json"
    wr(p, kind_lines(rd(p)))
    dsl_texts()
    oracle_json()
    protocol()
    fixtures()
    leaf_dirs()
    if not CHECK:
        for p, s in touched.items():
            p.write_text(s, encoding="utf8", errors="surrogateescape")
        for d in deleted:
            shutil.rmtree(d, ignore_errors=True)
    print(("check: would touch " if CHECK else "touched ") + str(len(touched)) + " files, deleted " + str(len(deleted)) + " dirs")


main()
