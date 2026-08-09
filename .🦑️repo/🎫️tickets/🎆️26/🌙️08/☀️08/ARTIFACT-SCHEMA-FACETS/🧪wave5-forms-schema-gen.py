#!/usr/bin/env python3
"""Generate handcrafted schema mirror leaves for forms wave-5 (ticket probe)."""
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms")

STEP_DEF = """    "FormStep": {
      "type": "object",
      "additionalProperties": false,
      "required": ["id", "title", "blocks"],
      "properties": {
        "id": { "type": "string" },
        "title": { "type": "string" },
        "description": { "type": "string" },
        "blocks": {
          "type": "array",
          "items": { "$ref": "#/$defs/FormQuestion" }
        }
      }
    },
    "FormQuestion": {
      "type": "object",
      "additionalProperties": true,
      "required": ["id", "label", "kind"],
      "properties": {
        "id": { "type": "string" },
        "label": { "type": "string" },
        "kind": { "type": "string" }
      }
    }"""

STRING_LIST = """    "FormsStringList": {
      "type": "object",
      "additionalProperties": false,
      "required": ["values"],
      "properties": {
        "values": {
          "type": "array",
          "items": { "type": "string" }
        }
      }
    }"""

STEPS_DELTA = """    "FormsStepsDelta": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "added": {
          "type": "array",
          "items": { "$ref": "#/$defs/FormStep" }
        },
        "removed": {
          "type": "array",
          "items": { "type": "string" }
        },
        "patched": {
          "type": "array",
          "items": { "$ref": "#/$defs/FormsStepPatchEntry" }
        },
        "reordered": {
          "type": "array",
          "items": { "type": "string" }
        }
      }
    },
    "FormsStepPatchEntry": {
      "type": "object",
      "additionalProperties": false,
      "required": ["id", "patch"],
      "properties": {
        "id": { "type": "string" },
        "patch": { "$ref": "#/$defs/FormsStepPatch" }
      }
    },
    "FormsStepPatch": {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "title": { "type": "string" },
        "description": { "type": "string" }
      }
    }"""


def artifact_json():
    return f"""{{
  "$id": "https://semio.tech/schema/s/forms/forms/artifact.json",
  "title": "FormsArtifact",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "schema",
    "id",
    "version",
    "steps",
    "selectedIds",
    "currentStepIndex",
    "tryValuesJson",
    "locale",
    "contributionsJson"
  ],
  "properties": {{
    "schema": {{ "type": "string", "x-semio-state": "persistent" }},
    "id": {{ "type": "string", "x-semio-state": "persistent" }},
    "version": {{ "type": "string", "x-semio-state": "persistent" }},
    "title": {{
      "oneOf": [{{ "type": "null" }}, {{ "type": "string" }}],
      "x-semio-state": "persistent"
    }},
    "steps": {{
      "type": "array",
      "items": {{ "$ref": "#/$defs/FormStep" }},
      "x-semio-state": "persistent"
    }},
    "selectedIds": {{
      "type": "array",
      "items": {{ "type": "string" }},
      "x-semio-state": "shared-ui"
    }},
    "currentStepIndex": {{
      "type": "integer",
      "format": "uint32",
      "minimum": 0,
      "x-semio-state": "local-ui"
    }},
    "tryValuesJson": {{ "type": "string", "x-semio-state": "local-ui" }},
    "locale": {{ "type": "string", "x-semio-state": "local-ui" }},
    "contributionsJson": {{ "type": "string", "x-semio-state": "local-ui" }}
  }},
  "$defs": {{
{STEP_DEF},
{STRING_LIST}
  }}
}}
"""


def snapshot_json():
    return f"""{{
  "$id": "https://semio.tech/schema/s/forms/forms/snapshot.json",
  "title": "FormsSnapshot",
  "type": "object",
  "additionalProperties": false,
  "required": ["schema", "id", "version", "steps"],
  "properties": {{
    "schema": {{ "type": "string", "x-semio-state": "persistent" }},
    "id": {{ "type": "string", "x-semio-state": "persistent" }},
    "version": {{ "type": "string", "x-semio-state": "persistent" }},
    "title": {{
      "oneOf": [{{ "type": "null" }}, {{ "type": "string" }}],
      "x-semio-state": "persistent"
    }},
    "steps": {{
      "type": "array",
      "items": {{ "$ref": "#/$defs/FormStep" }},
      "x-semio-state": "persistent"
    }}
  }},
  "$defs": {{
{STEP_DEF}
  }}
}}
"""


