#!/usr/bin/env python3
import json
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")

FEM2D_KINDS = {
    "mesh": ["create-node", "delete-node", "create-element", "delete-element", "replace-element", "create-section", "delete-section", "replace-section", "create-region", "delete-region", "replace-region"],
    "material": ["create-material", "delete-material", "replace-material"],
    "boundary": ["create-support", "delete-support", "replace-support"],
    "load": ["create-load-case", "delete-load-case", "add-load", "remove-load", "change-load-case-self-weight", "create-combination", "delete-combination"],
    "analysis": ["update-analysis-settings"],
}
FEM3D_KINDS = {
    "mesh": ["create-node", "delete-node", "create-element", "delete-element", "replace-element", "create-section", "delete-section", "replace-section", "create-solid", "delete-solid", "replace-solid"],
    "material": ["create-material", "delete-material", "replace-material"],
    "boundary": ["create-support", "delete-support", "replace-support"],
    "load": ["create-load-case", "delete-load-case", "add-load", "remove-load", "change-load-case-self-weight", "create-combination", "delete-combination"],
    "analysis": ["update-analysis-settings"],
}

def write(artifact_dir_name: str, slug: str, kinds_by_subset: dict):
    path = ROOT / f"✏️s/🔌️plugins/🏗️fem/🗿️artifacts/{artifact_dir_name}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json"
    d = json.loads(path.read_text(encoding="utf-8"))
    assert d["mutationCatalogs"] == [], f"expected empty mutationCatalogs at {path}"
    d["mutationCatalogs"] = [
        {"id": f"{slug}-1-any-{subset}", "capability": f"{slug}-1-mutate", "standardDirectoryName": "🔖️1", "subsetDirectoryName": "✳️any", "kinds": kinds, "vectors": []}
        for subset, kinds in kinds_by_subset.items()
    ]
    path.write_text(json.dumps(d, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print("wrote", path)

write("◻️2d", "fem2d", FEM2D_KINDS)
write("🧊️3d", "fem3d", FEM3D_KINDS)
