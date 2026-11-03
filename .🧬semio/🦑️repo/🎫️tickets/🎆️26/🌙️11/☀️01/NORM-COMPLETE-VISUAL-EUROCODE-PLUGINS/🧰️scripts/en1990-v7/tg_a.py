"""Traffic groups layer A: model, enum, root struct, snapshot/root twins (json, ts, graphql, proto), diff lane, examples, bun parse test."""
from tg_lib import *
import glob
ENUM = '''/// 🚦️ Traffic load group of EN 1991-2 Table 4.4a as used by EN 1990 Annex A2: the groups are mutually exclusive; the per-component coefficients are registry rows `ndp.en1990.a2.group.<group>.<category>`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum LoadGroup {
    #[dsl(key = "gr1a")]
    GR1A,
    #[dsl(key = "gr1b")]
    GR1B,
    #[dsl(key = "gr2")]
    GR2,
    #[dsl(key = "gr3")]
    GR3,
    #[dsl(key = "gr4")]
    GR4,
    #[dsl(key = "gr5")]
    GR5,
    #[dsl(key = "gr6")]
    GR6,
}

impl LoadGroup {
    pub const ALL: [Self; 7] = [Self::GR1A, Self::GR1B, Self::GR2, Self::GR3, Self::GR4, Self::GR5, Self::GR6];

    /// 🏷️ The lower-case group id (`gr1a`) that keys the registry rows and the generator's exclusive groups.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::GR1A => "gr1a",
            Self::GR1B => "gr1b",
            Self::GR2 => "gr2",
            Self::GR3 => "gr3",
            Self::GR4 => "gr4",
            Self::GR5 => "gr5",
            Self::GR6 => "gr6",
        }
    }

    /// 🔎️ The group of a lower-case id, `None` outside gr1a to gr6.
    pub fn from_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|key| key.as_str() == id)
    }
}

'''
def model():
    def go(t):
        if "pub struct TrafficGroup" in t:
            return t
        t = put(t, "    #[dsl(table)]\n    #[state(artifact)]\n    pub factor_overrides: Vec<FactorOverride>,\n}", "    #[dsl(table)]\n    #[state(artifact)]\n    pub factor_overrides: Vec<FactorOverride>,\n    #[dsl(table)]\n    #[state(artifact)]\n    pub %s: Vec<%s>,\n}" % (COLL, ENTITY))
        t = put(t, "            factor_overrides: vec![],\n", "            factor_overrides: vec![],\n            %s: vec![],\n" % COLL)
        m = re.search(r"((?:^#\[[^\n]*\n)+)pub struct FactorOverride \{.*?\n\}\n", t, re.M | re.S)
        attrs = m.group(1)
        struct = "/// 🚦️ One load component of a traffic load group: the variable action is a member of exactly this group, groups never act together; the coefficient of the component in the group is the registry row `ndp.en1990.a2.group.<group>.<category>`.\n" + attrs + "pub struct TrafficGroup {\n    pub id: String,\n    /// 🚦️ The load group of EN 1991-2 Table 4.4a.\n    pub group: TrafficGroupKey,\n    /// 🔗️ Id of the variable action that is the component.\n    pub variable_id: String,\n}\n\n"
        t = t[:m.end()] + "\n" + struct.rstrip("\n") + "\n" + t[m.end():]
        return rep(t, "/// 🏛 Seismic importance class", ENUM + "/// 🏛 Seismic importance class")
    edit(SNAP + "/🦀️.rs", go)
    def root(t):
        if COLL in t:
            return t
        t = put(t, "    pub factor_overrides: Vec<semio_s_artifact_norm_en1990_model::FactorOverride>,\n}", "    pub factor_overrides: Vec<semio_s_artifact_norm_en1990_model::FactorOverride>,\n    #[state(artifact)]\n    pub %s: Vec<%s>,\n}" % (COLL, M))
        t = put(t, "            factor_overrides: self.factor_overrides.clone(),\n", "            factor_overrides: self.factor_overrides.clone(),\n            %s: self.%s.clone(),\n" % (COLL, COLL))
        return put(t, "            factor_overrides: snapshot.factor_overrides,\n", "            factor_overrides: snapshot.factor_overrides,\n            %s: snapshot.%s,\n" % (COLL, COLL))
    edit(SCH + "/🦀️.rs", root)

