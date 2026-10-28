"""🧪 Verify camera timing against independent portable NumPy navigation boundaries."""
from pathlib import Path
import importlib
import os
import sys
import unittest

ROOT = Path(__file__).resolve().parents[7]
WORK = Path(__file__).parent / "🗑️generated/content-camera"
WORK.mkdir(parents=True, exist_ok=True)
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(WORK)
module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🥞️hierarchy.🧪️tests.🐍️")
result = unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromModule(module))
raise SystemExit(not result.wasSuccessful())
