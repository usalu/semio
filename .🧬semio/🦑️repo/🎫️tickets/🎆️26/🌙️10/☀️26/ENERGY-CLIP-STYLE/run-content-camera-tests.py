"""🧪 Run focused content camera verification with ticket-owned temporary output."""

import importlib
import os
import sys
import unittest
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
OUTPUT = Path(__file__).resolve().parent / "🗑️generated/content-camera-tests"
OUTPUT.mkdir(parents=True, exist_ok=True)
os.environ.update(SEMIO_TEACHING_OUTPUT_ROOT=str(OUTPUT), PYTHONDONTWRITEBYTECODE="1", PYTHONIOENCODING="utf-8")
BASE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation"
selection = ".🖱️interaction" if "--interaction" in sys.argv else ""
if selection:
    sys.argv.remove("--interaction")
module = importlib.import_module(BASE + selection + ".🧪️tests.🐍️")
loader = unittest.TestLoader()
suite = unittest.TestSuite(loader.loadTestsFromName(name, module) for name in sys.argv[1:]) if len(sys.argv) > 1 else loader.loadTestsFromModule(module)
result = unittest.TextTestRunner(verbosity=2).run(suite)
raise SystemExit(not result.wasSuccessful())
