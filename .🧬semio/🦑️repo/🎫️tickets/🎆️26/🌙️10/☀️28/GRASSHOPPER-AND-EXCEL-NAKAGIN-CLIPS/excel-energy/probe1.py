import importlib, sys
from pathlib import Path
ROOT = Path("C:/git/semio"); sys.path.insert(0, str(ROOT))
v = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🎨️visuals.🐍️")
print({k: getattr(v, k) for k in "INK MUTED ARCH STRUCT SERV DATA GOOD WARN GLASS CONCRETE".split()})
n = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
t = n.capsule_type("A"); print(t.window_u, t.window_v, t.width, t.length, t.height)
import inspect
d = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
src = inspect.getsource(d.NakaginDrawing.capsule_axon); print(src[:3500])
