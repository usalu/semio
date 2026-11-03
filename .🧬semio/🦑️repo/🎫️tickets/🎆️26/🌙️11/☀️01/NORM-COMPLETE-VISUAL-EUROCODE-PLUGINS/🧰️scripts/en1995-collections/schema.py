#!/usr/bin/env python3
"""Adds the typed EN 1995 collections of `spec.py` that the artifact does not have yet to every schema layer, idempotently. First run: anchors after `placements`; incremental mode (any later run): only the missing collections, anchored after the last collection present, table and field numbers continuing the existing ones.
model structs and snapshot fields, artifact schema, ts/proto/graphql/json twins (root and snapshot), diff lanes (rs, ts, graphql, proto, json), outline, duplicate ids, field-meta, SQLite (DDL, Rust and TS codec, admission, tests, contract) and `[]` in every fixture.
Usage: python3 -I schema.py [--dry] [--root PATH_TO_SUBSET_DIR] [--only plural,plural]"""
import difflib, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.dirname(HERE) + "/placements-adder")
import spec as S
from spec import ENTITIES, RS_TY, TS_TY, WIRE, SNAP_URL, UNIT_UI, FIELD_META_UNIT, DSL_UNIT
import padd
sys.path.insert(0, os.path.dirname(HERE))
from schema_json import dump_schema

DRY = "--dry" in sys.argv
ROOT = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else S.ANY
if not ROOT.endswith("/"):
    ROOT += "/"
SCH = ROOT + "🧬️schema/"
Q = ROOT + "🚪️io/🪶️sqlite/📸️snapshot/"
ALL = list(ENTITIES)
_model = open(SCH + "📸️snapshot/🦀️.rs", encoding="utf-8").read()
_only = sys.argv[sys.argv.index("--only") + 1].split(",") if "--only" in sys.argv else None
NEW = [e for e in ALL if (e["plural"] in _only if _only else "pub struct %s {" % e["ent"] not in _model)]
ENTITIES = NEW
_first = ALL.index(NEW[0]) if NEW else len(ALL)
PREV = ALL[_first - 1] if _first else dict(plural="placements", ent="Placement", table="placement")
PP, PE, PT = PREV["plural"], PREV["ent"], "en1995_" + PREV["table"]
G = NEW[0] if NEW else None
N0 = open(Q + "🗄️.sql", encoding="utf-8").read().count("CREATE TABLE")
ROOTMAX = max(int(x) for x in re.findall(r"n::field\(root,(\d+)\)", open(Q + "🛂️admission/🦀️.rs", encoding="utf-8").read()))
WORDS = {9: "nine", 14: "fourteen", 16: "sixteen", 17: "seventeen", 18: "eighteen", 19: "nineteen", 20: "twenty", 21: "twenty-one", 22: "twenty-two"}


def rd(p):
    return open(p, encoding="utf-8").read()


def edit(p, fn):
    assert len(p.encode()) < 256 + len("/home/deck/semio/"), "path too long: " + p
    t = rd(p)
    n = fn(t)
    if n != t:
        print("  edit", p.replace(ROOT, "")[:150], sum(1 for l in difflib.unified_diff(t.splitlines(), n.splitlines(), n=0) if l[:1] in "+-" and l[:3] not in ("+++", "---")), "lines")
        if not DRY:
            open(p, "w", encoding="utf-8").write(n)


def sub1(t, pat, repl, flags=0, count=1):
    n = len(re.findall(pat, t, flags))
    assert n == count, (pat[:120], "expected", count, "found", n)
    return re.sub(pat, repl, t, flags=flags)


def after_re(t, pat, new, guard, flags=re.M):
    if guard in t:
        return t
    m = list(re.finditer(pat, t, flags))
    assert len(m) >= 1, "anchor: " + pat[:120]
    return t[:m[0].end()] + new + t[m[0].end():]


def after_last(t, pat, new, guard, flags=re.M):
    if guard in t:
        return t
    m = list(re.finditer(pat, t, flags))
    assert m, "anchor: " + pat[:120]
    return t[:m[-1].end()] + new + t[m[-1].end():]


def fresh(t, e, token):
    return token in t


def rs_struct(e, attrs):
    out = ["", "/// %s %s" % (e["emoji"], e["doc"])] + attrs + ["pub struct %s {" % e["ent"], "    pub id: String,"]
    for fl in e["rest"]:
        if fl["unit"] in DSL_UNIT and fl["ty"] == "f64":
            out.append('    #[dsl(unit = "%s")]' % DSL_UNIT[fl["unit"]])
        out.append("    pub %s: %s," % (fl["name"], RS_TY[fl["ty"]]))
    out.append("}")
    return "\n".join(out) + "\n"


