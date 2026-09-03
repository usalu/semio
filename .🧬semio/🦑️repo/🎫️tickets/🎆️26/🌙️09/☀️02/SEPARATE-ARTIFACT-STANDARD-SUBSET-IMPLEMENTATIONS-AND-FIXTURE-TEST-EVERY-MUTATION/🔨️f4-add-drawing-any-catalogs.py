#!/usr/bin/env python3
"""🔨️ F4 — add 4 mutationCatalogs entries to drawing's ✳️any/🧪️oracle/🔣️.json, reusing each
subset's own already-manifested capability. Ticket 26/09/02/…MUTATION."""
import json
from pathlib import Path

PATH = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json")
d = json.loads(PATH.read_text(encoding="utf-8"))

NEW = [
    {"id": "drawing-1-any-metadata", "capability": "drawing-1-metadata-mutate", "standardDirectoryName": "🔖️1", "subsetDirectoryName": "✳️any", "kinds": ["rename-layer", "set-layer-locked", "set-layer-visible"], "vectors": []},
    {"id": "drawing-1-any-structure", "capability": "drawing-1-structure-mutate", "standardDirectoryName": "🔖️1", "subsetDirectoryName": "✳️any", "kinds": ["create-layer", "delete-layer", "duplicate-layer", "reorder-layer"], "vectors": []},
    {"id": "drawing-1-any-style", "capability": "drawing-1-style-mutate", "standardDirectoryName": "🔖️1", "subsetDirectoryName": "✳️any", "kinds": ["replace-layer-fill", "replace-layer-stroke", "set-layer-blend-mode", "set-layer-opacity"], "vectors": []},
    {"id": "drawing-1-any-transform", "capability": "drawing-1-transform-mutate", "standardDirectoryName": "🔖️1", "subsetDirectoryName": "✳️any", "kinds": ["set-layer-boolean-operation", "update-layer-trace-params", "update-layer-transform"], "vectors": []},
]

assert d["mutationCatalogs"] == [], "expected empty mutationCatalogs at ✳️any before this script runs"
d["mutationCatalogs"] = NEW
PATH.write_text(json.dumps(d, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print("wrote", PATH)
