import ctypes
import importlib
import io
import json
import os
import re
import sys
import tempfile
import time
import contextlib
from ctypes import wintypes
from pathlib import Path
from unittest.mock import patch

S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
vectors = json.loads((S._SOURCES / "🧫️fixtures/🎭️cast-cancellation/🔣️.json").read_text(encoding="utf-8"))
kernel = ctypes.WinDLL("kernel32", use_last_error=True)
kernel.OpenProcess.restype = wintypes.HANDLE
kernel.OpenProcess.argtypes = [wintypes.DWORD, wintypes.BOOL, wintypes.DWORD]
kernel.IsProcessInJob.argtypes = [wintypes.HANDLE, wintypes.HANDLE, ctypes.POINTER(wintypes.BOOL)]
kernel.CloseHandle.argtypes = [wintypes.HANDLE]
trees = []
real = S._Tree.__init__
_assign = S._Tree._assign


def slow_assign(self):
    time.sleep(float(os.environ.get("FK_DELAY", "0")))
    _assign(self)


S._Tree._assign = slow_assign


def init(self, command):
    real(self, command)
    trees.append(self)


def member(tree, pid):
    handle = kernel.OpenProcess(0x1000, False, pid)
    if not handle:
        return "gone"
    flag = wintypes.BOOL()
    kernel.IsProcessInJob(handle, tree.job[1], ctypes.byref(flag))
    kernel.CloseHandle(handle)
    return "in-job" if flag.value else "OUTSIDE-JOB"


def main():
    report = []

    def progress(line):
        found = re.match(vectors["progress"]["pid"], line)
        if found:
            for pid in map(int, found.groups()):
                report.append((pid, member(trees[-1], pid)))
        if re.match(vectors["progress"]["cast"], line):
            raise KeyboardInterrupt

    with tempfile.TemporaryDirectory() as directory:
        root = Path(directory)
        manifest = root / "manifest.json"
        manifest.write_text(json.dumps(vectors["manifest"]), encoding="utf-8")
        with patch.dict(os.environ, {"SEMIO_TEACHING_CACHE_ROOT": str(root / "cache")}), patch.object(S, "_MANIFEST", manifest), patch.object(S._Tree, "__init__", init), contextlib.redirect_stderr(io.StringIO()):
            try:
                S.ensure_casts(vectors["slowIds"], progress)
            except KeyboardInterrupt:
                pass
    print(report, flush=True)


for _ in range(int(sys.argv[1])):
    main()
