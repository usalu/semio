"""🧬 Check supervisor code coherence against portable cases and independent Node crypto."""
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest
import weakref
from unittest.mock import patch
from types import SimpleNamespace

TICKET = Path(__file__).resolve().parent
CASES = json.loads((TICKET / "inspection-owner-cases.json").read_text(encoding="utf-8"))
spec = importlib.util.spec_from_file_location("inspection_publisher", TICKET / "inspection-publish.py")
PUBLISHER = importlib.util.module_from_spec(spec)
spec.loader.exec_module(PUBLISHER)

class OwnerCoherence(unittest.TestCase):
    """🔒 Distinguish imported owner changes from ordinary authored body changes."""
    def setUp(self):
        """📂 Create only artificial portable source vectors under generated output."""
        output = TICKET / "🗑️generated" / "io"
        output.mkdir(parents=True, exist_ok=True)
        self.temporary = tempfile.TemporaryDirectory(dir=output)
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.root = self.base / "owner"
        for relative, content in CASES["files"].items():
            path = self.root / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(content, encoding="utf-8")

    def test_fingerprints_match_independent_node_crypto(self):
        """🔬 Compare every Python source digest with the provisioned Node crypto library."""
        actual = PUBLISHER.owner_state(self.root)
        repo = os.environ.get("SEMIO_REPO_EXE") or shutil.which("repo")
        self.assertIsNotNone(repo)
        script = "const fs=require('node:fs'),crypto=require('node:crypto');const files=JSON.parse(process.argv[1]);console.log(JSON.stringify(Object.fromEntries(files.map(p=>[p,crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex')]))));"
        result = subprocess.run([repo, "tools", "exec", "--light", "node", "--", "-e", script, json.dumps(list(actual))],
                                text=True, encoding="utf-8", capture_output=True, check=True)
        self.assertEqual(actual, json.loads(result.stdout))
        self.assertEqual(len(actual), 2)

    def test_changes_additions_removals_require_restart(self):
        """🚫 Reject mismatched loaded code and metadata, including changed closure membership."""
        expected = PUBLISHER.owner_state(self.root)
        PUBLISHER.require_owner_state(expected, self.root)
        target = self.root / CASES["mutation"]["path"]
        original = target.read_bytes()
        target.write_text(CASES["mutation"]["content"], encoding="utf-8")
        with self.assertRaisesRegex(PUBLISHER.RestartRequired, "restart required"):
            PUBLISHER.require_owner_state(expected, self.root)
        target.write_bytes(original)
        addition = self.root / CASES["addition"]["path"]
        addition.write_text(CASES["addition"]["content"], encoding="utf-8")
        with self.assertRaises(PUBLISHER.RestartRequired):
            PUBLISHER.require_owner_state(expected, self.root)
        addition.unlink()
        target.unlink()
        with self.assertRaises(PUBLISHER.RestartRequired):
            PUBLISHER.require_owner_state(expected, self.root)

    def test_unloaded_bodies_and_tests_do_not_force_owner_restart(self):
        """♻️ Keep legitimate body retries while enforcing explicitly captured catalog inputs."""
        expected = PUBLISHER.owner_state(self.root)
        (self.root / CASES["ignore"]).write_text("TEST = 2\n", encoding="utf-8")
        body = self.base / CASES["body_sibling"]["path"]
        body.parent.mkdir()
        body.write_text(CASES["body_sibling"]["content"], encoding="utf-8")
        PUBLISHER.require_owner_state(expected, self.root)
        captured = PUBLISHER.owner_state(self.root, (body,))
        body.write_text("BODY = 2\n", encoding="utf-8")
        with self.assertRaises(PUBLISHER.RestartRequired):
            PUBLISHER.require_owner_state(captured, self.root, (body,))

    def test_stale_python_cannot_issue_certificate_and_owned_ports_cancel(self):
        """🛑 Reject drift before or during provenance calculation and cancel only registered sessions."""
        for during in (False, True):
            with self.subTest(during=during):
                expected = PUBLISHER.owner_state(self.root)
                target = self.root / CASES["mutation"]["path"]
                original = target.read_bytes()
                calls = []
                class Port:
                    """🚦 Model the existing repository tool cancellation contract."""
                    def __init__(self):
                        self.cancelled = 0
                    def cancel(self):
                        self.cancelled += 1
                def state():
                    calls.append("old-python")
                    if during:
                        target.write_text(CASES["mutation"]["content"], encoding="utf-8")
                    return {"certificate": "must not escape"}
                runtime = SimpleNamespace(ToolSession=Port, presentation_state=state)
                with patch.multiple(PUBLISHER, _OWNER_STATE=expected, _OWNER_ROOT=self.root,
                        _OWNER_EXTRA=(), _OWNER_REPORTED=False, OUTPUT=self.base, _OWNED_SESSIONS=weakref.WeakSet()):
                    PUBLISHER.install_owner_guard(runtime)
                    sessions = [runtime.ToolSession() for _ in range(3)]
                    unrelated = Port()
                    if not during:
                        target.write_text(CASES["mutation"]["content"], encoding="utf-8")
                    with self.assertRaises(PUBLISHER.RestartRequired):
                        runtime.presentation_state()
                    self.assertEqual(len(calls), int(during))
                    self.assertEqual([session.cancelled for session in sessions], [1, 1, 1])
                    self.assertEqual(unrelated.cancelled, 0)
                    status = json.loads((self.base / "publish-owner.json").read_text(encoding="utf-8"))
                    self.assertEqual(status["state"], "restart_required")
                target.write_bytes(original)

if __name__ == "__main__":
    unittest.main()
