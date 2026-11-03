"""RSS accounting of one small tower variant at fit scale. Usage: xl3_rss.py [storeys]"""
import ctypes
import ctypes.wintypes as wt
import gc
import importlib
import os
import sys
import time

sys.path.insert(0, os.getcwd())


class Counters(ctypes.Structure):
    _fields_ = [("cb", wt.DWORD), ("PageFaultCount", wt.DWORD), ("PeakWorkingSetSize", ctypes.c_size_t), ("WorkingSetSize", ctypes.c_size_t),
                ("QuotaPeakPagedPoolUsage", ctypes.c_size_t), ("QuotaPagedPoolUsage", ctypes.c_size_t), ("QuotaPeakNonPagedPoolUsage", ctypes.c_size_t),
                ("QuotaNonPagedPoolUsage", ctypes.c_size_t), ("PagefileUsage", ctypes.c_size_t), ("PeakPagefileUsage", ctypes.c_size_t)]


def rss():
    counters = Counters()
    counters.cb = ctypes.sizeof(counters)
    ctypes.windll.kernel32.GetCurrentProcess.restype = ctypes.c_void_p
    ctypes.windll.psapi.GetProcessMemoryInfo(ctypes.c_void_p(ctypes.windll.kernel32.GetCurrentProcess()), ctypes.byref(counters), counters.cb)
    return counters.WorkingSetSize / 1024**2


V = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
fit = importlib.import_module("🎓️teaching.🏛️architecture.🧲️fit.🐍️").building_fit(5.4, 4.4, centre=(0, 0, 0))
storeys = int(sys.argv[1]) if len(sys.argv) > 1 else 4
draw = V.NakaginDrawing()
print(f"[rss] start {rss():.0f}MB", flush=True)
began = time.time()
drawing = draw.tower_axon(at=fit["at"], scale=fit["scale"], towers=(0,), where=lambda s: s.tower == 0 and s.storey < storeys, core=False, podium=False, capital=False)
shapes = [m for m in drawing.get_family() if len(m.points)]
print(f"[rss] built shapes={len(shapes)} t={time.time() - began:.0f}s rss={rss():.0f}MB", flush=True)
V._OPAQUE_VISIBILITY.clear()
V._OCCLUSION_ORDERS.clear()
gc.collect()
print(f"[rss] caches cleared rss={rss():.0f}MB", flush=True)
template = next(iter(V._TOWERS.values()))
V._TOWERS.clear()
del template
gc.collect()
print(f"[rss] template dropped rss={rss():.0f}MB (clone only)", flush=True)
attrs = {}
for shape in shapes[:2000]:
    for name, value in shape.__dict__.items():
        size = getattr(value, "nbytes", None)
        if size is None:
            size = sys.getsizeof(value)
            if isinstance(value, (tuple, list)):
                size += sum(getattr(v, "nbytes", sys.getsizeof(v)) for v in value)
        attrs[name] = attrs.get(name, 0) + size
print({k: round(v / 2000) for k, v in sorted(attrs.items(), key=lambda kv: -kv[1])[:10]}, flush=True)
