#!/usr/bin/env python3
"""Adds the nested action position leaves `x_m` / `y_m` (xM / yM) of `CharacteristicAction` (beam and slab actions) through every EN 1994 schema layer, idempotently:
snapshot model + literals, json/ts/graphql/proto twins (own and combined), diff patch structs and their twins, SQLite (sql, rs, ts, admission, tests), field-meta, every committed JSON fixture
(hand-fixed all at once, formatting preserved) and the mutation generator specification (kinds are generated afterwards by `en1994-leaf-kinds.py`).
Usage: python3 -I en1994-action-position.py [--dry]
"""
import json, os, re, sys

SEMIO = "/home/deck/semio"
ANY = SEMIO + "/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any"
TICKET = SEMIO + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS"
DRY = "--dry" in sys.argv
SNAP = ANY + "/🧬️schema/📸️snapshot"
DIFF = ANY + "/🧬️schema/🔺️diff"
SQL = ANY + "/🚪️io/🪶️sqlite/📸️snapshot"

FIELDS = [
    ("x_m", "xM", "Position x along the member span (0 = default)", "Position x entlang der Stützweite (0 = Standard)"),
    ("y_m", "yM", "Position y across the member width (0 = default)", "Position y quer zur Bauteilbreite (0 = Standard)"),
]


def rd(path):
    return open(path, encoding="utf-8").read()


def wr(path, text):
    if not DRY:
        open(path, "w", encoding="utf-8").write(text)


def edit(path, fn):
    text = rd(path)
    new = fn(text)
    if new != text:
        wr(path, new)
        print("  edited", path.replace(ANY, "…"))


def once(text, pattern, repl, flags=0, count=1):
    found = len(re.findall(pattern, text, flags))
    assert found == count, (pattern[:100], "expected", count, "found", found)
    return re.sub(pattern, repl, text, flags=flags)


def snapshot_rs(text):
    if "pub x_m: f64" in text:
        return text
    block = "    #[dsl(unit = \"Pa\")]\n    pub delta_tau_k_pa: f64,\n"
    new = block + "    /// 📍️ Position of a concentrated force along the member span [m] from its start (0 = mid-span).\n    #[dsl(unit = \"m\")]\n    pub x_m: f64,\n    /// 📍️ Position of a slab point force across the strip width [m] (0 = strip axis).\n    #[dsl(unit = \"m\")]\n    pub y_m: f64,\n"
    assert text.count(block) == 1
    text = text.replace(block, new)
    return once(text, r"delta_tau_k_pa: 0\.0,", "delta_tau_k_pa: 0.0, x_m: 0.0, y_m: 0.0,", count=5)


def json_props(path_list, defs_key, anchor, ref_style):
    for path in path_list:
        data = json.loads(rd(path))
        assert json.dumps(data, indent=2, ensure_ascii=False) + "\n" == rd(path), path
        for name in defs_key:
            container = data["$defs"][name]
            if "xM" in container["properties"]:
                continue
            props = {}
            for key, value in container["properties"].items():
                props[key] = value
                if key == anchor:
                    for _, cam, en, de in FIELDS:
                        props[cam] = ref_style(name, cam, en, de)
            container["properties"] = props
            required = container["required"]
            at = required.index(anchor) + 1
            required[at:at] = [cam for _, cam, _, _ in FIELDS]
        wr(path, json.dumps(data, indent=2, ensure_ascii=False) + "\n")
        print("  edited", path.replace(ANY, "…"))


def snapshot_prop(_name, _cam, en, de):
    return {"type": "number", "x-semio-ui": {"widget": "stepper", "label": {"en": en, "de": de}, "unit": "m", "step": 0.05, "precision": 2}}


def diff_prop(_name, cam, _en, _de):
    return {"anyOf": [{"$ref": "https://json.schemas.assets.semio-tech.com/s/norm/en1994/snapshot.json#/$defs/CharacteristicAction/properties/" + cam}, {"type": "null"}]}


