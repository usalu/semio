"""🧬 Fingerprint unrelated active source segments before and after interop addition."""
import hashlib
import importlib
import json
import sys
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
r=importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
settings={"language":"en","quality":"-ql","resolution":None,"fps":15,"audience":"speaker"}
jobs=r.source_plan(r.segment_plan())
selected=[job for job in jobs if job["source_ordinal"] in (1,122,124,125,127)]
selected += [next(job for job in jobs if job["video_id"]=="optimization_extrema")]
result={}
for job in selected:
    state=r.segment_state(job["segment"],settings)
    result[str(job["source_ordinal"])]=hashlib.sha256(json.dumps(state,sort_keys=True).encode()).hexdigest()
output=Path(__file__).parent/"🗑️generated"/("family-fingerprint-"+sys.argv[1]+".json")
output.write_text(json.dumps(result,indent=2)+"\n",encoding="utf-8")
print(result,flush=True)
