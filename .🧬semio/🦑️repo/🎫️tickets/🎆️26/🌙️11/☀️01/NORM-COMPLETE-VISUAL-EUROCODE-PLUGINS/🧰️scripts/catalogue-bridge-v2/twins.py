"""Case twins for the snapshot, root and diff schema facets: json (format-preserving), ts, graphql and proto, taken from the EN 1995 definitions."""
import json, re
from lib import *
import jsonspan as J

NAMES = ["CaseNumber", "CaseText", "CaseRow"]
REFS = ("https://json.schemas.assets.semio-tech.com/s/norm/%s/snapshot.json#/$defs/", "#/$defs/")


def tmpl_defs(art):
    d = json.loads(rd(T.snap + "/🔣️.json"))["$defs"]
    out = {}
    for n in NAMES:
        s = json.dumps(d[n], ensure_ascii=False).replace("EN 1995", "EN " + art.short[2:]).replace("EN-1995", "EN-" + art.short[2:])
        out[n] = json.loads(s)
    return out


def snapshot_json(art, formats=None, defs_key="$defs", state=True):
    p = art.snap + "/🔣️.json"
    t = rd(p)
    j = json.loads(t)
    assert "cases" not in j["properties"], "cases already in " + p
    prop = {"type": "array", "items": {"$ref": "#/$defs/CaseRow"}}
    if state:
        prop["x-semio-state"] = "artifact"
    prop["x-semio-ui"] = {"label": {"en": "Design cases", "de": "Bemessungsfälle"}}
    t = J.add_member(t, ["properties"], "cases", prop)
    t = J.add_item(t, ["required"], "cases")
    defs = tmpl_defs(art)
    if formats:
        for d in defs.values():
            d["x-semio-formats"] = formats
    for n in NAMES:
        t = J.add_member(t, [defs_key], n, defs[n])
    json.loads(t)
    wr(p, t)


def root_json(art):
    p = art.schema + "/🔣️.json"
    t = rd(p)
    t = J.add_member(t, ["properties"], "cases", {"type": "array", "items": {"type": "object"}, "x-semio-state": "artifact"})
    t = J.add_item(t, ["required"], "cases")
    json.loads(t)
    wr(p, t)


def ts_blocks():
    s = rd(T.snap + "/🟦️.ts")
    i0 = s.index("export interface CaseNumber")
    i1 = s.index("export interface TimberConnection")
    ifaces = s[i0:i1].rstrip("\n") + "\n"
    parsers = [l for l in s.splitlines() if re.match(r"export const parseCase(Number|Text|Row):", l)]
    return ifaces, parsers


def snapshot_ts(art, struct, iface_after=None):
    """Adds `cases: CaseRow[]` to the `<struct>` interface, the three row interfaces after it and the three parsers after the struct parser (before it when the parser references them lazily)."""
    p = art.snap + "/🟦️.ts"
    s = rd(p)
    ifaces, parsers = ts_blocks()
    m = re.search(r"export interface %s \{\n(.*?)\n\}\n" % struct, s, re.S)
    assert m, struct
    s = s[: m.end() - 2] + "\n  /** 🗂️ @state artifact */\n  cases: CaseRow[];\n}\n\n" + ifaces.rstrip("\n") + "\n" + s[m.end():]
    m = re.search(r"^export const parse%s: .*?\}\);$" % struct, s, re.M)
    assert m, "parser " + struct
    line = m.group(0)
    new = line[: -3] + ", cases: normWireRequired(normWireArray(normWireRef(() => parseCaseRow))) });"
    s = s.replace(line, new, 1) + ""
    s = s.replace(new, new + "\n" + "\n".join(parsers), 1)
    s = ensure_imports(s, ["normWireArray", "normWireRef", "normWireRequired", "normWireString", "normWireNumber", "normWireObject"])
    wr(p, s)


def ensure_imports(s, names):
    m = re.search(r"import \{([^}]*)\} from \"([^\"]*(?:norm-js/registry/contract|contract/🟦️\.ts))\";", s)
    assert m, "contract import missing"
    have = {x.strip() for x in m.group(1).split(",") if x.strip()}
    missing = [n for n in names if n not in have and ("type " + n) not in have]
    if not missing:
        return s
    new = "import { " + ", ".join(sorted(have | set(missing), key=lambda x: x.replace("type ", "").lower())) + " } from \"" + m.group(2) + "\";"
    return s.replace(m.group(0), new, 1)


