"""🧪 Law of `verdict.py` with a stand-in product command (no official build): exact walls instead of 5-s steps, parallel stages overlap, `unless served:` skips, edit probes are byte-exact and alone, a failing run makes the verdict RED and stops the recipe. `python3 -I tests/test_verdict.py`."""
import json, os, shutil, subprocess, sys, tempfile, time, unittest

HERE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
TICKET = os.path.dirname(HERE)
FAKE = r'''
import os, sys, time
args = sys.argv[1:]
target = " ".join(args)
if "refresh" in target: print("graph: refreshing 3 projects (generation 9)", flush=True)
if "served" in target: print("graph: 5 projects served from the memo", flush=True)
probe = os.path.join(os.environ["VERDICT_ROOT"], "probe.ts")
print("probe-present" if b"gate probe" in open(probe, "rb").read() else "probe-absent", flush=True)
time.sleep(1.3)
sys.exit(1 if "fail" in target else 0)
'''


class VerdictLaw(unittest.TestCase):
    def setUp(self):
        base = os.path.join(TICKET, "🗑️generated", "gatefast")
        os.makedirs(base, exist_ok=True)
        self.dir = tempfile.mkdtemp(prefix="vt-", dir=base)
        self.root, self.out = os.path.join(self.dir, "root"), os.path.join(self.dir, "out")
        os.makedirs(self.root)
        self.original = b"export const x = 1;"
        open(os.path.join(self.root, "probe.ts"), "wb").write(self.original)
        open(os.path.join(self.dir, "fake.py"), "w").write(FAKE)
        self.env = {**os.environ, "VERDICT_ROOT": self.root, "VERDICT_OUT": self.out, "VERDICT_COMMAND": json.dumps([sys.executable, "-I", os.path.join(self.dir, "fake.py")])}

    def tearDown(self):
        shutil.rmtree(self.dir, ignore_errors=True)

    def verdict(self, recipe):
        path = os.path.join(self.dir, "recipe.json")
        json.dump(recipe, open(path, "w"))
        started = time.monotonic()
        run = subprocess.run([sys.executable, "-I", os.path.join(HERE, "verdict.py"), "t", path], env=self.env, capture_output=True, text=True)
        return run, time.monotonic() - started, json.load(open(os.path.join(self.out, "t.verdict.json")))

    def record(self, suffix):
        return json.load(open(os.path.join(self.out, f"t-{suffix}.json")))

    def test_green_recipe(self):
        run, _, summary = self.verdict({"stages": [
            {"name": "warm", "steps": [{"suffix": "warm", "command": ["run-many", "--dry-run", "refresh"]}]},
            {"name": "first", "parallel": True, "steps": [{"suffix": "a", "target": "a:build"}, {"suffix": "b", "target": "b:test"}]},
            {"name": "second", "steps": [{"suffix": "hot2", "target": "served:build"}, {"suffix": "hot3", "target": "x:build", "unless": "served:hot2"}]},
            {"name": "edits", "steps": [{"suffix": "c", "edit": {"file": "probe.ts", "target": "c:build"}}]},
        ]})
        self.assertEqual(run.returncode, 0, run.stderr)
        self.assertEqual(summary["verdict"], "GREEN")
        self.assertTrue(summary["warm_effective"])
        for suffix in ("warm", "a", "b", "hot2", "c-edit", "c-revert"):
            wall = self.record(suffix)["wall_s"]
            self.assertGreaterEqual(wall, 1.3, suffix)
            self.assertLess(wall, 2.5, suffix + " is not rounded up to a 5 s step")
        first = next(s for s in summary["stages"] if s["name"] == "first")
        self.assertLess(first["wall_s"], 2.5, "a parallel stage overlaps its runs")
        self.assertEqual(self.record("warm")["refreshed"], 3)
        self.assertTrue(self.record("hot2")["served"])
        self.assertFalse(os.path.exists(os.path.join(self.out, "t-hot3.json")), "unless served: skips")
        self.assertIn("probe-present", open(os.path.join(self.out, "t-c-edit.log")).read())
        self.assertIn("probe-absent", open(os.path.join(self.out, "t-c-revert.log")).read())
        self.assertEqual(open(os.path.join(self.root, "probe.ts"), "rb").read(), self.original)
        self.assertIn("door+boot", self.record("a")["phases"])

    def test_red_recipe_stops(self):
        run, _, summary = self.verdict({"stages": [
            {"name": "first", "steps": [{"suffix": "a", "target": "fail:build"}]},
            {"name": "second", "steps": [{"suffix": "b", "target": "b:build"}]},
        ]})
        self.assertEqual(run.returncode, 1)
        self.assertEqual(summary["verdict"], "RED")
        self.assertEqual(summary["aborted"], "first")
        self.assertFalse(os.path.exists(os.path.join(self.out, "t-b.json")))

    def test_edit_never_parallel(self):
        path = os.path.join(self.dir, "recipe.json")
        json.dump({"stages": [{"name": "x", "parallel": True, "steps": [{"suffix": "c", "edit": {"file": "probe.ts", "target": "c"}}]}]}, open(path, "w"))
        run = subprocess.run([sys.executable, "-I", os.path.join(HERE, "verdict.py"), "t", path], env=self.env, capture_output=True, text=True)
        self.assertNotEqual(run.returncode, 0)
        self.assertIn("never runs in a parallel stage", run.stderr + run.stdout)


if __name__ == "__main__":
    unittest.main()
