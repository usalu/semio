"""EN 1997 profile of the field-adder: adds one `f64` field to a row of the snapshot (`slopes`, `piles`, `retainingWalls`) through the schema (snapshot and diff JSON; every twin and the struct field are generated), the diff macro (rs), SQLite (sql, rs, ts, tests, admission), DSL assets, field meta and every fixture (snapshot rows and diff rows). Idempotent. The new field is appended after `after`, which must be the last scalar data column of the table so no positional index of a stored column moves; chain several fields by naming the previous new one."""
import json, os, re

import field_adder as fa
import schema_twins

ANY = fa.NORM + "/🌍️en1997" + fa.SUBSET
PROFILE_PATHS = dict(
    model="/🧬️schema/📸️snapshot/🦀️.rs",
    sql="/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql",
    sqlite_rs="/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs",
    sqlite_ts="/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts",
    sqlite_tests_rs="/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs",
    sqlite_tests_ts="/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
    admission="/🚪️io/🪶️sqlite/📸️snapshot/🛂️admission/🦀️.rs",
)
ENTITIES = {
    "slopes": dict(model="Slope", singular="slope", table="en1997_slope", sqlite_const="SLOPE", row_var="v", sqlite_ts_type="En1997SqliteSlope", unit_prefix="slopes[]", snake="slopes", patch="En1997SlopesPatch", gql_input="SlopeInput", fields_head='"id","angleDeg"', dsl="slopes", context=None, root_field=11, nested_field=None),
    "piles": dict(model="Pile", singular="pile", table="en1997_pile", sqlite_const="PILE", row_var="v", sqlite_ts_type="En1997SqlitePile", unit_prefix="piles[]", snake="piles", patch="En1997PilesPatch", gql_input="PileInput", fields_head='"id","pileType"', dsl="piles", context=None, root_field=9, nested_field=12),
    "labTests": dict(model="LabTest", singular="test", table="en1997_lab_test", sqlite_const="LABTEST", row_var="v", sqlite_ts_type="En1997SqliteLabTest", unit_prefix="labTests[]", snake="lab_tests", patch="En1997LabTestsPatch", gql_input="LabTestInput", fields_head='"id","layerId"', dsl="lab-tests", context=None, root_field=16, nested_field=0),
    "layerWaters": dict(model="LayerWater", singular="water", table="en1997_layer_water", sqlite_const="LAYERWATER", row_var="v", sqlite_ts_type="En1997SqliteLayerWater", unit_prefix="layerWaters[]", snake="layer_waters", patch="En1997LayerWatersPatch", gql_input="LayerWaterInput", fields_head='"id","layerId"', dsl="layer-waters", context=None, root_field=17, nested_field=None),
    "retainingWalls": dict(model="RetainingWall", singular="wall", table="en1997_retaining_wall", sqlite_const="WALL", row_var="v", sqlite_ts_type="En1997SqliteWall", unit_prefix="retainingWalls[]", snake="retaining_walls", patch="En1997RetainingWallsPatch", gql_input="RetainingWallInput", fields_head='"id","height"', dsl="retaining-walls", context="wallMovement", root_field=10, nested_field=None),
}
PROFILE = dict(
    root=ANY,
    entities=ENTITIES,
    paths=dict(
        PROFILE_PATHS,
        json=["/🧬️schema/📸️snapshot/🔣️.json"],
        ts=["/🧬️schema/📸️snapshot/🟦️.ts"],
        graphql=["/🧬️schema/🔗️.graphql", "/🧬️schema/📸️snapshot/🔗️.graphql"],
        proto=["/🧬️schema/🛰️.proto", "/🧬️schema/📸️snapshot/🛰️.proto"],
    ),
)
P = lambda rel: ANY + rel
once, edit = fa.once, fa.edit
DSL_UNITS = {"N": "N", "m": "m", "Pa": "Pa"}


def lit(value):
    if value == int(value) and abs(value) >= 1000:
        text = "%d" % abs(int(value))
        groups = []
        while text:
            groups.insert(0, text[-3:])
            text = text[:-3]
        return ("-" if value < 0 else "") + "_".join(groups) + ".0"
    return repr(float(value))


def model(f):
    """The struct field (docstring, DSL unit) is generated from the schema; the demo literal of the model file names the new field by hand."""
    def go(text):
        if re.search(r"(?<!pub )\b%s: [0-9_.]+," % f.name, text):
            return text
        literal = re.search(r"(?<!struct )\b%s \{\n.*?\n(\s+)%s: [0-9_.]+,\n" % (f.e["model"], f.after), text, re.S)
        assert literal, ("model literal", f.after)
        demo = lit(f.spec.get("demo", f.default))
        return text.replace(literal.group(0), literal.group(0) + "%s%s: %s,\n" % (literal.group(1), f.name, demo), 1)
    edit(P(PROFILE["paths"]["model"]), go)


