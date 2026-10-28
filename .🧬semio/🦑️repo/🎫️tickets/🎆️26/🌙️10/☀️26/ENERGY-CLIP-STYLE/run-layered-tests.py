"""🧪 Run portable hierarchy and real layered browser verification inside the existing teaching environment."""
import os
import sys
import unittest
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
sys.stdout.reconfigure(encoding="utf-8")
sys.stderr.reconfigure(encoding="utf-8")
OUTPUT = Path(__file__).with_name("🗑️generated") / "layered-tests"
OUTPUT.mkdir(parents=True, exist_ok=True)
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(OUTPUT)
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(OUTPUT / "fonts")
PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation"
names = sys.argv[1:] or [PACKAGE + ".🥞️hierarchy.🧪️tests.🐍️", PACKAGE + ".🌐️browser.🧪️tests.🐍️"]
suite = unittest.defaultTestLoader.loadTestsFromNames(names)
result = unittest.TextTestRunner(verbosity=2).run(suite)
raise SystemExit(not result.wasSuccessful())
