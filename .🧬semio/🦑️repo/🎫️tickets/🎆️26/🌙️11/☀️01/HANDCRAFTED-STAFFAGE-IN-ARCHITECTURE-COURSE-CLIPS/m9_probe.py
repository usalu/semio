"""M9: export casts and print their index (bounds, frames, fps, duration): m9_probe.py <cast> ..."""
import importlib
import json
import sys

S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
folders = S.ensure_casts(list(sys.argv[1:]))
for cast, folder in folders.items():
    index = json.load(open(folder / "🔣️.json", encoding="utf8"))
    print(cast, "frames", len(index["frames"]), "fps", index["fps"], "dur", index["durationS"], "loop", index["loop"],
          "bounds", {k: round(v, 3) for k, v in index["boundsM"].items()}, "folder", folder)