def ts_snapshot(text):
    if re.search(r"^  xM: number;", text, re.M):
        return text
    text = once(text, r"(^  deltaTauKPa: number;\n)", lambda m: m.group(1) + "  xM: number;\n  yM: number;\n", re.M)
    return once(text, r"(parseCharacteristicAction: .*?deltaTauKPa: normWireRequired\(normWireNumber\))( \}\);)", lambda m: m.group(1) + ", xM: normWireRequired(normWireNumber), yM: normWireRequired(normWireNumber)" + m.group(2), re.M)


def graphql_snapshot(text):
    if re.search(r"^  xM:", text, re.M):
        return text
    return once(text, r"(type CharacteristicAction \{.*?  deltaTauKPa: Float!\n)", lambda m: m.group(1) + "  xM: Float!\n  yM: Float!\n", re.S)


def proto_snapshot(text):
    if " x_m = " in text:
        return text
    return once(text, r"(message CharacteristicAction \{.*?  double delta_tau_k_pa = 10;\n)", lambda m: m.group(1) + "  double x_m = 11;\n  double y_m = 12;\n", re.S)


def ts_diff(text):
    if re.search(r"^  xM: number \| null;", text, re.M):
        return text
    text = once(text, r"(^  deltaTauKPa: number \| null;\n)", lambda m: m.group(1) + "  xM: number | null;\n  yM: number | null;\n", re.M, count=2)
    return once(text, r"(deltaTauKPa: normWireRequired\(normWireNullable\(normWireNumber\)\))( \}\);)", lambda m: m.group(1) + ", xM: normWireRequired(normWireNullable(normWireNumber)), yM: normWireRequired(normWireNullable(normWireNumber))" + m.group(2), count=2)


def graphql_diff(text):
    if re.search(r"^  xM: Float$", text, re.M):
        return text
    return once(text, r"(^  deltaTauKPa: Float\n)", lambda m: m.group(1) + "  xM: Float\n  yM: Float\n", re.M, count=2)


def proto_diff(text):
    if " x_m = " in text:
        return text
    return once(text, r"(  optional double delta_tau_k_pa = 8;\n)", lambda m: m.group(1) + "  optional double x_m = 9;\n  optional double y_m = 10;\n", count=2)


def patch_rs(text):
    if "pub x_m: Option<f64>" in text:
        return text
    text = once(text, r"(    pub delta_tau_k_pa: Option<f64>,\n)", lambda m: m.group(1) + "    pub x_m: Option<f64>,\n    pub y_m: Option<f64>,\n")
    text = once(text, r"(        if let Some\(value\) = &self\.delta_tau_k_pa \{\n            row\.delta_tau_k_pa = value\.clone\(\);\n        \}\n)", lambda m: m.group(1) + "".join("        if let Some(value) = &self.%s {\n            row.%s = value.clone();\n        }\n" % (f, f) for f, *_ in FIELDS))
    text = once(text, r"(        if other\.delta_tau_k_pa\.is_some\(\) \{\n            self\.delta_tau_k_pa = other\.delta_tau_k_pa;\n        \}\n)", lambda m: m.group(1) + "".join("        if other.%s.is_some() {\n            self.%s = other.%s;\n        }\n" % (f, f, f) for f, *_ in FIELDS))
    return once(text, r"(            delta_tau_k_pa: self\.delta_tau_k_pa\.as_ref\(\)\.map\(\|_\| row\.delta_tau_k_pa\.clone\(\)\),\n)", lambda m: m.group(1) + "".join("            %s: self.%s.as_ref().map(|_| row.%s.clone()),\n" % (f, f, f) for f, *_ in FIELDS))


def sql(text):
    if "x_m REAL" in text:
        return text
    lines = text.split("\n")
    for i, line in enumerate(lines):
        if re.match(r"CREATE TABLE en1994_(beam|slab)_action ", line):
            line = line.replace("delta_tau_k_pa REAL,", "delta_tau_k_pa REAL,x_m REAL,y_m REAL,", 1)
            assert line.endswith(");")
            line = line[:-2] + "".join(",%s_ieee754_bits INTEGER NOT NULL,%s_ieee754_class TEXT NOT NULL" % (f, f) for f, *_ in FIELDS) + ");"
            lines[i] = line
    return "\n".join(lines)


