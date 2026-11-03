#!/usr/bin/env python3
"""Turns every enum-like string field of `spec.py` (field key `enum`, set `ENUMS`) into a closed Rust enum through every schema layer, in place and idempotently, with no migration (the wire values are kebab-case and equal the former strings, so fixtures stay valid):
model enum + field type, snapshot ts (type, literal reader, interface, reader), snapshot json (`$defs` with EN/DE option labels, `$ref` property), diff rs/ts, SQLite (CHECK constraint, Rust and TS codecs, admission `Enumeration`), the mutation leaf of each field (payload type, validation removed, schema json/ts), unit-test samples.
Usage: python3 -I enums.py [--dry] [--root PATH_TO_SUBSET_DIR] [--workspace PATH]"""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.dirname(HERE) + "/placements-adder")
import spec as S
import schema as SC
import kinds as K
import padd

DRY = "--dry" in sys.argv
SC.DRY = DRY
ROOT, SCH, Q = SC.ROOT, SC.SCH, SC.Q
MUT = K.MUT
EMOJI = {"TaperKind": "📐️", "TensionSide": "🔻️", "CutoutKind": "🕳️", "CutoutFace": "🧱️", "LayerOrientation": "🧭️", "DiaphragmKind": "🛡️", "DiaphragmMethod": "🧮️", "TrussKind": "🔺️", "BracingProduct": "🏷️", "BracingKind": "✖️", "LaminationProduct": "🪵️", "PrestressBarProduct": "🔩️"}
MODEL = "semio_s_artifact_norm_en1995_model::"


def snake(name):
    return re.sub(r"(?<!^)([A-Z])", r"_\1", name).lower()


def lower_camel(name):
    return name[:1].lower() + name[1:]


def targets():
    """Enum fields of the entities the snapshot model already has: spec entities that `schema.py` has not added yet are skipped (it creates them with their enums)."""
    out = []
    model = open(SC.SCH + "📸️snapshot/🦀️.rs", encoding="utf-8").read()
    for group, nested in ((S.ENTITIES, False), (S.CHILDREN, True)):
        for e in group:
            if "pub struct %s {" % e["ent"] not in model:
                continue
            for i, fl in enumerate(e["rest"]):
                if fl.get("enum"):
                    out.append(dict(e=e, fl=fl, idx=i + 1, nested=nested, enum=fl["enum"], variants=S.ENUMS[fl["enum"]]["variants"]))
    return out


TARGETS = targets()
ENUM_NAMES = sorted({t["enum"] for t in TARGETS}, key=lambda n: [t["enum"] for t in TARGETS].index(n))


def rs_enum(name):
    d = S.ENUMS[name]
    body = "".join("    %s,\n" % v[1] for v in d["variants"])
    return '\n/// %s %s\n#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]\n#[canonical_json(owner = semio_framework_pack_json)]\n#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]\n#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "kebab-case"))]\n#[value(rename_all = "kebab-case")]\npub enum %s {\n%s}\n' % (EMOJI[name], d["doc"], name, body)


def model():
    def go(t):
        for name in ENUM_NAMES:
            if "pub enum %s " % name not in t:
                m = re.search(r"pub enum SupportType \{.*?\n\}\n", t, re.S)
                t = t[:m.end()] + rs_enum(name) + t[m.end():]
        for name in ENUM_NAMES:
            if "impl %s {" % name not in t:
                m = re.search(r"pub enum %s \{.*?\n\}\n" % name, t, re.S)
                arms = "".join('            Self::%s => "%s",\n' % (v[1], v[0]) for v in S.ENUMS[name]["variants"])
                t = t[:m.end()] + "\nimpl %s {\n    /// %s Wire id of the variant, equal to its serialised form and to the catalogue case inputs.\n    pub const fn id(self) -> &'static str {\n        match self {\n%s        }\n    }\n}\n" % (name, EMOJI[name], arms) + t[m.end():]
        for tg in TARGETS:
            ent = tg["e"]["ent"]
            m = re.search(r"pub struct %s \{.*?\n\}\n" % ent, t, re.S)
            blk = m.group(0)
            old = "    pub %s: String,\n" % tg["fl"]["name"]
            if old in blk:
                t = t[:m.start()] + blk.replace(old, "    pub %s: %s,\n" % (tg["fl"]["name"], tg["enum"]), 1) + t[m.end():]
        return t
    SC.edit(SCH + "📸️snapshot/🦀️.rs", go)


