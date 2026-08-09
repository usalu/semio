#!/bin/bash
set -euo pipefail
BASE="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence"

# Handcrafted normative JSON + mirrors for three facets (policy parity).

cat > "$BASE/📸️snapshot/🧬️schema/🔣️component.json" <<'JSON'
{
  "$id": "https://semio.tech/schema/s/sequence/sequence/snapshot.json",
  "title": "SequenceSnapshot",
  "type": "object",
  "additionalProperties": false,
  "required": ["schema", "steps", "edges"],
  "properties": {
    "schema": { "type": "string", "x-semio-state": "persistent" },
    "steps": { "type": "array", "items": { "$ref": "#/$defs/SequenceStep" }, "x-semio-state": "persistent" },
    "edges": { "type": "array", "items": { "$ref": "#/$defs/SequenceEdge" }, "x-semio-state": "persistent" }
  },
  "$defs": {
    "SequenceStep": {
      "title": "SequenceStep",
      "type": "object",
      "additionalProperties": false,
      "required": ["id", "kind", "params", "x", "y", "collapsed"],
      "properties": {
        "id": { "type": "string" },
        "kind": { "type": "string" },
        "params": { "type": "object" },
        "x": { "type": "number", "format": "double" },
        "y": { "type": "number", "format": "double" },
        "slot": { "$ref": "#/$defs/SlotRef" },
        "collapsed": { "type": "boolean" }
      }
    },
    "SequenceEdge": {
      "title": "SequenceEdge",
      "type": "object",
      "additionalProperties": false,
      "required": ["id", "from", "to"],
      "properties": {
        "id": { "type": "string" },
        "from": { "type": "string" },
        "to": { "type": "string" }
      }
    },
    "SlotRef": {
      "title": "SlotRef",
      "type": "object",
      "additionalProperties": false,
      "required": ["owner", "name"],
      "properties": {
        "owner": { "type": "string" },
        "name": { "type": "string" }
      }
    }
  }
}
JSON

cat > "$BASE/🧬️schema/🔣️component.json" <<'JSON'
{
  "$id": "https://semio.tech/schema/s/sequence/sequence/artifact.json",
  "title": "SequenceArtifact",
  "type": "object",
  "additionalProperties": false,
  "required": ["schema", "steps", "edges", "selectedStepIds", "lastRunJson", "orientation", "camera", "locale"],
  "properties": {
    "schema": { "type": "string", "x-semio-state": "persistent" },
    "steps": { "type": "array", "items": { "$ref": "#/$defs/SequenceStep" }, "x-semio-state": "persistent" },
    "edges": { "type": "array", "items": { "$ref": "#/$defs/SequenceEdge" }, "x-semio-state": "persistent" },
    "selectedStepIds": { "type": "array", "items": { "type": "string" }, "x-semio-state": "shared-ui" },
    "lastRunJson": { "type": "string", "x-semio-state": "local-ui" },
    "orientation": { "type": "string", "x-semio-state": "local-ui" },
    "camera": { "$ref": "#/$defs/SequenceCamera", "x-semio-state": "local-ui" },
    "locale": { "type": "string", "x-semio-state": "local-ui" }
  },
  "$defs": {
    "SequenceStep": {
      "title": "SequenceStep",
      "type": "object",
      "additionalProperties": false,
      "required": ["id", "kind", "params", "x", "y", "collapsed"],
      "properties": {
        "id": { "type": "string" },
        "kind": { "type": "string" },
        "params": { "type": "object" },
        "x": { "type": "number", "format": "double" },
        "y": { "type": "number", "format": "double" },
        "slot": { "$ref": "#/$defs/SlotRef" },
        "collapsed": { "type": "boolean" }
      }
    },
    "SequenceEdge": {
      "title": "SequenceEdge",
      "type": "object",
      "additionalProperties": false,
      "required": ["id", "from", "to"],
      "properties": {
        "id": { "type": "string" },
        "from": { "type": "string" },
        "to": { "type": "string" }
      }
    },
    "SlotRef": {
      "title": "SlotRef",
      "type": "object",
      "additionalProperties": false,
      "required": ["owner", "name"],
      "properties": {
        "owner": { "type": "string" },
        "name": { "type": "string" }
      }
    },
    "SequenceCamera": {
      "title": "SequenceCamera",
      "type": "object",
      "additionalProperties": false,
      "required": ["x", "y", "zoom"],
      "properties": {
        "x": { "type": "number", "format": "double" },
        "y": { "type": "number", "format": "double" },
        "zoom": { "type": "number", "format": "double" }
      }
    }
  }
}
JSON

