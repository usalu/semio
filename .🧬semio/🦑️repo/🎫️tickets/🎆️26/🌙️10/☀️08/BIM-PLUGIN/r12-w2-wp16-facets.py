"""🧨️ WP-16: the inference facets (json schema, ts, graphql, proto) of the clash sets and the rule results.

The aggregate facets of `s.bim.model.inference` are hand maintained, so this one-shot adds exactly the new members to the current text of each file and is idempotent: a member that is already there is
left alone. Run from the repo root: `.venv/Scripts/python.exe <this file>`.
Added: `ClashKind`, `ClashPoint`, `Clash`, `ClashGroup`, `ClashSetResult`, `RuleFinding`, `RuleResult` and `ModelInference.clash_sets` / `ModelInference.rule_results`.
"""
import json
import os
import re
import sys

sys.stdout.reconfigure(encoding="utf-8")
I = "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/"


def put(path, text):
    try:
        with open(path, "w", encoding="utf8", newline="") as handle:
            handle.write(text)
    except OSError:
        tmp = path + ".r12tmp"
        with open(tmp, "w", encoding="utf8", newline="") as handle:
            handle.write(text)
        os.replace(tmp, path)


def edit(name, change):
    path = I + name
    before = open(path, encoding="utf8", newline="").read()
    after = change(before)
    if after != before:
        put(path, after)
    print(name, "updated" if after != before else "unchanged")


# region 🔖️Json
def json_facet(text):
    doc = json.loads(text)
    defs = doc["$defs"]
    if "ClashSetResult" in defs:
        return text
    number, integer, string = {"type": "number"}, {"type": "integer"}, {"type": "string"}
    ref = lambda name: {"$ref": "#/$defs/" + name}
    obj = lambda required, properties: {"type": "object", "additionalProperties": False, "required": required, "properties": properties}
    defs["ClashKind"] = {"enum": ["Hard", "Clearance"]}
    defs["ClashPoint"] = obj(["x", "y", "z"], {"x": number, "y": number, "z": number})
    defs["Clash"] = obj(["first", "second", "kind", "distance", "point", "min", "max", "storey"], {"first": string, "second": string, "kind": ref("ClashKind"), "distance": number, "point": ref("ClashPoint"), "min": ref("ClashPoint"), "max": ref("ClashPoint"), "storey": string})
    defs["ClashGroup"] = obj(["anchor", "members"], {"anchor": string, "members": {"type": "array", "items": integer}})
    defs["ClashSetResult"] = obj(["elements_a", "elements_b", "tested", "clashes", "groups"], {"elements_a": integer, "elements_b": integer, "tested": integer, "clashes": {"type": "array", "items": ref("Clash")}, "groups": {"type": "array", "items": ref("ClashGroup")}})
    defs["RuleFinding"] = obj(["element", "measured", "limit", "storey"], {"element": string, "measured": number, "limit": number, "storey": string})
    defs["RuleResult"] = obj(["checked", "violations"], {"checked": integer, "violations": {"type": "array", "items": ref("RuleFinding")}})
    doc["properties"]["clash_sets"] = {"type": "object", "additionalProperties": ref("ClashSetResult"), "x-semio-derived": True}
    doc["properties"]["rule_results"] = {"type": "object", "additionalProperties": ref("RuleResult"), "x-semio-derived": True}
    doc["required"] += ["clash_sets", "rule_results"]
    return json.dumps(doc, indent=2, ensure_ascii=False) + "\n"


# endregion 🔖️Json

TS_TYPES = """export type ClashKind = "Hard" | "Clearance";

export interface ClashPoint {
  x: number;
  y: number;
  z: number;
}

export interface Clash {
  first: string;
  second: string;
  kind: ClashKind;
  distance: number;
  point: ClashPoint;
  min: ClashPoint;
  max: ClashPoint;
  storey: string;
}

export interface ClashGroup {
  anchor: string;
  members: number[];
}

export interface ClashSetResult {
  elements_a: number;
  elements_b: number;
  tested: number;
  clashes: Clash[];
  groups: ClashGroup[];
}

export interface RuleFinding {
  element: string;
  measured: number;
  limit: number;
  storey: string;
}

export interface RuleResult {
  checked: number;
  violations: RuleFinding[];
}

"""