def diff(f):
    patch, model_name = f.e["patch"], f.e["model"]
    rs = P("/🧬️schema/🔺️diff/🦀️.rs")
    edit(rs, lambda t: t if re.search(r"%s of [\w:]*%s \{ set \{[^}]*\b%s: f64" % (patch, model_name, f.name), t) else once(t, r"(%s of [\w:]*%s \{ set \{[^}]*\b%s: f64)" % (patch, model_name, f.after), lambda m: m.group(1) + ", %s: f64" % f.name))

    def js(text):
        data = json.loads(text)
        node = data["$defs"][patch]
        if f.cam in node["properties"]:
            return text
        ref = "https://json.schemas.assets.semio-tech.com/s/norm/en1997/snapshot.json#/$defs/%s/properties/%s" % (model_name, f.cam)
        assert f.after_cam in node["properties"], "patch json required anchor"
        props, required = {}, []
        for key, value in node["properties"].items():
            props[key] = value
            if key == f.after_cam:
                props[f.cam] = {"anyOf": [{"$ref": ref}, {"type": "null"}]}
        for key in node["required"]:
            required.append(key)
            if key == f.after_cam:
                required.append(f.cam)
        node["properties"], node["required"] = props, required
        return schema_twins.dump_compact(data) if schema_twins.layout_of(text) == "compact" else schema_twins.dump_indent(data)
    edit(P("/🧬️schema/🔺️diff/🔣️.json"), js)


def sqlite(f):
    e = f.e
    table, const, var, ts_type = e["table"], e["sqlite_const"], e["row_var"], e["sqlite_ts_type"]
    rs_text = fa.rd(P(PROFILE["paths"]["sqlite_rs"]))
    columns = int(re.search(r'entities\(d,"%s",(\d+),' % table, rs_text).group(1))
    index = columns

    def sql(text):
        if re.search(r"CREATE TABLE %s \(.*?\b%s REAL" % (table, f.name), text, re.S):
            return text
        text = once(text, r"(CREATE TABLE %s \(.*? %s REAL,)" % (table, f.after), lambda m: m.group(1) + " %s REAL," % f.name, re.S)
        return once(text, r"(CREATE TABLE %s \(.*? %s_ieee754_class TEXT NOT NULL)\n" % (table, f.after), lambda m: m.group(1) + ",\n %s_ieee754_bits INTEGER NOT NULL, %s_ieee754_class TEXT NOT NULL\n" % (f.name, f.name), re.S)
    edit(P(PROFILE["paths"]["sql"]), sql)

    def rs(text):
        if "%s:v.real(" % f.name in text.replace(" ", ""):
            return text
        text = once(text, r"(Cell::Real\(v\.%s\))(\],%s\))" % (f.after, const), lambda m: m.group(1) + ",Cell::Real(v.%s)" % f.name + m.group(2))
        text = once(text, r"(const %s:&\[FloatColumn\]=&\[[^\]]*)\];" % const, lambda m: m.group(1) + ",FloatColumn::Binary64(%d)];" % index)
        text = once(text, r'(entities\(d,"%s",)%d,' % (table, columns), lambda m: m.group(1) + "%d," % (columns + 1))
        return once(text, r"(%s:v\.real\(%d\)\?)" % (f.after, index - 1), lambda m: m.group(1) + ",%s:v.real(%d)?" % (f.name, index))
    edit(P(PROFILE["paths"]["sqlite_rs"]), rs)

    def ts(text):
        if re.search(r"\b%s:readBinary64\(v,\d+,%s\)" % (f.cam, const), text):
            return text
        decl = re.search(r"export type %s=[^\n]*" % ts_type, text).group(0)
        new_decl = decl.replace('|"%s"' % f.after_cam, '|"%s"|"%s"' % (f.after_cam, f.cam), 1).replace("%s:Binary64" % f.after_cam, "%s:Binary64;%s:Binary64" % (f.after_cam, f.cam), 1)
        assert new_decl.count(f.cam) == 2, "sqlite ts type anchor"
        text = text.replace(decl, new_decl, 1)
        text = once(text, r"(const %s:readonly Ieee754Column\[\]=\[[0-9,]*?)\]\.map" % const, lambda m: m.group(1) + ",%d].map" % index)
        fields = re.search(r"fields\(v,\[%s[^\]]*?\"%s\"" % (re.escape(e["fields_head"]), f.after_cam), text)
        assert fields, "sqlite ts fields anchor"
        text = text.replace(fields.group(0), fields.group(0) + ',"%s"' % f.cam, 1)
        text = once(text, r"(,v\.%s)(\],%s,o\))" % (f.after_cam, const), lambda m: m.group(1) + ",v.%s" % f.cam + m.group(2))
        text = re.sub(r"(entities\(t\[\d+\]!,)%d,%s," % (columns, const), lambda m: m.group(1) + "%d,%s," % (columns + 1, const), text)
        return once(text, r"(%s:readBinary64\(v,%d,%s\))" % (f.after_cam, index - 1, const), lambda m: m.group(1) + ",%s:readBinary64(v,%d,%s)" % (f.cam, index, const))
    edit(P(PROFILE["paths"]["sqlite_ts"]), ts)

    def trs(text):
        anchor = "v.%s=w;" % f.after
        if "v.%s=w;" % f.name in text or anchor not in text:
            return text
        return text.replace(anchor, anchor + "v.%s=w;" % f.name, 1)
    edit(P(PROFILE["paths"]["sqlite_tests_rs"]), trs)

    def tts(text):
        if "%s:w" % f.cam in text and "%s:binary64(v.%s)" % (f.cam, f.cam) in text:
            return text
        coll = f.entity
        text = once(text, r"(%s:f\.%s\.map\(v=>\(\{\.\.\.v,[^}]*?\b%s:binary64\(v\.%s\))" % (coll, coll, f.after_cam, f.after_cam), lambda m: m.group(1) + ",%s:binary64(v.%s)" % (f.cam, f.cam))
        return once(text, r"(for\(const v of s\.%s\)\{?Object\.assign\(v,\{[^}]*?\b%s:w)" % (coll, f.after_cam), lambda m: m.group(1) + ",%s:w" % f.cam)
    edit(P(PROFILE["paths"]["sqlite_tests_ts"]), tts)

    edit(P(PROFILE["paths"]["admission"]), lambda text: admission(text, f))