def snapshot_ts():
    def go(t):
        for name in ENUM_NAMES:
            vals = [v[0] for v in S.ENUMS[name]["variants"]]
            if "export type %s =" % name not in t:
                t = SC.sub1(t, r"(export type SupportType = [^\n]*\n)", lambda m: m.group(1) + "export type %s = %s;\n" % (name, " | ".join('"%s"' % v for v in vals)))
                t = SC.sub1(t, r"(export const parseSupportType: [^\n]*\n)", lambda m: m.group(1) + "export const parse%s: NormWireReader<%s> = normWireLiteral(%s);\n" % (name, name, ", ".join('"%s"' % v for v in vals)))
        for tg in TARGETS:
            ent, fl = tg["e"]["ent"], tg["fl"]
            m = re.search(r"export interface %s \{\n.*?\n\}\n" % ent, t, re.S)
            blk = m.group(0)
            old = "  %s: string;\n" % fl["camel"]
            if old in blk:
                t = t[:m.start()] + blk.replace(old, "  %s: %s;\n" % (fl["camel"], tg["enum"]), 1) + t[m.end():]
            m = re.search(r"export const parse%s: [^\n]*\n" % ent, t)
            line = m.group(0)
            old = "%s: normWireRequired(normWireString)" % fl["camel"]
            if old in line:
                t = t[:m.start()] + line.replace(old, "%s: normWireRequired(normWireRef(() => parse%s))" % (fl["camel"], tg["enum"]), 1) + t[m.end():]
        return t
    SC.edit(SCH + "📸️snapshot/🟦️.ts", go)


def snapshot_json():
    def go(t):
        data = json.loads(t)
        form, tail = padd.json_form(t)
        D, nd = data["$defs"], {}
        for k, v in D.items():
            nd[k] = v
            if k == "SupportType":
                for name in ENUM_NAMES:
                    if name not in D:
                        nd[name] = {"title": name, "type": "string", "enum": [x[0] for x in S.ENUMS[name]["variants"]], "x-semio-ui": {"options": {x[0]: {"en": x[2], "de": x[3]} for x in S.ENUMS[name]["variants"]}}, "x-semio-formats": ["🔣️jsonschema", "🟦️typescript"]}
        data["$defs"] = nd
        for tg in TARGETS:
            prop = data["$defs"][tg["e"]["ent"]]["properties"][tg["fl"]["camel"]]
            if "$ref" not in prop:
                label = prop["x-semio-ui"]["label"]
                data["$defs"][tg["e"]["ent"]]["properties"][tg["fl"]["camel"]] = {"$ref": "#/$defs/" + tg["enum"], "x-semio-ui": {"label": label}}
        for e in list(S.ENTITIES) + list(S.CHILDREN):
            for fl in e["rest"]:
                if fl.get("ref") == "strength-class":
                    prop = data["$defs"][e["ent"]]["properties"][fl["camel"]]
                    prop["x-semio-ui"] = {"widget": "reference", "label": {"en": fl["en"], "de": fl["de"]}, "ref": {"kind": "strength-class", "catalogue": "timber-strength-classes"}}
        return SC.dump_schema(data)
    SC.edit(SCH + "📸️snapshot/🔣️.json", go)