def json_twin(path, key):
    t = rd(path); d = json.loads(t)
    if CAMEL in d["properties"]:
        return
    root = key == "definitions"
    prefix = "#/definitions/" if root else "#/$defs/"
    out = {}
    for k, v in d[key].items():
        out[k] = v
        if k == "FactorOverride":
            out[ENTITY] = definition(root)
    assert ENTITY in out
    d[key] = out
    pr = {}
    for k, v in d["properties"].items():
        pr[k] = v
        if k == "factorOverrides":
            pr[CAMEL] = {"type": "array", "items": {"$ref": prefix + ENTITY}, "x-semio-state": "artifact"}
    d["properties"] = pr
    d["required"].append(CAMEL)
    wr(path, json.dumps(d, indent=2, ensure_ascii=False) + "\n"); print("  edited", path.replace(SUB, ""))

def ts_twin(path, prefix, root):
    def go(t):
        if "TrafficGroup" in t:
            return t
        name = prefix + ENTITY
        t = put(t, "  factorOverrides: %sFactorOverride[];\n}" % prefix, "  factorOverrides: %sFactorOverride[];\n  /** 🚦️ @state artifact */\n  %s: %s[];\n}" % (prefix, CAMEL, name))
        iface = "export interface %s {\n  id: string;\n  group: %s;\n  variableId: string;\n}\n\n" % (name, LIT)
        m = re.search(r"export interface %sFactorOverride \{.*?\n\}\n\n" % prefix, t, re.S)
        t = t[:m.end()] + iface + t[m.end():]
        parser = "export const parse%s: NormWireReader<%s> = normWireObject<%s>({ id: normWireRequired(normWireString), group: normWireRequired(%s), variableId: normWireRequired(normWireString) });\n" % (name, name, name, WIRE)
        m = re.search(r"^export const parse%sFactorOverride: .*\n" % prefix, t, re.M)
        t = t[:m.end()] + parser + t[m.end():]
        rp = "parseEn1990Artifact" if root else "parseEn1990Snapshot"
        m = re.search(r"^export const %s: .*$" % rp, t, re.M)
        line = m.group(0)
        tail = "factorOverrides: normWireRequired(normWireArray(normWireRef(() => parse%sFactorOverride))) });" % prefix
        assert line.endswith(tail), line[-160:]
        line = line[:-4] + ", %s: normWireRequired(normWireArray(normWireRef(() => parse%s))) });" % (CAMEL, name)
        return t[:m.start()] + line + t[m.end():]
    edit(path, go)

def graphql_twin(path):
    def go(t):
        if "type TrafficGroup" in t:
            return t
        block = "enum %s { %s }\ntype TrafficGroup {\n  id: String!\n  group: %s!\n  variableId: String!\n}\n\n" % (KEY, " ".join(KEYS), KEY)
        m = re.search(r"type FactorOverride \{.*?\n\}\n\n", t, re.S)
        t = t[:m.end()] + block + t[m.end():]
        return put(t, "  factorOverrides: [FactorOverride!]!\n}", "  factorOverrides: [FactorOverride!]!\n  %s: [TrafficGroup!]!\n}" % CAMEL)
    edit(path, go)

def proto_twin(path):
    def go(t):
        if "message TrafficGroup" in t:
            return t
        enum = "enum %s { TRAFFIC_GROUP_KEY_UNSPECIFIED = 0; %s }\n" % (KEY, " ".join("%s = %d;" % (k, i + 1) for i, k in enumerate(KEYS)))
        m = re.search(r"^message FactorOverride \{[^\n]*\}\n", t, re.M)
        t = t[:m.end()] + enum + "message TrafficGroup { string id = 1; %s group = 2; string variable_id = 3; }\n" % KEY + t[m.end():]
        m = re.search(r"message En1990(?:Snapshot|Artifact) \{.*?\n\}", t, re.S)
        body = m.group(0)
        number = max(int(n) for n in re.findall(r"= (\d+);", body)) + 1
        return t.replace(body, body[:-1] + "  repeated TrafficGroup %s = %d;\n}" % (COLL, number))
    edit(path, go)

