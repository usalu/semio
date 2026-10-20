import hashlib
import os
import shutil
import subprocess
import sys
import tempfile
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
SCRATCH = sys.argv.pop(1) if len(sys.argv) > 1 and not sys.argv[1].startswith("-") else tempfile.gettempdir()
SOURCE = {"__file__": os.path.join(HERE, "apply.py")}
exec(compile(open(os.path.join(HERE, "apply.py"), encoding="utf-8").read().replace("sys.exit(main())", ""), "apply", "exec"), SOURCE)
ROOT = SOURCE["DEFAULT_ROOT"]
TARGETS = [relative for relative, _ in SOURCE["edits"]()]
NEW = list(SOURCE["NEW"].values())


def apply(root, flag):
    return subprocess.run([sys.executable, os.path.join(HERE, "apply.py"), flag, "--root", root], capture_output=True, text=True)


def digest(path):
    tool = subprocess.run(["shasum", "-a", "256", path], capture_output=True, text=True).stdout.split()[0]
    with open(path, "rb") as handle:
        assert tool == hashlib.sha256(handle.read()).hexdigest()
    return tool


class TwoPhaseApply(unittest.TestCase):
    def setUp(self):
        self.root = tempfile.mkdtemp(prefix="graph-apply-", dir=SCRATCH)
        for relative in TARGETS:
            target = os.path.join(self.root, relative)
            os.makedirs(os.path.dirname(target), exist_ok=True)
            shutil.copyfile(os.path.join(ROOT, relative), target)
        self.before = {relative: digest(os.path.join(self.root, relative)) for relative in TARGETS}

    def tearDown(self):
        shutil.rmtree(self.root, ignore_errors=True)

    def test_check_reports_pending_and_writes_nothing(self):
        result = apply(self.root, "--check")
        self.assertEqual(result.returncode, 1, result.stdout)
        self.assertEqual(self.before, {relative: digest(os.path.join(self.root, relative)) for relative in TARGETS})
        for relative in NEW:
            self.assertFalse(os.path.exists(os.path.join(self.root, relative)))

    def test_live_applies_everything_and_is_idempotent(self):
        self.assertEqual(apply(self.root, "--live").returncode, 0)
        self.assertEqual(apply(self.root, "--check").returncode, 0)
        after = {relative: digest(os.path.join(self.root, relative)) for relative in TARGETS}
        self.assertTrue(all(after[relative] != self.before[relative] for relative in TARGETS))
        self.assertEqual(apply(self.root, "--live").returncode, 0)
        self.assertEqual(after, {relative: digest(os.path.join(self.root, relative)) for relative in TARGETS})
        for relative in NEW:
            self.assertTrue(os.path.isfile(os.path.join(self.root, relative)), relative)

    def test_one_missing_anchor_refuses_before_any_write(self):
        victim = TARGETS[-1]
        path = os.path.join(self.root, victim)
        with open(path, encoding="utf-8") as handle:
            text = handle.read()
        anchor = "const publishAdmission" if False else "return closeBuildOwnerAdmissionV1(options.selection,registry!,async(owner,targets,inputs)=>"
        self.assertEqual(text.count(anchor), 1)
        with open(path, "w", encoding="utf-8") as handle:
            handle.write(text.replace(anchor, "return closeBuildOwnerAdmissionV1(options.selection,registry!,async(peerOwner,targets,inputs)=>"))
        tampered = {relative: digest(os.path.join(self.root, relative)) for relative in TARGETS}
        result = apply(self.root, "--live")
        self.assertEqual(result.returncode, 2, result.stdout)
        self.assertIn("REFUSED, nothing written", result.stdout)
        self.assertEqual(tampered, {relative: digest(os.path.join(self.root, relative)) for relative in TARGETS})
        for relative in NEW:
            self.assertFalse(os.path.exists(os.path.join(self.root, relative)))

    def test_a_duplicated_anchor_refuses(self):
        path = os.path.join(self.root, TARGETS[0])
        with open(path, encoding="utf-8") as handle:
            text = handle.read()
        with open(path, "w", encoding="utf-8") as handle:
            handle.write(text + "\n" + text[text.index(" const admit="):text.index(" const admit=") + 200])
        result = apply(self.root, "--check")
        self.assertEqual(result.returncode, 2, result.stdout)

    def test_a_partially_applied_tree_is_completed(self):
        self.assertEqual(apply(self.root, "--live").returncode, 0)
        victim = os.path.join(self.root, TARGETS[1])
        shutil.copyfile(os.path.join(ROOT, TARGETS[1]), victim)
        self.assertEqual(apply(self.root, "--check").returncode, 1)
        self.assertEqual(apply(self.root, "--live").returncode, 0)
        self.assertEqual(apply(self.root, "--check").returncode, 0)


unittest.main(argv=[sys.argv[0]])
