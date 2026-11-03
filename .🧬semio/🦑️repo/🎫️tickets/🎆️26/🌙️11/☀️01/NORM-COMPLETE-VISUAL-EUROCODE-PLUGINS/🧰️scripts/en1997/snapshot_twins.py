import json, re, sys
sys.path.insert(0, ".")
from spec import *

def structs():
    out = []
    for e in ENTITIES:
        if e.nested:
            out.append(e.nested[2])
        out.append(e)
    return out
ALL = structs()
ORDER = []
for e in ENTITIES:
    if e.nested: ORDER.append(e.nested[2])
    ORDER.append(e)

def jschema(e):
    props = {"id": {"type": "string"}}
    req = ["id"]
    for f in e.fields:
        req.append(f.camel)
        if f.kind == "s":
            props[f.camel] = {"type": "string", "x-semio-ui": f.ui()}
        elif f.kind == "u":
            props[f.camel] = {"type": "integer", "minimum": 0, "x-semio-ui": f.ui()}
        else:
            props[f.camel] = {"type": "number", "x-semio-ui": f.ui()}
    if e.nested:
        n = e.nested
        req.append(camel(n[0]))
        props[camel(n[0])] = {"type": "array", "items": {"$ref": "#/$defs/" + n[2].struct}, "x-semio-ui": {"label": {"en": n[2].en + "s", "de": n[2].de + "e"}}}
    return {"title": e.struct, "type": "object", "additionalProperties": False, "required": req, "properties": props, "x-semio-formats": ["🔣️jsonschema", "🔗️graphql", "🛰️protobuf"]}

def edit_json(path, snapshot):
    d = json.load(open(path, encoding="utf8"))
    for e in ENTITIES:
        c = e.camel_coll
        if c in d["required"]:
            continue
        d["required"].append(c)
        if snapshot:
            d["properties"][c] = {"type": "array", "items": {"$ref": "#/$defs/" + e.struct}, "x-semio-state": "artifact"}
        else:
            d["properties"][c] = {"type": "array", "items": {"type": "object"}, "x-semio-state": "artifact"}
    if snapshot:
        for e in ORDER:
            d["$defs"][e.struct] = jschema(e)
    open(path, "w", encoding="utf8").write(json.dumps(d, ensure_ascii=False, indent=2) + "\n")

edit_json(S + "/🧬️schema/📸️snapshot/🔣️.json", True)
edit_json(S + "/🧬️schema/🔣️.json", False)

GQ = {"s": "String", "n": "Float", "u": "Int"}
def gql(e):
    lines = [f"type {e.struct} {{", "  id: String!"]
    for f in e.fields:
        lines.append(f"  {f.camel}: {GQ[f.kind]}!")
    if e.nested:
        lines.append(f"  {camel(e.nested[0])}: [{e.nested[2].struct}!]!")
    lines.append("}")
    return "\n".join(lines) + "\n"
PT = {"s": "string", "n": "double", "u": "uint32"}
def proto(e):
    lines = [f"message {e.struct} {{", "  string id = 1;"]
    i = 2
    for f in e.fields:
        lines.append(f"  {PT[f.kind]} {f.name} = {i};"); i += 1
    if e.nested:
        lines.append(f"  repeated {e.nested[2].struct} {e.nested[0]} = {i};")
    lines.append("}")
    return "\n".join(lines) + "\n"

for p in (S + "/🧬️schema/🔗️.graphql", S + "/🧬️schema/📸️snapshot/🔗️.graphql"):
    s = open(p, encoding="utf8").read()
    if "type Anchor " in s: continue
    block = "".join(gql(e) for e in ORDER)
    s = s.replace("type En1997Artifact {", block + "\ntype En1997Artifact {")
    s = s.replace("  upliftCases: [UpliftCase!]!\n}", "  upliftCases: [UpliftCase!]!\n" + "".join(f"  {e.camel_coll}: [{e.struct}!]!\n" for e in ENTITIES) + "}")
    open(p, "w", encoding="utf8").write(s)