def diff():
    def rs(t):
        for tg in TARGETS:
            ent = tg["e"]["ent"]
            m = re.search(r"^[^\n]*pub En1995%sPatch of [^\n]*\n" % ent, t, re.M)
            line = m.group(0)
            old = " %s: String" % tg["fl"]["name"]
            if old in line:
                t = t[:m.start()] + line.replace(old, " %s: %s%s" % (tg["fl"]["name"], MODEL, tg["enum"]), 1) + t[m.end():]
        return t
    SC.edit(SCH + "🔺️diff/🦀️.rs", rs)

    def ts(t):
        imp = re.search(r'import \{ ([^}]*) \} from "\.\./📸️snapshot/🟦️\.ts";', t)
        names = imp.group(1)
        add = "".join(", type %s, parse%s" % (n, n) for n in ENUM_NAMES if "parse%s" % n not in names)
        if add:
            t = t[:imp.start(1)] + names + add + t[imp.end(1):]
        for tg in TARGETS:
            ent, fl = tg["e"]["ent"], tg["fl"]
            P = "En1995" + ent
            m = re.search(r"export interface %sPatch \{\n.*?\n\}\n" % P, t, re.S)
            blk = m.group(0)
            old = "  %s: string | null;\n" % fl["camel"]
            if old in blk:
                t = t[:m.start()] + blk.replace(old, "  %s: %s | null;\n" % (fl["camel"], tg["enum"]), 1) + t[m.end():]
            m = re.search(r"export const parse%sPatch: [^\n]*\n" % P, t)
            line = m.group(0)
            old = "%s: normWireDefault(normWireNullable(normWireString), () => null)" % fl["camel"]
            if old in line:
                t = t[:m.start()] + line.replace(old, "%s: normWireDefault(normWireNullable(normWireRef(() => parse%s)), () => null)" % (fl["camel"], tg["enum"]), 1) + t[m.end():]
        return t
    SC.edit(SCH + "🔺️diff/🟦️.ts", ts)


def sqlite_rs_fns(name):
    d = S.ENUMS[name]["variants"]
    sn = snake(name)
    to = ",".join('%s::%s=>"%s"' % (name, v[1], v[0]) for v in d)
    frm = ",".join('"%s"=>Ok(%s::%s)' % (v[0], name, v[1]) for v in d)
    return 'fn %s(value:%s)-> &\'static str{match value{%s}}\nfn read_%s(value:&str)->Result<%s, ValueError>{match value{%s,_=>Err(invalid("EN1995 unsupported %s"))}}\n' % (sn, name, to, sn, name, frm, sn.replace("_", " "))


def sqlite_ts_fn(name):
    vals = [v[0] for v in S.ENUMS[name]["variants"]]
    cond = "&&".join('value!=="%s"' % v for v in vals)
    return 'function %s(value:string):%s{if(%s)throw Error("EN1995 unsupported %s");return value as %s}\n' % (lower_camel(name), name, cond, snake(name).replace("_", " "), name)


