"""🚨️ R11 w02: the inference facets (json schema, ts, graphql, proto) of the diagnostic index.

The aggregate facets of `s.bim.model.inference` are hand maintained, so this one-shot adds exactly the new members to the current text of each file and is idempotent: a member that is
already there is left alone. Run from the repo root: `python r11-w02-diagnostics-facets.py`.
Added: `SeverityCounts`, `ElementFindings`, `DiagnosticIndex` and `ModelInference.diagnostic_index`, and the same three types in the slug facet `⚠️diagnostics/🟦️.ts`.
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
        tmp = path + ".r11tmp"
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
    integer = {"type": "integer"}
    ref = lambda name: {"$ref": "#/$defs/" + name}
    by_id = lambda name: {"type": "object", "additionalProperties": ref(name)}
    if "SeverityCounts" in defs:
        return text
    defs["SeverityCounts"] = {"type": "object", "additionalProperties": False, "required": ["error", "warning", "info"], "properties": {"error": integer, "warning": integer, "info": integer}}
    defs["ElementFindings"] = {"type": "object", "additionalProperties": False, "required": ["severity", "count", "codes"], "properties": {"severity": ref("Severity"), "count": integer, "codes": {"type": "array", "items": ref("DiagnosticCode")}}}
    defs["DiagnosticIndex"] = {
        "type": "object",
        "additionalProperties": False,
        "required": ["total", "elements", "categories", "codes", "storeys"],
        "properties": {"total": ref("SeverityCounts"), "elements": by_id("ElementFindings"), "categories": by_id("SeverityCounts"), "codes": {"type": "object", "additionalProperties": integer}, "storeys": by_id("SeverityCounts")},
    }
    doc["properties"]["diagnostic_index"] = {"$ref": "#/$defs/DiagnosticIndex", "x-semio-derived": True}
    doc["required"].append("diagnostic_index")
    return json.dumps(doc, indent=2, ensure_ascii=False) + "\n"


# endregion 🔖️Json

TS_TYPES = """export interface SeverityCounts {
  error: number;
  warning: number;
  info: number;
}

export interface ElementFindings {
  severity: Severity;
  count: number;
  codes: DiagnosticCode[];
}

export interface DiagnosticIndex {
  total: SeverityCounts;
  elements: Record<string, ElementFindings>;
  categories: Record<string, SeverityCounts>;
  codes: Record<string, number>;
  storeys: Record<string, SeverityCounts>;
}

"""


# region 🔖️TypeScript
def ts_facet(text):
    if "export interface DiagnosticIndex" not in text:
        text = text.replace("export interface Diagnostic {", TS_TYPES + "export interface Diagnostic {", 1)
    if "diagnostic_index" not in text:
        text = re.sub(r"(export interface ModelInference \{.*?)(\n\})", lambda m: m.group(1) + "\n  /** @derived */\n  diagnostic_index: DiagnosticIndex;" + m.group(2), text, count=1, flags=re.S)
    return text


def ts_slug_facet(text):
    if "export interface DiagnosticIndex" not in text:
        text = text.replace("export interface Diagnostic {", TS_TYPES + "export interface Diagnostic {", 1)
    return text


# endregion 🔖️TypeScript

GQL_TYPES = """type SeverityCounts {
  error: Int!
  warning: Int!
  info: Int!
}

type SeverityCountsRow {
  id: String!
  value: SeverityCounts!
}

type ElementFindings {
  severity: Severity!
  count: Int!
  codes: [DiagnosticCode!]!
}

type ElementFindingsRow {
  id: String!
  value: ElementFindings!
}

type CodeCountRow {
  id: String!
  value: Int!
}

type DiagnosticIndex {
  total: SeverityCounts!
  elements: [ElementFindingsRow!]!
  categories: [SeverityCountsRow!]!
  codes: [CodeCountRow!]!
  storeys: [SeverityCountsRow!]!
}

"""


# region 🔖️GraphQL
def graphql_facet(text):
    if "type DiagnosticIndex" not in text:
        text = text.replace("type Diagnostic {", GQL_TYPES + "type Diagnostic {", 1)
    if "diagnostic_index" not in text:
        text = re.sub(r"(type ModelInference \{.*?)(\n\})", lambda m: m.group(1) + "\n  diagnostic_index: DiagnosticIndex! @derived" + m.group(2), text, count=1, flags=re.S)
    return text


# endregion 🔖️GraphQL

PROTO_TYPES = """message SeverityCounts {
  uint32 error = 1;
  uint32 warning = 2;
  uint32 info = 3;
}

message ElementFindings {
  Severity severity = 1;
  uint32 count = 2;
  repeated DiagnosticCode codes = 3;
}

message DiagnosticIndex {
  SeverityCounts total = 1;
  map<string, ElementFindings> elements = 2;
  map<string, SeverityCounts> categories = 3;
  map<string, uint32> codes = 4;
  map<string, SeverityCounts> storeys = 5;
}

"""


# region 🔖️Proto
def proto_facet(text):
    if "message DiagnosticIndex" not in text:
        text = text.replace("message Diagnostic {", PROTO_TYPES + "message Diagnostic {", 1)
    if "diagnostic_index" not in text:

        def add(match):
            body = match.group(1)
            top = max(int(number) for number in re.findall(r"= (\d+);", body))
            return body + "\n  // @derived\n  DiagnosticIndex diagnostic_index = %d;" % (top + 1) + match.group(2)

        text = re.sub(r"(message ModelInference \{.*?)(\n\})", add, text, count=1, flags=re.S)
    return text


# endregion 🔖️Proto

edit("🔣️.json", json_facet)
edit("🟦️.ts", ts_facet)
edit("🔗️.graphql", graphql_facet)
edit("🛰️.proto", proto_facet)
edit("⚠️diagnostics/🟦️.ts", ts_slug_facet)
