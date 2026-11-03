import importlib
import sys
import time
from pathlib import Path

sys.path.insert(0, ".")
sys.path.insert(0, str(Path(__file__).parent))
from p1_rss import peak_mb

S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
for cast in sys.argv[1:]:
    t0 = time.time()
    S.ensure_casts([cast])
    t1 = time.time()
    m = S.staffage_motion(cast, unit=0.4)
    t2 = time.time()
    n = len(m.frames) if hasattr(m, "frames") else -1
    for _ in range(300):
        m.update(1 / 15)
    t3 = time.time()
    print(f"[DEBUG] {cast}: frames={n} ensure={t1 - t0:.1f}s load={t2 - t1:.1f}s 300 updates={t3 - t2:.2f}s peak={peak_mb():.0f}MB", flush=True)