def layer_model():
    p = SCH + "📸️snapshot/🦀️.rs"

    def go(t):
        if "pub struct %s {" % G["ent"] not in t:
            m = re.search(r"((?:^#\[[^\n]*\n)+)pub struct %s \{\n.*?\n\}\n" % PE, t, re.M | re.S)
            attrs = [l for l in m.group(1).split("\n") if l.startswith("#[")]
            t = t[:m.end()] + "".join(rs_struct(e, attrs) for e in ENTITIES) + t[m.end():]
        fields = "".join("    #[dsl(table)]\n    #[state(artifact)]\n    pub %s: Vec<%s>,\n" % (e["plural"], e["ent"]) for e in ENTITIES)
        t = after_re(t, r"^    pub %s: Vec<%s>,\n" % (PP, PE), fields, "pub %s: Vec<%s>," % (G["plural"], G["ent"]))
        tail = "".join(", %s: Vec::new()" % e["plural"] for e in ENTITIES)
        if "%s: Vec::new()" % G["plural"] not in t:
            t = sub1(t, r"%s: Vec::new\(\) \}" % PP, "%s: Vec::new()" % PP + tail + " }")
        if "%s: vec![]," % G["plural"] not in t:
            t = re.sub(r"^(\s+)%s: vec!\[\],\n" % PP, lambda m: m.group(0) + "".join("%s%s: vec![],\n" % (m.group(1), e["plural"]) for e in ENTITIES), t, flags=re.M)
        return t
    edit(p, go)

    def root(t):
        if "pub %s:" % G["plural"] not in t:
            t = after_re(t, r"^    pub %s: Vec<semio_s_artifact_norm_en1995_model::%s>,\n" % (PP, PE), "".join("    #[state(artifact)]\n    pub %s: Vec<semio_s_artifact_norm_en1995_model::%s>,\n" % (e["plural"], e["ent"]) for e in ENTITIES), "pub %s:" % G["plural"])
            t = sub1(t, r"%s: self\.%s\.clone\(\) \}" % (PP, PP), "%s: self.%s.clone()" % (PP, PP) + "".join(", %s: self.%s.clone()" % (e["plural"], e["plural"]) for e in ENTITIES) + " }")
            t = sub1(t, r"%s: snapshot\.%s \}" % (PP, PP), "%s: snapshot.%s" % (PP, PP) + "".join(", %s: snapshot.%s" % (e["plural"], e["plural"]) for e in ENTITIES) + " }")
        return t
    edit(SCH + "🦀️.rs", root)


def ts_iface(e):
    return "export interface %s {\n  id: string;\n%s}\n" % (e["ent"], "".join("  %s: %s;\n" % (fl["camel"], TS_TY[fl["ty"]]) for fl in e["rest"]))


def ts_parse(e):
    body = ", ".join("%s: normWireRequired(%s)" % (fl["camel"], WIRE[fl["ty"]]) for fl in e["fields"])
    return "export const parse%s: NormWireReader<%s> = normWireObject<%s>({ %s });" % (e["ent"], e["ent"], e["ent"], body)


def layer_twins():
    def root_ts(t):
        t = after_re(t, r"^  %s: \{ \[key: string\]: NormJson \}\[\];\n" % PP, "".join("  %s: { [key: string]: NormJson }[];\n" % e["plural"] for e in ENTITIES), "  %s: {" % G["plural"])
        if "%s: normWireRequired" % G["plural"] not in t:
            t = sub1(t, r"(%s: normWireRequired\(normWireArray\(normWireMap\(normWireJson\)\)\))(,| \})" % PP, lambda m: m.group(1) + "".join(", %s: normWireRequired(normWireArray(normWireMap(normWireJson)))" % e["plural"] for e in ENTITIES) + m.group(2))
        return t
    edit(SCH + "🟦️.ts", root_ts)

    def snap_ts(t):
        t = after_re(t, r"^  %s: %s\[\];\n" % (PP, PE), "".join("  %s: %s[];\n" % (e["plural"], e["ent"]) for e in ENTITIES), "  %s: %s[];" % (G["plural"], G["ent"]))
        if "export interface %s " % G["ent"] not in t:
            t = sub1(t, r"(export interface %s \{.*?\n\}\n)" % PE, lambda m: m.group(1) + "".join("\n" + ts_iface(e) for e in ENTITIES), re.S)
        if "%s: normWireRequired" % G["plural"] not in t:
            t = sub1(t, r"(%s: normWireRequired\(normWireArray\(normWireRef\(\(\) => parse%s\)\)\))(,| \})" % (PP, PE), lambda m: m.group(1) + "".join(", %s: normWireRequired(normWireArray(normWireRef(() => parse%s)))" % (e["plural"], e["ent"]) for e in ENTITIES) + m.group(2))
        if "export const parse%s:" % G["ent"] not in t:
            t = sub1(t, r"(export const parse%s: [^\n]*\n)" % PE, lambda m: m.group(1) + "".join("\n" + ts_parse(e) + "\n" for e in ENTITIES))
        return t
    edit(SCH + "📸️snapshot/🟦️.ts", snap_ts)
    for gp in ("🔗️.graphql", "📸️snapshot/🔗️.graphql"):
        edit(SCH + gp, lambda t: after_re(t, r"^  %s: \[JSON!\]!\n" % PP, "".join("  %s: [JSON!]!\n" % e["plural"] for e in ENTITIES), "  %s:" % G["plural"]))
    for pp in ("🛰️.proto", "📸️snapshot/🛰️.proto"):
        def pr(t):
            n = max(int(x) for x in re.findall(r"= (\d+);", t))
            return after_re(t, r"^  repeated bytes %s = \d+;\n" % PP, "".join("  repeated bytes %s = %d;\n" % (e["plural"], n + 1 + i) for i, e in enumerate(ENTITIES)), "bytes %s" % G["plural"])
        edit(SCH + pp, pr)


def ui(fl):
    if fl["ty"] == "u32":
        d = {"widget": "stepper", "label": {"en": fl["en"], "de": fl["de"]}, "step": 1, "precision": 0}
        return d
    if fl["ty"] == "String":
        if fl["options"]:
            d = {"widget": "select", "options": [o[0] for o in fl["options"]], "label": {"en": fl["en"], "de": fl["de"]}}
        elif fl["ref"]:
            d = {"widget": "reference", "label": {"en": fl["en"], "de": fl["de"]}, "ref": {"kind": fl["ref"]}}
        else:
            d = {"widget": "text", "label": {"en": fl["en"], "de": fl["de"]}}
        return d
    d = {"label": {"en": fl["en"], "de": fl["de"]}, "widget": "stepper"}
    d.update(UNIT_UI[fl["unit"]])
    return d