def sqlite():
    sqlp = Q + "🗄️.sql"

    def sql(t):
        for tg in TARGETS:
            tbl = tg["e"]["tbl"]
            m = re.search(r"CREATE TABLE %s \(.*?\n\);\n" % tbl, t, re.S)
            blk = m.group(0)
            old = "\n  %s TEXT NOT NULL" % tg["fl"]["name"]
            check = " CHECK (%s IN (%s))" % (tg["fl"]["name"], ",".join("'%s'" % v[0] for v in tg["variants"]))
            if old in blk and check not in blk:
                t = t[:m.start()] + blk.replace(old, old + check, 1) + t[m.end():]
        return t
    SC.edit(sqlp, sql)

    def rs(t):
        use = re.search(r"use crate::\{[^}]*\};", t)
        clause = use.group(0)
        add = "".join(", " + n for n in ENUM_NAMES if n not in clause)
        if add:
            t = t.replace(clause, clause[:-2] + add + "};", 1)
        anchor = re.search(r"^fn read_support[^\n]*\n", t, re.M)
        fns = "".join(sqlite_rs_fns(n) for n in ENUM_NAMES if "fn read_%s(" % snake(n) not in t)
        if fns:
            t = t[:anchor.end()] + fns + t[anchor.end():]
        for tg in TARGETS:
            tbl, fl, name = tg["e"]["tbl"], tg["fl"], tg["enum"]
            sn = snake(name)
            m = re.search(r'insert_float\("%s",&\[.*?\],\w+_FLOATS\)' % tbl, t, re.S)
            blk = m.group(0)
            nb = re.sub(r"Cell::Text\(&(\w+)\.%s\)" % fl["name"], lambda x: "Cell::Text(%s(%s.%s))" % (sn, x.group(1), fl["name"]), blk)
            t = t[:m.start()] + nb + t[m.end():]
            plural = tg["e"]["plural"]
            lines = t.split("\n")
            for i, ln in enumerate(lines):
                if re.search(r"in &(?:self|\w+)\.%s\{bound" % plural, ln):
                    lines[i] = re.sub(r",&\w+\.%s\b" % fl["name"], "", ln)
            t = "\n".join(lines)
            ent = tg["e"]["ent"]
            t = re.sub(r"(%s\{[^}]*?)\b%s:reconstruct_text\(control,(\w+)\.text\((\d+)\)\?\)\?" % (ent, fl["name"]), lambda x: "%s%s:read_%s(%s.text(%s)?)?" % (x.group(1), fl["name"], sn, x.group(2), x.group(3)), t, count=1)
        return t
    SC.edit(Q + "🦀️.rs", rs)

    def ts(t):
        imp = re.search(r"import type\{([^}]*)\}from", t)
        add = "".join("," + n for n in ENUM_NAMES if n not in imp.group(1).split(","))
        if add:
            t = t[:imp.end(1)] + add + t[imp.end(1):]
        anchor = re.search(r"^function support\([^\n]*\n", t, re.M)
        fns = "".join(sqlite_ts_fn(n) for n in ENUM_NAMES if "function %s(" % lower_camel(n) not in t)
        if fns:
            t = t[:anchor.end()] + fns + t[anchor.end():]
        for tg in TARGETS:
            fl = tg["fl"]
            plural = tg["e"]["plural"]
            t = re.sub(r"(\b%s\.push\(\{[^}]*?)\b%s:artifactSqliteText\((\w+),(\d+)\)" % (plural, fl["camel"]), lambda x: "%s%s:%s(artifactSqliteText(%s,%s))" % (x.group(1), fl["camel"], lower_camel(tg["enum"]), x.group(2), x.group(3)), t, count=1)
        return t
    SC.edit(Q + "🟦️.ts", ts)

    def adm(t):
        for tg in TARGETS:
            tbl = tg["e"]["tbl"]
            m = re.search(r'[^\n]*"%s",[^\n]*\n' % tbl, t)
            line = m.group(0)
            arr = re.search(r"n::cells\(n::record\(\w+\)\?,\[(.*)\]\)\?", line)
            elements = re.findall(r"(?:Text|Real|Unsigned|Boolean|Enumeration)\((?:[^()]|\([^()]*\))*\)", arr.group(1))
            old = "Text(%d)" % tg["idx"]
            if elements[tg["idx"]] == old:
                elements[tg["idx"]] = "Enumeration(%d,&[%s])" % (tg["idx"], ",".join('"%s"' % v[0] for v in tg["variants"]))
                line2 = line[:arr.start(1)] + ",".join(elements) + line[arr.end(1):]
                t = t.replace(line, line2, 1)
        return t
    SC.edit(Q + "🛂️admission/🦀️.rs", adm)
    new_bytes = SC.schema_bytes(open(sqlp, encoding="utf-8").read())
    widths = None

    def sj(t):
        d = json.loads(t)
        d["schemaBytes"] = new_bytes
        return json.dumps(d, indent=2, ensure_ascii=False) + "\n"
    SC.edit(Q + "🧪️tests/🧫️fixtures/🎛️semantic.json", sj)
    SC.edit(Q + "🧪️tests/🟦️.ts", lambda t: re.sub(r'(normSemanticContract\["schemaBytes"\]\)\.toEqual\()\d+', lambda m: m.group(1) + str(new_bytes), t))


