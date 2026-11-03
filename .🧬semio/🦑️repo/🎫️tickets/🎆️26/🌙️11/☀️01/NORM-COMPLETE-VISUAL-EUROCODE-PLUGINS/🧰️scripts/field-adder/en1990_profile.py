"""EN 1990 profile of the field-adder: adds `f64` fields to the `members` and `bridgeSls` rows through the schema (snapshot, artifact and diff JSON; every twin and the model struct field are generated), the diff macro (rs), SQLite (sql, rs, ts, tests, fixture), field meta, struct literals and every JSON fixture. Idempotent. Usage: python3 -I field_adder.py specs/en1990-wave3.json [--dry]. The contract numbers (`🎛️semantic.json`) are recomputed by `en1990_contract.py`."""
import json, os, re

import field_adder as fa
import schema_twins

ANY = fa.NORM + "/⚖️en1990" + fa.SUBSET
ENT = {
    "bridgeSls": dict(model="BridgeSls", patch="En1990BridgeSlsPatch", table="en1990_bridge_sls", floats="BRIDGE_FLOATS", rs_var="bridge", ts_var="bridge", ts_type="En1990SqliteBridgeSls", ts_test="b", prefix="bridgeSls[]", ts_snap="BridgeSls", literal_files=["/📚️examples/🏢️road-bridge-compliant/🦀️.rs", "/📚️examples/🏢️road-bridge-failing/🦀️.rs", "/🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs"]),
    "members": dict(model="Member", patch="En1990MemberPatch", table="en1990_member", floats="MEMBER_FLOATS", rs_var="member", ts_var="member", ts_type="En1990SqliteMember", ts_test="m", prefix="members[]", ts_snap="Member", literal_files=["/📚️examples/🏢️%s/🦀️.rs" % n for n in ("accidental-seismic-failing", "accidental-seismic-compliant", "fatigue-compliant", "road-bridge-failing", "road-bridge-compliant", "high-consequence-office", "fatigue-failing")]),
}
P = lambda rel: ANY + rel
once, edit = fa.once, fa.edit
SNAP = "/🧬️schema/📸️snapshot"
SCHEMA = "/🧬️schema"
DIFF = "/🧬️schema/🔺️diff"
SQ = "/🚪️io/🪶️sqlite/📸️snapshot"


class F:
    def __init__(self, spec):
        self.e = ENT[spec["entity"]]
        self.name = spec["field"]
        self.cam = fa.camel(self.name)
        self.after = spec["after"]
        self.after_cam = fa.camel(self.after)
        self.spec = spec
        self.default = repr(float(spec["default"]))


def literals(f):
    multi = re.compile(r"^(\s*)(%s: [^,}\n]+),\n" % f.after, re.M)
    single = re.compile(r"(\b%s: [^,}\n]+?)( \})" % f.after)

    def go(text):
        if re.search(r"\b%s: " % f.name, text):
            return text
        if multi.search(text):
            return multi.sub(lambda m: "%s%s,\n%s%s: %s,\n" % (m.group(1), m.group(2), m.group(1), f.name, f.default), text)
        assert single.search(text), f.after
        return single.sub(lambda m: "%s, %s: %s%s" % (m.group(1), f.name, f.default, m.group(2)), text)
    for rel in f.e["literal_files"]:
        edit(P(rel), go)


def json_schema(f):
    """The field goes into the JSON Schema of the snapshot and of the artifact (the layout of each file kept); the snapshot struct, the artifact struct and the ts, graphql and proto twins are generated from it."""
    ui = {"label": {"en": f.spec["en"], "de": f.spec["de"]}, "widget": "stepper", "precision": f.spec.get("precision", 3), "step": f.spec.get("step", 0.001)}
    for rel in (SNAP + "/🔣️.json", SCHEMA + "/🔣️.json"):
        if not fa.DRY and schema_twins.add_field(P(rel), f.cam, "f64", after=f.after_cam, container=f.e["model"], ui=ui):
            print("  edited", P(rel).replace(fa.SEMIO, ""))


