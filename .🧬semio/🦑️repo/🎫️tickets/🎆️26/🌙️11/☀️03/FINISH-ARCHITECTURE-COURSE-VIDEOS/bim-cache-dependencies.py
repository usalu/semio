"""🔎 Inventory native BIM geometry producer sources and discovered imports."""
import importlib
import inspect
import hashlib
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = next(path for path in HERE.parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
producers, files, unresolved = [], set(), []
for producer in api.geometry_producers():
    try:
        file = Path(inspect.getsourcefile(inspect.unwrap(producer))).resolve()
        relative = file.relative_to(ROOT).as_posix()
        files.add(file)
        producers.append({"module": inspect.unwrap(producer).__module__, "name": inspect.unwrap(producer).__qualname__, "file": relative})
    except (TypeError, ValueError):
        unresolved.append(repr(producer))
closure = set(files)
for file in list(files):
    closure.update(path.resolve() for path in runtime.source_dependencies(file))
result = {"producers": producers, "producerFiles": sorted(path.relative_to(ROOT).as_posix() for path in files),
          "literalImportClosure": sorted(path.relative_to(ROOT).as_posix() for path in closure),
          "sha256": {path.relative_to(ROOT).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(closure)}, "unresolved": unresolved}
(HERE / "🗑️generated" / "bim-dependencies.json").write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print("producers", len(producers), "producer files", len(files), "literal closure", len(closure), "unresolved", len(unresolved), flush=True)
