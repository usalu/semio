from spec import *
import json
SN = SCH + "/📸️snapshot"
# ---- JSON (snapshot + root)
for path, root in ((SN + "/🔣️.json", False), (SCH + "/🔣️.json", True)):
    d = json.load(open(path, encoding="utf8"))
    defs = d["definitions" if root else "$defs"]
    if "FatigueSpectrum" in defs: continue
    pre = "#/definitions/" if root else "#/$defs/"
    var = defs["VariableAction"]
    var["required"].append("group"); var["properties"]["group"] = prop_json(GROUP_FIELD, root)
    if not root:
        var["properties"]["group"] = {k: v for k, v in var["properties"]["group"].items()}
    new_defs = {}
    for e in ENTITIES.values():
        d["required"].append(e["coll_camel"])
        d["properties"][e["coll_camel"]] = {"type": "array", "items": {"$ref": pre + e["name"]}, "x-semio-state": "artifact"}
    items = list(defs.items())
    # insert entity defs after BridgeSls to keep entities together
    out = {}
    for k, v in items:
        out[k] = v
        if k == "BridgeSls":
            for e in ENTITIES.values(): out[e["name"]] = def_json(e, pre, root=root)
    d["definitions" if root else "$defs"] = out
    open(path, "w", encoding="utf8").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")

# ---- TS
EMOJI = {"fatigue_spectra": "🔩️", "reliability_analyses": "🎯️", "test_series": "🧪️"}
def ts_iface(e, prefix):
    return f"export interface {prefix}{e['name']} {{\n" + "".join(f"  {camel(f['snake'])}: {ts_type(f)};\n" for f in e["fields"]) + "}\n\n"
def ts_parser(e, prefix, rootref=False):
    body = ", ".join(f"{camel(f['snake'])}: normWireRequired({ts_reader(f)})" for f in e["fields"])
    return f"export const parse{prefix}{e['name']}: NormWireReader<{prefix}{e['name']}> = normWireObject<{prefix}{e['name']}>({{ {body} }});\n"

for path, prefix, root in ((SN + "/🟦️.ts", "En1990", False), (SCH + "/🟦️.ts", "", True)):
    t = open(path, encoding="utf8").read()
    if "FatigueSpectrum" in t: continue
    var = prefix + "VariableAction"
    t = t.replace(f"export interface {var} {{\n  id: string;\n  category: string;\n  qk: number;\n}}", f"export interface {var} {{\n  id: string;\n  category: string;\n  qk: number;\n  group: string;\n}}")
    t = t.replace("qk: normWireRequired(normWireNumber) });", "qk: normWireRequired(normWireNumber), group: normWireRequired(normWireString) });", 1)
    assert "group: normWireRequired" in t and "group: string" in t
    # snapshot interface fields
    last_field = f"  effects: {prefix + 'MemberEffect' if not root else 'MemberEffect'}[];\n"
    assert last_field in t, path
    add = "".join(f"  /** {EMOJI[e['coll']]} @state artifact */\n  {e['coll_camel']}: {prefix}{e['name']}[];\n" for e in ENTITIES.values())
    t = t.replace(last_field, last_field + add, 1)
    # interfaces after BridgeSls interface
    marker = f"export interface {prefix}BridgeSls {{"
    i = t.index(marker); j = t.index("}\n\n", i) + 3
    t = t[:j] + "".join(ts_iface(e, prefix) for e in ENTITIES.values()) + t[j:]
    # root reader: add fields before the closing of parse...Snapshot/Artifact
    rname = "parseEn1990Snapshot" if not root else "parseEn1990Artifact"
    k = t.index(f"export const {rname}")
    eol = t.index("\n", k)
    line = t[k:eol]
    assert line.endswith(" });")
    addr = "".join(f", {e['coll_camel']}: normWireRequired(normWireArray(normWireRef(() => parse{prefix}{e['name']})))" for e in ENTITIES.values())
    line2 = line[:-4] + addr + " });"
    t = t[:k] + line2 + t[eol:]
    # parsers after BridgeSls parser
    pk = t.index(f"export const parse{prefix}BridgeSls")
    pe = t.index("\n", pk) + 1
    t = t[:pe] + "".join(ts_parser(e, prefix) for e in ENTITIES.values()) + t[pe:]
    open(path, "w", encoding="utf8").write(t)

