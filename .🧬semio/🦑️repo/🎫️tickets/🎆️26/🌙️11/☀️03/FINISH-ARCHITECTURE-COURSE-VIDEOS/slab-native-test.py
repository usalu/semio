"""🧱 Native small slab exact cache evidence against independent geometry."""
import importlib
import importlib.util
import inspect
import json
import sys
import unittest
from pathlib import Path

import numpy as np
from scipy.spatial import ConvexHull
from shapely.geometry import Polygon, box

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
ROOT = next(path for path in HERE.parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
cache_spec = importlib.util.spec_from_file_location("slab_cache", HERE / "slab-cache.py")
CACHE = importlib.util.module_from_spec(cache_spec)
cache_spec.loader.exec_module(CACHE)
api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
fixture_path = ROOT / "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🚬️prisms.py"
fixture_spec = importlib.util.spec_from_file_location("native_slab_fixture", fixture_path)
fixture = importlib.util.module_from_spec(fixture_spec)
fixture_spec.loader.exec_module(fixture)
inventory = json.loads((HERE / "🗑️generated/bim-dependencies.json").read_text(encoding="utf-8"))
FILES = [ROOT / name for name in inventory["literalImportClosure"]]


class NativeSlab(unittest.TestCase):
    """🔬 Native exact result survives cold persistence and independent volume checks."""

    def test_exact_small_slab_with_owned_reserve(self):
        model, owner, slab = fixture.synthetic()
        bindings = api.geometry_producers()
        slots = tuple((inspect.unwrap(p).__module__, inspect.unwrap(p).__qualname__) for p in bindings)
        loaded = {str(path.resolve()): __import__("hashlib").sha256(path.read_bytes()).hexdigest() for path in FILES}
        registry = CACHE.register_producers(bindings, slots, ROOT, FILES, loaded)
        self.assertIsNotNone(registry)
        eligible = lambda: CACHE.producers_current(registry, api.geometry_producers())
        self.assertTrue(eligible())
        folder = HERE / "🗑️generated" / "native-slab"
        folder.mkdir(parents=True, exist_ok=True)
        key = CACHE.identity(model, slab, ROOT, FILES, slots)
        (folder / (key + ".json")).unlink(missing_ok=True)
        phases = []
        build = lambda cancelled, progress: api._slab_geometry_entry(model, slab, exact=True, cancel=cancelled)
        cold = CACHE.get_or_compute(model, slab, ROOT, FILES, folder, build, eligibility=eligible, producer_slots=slots, progress=phases.append)
        warm = CACHE.get_or_compute(model, slab, ROOT, FILES, folder, lambda *_: self.fail("warm cache recomputed"), eligibility=eligible, producer_slots=slots, progress=phases.append)
        self.assertEqual(cold, warm)
        self.assertEqual(phases, ["compute", "published", "hit"])
        self.assertTrue(cold["shared_finish_parts"])
        structural = sum(Polygon(part["outline"], part["voids"]).area * float(part["z1"] - part["z0"]) for part in cold["parts"])
        expected = box(-1, -1, 1, 1).difference(box(-0.075, -0.075, 0.075, 0.075)).area * 0.17
        self.assertAlmostEqual(structural, expected, places=8)
        shared = sum(ConvexHull(np.asarray(api.part_vertices(part), dtype=float)).volume for part in cold["shared_finish_parts"])
        self.assertAlmostEqual(shared, 0.009, places=9)
        for part in cold["parts"] + cold["finish_parts"] + cold["shared_finish_parts"]:
            if "mesh" in part:
                oracle = ConvexHull(np.asarray(api.part_vertices(part), dtype=float)).volume
                exact = float(api._native_parts_exact_volume([part]))
                self.assertAlmostEqual(exact, oracle, places=8)


if __name__ == "__main__":
    unittest.main()
