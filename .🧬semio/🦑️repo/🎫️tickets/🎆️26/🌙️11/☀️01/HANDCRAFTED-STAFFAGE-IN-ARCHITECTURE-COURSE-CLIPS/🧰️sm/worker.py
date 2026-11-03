import sys, os, json, threading, importlib.util, traceback, inspect, time
from pathlib import Path
file = Path(sys.argv[1]).resolve()
out = Path(sys.argv[2])
only = sys.argv[3].split(",") if len(sys.argv) > 3 and sys.argv[3] else None
skip = set(sys.argv[4].split(",")) if len(sys.argv) > 4 and sys.argv[4] else set()
def rec(**k):
    k["file"] = str(file)
    with out.open("a", encoding="utf-8") as f:
        f.write(json.dumps(k, ensure_ascii=False) + "\n")
from manim import config, Scene
config.dry_run = True
config.quality = "low_quality"
config.frame_rate = 15
config.disable_caching = True
config.media_dir = os.environ["SM_MEDIA"]
config.verbosity = "ERROR"
config.progress_bar = "none"
sys.path.insert(0, str(file.parent))
name = "sm_" + str(abs(hash(str(file))))
try:
    spec = importlib.util.spec_from_file_location(name, file)
    mod = importlib.util.module_from_spec(spec)
    sys.modules[name] = mod
    spec.loader.exec_module(mod)
except BaseException:
    tb = traceback.format_exc().strip().splitlines()
    rec(scene="<import>", status="fail", head=tb[-1][:300], tb=tb[-6:])
    sys.exit(0)
scenes = [n for n, c in vars(mod).items() if inspect.isclass(c) and issubclass(c, Scene) and c.__module__ == name and c is not Scene and c.__dict__.get("construct") is not None or (inspect.isclass(c) and issubclass(c, Scene) and c.__module__ == name and "construct" in dir(c) and n != "Scene")]
scenes = list(dict.fromkeys(scenes))
if not scenes:
    rec(scene="<none>", status="noscene")
for n in scenes:
    if only and n not in only:
        continue
    if n in skip:
        continue
    t = time.time()
    rec(scene=n, status="start")
    def _wd(n=n):
        rec(scene=n, status="timeout", sec=int(os.environ.get("SM_SCENE_TIMEOUT", "420")))
        os._exit(3)
    wd = threading.Timer(float(os.environ.get("SM_SCENE_TIMEOUT", "420")), _wd)
    wd.daemon = True
    wd.start()
    try:
        inst = getattr(mod, n)(skip_animations=True)
        inst.render()
        wd.cancel()
        rec(scene=n, status="pass", sec=round(time.time() - t, 1))
    except BaseException as e:
        wd.cancel()
        tb = traceback.format_exc().strip().splitlines()
        rec(scene=n, status="fail", head=tb[-1][:300], tb=tb[-8:], sec=round(time.time() - t, 1))