cat > "$BASE/🔺️diff/🧬️schema/🔣️component.json" <<'JSON'
{
  "$id": "https://semio.tech/schema/s/sequence/sequence/diff.json",
  "title": "SequenceDiff",
  "type": "object",
  "additionalProperties": false,
  "required": [],
  "properties": {
    "artifact": { "title": "SequenceArtifact", "type": "object", "x-semio-state": "persistent" },
    "schema": { "type": "string", "x-semio-state": "persistent" },
    "steps": { "$ref": "#/$defs/SequenceStepsDelta", "x-semio-state": "persistent" },
    "edges": { "$ref": "#/$defs/SequenceEdgesDelta", "x-semio-state": "persistent" },
    "selectedStepIds": { "$ref": "#/$defs/SequenceStringList", "x-semio-state": "shared-ui" },
    "lastRunJson": { "type": "string", "x-semio-state": "local-ui" },
    "orientation": { "type": "string", "x-semio-state": "local-ui" },
    "camera": { "$ref": "#/$defs/SequenceCamera", "x-semio-state": "local-ui" },
    "locale": { "type": "string", "x-semio-state": "local-ui" }
  },
  "$defs": {
    "SequenceStringList": {
      "title": "SequenceStringList",
      "type": "object",
      "additionalProperties": false,
      "required": ["values"],
      "properties": { "values": { "type": "array", "items": { "type": "string" } } }
    },
    "SequenceStepsDelta": {
      "title": "SequenceStepsDelta",
      "type": "object",
      "additionalProperties": false,
      "required": ["added", "removed", "patched"],
      "properties": {
        "added": { "type": "array", "items": { "$ref": "#/$defs/SequenceStep" } },
        "removed": { "type": "array", "items": { "type": "string" } },
        "patched": { "type": "array", "items": { "$ref": "#/$defs/SequenceStepPatchEntry" } },
        "reordered": { "type": "array", "items": { "type": "string" } }
      }
    },
    "SequenceEdgesDelta": {
      "title": "SequenceEdgesDelta",
      "type": "object",
      "additionalProperties": false,
      "required": ["added", "removed", "patched"],
      "properties": {
        "added": { "type": "array", "items": { "$ref": "#/$defs/SequenceEdge" } },
        "removed": { "type": "array", "items": { "type": "string" } },
        "patched": { "type": "array", "items": { "$ref": "#/$defs/SequenceEdgePatchEntry" } },
        "reordered": { "type": "array", "items": { "type": "string" } }
      }
    },
    "SequenceStepPatchEntry": {
      "title": "SequenceStepPatchEntry",
      "type": "object",
      "additionalProperties": false,
      "required": ["id", "patch"],
      "properties": {
        "id": { "type": "string" },
        "patch": { "$ref": "#/$defs/SequenceStepPatch" }
      }
    },
    "SequenceEdgePatchEntry": {
      "title": "SequenceEdgePatchEntry",
      "type": "object",
      "additionalProperties": false,
      "required": ["id", "patch"],
      "properties": {
        "id": { "type": "string" },
        "patch": { "$ref": "#/$defs/SequenceEdgePatch" }
      }
    },
    "SequenceStep": {
      "title": "SequenceStep",
      "type": "object",
      "additionalProperties": false,
      "required": ["id", "kind", "params", "x", "y", "collapsed"],
      "properties": {
        "id": { "type": "string" },
        "kind": { "type": "string" },
        "params": { "type": "object" },
        "x": { "type": "number", "format": "double" },
        "y": { "type": "number", "format": "double" },
        "slot": { "$ref": "#/$defs/SlotRef" },
        "collapsed": { "type": "boolean" }
      }
    },
    "SequenceEdge": {
      "title": "SequenceEdge",
      "type": "object",
      "additionalProperties": false,
      "required": ["id", "from", "to"],
      "properties": {
        "id": { "type": "string" },
        "from": { "type": "string" },
        "to": { "type": "string" }
      }
    },
    "SlotRef": {
      "title": "SlotRef",
      "type": "object",
      "additionalProperties": false,
      "required": ["owner", "name"],
      "properties": {
        "owner": { "type": "string" },
        "name": { "type": "string" }
      }
    },
    "SequenceStepPatch": {
      "title": "SequenceStepPatch",
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "params": { "type": "object" },
        "x": { "type": "number", "format": "double" },
        "y": { "type": "number", "format": "double" },
        "collapsed": { "type": "boolean" }
      }
    },
    "SequenceEdgePatch": {
      "title": "SequenceEdgePatch",
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "from": { "type": "string" },
        "to": { "type": "string" }
      }
    },
    "SequenceCamera": {
      "title": "SequenceCamera",
      "type": "object",
      "additionalProperties": false,
      "required": ["x", "y", "zoom"],
      "properties": {
        "x": { "type": "number", "format": "double" },
        "y": { "type": "number", "format": "double" },
        "zoom": { "type": "number", "format": "double" }
      }
    }
  }
}
JSON

