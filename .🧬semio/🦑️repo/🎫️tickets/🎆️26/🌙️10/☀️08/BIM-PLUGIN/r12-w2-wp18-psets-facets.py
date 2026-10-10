"""🏷️ WP-18: the inference facets (json schema, ts, graphql, proto) of the effective properties and the six new diagnostic codes.

The aggregate facets of `s.bim.model.inference` are hand maintained, so this one-shot adds exactly the new members to the current text of each file and is idempotent. Run from the repo root:
`python r12-w2-wp18-psets-facets.py`. Added: `PropertySource`, `PropertyIssue`, `PropertyFinding`, `EffectivePropertyValue`, `EffectiveProperties`, `ModelInference.effective_properties` and the diagnostic codes
`PropertyRequiredMissing`, `PropertyKindMismatch`, `PropertyOutOfRange`, `PropertyNotAllowed`, `ClassificationUnknownCode`, `RefClassificationSystem`.
"""
import json
import os
import re
import sys

sys.stdout.reconfigure(encoding="utf-8")
I = "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/"
CODES = ["PropertyRequiredMissing", "PropertyKindMismatch", "PropertyOutOfRange", "PropertyNotAllowed", "ClassificationUnknownCode", "RefClassificationSystem"]
SOURCES = ["Own", "Type", "Default"]
ISSUES = ["Missing", "KindMismatch", "BelowMinimum", "AboveMaximum", "NotAllowed"]
VALUE = "https://json.schemas.assets.semio-tech.com/s/bim/model/artifact.json#/$defs/PropertyValue"


def put(path, text):
    try:
        os.remove(path)
    except OSError:
        pass
    with open(path, "w", encoding="utf8", newline="") as handle:
        handle.write(text)


def edit(name, change):
    path = I + name
    before = open(path, encoding="utf8", newline="").read()
    after = change(before)
    if after != before:
        put(path, after)
    print(name, "updated" if after != before else "unchanged")


def snake(name):
    return re.sub(r"(?<!^)(?=[A-Z])", "_", name).upper()


def json_facet(text):
    doc = json.loads(text)
    defs = doc["$defs"]
    defs.setdefault("PropertySource", {"enum": SOURCES})
    defs.setdefault("PropertyIssue", {"enum": ISSUES})
    defs.setdefault("PropertyFinding", {"type": "object", "additionalProperties": False, "required": ["template", "set", "property", "issue"], "properties": {"template": {"type": "string"}, "set": {"type": "string"}, "property": {"type": "string"}, "issue": {"$ref": "#/$defs/PropertyIssue"}}})
    defs.setdefault("EffectivePropertyValue", {"type": "object", "additionalProperties": False, "required": ["value", "source"], "properties": {"value": {"$ref": VALUE}, "source": {"$ref": "#/$defs/PropertySource"}, "template": {"type": "string"}}})
    defs.setdefault("EffectiveProperties", {"type": "object", "additionalProperties": False, "required": ["values", "findings"], "properties": {"values": {"type": "object", "additionalProperties": {"type": "object", "additionalProperties": {"$ref": "#/$defs/EffectivePropertyValue"}}}, "findings": {"type": "array", "items": {"$ref": "#/$defs/PropertyFinding"}}}})
    for code in CODES:
        if code not in defs["DiagnosticCode"]["enum"]:
            defs["DiagnosticCode"]["enum"].append(code)
    if "effective_properties" not in doc["properties"]:
        doc["properties"]["effective_properties"] = {"type": "object", "additionalProperties": {"$ref": "#/$defs/EffectiveProperties"}, "x-semio-derived": True}
        doc["required"].append("effective_properties")
    return json.dumps(doc, indent=2, ensure_ascii=False) + "\n"


def ts_facet(text):
    if "export interface EffectiveProperties" not in text:
        block = (
            'export type PropertySource = "Own" | "Type" | "Default";\n\n'
            'export type PropertyIssue = "Missing" | "KindMismatch" | "BelowMinimum" | "AboveMaximum" | "NotAllowed";\n\n'
            "export interface PropertyFinding {\n  template: string;\n  set: string;\n  property: string;\n  issue: PropertyIssue;\n}\n\n"
            "export interface EffectivePropertyValue {\n  value: PropertyValue;\n  source: PropertySource;\n  template?: string;\n}\n\n"
            "export interface EffectiveProperties {\n  values: Record<string, Record<string, EffectivePropertyValue>>;\n  findings: PropertyFinding[];\n}\n\n"
        )
        text = text.replace("export interface StoreyLevel {", block + "export interface StoreyLevel {", 1)
        if 'import type { PropertyValue }' not in text:
            text = text.replace("\n\n", '\n\nimport type { PropertyValue } from "../🟦️.ts";\n\n', 1)
    missing = [code for code in CODES if f'"{code}"' not in text]
    if missing:
        text = re.sub(r'(export type DiagnosticCode = [^;]*?)(;)', lambda m: m.group(1) + "".join(f' | "{code}"' for code in missing) + m.group(2), text, count=1, flags=re.S)
    if "effective_properties" not in text:
        text = re.sub(r"(export interface ModelInference \{.*?)(\n\})", lambda m: m.group(1) + "\n  /** @derived */\n  effective_properties: Record<string, EffectiveProperties>;" + m.group(2), text, count=1, flags=re.S)
    return text