def json_def(e):
    props = {"id": {"type": "string"}}
    for fl in e["rest"]:
        if fl["ty"] == "u32":
            props[fl["camel"]] = {"type": "integer", "minimum": 0, "x-semio-ui": ui(fl), "maximum": 4294967295}
        elif fl["ty"] == "String":
            props[fl["camel"]] = {"type": "string", "x-semio-ui": ui(fl)}
        else:
            props[fl["camel"]] = {"type": "number", "x-semio-ui": ui(fl)}
    d = {"title": e["ent"], "type": "object", "additionalProperties": False, "required": [fl["camel"] for fl in e["fields"]], "properties": props}
    return d


def layer_json():
    def snap(t):
        data = json.loads(t)
        form, tail = padd.json_form(t)
        if G["plural"] in data["properties"]:
            return t
        dk = "$defs"
        nd = {}
        for k, v in data[dk].items():
            nd[k] = v
            if k == PE:
                for e in ENTITIES:
                    d = json_def(e)
                    if "x-semio-formats" in v:
                        d["x-semio-formats"] = v["x-semio-formats"]
                    nd[e["ent"]] = d
        data[dk] = nd
        props = {}
        for k, v in data["properties"].items():
            props[k] = v
            if k == PP:
                for e in ENTITIES:
                    props[e["plural"]] = {"type": "array", "items": {"$ref": "#/$defs/" + e["ent"]}, "x-semio-state": "artifact"}
        data["properties"] = props
        data["required"] = data["required"] + [e["plural"] for e in ENTITIES]
        return dump_schema(data)
    edit(SCH + "📸️snapshot/🔣️.json", snap)

    def root(t):
        data = json.loads(t)
        form, tail = padd.json_form(t)
        if G["plural"] in data["properties"]:
            return t
        props = {}
        for k, v in data["properties"].items():
            props[k] = v
            if k == PP:
                for e in ENTITIES:
                    props[e["plural"]] = json.loads(json.dumps(v))
        data["properties"] = props
        data["required"] = data["required"] + [e["plural"] for e in ENTITIES]
        return dump_schema(data)
    edit(SCH + "🔣️.json", root)


def patch_rs(e):
    return ", ".join("%s: %s" % (fl["name"], RS_TY[fl["ty"]]) for fl in e["rest"])


def diff_rs_blocks(e):
    E, P = e["ent"], "En1995" + e["ent"]
    key = "/// 🔑️ Key extractor of the foreign `%s` rows (`id`).\n#[derive(Clone, Copy, Debug, PartialEq)]\npub struct %sKey;\n\n" % (E, E)
    keyof = "impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1995_model::%s> for %sKey {\n    type Key = String;\n    fn key_of(row: &semio_s_artifact_norm_en1995_model::%s) -> String {\n        row.id.clone()\n    }\n}\n\n" % (E, E, E)
    patch = "protocol::row_patch! {\n    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]\n    #[cfg_attr(test, serde(rename_all = \"camelCase\"))]\n    /// 🩹 Sparse field patch of one `%s`.\n    pub %sPatch of semio_s_artifact_norm_en1995_model::%s { set { %s } }\n}\n\n" % (E, P, E, patch_rs(e))
    delta = "protocol::list_delta! {\n    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]\n    #[cfg_attr(test, serde(rename_all = \"camelCase\"))]\n    /// 📋️ Positional row delta of one `%s` list.\n    pub %sDelta { removal: %sRemoval, insertion: %sInsertion, relocation: %sRelocation, modification: %sModification, row: semio_s_artifact_norm_en1995_model::%s, patch: %sPatch, list: Vec<semio_s_artifact_norm_en1995_model::%s>, key: String = by %sKey }\n}\n\n" % (E, P, P, P, P, P, E, P, E, E)
    return key, keyof, patch, delta


