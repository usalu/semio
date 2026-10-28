"""🧪 Validate ticket publication fairness against portable scripts and p-limit."""
from pathlib import Path
import ast
from collections import Counter
import contextlib
import contextvars
from concurrent.futures import ThreadPoolExecutor, as_completed
import importlib.util
import io
import json
import os
import subprocess
import sys
import threading
import time
import types
import unittest

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
FIXTURE = json.loads((TICKET / "inspection-fair-scheduler.json").read_text(encoding="utf-8"))
SOURCE = Path(sys.argv[1]) if len(sys.argv) > 1 else TICKET / "inspection-publish.py"
SPEC = importlib.util.spec_from_file_location("fair_publisher", SOURCE)
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)
HAS_FAIR = hasattr(MODULE, "run_fair")

def old_starts():
    """🔴 Exercise the actual prior nested workers without any media or production imports."""
    tree = ast.parse(SOURCE.read_text(encoding="utf-8"))
    publish = next(node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name == "publish")
    functions = [node for node in publish.body if isinstance(node, ast.FunctionDef) and node.name in ("compose", "parallel")]
    stop, lock = threading.Event(), threading.Lock()
    starts = []
    class Stopped(BaseException):
        pass
    class Runtime:
        def compose_videos(self, identities, **kwargs):
            if stop.is_set():
                raise Stopped()
            with lock:
                starts.append(identities[0])
            raise RuntimeError("Authored sources changed")
    fake_time = types.SimpleNamespace(time=time.time, sleep=lambda seconds: stop.wait(0.002))
    scope = dict(runtime=Runtime(), native=set(FIXTURE["initial"]), versions=lambda video: [],
                 ensure_owner_current=lambda: None, record=lambda *args, **kwargs: None, retry_token=lambda identity: 0,
                 RestartRequired=MODULE.RestartRequired, OUTPUT=TICKET, time=fake_time,
                 ThreadPoolExecutor=ThreadPoolExecutor, as_completed=as_completed, contextvars=contextvars)
    exec(compile(ast.Module(body=functions, type_ignores=[]), str(SOURCE), "exec"), scope)
    def run():
        try:
            scope["parallel"]([dict(id=identity) for identity in FIXTURE["initial"]])
        except BaseException:
            pass
    with contextlib.redirect_stdout(io.StringIO()):
        thread = threading.Thread(target=run, daemon=True)
        thread.start()
        time.sleep(0.12)
        stop.set()
        thread.join(2)
    if thread.is_alive():
        raise AssertionError("Old worker control did not stop")
    return starts