def sqlite_rs(text):
    if "x_m:row.real(11)" in text:
        return text
    text = once(text, r"(const ACTION:&\[FloatColumn\]=&\[FloatColumn::Binary64\(7\),FloatColumn::Binary64\(8\),FloatColumn::Binary64\(9\),FloatColumn::Binary64\(10\))\];", lambda m: m.group(1) + ",FloatColumn::Binary64(11),FloatColumn::Binary64(12)];")
    text = once(text, r"(Cell::Real\(value\.delta_tau_k_pa\))\],ACTION\)", lambda m: m.group(1) + ",Cell::Real(value.x_m),Cell::Real(value.y_m)],ACTION)")
    text = once(text, r"(delta_tau_k_pa:row\.real\(10\)\?)\}\)", lambda m: m.group(1) + ",x_m:row.real(11)?,y_m:row.real(12)?})")
    return once(text, r'(entities\(database,"en1994_(?:beam|slab)_action",)11,', lambda m: m.group(1) + "13,", count=2)


def sqlite_ts(text):
    if "xM:readBinary64(row,11,ACTION)" in text:
        return text
    text = once(text, r'("qAreaPa"\|"fKN"\|"deltaSigmaKPa"\|"deltaTauKPa")(>)', lambda m: m.group(1) + '|"xM"|"yM"' + m.group(2))
    text = once(text, r"(const ACTION:readonly Ieee754Column\[\]=\[7,8,9,10)\]", lambda m: m.group(1) + ",11,12]")
    text = once(text, r"(value\.deltaTauKPa)\],ACTION", lambda m: m.group(1) + ",value.xM,value.yM],ACTION")
    text = once(text, r"(deltaTauKPa:readBinary64\(row,10,ACTION\))\}\)", lambda m: m.group(1) + ",xM:readBinary64(row,11,ACTION),yM:readBinary64(row,12,ACTION)})")
    return once(text, r"entities\((tables\[(?:5|10)\]!),11,ACTION,", lambda m: "entities(%s,13,ACTION," % m.group(1), count=2)


def admission(text):
    if "Real(7),Real(8),Real(9)" in text:
        return text
    return once(text, r"Text\(3\),Real\(4\),Real\(5\),Real\(6\),Real\(7\)\]\)\?,ACTION\)", "Text(3),Real(4),Real(5),Real(6),Real(7),Real(8),Real(9)])?,ACTION)")


def sqlite_tests_rs(text):
    if "a.x_m=v;" in text:
        return text
    return once(text, r"a\.delta_tau_k_pa=v;", "a.delta_tau_k_pa=v;a.x_m=v;a.y_m=v;", count=2)


def sqlite_tests_ts(text):
    if '"deltaTauKPa","xM"' in text:
        return text
    return once(text, r'"deltaTauKPa"\]', '"deltaTauKPa","xM","yM"]', count=2)


def sqlite_fixture(text):
    if '"xM"' in text:
        return text
    return once(text, r'"deltaTauKPa":(\d+)\}', lambda m: '"deltaTauKPa":%s,"xM":0,"yM":0}' % m.group(1), count=3)


def field_meta(text):
    if "actions[].xM" in text:
        return text
    for coll in ("beams", "slabs"):
        anchor = '    ("%s[].actions[].deltaTauKPa", mu(' % coll
        i = text.index(anchor)
        j = text.index("\n", i) + 1
        rows = ""
        if coll == "beams":
            rows += '    ("beams[].actions[].xM", mu("Point force position x", "Position der Einzellast x", UnitId::Metre)),\n'
        else:
            rows += '    ("slabs[].actions[].xM", mu("Point force position x along the strip", "Position der Einzellast x entlang des Streifens", UnitId::Metre)),\n'
            rows += '    ("slabs[].actions[].yM", mu("Point force position y across the strip", "Position der Einzellast y quer zum Streifen", UnitId::Metre)),\n'
        text = text[:j] + rows + text[j:]
    return text


def literal(text):
    if "delta_tau_k_pa: 40e6, x_m" in text:
        return text
    return once(text, r"delta_tau_k_pa: 40e6,", "delta_tau_k_pa: 40e6, x_m: 0.0, y_m: 0.0,")


