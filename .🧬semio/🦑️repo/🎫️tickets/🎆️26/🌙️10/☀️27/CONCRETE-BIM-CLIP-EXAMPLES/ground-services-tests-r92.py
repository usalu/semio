"""🧪 Execute selected native ground services contracts without another test framework."""
import importlib
import json
import sys
import time
import traceback
from pathlib import Path

sys.path.insert(0,str(Path.cwd()))
module=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚿️ground")
names=sys.argv[1:] or [name for name in dir(module) if name.startswith("test_")]
results=[]
for name in names:
    started=time.perf_counter()
    try:
        getattr(module,name)()
        results.append({"test":name,"passed":True,"seconds":time.perf_counter()-started})
    except Exception:
        results.append({"test":name,"passed":False,"seconds":time.perf_counter()-started,"traceback":traceback.format_exc()})
    print("[DEBUG] "+json.dumps(results[-1],ensure_ascii=False),flush=True)
output=Path(__file__).parent/"🗑️generated"/"ground-services-tests-r92.json"
output.write_text(json.dumps(results,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
sys.exit(0 if all(row["passed"] for row in results) else 1)
