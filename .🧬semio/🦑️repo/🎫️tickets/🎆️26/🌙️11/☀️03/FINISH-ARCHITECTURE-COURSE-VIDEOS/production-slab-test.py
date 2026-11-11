import importlib
import importlib.util
import os
import sys
import unittest
from pathlib import Path
from scipy.spatial import ConvexHull
from shapely.geometry import Polygon, box
import numpy as np

ROOT = Path.cwd()
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_BIM_SLAB_CACHE"] = str(HERE / "🗑️generated" / "production-slab-final")
api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
cache = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.♻️cache.🐍️")
spec = importlib.util.spec_from_file_location("fixture", ROOT / "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🚬️prisms.py")
fixture = importlib.util.module_from_spec(spec)
spec.loader.exec_module(fixture)

class ProductionSlab(unittest.TestCase):
    def test_cold_warm_and_independent_geometry(self):
        model, owner, slab = fixture.synthetic()
        folder = Path(os.environ["SEMIO_BIM_SLAB_CACHE"])
        folder.mkdir(parents=True, exist_ok=True)
        self.assertFalse(list(folder.glob("*.json")), "test folder must start empty")
        events = []
        cold = api._slab_geometry_entry(model, slab, exact=True, progress=events.append)
        self.assertTrue(any(event.get("status") == "published" for event in events))
        api._SLAB_GEOMETRIES.clear()
        events.clear()
        warm = api._slab_geometry_entry(model, slab, exact=True, progress=events.append)
        self.assertTrue(any(event.get("status") == "hit" for event in events))
        self.assertEqual(cold, warm)
        structural = sum(Polygon(part["outline"], part["voids"]).area * float(part["z1"] - part["z0"]) for part in cold["parts"])
        expected = box(-1, -1, 1, 1).difference(box(-0.075, -0.075, 0.075, 0.075)).area * 0.17
        self.assertAlmostEqual(structural, expected, places=8)
        self.assertAlmostEqual(sum(ConvexHull(np.asarray(api.part_vertices(part), dtype=float)).volume for part in cold["shared_finish_parts"]), 0.009, places=9)

if __name__ == "__main__":
    unittest.main()
