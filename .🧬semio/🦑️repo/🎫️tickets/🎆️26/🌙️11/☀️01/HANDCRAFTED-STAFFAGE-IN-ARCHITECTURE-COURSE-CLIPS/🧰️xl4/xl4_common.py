"""Shared helpers of the xl4 scripts: working-set sampling with a kill guard, a fixed ordering-cache digest and the tower builders."""
import ctypes
import ctypes.wintypes as wt
import importlib
import os
import sys
import threading
import time

sys.path.insert(0, os.getcwd())
GUARD = float(os.environ.get("XL4_GUARD_GB", "6")) * 1024**3
START = time.time()
STATE = {"where": "import", "peak": 0}


class Counters(ctypes.Structure):
    _fields_ = [("cb", wt.DWORD), ("PageFaultCount", wt.DWORD), ("PeakWorkingSetSize", ctypes.c_size_t), ("WorkingSetSize", ctypes.c_size_t),
                ("QuotaPeakPagedPoolUsage", ctypes.c_size_t), ("QuotaPagedPoolUsage", ctypes.c_size_t), ("QuotaPeakNonPagedPoolUsage", ctypes.c_size_t),
                ("QuotaNonPagedPoolUsage", ctypes.c_size_t), ("PagefileUsage", ctypes.c_size_t), ("PeakPagefileUsage", ctypes.c_size_t)]


def counters():
    value = Counters()
    value.cb = ctypes.sizeof(value)
    ctypes.windll.kernel32.GetCurrentProcess.restype = ctypes.c_void_p
    ctypes.windll.psapi.GetProcessMemoryInfo(ctypes.c_void_p(ctypes.windll.kernel32.GetCurrentProcess()), ctypes.byref(value), value.cb)
    return value


def rss():
    return counters().WorkingSetSize


def peak():
    return counters().PeakWorkingSetSize


def say(text):
    print(f"[xl4] t={time.time() - START:7.1f}s rss={rss() / 1024**2:7.0f}MB peak={peak() / 1024**2:7.0f}MB {text}", flush=True)


def watch(period=15):
    def run():
        while True:
            value = rss()
            if value > GUARD:
                say(f"GUARD EXCEEDED at {STATE['where']}, killing")
                os._exit(99)
            if int(time.time() - START) % 60 < period:
                say(f"where={STATE['where']}")
            time.sleep(period)
    threading.Thread(target=run, daemon=True).start()


def load():
    visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
    visuals._source_digest = lambda: "xl4-fixed"
    visuals._FROZEN_SHAPES = int(os.environ.get("XL4_FROZEN", visuals._FROZEN_SHAPES))
    return visuals


def phases(visuals):
    for name in ("_tower_items", "_section_order", "_paint_ordered", "_compose", "_tower_template", "tower_axon", "tower_capsules"):
        owner = visuals.NakaginDrawing if name in ("_tower_items", "_compose", "_tower_template", "tower_axon", "tower_capsules") else visuals
        original = getattr(owner, name)

        def wrapped(*args, __o=original, __n=name, **kwargs):
            before = STATE["where"]
            STATE["where"] = __n
            say(f"begin {__n}")
            try:
                return __o(*args, **kwargs)
            finally:
                say(f"end {__n}")
                STATE["where"] = before
        setattr(owner, name, wrapped)
    for name in ("thaw", "capsule"):
        original = getattr(visuals._Frozen, name)

        def wrapped_frozen(*args, __o=original, __n=name, **kwargs):
            say(f"begin frozen.{__n}")
            try:
                return __o(*args, **kwargs)
            finally:
                say(f"end frozen.{__n}")
        setattr(visuals._Frozen, name, wrapped_frozen)


def program_fit():
    excel = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🎨️visuals.🐍️")
    import numpy as np
    return excel.building_fit(5.4, 4.4, centre=np.array([4.3, 0.0, 0.0]))
