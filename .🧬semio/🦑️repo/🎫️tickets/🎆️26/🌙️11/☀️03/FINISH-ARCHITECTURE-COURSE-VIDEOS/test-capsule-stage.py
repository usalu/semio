"""Verify the capsule stage chooses the authored Center Box geometry."""
import sys
from pathlib import Path
sys.path.insert(0, str(Path.cwd()))
import importlib
import unittest
from types import SimpleNamespace
from unittest.mock import Mock
from manim import VGroup, ValueTracker

capsule = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.💊️capsule.🐍️")
names = ("grid", "env", "hit", "body", "round", "window", "dw", "dl", "dh", "dd", "du", "bolts", "axes", "plane2", "over", "dr", "wrong", "front", "lift")

class StageBoxTest(unittest.TestCase):
    def test_center_box_is_not_fabricated_capsule(self):
        rv = {name: ValueTracker(1 if name in ("body", "lift") else 0) for name in names}
        geometry = VGroup()
        geometry.dims = {}
        build_box = Mock(return_value=geometry)
        build_capsule = Mock(return_value=geometry)
        params = dict(width=2.6, length=4.2, height=2.4, back=0, front=4.2, corner_radius=.2,
                      window_diameter=1.2, window_u=0, window_v=1.2, glazing_y=4.2)
        scene = SimpleNamespace(st=SimpleNamespace(rv=rv, oy=ValueTracker(0), az=ValueTracker(55)),
            _solve=lambda: None, _read=lambda _: params, _grid=lambda _: VGroup(),
            capsule_axon=build_capsule, analytical_box_axon=build_box)
        scene._preview = capsule.Beat2_Capsule._preview.__get__(scene)
        capsule.Beat2_Capsule._stage(scene)
        build_box.assert_called_once()
        build_capsule.assert_not_called()

if __name__ == "__main__":
    unittest.main()