def admission(text, f):
    line = re.search(r'^[^\n]*n::entity\(out,"%s",[^\n]*$' % f.e["table"], text, re.M).group(0)
    cells = re.search(r"\[(Text\(0\)[^\]]*)\]\)\?,%s\)" % f.e["sqlite_const"], line)
    model_text = fa.rd(P(PROFILE["paths"]["model"]))
    body = re.search(r"pub struct %s \{.*?\n\}" % f.e["model"], model_text, re.S).group(0)
    expected = len([1 for _, ty in re.findall(r"^    pub ([a-z_0-9]+): ([^,\n]+),$", body, re.M) if not ty.startswith("Vec<")])
    count = cells.group(1).count("(")
    if count >= expected:
        return text
    grown = cells.group(1) + "".join(",Real(%d)" % i for i in range(count, expected))
    new = line.replace(cells.group(0), cells.group(0).replace(cells.group(1), grown, 1), 1)
    text = text.replace(line, new, 1)
    if f.e["nested_field"] is not None:
        at = text.index(new) + len(new)
        text = text[:at] + re.sub(r"n::field\(v,\d+\)", "n::field(v,%d)" % expected, text[at:], count=1)
    return text


def assets(f):
    kebab = f.name.replace("_", "-")
    after = f.after.replace("_", "-")
    value = f.spec.get("demo", f.default)
    dsl_unit = f.spec.get("dsl_unit")
    token = ("%s%s" % (("%d" % value) if value == int(value) else repr(value), dsl_unit)) if dsl_unit else (("%d" % value) if value == int(value) else repr(value))
    ty = "QTY" if dsl_unit else "NUM"
    for name in ("🏗️compliant", "🎬️demo", "🚨noncompliant"):
        def go(text):
            if " %s:%s" % (kebab, ty) in text.split("\n")[0] or re.search(r"^%s \[[^\]]*\b%s:" % (re.escape(f.e["dsl"]), re.escape(kebab)), text, re.M):
                return text
            header = re.search(r"^%s \[([^\n]*)\] \{$" % re.escape(f.e["dsl"]), text, re.M)
            columns = header.group(1).split(" ")
            position = [c.split(":")[0] for c in columns].index(after)
            columns.insert(position + 1, "%s:%s" % (kebab, ty))
            start = header.end()
            end = text.index("\n}", start)
            rows = []
            for row in text[start:end].split("\n"):
                if not row.strip():
                    rows.append(row)
                    continue
                indent = re.match(r"\s*", row).group(0)
                body = row[len(indent):]
                table = ""
                if " [" in body:
                    body, table = body.split(" [", 1)
                    table = " [" + table
                tokens = body.split(" ")
                tokens.insert(position + 1, token)
                rows.append(indent + " ".join(tokens) + table)
            return text[:header.start()] + "%s [%s] {" % (f.e["dsl"], " ".join(columns)) + "\n".join(rows) + text[end:]
        edit(P("/🖼️assets/%s/🗣️.dsl.semio" % name), go)


