#!/usr/bin/env python3
"""🏷️ WP4b helper — annotates the law/contract exports this ticket merged into multi-format modules.

Contract §B: every export must exist in every format its scope provides, unless it declares
`"x-semio-formats"` honestly. The retained-command / scene-owner / identity laws WP4 and WP4b moved
out of fixture directories are validated by an Ajv oracle and never transported, so they exist in
JSON Schema only. The base export ids below are exactly the ones named in `📓️wp4-plugins.md` §1–§4
and `📓️wp4b-plugins.md` §2; a `$defs` key qualifies when it starts with one of them *and* appears in
none of the module's other format files (so a merely differently-spelled Rust twin is never claimed).

Usage: wp4b-law-export-formats.py plan|apply
"""
import json
import os
import sys
from collections import OrderedDict

ROOT = "✏️s"
FORMATS = ["🦀️.rs", "🟦️.ts", "🔗️.graphql", "🛰️.proto"]
JSON_ONLY = ["🔣️jsonschema"]
BASE_EXPORTS = (
    "PresentationRetainedCommandLimits", "ShootingRetainedCommandLimits", "Fem3dRetainedCommandLimits",
    "RemodelingRetainedCommand", "HomeRetainedCommandLimits", "SpaceIndexRetainedCommandLimits",
    "SpacePlayRetainedCommandLimits", "VcsRetainedCommandRoutes", "WiresRetainedCommandRoutes",
    "ProcedureRetainedCommandRoutes", "EquationRetainedCommandLaw", "EquationSceneOwnerLaw",
    "PlaybookSceneOwnerLaw", "Process3dRetainedRouteLaws", "WriterChildLocalTextLaw",
    "FlowChildAddWidget", "FlowTreeProjection", "FlowHostWire", "FlowArtifactRecipes", "FlowGrantFrontier",
    "FlowSliderLabels", "FlowArtifactCanonical", "FlowDeleteCascade", "FlowContentIdentity",
    "FlowStoreOwners", "FlowPresenceOwners", "FlowTransientOwners", "FlowAddWidgetRetained",
    "FlowViewerOwners", "FlowSceneOwnerLaw", "GisMapCreateRegionGroup", "Puzzle2dWasmSessionFactory",
    "Puzzle3dFillPreview", "HomeProjectionPersistence", "HomeDirectoryProjectionReceipt",
    "CadRetainedJobs", "CadPresenceRetirementLaws", "SequenceRetainedActions",
)


def main():
    action = sys.argv[1] if len(sys.argv) > 1 else "plan"
    modules = 0
    annotated = 0
    for directory, names, files in os.walk(ROOT):
        names[:] = [name for name in names if name not in ("target", "node_modules", ".venv", "dist")]
        if os.path.basename(directory) != "🧬️schema":
            continue
        segments = directory.split(os.sep)
        if "🧬️mutations" in segments or any(s.startswith("🧪️") or s.startswith("🧫️") for s in segments):
            continue
        root = os.path.join(directory, "🔣️.json")
        siblings = [os.path.join(directory, name) for name in FORMATS if os.path.exists(os.path.join(directory, name))]
        if not os.path.exists(root) or not siblings:
            continue
        texts = [open(path, encoding="utf-8").read() for path in siblings]
        with open(root, encoding="utf-8") as handle:
            document = json.load(handle, object_pairs_hook=OrderedDict)
        defs = document.get("$defs")
        if not isinstance(defs, dict):
            continue
        local = 0
        for key, value in defs.items():
            if not key.startswith(BASE_EXPORTS) or not isinstance(value, dict):
                continue
            if value.get("x-semio-formats") == JSON_ONLY or any(key in text for text in texts):
                continue
            local += 1
            if action == "apply":
                value["x-semio-formats"] = JSON_ONLY
            else:
                print(f"  {directory}#/$defs/{key}")
        if not local:
            continue
        modules += 1
        annotated += local
        if action == "apply":
            with open(root, "w", encoding="utf-8") as handle:
                json.dump(document, handle, ensure_ascii=False, indent=2)
                handle.write("\n")
    print(f"modules={modules} annotated={annotated} action={action}")


if __name__ == "__main__":
    main()