def diff_rs():
    def go(t):
        if "TrafficGroupKey" in t:
            return t
        t = put(t, "/// 🔑️ Key extractor of the foreign `FactorOverride` rows (`id`).\n#[derive(Clone, Copy, Debug, PartialEq)]\npub struct FactorOverrideKey;\n", "/// 🔑️ Key extractor of the foreign `FactorOverride` rows (`id`).\n#[derive(Clone, Copy, Debug, PartialEq)]\npub struct FactorOverrideKey;\n\n/// 🔑️ Key extractor of the foreign `TrafficGroup` rows (`id`).\n#[derive(Clone, Copy, Debug, PartialEq)]\npub struct TrafficGroupKey;\n")
        anchor = "impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1990_model::FactorOverride> for FactorOverrideKey {\n    type Key = String;\n    fn key_of(row: &semio_s_artifact_norm_en1990_model::FactorOverride) -> String {\n        row.id.clone()\n    }\n}\n\n"
        t = put(t, anchor, anchor + "impl protocol::list_delta::KeyOf<%s> for TrafficGroupKey {\n    type Key = String;\n    fn key_of(row: &%s) -> String {\n        row.id.clone()\n    }\n}\n\n" % (M, M))
        rows = ("\nprotocol::row_patch! {\n    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]\n    #[cfg_attr(test, serde(rename_all = \"camelCase\"))]\n    /// 🩹 Sparse field patch of one `TrafficGroup`.\n    pub %sPatch of %s { set { group: %s, variable_id: String } }\n}\n\n" % (N, M, KM)
            + "protocol::list_delta! {\n    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]\n    #[cfg_attr(test, serde(rename_all = \"camelCase\"))]\n    /// 📋️ Keyed row delta of one `TrafficGroup` list.\n    pub %sDelta { removal: %sRemoval, insertion: %sInsertion, relocation: %sRelocation, modification: %sModification, row: %s, patch: %sPatch, list: Vec<%s>, key: String = by TrafficGroupKey }\n}\n" % (N, N, N, N, N, M, N, M))
        t = put(t, "}\n//#endregion 🔖️Rows", "}\n" + rows + "//#endregion 🔖️Rows", guard="pub %sDelta" % N)
        t = put(t, "    #[state(artifact)]\n    pub factor_overrides: En1990FactorOverrideDelta,\n}", "    #[state(artifact)]\n    pub factor_overrides: En1990FactorOverrideDelta,\n    #[state(artifact)]\n    pub %s: %sDelta,\n}" % (COLL, N))
        t = put(t, "        self.factor_overrides.absorb(other.factor_overrides);\n", "        self.factor_overrides.absorb(other.factor_overrides);\n        self.%s.absorb(other.%s);\n" % (COLL, COLL))
        t = put(t, '        next.factor_overrides = self.factor_overrides.commit_onto(&base.factor_overrides, capability).map_err(|error| error.under(["factorOverrides"]))?;\n', '        next.factor_overrides = self.factor_overrides.commit_onto(&base.factor_overrides, capability).map_err(|error| error.under(["factorOverrides"]))?;\n        next.%s = self.%s.commit_onto(&base.%s, capability).map_err(|error| error.under(["%s"]))?;\n' % (COLL, COLL, COLL, CAMEL))
        t = put(t, "            && self.factor_overrides.is_empty()\n", "            && self.factor_overrides.is_empty()\n            && self.%s.is_empty()\n" % COLL)
        return put(t, "            factor_overrides: self.factor_overrides.inverse(&base.factor_overrides),\n", "            factor_overrides: self.factor_overrides.inverse(&base.factor_overrides),\n            %s: self.%s.inverse(&base.%s),\n" % (COLL, COLL, COLL))
    edit(DIFF + "/🦀️.rs", go)

