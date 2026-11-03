"""Task 1 layer C: schema twins (json, ts, graphql, proto) of the reliability class enum and the dropped K_FI field."""
from lib import *
LIT = '"RC1" | "RC2" | "RC3"'
WIRE = 'normWireLiteral("RC1", "RC2", "RC3")'
ENUM_JSON = '{"type": "string", "enum": ["RC1", "RC2", "RC3"], "x-semio-state": "artifact"}'

def json_twin(path, annex_style):
    def go(t):
        d = json.loads(t)
        assert json.dumps(d, indent=2, ensure_ascii=False) + "\n" == t, path
        if "kFiDeclared" not in d["properties"]:
            return t
        d["properties"]["reliabilityClass"] = {"type": "string", "enum": ["RC1", "RC2", "RC3"], "x-semio-state": "artifact"}
        del d["properties"]["kFiDeclared"]
        d["required"].remove("kFiDeclared")
        return json.dumps(d, indent=2, ensure_ascii=False) + "\n"
    edit(path, go)
json_twin(SNAP + "/🔣️.json", True)
json_twin(SCH + "/🔣️.json", True)

def diff_json(t):
    if '"kFiDeclared"' not in t:
        return t
    t = sub(t, r'"kFiDeclared":\{"anyOf":\[\{"\$ref":"[^"]*#/properties/kFiDeclared"\},\{"type":"null"\}\],"x-semio-state":"artifact"\},', "")
    t = rep(t, '"reliabilityClass":{"anyOf":[{"$ref":"https://json.schemas.assets.semio-tech.com/s/norm/en1990/snapshot.json#/properties/reliabilityClass"}', '"reliabilityClass":{"anyOf":[{"$ref":"https://json.schemas.assets.semio-tech.com/s/norm/en1990/snapshot.json#/properties/reliabilityClass"}')
    return t
edit(DIFF + "/🔣️.json", diff_json)

def ts_snapshot_like(wire_kind):
    def go(t):
        if "kFiDeclared" not in t:
            return t
        t = rep(t, "  reliabilityClass: number;", "  reliabilityClass: %s;" % LIT)
        t = rep(t, "  /** 📜️ @state artifact */\n  kFiDeclared: number;\n", "")
        t = rep(t, 'reliabilityClass: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))', "reliabilityClass: normWireRequired(%s)" % WIRE)
        t = rep(t, "kFiDeclared: normWireRequired(normWireNumber), ", "")
        return t
    return go
edit(SNAP + "/🟦️.ts", ts_snapshot_like(0))
edit(SCH + "/🟦️.ts", ts_snapshot_like(0))

def ts_diff(t):
    if "kFiDeclared" not in t:
        return t
    t = rep(t, "  reliabilityClass: number | null;", "  reliabilityClass: (%s) | null;" % LIT)
    t = rep(t, "  /** ✏️ @state artifact */\n  kFiDeclared: number | null;\n", "")
    t = rep(t, 'reliabilityClass: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null)', "reliabilityClass: normWireDefault(normWireNullable(%s), () => null)" % WIRE)
    t = rep(t, "kFiDeclared: normWireDefault(normWireNullable(normWireNumber), () => null), ", "")
    return t
edit(DIFF + "/🟦️.ts", ts_diff)

def bun(t):
    if "kFiDeclared" not in t:
        return t
    t = rep(t, "  reliabilityClass: 2,\n", '  reliabilityClass: "RC2",\n')
    return rep(t, "  kFiDeclared: 1,\n", "")
edit(SCH + "/🧪parse-en1990-artifact.bun.ts", bun)

def gql_snap(t):
    if "kFiDeclared" not in t:
        return t
    t = rep(t, "enum AnnexChoice { En De }\n", "enum AnnexChoice { En De }\nenum ReliabilityClass { RC1 RC2 RC3 }\n")
    t = rep(t, "  reliabilityClass: Int!\n", "  reliabilityClass: ReliabilityClass!\n")
    return rep(t, "  kFiDeclared: Float!\n", "")
edit(SNAP + "/🔗️.graphql", gql_snap)
edit(SCH + "/🔗️.graphql", gql_snap)

def gql_diff(t):
    if "kFiDeclared" not in t:
        return t
    t = rep(t, "  reliabilityClass: Int @state(class: ARTIFACT)\n", "  reliabilityClass: String @state(class: ARTIFACT)\n")
    return rep(t, "  kFiDeclared: Float @state(class: ARTIFACT)\n", "")
edit(DIFF + "/🔗️.graphql", gql_diff)

def proto_snap(t):
    if "k_fi_declared" not in t:
        return t
    t = rep(t, "enum AnnexChoice { ANNEX_CHOICE_UNSPECIFIED = 0; En = 1; De = 2; }\n", "enum AnnexChoice { ANNEX_CHOICE_UNSPECIFIED = 0; En = 1; De = 2; }\nenum ReliabilityClass { RELIABILITY_CLASS_UNSPECIFIED = 0; RC1 = 1; RC2 = 2; RC3 = 3; }\n")
    t = rep(t, "  uint32 reliability_class = 6;\n", "  ReliabilityClass reliability_class = 6;\n")
    return rep(t, "  double k_fi_declared = 12;\n", "")
edit(SNAP + "/🛰️.proto", proto_snap)

def proto_root(t):
    return rep(t, "  uint32 reliability_class = 4;\n", "  string reliability_class = 4;\n", guard="string reliability_class")
edit(SCH + "/🛰️.proto", proto_root)

def proto_diff(t):
    if "k_fi_declared" not in t:
        return t
    t = rep(t, "  optional uint32 reliability_class = 6;\n", "  optional string reliability_class = 6;\n")
    return rep(t, "  // @state artifact\n  optional double k_fi_declared = 12;\n", "")
edit(DIFF + "/🛰️.proto", proto_diff)
