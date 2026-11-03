import sys, json
sys.path.insert(0, ".")
from spec import *
D = S + "/🧬️schema/🔺️diff"
def P(e): return "En1997" + pascal(e.coll)
SN = "https://json.schemas.assets.semio-tech.com/s/norm/en1997/snapshot.json#/$defs/"

# graphql
GQ = {"s": "String", "n": "Float", "u": "Int"}
p = D + "/🔗️.graphql"; s = open(p, encoding="utf8").read()
if "En1997AnchorsPatch" not in s:
    blk = ""
    for e in ENTITIES:
        pn = P(e)
        blk += f"type {pn}Patch {{\n" + "".join(f"  {f.camel}: {GQ[f.kind]}\n" for f in e.fields) + "}\n\n"
        blk += f"type {pn}Removed {{\n  id: String!\n  index: Int!\n}}\n\ntype {pn}Inserted {{\n  index: Int!\n  row: {e.struct}!\n}}\n\n"
        blk += f"type {pn}Moved {{\n  id: String!\n  from: Int!\n  to: Int!\n}}\n\ntype {pn}Modified {{\n  id: String!\n  patch: {pn}Patch!\n}}\n\n"
        blk += f"type {pn}Rows {{\n  removed: [{pn}Removed!]!\n  inserted: [{pn}Inserted!]!\n  moved: [{pn}Moved!]!\n  modified: [{pn}Modified!]!\n}}\n\n"
    s = s.replace("type En1997Diff {", blk + "type En1997Diff {", 1)
    s = s.replace("  upliftCases: En1997UpliftCasesRows @state(class: ARTIFACT)\n}", "  upliftCases: En1997UpliftCasesRows @state(class: ARTIFACT)\n" + "".join(f"  {e.camel_coll}: {P(e)}Rows @state(class: ARTIFACT)\n" for e in ENTITIES) + "}")
    open(p, "w", encoding="utf8").write(s)

# proto
PT = {"s": "string", "n": "double", "u": "uint32"}
p = D + "/🛰️.proto"; s = open(p, encoding="utf8").read()
if "En1997AnchorsPatch" not in s:
    blk = ""
    for e in ENTITIES:
        pn = P(e)
        blk += f"message {pn}Patch {{\n" + "".join(f"  optional {PT[f.kind]} {f.name} = {i+1};\n" for i, f in enumerate(e.fields)) + "}\n\n"
        blk += f"message {pn}Removed {{\n  string id = 1;\n  uint64 index = 2;\n}}\n\nmessage {pn}Inserted {{\n  uint64 index = 1;\n}}\n\n"
        blk += f"message {pn}Moved {{\n  string id = 1;\n  uint64 from = 2;\n  uint64 to = 3;\n}}\n\nmessage {pn}Modified {{\n  string id = 1;\n  {pn}Patch patch = 2;\n}}\n\n"
        blk += f"message {pn}Rows {{\n  repeated {pn}Removed removed = 1;\n  repeated {pn}Inserted inserted = 2;\n  repeated {pn}Moved moved = 3;\n  repeated {pn}Modified modified = 4;\n}}\n\n"
    s = s.replace("message En1997Diff {", blk + "message En1997Diff {", 1)
    n = 13
    add = ""
    for e in ENTITIES:
        add += f"  optional {P(e)}Rows {e.coll} = {n};\n"; n += 1
    s = s.replace("  optional En1997UpliftCasesRows uplift_cases = 12;\n}", "  optional En1997UpliftCasesRows uplift_cases = 12;\n" + add + "}")
    open(p, "w", encoding="utf8").write(s)

