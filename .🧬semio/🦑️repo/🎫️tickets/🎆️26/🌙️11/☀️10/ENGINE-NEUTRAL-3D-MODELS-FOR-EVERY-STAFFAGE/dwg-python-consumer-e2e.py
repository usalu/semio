"""Renders one elevation and one axonometric cast through the real exporter (bun) via the Python consumer and reads them back (run from the workspace root with the course Python environment)."""
import importlib, json, sys
S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
base = {"asset": "standing-adult", "pose": "relaxed", "view": "dark-line", "theme": "dark", "scale": 50}
manifest = {"version": 1, "casts": {"e2e-elevation": {**base, "viewpoint": "elevation"}, "e2e-axonometric": {**base, "viewpoint": "axonometric"}}}
S._manifest = lambda: manifest
folders = S.ensure_casts(list(manifest["casts"]), progress=lambda line: print("[progress]", line, flush=True))
for cast, folder in folders.items():
    index = S._index(folder)
    S._provenance(cast, index)
    print(cast, index["viewpoint"], len(index["frames"]), "frames", "provenance ok")
    group = S.staffage(cast, unit=1.0)
    print("  mobject", type(group).__name__, "height", round(S.staffage_height(cast), 4))
