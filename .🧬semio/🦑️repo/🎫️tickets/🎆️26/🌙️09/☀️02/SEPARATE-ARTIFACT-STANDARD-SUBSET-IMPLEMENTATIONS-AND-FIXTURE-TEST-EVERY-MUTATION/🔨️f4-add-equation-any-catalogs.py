#!/usr/bin/env python3
import json
from pathlib import Path

PATH = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json")
d = json.loads(PATH.read_text(encoding="utf-8"))

NEW = [
    {"id": "equation-1-any-graph", "capability": "equation-1-graph-mutate", "standardDirectoryName": "🔖️1", "subsetDirectoryName": "✳️any", "kinds": ["change-graph-directed", "update-graph-algorithm", "replace-graph", "create-node", "delete-node", "delete-nodes", "change-node-label", "move-node", "connect-nodes", "disconnect-nodes"], "vectors": []},
    {"id": "equation-1-any-geometry", "capability": "equation-1-geometry-mutate", "standardDirectoryName": "🔖️1", "subsetDirectoryName": "✳️any", "kinds": ["replace-points", "insert-point", "remove-point", "move-point"], "vectors": []},
    {"id": "equation-1-any-equation", "capability": "equation-1-equation-mutate", "standardDirectoryName": "🔖️1", "subsetDirectoryName": "✳️any", "kinds": ["change-coefficient"], "vectors": []},
]

assert d["mutationCatalogs"] == [], "expected empty mutationCatalogs at ✳️any before this script runs"
d["mutationCatalogs"] = NEW
PATH.write_text(json.dumps(d, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print("wrote", PATH)