def diff_json():
    return f"""{{
  "$id": "https://semio.tech/schema/s/forms/forms/diff.json",
  "title": "FormsDiff",
  "type": "object",
  "additionalProperties": false,
  "required": [],
  "properties": {{
    "artifact": {{
      "title": "FormsArtifact",
      "type": "object",
      "x-semio-state": "persistent"
    }},
    "schema": {{ "type": "string", "x-semio-state": "persistent" }},
    "id": {{ "type": "string", "x-semio-state": "persistent" }},
    "version": {{ "type": "string", "x-semio-state": "persistent" }},
    "title": {{
      "oneOf": [{{ "type": "null" }}, {{ "type": "string" }}],
      "x-semio-state": "persistent"
    }},
    "steps": {{
      "$ref": "#/$defs/FormsStepsDelta",
      "x-semio-state": "persistent"
    }},
    "selectedIds": {{
      "$ref": "#/$defs/FormsStringList",
      "x-semio-state": "shared-ui"
    }},
    "currentStepIndex": {{
      "type": "integer",
      "format": "uint32",
      "minimum": 0,
      "x-semio-state": "local-ui"
    }},
    "tryValuesJson": {{ "type": "string", "x-semio-state": "local-ui" }},
    "locale": {{ "type": "string", "x-semio-state": "local-ui" }},
    "contributionsJson": {{ "type": "string", "x-semio-state": "local-ui" }}
  }},
  "$defs": {{
{STEP_DEF},
{STRING_LIST},
{STEPS_DELTA}
  }}
}}
"""


def ts_artifact():
    return """/** @emoji 🧬️ Forms artifact schema — persistent, shared-ui and local-ui fields. */
export interface FormsArtifact {
  schema: string;
  id: string;
  version: string;
  title?: string | null;
  steps: FormStep[];
  selectedIds: string[];
  currentStepIndex: number;
  tryValuesJson: string;
  locale: string;
  contributionsJson: string;
}

export interface FormStep {
  id: string;
  title: string;
  description?: string;
  blocks: FormQuestion[];
}

export interface FormQuestion {
  id: string;
  label: string;
  kind: string;
  [key: string]: unknown;
}
"""


def ts_snapshot():
    return """/** @emoji 📸️ Forms snapshot — persistent fields only. */
export interface FormsSnapshot {
  schema: string;
  id: string;
  version: string;
  title?: string | null;
  steps: FormStep[];
}

export interface FormStep {
  id: string;
  title: string;
  description?: string;
  blocks: FormQuestion[];
}

export interface FormQuestion {
  id: string;
  label: string;
  kind: string;
  [key: string]: unknown;
}
"""


def ts_diff():
    return """/** @emoji 🔺️ Sparse field delta over the forms artifact. */
export interface FormsDiff {
  artifact?: FormsArtifact;
  schema?: string;
  id?: string;
  version?: string;
  title?: string | null;
  steps?: FormsStepsDelta;
  selectedIds?: FormsStringList;
  currentStepIndex?: number;
  tryValuesJson?: string;
  locale?: string;
  contributionsJson?: string;
}

export interface FormsStringList {
  values: string[];
}

export interface FormsStepsDelta {
  added?: FormStep[];
  removed?: string[];
  patched?: FormsStepPatchEntry[];
  reordered?: string[];
}

export interface FormsStepPatchEntry {
  id: string;
  patch: FormsStepPatch;
}

export interface FormsStepPatch {
  title?: string;
  description?: string;
}

export interface FormStep {
  id: string;
  title: string;
  description?: string;
  blocks: FormQuestion[];
}

export interface FormQuestion {
  id: string;
  label: string;
  kind: string;
  [key: string]: unknown;
}

export interface FormsArtifact {
  schema: string;
  id: string;
  version: string;
  title?: string | null;
  steps: FormStep[];
  selectedIds: string[];
  currentStepIndex: number;
  tryValuesJson: string;
  locale: string;
  contributionsJson: string;
}
"""


def graphql_preamble():
    return """# @emoji 🧬️ Shared @state directive is defined in the framework schema module.
"""


def gql_artifact():
    return graphql_preamble() + """
""" + """type FormsArtifact {
  schema: String! @state(class: PERSISTENT)
  id: String! @state(class: PERSISTENT)
  version: String! @state(class: PERSISTENT)
  title: String @state(class: PERSISTENT)
  steps: [FormStep!]! @state(class: PERSISTENT)
  selectedIds: [String!]! @state(class: SHARED_UI)
  currentStepIndex: Int! @state(class: LOCAL_UI)
  tryValuesJson: String! @state(class: LOCAL_UI)
  locale: String! @state(class: LOCAL_UI)
  contributionsJson: String! @state(class: LOCAL_UI)
}

type FormStep {
  id: String!
  title: String!
  description: String
  blocks: [FormQuestion!]!
}

type FormQuestion {
  id: String!
  label: String!
  kind: String!
}
"""


def gql_snapshot():
    return graphql_preamble() + """
type FormsSnapshot {
  schema: String! @state(class: PERSISTENT)
  id: String! @state(class: PERSISTENT)
  version: String! @state(class: PERSISTENT)
  title: String @state(class: PERSISTENT)
  steps: [FormStep!]! @state(class: PERSISTENT)
}

type FormStep {
  id: String!
  title: String!
  description: String
  blocks: [FormQuestion!]!
}

type FormQuestion {
  id: String!
  label: String!
  kind: String!
}
"""


