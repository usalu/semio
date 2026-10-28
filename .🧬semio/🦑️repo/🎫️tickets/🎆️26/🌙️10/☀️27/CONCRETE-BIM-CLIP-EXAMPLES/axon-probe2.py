import importlib, sys
from pathlib import Path
TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
for name in ("architecture", "structure", "services"):
    info = m.partial_model(name)
    print(name, info["counts_by_class"], len(info["element_tags"]))
    print(sorted(info["element_tags"]) if name != "structure" else sorted(t for t in info["element_tags"] if not t[0] in "ABCDE" or len(t) > 2))
print(m.element("SL-R")["material"]["layers"][0].keys())
for e in m.load_model()["elements"]:
    if e["tag"] in ("W-S", "RW-S", "OP-S"):
        print(e["tag"], e["class"], e.get("geometry"), e.get("attributes"))
