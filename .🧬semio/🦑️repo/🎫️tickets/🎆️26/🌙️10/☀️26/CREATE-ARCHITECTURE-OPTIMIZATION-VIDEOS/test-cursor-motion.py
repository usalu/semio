"""🧪 Verify calm cursor examples with an independent polynomial oracle."""

import copy
import importlib.util
import json
import math
from pathlib import Path
import sys
import tempfile
import unittest

import numpy as np
from numpy.polynomial import Polynomial

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parent
CASES = json.loads((ROOT / "cursor-motion.cases.json").read_text(encoding="utf-8"))
SPEC = importlib.util.spec_from_file_location("calm_cursor_motion", ROOT / "cursor-motion.py")
MOTION = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MOTION)
RENDERER_SPEC = importlib.util.spec_from_file_location("calm_cursor_renderer", ROOT / "render-calm-cursor.py")
RENDERER = importlib.util.module_from_spec(RENDERER_SPEC)
RENDERER_SPEC.loader.exec_module(RENDERER)


class CalmCursorTests(unittest.TestCase):
    def test_native_capture_delivery_schedule(self):
        case = CASES["resampling"]
        plan = RENDERER.frame_schedule(CASES["durationSeconds"], case["deliveryFps"], case["captureFps"])
        self.assertEqual(plan["native_count"], case["nativeCount"])
        self.assertEqual(plan["output_count"], case["outputCount"])
        native_times = np.arange(case["nativeCount"]) / case["captureFps"]
        output_ticks = np.arange(case["outputCount"])
        quantized_starts = np.floor_divide(np.arange(case["nativeCount"]) * case["deliveryFps"], case["captureFps"])
        expected = np.searchsorted(quantized_starts, output_ticks, side="right") - 1
        np.testing.assert_array_equal(plan["native_indices"], expected)
        for output_index, native_index in case["outputToNative"]:
            self.assertEqual(plan["native_indices"][output_index], native_index)
        native = [MOTION.sample(float(t) / CASES["durationSeconds"], CASES["park"], CASES["target"]) for t in native_times]
        output = [native[index] for index in plan["native_indices"]]
        self.assertEqual(sum(item[1] for item in native), case["nativeHoverCount"])
        self.assertEqual(sum(item[1] for item in output), case["outputHoverCount"])
        for index, item in enumerate(output):
            exact = MOTION.sample(index / case["outputCount"], CASES["park"], CASES["target"])
            self.assertEqual(item[1:], exact[1:])

    def test_portable_samples(self):
        for case in CASES["samples"]:
            with self.subTest(seconds=case["seconds"]):
                point, revealed, stage = MOTION.sample(case["seconds"] / CASES["durationSeconds"], CASES["park"], CASES["target"])
                np.testing.assert_allclose(point, case["point"], atol=1e-8, rtol=0)
                self.assertIs(revealed, case["revealed"])
                self.assertEqual(stage, case["stage"])

    def test_contract_single_target_and_slow_legs(self):
        model = MOTION.load_contract()
        self.assertEqual(model["target"], {"kind": "topic", "id": "optimization"})
        self.assertEqual(model["fps"], 30)
        self.assertEqual(model["resolution"], [1600, 900])
        self.assertEqual([stage["id"] for stage in model["stages"]], ["idle", "approach", "hover", "return", "settle"])
        self.assertEqual(sum(stage["durationSeconds"] for stage in model["stages"]), CASES["durationSeconds"])
        for index in (1, 2, 3):
            self.assertGreaterEqual(model["stages"][index]["durationSeconds"], 2.5)
        self.assertNotIn("click", json.dumps(model).lower())

    def test_independent_polynomial_positions_and_speed(self):
        easing = Polynomial([0, 0, 3, -2])
        park, target = np.asarray(CASES["park"], dtype=float), np.asarray(CASES["target"], dtype=float)
        local = np.linspace(0, 1, 301)
        for start, source, destination in ((1.5, park, target), (7, target, park)):
            expected = source + easing(local)[:, None] * (destination - source)
            actual = np.array([MOTION.sample((start + float(t) * 2.5) / CASES["durationSeconds"], park.tolist(), target.tolist())[0] for t in local])
            np.testing.assert_allclose(actual, expected, atol=1e-8, rtol=0)
            measured = np.linalg.norm(np.diff(actual, axis=0), axis=1) / (2.5 / 300)
            middle = (local[:-1] + local[1:]) / 2
            oracle = easing.deriv()(middle) * np.linalg.norm(destination - source) / 2.5
            np.testing.assert_allclose(measured, oracle, atol=0.004, rtol=0)
            self.assertLess(measured[0], measured[len(measured) // 2] / 100)
            self.assertLess(measured[-1], measured[len(measured) // 2] / 100)

    def test_one_continuous_hover_plateau(self):
        final_tick = int(CASES["durationSeconds"] * 30)
        ticks = np.arange(final_tick + 1)
        samples = [MOTION.sample(float(tick) / final_tick, CASES["park"], CASES["target"]) for tick in ticks]
        revealed = np.array([sample[1] for sample in samples], dtype=bool)
        edges = np.flatnonzero(np.diff(np.r_[False, revealed, False]))
        np.testing.assert_array_equal(edges, [120, 210])
        for index in range(120, 210):
            np.testing.assert_array_equal(samples[index][0], CASES["target"])
            self.assertEqual(samples[index][2], "hover")

    def test_rest_and_continuous_endpoint_velocity(self):
        h = 1e-5
        for seconds in (0.5, 5.5, 10.5):
            before = np.array(MOTION.sample((seconds - h) / CASES["durationSeconds"], CASES["park"], CASES["target"])[0])
            after = np.array(MOTION.sample((seconds + h) / CASES["durationSeconds"], CASES["park"], CASES["target"])[0])
            np.testing.assert_array_equal(after, before)
        for seconds in (1.5, 4, 7, 9.5):
            at = np.array(MOTION.sample(seconds / CASES["durationSeconds"], CASES["park"], CASES["target"])[0])
            for offset in (-h, h):
                nearby = np.array(MOTION.sample((seconds + offset) / CASES["durationSeconds"], CASES["park"], CASES["target"])[0])
                self.assertLess(np.linalg.norm(nearby - at) / h, 0.01)

    def test_finite_inputs_only(self):
        for value in (float("nan"), float("inf"), -float("inf"), "half", None, True):
            with self.subTest(time=repr(value)), self.assertRaises((TypeError, ValueError)):
                MOTION.sample(value, CASES["park"], CASES["target"])
        for point in ([0], [0, 1, 2], [0, math.nan], [math.inf, 0], ["x", 0], None, [True, 0]):
            for field in ("park", "target"):
                args = dict(t=0.5, park=CASES["park"], target=CASES["target"])
                args[field] = point
                with self.subTest(field=field, point=repr(point)), self.assertRaises((TypeError, ValueError)):
                    MOTION.sample(**args)

    def test_same_point_is_stationary(self):
        for t in np.linspace(-1, 2, 41):
            point, _, _ = MOTION.sample(float(t), [5, 7], [5, 7])
            np.testing.assert_array_equal(point, [5, 7])

    def test_invalid_contracts_rejected(self):
        model = json.loads((ROOT / "cursor-motion.json").read_text(encoding="utf-8"))
        generated = ROOT / "🗑️generated"
        generated.mkdir(exist_ok=True)
        with tempfile.TemporaryDirectory(prefix="cursor-tests-", dir=generated) as temporary:
            candidate_path = Path(temporary) / "invalid.json"
            for case in CASES["invalidModels"]:
                candidate = copy.deepcopy(model)
                parent = candidate
                for key in case["path"][:-1]:
                    parent = parent[key]
                key = case["path"][-1]
                if case.get("remove"):
                    del parent[key]
                else:
                    parent[key] = case["value"]
                candidate_path.write_text(json.dumps(candidate), encoding="utf-8")
                with self.subTest(case=case["id"]), self.assertRaises((TypeError, ValueError)):
                    MOTION.load_contract(candidate_path)


if __name__ == "__main__":
    unittest.main(verbosity=2)