def gql_diff():
    return graphql_preamble() + """
type FormsDiff {
  artifact: FormsArtifact @state(class: PERSISTENT)
  schema: String @state(class: PERSISTENT)
  id: String @state(class: PERSISTENT)
  version: String @state(class: PERSISTENT)
  title: String @state(class: PERSISTENT)
  steps: FormsStepsDelta @state(class: PERSISTENT)
  selectedIds: FormsStringList @state(class: SHARED_UI)
  currentStepIndex: Int @state(class: LOCAL_UI)
  tryValuesJson: String @state(class: LOCAL_UI)
  locale: String @state(class: LOCAL_UI)
  contributionsJson: String @state(class: LOCAL_UI)
}

type FormsStringList {
  values: [String!]!
}

type FormsStepsDelta {
  added: [FormStep!]
  removed: [String!]
  patched: [FormsStepPatchEntry!]
  reordered: [String!]
}

type FormsStepPatchEntry {
  id: String!
  patch: FormsStepPatch!
}

type FormsStepPatch {
  title: String
  description: String
}

type FormStep {
  id: String!
  title: String!
  description: String
  blocks: [FormQuestion!]!
}

type FormQuestion {
  id: String!
  label: String!
  kind: String!
}

type FormsArtifact {
  schema: String!
  id: String!
  version: String!
  title: String
  steps: [FormStep!]!
  selectedIds: [String!]!
  currentStepIndex: Int!
  tryValuesJson: String!
  locale: String!
  contributionsJson: String!
}
"""


def proto_artifact():
    return """syntax = "proto3";
package semio.s.forms.forms.artifact;

message FormsArtifact {
  string schema = 1;
  string id = 2;
  string version = 3;
  optional string title = 4;
  repeated FormStep steps = 5;
  repeated string selected_ids = 6;
  uint32 current_step_index = 7;
  string try_values_json = 8;
  string locale = 9;
  string contributions_json = 10;
}

message FormStep {
  string id = 1;
  string title = 2;
  optional string description = 3;
  repeated FormQuestion blocks = 4;
}

message FormQuestion {
  string id = 1;
  string label = 2;
  string kind = 3;
}
"""


def proto_snapshot():
    return """syntax = "proto3";
package semio.s.forms.forms.snapshot;

message FormsSnapshot {
  string schema = 1;
  string id = 2;
  string version = 3;
  optional string title = 4;
  repeated FormStep steps = 5;
}

message FormStep {
  string id = 1;
  string title = 2;
  optional string description = 3;
  repeated FormQuestion blocks = 4;
}

message FormQuestion {
  string id = 1;
  string label = 2;
  string kind = 3;
}
"""


def proto_diff():
    return """syntax = "proto3";
package semio.s.forms.forms.diff;

message FormsDiff {
  optional FormsArtifact artifact = 1;
  optional string schema = 2;
  optional string id = 3;
  optional string version = 4;
  optional string title = 5;
  optional FormsStepsDelta steps = 6;
  optional FormsStringList selected_ids = 7;
  optional uint32 current_step_index = 8;
  optional string try_values_json = 9;
  optional string locale = 10;
  optional string contributions_json = 11;
}

message FormsStringList {
  repeated string values = 1;
}

message FormsStepsDelta {
  repeated FormStep added = 1;
  repeated string removed = 2;
  repeated FormsStepPatchEntry patched = 3;
  repeated string reordered = 4;
}

message FormsStepPatchEntry {
  string id = 1;
  FormsStepPatch patch = 2;
}

message FormsStepPatch {
  optional string title = 1;
  optional string description = 2;
}

message FormStep {
  string id = 1;
  string title = 2;
  optional string description = 3;
  repeated FormQuestion blocks = 4;
}

message FormQuestion {
  string id = 1;
  string label = 2;
  string kind = 3;
}

message FormsArtifact {
  string schema = 1;
  string id = 2;
  string version = 3;
  optional string title = 4;
  repeated FormStep steps = 5;
  repeated string selected_ids = 6;
  uint32 current_step_index = 7;
  string try_values_json = 8;
  string locale = 9;
  string contributions_json = 10;
}
"""


def write(path: Path, text: str):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text.strip() + "\n", encoding="utf-8")


def main():
    write(ROOT / "🧬️schema/🔣️component.json", artifact_json())
    write(ROOT / "📸️snapshot/🧬️schema/🔣️component.json", snapshot_json())
    write(ROOT / "🔺️diff/🧬️schema/🔣️component.json", diff_json())
    write(ROOT / "🧬️schema/🟦️component.ts", ts_artifact())
    write(ROOT / "📸️snapshot/🧬️schema/🟦️component.ts", ts_snapshot())
    write(ROOT / "🔺️diff/🧬️schema/🟦️component.ts", ts_diff())
    write(ROOT / "🧬️schema/🔗️component.graphql", gql_artifact())
    write(ROOT / "📸️snapshot/🧬️schema/🔗️component.graphql", gql_snapshot())
    write(ROOT / "🔺️diff/🧬️schema/🔗️component.graphql", gql_diff())
    write(ROOT / "🧬️schema/🛰️component.proto", proto_artifact())
    write(ROOT / "📸️snapshot/🧬️schema/🛰️component.proto", proto_snapshot())
    write(ROOT / "🔺️diff/🧬️schema/🛰️component.proto", proto_diff())
    print("wrote mirror leaves (json/ts/graphql/proto)")


if __name__ == "__main__":
    main()
