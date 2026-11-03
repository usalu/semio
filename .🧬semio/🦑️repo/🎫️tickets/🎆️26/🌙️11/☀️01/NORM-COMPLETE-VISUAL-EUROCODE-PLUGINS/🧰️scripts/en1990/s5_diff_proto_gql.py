from spec import *
D = SCH + "/🔺️diff"
p = D + "/🛰️.proto"; t = open(p, encoding="utf8").read()
if "En1990FatigueSpectrumDelta" not in t:
    t = t.replace("message En1990VariablePatch {\n  optional string category = 1;\n  optional double qk = 2;\n}", "message En1990VariablePatch {\n  optional string category = 1;\n  optional double qk = 2;\n  optional string group = 3;\n}")
    out = ""
    for e in ENTITIES.values():
        n = "En1990" + e["name"]
        out += f"message {n}Removal {{\n  string id = 1;\n  uint32 index = 2;\n}}\n\nmessage {n}Insertion {{\n  uint32 index = 1;\n  bytes row = 2;\n}}\n\nmessage {n}Relocation {{\n  string id = 1;\n  uint32 from = 2;\n  uint32 to = 3;\n}}\n\nmessage {n}Modification {{\n  string id = 1;\n  {n}Patch patch = 2;\n}}\n\nmessage {n}Delta {{\n  repeated {n}Removal removed = 1;\n  repeated {n}Insertion inserted = 2;\n  repeated {n}Relocation moved = 3;\n  repeated {n}Modification modified = 4;\n}}\n\nmessage {n}Patch {{\n"
        out += "".join(f"  optional {proto_type(f)} {f['snake']} = {i+1};\n" for i, f in enumerate([f for f in e["fields"] if f["snake"] != "id"])) + "}\n\n"
    t = t.replace("enum En1990RowOp {", out + "enum En1990RowOp {", 1)
    t = t.replace("  repeated En1990EffectEdit effects = 20;\n}", "  repeated En1990EffectEdit effects = 20;\n" + "".join(f"  // @state artifact\n  {n_} = {21+i};\n".replace("{n_}", "") for i, _ in enumerate([])) + "".join(f"  // @state artifact\n  En1990{e['name']}Delta {e['coll']} = {21+i};\n" for i, e in enumerate(ENTITIES.values())) + "}")
    open(p, "w", encoding="utf8").write(t)
p = D + "/🔗️.graphql"; t = open(p, encoding="utf8").read()
if "En1990FatigueSpectrumDelta" not in t:
    t = t.replace("type En1990VariablePatch {\n  category: String\n  qk: Float\n}", "type En1990VariablePatch {\n  category: String\n  qk: Float\n  group: String\n}")
    out = ""
    for e in ENTITIES.values():
        n = "En1990" + e["name"]
        out += f"type {n}Delta {{\n  removed: [{n}Removal!]!\n  inserted: [{n}Insertion!]!\n  moved: [{n}Relocation!]!\n  modified: [{n}Modification!]!\n}}\n\ntype {n}Removal {{\n  id: String!\n  index: Int!\n}}\n\ntype {n}Insertion {{\n  index: Int!\n  row: {n}!\n}}\n\ntype {n}Relocation {{\n  id: String!\n  from: Int!\n  to: Int!\n}}\n\ntype {n}Modification {{\n  id: String!\n  patch: {n}Patch!\n}}\n\ntype {n}Patch {{\n"
        out += "".join(f"  {camel(f['snake'])}: {gql_type(f)[:-1]}\n" for f in e["fields"] if f["snake"] != "id") + "}\n\n"
    t = t.replace("enum En1990RowOp {", out + "enum En1990RowOp {", 1)
    t = t.replace("  effects: [En1990EffectEdit!]! @state(class: ARTIFACT)\n}", "  effects: [En1990EffectEdit!]! @state(class: ARTIFACT)\n" + "".join(f"  {e['coll_camel']}: En1990{e['name']}Delta! @state(class: ARTIFACT)\n" for e in ENTITIES.values()) + "}")
    open(p, "w", encoding="utf8").write(t)
print("ok")
