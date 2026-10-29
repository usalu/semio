import importlib.util
import sys
import unittest
from pathlib import Path

ROOT = Path("C:/git/semio")
sys.path.insert(0, str(ROOT))
path = next((ROOT / "🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️tests/🎭️presentation").glob("🐍️.py"))
spec = importlib.util.spec_from_file_location("gh_presentation", path)
module = importlib.util.module_from_spec(spec)
sys.modules["gh_presentation"] = module
spec.loader.exec_module(module)
cls = next(v for v in vars(module).values() if isinstance(v, type) and issubclass(v, unittest.TestCase) and hasattr(v, "test_every_engine_view_builds"))
suite = unittest.TestSuite([cls("test_every_engine_view_builds")])
unittest.TextTestRunner(verbosity=1).run(suite)
