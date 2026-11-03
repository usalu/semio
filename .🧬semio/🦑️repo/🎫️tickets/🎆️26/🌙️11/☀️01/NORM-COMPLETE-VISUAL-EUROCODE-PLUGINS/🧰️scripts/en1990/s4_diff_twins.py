from spec import *
import json
D = SCH + "/🔺️diff"
SNAP = "https://json.schemas.assets.semio-tech.com/s/norm/en1990/snapshot.json#/$defs/"
# ---- json
p = D + "/🔣️.json"
d = json.load(open(p, encoding="utf8"))
defs = d["$defs"]
if "En1990FatigueSpectrumDelta" not in defs:
    defs["En1990VariablePatch"]["properties"]["group"] = {"anyOf": [{"type": "string"}, {"type": "null"}]}
    out = {}
    for k, v in defs.items():
        if k == "En1990RowOp":
            for e in ENTITIES.values():
                n = "En1990" + e["name"]
                out[n + "Delta"] = {"type": "object", "additionalProperties": False, "properties": {"removed": {"type": "array", "items": {"$ref": f"#/$defs/{n}Removal"}}, "inserted": {"type": "array", "items": {"$ref": f"#/$defs/{n}Insertion"}}, "moved": {"type": "array", "items": {"$ref": f"#/$defs/{n}Relocation"}}, "modified": {"type": "array", "items": {"$ref": f"#/$defs/{n}Modification"}}}}
                out[n + "Removal"] = {"type": "object", "additionalProperties": False, "required": ["id", "index"], "properties": {"id": {"type": "string"}, "index": {"type": "integer", "minimum": 0, "maximum": 4294967295}}}
                out[n + "Insertion"] = {"type": "object", "additionalProperties": False, "required": ["index", "row"], "properties": {"index": {"type": "integer", "minimum": 0, "maximum": 4294967295}, "row": {"$ref": SNAP + e["name"]}}}
                out[n + "Relocation"] = {"type": "object", "additionalProperties": False, "required": ["id", "from", "to"], "properties": {"id": {"type": "string"}, "from": {"type": "integer", "minimum": 0, "maximum": 4294967295}, "to": {"type": "integer", "minimum": 0, "maximum": 4294967295}}}
                out[n + "Modification"] = {"type": "object", "additionalProperties": False, "required": ["id", "patch"], "properties": {"id": {"type": "string"}, "patch": {"$ref": f"#/$defs/{n}Patch"}}}
                out[n + "Patch"] = {"type": "object", "additionalProperties": False, "properties": {camel(f["snake"]): {"anyOf": [json_type(f), {"type": "null"}]} for f in e["fields"] if f["snake"] != "id"}}
        out[k] = v
    d["$defs"] = out
    for e in ENTITIES.values():
        d["properties"][e["coll_camel"]] = {"anyOf": [{"$ref": f"#/$defs/En1990{e['name']}Delta"}], "x-semio-state": "artifact"}
    open(p, "w", encoding="utf8").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")

