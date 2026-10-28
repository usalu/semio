"""📐 Recheck only the current model translation layout through the existing native oracle."""
import copy
import importlib
import importlib.util
from pathlib import Path
import sys
import unittest
from unittest.mock import patch

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
DRIVER = ROOT / "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🎭️presentation/🐍️.py"


def main():
    """🎞️ Retain real typography, model, animation and Cairo; select one authored beat for this layout review."""
    spec = importlib.util.spec_from_file_location("inspection_model_translation", DRIVER)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    chapter = copy.deepcopy(next(case for case in module.CASES["chapters"] if case["topic"] == "models"))
    chapter["keys"] = ["translation"]
    module.CASES["chapters"] = [chapter]
    scene_module = importlib.import_module("🎓️teaching." + chapter["script"].removesuffix(".py").replace("/", "."))
    scene = getattr(scene_module, chapter["scene"])
    engine = importlib.import_module("manim")

    def translation(instance):
        """🔁 Start with an empty previous stage and execute the unchanged production translation method."""
        instance.stage, instance.ghost = engine.VGroup(), None
        instance._translation()

    with patch.object(scene, "present_models", translation):
        case = module.BimPresentationContract("test_native_lifecycle")
        result = unittest.TextTestRunner(verbosity=1).run(unittest.TestSuite([case]))
    if result.wasSuccessful():
        (TICKET / "inspection-model-layout.md").write_text(
            "# Inspection Model Translation Layout\n\nThe focused English/German translation-only native oracle passed after the footer-position fix. "
            "It uses the actual production model, typography, native animation lifecycle and Cairo capture while starting with an empty prior stage. "
            "Current clause snapshots are generated/ib/models-en-translation.png and models-de-translation.png. "
            "The scout separately reviews the mapping/footer gap; complete model lifecycle and final movie acceptance remain separate.\n",
            encoding="utf-8")
    return not result.wasSuccessful()


if __name__ == "__main__":
    sys.exit(main())
