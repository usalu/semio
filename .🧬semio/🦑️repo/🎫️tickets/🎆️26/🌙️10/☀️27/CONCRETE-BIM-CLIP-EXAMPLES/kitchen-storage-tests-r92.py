"""🧪 Execute the dedicated native kitchen and preserved hollow cabinet contracts."""
import importlib,json,sys,time,traceback
from pathlib import Path
sys.path.insert(0,str(Path.cwd()));results=[]
for suffix in ("🍳️kitchen","🗄️storage"):
 module=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests."+suffix)
 for name in sorted(n for n in dir(module) if n.startswith("test_")):
  start=time.perf_counter()
  try:getattr(module,name)();row={"module":suffix,"test":name,"passed":True,"seconds":time.perf_counter()-start}
  except Exception as error:
   detail=(type(error).__name()+": "+error.message+"; path="+str(list(error.absolute_path))) if hasattr(error,"absolute_path") else traceback.format_exc()
   row={"module":suffix,"test":name,"passed":False,"seconds":time.perf_counter()-start,"traceback":detail}
  results.append(row);print("[DEBUG] "+json.dumps(row,ensure_ascii=False),flush=True)
(Path(__file__).parent/"🗑️generated"/"kitchen-storage-tests-r92.json").write_text(json.dumps(results,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
sys.exit(0 if all(r["passed"] for r in results) else 1)
