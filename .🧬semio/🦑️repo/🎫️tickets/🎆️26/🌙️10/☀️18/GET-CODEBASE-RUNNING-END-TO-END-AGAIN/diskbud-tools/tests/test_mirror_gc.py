"""🧪 Contract of mirror-gc.py: the pure rule drops only old, idle mirrors of the named lane (independent table of expectations), `--apply` without a prefix is refused and changes nothing. usage: python3 -I test_mirror_gc.py"""
import importlib.util, json, os, subprocess, sys, tempfile, unittest
HERE = os.path.dirname(os.path.abspath(__file__)); TOOL = os.path.join(os.path.dirname(HERE), "mirror-gc.py")
spec = importlib.util.spec_from_file_location("mirror_gc", TOOL); module = importlib.util.module_from_spec(spec); spec.loader.exec_module(module)
NOW = 1_800_000_000.0
class MirrorGc(unittest.TestCase):
    def test_rule_table(self):
        mirrors = [{"name": "db-old", "created": NOW - 9 * 3600, "busy": False}, {"name": "db-new", "created": NOW - 3600, "busy": False}, {"name": "db-busy", "created": NOW - 9 * 3600, "busy": True}, {"name": "zz-old", "created": NOW - 9 * 3600, "busy": False}]
        got = {row["name"]: row["droppable"] for row in module.plan(mirrors, NOW, 6, ["db-"])}
        self.assertEqual(got, {"db-old": True, "db-new": False, "db-busy": False, "zz-old": False})
        self.assertTrue(all(row["droppable"] is not False or row["reasons"] for row in module.plan(mirrors, NOW, 6, [])))
        self.assertEqual({row["name"] for row in module.plan(mirrors, NOW, 6, []) if row["droppable"]}, {"db-old", "zz-old"})
    def test_apply_needs_a_prefix_and_report_changes_nothing(self):
        with tempfile.TemporaryDirectory() as home:
            os.makedirs(os.path.join(home, ".meta")); os.makedirs(os.path.join(home, "db-old"))
            json.dump({"name": "db-old", "created": 1.0, "files": []}, open(os.path.join(home, ".meta", "db-old.json"), "w"))
            refused = subprocess.run([sys.executable, "-I", TOOL, "--home", home, "--apply"], capture_output=True, text=True)
            self.assertNotEqual(refused.returncode, 0); self.assertIn("--prefix", refused.stderr)
            report = subprocess.run([sys.executable, "-I", TOOL, "--home", home, "--prefix", "db-"], capture_output=True, text=True)
            self.assertEqual(report.returncode, 0, report.stderr)
            row = json.loads(report.stdout.splitlines()[0]); self.assertEqual((row["name"], row["droppable"]), ("db-old", True))
            self.assertTrue(os.path.isdir(os.path.join(home, "db-old")))
if __name__ == "__main__": unittest.main(argv=[sys.argv[0]])
