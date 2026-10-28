"""🧪 Contract of the stage.py of a batch: applying and reverting keep the mode of every file it replaces, a new file gets 0666 minus the umask (never the 0600 of `mkstemp`), and the root search ends at a filesystem root. usage: python3 -I test_stage_mode.py <stage.template.py> [--expect-red]   (stdlib `unittest`; the oracle is `os.stat` plus the process umask, read independently of the script under test)"""
import hashlib, json, os, shutil, stat, subprocess, sys, tempfile, unittest

TEMPLATE = os.path.abspath(sys.argv[1])
TITLE = "🌌 Staged batch `universe` (lane rustedit): print's Bazel plan becomes reusable although a dependency reads a root-anchored pattern."
def sha(data): return hashlib.sha256(data).hexdigest()
def mask():
    value = os.umask(0); os.umask(value); return value

class StageMode(unittest.TestCase):
    def build(self, directory):
        batch, root = os.path.join(directory, "batch"), os.path.join(directory, "root")
        os.makedirs(os.path.join(batch, "files")); os.makedirs(os.path.join(batch, "base")); os.makedirs(os.path.join(root, "🧰️framework", "d"))
        rows = {}
        for index, (name, mode, base, new) in enumerate([("🧰️framework/d/plain.ts", 0o644, b"old\n", b"new\n"), ("🧰️framework/d/tool.sh", 0o755, b"#!/bin/sh\necho 1\n", b"#!/bin/sh\necho 2\n"), ("🧰️framework/d/fresh.json", None, None, b"{}\n")]):
            key = "%02d" % index
            open(os.path.join(batch, "files", key), "wb").write(new)
            if base is not None:
                open(os.path.join(batch, "base", key), "wb").write(base)
                target = os.path.join(root, name); open(target, "wb").write(base); os.chmod(target, mode)
            rows[name] = {"file": key, "base": None if base is None else sha(base), "new": sha(new)}
        json.dump(rows, open(os.path.join(batch, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False)
        open(os.path.join(batch, "stage.py"), "w", encoding="utf8").write(open(TEMPLATE, encoding="utf8").read())
        return batch, root
    def run_stage(self, batch, root, verb):
        run = subprocess.run([sys.executable, "-I", os.path.join(batch, "stage.py"), verb, "--root", root], capture_output=True, text=True)
        self.assertEqual(run.returncode, 0, run.stdout + run.stderr)
    def mode(self, root, name): return stat.S_IMODE(os.stat(os.path.join(root, name)).st_mode)
    def test_apply_and_revert_keep_modes(self):
        with tempfile.TemporaryDirectory() as directory:
            batch, root = self.build(directory)
            self.run_stage(batch, root, "apply")
            self.assertEqual(self.mode(root, "🧰️framework/d/plain.ts"), 0o644)
            if os.name != "nt": self.assertEqual(self.mode(root, "🧰️framework/d/tool.sh"), 0o755)
            self.assertEqual(self.mode(root, "🧰️framework/d/fresh.json"), 0o666 & ~mask() if os.name != "nt" else self.mode(root, "🧰️framework/d/fresh.json"))
            self.assertEqual(open(os.path.join(root, "🧰️framework/d/plain.ts"), "rb").read(), b"new\n")
            self.run_stage(batch, root, "revert")
            self.assertEqual(self.mode(root, "🧰️framework/d/plain.ts"), 0o644)
            if os.name != "nt": self.assertEqual(self.mode(root, "🧰️framework/d/tool.sh"), 0o755)
            self.assertFalse(os.path.exists(os.path.join(root, "🧰️framework/d/fresh.json")))
    def test_root_search_ends_at_the_filesystem_root(self):
        with tempfile.TemporaryDirectory() as directory:
            batch, _root = self.build(directory)
            run = subprocess.run([sys.executable, "-I", os.path.join(batch, "stage.py"), "check"], capture_output=True, text=True, timeout=20)
            self.assertIn(run.returncode, (0, 1, 2), run.stderr)
if __name__ == "__main__":
    sys.argv = sys.argv[:1] + [a for a in sys.argv[2:] if a != "--expect-red"]
    unittest.main()
