import importlib.util, sys, time, traceback
from pathlib import Path

sys.path.insert(0, "C:/git/semio")
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace", line_buffering=True)
path = Path(sys.argv[1])
spec = importlib.util.spec_from_file_location("bw_module", path)
module = importlib.util.module_from_spec(spec)
sys.modules["bw_module"] = module
spec.loader.exec_module(module)
names = sorted(n for n in dir(module) if n.startswith("test_") and callable(getattr(module, n)))
ok = failed = 0
for name in names:
    started = time.time()
    try:
        getattr(module, name)()
        ok += 1
        status = "ok"
    except Exception:
        failed += 1
        status = "FAIL\n" + traceback.format_exc()
    print(f"{name} {status} ({time.time() - started:.1f}s)", flush=True)
print(f"Ran {len(names)} functions: ok={ok} failed={failed}", flush=True)
sys.exit(1 if failed else 0)
