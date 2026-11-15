import sys; sys.path.insert(0, ".")
import importlib, os
os.environ.setdefault("SEMIO_TEACHING_PROFILE","preview"); os.environ.setdefault("SEMIO_TEACHING_LANGUAGE","en")
S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
c = "physics-fundamentals-shower"
d = S._index(S.ensure_casts([c])[c])["logical"]
print({k: v for k, v in d.items() if k != "partsM"})
for n, p in d["partsM"].items():
    import numpy as np
    a = np.array(p); print(n, a.min(0).round(3), a.max(0).round(3))
import numpy as np
for n in ("head", "handle", "holder"):
    print(n, np.array(d["partsM"][n]).round(3).tolist())