def layer_diff():
    def rs(t):
        if "pub struct %sKey;" % G["ent"] in t:
            return t
        t = sub1(t, r"(pub struct %sKey;\n\n)" % PE, lambda m: m.group(1) + "".join(diff_rs_blocks(e)[0] for e in ENTITIES))
        t = sub1(t, r"(impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1995_model::%s> for %sKey \{.*?\n\}\n\n)" % (PE, PE), lambda m: m.group(1) + "".join(diff_rs_blocks(e)[1] for e in ENTITIES), re.S)
        t = sub1(t, r"(protocol::row_patch! \{\n(?:[^\n]*\n){3}    pub En1995%sPatch of [^\n]*\n\}\n\n)" % PE, lambda m: m.group(1) + "".join(diff_rs_blocks(e)[2] for e in ENTITIES))
        t = sub1(t, r"(protocol::list_delta! \{\n(?:[^\n]*\n){3}    pub En1995%sDelta [^\n]*\n\}\n\n)" % PE, lambda m: m.group(1) + "".join(diff_rs_blocks(e)[3] for e in ENTITIES))
        t = sub1(t, r"(    pub %s: En1995%sDelta,\n)" % (PP, PE), lambda m: m.group(1) + "".join("    #[state(artifact)]\n    pub %s: En1995%sDelta,\n" % (e["plural"], e["ent"]) for e in ENTITIES))
        t = sub1(t, r"(        self\.%s\.absorb\(other\.%s\);\n)" % (PP, PP), lambda m: m.group(1) + "".join("        self.%s.absorb(other.%s);\n" % (e["plural"], e["plural"]) for e in ENTITIES))
        t = sub1(t, r"(            %s: self\.%s\.commit_onto\(&base\.%s, capability\)\.map_err\(\|error\| error\.under\(\[\"%s\"\]\)\)\?,\n)" % (PP, PP, PP, PP), lambda m: m.group(1) + "".join('            %s: self.%s.commit_onto(&base.%s, capability).map_err(|error| error.under(["%s"]))?,\n' % (e["plural"], e["plural"], e["plural"], e["plural"]) for e in ENTITIES))
        t = sub1(t, r"(            && self\.%s\.is_empty\(\)\n)" % PP, lambda m: m.group(1) + "".join("            && self.%s.is_empty()\n" % e["plural"] for e in ENTITIES))
        t = sub1(t, r"(            %s: self\.%s\.inverse\(&base\.%s\),\n)" % (PP, PP, PP), lambda m: m.group(1) + "".join("            %s: self.%s.inverse(&base.%s),\n" % (e["plural"], e["plural"], e["plural"]) for e in ENTITIES))
        return t
    edit(SCH + "🔺️diff/🦀️.rs", rs)

    def dts(t):
        if "interface En1995%sPatch" % G["ent"] in t:
            return t
        t = sub1(t, r"(, type %s,)" % PE, lambda m: m.group(1) + "".join(" type %s," % e["ent"] for e in ENTITIES)[:-1] + ",")
        t = sub1(t, r"( parse%s,)" % PE, lambda m: m.group(1) + "".join(" parse%s," % e["ent"] for e in ENTITIES))
        t = sub1(t, r"(  %s(\??): En1995%sDelta;\n)" % (PP, PE), lambda m: m.group(1) + "".join("  /** %s @state artifact */\n  %s%s: En1995%sDelta;\n" % (e["emoji"], e["plural"], m.group(2), e["ent"]) for e in ENTITIES))
        ifaces = ""
        for e in ENTITIES:
            P = "En1995" + e["ent"]
            ifaces += "\nexport interface %sPatch {\n%s}\n" % (P, "".join("  %s: %s | null;\n" % (fl["camel"], TS_TY[fl["ty"]]) for fl in e["rest"]))
            ifaces += "export interface %sRemoval {\n  id: string;\n  index: number;\n}\n" % P
            ifaces += "export interface %sInsertion {\n  index: number;\n  row: %s;\n}\n" % (P, e["ent"])
            ifaces += "export interface %sRelocation {\n  id: string;\n  from: number;\n  to: number;\n}\n" % P
            ifaces += "export interface %sModification {\n  id: string;\n  patch: %sPatch;\n}\n" % (P, P)
            ifaces += "export interface %sDelta {\n  removed: %sRemoval[];\n  inserted: %sInsertion[];\n  moved: %sRelocation[];\n  modified: %sModification[];\n}\n" % (P, P, P, P, P)
        t = sub1(t, r"(export interface En1995%sDelta \{\n(?:[^\n]*\n){4}\}\n)" % PE, lambda m: m.group(1) + ifaces)
        t = sub1(t, r"(%s: normWire(Optional|Required)\(normWireRef\(\(\) => parseEn1995%sDelta\)\),)" % (PP, PE), lambda m: m.group(1) + "".join(" %s: normWire%s(normWireRef(() => parseEn1995%sDelta))," % (e["plural"], m.group(2), e["ent"]) for e in ENTITIES))
        parses = ""
        nn = lambda w: "normWireRequired(normWireRange(normWireInteger, {\"minimum\":0}))"
        for e in ENTITIES:
            P = "En1995" + e["ent"]
            pf = ", ".join("%s: normWireDefault(normWireNullable(%s), () => null)" % (fl["camel"], WIRE[fl["ty"]]) for fl in e["rest"])
            parses += "\nexport const parse%sPatch: NormWireReader<%sPatch> = normWireObject<%sPatch>({ %s });\n" % (P, P, P, pf)
            parses += "\nexport const parse%sRemoval: NormWireReader<%sRemoval> = normWireObject<%sRemoval>({ id: normWireRequired(normWireString), index: %s });\n" % (P, P, P, nn(0))
            parses += "\nexport const parse%sInsertion: NormWireReader<%sInsertion> = normWireObject<%sInsertion>({ index: %s, row: normWireRequired(parse%s) });\n" % (P, P, P, nn(0), e["ent"])
            parses += "\nexport const parse%sRelocation: NormWireReader<%sRelocation> = normWireObject<%sRelocation>({ id: normWireRequired(normWireString), from: %s, to: %s });\n" % (P, P, P, nn(0), nn(0))
            parses += "\nexport const parse%sModification: NormWireReader<%sModification> = normWireObject<%sModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parse%sPatch)) });\n" % (P, P, P, P)
            parses += "\nexport const parse%sDelta: NormWireReader<%sDelta> = normWireObject<%sDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parse%sRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parse%sInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parse%sRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parse%sModification))) });\n" % (P, P, P, P, P, P, P)
        t = sub1(t, r"(export const parseEn1995%sDelta: [^\n]*\n)" % PE, lambda m: m.group(1) + parses)
        return t
    edit(SCH + "🔺️diff/🟦️.ts", dts)
    def dgql(t):
        bang = re.search(r"^  %s: JSON(!?) @state\(class: ARTIFACT\)\n" % PP, t, re.M).group(1)
        return after_re(t, r"^  %s: JSON!? @state\(class: ARTIFACT\)\n" % PP, "".join("  %s: JSON%s @state(class: ARTIFACT)\n" % (e["plural"], bang) for e in ENTITIES), "  %s:" % G["plural"])
    edit(SCH + "🔺️diff/🔗️.graphql", dgql)

    def dproto(t):
        n = max(int(x) for x in re.findall(r"= (\d+);", t))
        return after_re(t, r"^  bytes %s = \d+;\n" % PP, "".join("  // @state artifact\n  bytes %s = %d;\n" % (e["plural"], n + 1 + i) for i, e in enumerate(ENTITIES)), "bytes %s" % G["plural"])
    edit(SCH + "🔺️diff/🛰️.proto", dproto)

    def djson(t):
        data = json.loads(t)
        form, tail = padd.json_form(t)
        if G["plural"] in data["properties"]:
            return t
        D = data["$defs"]
        nd = {}
        for k, v in D.items():
            nd[k] = v
            if k == "En1995%sDelta" % PE:
                for e in ENTITIES:
                    P = "En1995" + e["ent"]
                    patch_props = {fl["camel"]: {"anyOf": [{"$ref": "%s#/$defs/%s/properties/%s" % (SNAP_URL, e["ent"], fl["camel"])}, {"type": "null"}]} for fl in e["rest"]}
                    for suffix in ("Patch", "Removal", "Insertion", "Relocation", "Modification", "Delta"):
                        d = json.loads(json.dumps(D["En1995" + PE + suffix]).replace("En1995" + PE, P).replace("snapshot.json#/$defs/" + PE, "snapshot.json#/$defs/" + e["ent"]))
                        if suffix == "Patch":
                            d["properties"] = patch_props
                        nd[P + suffix] = d
        data["$defs"] = nd
        props = {}
        for k, v in data["properties"].items():
            props[k] = v
            if k == PP:
                for e in ENTITIES:
                    props[e["plural"]] = {"$ref": "#/$defs/En1995%sDelta" % e["ent"], "x-semio-state": "artifact"}
        data["properties"] = props
        return dump_schema(data)
    edit(SCH + "🔺️diff/🔣️.json", djson)


