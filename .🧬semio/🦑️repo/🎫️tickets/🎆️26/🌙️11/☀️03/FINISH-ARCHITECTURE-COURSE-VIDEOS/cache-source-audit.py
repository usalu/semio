import importlib
import sys
from pathlib import Path
sys.path.insert(0, str(Path.cwd()))
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
model = Path("🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🐍️.py").resolve()
cache = model.parent / "♻️cache"
dependencies = {p.resolve() for p in runtime.source_dependencies(model)}
assert (cache / "🐍️.py").resolve() in dependencies
assets = {p.resolve() for source in dependencies for p in source.parent.rglob("*.json")}
assert (cache / "🔣️.manifest.json").resolve() in assets
assert (cache / "🔣️.schema.json").resolve() in assets
print("production cache module, manifest and schema enter segment fingerprints")
