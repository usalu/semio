"""🧪 Check native chrome against independently measured React and Chromium fixtures."""

import importlib
import json
import os
import sys
import unittest
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
sys.stdout.reconfigure(encoding="utf-8")
sys.stderr.reconfigure(encoding="utf-8")
OUTPUT = Path(__file__).with_name("🗑️generated") / "exact-tests"
OUTPUT.mkdir(parents=True, exist_ok=True)
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(OUTPUT)
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(OUTPUT / "fonts")
PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring"
PRESENTATION = PACKAGE + ".🎬️clip.🪟️presentation"
browser_only = len(sys.argv) > 1 and sys.argv[1] == "--browser"
if browser_only:
    sys.argv.pop(1)
module = importlib.import_module(PRESENTATION + (".🌐️browser" if browser_only else ".📐️geometry") + ".🧪️tests.🐍️")
name = sys.argv[1] if len(sys.argv) > 1 else None
suite = unittest.defaultTestLoader.loadTestsFromName(name, module) if name else unittest.defaultTestLoader.loadTestsFromModule(module)
result = unittest.TextTestRunner(verbosity=2).run(suite)
raise SystemExit(not result.wasSuccessful())
