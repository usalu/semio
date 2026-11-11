"""🔎 Section cached finish cells that publish no prism outline."""
import json
import importlib
from pathlib import Path

cache = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.♻️cache.🐍️")
model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
root = Path(__file__).resolve().parent / "🗑️generated" / "full-slab-render"
missing = sectioned = prism = 0
notes = []
for path in sorted(root.glob("*.json")):
    try:
        value = model._native_public_value(cache.decode(json.loads(path.read_text(encoding="utf-8"))["value"]))
    except Exception as error:
        notes.append(f"{path.name} {type(error).__name__}: {error}")
        continue
    if not isinstance(value, dict):
        continue
    for part in value.get("finish_parts", []):
        if "outline" in part and "z0" in part and "z1" in part and not any(key in part for key in ("mesh", "section", "top_outline")):
            prism += 1
            continue
        missing += 1
        if "mesh" not in part:
            notes.append(f"{path.name} {value.get('tag')} keys={sorted(part)}")
            continue
        vertices = part["mesh"]["vertices"]
        for axis, samples in ((0, [v[0] for v in vertices]), (1, [v[1] for v in vertices])):
            ring = model.part_section(part, (min(samples) + max(samples)) / 2, axis)
            if len(ring) >= 3:
                sectioned += 1
print(f"prism={prism} mesh_cells={missing} sectioned={sectioned} unreadable={len(notes)}")
print("\n".join(notes))
