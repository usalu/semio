"""Layer 2: the keyed diff lane of `factorOverrides` (rs + json, ts, graphql, proto twins), modelled on the placements lane with one line per twin type."""
from fo_lib import *

N = "En1990" + ENTITY
M = "semio_s_artifact_norm_en1990_model::" + ENTITY
PATCH = [("factor", "String", "string"), ("target", "String", "string"), ("value", "f64", "number"), ("justification", "String", "string")]


def rs():
    def go(t):
        if "FactorOverrideKey" in t:
            return t
        t = put(t, "/// 🔑️ Key extractor of the foreign `Placement` rows (`id`).\n#[derive(Clone, Copy, Debug, PartialEq)]\npub struct PlacementKey;\n", "/// 🔑️ Key extractor of the foreign `Placement` rows (`id`).\n#[derive(Clone, Copy, Debug, PartialEq)]\npub struct PlacementKey;\n\n/// 🔑️ Key extractor of the foreign `FactorOverride` rows (`id`).\n#[derive(Clone, Copy, Debug, PartialEq)]\npub struct FactorOverrideKey;\n")
        key_impl = "impl protocol::list_delta::KeyOf<%s> for FactorOverrideKey {\n    type Key = String;\n    fn key_of(row: &%s) -> String {\n        row.id.clone()\n    }\n}\n\n" % (M, M)
        anchor = "impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1990_model::Placement> for PlacementKey {\n    type Key = String;\n    fn key_of(row: &semio_s_artifact_norm_en1990_model::Placement) -> String {\n        row.id.clone()\n    }\n}\n\n"
        t = put(t, anchor, anchor + key_impl)
        sets = ", ".join("%s: %s" % (n, ty) for n, ty, _ in PATCH)
        rows = (
            "\nprotocol::row_patch! {\n    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]\n    #[cfg_attr(test, serde(rename_all = \"camelCase\"))]\n    /// 🩹 Sparse field patch of one `FactorOverride`.\n    pub %sPatch of %s { set { %s } }\n}\n\n" % (N, M, sets)
            + "protocol::list_delta! {\n    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]\n    #[cfg_attr(test, serde(rename_all = \"camelCase\"))]\n    /// 📋️ Keyed row delta of one `FactorOverride` list.\n    pub %sDelta { removal: %sRemoval, insertion: %sInsertion, relocation: %sRelocation, modification: %sModification, row: %s, patch: %sPatch, list: Vec<%s>, key: String = by FactorOverrideKey }\n}\n" % (N, N, N, N, N, M, N, M)
        )
        t = put(t, "}\n//#endregion 🔖️Rows", "}\n" + rows + "//#endregion 🔖️Rows", guard="pub %sDelta" % N)
        t = put(t, "    #[state(artifact)]\n    pub cases: semio_s_plugin_norm_compliance::design_case::CaseRowDelta,\n}", "    #[state(artifact)]\n    pub cases: semio_s_plugin_norm_compliance::design_case::CaseRowDelta,\n    #[state(artifact)]\n    pub %s: %sDelta,\n}" % (COLL, N))
        t = put(t, "        self.cases.absorb(other.cases);\n", "        self.cases.absorb(other.cases);\n        self.%s.absorb(other.%s);\n" % (COLL, COLL))
        t = put(t, '        next.cases = self.cases.commit_onto(&base.cases, capability).map_err(|error| error.under(["cases"]))?;\n', '        next.cases = self.cases.commit_onto(&base.cases, capability).map_err(|error| error.under(["cases"]))?;\n        next.%s = self.%s.commit_onto(&base.%s, capability).map_err(|error| error.under(["%s"]))?;\n' % (COLL, COLL, COLL, CAMEL))
        t = put(t, "            && self.cases.is_empty()\n", "            && self.cases.is_empty()\n            && self.%s.is_empty()\n" % COLL)
        return put(t, "            cases: self.cases.inverse(&base.cases),\n", "            cases: self.cases.inverse(&base.cases),\n            %s: self.%s.inverse(&base.%s),\n" % (COLL, COLL, COLL))
    edit(DIFF + "/🦀️.rs", go)