def layer_inference():
    def outline(t):
        if '"%s"' % G["plural"] in t:
            return t
        t = sub1(t, r'"%s"\]' % PP, '"%s"' % PP + "".join(', "%s"' % e["plural"] for e in ENTITIES) + "]")
        return sub1(t, r"snapshot\.%s\.len\(\)\)" % PP, "snapshot.%s.len()" % PP + "".join(" + snapshot.%s.len()" % e["plural"] for e in ENTITIES) + ")")
    edit(SCH + "💡️inferences/🧾outline/🦀️.rs", outline)

    def integrity(t):
        if "push_duplicate_ids(report, annex, \"%s\"" % G["plural"] in t:
            return t
        t = sub1(t, r"(    push_duplicate_ids\(report, annex, \"%s\", [^\n]*\n)" % PP, lambda m: m.group(1) + "".join('    push_duplicate_ids(report, annex, "%s", &%s.iter().map(|p| p.id.clone()).collect::<Vec<_>>(), &|id| format!("%s[id={id}].id"));\n' % (e["plural"], e["plural"], e["plural"]) for e in ENTITIES))
        t = sub1(t, r"%s: &\[%s\]\) \{" % (PP, PE), "%s: &[%s], " % (PP, PE) + ", ".join("%s: &[%s]" % (e["plural"], e["ent"]) for e in ENTITIES) + ") {")
        t = sub1(t, r"&document\.%s\);" % PP, "&document.%s, " % PP + ", ".join("&document.%s" % e["plural"] for e in ENTITIES) + ");")
        return t
    edit(SCH + "⚖️timber/🏗️structure-and-members/🦀️.rs", integrity)


def meta_rows(e):
    rows = ['    ("%s", FieldSpec::new("%s", "%s")),' % (e["plural"], e["label_en"], e["label_de"]), '    ("%s[].id", FieldSpec::new("%s", "%s")),' % (e["plural"], e["id_en"], e["id_de"])]
    for fl in e["rest"]:
        path = "%s[].%s" % (e["plural"], fl["camel"])
        if fl["options"]:
            rows.append('    ("%s", FieldSpec::choice("%s", "%s", %s)),' % (path, fl["en"], fl["de"], choices_const(e, fl)))
        elif fl["unit"] in FIELD_META_UNIT and fl["ty"] == "f64":
            rows.append('    ("%s", FieldSpec::quantity("%s", "%s", %s)),' % (path, fl["en"], fl["de"], FIELD_META_UNIT[fl["unit"]]))
        else:
            rows.append('    ("%s", FieldSpec::new("%s", "%s")),' % (path, fl["en"], fl["de"]))
    return rows


def choices_const(e, fl):
    return "%s_%s" % (e["table"].upper(), fl["name"].upper())


def layer_meta():
    p = ROOT + "✏️editor/🏷️field-meta/🦀️.rs"

    def go(t):
        if '("%s",' % G["plural"] in t:
            return t
        consts = ""
        for e in ENTITIES:
            for fl in e["rest"]:
                if fl["options"]:
                    consts += "const %s: &[NormFieldChoice] = &[\n%s];\n" % (choices_const(e, fl), "".join('    choice("%s", "%s", "%s"),\n' % o for o in fl["options"]))
        t = sub1(t, r"(const ANNEX: &\[NormFieldChoice\])", lambda m: consts + m.group(1))
        rows = [r for e in ENTITIES for r in meta_rows(e)]
        lines = t.split("\n")
        idx = [i for i, l in enumerate(lines) if l.startswith('    ("%s[]' % PP) or l.startswith('    ("%s",' % PP)]
        assert idx and idx == list(range(idx[0], idx[-1] + 1)), idx
        return "\n".join(lines[:idx[-1] + 1] + rows + lines[idx[-1] + 1:])
    edit(p, go)


