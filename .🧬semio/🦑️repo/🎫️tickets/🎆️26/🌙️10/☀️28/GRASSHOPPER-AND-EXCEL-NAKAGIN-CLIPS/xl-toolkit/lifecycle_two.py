import faulthandler, importlib, importlib.util, sys, unittest
faulthandler.dump_traceback_later(1500, exit=True)
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
spec = importlib.util.spec_from_file_location("excel_presentation", ROOT / "🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🧪️tests/🎭️presentation/🐍️.py")
mod = importlib.util.module_from_spec(spec); spec.loader.exec_module(mod)
full = mod.discover
mod.discover = lambda: [e for e in full() if e["scene"] in sys.argv[1:]]
suite = unittest.TestSuite([mod.SceneContract("test_native_lifecycle")])
result = unittest.TextTestRunner(verbosity=1).run(suite)
print("[DEBUG] RESULT", sys.argv[1:], "ok" if result.wasSuccessful() else "FAILED", len(result.errors), len(result.failures))
