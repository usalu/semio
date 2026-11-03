"""📦 bj: export casts through the staffage API and write their cache folders to folders.json: export.py <cast> ..."""
import importlib
import json
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
folders = {cast: str(S.ensure_casts([cast])[cast]) for cast in sys.argv[1:]}
(Path(__file__).parent / "folders.json").write_text(json.dumps(folders, ensure_ascii=False, indent=1), encoding="utf-8")
print(json.dumps(folders, ensure_ascii=False))
