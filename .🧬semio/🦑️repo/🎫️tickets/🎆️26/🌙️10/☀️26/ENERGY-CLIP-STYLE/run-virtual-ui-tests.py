"""🧪 Run virtual presentation regression suites inside ticket output boundaries."""

import importlib
import io
import os
import sys
import unittest
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
sys.stdout.reconfigure(encoding="utf-8")
sys.stderr.reconfigure(encoding="utf-8")
OUTPUT = Path(__file__).with_name("🗑️generated") / "virtual-ui-tests"
OUTPUT.mkdir(parents=True, exist_ok=True)
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(OUTPUT)
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(OUTPUT / "fonts")
module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🧪️tests.🐍️")
mode = sys.argv[1] if len(sys.argv) > 1 else None
if mode == "--runtime":
    module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🧪️tests.🐍️")
name = mode if mode and mode != "--runtime" else None
suite = unittest.defaultTestLoader.loadTestsFromName(name, module) if name else unittest.defaultTestLoader.loadTestsFromModule(module)
transcript = io.StringIO()
result = unittest.TextTestRunner(stream=transcript, verbosity=2).run(suite)
log_name = "runtime.log" if mode == "--runtime" else ("tests-" + name.split(".")[0] + ".log" if name else "tests.log")
(OUTPUT / log_name).write_text(transcript.getvalue(), encoding="utf-8")
sys.stdout.reconfigure(encoding="utf-8")
print(transcript.getvalue())
raise SystemExit(not result.wasSuccessful())
