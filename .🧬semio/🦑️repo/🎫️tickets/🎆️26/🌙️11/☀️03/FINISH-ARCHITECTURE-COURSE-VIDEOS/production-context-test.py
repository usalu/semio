import importlib
import sys
import unittest
from pathlib import Path
import numpy as np

sys.path.insert(0, str(Path.cwd()))
api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
cache = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.♻️cache.🐍️")

class ContextTest(unittest.TestCase):
    def test_global_key_and_noncanonical_fallback(self):
        bindings = api.geometry_producers()
        original = cache.canonical_context(bindings)
        self.assertIsNotNone(original)
        original_hash = cache.digest(cache.encode(original[2]))
        value = api.ARC_STEP
        try:
            api.ARC_STEP = value + 1
            self.assertFalse(original[3](bindings))
            changed = cache.canonical_context(bindings)
            self.assertIsNotNone(changed)
            self.assertNotEqual(original_hash, cache.digest(cache.encode(changed[2])))
            self.assertTrue(np.isclose(api.ARC_STEP, value + 1))
        finally:
            api.ARC_STEP = value
        self.assertIsNone(cache.canonical_context((lambda: None,) + bindings[1:]))

    def test_container_types_remain_distinct(self):
        self.assertNotEqual(cache._global_value([1]), cache._global_value((1,)))
        self.assertNotEqual(cache._global_value({1}), cache._global_value(frozenset({1})))

    def test_code_replacement_falls_back(self):
        bindings = api.geometry_producers()
        original = cache.canonical_context(bindings)
        function = bindings[0]
        code = function.__code__
        def replacement(*args, **kwargs):
            return 0
        try:
            function.__code__ = replacement.__code__
            self.assertFalse(original[3](bindings))
            self.assertIsNone(cache.canonical_context(bindings))
        finally:
            function.__code__ = code

if __name__ == "__main__":
    unittest.main()