# ---- ts
p = D + "/🟦️.ts"
t = open(p, encoding="utf8").read()
if "En1990FatigueSpectrumDelta" not in t:
    t = t.replace("type En1990BridgeSls, type En1990ImportanceClass,", "type En1990BridgeSls, type En1990FatigueSpectrum, type En1990ImportanceClass,", 1)
    t = t.replace("type En1990PermanentAction, type En1990SeismicAction, type En1990VariableAction,", "type En1990PermanentAction, type En1990ReliabilityAnalysis, type En1990SeismicAction, type En1990TestSeries, type En1990VariableAction,", 1)
    t = t.replace("parseEn1990BridgeSls, parseEn1990ImportanceClass,", "parseEn1990BridgeSls, parseEn1990FatigueSpectrum, parseEn1990ImportanceClass,", 1)
    t = t.replace("parseEn1990PermanentAction, parseEn1990SeismicAction, parseEn1990VariableAction }", "parseEn1990PermanentAction, parseEn1990ReliabilityAnalysis, parseEn1990SeismicAction, parseEn1990TestSeries, parseEn1990VariableAction }", 1)
    # variable patch group
    import re
    m = re.search(r"export interface En1990VariablePatch \{ ([^\n]*) \}", t)
    t = t.replace(m.group(0), f"export interface En1990VariablePatch {{ {m.group(1)} group: (string) | null; }}")
    t = t.replace("qk: normWireDefault(normWireNullable(normWireNumber), () => null) });\nexport const parseEn1990VariableRemoval", "qk: normWireDefault(normWireNullable(normWireNumber), () => null), group: normWireDefault(normWireNullable(normWireString), () => null) });\nexport const parseEn1990VariableRemoval", 1)
    assert "group: normWireDefault" in t
    def nullable_ts(f): return f"{f['snake']}"
    ifaces = ""; parsers = ""
    for e in ENTITIES.values():
        n = "En1990" + e["name"]
        patch = " ".join(f"{camel(f['snake'])}: ({ts_type(f)}) | null;" for f in e["fields"] if f["snake"] != "id")
        ifaces += f"export interface {n}Patch {{ {patch} }}\nexport interface {n}Removal {{ id: string; index: number; }}\nexport interface {n}Insertion {{ index: number; row: {n}; }}\nexport interface {n}Relocation {{ id: string; from: number; to: number; }}\nexport interface {n}Modification {{ id: string; patch: {n}Patch; }}\nexport interface {n}Delta {{ removed: {n}Removal[]; inserted: {n}Insertion[]; moved: {n}Relocation[]; modified: {n}Modification[]; }}\n"
        pf = ", ".join(f"{camel(f['snake'])}: normWireDefault(normWireNullable({ts_reader(f)}), () => null)" for f in e["fields"] if f["snake"] != "id")
        parsers += f"export const parse{n}Patch: NormWireReader<{n}Patch> = normWireObject<{n}Patch>({{ {pf} }});\n"
        parsers += f"export const parse{n}Removal: NormWireReader<{n}Removal> = normWireObject<{n}Removal>({{ id: normWireRequired(normWireString), index: normWireRequired(normWireInteger) }});\n"
        parsers += f"export const parse{n}Insertion: NormWireReader<{n}Insertion> = normWireObject<{n}Insertion>({{ index: normWireRequired(normWireInteger), row: normWireRequired(parse{n}) }});\n"
        parsers += f"export const parse{n}Relocation: NormWireReader<{n}Relocation> = normWireObject<{n}Relocation>({{ id: normWireRequired(normWireString), from: normWireRequired(normWireInteger), to: normWireRequired(normWireInteger) }});\n"
        parsers += f"export const parse{n}Modification: NormWireReader<{n}Modification> = normWireObject<{n}Modification>({{ id: normWireRequired(normWireString), patch: normWireRequired(parse{n}Patch) }});\n"
        parsers += f"export const parse{n}Delta: NormWireReader<{n}Delta> = normWireObject<{n}Delta>({{ removed: normWireDefault(normWireArray(parse{n}Removal), () => []), inserted: normWireDefault(normWireArray(parse{n}Insertion), () => []), moved: normWireDefault(normWireArray(parse{n}Relocation), () => []), modified: normWireDefault(normWireArray(parse{n}Modification), () => []) }});\n"
    t = t.replace("export type En1990RowOp =", ifaces + "export type En1990RowOp =", 1)
    t = t.replace("export const parseEn1990RowOp:", parsers + "export const parseEn1990RowOp:", 1)
    emo = {"fatigue_spectra": "🔩️", "reliability_analyses": "🎯️", "test_series": "🧪️"}
    t = t.replace("  effects: En1990EffectEdit[];\n}", "  effects: En1990EffectEdit[];\n" + "".join(f"  /** {emo[e['coll']]} @state artifact */\n  {e['coll_camel']}: En1990{e['name']}Delta;\n" for e in ENTITIES.values()) + "}", 1)
    k = t.index("export const parseEn1990Diff"); eol = t.index("\n", k); line = t[k:eol]
    assert line.endswith(" });")
    line = line[:-4] + "".join(f", {e['coll_camel']}: normWireDefault(parseEn1990{e['name']}Delta, () => ({{ removed: [], inserted: [], moved: [], modified: [] }}))" for e in ENTITIES.values()) + " });"
    t = t[:k] + line + t[eol:]
    open(p, "w", encoding="utf8").write(t)
print("ts ok")
