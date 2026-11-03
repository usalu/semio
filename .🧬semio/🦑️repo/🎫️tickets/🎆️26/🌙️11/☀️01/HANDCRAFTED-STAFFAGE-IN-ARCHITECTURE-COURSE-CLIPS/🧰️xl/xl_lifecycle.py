"""Run test_native_lifecycle for one (folder, language) with RSS sampling, kill above 6 GB. Usage: xl_lifecycle.py FOLDER LANG [only-scene]"""
import ctypes
import ctypes.wintypes as wt
import os
import sys
import threading
import time
import unittest

folder, language = sys.argv[1], sys.argv[2]
LIMIT = 6 * 1024**3


class Counters(ctypes.Structure):
    _fields_ = [("cb", wt.DWORD), ("PageFaultCount", wt.DWORD), ("PeakWorkingSetSize", ctypes.c_size_t), ("WorkingSetSize", ctypes.c_size_t),
                ("QuotaPeakPagedPoolUsage", ctypes.c_size_t), ("QuotaPagedPoolUsage", ctypes.c_size_t), ("QuotaPeakNonPagedPoolUsage", ctypes.c_size_t),
                ("QuotaNonPagedPoolUsage", ctypes.c_size_t), ("PagefileUsage", ctypes.c_size_t), ("PeakPagefileUsage", ctypes.c_size_t)]


def rss():
    counters = Counters()
    counters.cb = ctypes.sizeof(counters)
    ctypes.windll.kernel32.GetCurrentProcess.restype = ctypes.c_void_p
    ctypes.windll.psapi.GetProcessMemoryInfo(ctypes.c_void_p(ctypes.windll.kernel32.GetCurrentProcess()), ctypes.byref(counters), counters.cb)
    return counters.WorkingSetSize


start = time.time()
state = {"where": "import"}


def sample():
    while True:
        value = rss()
        print(f"[xl] t={time.time() - start:7.1f}s rss={value / 1024**2:8.0f}MB where={state['where']}", flush=True)
        if value > LIMIT:
            print("[xl] LIMIT EXCEEDED, killing", flush=True)
            os._exit(99)
        time.sleep(10)


import faulthandler
faulthandler.dump_traceback_later(900, repeat=True, file=sys.stdout)
threading.Thread(target=sample, daemon=True).start()
sys.argv = [sys.argv[0]]
module = __import__("importlib").import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🧪️tests.🎭️presentation.🐍️")
original = module.discover
module.discover = lambda: [entry for entry in original() if entry["folder"] == folder]
module.CASES["languages"] = [language]
base_scene_class = module.scene_class


def traced(entry):
    cls = base_scene_class(entry)
    state["where"] = f"{entry['scene']}:{language}"
    return cls


module.scene_class = traced
suite = unittest.TestSuite([module.SceneContract("test_native_lifecycle")])
nakagin = __import__("importlib").import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
drawing_class = nakagin.NakaginDrawing
builds = []
original_drawing = drawing_class._tower_drawing


def timed_drawing(self, **kwargs):
    began = time.time()
    before = rss()
    result = original_drawing(self, **kwargs)
    builds.append(time.time() - began)
    print(f"[xl] BUILD #{len(builds)} {builds[-1]:.1f}s rss {before / 1024**2:.0f}->{rss() / 1024**2:.0f}MB tint={kwargs.get('tint') is not None} lit={kwargs.get('lit') is not None} cache={len(nakagin._TOWERS)}", flush=True)
    return result


drawing_class._tower_drawing = timed_drawing
visible_calls = []
original_visible = nakagin._opaque_visible


def timed_visible(items, view, padding=0.0):
    began = time.time()
    result = original_visible(items, view, padding)
    visible_calls.append((len(items), len(result), time.time() - began))
    if len(visible_calls) % 5 == 1:
        print(f"[xl] VISIBLE call#{len(visible_calls)} items={len(items)} kept={len(result)} {visible_calls[-1][2]:.1f}s total={sum(c[2] for c in visible_calls):.0f}s cache={len(nakagin._OPAQUE_VISIBILITY)}", flush=True)
    return result


nakagin._opaque_visible = timed_visible
print(f"[xl] pid={os.getpid()}", flush=True)
import collections

hits, chains = collections.Counter(), collections.Counter()
main = threading.main_thread().ident


def stacks():
    last = time.time()
    while True:
        frame = sys._current_frames().get(main)
        names = []
        while frame:
            names.append(f"{os.path.basename(frame.f_code.co_filename)[-12:]}:{frame.f_code.co_name}:{frame.f_lineno}")
            frame = frame.f_back
        if names:
            hits[names[0]] += 1
            chains[" < ".join(names[:4])] += 1
        if time.time() - last > 180:
            last = time.time()
            print("[xl] TOP leaf", hits.most_common(8), flush=True)
            print("[xl] TOP chain", chains.most_common(4), flush=True)
        time.sleep(0.25)


threading.Thread(target=stacks, daemon=True).start()
result = unittest.TextTestRunner(verbosity=2).run(suite)
print(f"[xl] done t={time.time() - start:.1f}s peak={rss() / 1024**2:.0f}MB ok={result.wasSuccessful()} run={result.testsRun} fail={len(result.failures)} err={len(result.errors)}", flush=True)