def json_twin():
    sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
    from schema_json import dump_schema

    def go(t):
        d = json.loads(t)
        if N + "Delta" in d["$defs"]:
            return t
        ref = lambda n: {"$ref": "#/$defs/" + N + n}
        jt = {"String": {"type": "string"}, "f64": {"type": "number"}}
        out = {}
        for k, v in d["$defs"].items():
            out[k] = v
            if k == "En1990PlacementModification":
                continue
        defs = {}
        for k, v in d["$defs"].items():
            defs[k] = v
            if k == "En1990PlacementPatch":
                defs[N + "Delta"] = {"type": "object", "additionalProperties": False, "properties": {"removed": {"type": "array", "items": ref("Removal")}, "inserted": {"type": "array", "items": ref("Insertion")}, "moved": {"type": "array", "items": ref("Relocation")}, "modified": {"type": "array", "items": ref("Modification")}}}
                defs[N + "Removal"] = {"type": "object", "additionalProperties": False, "required": ["id", "index"], "properties": {"id": {"type": "string"}, "index": {"type": "integer", "minimum": 0, "maximum": 4294967295}}}
                defs[N + "Insertion"] = {"type": "object", "additionalProperties": False, "required": ["index", "row"], "properties": {"index": {"type": "integer", "minimum": 0, "maximum": 4294967295}, "row": {"$ref": "https://json.schemas.assets.semio-tech.com/s/norm/en1990/snapshot.json#/$defs/" + ENTITY}}}
                defs[N + "Relocation"] = {"type": "object", "additionalProperties": False, "required": ["id", "from", "to"], "properties": {"id": {"type": "string"}, "from": {"type": "integer", "minimum": 0, "maximum": 4294967295}, "to": {"type": "integer", "minimum": 0, "maximum": 4294967295}}}
                defs[N + "Modification"] = {"type": "object", "additionalProperties": False, "required": ["id", "patch"], "properties": {"id": {"type": "string"}, "patch": ref("Patch")}}
                defs[N + "Patch"] = {"type": "object", "additionalProperties": False, "properties": {n: {"anyOf": [jt[ty], {"type": "null"}]} for n, ty, _ in PATCH}}
        assert N + "Patch" in defs
        d["$defs"] = defs
        props = {}
        for k, v in d["properties"].items():
            props[k] = v
            if k == "placements":
                props[CAMEL] = {"anyOf": [{"$ref": "#/$defs/" + N + "Delta"}], "x-semio-state": "artifact"}
        d["properties"] = props
        return dump_schema(d)
    edit(DIFF + "/🔣️.json", go)


