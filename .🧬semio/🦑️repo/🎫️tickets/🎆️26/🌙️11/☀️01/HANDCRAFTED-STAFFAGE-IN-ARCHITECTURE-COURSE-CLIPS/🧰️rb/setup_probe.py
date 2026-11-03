import faulthandler, sys, time, importlib.util
from pathlib import Path
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
sys.path.insert(0, "C:/git/semio")
import threading, traceback, collections
main = threading.get_ident()
def sample():
    counts = collections.Counter()
    while True:
        time.sleep(0.5)
        f = sys._current_frames().get(main)
        if f is None: continue
        stack = traceback.extract_stack(f)
        counts.update([f"{Path(s.filename).parent.name}/{Path(s.filename).name}:{s.name}:{s.lineno}" for s in stack[-3:]])
        if int(time.time()) % 60 == 0:
            print("[DEBUG] top", counts.most_common(14), flush=True)
threading.Thread(target=sample, daemon=True).start()
p = Path("C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🐍️.py")
spec = importlib.util.spec_from_file_location("bim_tests", p)
m = importlib.util.module_from_spec(spec); sys.modules["bim_tests"] = m
t = time.time(); spec.loader.exec_module(m); print("[DEBUG] import", time.time() - t, flush=True)
t = time.time(); api = m.model_api(); print("[DEBUG] model_api", time.time() - t, flush=True)
t = time.time(); model = api.load_model(); print("[DEBUG] load_model", time.time() - t, flush=True)
t = time.time(); geo = api.building_geometry(model); print("[DEBUG] building_geometry", time.time() - t, flush=True)