def diff(f):
    def rs(text):
        if re.search(r"\b%s: f64" % f.name, text):
            return text
        return once(text, r"(%s \{ set \{[^}]*?\b%s: f64) \}" % (re.escape(f.e["model"]), f.after), lambda m: m.group(1) + ", %s: f64 }" % f.name)
    edit(P(DIFF + "/🦀️.rs"), rs)

    def js(text):
        import sys
        sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
        from schema_json import dump_schema
        data = json.loads(text)
        props = data["$defs"][f.e["patch"]]["properties"]
        if f.cam in props:
            return text
        out = {}
        for key, value in props.items():
            out[key] = value
            if key == f.after_cam:
                out[f.cam] = {"anyOf": [{"type": "number"}, {"type": "null"}]}
        assert f.cam in out, f.after_cam
        data["$defs"][f.e["patch"]]["properties"] = out
        return dump_schema(data)
    edit(P(DIFF + "/🔣️.json"), js)


def sqlite(f):
    table, floats, var, tvar = f.e["table"], f.e["floats"], f.e["rs_var"], f.e["ts_var"]
    state = {}

    def sql(text):
        line = re.search(r"^CREATE TABLE %s\(.*$" % table, text, re.M).group(0)
        if " %s REAL," % f.name in line:
            return text
        new = once(line, r"( %s REAL,)" % f.after, lambda m: m.group(1) + " %s REAL," % f.name)
        assert new.rstrip().endswith(");"), "sql tail"
        new = new.rstrip()[:-2] + ", %s_ieee754_bits INTEGER NOT NULL, %s_ieee754_class TEXT NOT NULL);" % (f.name, f.name)
        return text.replace(line, new)
    edit(P(SQ + "/🗄️.sql"), sql)

    rs_text = fa.rd(P(SQ + "/🦀️.rs"))
    count = int(re.search(r'entities\(database, "%s", (\d+),' % table, rs_text).group(1))
    if "%s: row.real(" % f.name in rs_text:
        pass
    else:
        def rs(text):
            text = once(text, r"(Cell::Real\(%s\.%s\))(\], %s\))" % (var, f.after, floats), lambda m: m.group(1) + ", Cell::Real(%s.%s)" % (var, f.name) + m.group(2))
            text = once(text, r"(const %s: &\[FloatColumn\] = &\[[^\]]*)\];" % floats, lambda m: m.group(1) + ", FloatColumn::Binary64(%d)];" % count)
            text = once(text, r'(entities\(database, "%s", )%d,' % (table, count), lambda m: m.group(1) + "%d," % (count + 1))
            return once(text, r"(\b%s: row\.real\(%d\)\?)( \}|,)" % (f.after, count - 1), lambda m: m.group(1) + ", %s: row.real(%d)?" % (f.name, count) + m.group(2))
        edit(P(SQ + "/🦀️.rs"), rs)

    def ts(text):
        if "%s:readBinary64(" % f.cam in text and "%s:Binary64" % f.cam in text:
            return text
        count = int(re.search(r"entities\(tables\[\d+\]!,(\d+),%s,options\)" % floats, text).group(1))
        text = once(text, r'(export type %s=Omit<%s,[^>]*?"%s")(>&\{[^}]*?%s:Binary64)' % (f.e["ts_type"], f.e["ts_snap"], f.after_cam, f.after_cam), lambda m: m.group(1) + '|"%s"' % f.cam + m.group(2) + ";%s:Binary64" % f.cam)
        text = once(text, r"(const %s:readonly Ieee754Column\[\]=\[[^\]]*)\];" % floats, lambda m: m.group(1) + ",{index:%d,width:64}];" % count)
        text = once(text, r'(,%s\.%s)(\],%s,options)' % (tvar, f.after_cam, floats), lambda m: m.group(1) + ",%s.%s" % (tvar, f.cam) + m.group(2))
        text = once(text, r"(entities\(tables\[\d+\]!,)%d(,%s,options\))" % (count, floats), lambda m: m.group(1) + "%d" % (count + 1) + m.group(2))
        return once(text, r"(\b%s:readBinary64\(row,%d,%s\))(\}\)|,)" % (f.after_cam, count - 1, floats), lambda m: m.group(1) + ",%s:readBinary64(row,%d,%s)" % (f.cam, count, floats) + m.group(2))
    edit(P(SQ + "/🟦️.ts"), ts)

    def trs(text):
        anchor = "%s.%s = value;" % (var, f.after)
        if "%s.%s = value;" % (var, f.name) in text:
            return text
        assert anchor in text, anchor
        return text.replace(anchor, anchor + " %s.%s = value;" % (var, f.name), 1)
    edit(P(SQ + "/🧪️tests/🦀️.rs"), trs)

    def tts(text):
        if "%s:binary64(" % f.cam in text:
            return text
        text = once(text, r"(\b%s:binary64\(%s\.%s\))" % (f.after_cam, f.e["ts_test"], f.after_cam), lambda m: m.group(1) + ",%s:binary64(%s.%s)" % (f.cam, f.e["ts_test"], f.cam))
        anchor = "%s.%s=value;" % (f.e["ts_test"], f.after_cam)
        return text.replace(anchor, anchor + "%s.%s=value;" % (f.e["ts_test"], f.cam), 1) if anchor in text else text
    edit(P(SQ + "/🧪️tests/🟦️.ts"), tts)


