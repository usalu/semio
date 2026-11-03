"""🌡️ tp: export every thermal cast of the manifest (or the ids given) and print index timing."""
import importlib
import json
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
manifest = json.load(open(ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json", encoding="utf8"))["casts"]
ids = sys.argv[1:] or [k for k, v in manifest.items() if v["view"] == "thermal"]
out = {}
for cast in ids:
    folder = S.ensure_casts([cast])[cast]
    index = json.load(open(folder / "🔣️.json", encoding="utf8"))
    out[cast] = str(folder)
    entry = manifest[cast]
    print(cast, entry.get("asset"), entry.get("clip") or entry.get("pose"), entry.get("environment"), "frames", len(index["frames"]), "fps", round(index["fps"], 3), "dur", index["durationS"], "loop", index["loop"], flush=True)
json.dump(out, open(Path(__file__).parent / "folders.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
