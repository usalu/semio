"""🌍 Energy diagram translations use explicit English and German."""
import ast
import importlib
import json
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = next(path for path in HERE.parents if (path / "🧰️framework").is_dir())
import sys
sys.path.insert(0, str(ROOT))
FONT = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
CASES = json.loads((HERE / "label-cases.json").read_text(encoding="utf-8"))


class Labels(unittest.TestCase):
    """🔤 Both authored translations render with the installed course font."""

    def test_labels(self):
        for case in CASES:
            path = ROOT / "🎓️teaching/🏛️architecture/⚡️energy" / case["file"]
            tree = ast.parse(path.read_text(encoding="utf-8"))
            pairs = [(node.args[0].value, node.args[1].value) for node in ast.walk(tree) if isinstance(node, ast.Call) and isinstance(node.func, ast.Name) and node.func.id == "tr" and len(node.args) == 2 and all(isinstance(arg, ast.Constant) and isinstance(arg.value, str) for arg in node.args)]
            with self.subTest(file=case["file"]):
                self.assertIn((case["en"], case["de"]), pairs)
                for language in ("en", "de"):
                    self.assertGreater(FONT.body_text(case[language], font_size=FONT.LABEL_FONT_SIZE, color="#ffffff").width, 0)

    def test_surface_heading_translates_floor(self):
        path = ROOT / "🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🔁️recap/🐍️.py"
        tree = ast.parse(path.read_text(encoding="utf-8"))
        scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == "Beat2_Verlustseite")
        assignment = next(node for node in ast.walk(scene) if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == "heads" for target in node.targets))
        translated = [node for node in ast.walk(assignment.value) if isinstance(node, ast.Call) and isinstance(node.func, ast.Name) and node.func.id == "tr"]
        self.assertEqual(len(translated), 1)
        mapping = ast.literal_eval(translated[0].args[0].value)
        self.assertEqual(mapping["Boden"], "Floor")
        self.assertGreater(FONT.body_text(mapping["Boden"], font_size=FONT.LABEL_FONT_SIZE, color="#ffffff").width, 0)


if __name__ == "__main__":
    unittest.main()
