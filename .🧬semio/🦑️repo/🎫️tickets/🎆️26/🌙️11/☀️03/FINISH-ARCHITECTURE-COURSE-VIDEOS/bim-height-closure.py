"""🔎 Check active BIM source dependency closures for the proposed 117 source edit."""
import importlib,json,sys
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
r=importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
settings={"language":"en","quality":"-ql","resolution":None,"fps":15,"audience":"speaker"}
jobs=r.source_plan(r.segment_plan())
target=next((arg for arg in sys.argv[1:] if arg!="--other"),None) or "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧩️models/🐍️.py"
def strings(value):
    if isinstance(value,dict):
        for k,v in value.items():yield str(k);yield from strings(v)
    elif isinstance(value,(tuple,list)):
        for v in value:yield from strings(v)
    elif isinstance(value,str):yield value
for job in jobs:
    state=r.segment_state(job["segment"],settings)
    matches=[x for x in strings(state) if target in x.replace("\\","/")]
    print("[DEBUG] ordinal",job["source_ordinal"],"target matches",len(matches),flush=True)
    if matches:print(matches,flush=True)