def ts():
    def go(t):
        if N + "Delta" in t:
            return t
        patch = " ".join("%s: (%s) | null;" % (n, tsx) for n, _, tsx in PATCH)
        ifaces = (
            "export interface %sPatch { %s }\nexport interface %sRemoval { id: string; index: number; }\nexport interface %sInsertion { index: number; row: En1990%s; }\n"
            "export interface %sRelocation { id: string; from: number; to: number; }\nexport interface %sModification { id: string; patch: %sPatch; }\n"
            "export interface %sDelta { removed: %sRemoval[]; inserted: %sInsertion[]; moved: %sRelocation[]; modified: %sModification[]; }\n"
        ) % (N, patch, N, N, ENTITY, N, N, N, N, N, N, N, N)
        t = put(t, "export type En1990RowOp =", ifaces + "export type En1990RowOp =")
        readers = {"string": "normWireString", "number": "normWireNumber"}
        pf = ", ".join("%s: normWireDefault(normWireNullable(%s), () => null)" % (n, readers[tsx]) for n, _, tsx in PATCH)
        empty = "({ removed: [], inserted: [], moved: [], modified: [] })"
        parsers = (
            "export const parse%sPatch: NormWireReader<%sPatch> = normWireObject<%sPatch>({ %s });\n" % (N, N, N, pf)
            + "export const parse%sRemoval: NormWireReader<%sRemoval> = normWireObject<%sRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireInteger) });\n" % (N, N, N)
            + "export const parse%sInsertion: NormWireReader<%sInsertion> = normWireObject<%sInsertion>({ index: normWireRequired(normWireInteger), row: normWireRequired(parseEn1990%s) });\n" % (N, N, N, ENTITY)
            + "export const parse%sRelocation: NormWireReader<%sRelocation> = normWireObject<%sRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireInteger), to: normWireRequired(normWireInteger) });\n" % (N, N, N)
            + "export const parse%sModification: NormWireReader<%sModification> = normWireObject<%sModification>({ id: normWireRequired(normWireString), patch: normWireRequired(parse%sPatch) });\n" % (N, N, N, N)
            + "export const parse%sDelta: NormWireReader<%sDelta> = normWireObject<%sDelta>({ removed: normWireDefault(normWireArray(parse%sRemoval), () => []), inserted: normWireDefault(normWireArray(parse%sInsertion), () => []), moved: normWireDefault(normWireArray(parse%sRelocation), () => []), modified: normWireDefault(normWireArray(parse%sModification), () => []) });\n" % (N, N, N, N, N, N, N)
        )
        t = put(t, "export const parseEn1990RowOp:", parsers + "export const parseEn1990RowOp:")
        t = put(t, "type En1990Placement, ", "type En1990Placement, type En1990%s, " % ENTITY)
        t = put(t, "parseEn1990Placement, ", "parseEn1990Placement, parseEn1990%s, " % ENTITY)
        t = put(t, "  /** 🗂️ @state artifact */\n  cases?: CaseRowDelta;\n}", "  /** 🗂️ @state artifact */\n  cases?: CaseRowDelta;\n  /** 🎚️ @state artifact */\n  %s: %sDelta;\n}" % (CAMEL, N))
        tail = ", cases: normWireOptional(normWireRef(() => parseCaseRowDelta)) });"
        return put(t, tail, ", cases: normWireOptional(normWireRef(() => parseCaseRowDelta)), %s: normWireDefault(parse%sDelta, () => %s) });" % (CAMEL, N, empty))
    edit(DIFF + "/🟦️.ts", go)


def graphql():
    def go(t):
        if "type %sDelta" % N in t:
            return t
        block = (
            "type %sDelta { removed: [%sRemoval!]! inserted: [%sInsertion!]! moved: [%sRelocation!]! modified: [%sModification!]! }\n"
            "type %sRemoval { id: String! index: Int! }\ntype %sInsertion { index: Int! row: %s! }\ntype %sRelocation { id: String! from: Int! to: Int! }\n"
            "type %sModification { id: String! patch: %sPatch! }\ntype %sPatch { factor: String target: String value: Float justification: String }\n\n"
        ) % (N, N, N, N, N, N, N, "FactorOverride", N, N, N, N)
        t = put(t, "enum En1990RowOp {", block + "enum En1990RowOp {")
        return put(t, "  cases: CaseRowDelta! @state(class: ARTIFACT)\n}", "  cases: CaseRowDelta! @state(class: ARTIFACT)\n  %s: %sDelta! @state(class: ARTIFACT)\n}" % (CAMEL, N))
    edit(DIFF + "/🔗️.graphql", go)


def proto():
    def go(t):
        if "message %sDelta" % N in t:
            return t
        block = (
            "message %sRemoval { string id = 1; uint32 index = 2; }\nmessage %sInsertion { uint32 index = 1; bytes row = 2; }\nmessage %sRelocation { string id = 1; uint32 from = 2; uint32 to = 3; }\n"
            "message %sModification { string id = 1; %sPatch patch = 2; }\nmessage %sDelta { repeated %sRemoval removed = 1; repeated %sInsertion inserted = 2; repeated %sRelocation moved = 3; repeated %sModification modified = 4; }\n"
            "message %sPatch { optional string factor = 1; optional string target = 2; optional double value = 3; optional string justification = 4; }\n\n"
        ) % ((N,) * 11)
        t = put(t, "enum En1990RowOp {", block + "enum En1990RowOp {")
        return put(t, "  CaseRowDelta cases = 25;\n}", "  CaseRowDelta cases = 25;\n  %sDelta %s = 26;\n}" % (N, COLL))
    edit(DIFF + "/🛰️.proto", go)


def main():
    rs()
    json_twin()
    ts()
    graphql()
    proto()


if __name__ == "__main__":
    main()