DDL_ORDER = ("id INTEGER PRIMARY KEY", "document_id INTEGER NOT NULL REFERENCES en1995_document(id)", "ordinal INTEGER NOT NULL CHECK (ordinal >= 0)")


def ddl(e):
    s = "CREATE TABLE %s (\n  id INTEGER PRIMARY KEY,\n  document_id INTEGER NOT NULL REFERENCES en1995_document(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  logical_id TEXT NOT NULL,\n" % e["tbl"]
    cols = []
    for fl in e["rest"]:
        if fl["ty"] == "String":
            cols.append("%s TEXT NOT NULL" % fl["name"])
        elif fl["ty"] == "u32":
            cols.append("%s INTEGER NOT NULL CHECK (%s BETWEEN 0 AND 4294967295)" % (fl["name"], fl["name"]))
        else:
            cols.append("%s REAL" % fl["name"])
    s += "  " + ",\n  ".join(cols) + ",\n"
    s += ",\n".join("  %s_ieee754_bits INTEGER NOT NULL, %s_ieee754_class TEXT NOT NULL" % (fl["name"], fl["name"]) for fl in e["rest"] if fl["ty"] == "f64") + "\n);\n"
    return s


def schema_bytes(text):
    total = 0
    for s in text.split(";"):
        s = s.strip()
        if s:
            total += len(s.encode()) + len(s[len("CREATE TABLE "):].split("(")[0].strip().encode())
    return total