class SchedulerTest(unittest.TestCase):
    def test_changing_inputs_release_slots_for_every_initial_video(self):
        if not HAS_FAIR:
            starts = old_starts()
            self.assertEqual(set(starts), set(FIXTURE["initial"]), "Actual nested retry workers starve never-attempted videos")
            return
        repo = os.environ.get("SEMIO_REPO_EXE", "C:/Users/Ueli/AppData/Local/semio/bin/repo.exe")
        oracle = subprocess.run([repo, "tools", "exec", "bun", "--", str(TICKET / "inspection-fair-oracle.mjs"),
                                 str(TICKET / "inspection-fair-scheduler.json")],
                                cwd=ROOT, text=True, encoding="utf-8", capture_output=True, timeout=30)
        self.assertEqual(oracle.returncode, 0, oracle.stderr)
        independent = json.loads(oracle.stdout.strip().splitlines()[-1])
        for workers in FIXTURE["workers"]:
            with self.subTest(workers=workers):
                starts, completed, counts, active = [], set(), Counter(), set()
                lock = threading.Lock()
                peak = 0
                deadline = time.monotonic() + 4
                cancelled = []
                def checkpoint():
                    if time.monotonic() > deadline:
                        raise AssertionError("Fair scheduler did not converge")
                def attempt(video):
                    nonlocal peak
                    identity = video["id"]
                    with lock:
                        self.assertNotIn(identity, active)
                        active.add(identity)
                        peak = max(peak, len(active))
                        starts.append(identity)
                        index = counts[identity]
                        counts[identity] += 1
                    try:
                        time.sleep(FIXTURE["attempt_seconds"])
                        state = FIXTURE["outcomes"][identity][index]
                        if state == "completed":
                            with lock:
                                completed.add(identity)
                        return dict(state=state, output=identity, not_before=time.monotonic() + FIXTURE["retry_seconds"])
                    finally:
                        with lock:
                            active.remove(identity)
                def ready(video, outcome):
                    if outcome["state"] == "retry":
                        return time.monotonic() >= outcome["not_before"]
                    with lock:
                        return set(FIXTURE["release_after"]).issubset(completed)
                result = MODULE.run_fair([dict(id=identity) for identity in FIXTURE["initial"]],
                    attempt, ready, checkpoint, lambda: cancelled.append(True), workers=workers,
                    poll_seconds=FIXTURE["poll_seconds"], ready_seconds=FIXTURE["poll_seconds"])
                self.assertEqual(set(result), set(FIXTURE["initial"]))
                self.assertEqual(dict(counts), FIXTURE["expected_attempts"])
                self.assertEqual(set(starts[:len(FIXTURE["initial"])]), set(FIXTURE["initial"]))
                if workers == 1:
                    self.assertEqual(starts[:len(FIXTURE["initial"])], independent[str(workers)]["initial"])
                self.assertEqual(dict(counts), independent[str(workers)]["counts"])
                self.assertLessEqual(peak, workers)
                self.assertEqual(cancelled, [])

    @unittest.skipUnless(HAS_FAIR, "The old helper has no coordinator")
    def test_owner_failure_cancels_active_workers_before_join(self):
        release, entered = threading.Event(), threading.Event()
        calls, cancelled = [], []
        class OwnerChanged(RuntimeError):
            pass
        def checkpoint():
            calls.append(True)
            if entered.is_set():
                raise OwnerChanged("restart required")
        def attempt(video):
            entered.set()
            self.assertTrue(release.wait(1), "Cancellation failed to release owned work")
            return dict(state="completed")
        def cancel():
            cancelled.append(True)
            release.set()
        with self.assertRaisesRegex(OwnerChanged, "restart required"):
            MODULE.run_fair([dict(id="a"), dict(id="b")], attempt, lambda *args: False, checkpoint, cancel,
                            workers=1, poll_seconds=0.003)
        self.assertEqual(cancelled, [True])

    @unittest.skipUnless(HAS_FAIR, "The old helper has no coordinator")
    def test_waiting_jobs_use_no_worker_and_owner_poll_is_bounded(self):
        attempts, polls, cancelled = [], [], []
        class OwnerChanged(RuntimeError):
            pass
        def attempt(video):
            attempts.append(video["id"])
            return dict(state="waiting")
        def checkpoint():
            polls.append(True)
            if len(polls) == 12:
                raise OwnerChanged("restart required")
        with self.assertRaises(OwnerChanged):
            MODULE.run_fair([dict(id=str(index)) for index in range(7)], attempt, lambda *args: False,
                checkpoint, lambda: cancelled.append(True), workers=2, poll_seconds=0.003, ready_seconds=0.003)
        self.assertEqual(set(attempts), set(map(str, range(7))))
        self.assertEqual(len(attempts), 7)
        self.assertEqual(cancelled, [True])

    @unittest.skipUnless(HAS_FAIR, "The old helper has no coordinator")
    def test_worker_restart_failure_cancels_other_active_job(self):
        release = threading.Event()
        entered = threading.Barrier(2)
        cancelled = []
        def attempt(video):
            entered.wait(timeout=1)
            if video["id"] == "a":
                raise MODULE.RestartRequired("restart required")
            self.assertTrue(release.wait(1))
            return dict(state="completed")
        def cancel():
            cancelled.append(True)
            release.set()
        with self.assertRaises(MODULE.RestartRequired):
            MODULE.run_fair([dict(id="a"), dict(id="b")], attempt, lambda *args: False,
                            lambda: None, cancel, workers=2, poll_seconds=0.003)
        self.assertEqual(cancelled, [True])

    @unittest.skipUnless(HAS_FAIR, "The old helper has no coordinator")
    def test_owner_guard_returns_original_product_state_unchanged(self):
        marker = dict(sources={"body.py": "abc"}, catalog={"courses": ["architecture"]})
        runtime = types.SimpleNamespace(ToolSession=type("Session", (), {}),
                                        presentation_state=lambda *args, **kwargs: marker)
        MODULE.install_owner_guard(runtime)
        self.assertIs(runtime.presentation_state("clip", {}, "de"), marker)

    @unittest.skipUnless(HAS_FAIR, "The old helper has no coordinator")
    def test_duplicate_identity_is_rejected_before_submission(self):
        with self.assertRaisesRegex(ValueError, "Duplicate"):
            MODULE.run_fair([dict(id="a"), dict(id="a")], lambda *args: None,
                            lambda *args: False, lambda: None, lambda: None)

if __name__ == "__main__":
    suite = unittest.defaultTestLoader.loadTestsFromTestCase(SchedulerTest)
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    raise SystemExit(not result.wasSuccessful())
