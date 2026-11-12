"""Validates the Grasshopper batch transition documents with the spec validator: validate_docs.py"""
import importlib
import sys
from pathlib import Path

spec = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📋️spec.🐍️")
full = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🎞️full.🐍️")
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
CLIPS = ROOT / "🎓️teaching" / "🏛️architecture" / "🦗️grasshopper" / "🎬️clip"
failed = 0
for path in sorted(CLIPS.glob("*/🔀️transitions/🔣️.json")):
    problems = spec.validate(spec.load(path).__class__ and __import__("json").loads(path.read_text(encoding="utf-8")))
    print(path.parent.parent.name, len(spec.load(path).transitions), "transitions", "OK" if not problems else problems, flush=True)
    failed += bool(problems)
sys.exit(1 if failed else 0)
