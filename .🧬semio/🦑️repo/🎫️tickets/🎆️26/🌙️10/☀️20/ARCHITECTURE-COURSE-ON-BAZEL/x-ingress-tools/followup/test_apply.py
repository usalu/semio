import hashlib
import os
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

HERE = os.path.dirname(os.path.abspath(__file__))
SCRATCH = sys.argv.pop(1) if len(sys.argv) > 1 and not sys.argv[1].startswith("-") else tempfile.gettempdir()
SOURCE = {"__file__": os.path.join(HERE, "apply.py")}
exec(compile(open(os.path.join(HERE, "apply.py"), encoding="utf-8").read().replace("sys.exit(main())", ""), "apply", "exec"), SOURCE)
ROOT = SOURCE["DEFAULT_ROOT"]
TARGETS = [relative for relative, _ in SOURCE["edits"]()]
BASE = SOURCE["BASE"]


def apply(root, flag):
    return subprocess.run([sys.executable, os.path.join(HERE, "apply.py"), flag, "--root", root], capture_output=True, text=True)


def digest(path):
    tool = subprocess.run(["shasum", "-a", "256", path], capture_output=True, text=True).stdout.split()[0]
    with open(path, "rb") as handle:
        assert tool == hashlib.sha256(handle.read()).hexdigest()
    return tool


class TwoPhaseApply(unittest.TestCase):
    def setUp(self):
        self.root = tempfile.mkdtemp(prefix="followup-apply-", dir=SCRATCH)
        for relative in TARGETS:
            target = os.path.join(self.root, relative)
            os.makedirs(os.path.dirname(target), exist_ok=True)
            shutil.copyfile(os.path.join(ROOT, relative), target)
        self.before = {relative: digest(os.path.join(self.root, relative)) for relative in TARGETS}

    def tearDown(self):
        shutil.rmtree(self.root, ignore_errors=True)

    def hashes(self):
        return {relative: digest(os.path.join(self.root, relative)) for relative in TARGETS}

    def test_check_is_read_only_and_reports_pending(self):
        done = apply(self.root, "--check")
        self.assertIn(done.returncode, (0, 1))
        self.assertEqual(self.hashes(), self.before)
        self.assertFalse(os.path.exists(os.path.join(self.root, SOURCE["TESTS"])))

    def test_live_applies_everything_then_is_idempotent(self):
        first = apply(self.root, "--live")
        self.assertEqual(first.returncode, 0, first.stdout + first.stderr)
        self.assertEqual(apply(self.root, "--check").returncode, 0)
        after = self.hashes()
        second = apply(self.root, "--live")
        self.assertEqual(second.returncode, 0)
        self.assertEqual(self.hashes(), after)
        self.assertNotEqual(after, self.before)
        for relative in SOURCE["NEW"].values():
            self.assertTrue(os.path.isfile(os.path.join(self.root, relative)))

    def test_one_missing_anchor_refuses_the_whole_set_and_writes_nothing(self):
        boot = os.path.join(self.root, SOURCE["BOOT"])
        text = Path(boot).read_text(encoding="utf-8")
        anchor = "walk!,walkStartedAt),walk:()=>currentOwner()"
        self.assertEqual(text.count(anchor) + text.count("walk!,walkOriginAt),walk:()=>currentOwner()"), 1)
        if anchor in text:
            Path(boot).write_text(text.replace(anchor, "walk!,somethingElse),walk:()=>currentOwner()"))
        else:
            Path(boot).write_text(text.replace("walk!,walkOriginAt),walk:()=>currentOwner()", "walk!,somethingElse),walk:()=>currentOwner()").replace("generation.generationWalkOriginV1(cached?.startedAt,walkStartedAt)", "x"))
        snapshot = self.hashes()
        for flag in ("--live", "--check"):
            done = apply(self.root, flag)
            self.assertEqual(done.returncode, 2, done.stdout)
            self.assertIn("REFUSED, nothing written", done.stdout)
        self.assertEqual(self.hashes(), snapshot)
        self.assertFalse(os.path.exists(os.path.join(self.root, SOURCE["TESTS"])))

    def test_an_ambiguous_anchor_refuses(self):
        glue = os.path.join(self.root, SOURCE["GLUE"])
        text = Path(glue).read_text(encoding="utf-8")
        marker = "/** 🤝️ Publishes one receipt under the caller's SHARED"
        self.assertEqual(text.count(marker), 1)
        Path(glue).write_text(text + "\n" + marker + "\n")
        snapshot = self.hashes()
        done = apply(self.root, "--live")
        self.assertEqual(done.returncode, 2)
        self.assertEqual(self.hashes(), snapshot)

    def test_missing_base_set_refuses(self):
        core = os.path.join(self.root, SOURCE["CORE"])
        text = Path(core).read_text(encoding="utf-8")
        Path(core).write_text(text.replace("export async function confirmGenerationWalkV1", "export async function somethingElseV1"))
        snapshot = self.hashes()
        done = apply(self.root, "--live")
        self.assertEqual(done.returncode, 2)
        self.assertIn("base set is not applied", done.stdout)
        self.assertEqual(self.hashes(), snapshot)

    def test_a_missing_target_refuses(self):
        os.remove(os.path.join(self.root, SOURCE["SCHEMA"]))
        snapshot = {relative: digest(os.path.join(self.root, relative)) for relative in TARGETS if relative != SOURCE["SCHEMA"]}
        done = apply(self.root, "--live")
        self.assertEqual(done.returncode, 2)
        self.assertEqual({relative: digest(os.path.join(self.root, relative)) for relative in snapshot}, snapshot)


unittest.main(argv=[sys.argv[0]], verbosity=2)