def graphql_facet(text):
    if "type EffectiveProperties" not in text:
        block = (
            "enum PropertySource {\n  Own\n  Type\n  Default\n}\n\n"
            "enum PropertyIssue {\n  Missing\n  KindMismatch\n  BelowMinimum\n  AboveMaximum\n  NotAllowed\n}\n\n"
            "type PropertyFinding {\n  template: String!\n  set: String!\n  property: String!\n  issue: PropertyIssue!\n}\n\n"
            "type EffectivePropertyValue {\n  value: PropertyValue!\n  source: PropertySource!\n  template: String\n}\n\n"
            "type EffectivePropertyRow {\n  set: String!\n  name: String!\n  value: EffectivePropertyValue!\n}\n\n"
            "type EffectiveProperties {\n  values: [EffectivePropertyRow!]!\n  findings: [PropertyFinding!]!\n}\n\n"
            "type EffectivePropertiesRow {\n  id: String!\n  value: EffectiveProperties!\n}\n\n"
        )
        text = text.replace("type ZoneTotalsRow {", block + "type ZoneTotalsRow {", 1)
    for code in CODES:
        if not re.search(rf"^  {code}$", text, flags=re.M):
            text = re.sub(r"(enum DiagnosticCode \{.*?)(\n\})", lambda m: m.group(1) + f"\n  {code}" + m.group(2), text, count=1, flags=re.S)
    if "effective_properties" not in text:
        text = re.sub(r"(type ModelInference \{.*?)(\n\})", lambda m: m.group(1) + "\n  effective_properties: [EffectivePropertiesRow!]! @derived" + m.group(2), text, count=1, flags=re.S)
    return text


def proto_facet(text):
    if "message EffectiveProperties" not in text:
        block = (
            "enum PropertySource {\n  PROPERTY_SOURCE_OWN = 0;\n  PROPERTY_SOURCE_TYPE = 1;\n  PROPERTY_SOURCE_DEFAULT = 2;\n}\n\n"
            "enum PropertyIssue {\n  PROPERTY_ISSUE_MISSING = 0;\n  PROPERTY_ISSUE_KIND_MISMATCH = 1;\n  PROPERTY_ISSUE_BELOW_MINIMUM = 2;\n  PROPERTY_ISSUE_ABOVE_MAXIMUM = 3;\n  PROPERTY_ISSUE_NOT_ALLOWED = 4;\n}\n\n"
            "message PropertyFinding {\n  string template = 1;\n  string set = 2;\n  string property = 3;\n  PropertyIssue issue = 4;\n}\n\n"
            "message EffectivePropertyValue {\n  PropertyValue value = 1;\n  PropertySource source = 2;\n  optional string template = 3;\n}\n\n"
            "message EffectivePropertyGroup {\n  map<string, EffectivePropertyValue> properties = 1;\n}\n\n"
            "message EffectiveProperties {\n  map<string, EffectivePropertyGroup> values = 1;\n  repeated PropertyFinding findings = 2;\n}\n\n"
        )
        text = text.replace("message ZoneTotals {", block + "message ZoneTotals {", 1)
    numbers = [int(n) for n in re.findall(r"DIAGNOSTIC_CODE_\w+ = (\d+);", text)]
    top = max(numbers) if numbers else -1
    for code in CODES:
        token = f"DIAGNOSTIC_CODE_{snake(code)}"
        if token not in text:
            top += 1
            text = re.sub(r"(enum DiagnosticCode \{.*?)(\n\})", lambda m: m.group(1) + f"\n  {token} = {top};" + m.group(2), text, count=1, flags=re.S)
    if "effective_properties" not in text:
        def add(match):
            body = match.group(1)
            peak = max(int(n) for n in re.findall(r"= (\d+);", body))
            return body + f"\n  // @derived\n  map<string, EffectiveProperties> effective_properties = {peak + 1};" + match.group(2)

        text = re.sub(r"(message ModelInference \{.*?)(\n\})", add, text, count=1, flags=re.S)
    return text


edit("🔣️.json", json_facet)
edit("🟦️.ts", ts_facet)
edit("🔗️.graphql", graphql_facet)
edit("🛰️.proto", proto_facet)