def root_ts(art, struct):
    p = art.schema + "/🟦️.ts"
    s = rd(p)
    m = re.search(r"export interface %s \{\n(.*?)\n\}\n" % struct, s, re.S)
    s = s[: m.end() - 2] + "\n  /** 🗂️ @state artifact */\n  cases: { [key: string]: NormJson }[];\n}\n" + s[m.end():]
    m = re.search(r"^export const parse%s: .*?\}\);$" % struct, s, re.M)
    line = m.group(0)
    s = s.replace(line, line[:-3] + ", cases: normWireRequired(normWireArray(normWireMap(normWireJson))) });", 1)
    wr(p, s)


GQL_ROWS = """type CaseNumber {
  id: String!
  value: Float!
}

type CaseText {
  id: String!
  value: String!
}

type CaseRow {
  id: String!
  labelEn: String!
  labelDe: String!
  entryId: String!
  numbers: [CaseNumber!]!
  texts: [CaseText!]!
}

"""
PROTO_ROWS = """message CaseNumber {
  string id = 1;
  double value = 2;
}

message CaseText {
  string id = 1;
  string value = 2;
}

message CaseRow {
  string id = 1;
  string label_en = 2;
  string label_de = 3;
  string entry_id = 4;
  repeated CaseNumber numbers = 5;
  repeated CaseText texts = 6;
}

"""


def graphql_typed(path, struct):
    s = rd(path)
    i = s.index("type %s {" % struct)
    s = s[:i] + GQL_ROWS + s[i:]
    j = s.index("\n}", s.index("type %s {" % struct))
    s = s[:j] + "\n  cases: [CaseRow!]!" + s[j:]
    wr(path, s)


def graphql_json(path, struct):
    s = rd(path)
    j = s.index("\n}", s.index("type %s {" % struct))
    wr(path, s[:j] + "\n  cases: [JSON!]!" + s[j:])


def proto_typed(path, struct):
    s = rd(path)
    i = s.index("message %s {" % struct)
    s = s[:i] + PROTO_ROWS + s[i:]
    a = s.index("message %s {" % struct)
    j = s.index("\n}", a)
    n = max(int(x) for x in re.findall(r"= (\d+);", s[a:j])) + 1
    wr(path, s[:j] + "\n  repeated CaseRow cases = %d;" % n + s[j:])


def proto_bytes(path, struct):
    s = rd(path)
    a = s.index("message %s {" % struct)
    j = s.index("\n}", a)
    n = max(int(x) for x in re.findall(r"= (\d+);", s[a:j])) + 1
    wr(path, s[:j] + "\n  repeated bytes cases = %d;" % n + s[j:])


SH = NORM + "/⚖️compliance/🧰️design-case/🧬️schema"
DELTA = ["CaseNumberPatch", "CaseNumberRemoval", "CaseNumberInsertion", "CaseNumberRelocation", "CaseNumberModification", "CaseNumberDelta", "CaseTextPatch", "CaseTextRemoval", "CaseTextInsertion", "CaseTextRelocation", "CaseTextModification", "CaseTextDelta", "CaseRowPatch", "CaseRowRemoval", "CaseRowInsertion", "CaseRowRelocation", "CaseRowModification", "CaseRowDelta"]


def diff_json(art, bare=True):
    p = art.diff + "/🔣️.json"
    t = rd(p)
    shared = json.loads(rd(SH + "/🔣️.json"))["definitions"]
    snap = "https://json.schemas.assets.semio-tech.com/s/norm/%s/snapshot.json#/$defs/" % art.short
    t = J.add_member(t, ["properties"], "cases", {"$ref": "#/$defs/CaseRowDelta", "x-semio-state": "artifact"})
    for n in DELTA:
        d = json.loads(json.dumps(shared[n], ensure_ascii=False).replace("#/definitions/Case", snap + "Case"))
        for k, v in list(d.items()):
            pass
        d["x-semio-formats"] = ["🔣️jsonschema", "🟦️typescript"]
        d.setdefault("title", n)
        t = J.add_member(t, ["$defs"], n, d, compact=True)
    # internal refs between delta defs point at siblings in the same $defs
    t = t.replace(snap + "CaseNumberDelta", "#/$defs/CaseNumberDelta").replace(snap + "CaseTextDelta", "#/$defs/CaseTextDelta")
    for n in DELTA:
        t = t.replace(snap + n, "#/$defs/" + n)
    json.loads(t)
    wr(p, t)