for p in (S + "/🧬️schema/🛰️.proto", S + "/🧬️schema/📸️snapshot/🛰️.proto"):
    s = open(p, encoding="utf8").read()
    if "message Anchor " in s: continue
    block = "".join(proto(e) for e in ORDER)
    s = s.replace("message En1997Artifact {", block + "message En1997Artifact {")
    n = 14
    add = ""
    for e in ENTITIES:
        add += f"  repeated {e.struct} {e.coll} = {n};\n"; n += 1
    s = s.replace("  repeated UpliftCase uplift_cases = 13;\n}", "  repeated UpliftCase uplift_cases = 13;\n" + add + "}")
    open(p, "w", encoding="utf8").write(s)

TS = {"s": "string", "n": "number", "u": "number"}
RD = {"s": "normWireString", "n": "normWireNumber", "u": "normWireInteger"}
def ts_iface(e):
    l = [f"export interface {e.struct} {{", "  id: string;"]
    for f in e.fields: l.append(f"  {f.camel}: {TS[f.kind]};")
    if e.nested: l.append(f"  {camel(e.nested[0])}: {e.nested[2].struct}[];")
    l.append("}")
    return "\n".join(l) + "\n\n"
def ts_parse(e, ref):
    items = ["id: normWireRequired(normWireString)"]
    for f in e.fields:
        r = RD[f.kind]
        if f.kind == "u": r = "normWireRange(normWireInteger, {\"minimum\":0})"
        items.append(f"{f.camel}: normWireRequired({r})")
    if e.nested:
        items.append(f"{camel(e.nested[0])}: normWireRequired(normWireArray(normWireRef(() => parse{e.nested[2].struct})))")
    return f"export const parse{e.struct}: NormWireReader<{e.struct}> = normWireObject<{e.struct}>({{ " + ", ".join(items) + " });\n"

p = S + "/🧬️schema/📸️snapshot/🟦️.ts"
s = open(p, encoding="utf8").read()
if "interface Anchor " not in s:
    s = s.replace("  upliftCases: UpliftCase[];\n}", "  upliftCases: UpliftCase[];\n" + "".join(f"  /** 🆕 @state artifact */\n  {e.camel_coll}: {e.struct}[];\n" for e in ENTITIES) + "}")
    iface = "".join(ts_iface(e) for e in ORDER)
    s = s.replace("export const parseEn1997Snapshot", iface + "export const parseEn1997Snapshot", 1)
    s = s.replace("upliftCases: normWireRequired(normWireArray(normWireRef(() => parseUpliftCase))) });", "upliftCases: normWireRequired(normWireArray(normWireRef(() => parseUpliftCase))), " + ", ".join(f"{e.camel_coll}: normWireRequired(normWireArray(normWireRef(() => parse{e.struct})))" for e in ENTITIES) + " });", 1)
    s = s.rstrip("\n") + "\n" + "".join(ts_parse(e, True) for e in ORDER)
    open(p, "w", encoding="utf8").write(s)
p = S + "/🧬️schema/🟦️.ts"
s = open(p, encoding="utf8").read()
if "anchors:" not in s:
    s = s.replace("  upliftCases: { [key: string]: NormJson }[];\n}", "  upliftCases: { [key: string]: NormJson }[];\n" + "".join(f"  /** 🆕 @state artifact */\n  {e.camel_coll}: {{ [key: string]: NormJson }}[];\n" for e in ENTITIES) + "}")
    s = s.replace("upliftCases: normWireRequired(normWireArray(normWireMap(normWireJson))) });", "upliftCases: normWireRequired(normWireArray(normWireMap(normWireJson))), " + ", ".join(f"{e.camel_coll}: normWireRequired(normWireArray(normWireMap(normWireJson)))" for e in ENTITIES) + " });")
    open(p, "w", encoding="utf8").write(s)
print("done")
