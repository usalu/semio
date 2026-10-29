"""Run the series presentation lifecycle check for the casting scene only (both languages)."""
import importlib.util
import sys
import tempfile
import time
import unittest
from pathlib import Path

TICKET = Path(__file__).resolve().parents[1]
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
path = ROOT / "🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️tests/🎭️presentation/🐍️.py"
spec = importlib.util.spec_from_file_location("presentation_contract", path)
module = importlib.util.module_from_spec(spec)
sys.modules["presentation_contract"] = module
spec.loader.exec_module(module)
casting = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🫗️casting.🐍️")


class Only(module.GrasshopperPresentationContract):
    @classmethod
    def setUpClass(cls):
        cls.scenes = [("🫗️casting", casting.Beat4_Casting, Path(casting.__file__))]

    def test_native_lifecycle(self):
        output = Path(tempfile.mkdtemp(prefix="gh-casting-"))
        for language in ("en", "de"):
            start = time.perf_counter()
            self.lifecycle("Beat4_Casting", casting.Beat4_Casting, language, output)
            print(f"[DEBUG] lifecycle {language} {time.perf_counter() - start:.1f}s", flush=True)


if __name__ == "__main__":
    suite = unittest.TestSuite()
    for name in ("test_narration_is_bilingual_and_unique", "test_dispatch_and_step_keys", "test_imports_resolve", "test_provenance_files_exist", "test_language_is_explicit"):
        if hasattr(Only, name):
            suite.addTest(Only(name))
    unittest.TextTestRunner(verbosity=2).run(suite)