def compact_iface(block):
    return re.sub(r"\{\n((?:  .*\n)+)\}", lambda m: "{ " + " ".join(l.strip() for l in m.group(1).splitlines()) + " }", block)


def diff_ts(art, struct):
    p = art.diff + "/🟦️.ts"
    s = rd(p)
    sh = rd(SH + "/🟦️.ts")
    i0 = sh.index("export interface CaseNumberPatch")
    i1 = sh.index("export const parseInputKind")
    ifaces = compact_iface(sh[i0:i1].rstrip("\n") + "\n")
    ifaces = re.sub(r"\n\n(?=export interface)", "\n", ifaces)
    parsers = [l for l in sh.splitlines() if re.match(r"export const parseCase(Number|Text|Row)(Patch|Removal|Insertion|Relocation|Modification|Delta) ", l)]
    m = re.search(r"export interface %s \{\n(.*?)\n\}\n" % struct, s, re.S)
    assert m, struct
    s = s[: m.end() - 2] + "\n  /** 🗂️ @state artifact */\n  cases?: CaseRowDelta;\n}\n\n" + ifaces.rstrip("\n") + "\n" + s[m.end():]
    m = re.search(r"^export const parse%s: .*?\}\);$" % struct, s, re.M)
    assert m, "parser"
    line = m.group(0)
    new = line[:-3] + ", cases: normWireOptional(normWireRef(() => parseCaseRowDelta)) });"
    s = s.replace(line, new + "\n" + "\n".join(parsers), 1)
    im = re.search(r"import \{([^}]*)\} from \"\.\./📸️snapshot/🟦️\.ts\";", s)
    have = [x.strip() for x in im.group(1).split(",") if x.strip()]
    for n in ["type CaseNumber", "type CaseText", "type CaseRow", "parseCaseNumber", "parseCaseText", "parseCaseRow"]:
        if n not in have:
            have.append(n)
    s = s.replace(im.group(0), "import { " + ", ".join(have) + " } from \"../📸️snapshot/🟦️.ts\";", 1)
    s = ensure_imports(s, ["normWireArray", "normWireDefault", "normWireInteger", "normWireNullable", "normWireNumber", "normWireObject", "normWireOptional", "normWireRange", "normWireRef", "normWireRequired", "normWireString"])
    wr(p, s)


def diff_graphql(art, struct, typed=True):
    p = art.diff + "/🔗️.graphql"
    s = rd(p)
    if not typed:
        j = s.index("\n}", s.index("type %s {" % struct))
        wr(p, s[:j] + "\n  cases: JSON @state(class: ARTIFACT)" + s[j:])
        return
    sh = rd(SH + "/🔗️.graphql")
    block = sh[sh.index("type CaseNumberPatch"):]
    a = s.index("type %s {" % struct)
    j = s.index("\n}", a)
    s = s[:j] + "\n  cases: CaseRowDelta @state(class: ARTIFACT)" + s[j:]
    wr(p, s.rstrip("\n") + "\n\n" + block)


def diff_proto(art, struct, typed=True):
    p = art.diff + "/🛰️.proto"
    s = rd(p)
    a = s.index("message %s {" % struct)
    j = s.index("\n}", a)
    n = max(int(x) for x in re.findall(r"= (\d+);", s[a:j])) + 1
    if not typed:
        wr(p, s[:j] + "\n  bytes cases = %d;" % n + s[j:])
        return
    sh = rd(SH + "/🛰️.proto")
    block = sh[sh.index("message CaseNumberPatch"):]
    s = s[:j] + "\n  optional CaseRowDelta cases = %d;" % n + s[j:]
    wr(p, s.rstrip("\n") + "\n\n" + block)
