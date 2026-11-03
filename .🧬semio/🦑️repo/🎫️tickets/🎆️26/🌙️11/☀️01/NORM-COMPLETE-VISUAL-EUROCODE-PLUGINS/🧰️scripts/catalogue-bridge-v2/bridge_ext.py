"""Fork-local twin variants: typed root twins (json `definitions`, typed ts) and path-parametrised injectors."""
import json, re
from lib import *
import jsonspan as J
import twins as W


def ts_typed(path, struct):
    s = rd(path)
    ifaces, parsers = W.ts_blocks()
    m = re.search(r"export interface %s \{\n(.*?)\n\}\n" % struct, s, re.S)
    assert m, struct
    s = s[: m.end() - 2] + "\n  /** 🗂️ @state artifact */\n  cases: CaseRow[];\n}\n\n" + ifaces.rstrip("\n") + "\n" + s[m.end():]
    m = re.search(r"^export const parse%s: .*?\}\);$" % struct, s, re.M)
    assert m, "parser"
    line = m.group(0)
    new = line[:-3] + ", cases: normWireRequired(normWireArray(normWireRef(() => parseCaseRow))) });"
    s = s.replace(line, new + "\n" + "\n".join(parsers), 1)
    s = W.ensure_imports(s, ["normWireArray", "normWireRef", "normWireRequired", "normWireString", "normWireNumber", "normWireObject"])
    wr(path, s)


def json_typed(art, path, defs_key, formats=None, state=True, items_ref=None):
    t = rd(path)
    prop = {"type": "array", "items": {"$ref": items_ref or "#/%s/CaseRow" % defs_key}}
    if state:
        prop["x-semio-state"] = "artifact"
    t = J.add_member(t, ["properties"], "cases", prop)
    t = J.add_item(t, ["required"], "cases")
    defs = W.tmpl_defs(Art(art.short))
    for n in W.NAMES:
        d = json.loads(json.dumps(defs[n]).replace("#/$defs/", "#/%s/" % defs_key))
        if formats:
            d["x-semio-formats"] = formats
        else:
            d.pop("x-semio-formats", None)
        t = J.add_member(t, [defs_key], n, d)
    json.loads(t)
    wr(path, t)


def diff_graphql_bare(path, struct):
    s = rd(path)
    sh = rd(W.SH + "/🔗️.graphql")
    block = sh[sh.index("type CaseNumberPatch"):]
    j = s.index("\n}", s.index("type %s {" % struct))
    s = s[:j] + "\n  cases: CaseRowDelta! @state(class: ARTIFACT)" + s[j:]
    wr(path, s.rstrip("\n") + "\n\n" + block)


def diff_proto_bare(path, struct):
    s = rd(path)
    a = s.index("message %s {" % struct)
    j = s.index("\n}", a)
    n = max(int(x) for x in re.findall(r"= (\d+);", s[a:j])) + 1
    sh = rd(W.SH + "/🛰️.proto")
    block = sh[sh.index("message CaseNumberPatch"):]
    s = s[:j] + "\n  CaseRowDelta cases = %d;" % n + s[j:]
    wr(path, s.rstrip("\n") + "\n\n" + block)
