import importlib,json,sys
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
c=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.♻️cache.🐍️")
d=json.loads((Path(__file__).parent/"🗑️generated/bim-boundary-minimal.json").read_text(encoding="utf-8"))
print(json.dumps(c.decode(d["facets"]),ensure_ascii=False,indent=1))