def scene_test_literal(text):
    if "delta_tau_k_pa: 0.0, x_m" in text:
        return text
    return once(text, r"delta_tau_k_pa: 0\.0 \}\);", "delta_tau_k_pa: 0.0, x_m: 0.0, y_m: 0.0 });")


def semantic(text):
    data = json.loads(text)
    for t in ("en1994_beam_action", "en1994_slab_action"):
        if data["tableWidths"][t] == 19:
            data["tableWidths"][t] = 25
    return json.dumps(data, indent=2, ensure_ascii=False) + "\n"


def walk(node, is_diff):
    changed = 0
    if isinstance(node, dict):
        if "deltaTauKPa" in node and "xM" not in node:
            patch = is_diff and "index" in node and "id" not in node
            items = []
            for key, value in node.items():
                items.append((key, value))
                if key == "deltaTauKPa":
                    items += [("xM", None if patch else 0.0), ("yM", None if patch else 0.0)]
            node.clear()
            node.update(items)
            changed += 1
        for value in list(node.values()):
            changed += walk(value, is_diff)
    elif isinstance(node, list):
        for value in node:
            changed += walk(value, is_diff)
    return changed


def fixtures():
    total = files = 0
    for dirpath, dirs, names in os.walk(ANY):
        dirs[:] = [d for d in dirs if d not in ("target", "node_modules", "__pycache__")]
        if "/🧬️schema" in dirpath and "/🧪️tests/" not in dirpath and "🧫️fixtures" not in dirpath:
            continue
        for name in names:
            if not name.endswith(".json"):
                continue
            path = os.path.join(dirpath, name)
            text = rd(path)
            if "deltaTauKPa" not in text or "sqlite" in path or "mutation-leaf" in path:
                continue
            data = json.loads(text)
            if json.dumps(data, indent=2, ensure_ascii=False) + "\n" != text:
                continue
            n = walk(data, "🔺️diff" in path)
            if n:
                wr(path, json.dumps(data, indent=2, ensure_ascii=False) + "\n")
                total += n
                files += 1
    print("  fixtures:", files, "files,", total, "action objects")


def main():
    edit(SNAP + "/🦀️.rs", snapshot_rs)
    json_props([SNAP + "/🔣️.json", ANY + "/🧬️schema/🔣️.json"], ["CharacteristicAction"], "deltaTauKPa", snapshot_prop)
    json_props([DIFF + "/🔣️.json"], ["En1994BeamsActionsPatch", "En1994SlabsActionsPatch"], "deltaTauKPa", diff_prop)
    edit(SNAP + "/🟦️.ts", ts_snapshot)
    edit(ANY + "/🧬️schema/🟦️.ts", ts_snapshot)
    edit(SNAP + "/🔗️.graphql", graphql_snapshot)
    edit(ANY + "/🧬️schema/🔗️.graphql", graphql_snapshot)
    edit(SNAP + "/🛰️.proto", proto_snapshot)
    edit(DIFF + "/🟦️.ts", ts_diff)
    edit(DIFF + "/🔗️.graphql", graphql_diff)
    edit(DIFF + "/🛰️.proto", proto_diff)
    edit(DIFF + "/🏗️beam-rows/🦀️.rs", patch_rs)
    edit(DIFF + "/🧱️slab-rows/🦀️.rs", patch_rs)
    edit(SQL + "/🗄️.sql", sql)
    edit(SQL + "/🦀️.rs", sqlite_rs)
    edit(SQL + "/🟦️.ts", sqlite_ts)
    edit(SQL + "/🛂️admission/🦀️.rs", admission)
    edit(SQL + "/🧪️tests/🦀️.rs", sqlite_tests_rs)
    edit(SQL + "/🧪️tests/🟦️.ts", sqlite_tests_ts)
    edit(SQL + "/🧪️tests/🧫️fixtures/🔣️.json", sqlite_fixture)
    edit(SQL + "/🧪️tests/🧫️fixtures/🎛️semantic.json", semantic)
    edit(ANY + "/✏️editor/🏷️field-meta/🦀️.rs", field_meta)
    edit(ANY + "/🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs", literal)
    edit(ANY + "/🎬️scene/🧪️tests/🔬️unit/🦀️.rs", scene_test_literal)
    fixtures()


if __name__ == "__main__":
    main()