# ---- proto
pp = SN + "/🛰️.proto"
t = open(pp, encoding="utf8").read()
if "FatigueSpectrum" not in t:
    t = t.replace("message VariableAction { string id = 1; string category = 2; double qk = 3; }", "message VariableAction { string id = 1; string category = 2; double qk = 3; string group = 4; }")
    msgs = ""
    for e in ENTITIES.values():
        msgs += f"message {e['name']} {{ " + " ".join(f"{proto_type(f)} {f['snake']} = {i+1};" for i, f in enumerate(e["fields"])) + " }\n"
    t = t.replace("message MemberEffect { string member_id = 1; string action_id = 2; double influence = 3; }\n", "message MemberEffect { string member_id = 1; string action_id = 2; double influence = 3; }\n" + msgs)
    t = t.replace("  repeated MemberEffect effects = 20;\n}", "  repeated MemberEffect effects = 20;\n" + "".join(f"  repeated {e['name']} {e['coll']} = {21+i};\n" for i, e in enumerate(ENTITIES.values())) + "}")
    open(pp, "w", encoding="utf8").write(t)
pp = SCH + "/🛰️.proto"
t = open(pp, encoding="utf8").read()
if "FatigueSpectrum" not in t:
    t = t.replace("message VariableAction { string id = 1; string category = 2; double qk = 3; }", "message VariableAction { string id = 1; string category = 2; double qk = 3; string group = 4; }")
    msgs = ""
    for e in ENTITIES.values():
        msgs += f"message {e['name']} {{ " + " ".join(f"{proto_type(f)} {f['snake']} = {i+1};" for i, f in enumerate(e["fields"])) + " }\n"
    t = t.rstrip("\n") + "\n" + msgs
    t = t.replace("  repeated MemberEffect effects = 16;\n}", "  repeated MemberEffect effects = 16;\n" + "".join(f"  repeated {e['name']} {e['coll']} = {17+i};\n" for i, e in enumerate(ENTITIES.values())) + "}")
    open(pp, "w", encoding="utf8").write(t)

# ---- graphql
for gp, tname in ((SN + "/🔗️.graphql", "En1990Snapshot"), (SCH + "/🔗️.graphql", "En1990Artifact")):
    t = open(gp, encoding="utf8").read()
    if "FatigueSpectrum" in t: continue
    t = t.replace("type VariableAction { id: String! category: String! qk: Float! }", "type VariableAction { id: String! category: String! qk: Float! group: String! }")
    types = ""
    for e in ENTITIES.values():
        types += f"type {e['name']} {{\n" + "".join(f"  {camel(f['snake'])}: {gql_type(f)}\n" for f in e["fields"]) + "}\n"
    t = t.replace("type MemberEffect { memberId: String! actionId: String! influence: Float! }\n", "type MemberEffect { memberId: String! actionId: String! influence: Float! }\n" + types)
    t = t.replace("  effects: [MemberEffect!]!\n}", "  effects: [MemberEffect!]!\n" + "".join(f"  {e['coll_camel']}: [{e['name']}!]!\n" for e in ENTITIES.values()) + "}")
    open(gp, "w", encoding="utf8").write(t)

# ---- artifact root ts + parse test
edit_once_if_absent(ROOT + "/🟦️.ts", "FatigueSpectrum", 'export type {BridgeSls} from', 'export type {FatigueSpectrum, ReliabilityAnalysis, TestSeries} from "./🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts";\nexport type {BridgeSls} from')
edit_once_if_absent(SCH + "/🧪parse-en1990-artifact.bun.ts", "fatigueSpectra", "  effects: [],\n", "  effects: [],\n  fatigueSpectra: [],\n  reliabilityAnalyses: [],\n  testSeries: [],\n")
print("twins ok")