def leaves():
    by_field = {id(k["fl"]): k for k in K.LEAVES if k["op"] == "change"}
    for tg in TARGETS:
        k = by_field[id(tg["fl"])]
        name = tg["enum"]
        d = MUT + "/" + k["dirname"]

        def leaf(t, name=name):
            return t.replace("new_value: String }", "new_value: %s%s }" % (MODEL, name), 1)
        SC.edit(d + "/🦀️.rs", leaf)

        def dif(t, name=name):
            t = re.sub(r"    if !\[[^\]]*\]\.contains\(&payload\.new_value\.as_str\(\)\) \{\n[^\n]*\n    \}\n", "", t)
            return t.replace("Some(payload.new_value.clone())", "Some(payload.new_value)")
        SC.edit(d + "/🔺️diff/🦀️.rs", dif)
        SC.edit(d + "/↩️inverse/🦀️.rs", lambda t: re.sub(r"new_value: (item|row)\.(\w+)\.clone\(\)", r"new_value: \1.\2", t))

        def js(t, name=name):
            o = json.loads(t)
            prop = o["properties"]["newValue"]
            if "$ref" not in prop:
                ui = {kk: vv for kk, vv in prop["x-semio-ui"].items() if kk != "options"}
                o["properties"]["newValue"] = {"$ref": K.SNAP + name, "x-semio-ui": ui}
            return K.dumps(o)
        SC.edit(d + "/🧬️schema/🔣️.json", js)

        def ts(t, name=name):
            if "parse%s" % name in t:
                return t
            t = t.replace("  newValue: string;", "  newValue: %s;" % name, 1)
            t = t.replace("newValue: normWireRequired(normWireString)", "newValue: normWireRequired(parse%s)" % name, 1)
            m = re.search(r'import \{[^\n]*\} from "@semio-tech/norm-js/registry/contract";\n', t)
            return t[:m.end()] + 'import { parse%s, type %s } from "../../../📸️snapshot/🟦️.ts";\n' % (name, name) + t[m.end():]
        SC.edit(d + "/🧬️schema/🟦️.ts", ts)
        for path in (MUT + "/🧪️tests/🔬️unit/🦀️.rs", ROOT + "🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs", ROOT + "🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️op-round-trip/🦀️.rs"):
            def sample(t, k=k, tg=tg):
                variant = {v[0]: v[1] for v in tg["variants"]}
                return re.sub(r"(::%s \{[^}]*?new_value: )\"([\w-]+)\"\.into\(\)" % k["rec"], lambda m: "%s%s%s::%s" % (m.group(1), MODEL, tg["enum"], variant[m.group(2)]), t)
            SC.edit(path, sample)


def field_meta():
    p = ROOT + "✏️editor/🏷️field-meta/🦀️.rs"

    def go(t):
        t = t.replace('("panels[].strengthClass", FieldSpec::new("Strength class of the layers", "Festigkeitsklasse der Lagen")),', '("panels[].strengthClass", FieldSpec::choice("Strength class of the layers", "Festigkeitsklasse der Lagen", STRENGTH)),')
        return t.replace('("panels[].layers[].grade", FieldSpec::new("Strength grade", "Festigkeitsklasse")),', '("panels[].layers[].grade", FieldSpec::choice("Strength grade", "Festigkeitsklasse", STRENGTH)),')
    SC.edit(p, go)


if __name__ == "__main__":
    print("enums:", ", ".join(ENUM_NAMES), "|", len(TARGETS), "fields")
    model()
    snapshot_ts()
    snapshot_json()
    diff()
    sqlite()
    leaves()
    field_meta()