def diff_json():
    sys.path.insert(0, SEMIO + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts")
    from schema_json import dump_schema
    def go(t):
        d = json.loads(t)
        if N + "Delta" in d["$defs"]:
            return t
        ref = lambda n: {"$ref": "#/$defs/" + N + n}
        defs = {}
        for k, v in d["$defs"].items():
            defs[k] = v
            if k == "En1990FactorOverridePatch":
                defs[N + "Delta"] = {"type": "object", "additionalProperties": False, "properties": {"removed": {"type": "array", "items": ref("Removal")}, "inserted": {"type": "array", "items": ref("Insertion")}, "moved": {"type": "array", "items": ref("Relocation")}, "modified": {"type": "array", "items": ref("Modification")}}}
                defs[N + "Removal"] = {"type": "object", "additionalProperties": False, "required": ["id", "index"], "properties": {"id": {"type": "string"}, "index": {"type": "integer", "minimum": 0, "maximum": 4294967295}}}
                defs[N + "Insertion"] = {"type": "object", "additionalProperties": False, "required": ["index", "row"], "properties": {"index": {"type": "integer", "minimum": 0, "maximum": 4294967295}, "row": {"$ref": SNAP_URL + ENTITY}}}
                defs[N + "Relocation"] = {"type": "object", "additionalProperties": False, "required": ["id", "from", "to"], "properties": {"id": {"type": "string"}, "from": {"type": "integer", "minimum": 0, "maximum": 4294967295}, "to": {"type": "integer", "minimum": 0, "maximum": 4294967295}}}
                defs[N + "Modification"] = {"type": "object", "additionalProperties": False, "required": ["id", "patch"], "properties": {"id": {"type": "string"}, "patch": ref("Patch")}}
                defs[N + "Patch"] = {"type": "object", "additionalProperties": False, "properties": {"group": {"anyOf": [{"type": "string", "enum": KEYS}, {"type": "null"}]}, "variableId": {"anyOf": [{"type": "string"}, {"type": "null"}]}}}
        assert N + "Patch" in defs
        d["$defs"] = defs
        props = {}
        for k, v in d["properties"].items():
            props[k] = v
            if k == "factorOverrides":
                props[CAMEL] = {"anyOf": [{"$ref": "#/$defs/" + N + "Delta"}], "x-semio-state": "artifact"}
        d["properties"] = props
        return dump_schema(d)
    edit(DIFF + "/🔣️.json", go)

def diff_ts():
    def go(t):
        if N + "Delta" in t:
            return t
        ifaces = ("export interface %sPatch { group: (%s) | null; variableId: (string) | null; }\nexport interface %sRemoval { id: string; index: number; }\nexport interface %sInsertion { index: number; row: En1990%s; }\n"
                  "export interface %sRelocation { id: string; from: number; to: number; }\nexport interface %sModification { id: string; patch: %sPatch; }\n"
                  "export interface %sDelta { removed: %sRemoval[]; inserted: %sInsertion[]; moved: %sRelocation[]; modified: %sModification[]; }\n") % (N, LIT, N, N, ENTITY, N, N, N, N, N, N, N, N)
        t = put(t, "export type En1990RowOp =", ifaces + "export type En1990RowOp =")
        empty = "({ removed: [], inserted: [], moved: [], modified: [] })"
        parsers = ("export const parse%sPatch: NormWireReader<%sPatch> = normWireObject<%sPatch>({ group: normWireDefault(normWireNullable(%s), () => null), variableId: normWireDefault(normWireNullable(normWireString), () => null) });\n" % (N, N, N, WIRE)
            + "export const parse%sRemoval: NormWireReader<%sRemoval> = normWireObject<%sRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireInteger) });\n" % (N, N, N)
            + "export const parse%sInsertion: NormWireReader<%sInsertion> = normWireObject<%sInsertion>({ index: normWireRequired(normWireInteger), row: normWireRequired(parseEn1990%s) });\n" % (N, N, N, ENTITY)
            + "export const parse%sRelocation: NormWireReader<%sRelocation> = normWireObject<%sRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireInteger), to: normWireRequired(normWireInteger) });\n" % (N, N, N)
            + "export const parse%sModification: NormWireReader<%sModification> = normWireObject<%sModification>({ id: normWireRequired(normWireString), patch: normWireRequired(parse%sPatch) });\n" % (N, N, N, N)
            + "export const parse%sDelta: NormWireReader<%sDelta> = normWireObject<%sDelta>({ removed: normWireDefault(normWireArray(parse%sRemoval), () => []), inserted: normWireDefault(normWireArray(parse%sInsertion), () => []), moved: normWireDefault(normWireArray(parse%sRelocation), () => []), modified: normWireDefault(normWireArray(parse%sModification), () => []) });\n" % (N, N, N, N, N, N, N))
        t = put(t, "export const parseEn1990RowOp:", parsers + "export const parseEn1990RowOp:")
        t = put(t, "type En1990FactorOverride, ", "type En1990FactorOverride, type En1990%s, " % ENTITY)
        t = put(t, "parseEn1990FactorOverride, ", "parseEn1990FactorOverride, parseEn1990%s, " % ENTITY)
        t = put(t, "  factorOverrides: En1990FactorOverrideDelta;\n}", "  factorOverrides: En1990FactorOverrideDelta;\n  /** 🚦️ @state artifact */\n  %s: %sDelta;\n}" % (CAMEL, N))
        tail = "factorOverrides: normWireRequired(normWireJson) });"
        return put(t, tail, tail[:-4] + ", %s: normWireRequired(normWireJson) });" % CAMEL)
    edit(DIFF + "/🟦️.ts", go)

def diff_graphql():
    def go(t):
        if "type %sDelta" % N in t:
            return t
        block = ("type %sDelta { removed: [%sRemoval!]! inserted: [%sInsertion!]! moved: [%sRelocation!]! modified: [%sModification!]! }\n"
            "type %sRemoval { id: String! index: Int! }\ntype %sInsertion { index: Int! row: TrafficGroup! }\ntype %sRelocation { id: String! from: Int! to: Int! }\n"
            "type %sModification { id: String! patch: %sPatch! }\ntype %sPatch { group: String variableId: String }\n\n") % ((N,) * 11)
        t = put(t, "enum En1990RowOp {", block + "enum En1990RowOp {")
        return put(t, "  factorOverrides: En1990FactorOverrideDelta! @state(class: ARTIFACT)\n", "  factorOverrides: En1990FactorOverrideDelta! @state(class: ARTIFACT)\n  %s: %sDelta! @state(class: ARTIFACT)\n" % (CAMEL, N))
    edit(DIFF + "/🔗️.graphql", go)

def diff_proto():
    def go(t):
        if "message %sDelta" % N in t:
            return t
        block = ("message %sRemoval { string id = 1; uint32 index = 2; }\nmessage %sInsertion { uint32 index = 1; bytes row = 2; }\nmessage %sRelocation { string id = 1; uint32 from = 2; uint32 to = 3; }\n"
            "message %sModification { string id = 1; %sPatch patch = 2; }\nmessage %sDelta { repeated %sRemoval removed = 1; repeated %sInsertion inserted = 2; repeated %sRelocation moved = 3; repeated %sModification modified = 4; }\n"
            "message %sPatch { optional string group = 1; optional string variable_id = 2; }\n\n") % ((N,) * 11)
        t = put(t, "enum En1990RowOp {", block + "enum En1990RowOp {")
        return put(t, "  En1990FactorOverrideDelta factor_overrides = 26;\n}", "  En1990FactorOverrideDelta factor_overrides = 26;\n  %sDelta %s = 27;\n}" % (N, COLL))
    edit(DIFF + "/🛰️.proto", go)

def examples():
    for p in glob.glob(SUB + "/📚️examples/*/🦀️.rs"):
        edit(p, lambda t: t if COLL in t else put(t, "        factor_overrides: vec![],\n", "        factor_overrides: vec![],\n        %s: vec![],\n" % COLL))
    edit(SCH + "/🧪parse-en1990-artifact.bun.ts", lambda t: t if CAMEL in t else put(t, "  factorOverrides: [],\n", "  factorOverrides: [],\n  %s: [],\n" % CAMEL))

def put(t, old, new, guard=None):
    if (guard or new) in t:
        return t
    assert t.count(old) == 1, ("anchor", old[:100], t.count(old))
    return t.replace(old, new)

if __name__ == "__main__":
    model()
    json_twin(SNAP + "/🔣️.json", "$defs")
    json_twin(SCH + "/🔣️.json", "definitions")
    ts_twin(SNAP + "/🟦️.ts", "En1990", False)
    ts_twin(SCH + "/🟦️.ts", "", True)
    for p in (SNAP + "/🔗️.graphql", SCH + "/🔗️.graphql"):
        graphql_twin(p)
    for p in (SNAP + "/🛰️.proto", SCH + "/🛰️.proto"):
        proto_twin(p)
    diff_rs(); diff_json(); diff_ts(); diff_graphql(); diff_proto(); examples()
