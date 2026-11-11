"""🧪 Persist the minimized exact-float boundary case as a portable fixture."""
import importlib,json,sys
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
cache=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.♻️cache.🐍️")
here=Path(__file__).parent
data=json.loads((here/"🗑️generated/bim-boundary-minimal.json").read_text(encoding="utf-8"))
facets=cache.decode(data["facets"])
out=Path("🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/📐️drafting/🧩️surfaces/🧪️tests/🔣️tap.json")
out.write_text(json.dumps({"id":"tap-hall-coplanar-cancellation","facets":facets},ensure_ascii=False,indent=2)+"\n",encoding="utf-8")
print("[DEBUG] persisted",len(facets),"exact source facets",flush=True)
