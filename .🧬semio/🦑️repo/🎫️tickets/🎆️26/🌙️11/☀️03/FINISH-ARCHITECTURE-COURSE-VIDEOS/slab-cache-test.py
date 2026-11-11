"""🧪 Exact slab cache prototype acceptance against Shapely."""
import importlib.util
import json
import uuid
from unittest.mock import patch
import os
import subprocess
import sys
import threading
import time
import unittest
from fractions import Fraction
from pathlib import Path

import jsonschema
from shapely.geometry import Polygon

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("slab_cache", HERE / "slab-cache.py")
CACHE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CACHE)
CASES = json.loads((HERE / "slab-cache-cases.json").read_text(encoding="utf-8"))
ROOT = HERE / "🗑️generated" / "slab-cache-test"
ROOT.mkdir(parents=True, exist_ok=True)
SOURCE = ROOT / "producer.py"
SOURCE.write_text("producer-v1", encoding="utf-8")


def producer_a():
    return 1


def producer_b():
    return 2


def safe_get(*args, **kwargs):
    """🧪 Exercise cache mechanics with a fixed eligible fixture producer."""
    return CACHE.get_or_compute(*args, eligibility=lambda: True, producer_slots=(("__main__", "fixture"),), **kwargs)


def sample():
    """📐 Exact geometry sample of a rectangular slab."""
    return {"points": tuple((Fraction(x), Fraction(y)) for x, y in CASES["model"]["vertices"]),
            "height": Fraction(CASES["model"]["height"]), "volume": Fraction(CASES["expectedVolume"]),
            "nested": {(Fraction(1, 3), "owner"): [True, None, 1.25, 2**80, -(2**100)]}}


def worker(folder, marker):
    """👷 Competing writer appends only if it actually computes the shared value."""
    def build(cancelled, progress):
        with Path(marker).open("a", encoding="utf-8") as handle:
            handle.write("compute\n")
        time.sleep(0.5)
        return sample()
    safe_get(CASES["model"], CASES["slab"], ROOT, [SOURCE], Path(folder), build)


def holder(path, ready):
    """🔒 Hold a lock until the test terminates this process."""
    with CACHE.lock(Path(path), lambda: False, lambda _: None):
        Path(ready).write_text("ready", encoding="utf-8")
        time.sleep(30)


