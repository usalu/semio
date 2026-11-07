"""🏢 Verify the index carbon glyph resolves and retains canonical staffage geometry."""

import importlib
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path.cwd()))
import numpy as np
from manim import Rectangle

scene = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.⚖️index.🐍️")


class IndexGlyphTest(unittest.TestCase):
    def test_carbon_glyph_matches_canonical_cast_and_manim_width(self):
        width = 0.34
        actual = scene.glyph("carbon", width=width)
        expected = scene._art.fitted_staffage("optimization-carbon-curtain-wall", width=width)
        expected.move_to((0, 0, 0))
        self.assertGreater(actual.height, 0)
        self.assertAlmostEqual(actual.width, Rectangle(width=width).width)
        self.assertAlmostEqual(actual.height, expected.height)
        np.testing.assert_allclose(actual.get_all_points(), expected.get_all_points())


if __name__ == "__main__":
    unittest.main()
