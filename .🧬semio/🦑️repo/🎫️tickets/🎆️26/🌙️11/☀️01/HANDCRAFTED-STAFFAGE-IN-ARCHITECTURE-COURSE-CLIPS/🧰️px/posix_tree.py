"""POSIX check of the staffage `_Tree` and `_run` without manim: extracts both from the module source and drives them with shell stand-ins.
Run inside Linux or macOS: `python3 posix_tree.py <path to staffage 🐍️.py>`."""
from __future__ import annotations

import ast
import os
import signal
import subprocess
import sys
import tempfile
import threading
import time
import unittest
from pathlib import Path
from typing import Callable

SOURCE = Path(sys.argv.pop(1)).read_text(encoding="utf-8")
NAMESPACE = {"os": os, "sys": sys, "signal": signal, "subprocess": subprocess, "threading": threading, "time": time, "Callable": Callable, "_WORKSPACE": Path(tempfile.gettempdir())}
for node in ast.parse(SOURCE).body:
    if isinstance(node, ast.ClassDef) and node.name == "_Tree" or isinstance(node, ast.FunctionDef) and node.name == "_run":
        exec(compile(ast.Module([node], []), "staffage", "exec"), NAMESPACE)
Tree, run = NAMESPACE["_Tree"], NAMESPACE["_run"]


def alive(pid: int) -> bool:
    try:
        os.kill(pid, 0)
    except ProcessLookupError:
        return False
    try:
        return Path(f"/proc/{pid}/stat").read_text().split()[2] != "Z" if Path("/proc").exists() else True
    except OSError:
        return False


def pids(path: Path, count: int, timeout: float = 10.0) -> list[int]:
    end = time.time() + timeout
    while time.time() < end:
        found = [int(line) for line in path.read_text().split() if line] if path.exists() else []
        if len(found) >= count:
            return found
        time.sleep(0.05)
    raise AssertionError(f"expected {count} pids in {path}")


def gone(listed: list[int], timeout: float = 10.0) -> bool:
    end = time.time() + timeout
    while time.time() < end:
        if not any(alive(pid) for pid in listed):
            return True
        time.sleep(0.05)
    return False


class PosixTree(unittest.TestCase):
    def setUp(self) -> None:
        self.directory = Path(tempfile.mkdtemp())
        self.record = self.directory / "pids"

    def script(self, body: str) -> list[str]:
        return ["sh", "-c", body.replace("RECORD", str(self.record))]

    def test_stop_signals_the_whole_session_group(self):
        tree = Tree(self.script('echo $$ >> RECORD; sleep 300 & echo $! >> RECORD; sleep 300 & echo $! >> RECORD; wait'))
        listed = pids(self.record, 3)
        self.assertEqual(os.getpgid(tree.process.pid), tree.process.pid)
        self.assertNotEqual(os.getsid(tree.process.pid), os.getsid(os.getpid()))
        started = time.time()
        tree.stop()
        self.assertLess(time.time() - started, 2.5)
        self.assertTrue(gone(listed))
        self.assertEqual(tree.process.returncode, -signal.SIGTERM)

    def test_stop_escalates_to_sigkill_when_sigterm_is_ignored(self):
        tree = Tree(self.script('trap "" TERM; echo $$ >> RECORD; (trap "" TERM; sleep 300) & echo $! >> RECORD; while true; do sleep 1; done'))
        listed = pids(self.record, 2)
        started = time.time()
        tree.stop(grace=0.5)
        self.assertLess(time.time() - started, 5)
        self.assertTrue(gone(listed))
        self.assertEqual(tree.process.returncode, -signal.SIGKILL)

    def test_stop_after_the_leader_is_gone_still_kills_the_group(self):
        tree = Tree(self.script('(sleep 300 & echo $! >> RECORD; echo $$ >> RECORD; wait) & echo $! >> RECORD'))
        listed = pids(self.record, 3)
        tree.process.wait(timeout=5)
        self.assertTrue(any(alive(pid) for pid in listed))
        tree.kill()
        self.assertTrue(gone(listed))

    def test_run_cancels_the_tree_when_the_progress_callback_raises(self):
        seen = []

        def progress(line: str) -> None:
            seen.append(line)
            if line.startswith("[cast 1/3]"):
                raise KeyboardInterrupt

        command = self.script('echo $$ >> RECORD; sleep 300 & echo $! >> RECORD; echo "[cast 1/3] a" >&2; wait')
        started = time.time()
        with self.assertRaises(KeyboardInterrupt):
            run(command, progress)
        self.assertLess(time.time() - started, 6)
        self.assertTrue(gone(pids(self.record, 2)))
        self.assertEqual(seen, ["[cast 1/3] a"])

    def test_run_reports_failure_with_the_stderr_tail(self):
        with self.assertRaisesRegex(RuntimeError, "boom"):
            run(self.script('echo boom >&2; exit 3'))

    def test_run_survives_ctrl_c_delivered_to_the_main_thread(self):
        command = self.script('echo $$ >> RECORD; sleep 300 & echo $! >> RECORD; echo "[cast 1/3] a" >&2; wait')
        timer = threading.Timer(0.7, lambda: os.kill(os.getpid(), signal.SIGINT))
        timer.start()
        with self.assertRaises(KeyboardInterrupt):
            run(command, None)
        timer.cancel()
        self.assertTrue(gone(pids(self.record, 2)))

    def test_hard_killed_python_leaves_the_group_running_without_a_watchdog(self):
        launcher = self.directory / "launcher.py"
        launcher.write_text(
            "import os, subprocess, sys, time\n"
            "p = subprocess.Popen(['sh', '-c', 'echo $$ >> %s; sleep 300 & echo $! >> %s; wait'], start_new_session=True)\n"
            "time.sleep(300)\n" % (self.record, self.record),
            encoding="utf-8",
        )
        parent = subprocess.Popen([sys.executable, str(launcher)])
        listed = pids(self.record, 2)
        parent.kill()
        parent.wait()
        time.sleep(0.5)
        survivors = [pid for pid in listed if alive(pid)]
        for pid in survivors:
            os.kill(pid, signal.SIGKILL)
        self.assertEqual(len(survivors), 2, "documents why the CLI --parent watchdog is needed on POSIX")


if __name__ == "__main__":
    unittest.main(verbosity=2)