class SlabCache(unittest.TestCase):
    """🧬 Portable exactness, invalidation, isolation and cancellation."""

    def setUp(self):
        self.folder = ROOT / uuid.uuid4().hex[:8]

    def test_noncanonical_producers_decline_persistence(self):
        files = [SOURCE, Path(__file__)]
        slots = ((producer_a.__module__, producer_a.__qualname__),)
        original = producer_a
        loaded = {str(path.resolve()): __import__("hashlib").sha256(path.read_bytes()).hexdigest() for path in files}
        registry = CACHE.register_producers([original], slots, HERE, files, loaded)
        self.assertIsNotNone(registry)
        self.assertTrue(CACHE.producers_current(registry, [original]))
        self.assertFalse(CACHE.producers_current(registry, [producer_b]))
        original_code = original.__code__
        try:
            original.__code__ = producer_b.__code__
            self.assertFalse(CACHE.producers_current(registry, [original]))
        finally:
            original.__code__ = original_code
        SOURCE.write_text("changed-after-registration", encoding="utf-8")
        try:
            self.assertIsNone(CACHE.register_producers([original], slots, HERE, files, loaded))
            self.assertFalse(CACHE.producers_current(registry, [original]))
            phases = []
            result = CACHE.get_or_compute(CASES["model"], CASES["slab"], HERE, files, self.folder, lambda *_: sample(),
                                          eligibility=lambda: CACHE.producers_current(registry, [original]), producer_slots=slots, progress=phases.append)
            self.assertEqual(result, sample())
            self.assertEqual(phases, ["ineligible"])
            self.assertFalse(self.folder.exists())
        finally:
            SOURCE.write_text("producer-v1", encoding="utf-8")
        self.assertIsNone(CACHE.register_producers([producer_b], slots, HERE, files, loaded))
        def local():
            return 1
        self.assertIsNone(CACHE.register_producers([local], ((local.__module__, local.__qualname__),), HERE, files, loaded))

    def test_round_trip_and_shapely_volume(self):
        exact = sample()
        phases = []
        build = lambda cancelled, progress: exact
        first = safe_get(CASES["model"], CASES["slab"], ROOT, [SOURCE], self.folder, build, progress=phases.append)
        second = safe_get(CASES["model"], CASES["slab"], ROOT, [SOURCE], self.folder, lambda *_: self.fail("unexpected recomputation"), progress=phases.append)
        self.assertEqual(first, exact)
        self.assertEqual(second, exact)
        schema = json.loads((HERE / "slab-cache.schema.json").read_text(encoding="utf-8"))
        record = json.loads(next(self.folder.glob("*.json")).read_text(encoding="utf-8"))
        jsonschema.validate(record, schema)
        self.assertEqual(phases, ["compute", "published", "hit"])
        self.assertIsInstance(second["points"], tuple)
        self.assertIsInstance(second["height"], Fraction)
        shape = Polygon(CASES["model"]["vertices"])
        self.assertAlmostEqual(float(second["volume"]), shape.area * CASES["rectangle"]["height"])

    def test_integers_are_canonical_exact_decimals(self):
        for value in map(int, CASES["exactIntegers"]):
            with self.subTest(value=value):
                self.assertEqual(CACHE.decode(CACHE.encode(value)), value)
                self.assertEqual(CACHE.encode(value), ["integer", str(value)])
        for text in CASES["invalidIntegers"]:
            with self.subTest(text=text), self.assertRaises(ValueError):
                CACHE.decode(["integer", text])

    def test_model_and_producer_invalidate(self):
        a = CACHE.identity(CASES["model"], CASES["slab"], ROOT, [SOURCE])
        model = dict(CASES["model"], height="1/3")
        self.assertNotEqual(a, CACHE.identity(model, CASES["slab"], ROOT, [SOURCE]))
        SOURCE.write_text("producer-v2", encoding="utf-8")
        try:
            self.assertNotEqual(a, CACHE.identity(CASES["model"], CASES["slab"], ROOT, [SOURCE]))
        finally:
            SOURCE.write_text("producer-v1", encoding="utf-8")

    def test_each_declared_dependency_class_invalidates(self):
        self.folder.mkdir(parents=True)
        files = []
        for kind in ("producer", "backend", "schema", "asset"):
            path = self.folder / (kind + ".txt")
            path.write_text(kind + "-v1", encoding="utf-8")
            files.append(path)
        baseline = CACHE.identity(CASES["model"], CASES["slab"], ROOT, files)
        for path in files:
            original = path.read_text(encoding="utf-8")
            path.write_text(original + "-changed", encoding="utf-8")
            with self.subTest(kind=path.stem):
                self.assertNotEqual(baseline, CACHE.identity(CASES["model"], CASES["slab"], ROOT, files))
            path.write_text(original, encoding="utf-8")

    def test_concurrent_writers_compute_once(self):
        marker = ROOT / (self.folder.name + ".txt")
        commands = [[sys.executable, "-B", str(HERE / "slab-cache-test.py"), "--worker", str(self.folder), str(marker)] for _ in range(CASES["writers"])]
        workers = [subprocess.Popen(command) for command in commands]
        self.assertEqual([p.wait(timeout=30) for p in workers], [0] * len(workers))
        self.assertEqual(marker.read_text(encoding="utf-8").splitlines(), ["compute"])
        self.assertEqual(len(list(self.folder.glob("*.json"))), 1)

    def test_wait_cancellation_and_process_death(self):
        self.folder.mkdir(parents=True)
        path = self.folder / "guard.lock"
        ready = self.folder / "ready"
        process = subprocess.Popen([sys.executable, str(HERE / "slab-cache-test.py"), "--holder", str(path), str(ready)])
        try:
            deadline = time.monotonic() + 20
            while not ready.exists() and time.monotonic() < deadline:
                time.sleep(0.02)
            self.assertTrue(ready.exists())
            cancel = threading.Event()
            phases = []
            timer = threading.Timer(0.15, cancel.set)
            timer.start()
            with self.assertRaises(InterruptedError):
                with CACHE.lock(path, cancel.is_set, phases.append):
                    self.fail("acquired occupied lock")
            timer.join()
            self.assertEqual(phases, ["waiting"])
        finally:
            process.terminate()
            process.wait(timeout=20)
        with CACHE.lock(path, lambda: False, lambda _: None):
            pass

    def test_changed_source_never_publishes(self):
        def mutate(cancelled, progress):
            SOURCE.write_text("mutated", encoding="utf-8")
            return sample()
        try:
            with self.assertRaises(RuntimeError):
                safe_get(CASES["model"], CASES["slab"], ROOT, [SOURCE], self.folder, mutate)
            self.assertEqual(list(self.folder.glob("*.json")), [])
        finally:
            SOURCE.write_text("producer-v1", encoding="utf-8")

    def test_cancelled_build_does_not_publish(self):
        cancel = threading.Event()
        def build(cancelled, progress):
            cancel.set()
            return sample()
        with self.assertRaises(InterruptedError):
            safe_get(CASES["model"], CASES["slab"], ROOT, [SOURCE], self.folder, build, cancelled=cancel.is_set)
        self.assertEqual(list(self.folder.glob("*.json")), [])
        self.assertEqual(list(self.folder.glob("*.tmp")), [])

    def test_source_change_during_decode_or_encode(self):
        safe_get(CASES["model"], CASES["slab"], ROOT, [SOURCE], self.folder, lambda *_: sample())
        original_decode = CACHE.decode
        changed = False
        def mutate_decode(node):
            nonlocal changed
            if not changed:
                changed = True
                SOURCE.write_text("changed-during-decode", encoding="utf-8")
            return original_decode(node)
        try:
            with patch.object(CACHE, "decode", side_effect=mutate_decode), self.assertRaises(RuntimeError):
                safe_get(CASES["model"], CASES["slab"], ROOT, [SOURCE], self.folder, lambda *_: sample())
        finally:
            SOURCE.write_text("producer-v1", encoding="utf-8")
        other = ROOT / uuid.uuid4().hex[:8]
        result = sample()
        original_encode = CACHE.encode
        def mutate_encode(value):
            if value is result:
                SOURCE.write_text("changed-during-encode", encoding="utf-8")
            return original_encode(value)
        try:
            with patch.object(CACHE, "encode", side_effect=mutate_encode), self.assertRaises(RuntimeError):
                safe_get(CASES["model"], CASES["slab"], ROOT, [SOURCE], other, lambda *_: result)
            self.assertEqual(list(other.glob("*.json")), [])
        finally:
            SOURCE.write_text("producer-v1", encoding="utf-8")

    def test_cancellation_and_tampering(self):
        event = threading.Event()
        event.set()
        with self.assertRaises(InterruptedError):
            safe_get(CASES["model"], CASES["slab"], ROOT, [SOURCE], self.folder, lambda *_: sample(), cancelled=event.is_set)
        self.assertEqual(list(self.folder.glob("*.json")), [])
        safe_get(CASES["model"], CASES["slab"], ROOT, [SOURCE], self.folder, lambda *_: sample())
        record = next(self.folder.glob("*.json"))
        data = json.loads(record.read_text(encoding="utf-8"))
        data["value"] = ["scalar", "forged"]
        record.write_text(json.dumps(data), encoding="utf-8")
        phases = []
        restored = safe_get(CASES["model"], CASES["slab"], ROOT, [SOURCE], self.folder, lambda *_: sample(), progress=phases.append)
        self.assertEqual(restored, sample())
        self.assertEqual(phases, ["corrupt", "compute", "published"])


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "--worker":
        worker(*sys.argv[2:])
    elif len(sys.argv) > 1 and sys.argv[1] == "--holder":
        holder(*sys.argv[2:])
    else:
        unittest.main()