def json_insert(text, f):
    ctx, after, cam, default = f.e["context"], f.after_cam, f.cam, f.default_json
    if '"%s"' % cam in text or '"%s"' % after not in text:
        return text
    if ctx is None:
        pretty = re.compile(r'^(\s*)"%s": ([^\n,}]+)(,?)\n' % after, re.M)
        text = pretty.sub(lambda m: '%s"%s": %s,\n%s"%s": %s%s\n' % (m.group(1), after, m.group(2), m.group(1), cam, "null" if m.group(2).strip() == "null" else default, m.group(3)) if m.group(3) else '%s"%s": %s,\n%s"%s": %s\n' % (m.group(1), after, m.group(2), m.group(1), cam, "null" if m.group(2).strip() == "null" else default), text)
        compact = re.compile(r'("%s":\s*)([^,}\s\]]+)(\s*[,}])' % after)
        if "\n" not in text.strip():
            text = compact.sub(lambda m: '%s%s,"%s":%s%s' % (m.group(1), m.group(2), cam, "null" if m.group(2) == "null" else default, m.group(3)), text)
        return text
    pretty = re.compile(r'("%s":[^{}]*?\n(\s*)"%s": )([^\n,}]+)(,?)(\n)' % (ctx, after))
    def pretty_sub(m):
        value = "null" if m.group(3).strip() == "null" else default
        if m.group(4):
            return '%s%s,\n%s"%s": %s,%s' % (m.group(1), m.group(3), m.group(2), cam, value, m.group(5))
        return '%s%s,\n%s"%s": %s%s' % (m.group(1), m.group(3), m.group(2), cam, value, m.group(5))
    text = pretty.sub(pretty_sub, text)
    if "\n" not in text.strip():
        text = re.sub(r'("%s":[^{}]*?"%s":\s*)([^,}\s]+)' % (ctx, after), lambda m: '%s%s,"%s":%s' % (m.group(1), m.group(2), cam, "null" if m.group(2) == "null" else default), text)
    return text


def fixtures(f):
    count = 0
    for top in ("/🧪️tests", "/🚪️io"):
        for dirpath, dirs, files in os.walk(P(top)):
            dirs[:] = [d for d in dirs if d not in ("target", "node_modules")]
            for name in files:
                if not name.endswith(".json"):
                    continue
                path = os.path.join(dirpath, name)
                text = fa.rd(path)
                if '"%s"' % f.after_cam not in text or '"%s"' % f.cam in text:
                    continue
                if f.e["context"] and '"%s"' % f.e["context"] not in text:
                    continue
                new = json_insert(text, f)
                if new != text:
                    fa.wr(path, new)
                    count += 1
    print("  fixtures updated:", count)


def field_meta(f):
    unit_id = fa.UNIT_IDS.get(f.unit) if not f.spec.get("unit_id") else f.spec["unit_id"]
    unit_id = f.spec.get("unit_id", fa.UNIT_IDS.get(f.unit))
    spec = 'FieldSpec::quantity("%s", "%s", UnitId::%s)' % (f.en, f.de, unit_id) if unit_id else 'FieldSpec::new("%s", "%s")' % (f.en, f.de)
    if f.spec.get("range"):
        spec = "ranged(%s, %s, %s)" % (spec, repr(float(f.spec["range"][0])), repr(float(f.spec["range"][1])))
    prefix = f.e["unit_prefix"]

    def go(text):
        if '"%s.%s"' % (prefix, f.cam) in text:
            return text
        return once(text, r'(        \("%s\.%s", [^\n]*\),\n)' % (re.escape(prefix), f.after_cam), lambda m: m.group(1) + '        ("%s.%s", %s),\n' % (prefix, f.cam, spec))
    edit(P("/🧬️schema/💡️inferences/⚖️checks/🦀️.rs"), go)


def apply(specs, dry):
    for spec in specs["fields"]:
        print("field", spec["entity"], spec["field"])
        f = fa.Field(PROFILE, spec)
        fa.json_schema(f)
        diff(f)
        fa.twins_from_json(f)
        model(f)
        sqlite(f)
        assets(f)
        field_meta(f)
        fixtures(f)
