"""test_native_lifecycle for one folder and language with working-set sampling and a kill guard. Usage: xl4_lifecycle.py FOLDER LANG  (env XL4_GUARD_GB)"""
import importlib
import sys
import unittest

import xl4_common as C

folder, language = sys.argv[1], sys.argv[2]
visuals = C.load()
C.phases(visuals)
C.watch()
sys.argv = [sys.argv[0]]
module = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🧪️tests.🎭️presentation.🐍️")
original = module.discover
module.discover = lambda: [entry for entry in original() if entry["folder"] == folder]
module.CASES["languages"] = [language]
base_scene_class = module.scene_class


def traced(entry):
    C.STATE["where"] = f"{entry['scene']}:{language}"
    return base_scene_class(entry)


module.scene_class = traced
drawing_class = visuals.NakaginDrawing
original_axon = drawing_class.tower_axon
calls = []


def counted(self, *args, **kwargs):
    calls.append(1)
    C.say(f"tower_axon call #{len(calls)} templates={len(visuals._TOWERS)}")
    return original_axon(self, *args, **kwargs)


drawing_class.tower_axon = counted
result = unittest.TextTestRunner(verbosity=2).run(unittest.TestSuite([module.SceneContract("test_native_lifecycle")]))
C.say(f"done ok={result.wasSuccessful()} run={result.testsRun} fail={len(result.failures)} err={len(result.errors)} tower_axon calls={len(calls)}")
print(f"[xl4] RESULT lifecycle peak_mb={C.peak() / 1024**2:.0f} ok={result.wasSuccessful()}", flush=True)