def field_meta(f):
    unit = f.spec.get("unit_id")
    row = '    ("%s.%s", FieldSpec::%s),\n' % (f.e["prefix"], f.cam, 'quantity("%s", "%s", UnitId::%s)' % (f.spec["en"], f.spec["de"], unit) if unit else 'new("%s", "%s")' % (f.spec["en"], f.spec["de"]))

    def go(text):
        if '"%s.%s"' % (f.e["prefix"], f.cam) in text:
            return text
        return once(text, r'(    \("%s\.%s", [^\n]*\),\n)' % (re.escape(f.e["prefix"]), f.after_cam), lambda m: m.group(1) + row)
    edit(P("/✏️editor/🏷️field-meta/🦀️.rs"), go)

    def test(text):
        if '"%s.%s"' % (f.e["prefix"], f.cam) in text:
            return text
        return once(text, r'(        "%s\.%s",\n)' % (re.escape(f.e["prefix"]), f.after_cam), lambda m: m.group(1) + '        "%s.%s",\n' % (f.e["prefix"], f.cam))
    edit(P("/✏️editor/🏷️field-meta/🧪️tests/🔬️unit/🦀️.rs"), test)


def insert_after_key(text, f):
    lines = text.split("\n")
    out = []
    pattern = re.compile(r'^(\s*)"%s": ([^\n,}]+)(,?)$' % f.after_cam)
    for index, line in enumerate(lines):
        m = pattern.match(line)
        nxt = lines[index + 1] if index + 1 < len(lines) else ""
        if not m or '"%s"' % f.cam in nxt:
            out.append(line)
            continue
        indent = len(m.group(1))
        parent = next((l for l in reversed(lines[:index]) if l.strip() and len(l) - len(l.lstrip()) < indent), "")
        value = "null" if m.group(2).strip() == "null" or parent.strip().startswith('"patch"') else f.default
        out.append('%s"%s": %s,' % (m.group(1), f.after_cam, m.group(2)) if not m.group(3) else line)
        if not m.group(3):
            out.append('%s"%s": %s' % (m.group(1), f.cam, value))
        else:
            out.append('%s"%s": %s,' % (m.group(1), f.cam, value))
    new = "\n".join(out)
    inline = re.compile(r'("%s":\s*)([^,}\s\n]+)(?!,\s*"%s")(\s*[,}])' % (f.after_cam, f.cam))
    return inline.sub(lambda m: '%s%s, "%s": %s%s' % (m.group(1), m.group(2), f.cam, "null" if m.group(2) == "null" else f.default, m.group(3)), new)


def fixtures(f):
    count = 0
    for dirpath, dirs, files in os.walk(ANY):
        dirs[:] = [d for d in dirs if d not in ("target", "node_modules")]
        if "/🧫️fixtures" not in dirpath + "/" and "/🔮️oracles" not in dirpath + "/":
            continue
        for name in files:
            if not name.endswith(".json"):
                continue
            path = os.path.join(dirpath, name)
            text = fa.rd(path)
            if '"%s"' % f.after_cam not in text or text.count('"%s"' % f.after_cam) == text.count('"%s"' % f.cam):
                continue
            new = insert_after_key(text, f)
            if new != text:
                fa.wr(path, new)
                count += 1
    print("  fixtures updated:", count)


def apply(specs, dry):
    for spec in specs["fields"]:
        print("field", spec["entity"], spec["field"])
        f = F(spec)
        json_schema(f)
        diff(f)
        if not fa.DRY:
            schema_twins.regenerate("✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990")
        literals(f)
        sqlite(f)
        field_meta(f)
        fixtures(f)