# ts
TS = {"s": "string", "n": "number", "u": "number"}
RD = {"s": "normWireString", "n": "normWireNumber", "u": "normWireInteger"}
p = D + "/🟦️.ts"; s = open(p, encoding="utf8").read()
if "En1997AnchorsPatch" not in s:
    imp_old = s[s.index("import { type AnnexChoice"):s.index("\n", s.index("import { type AnnexChoice"))]
    names = [e.struct for e in ENTITIES]
    parts = [x.strip() for x in imp_old[len("import { "):imp_old.index(" }")].split(",")]
    for n in names:
        parts += [f"parse{n}", f"type {n}"]
    def key(x): return x.replace("type ", "").lower()
    imp_new = "import { " + ", ".join(sorted(set(parts), key=key)) + imp_old[imp_old.index(" }"):]
    s = s.replace(imp_old, imp_new, 1)
    s = s.replace("  upliftCases: En1997UpliftCasesRows | null;\n}", "  upliftCases: En1997UpliftCasesRows | null;\n" + "".join(f"  {e.camel_coll}: {P(e)}Rows | null;\n" for e in ENTITIES) + "}", 1)
    ifs = ""
    for e in ENTITIES:
        pn = P(e)
        ifs += f"export interface {pn}Inserted {{\n  index: number;\n  row: {e.struct};\n}}\n\nexport interface {pn}Modified {{\n  id: string;\n  patch: {pn}Patch;\n}}\n\nexport interface {pn}Moved {{\n  id: string;\n  from: number;\n  to: number;\n}}\n\n"
        ifs += f"export interface {pn}Patch {{\n" + "".join(f"  {f.camel}: {TS[f.kind]} | null;\n" for f in e.fields) + "}\n\n"
        ifs += f"export interface {pn}Removed {{\n  id: string;\n  index: number;\n}}\n\nexport interface {pn}Rows {{\n  removed: {pn}Removed[];\n  inserted: {pn}Inserted[];\n  moved: {pn}Moved[];\n  modified: {pn}Modified[];\n}}\n\n"
    s = s.replace("export const parseEn1997Diff:", ifs + "export const parseEn1997Diff:", 1)
    i = s.index("export const parseEn1997Diff:")
    j = s.index("\n", i)
    line = s[i:j]
    assert line.endswith(" });"), line[-80:]
    line = line[:-4] + ", " + ", ".join(f"{e.camel_coll}: normWireDefault(normWireNullable(normWireRef(() => parse{P(e)}Rows)), () => null)" for e in ENTITIES) + " });"
    s = s[:i] + line + s[j:]
    pr = ""
    for e in ENTITIES:
        pn = P(e)
        pr += f"export const parse{pn}Inserted: NormWireReader<{pn}Inserted> = normWireObject<{pn}Inserted>({{ index: normWireRequired(normWireRange(normWireInteger, {{\"minimum\":0}})), row: normWireRequired(parse{e.struct}) }});\n"
        pr += f"export const parse{pn}Modified: NormWireReader<{pn}Modified> = normWireObject<{pn}Modified>({{ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parse{pn}Patch)) }});\n"
        pr += f"export const parse{pn}Moved: NormWireReader<{pn}Moved> = normWireObject<{pn}Moved>({{ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {{\"minimum\":0}})), to: normWireRequired(normWireRange(normWireInteger, {{\"minimum\":0}})) }});\n"
        pr += f"export const parse{pn}Patch: NormWireReader<{pn}Patch> = normWireObject<{pn}Patch>({{ " + ", ".join(f"{f.camel}: normWireRequired(normWireNullable({RD[f.kind]}))" for f in e.fields) + " });\n"
        pr += f"export const parse{pn}Removed: NormWireReader<{pn}Removed> = normWireObject<{pn}Removed>({{ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {{\"minimum\":0}})) }});\n"
        pr += f"export const parse{pn}Rows: NormWireReader<{pn}Rows> = normWireObject<{pn}Rows>({{ removed: normWireRequired(normWireArray(normWireRef(() => parse{pn}Removed))), inserted: normWireRequired(normWireArray(normWireRef(() => parse{pn}Inserted))), moved: normWireRequired(normWireArray(normWireRef(() => parse{pn}Moved))), modified: normWireRequired(normWireArray(normWireRef(() => parse{pn}Modified))) }});\n"
    s = s.rstrip("\n") + "\n" + pr
    open(p, "w", encoding="utf8").write(s)

# json
p = D + "/🔣️.json"
d = json.load(open(p, encoding="utf8"))
if "En1997AnchorsRows" not in d["$defs"]:
    for e in ENTITIES:
        pn = P(e)
        d["properties"][e.camel_coll] = {"anyOf": [{"$ref": "#/$defs/" + pn + "Rows"}, {"type": "null"}], "x-semio-state": "artifact"}
        df = d["$defs"]
        df[pn + "Inserted"] = {"type": "object", "additionalProperties": False, "required": ["index", "row"], "properties": {"index": {"type": "integer", "minimum": 0}, "row": {"$ref": SN + e.struct}}}
        df[pn + "Modified"] = {"type": "object", "additionalProperties": False, "required": ["id", "patch"], "properties": {"id": {"type": "string"}, "patch": {"$ref": "#/$defs/" + pn + "Patch"}}}
        df[pn + "Moved"] = {"type": "object", "additionalProperties": False, "required": ["id", "from", "to"], "properties": {"id": {"type": "string"}, "from": {"type": "integer", "minimum": 0}, "to": {"type": "integer", "minimum": 0}}}
        df[pn + "Patch"] = {"type": "object", "additionalProperties": False, "required": [f.camel for f in e.fields], "properties": {f.camel: {"anyOf": [{"$ref": SN + e.struct + "/properties/" + f.camel}, {"type": "null"}]} for f in e.fields}}
        df[pn + "Removed"] = {"type": "object", "additionalProperties": False, "required": ["id", "index"], "properties": {"id": {"type": "string"}, "index": {"type": "integer", "minimum": 0}}}
        df[pn + "Rows"] = {"type": "object", "additionalProperties": False, "required": ["removed", "inserted", "moved", "modified"], "properties": {k: {"type": "array", "items": {"$ref": "#/$defs/" + pn + k.capitalize() if False else "#/$defs/" + pn + {"removed": "Removed", "inserted": "Inserted", "moved": "Moved", "modified": "Modified"}[k]}} for k in ["removed", "inserted", "moved", "modified"]}}
    d["$defs"] = dict(sorted(d["$defs"].items()))
    open(p, "w", encoding="utf8").write(json.dumps(d, ensure_ascii=False, indent=2) + "\n")
print("ok")