def layer_sqlite():
    def sql(t):
        if "en1995_%s" % G["table"] in t:
            return t
        return t.rstrip("\n") + "\n" + "".join(ddl(e) for e in ENTITIES)
    edit(Q + "🗄️.sql", sql)
    total = N0 + len(ENTITIES)

    def cell(e, fl, var):
        if fl["ty"] == "String":
            return "Cell::Text(&%s.%s)" % (var, fl["name"])
        if fl["ty"] == "u32":
            return "Cell::Integer(i64::from(%s.%s))" % (var, fl["name"])
        return "Cell::Real(%s.%s)" % (var, fl["name"])

    def rs(t):
        if G["const"] in t:
            return t
        t = sub1(t, r"\b%s, MemberRole" % PE, "%s, " % PE + ", ".join(e["ent"] for e in ENTITIES) + ", MemberRole")
        t = sub1(t, r"(const PLACEMENT_FLOATS:[^\n]*\n)", lambda m: m.group(1) + "".join("const %s:&[FloatColumn]=&[%s];\n" % (e["const"], ",".join("FloatColumn::Binary64(%d)" % i for i in e["floats"])) for e in ENTITIES))

        def writes(m):
            out = ""
            for e in ENTITIES:
                v = e["table"]
                out += '        for(position,%s)in self.%s.iter().enumerate(){out.insert_float("%s",&[Cell::Integer(1),ordinal(position)?,Cell::Text(&%s.id),%s],%s)?;}\n' % (v, e["plural"], e["tbl"], v, ",".join(cell(e, fl, v) for fl in e["rest"]), e["const"])
            return m.group(1) + out + m.group(2)
        t = sub1(t, r'(for\(position,%s\)in self\.%s\.iter\(\)\.enumerate\(\)\{out\.insert_float\("%s",[^\n]*\n)(        Ok\(\(\)\))' % (PREV["table"], PP, PT), writes)
        t, hits = re.subn(r"SQLITE_SCHEMA,\d+,70", "SQLITE_SCHEMA,%d,70" % total, t)
        assert hits >= 1
        t = sub1(t, r"self\.%s\.len\(\)\]" % PP, "self.%s.len()" % PP + "".join(",self.%s.len()" % e["plural"] for e in ENTITIES) + "]")
        t = sub1(t, r"(        for %s in &self\.%s\{bound\.add\(4096\)\?;[^\n]*\n)" % (PREV["table"], PP), lambda m: m.group(1) + "".join("        for %s in &self.%s{bound.add(4096)?;for text in [&%s.id%s]{bound.repeated(text.len(),24)?;}}\n" % (e["table"], e["plural"], e["table"], "".join(",&%s.%s" % (e["table"], fl["name"]) for fl in e["rest"] if fl["ty"] == "String")) for e in ENTITIES))

        def reads(m):
            out = ""
            for e in ENTITIES:
                parts = []
                for i, fl in enumerate(e["rest"]):
                    c = 4 + i
                    if fl["ty"] == "String":
                        parts.append("%s:reconstruct_text(control,row.text(%d)?)?" % (fl["name"], c))
                    elif fl["ty"] == "u32":
                        parts.append("%s:read_u32(row,%d)?" % (fl["name"], c))
                    else:
                        parts.append("%s:row.real(%d)?" % (fl["name"], c))
                out += '        let mut %s=Vec::new();for row in ordered(entities(database,"%s",%d,%s,control)?.into_values(),1,control)?{checkpoint(control,%s.len(),0)?;%s.push(%s{id:reconstruct_text(control,row.text(3)?)?,%s});}\n' % (e["plural"], e["tbl"], e["cols"], e["const"], e["plural"], e["plural"], e["ent"], ",".join(parts))
            return m.group(1) + out
        t = sub1(t, r'(        let mut %s=Vec::new\(\);[^\n]*\n)' % PP, reads)
        t = sub1(t, r",%s\};" % PP, ",%s" % PP + "".join(",%s" % e["plural"] for e in ENTITIES) + "};")
        return t
    edit(Q + "🦀️.rs", rs)

    def ts(t):
        if G["const"] in t:
            return t
        t = sub1(t, r"(import type\{En1995Snapshot[^\n]*?,%s)(?=[,}])" % PE, lambda m: m.group(1) + "".join(",%s" % e["ent"] for e in ENTITIES))
        types = ""
        for e in ENTITIES:
            fl64 = [fl for fl in e["rest"] if fl["ty"] == "f64"]
            types += 'export type En1995Sqlite%s=Omit<%s,%s>&{%s};\n' % (e["ent"], e["ent"], "|".join('"%s"' % fl["camel"] for fl in fl64), ";".join("%s:Binary64" % fl["camel"] for fl in fl64))
        t = sub1(t, r"(export type En1995SqlitePlacement=[^\n]*\n)", lambda m: m.group(1) + types)
        t = sub1(t, r'\|"%s">&\{' % PP, '|"%s"' % PP + "".join('|"%s"' % e["plural"] for e in ENTITIES) + ">&{")
        t = sub1(t, r"%s:En1995Sqlite%s\[\]\};" % (PP, PE), "%s:En1995Sqlite%s[]" % (PP, PE) + "".join(";%s:En1995Sqlite%s[]" % (e["plural"], e["ent"]) for e in ENTITIES) + "};")
        t = sub1(t, r"(const PLACEMENT_FLOATS:[^\n]*\n)", lambda m: m.group(1) + "".join("const %s:readonly Ieee754Column[]=[%s].map(index=>({index,width:64}));\n" % (e["const"], ",".join(str(i) for i in e["floats"])) for e in ENTITIES))

        def val(fl, var):
            return "unsigned(%s.%s,4294967295)" % (var, fl["camel"]) if fl["ty"] == "u32" else "%s.%s" % (var, fl["camel"])

        def writes(m):
            out = ""
            for e in ENTITIES:
                out += ' for(let position=0;position<snapshot.%s.length;position++){const p=snapshot.%s[position]!;await write(out,"%s",[1n,BigInt(position),p.id,%s],%s,options)}\n' % (e["plural"], e["plural"], e["tbl"], ",".join(val(fl, "p") for fl in e["rest"]), e["const"])
            return m.group(1) + out + m.group(2)
        t = sub1(t, r'(await write\(out,"%s",[^\n]*\n)( return out\.finish\(\);)' % PT, writes)

        def reads(m):
            out = ""
            for i, e in enumerate(ENTITIES):
                parts = []
                for j, fl in enumerate(e["rest"]):
                    c = 4 + j
                    if fl["ty"] == "String":
                        parts.append("%s:artifactSqliteText(r,%d)" % (fl["camel"], c))
                    elif fl["ty"] == "u32":
                        parts.append("%s:readUnsigned(r,%d,4294967295n)" % (fl["camel"], c))
                    else:
                        parts.append("%s:readBinary64(r,%d,%s)" % (fl["camel"], c, e["const"]))
                out += ' const %sRows=await entities(tables[%d]!,%d,%s,options),%s:En1995Sqlite%s[]=[];for(const r of await ordered(%sRows.values(),1n,options)){await checkpoint(options,%s.length,0);%s.push({id:artifactSqliteText(r,3),%s})}\n' % (e["plural"], N0 + i, e["cols"], e["const"], e["plural"], e["ent"], e["plural"], e["plural"], e["plural"], ",".join(parts))
            return m.group(1) + out + m.group(2)
        t = sub1(t, r"(%s\.push\(\{id:artifactSqliteText\(r,3\)[^\n]*\n)( const result:En1995SqliteSnapshot=)" % PP, reads)
        t = sub1(t, r",%s\};await" % PP, ",%s" % PP + "".join(",%s" % e["plural"] for e in ENTITIES) + "};await")
        return t
    edit(Q + "🟦️.ts", ts)

    def adm(t):
        if "en1995_%s" % G["table"] in t:
            return t
        t = sub1(t, r"(rows=n::add\(rows,n::list\(n::field\(root,%d\)\?\)\?\.len\(\)\)\?;)Ok\(rows\)" % ROOTMAX, lambda m: m.group(1) + "".join("rows=n::add(rows,n::list(n::field(root,%d)?)?.len())?;" % (ROOTMAX + 1 + i) for i in range(len(ENTITIES))) + "Ok(rows)")

        def cols(e):
            out = ["Text(0)"]
            for i, fl in enumerate(e["rest"]):
                k = i + 1
                out.append("Text(%d)" % k if fl["ty"] == "String" else ("Unsigned(%d,u32::MAX as u64)" % k if fl["ty"] == "u32" else "Real(%d)" % k))
            return ",".join(out)
        ent = "".join(' for(index,p)in n::list(n::field(root,%d)?)?.iter().enumerate(){n::entity(out,"%s",1,index,&n::cells(n::record(p)?,[%s])?,%s)?;}\n' % (ROOTMAX + 1 + i, e["tbl"], cols(e), e["const"]) for i, e in enumerate(ENTITIES))
        t = sub1(t, r'(\n for\(index,p\)in n::list\(n::field\(root,%d\)\?\)\?\.iter\(\)\.enumerate\(\)\{n::entity\(out,"%s",[^\n]*\n)( Ok\(\(\)\))' % (ROOTMAX, PT), lambda m: m.group(1) + ent + m.group(2))
        t = sub1(t, r"SQLITE_SCHEMA,\d+,70", "SQLITE_SCHEMA,%d,70" % total)
        t = re.sub(r"Exact [a-z-]+-table", "Exact %s-table" % WORDS[total], t)
        return t
    edit(Q + "🛂️admission/🦀️.rs", adm)

    def rtests(t):
        if "snapshot.%s" % G["plural"] in t:
            return t
        sets = "".join("    for p in &mut snapshot.%s{%s}\n" % (e["plural"], "".join("p.%s=value;" % fl["name"] for fl in e["rest"] if fl["ty"] == "f64")) for e in ENTITIES)
        t = sub1(t, r"(    for p in &mut snapshot\.placements\{[^\n]*\n)", lambda m: m.group(1) + sets)
        t = sub1(t, r'\("tables",\d+\)', '("tables",%d)' % total)
        t = sub1(t, r"database\(&empty\)\.tables\.len\(\),\d+", "database(&empty).tables.len(),%d" % total)
        t = sub1(t, r"empty\.placements\.clear\(\);", "empty.placements.clear();" + "".join("empty.%s.clear();" % e["plural"] for e in ENTITIES))
        return t
    edit(Q + "🧪️tests/🦀️.rs", rtests)

    def ttests(t):
        if "%s:" % G["plural"] in t:
            return t
        maps = ""
        for e in ENTITIES:
            fl64 = [fl for fl in e["rest"] if fl["ty"] == "f64"]
            maps += "%s:f.%s.map(p=>({...p,%s}))," % (e["plural"], e["plural"], ",".join("%s:binary64(p.%s)" % (fl["camel"], fl["camel"]) for fl in fl64))
        t = sub1(t, r"(placements:f\.placements\.map\(p=>\(\{\.\.\.p,x:binary64\(p\.x\),y:binary64\(p\.y\),z:binary64\(p\.z\),yaw:binary64\(p\.yaw\)\}\)\),)", lambda m: m.group(1) + maps)
        sets = "".join(" for(const p of s.%s){%s}\n" % (e["plural"], ";".join("p.%s=v" % fl["camel"] for fl in e["rest"] if fl["ty"] == "f64")) for e in ENTITIES)
        t = sub1(t, r"( for\(const p of s\.placements\)\{p\.x=v;p\.y=v;p\.z=v;p\.yaw=v\}\n)", lambda m: m.group(1) + sets)
        t = sub1(t, r"database\.tables\.length\)\.toBe\(\d+\)", "database.tables.length).toBe(%d)" % total)
        return t
    edit(Q + "🧪️tests/🟦️.ts", ttests)

    new_bytes = schema_bytes(rd(Q + "🗄️.sql") if not DRY else rd(Q + "🗄️.sql") + "".join(ddl(e) for e in ENTITIES if "en1995_%s" % e["table"] not in rd(Q + "🗄️.sql")))
    widths = {e["tbl"]: e["width"] for e in ENTITIES}

    def sj(t):
        d = json.loads(t)
        d["schemaBytes"] = new_bytes
        d["tableWidths"].update(widths)
        d["tableWidths"] = dict(sorted(d["tableWidths"].items()))
        return json.dumps(d, indent=2, ensure_ascii=False) + "\n"
    edit(Q + "🧪️tests/🧫️fixtures/🎛️semantic.json", sj)

    def tt(t):
        t = re.sub(r'(normSemanticContract\["schemaBytes"\]\)\.toEqual\()\d+', lambda m: m.group(1) + str(new_bytes), t)
        return re.sub(r'("tableWidths"\]\)\.toEqual\(\{)([^}]*)\}', lambda m: m.group(1) + ",".join(sorted(set(m.group(2).split(",")) | {'"%s":%d' % (k, v) for k, v in widths.items()})) + "}", t)
    edit(Q + "🧪️tests/🟦️.ts", tt)


