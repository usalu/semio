"""🔬 Sample building_geometry's main-thread stack every 3 s for N seconds and print function histograms plus slab/wall tags seen.
Usage: sample_geometry.py <seconds> <outfile-ascii>"""
import collections, importlib, sys, threading, time
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
seconds, out = float(sys.argv[1]), sys.argv[2]
main = threading.main_thread().ident
leaf, incl, tags = collections.Counter(), collections.Counter(), collections.Counter()
def sample():
    end = time.time() + seconds
    while time.time() < end:
        frame = sys._current_frames().get(main)
        names = []
        while frame:
            names.append(frame.f_code.co_name)
            for key in ("slab", "obj", "wall"):
                value = frame.f_locals.get(key) if frame.f_code.co_name in ("_slab_voids", "_slab_geometry_uncached", "_wall_geometry_unjoined", "_wall_cutters") else None
                if isinstance(value, dict) and "tag" in value:
                    tags[(frame.f_code.co_name, str(value["tag"]))] += 1
            frame = frame.f_back
        leaf[names[0]] += 1
        for n in set(names):
            incl[n] += 1
        time.sleep(3)
    with open(out, "w", encoding="ascii", errors="replace") as f:
        f.write("LEAF %s\nINCL %s\nTAGS %s\n" % (leaf.most_common(8), incl.most_common(25), tags.most_common(15)))
    import os; os._exit(0)
threading.Thread(target=sample, daemon=True).start()
importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️").building_geometry()
