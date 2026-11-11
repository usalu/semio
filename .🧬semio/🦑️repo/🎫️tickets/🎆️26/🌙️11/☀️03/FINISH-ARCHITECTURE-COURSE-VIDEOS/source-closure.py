"""🔎 List canonical lessons whose source closures include one authored file."""
import importlib
import json
import sys
from pathlib import Path

sys.path.insert(0, str(next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())))
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
root = Path("🎓️teaching").resolve()
target = Path(sys.argv[1]).resolve(strict=True)
found = {}
for video in runtime.load_manifest():
    scenes = []
    for chapter in video["chapters"]:
        for segment in chapter["segments"]:
            if target in {path.resolve() for path in runtime.source_dependencies(root / segment["script"])}:
                scenes.append(segment["scene"])
    if scenes:
        found[video["id"]] = scenes
print(json.dumps({"target": str(target), "lessons": found}, ensure_ascii=False))