def fixture_sweep():
    n = 0
    empty_delta = padd.EMPTY_DELTA
    pat = re.compile(r'"%s"\s*:\s*' % PP)
    for dp, ds, fs in os.walk(ROOT):
        ds[:] = [d for d in ds if d not in ("target", "node_modules")]
        rel = dp.replace(ROOT, "")
        if rel.startswith("🧬️schema") and "🧬️mutations" not in rel:
            continue
        if rel.startswith("🧬️schema/🧬️mutations") and "🧫️fixtures" not in rel:
            continue
        for fn in fs:
            if not fn.endswith(".json"):
                continue
            p = os.path.join(dp, fn)
            t = rd(p)
            if '"%s"' % PP not in t or '"%s"' % G["plural"] in t:
                continue
            out, pos, hits = [], 0, 0
            for m in pat.finditer(t):
                if m.start() < pos:
                    continue
                s = m.end()
                e = padd.value_end(t, s)
                val = t[s:e]
                if val.lstrip().startswith("["):
                    new = "[]"
                elif val.lstrip().startswith("{") and '"removed"' in val:
                    new = empty_delta
                else:
                    continue
                ls = t.rfind("\n", 0, m.start()) + 1
                lead = t[ls:m.start()]
                indent = lead if lead.strip() == "" else None
                sep = (",\n" + indent) if indent is not None else ", "
                colon = ": " if ": " in t[m.start():s] else ":"
                out.append(t[pos:e])
                for ent in ENTITIES:
                    out.append('%s"%s"%s%s' % (sep, ent["plural"], colon, new))
                pos = e
                hits += 1
            if hits:
                out.append(t[pos:])
                nt = "".join(out)
                json.loads(nt)
                n += 1
                if not DRY:
                    open(p, "w", encoding="utf-8").write(nt)
    print("  fixtures updated:", n)


if __name__ == "__main__":
    if not NEW:
        print("nothing to add: every collection of spec.py is present")
        sys.exit(0)
    print("adding:", ", ".join(e["plural"] for e in NEW), "after", PP)
    layer_model()
    layer_twins()
    layer_json()
    layer_diff()
    layer_inference()
    layer_meta()
    layer_sqlite()
    fixture_sweep()
