"""🧪 Check bounded status commits against native reader locks and independent Node JSON."""
from pathlib import Path
import copy
import importlib.util
import json
import os
import subprocess
import tempfile
import threading
import time
import unittest
from unittest.mock import patch

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
CASES = json.loads((TICKET / "inspection-record-cases.json").read_text(encoding="utf-8"))
SPEC = importlib.util.spec_from_file_location("record_publisher", TICKET / "inspection-publish.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)

class RecordTest(unittest.TestCase):
    def setUp(self):
        """📁 Isolate all status writes beneath the ticket's generated scope."""
        folder = TICKET / "🗑️generated/ifair"
        folder.mkdir(parents=True, exist_ok=True)
        self.directory = tempfile.TemporaryDirectory(prefix="record-", dir=folder)
        self.addCleanup(self.directory.cleanup)
        self.base = Path(self.directory.name)
        self.path = self.base / "publish.json"
        self.old = json.dumps(CASES["initial"], ensure_ascii=False).encode("utf-8")
        self.path.write_bytes(self.old)
        self.patch_output = patch.object(MODULE, "OUTPUT", self.base)
        self.patch_status = patch.object(MODULE, "_STATUS", copy.deepcopy(CASES["initial"]))
        self.patch_output.start()
        self.patch_status.start()
        self.addCleanup(self.patch_output.stop)
        self.addCleanup(self.patch_status.stop)

    def test_native_reader_lock_releases_and_atomic_commit_succeeds(self):
        if os.name == "nt":
            import ctypes
            from ctypes import wintypes
            kernel = ctypes.WinDLL("kernel32", use_last_error=True)
            kernel.CreateFileW.restype = wintypes.HANDLE
            kernel.CreateFileW.argtypes = [wintypes.LPCWSTR, wintypes.DWORD, wintypes.DWORD, ctypes.c_void_p,
                                          wintypes.DWORD, wintypes.DWORD, wintypes.HANDLE]
            kernel.CloseHandle.argtypes = [wintypes.HANDLE]
            handle = kernel.CreateFileW(str(self.path), 0x80000000, 1, None, 3, 0, None)
            self.assertNotEqual(handle, wintypes.HANDLE(-1).value)
            def release():
                time.sleep(CASES["reader_hold_seconds"])
                kernel.CloseHandle(handle)
            reader = threading.Thread(target=release)
            reader.start()
            try:
                MODULE.record("a", state="running", attempt=1)
            finally:
                reader.join()
        else:
            original, calls = Path.replace, []
            def transient(path, destination):
                calls.append(True)
                if len(calls) <= CASES["transient_failures"]:
                    raise PermissionError("reader lock")
                return original(path, destination)
            with patch.object(Path, "replace", transient):
                MODULE.record("a", state="running", attempt=1)
        self.assertEqual(json.loads(self.path.read_text(encoding="utf-8")),
                         {**CASES["initial"], "a": dict(state="running", attempt=1)})
        self.assertFalse((self.base / "publish.new").exists())

    def test_transient_permission_errors_retry_only_atomic_replace(self):
        original, calls, sleeps = Path.replace, [], []
        def transient(path, destination):
            calls.append(path.read_bytes())
            if len(calls) <= CASES["transient_failures"]:
                self.assertEqual(self.path.read_bytes(), self.old)
                raise PermissionError("reader lock")
            return original(path, destination)
        with patch.object(Path, "replace", transient), patch.object(MODULE.time, "sleep", sleeps.append):
            MODULE.record("a", state="running")
        self.assertEqual(len(calls), CASES["transient_failures"] + 1)
        self.assertTrue(all(data == calls[0] for data in calls))
        self.assertEqual(sleeps, [CASES["delay_seconds"]] * CASES["transient_failures"])

    def test_permanent_permission_error_is_bounded_and_preserves_old_json(self):
        sleeps = []
        with patch.object(Path, "replace", side_effect=PermissionError("permanent")) as replace:
            with patch.object(MODULE.time, "sleep", sleeps.append):
                with self.assertRaises(PermissionError):
                    MODULE.record("a", state="running")
        self.assertEqual(replace.call_count, CASES["attempts"])
        self.assertEqual(sleeps, [CASES["delay_seconds"]] * (CASES["attempts"] - 1))
        self.assertEqual(self.path.read_bytes(), self.old)

    def test_concurrent_updates_match_independent_node_json(self):
        snapshots, errors = [], []
        stop = threading.Event()
        def reader():
            while not stop.is_set():
                try:
                    raw = self.path.read_text(encoding="utf-8")
                    json.loads(raw)
                    snapshots.append(raw)
                except PermissionError:
                    pass
                except Exception as error:
                    errors.append(repr(error))
                stop.wait(0.001)
        watch = threading.Thread(target=reader)
        watch.start()
        def writer(index):
            try:
                for sequence in range(CASES["updates_per_writer"]):
                    MODULE.record("writer" + str(index), state="running", sequence=sequence)
            except Exception as error:
                errors.append(repr(error))
        writers = [threading.Thread(target=writer, args=(index,)) for index in range(CASES["writers"])]
        for writer_thread in writers:
            writer_thread.start()
        for writer_thread in writers:
            writer_thread.join()
        stop.set()
        watch.join()
        self.assertEqual(errors, [])
        self.assertTrue(snapshots)
        payload = dict(snapshots=snapshots, final=self.path.read_text(encoding="utf-8"), cases=CASES)
        oracle_input = self.base / "oracle.json"
        oracle_input.write_text(json.dumps(payload, ensure_ascii=False), encoding="utf-8")
        repo = os.environ.get("SEMIO_REPO_EXE", "C:/Users/Ueli/AppData/Local/semio/bin/repo.exe")
        result = subprocess.run([repo, "tools", "exec", "bun", "--", str(TICKET / "inspection-record-oracle.mjs"),
                                 str(oracle_input)], cwd=ROOT, text=True, encoding="utf-8",
                                capture_output=True, timeout=30)
        self.assertEqual(result.returncode, 0, result.stderr)
        oracle = json.loads(result.stdout.strip().splitlines()[-1])
        self.assertTrue(oracle["valid"])
        self.assertEqual(oracle["final"], json.loads(self.path.read_text(encoding="utf-8")))

    def test_permanent_record_failure_cancels_active_peer(self):
        barrier, release, cancelled = threading.Barrier(2), threading.Event(), []
        def attempt(video):
            barrier.wait(timeout=1)
            if video["id"] == "a":
                MODULE.record("a", state="running")
            self.assertTrue(release.wait(1))
            return dict(state="completed")
        def cancel():
            cancelled.append(True)
            release.set()
        with patch.object(Path, "replace", side_effect=PermissionError("permanent")) as replace:
            with patch.object(MODULE.time, "sleep"):
                with self.assertRaises(PermissionError):
                    MODULE.run_fair([dict(id="a"), dict(id="b")], attempt, lambda *args: False,
                                    lambda: None, cancel, workers=2, poll_seconds=0.003)
        self.assertEqual(replace.call_count, CASES["attempts"])
        self.assertEqual(cancelled, [True])
        self.assertEqual(self.path.read_bytes(), self.old)

if __name__ == "__main__":
    unittest.main()