# region 🔖️TypeScript
def ts_facet(text):
    if "export interface ClashSetResult" not in text:
        text = text.replace("export interface ZoneTotals {", TS_TYPES + "export interface ZoneTotals {", 1)
    if "clash_sets" not in text:
        text = re.sub(r"(export interface ModelInference \{.*?)(\n\})", lambda m: m.group(1) + "\n  /** @derived */\n  clash_sets: Record<string, ClashSetResult>;\n  /** @derived */\n  rule_results: Record<string, RuleResult>;" + m.group(2), text, count=1, flags=re.S)
    return text


# endregion 🔖️TypeScript

GQL_TYPES = """enum ClashKind {
  HARD
  CLEARANCE
}

type ClashPoint {
  x: Float!
  y: Float!
  z: Float!
}

type Clash {
  first: String!
  second: String!
  kind: ClashKind!
  distance: Float!
  point: ClashPoint!
  min: ClashPoint!
  max: ClashPoint!
  storey: String!
}

type ClashGroup {
  anchor: String!
  members: [Int!]!
}

type ClashSetResult {
  elements_a: Int!
  elements_b: Int!
  tested: Int!
  clashes: [Clash!]!
  groups: [ClashGroup!]!
}

type ClashSetResultRow {
  id: String!
  value: ClashSetResult!
}

type RuleFinding {
  element: String!
  measured: Float!
  limit: Float!
  storey: String!
}

type RuleResult {
  checked: Int!
  violations: [RuleFinding!]!
}

type RuleResultRow {
  id: String!
  value: RuleResult!
}

"""


# region 🔖️GraphQL
def graphql_facet(text):
    if "type ClashSetResult" not in text:
        text = text.replace("type ZoneTotals {", GQL_TYPES + "type ZoneTotals {", 1)
    if "clash_sets" not in text:
        text = re.sub(r"(type ModelInference \{.*?)(\n\})", lambda m: m.group(1) + "\n  clash_sets: [ClashSetResultRow!]! @derived\n  rule_results: [RuleResultRow!]! @derived" + m.group(2), text, count=1, flags=re.S)
    return text


# endregion 🔖️GraphQL

PROTO_TYPES = """enum ClashKind {
  CLASH_KIND_UNSPECIFIED = 0;
  CLASH_KIND_HARD = 1;
  CLASH_KIND_CLEARANCE = 2;
}

message ClashPoint {
  double x = 1;
  double y = 2;
  double z = 3;
}

message Clash {
  string first = 1;
  string second = 2;
  ClashKind kind = 3;
  double distance = 4;
  ClashPoint point = 5;
  ClashPoint min = 6;
  ClashPoint max = 7;
  string storey = 8;
}

message ClashGroup {
  string anchor = 1;
  repeated uint32 members = 2;
}

message ClashSetResult {
  uint32 elements_a = 1;
  uint32 elements_b = 2;
  uint32 tested = 3;
  repeated Clash clashes = 4;
  repeated ClashGroup groups = 5;
}

message RuleFinding {
  string element = 1;
  double measured = 2;
  double limit = 3;
  string storey = 4;
}

message RuleResult {
  uint32 checked = 1;
  repeated RuleFinding violations = 2;
}

"""


# region 🔖️Proto
def proto_facet(text):
    if "message ClashSetResult" not in text:
        text = text.replace("message ZoneTotals {", PROTO_TYPES + "message ZoneTotals {", 1)
    if "clash_sets" not in text:

        def add(match):
            body = match.group(1)
            top = max(int(number) for number in re.findall(r"= (\d+);", body))
            return body + "\n  // @derived\n  map<string, ClashSetResult> clash_sets = %d;\n  // @derived\n  map<string, RuleResult> rule_results = %d;" % (top + 1, top + 2) + match.group(2)

        text = re.sub(r"(message ModelInference \{.*?)(\n\})", add, text, count=1, flags=re.S)
    return text


# endregion 🔖️Proto

edit("🔣️.json", json_facet)
edit("🟦️.ts", ts_facet)
edit("🔗️.graphql", graphql_facet)
edit("🛰️.proto", proto_facet)
