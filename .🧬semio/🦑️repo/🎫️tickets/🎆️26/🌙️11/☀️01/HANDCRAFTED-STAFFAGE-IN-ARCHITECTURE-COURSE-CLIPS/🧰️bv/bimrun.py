import atexit, ctypes, faulthandler, signal, sys, threading, time, unittest, importlib.util
from pathlib import Path

OUT = Path(sys.argv[1])
CLASSES = [a for a in sys.argv[2:] if not a.startswith("--")]
OUT.mkdir(parents=True, exist_ok=True)
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace", line_buffering=True)
sys.stderr.reconfigure(encoding="utf-8", errors="backslashreplace", line_buffering=True)
fault = open(OUT / "fault.log", "a", buffering=1)
faulthandler.enable(fault, all_threads=True)
life = open(OUT / "life.log", "a", encoding="utf-8", buffering=1)


def note(text):
    life.write(f"{time.strftime('%H:%M:%S')} {text}\n")


def on_signal(number, frame):
    note(f"signal {number}")
    sys.exit(1)


for name in ("SIGINT", "SIGTERM", "SIGBREAK"):
    if hasattr(signal, name):
        signal.signal(getattr(signal, name), on_signal)
atexit.register(lambda: note("atexit"))
note("start")


def beat():
    class Counters(ctypes.Structure):
        _fields_ = [("cb", ctypes.c_ulong), ("pf", ctypes.c_ulong), ("peak", ctypes.c_size_t), ("ws", ctypes.c_size_t),
                    ("a", ctypes.c_size_t), ("b", ctypes.c_size_t), ("c", ctypes.c_size_t), ("d", ctypes.c_size_t),
                    ("pagefile", ctypes.c_size_t), ("peakpagefile", ctypes.c_size_t)]
    k32 = ctypes.windll.kernel32
    psapi = ctypes.windll.psapi
    k32.GetCurrentProcess.restype = ctypes.c_void_p
    psapi.GetProcessMemoryInfo.argtypes = [ctypes.c_void_p, ctypes.POINTER(Counters), ctypes.c_ulong]
    while True:
        c = Counters()
        c.cb = ctypes.sizeof(c)
        psapi.GetProcessMemoryInfo(k32.GetCurrentProcess(), ctypes.byref(c), c.cb)
        note(f"rss {c.ws / 2**20:.0f} MiB peak {c.peak / 2**20:.0f} MiB commit {c.pagefile / 2**20:.0f} MiB")
        time.sleep(300)


threading.Thread(target=beat, daemon=True).start()
sys.path.insert(0, "C:/git/semio")
p = Path("C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🐍️.py")
spec = importlib.util.spec_from_file_location("bim_tests", p)
m = importlib.util.module_from_spec(spec)
sys.modules["bim_tests"] = m
spec.loader.exec_module(m)
suite = unittest.TestSuite()
for name in CLASSES:
    suite.addTests(unittest.defaultTestLoader.loadTestsFromTestCase(getattr(m, name)))
result = unittest.TextTestRunner(verbosity=2, stream=sys.stdout).run(suite)
note(f"done run={result.testsRun} failures={len(result.failures)} errors={len(result.errors)} skipped={len(result.skipped)}")
sys.exit(0 if result.wasSuccessful() else 1)