# TypeScript mirrors
for facet in "📸️snapshot/🧬️schema:SequenceSnapshot" "🧬️schema:SequenceArtifact" "🔺️diff/🧬️schema:SequenceDiff"; do
  dir="${facet%%:*}"
  title="${facet##*:}"
  path="$BASE/$dir/🟦️component.ts"
  if [ "$title" = "SequenceSnapshot" ]; then
    cat > "$path" <<'TS'
/** 🧬️ Sequence snapshot schema — persistent fields only. */
export interface SequenceSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ steps: SequenceStep[];
  /** @state persistent */ edges: SequenceEdge[];
}
export interface SequenceStep { id: string; kind: string; params: Record<string, unknown>; x: number; y: number; slot?: SlotRef; collapsed: boolean; }
export interface SequenceEdge { id: string; from: string; to: string; }
export interface SlotRef { owner: string; name: string; }
TS
  elif [ "$title" = "SequenceArtifact" ]; then
    cat > "$path" <<'TS'
/** 🧬️ Sequence artifact schema — every field with its state class. */
export interface SequenceArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ steps: SequenceStep[];
  /** @state persistent */ edges: SequenceEdge[];
  /** @state shared-ui */ selectedStepIds: string[];
  /** @state local-ui */ lastRunJson: string;
  /** @state local-ui */ orientation: string;
  /** @state local-ui */ camera: SequenceCamera;
  /** @state local-ui */ locale: string;
}
export interface SequenceStep { id: string; kind: string; params: Record<string, unknown>; x: number; y: number; slot?: SlotRef; collapsed: boolean; }
export interface SequenceEdge { id: string; from: string; to: string; }
export interface SlotRef { owner: string; name: string; }
export interface SequenceCamera { x: number; y: number; zoom: number; }
TS
  else
    cat > "$path" <<'TS'
/** 🧬️ Sequence diff schema — sparse field delta. */
export interface SequenceDiff {
  /** @state persistent */ artifact?: SequenceArtifact;
  /** @state persistent */ schema?: string;
  /** @state persistent */ steps?: SequenceStepsDelta;
  /** @state persistent */ edges?: SequenceEdgesDelta;
  /** @state shared-ui */ selectedStepIds?: SequenceStringList;
  /** @state local-ui */ lastRunJson?: string;
  /** @state local-ui */ orientation?: string;
  /** @state local-ui */ camera?: SequenceCamera;
  /** @state local-ui */ locale?: string;
}
export interface SequenceStringList { values: string[]; }
export interface SequenceStepsDelta { added: SequenceStep[]; removed: string[]; patched: SequenceStepPatchEntry[]; reordered?: string[]; }
export interface SequenceEdgesDelta { added: SequenceEdge[]; removed: string[]; patched: SequenceEdgePatchEntry[]; reordered?: string[]; }
export interface SequenceStepPatchEntry { id: string; patch: SequenceStepPatch; }
export interface SequenceEdgePatchEntry { id: string; patch: SequenceEdgePatch; }
export interface SequenceStep { id: string; kind: string; params: Record<string, unknown>; x: number; y: number; slot?: SlotRef; collapsed: boolean; }
export interface SequenceEdge { id: string; from: string; to: string; }
export interface SlotRef { owner: string; name: string; }
export interface SequenceStepPatch { params?: Record<string, unknown>; x?: number; y?: number; collapsed?: boolean; }
export interface SequenceEdgePatch { from?: string; to?: string; }
export interface SequenceCamera { x: number; y: number; zoom: number; }
export interface SequenceArtifact {
  schema: string; steps: SequenceStep[]; edges: SequenceEdge[];
  selectedStepIds: string[]; lastRunJson: string; orientation: string; camera: SequenceCamera; locale: string;
}
TS
  fi
done

echo "wrote json + ts leaves"
