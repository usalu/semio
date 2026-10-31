"""🔬 Re-evaluate retained actual union witnesses with the independently repaired sloping-section oracle."""
import importlib,json,math,sys,time
from pathlib import Path
sys.path.insert(0,str(Path.cwd()));B=Path(__file__).parent;O=B/"🗑️generated/native-r92/structural";start=time.monotonic()
T=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🏗️structure.🔗️joints.🧪️tests.🐍️")
data=json.loads((O/"actual-parts.json").read_text(encoding="utf-8"));case=json.loads((B/"structural-joint-source-cases-r95.json").read_text(encoding="utf-8"));rows=[]
for key,group in zip(case["group_ids"],case["groups"]):
    if not all(tag in data["baseline"]and tag in data["candidate"]for tag in group):continue
    before=T.volume([p for tag in group for p in data["baseline"][tag]["parts"]]);after=T.volume([p for tag in group for p in data["candidate"][tag]["parts"]]);rows.append({"group":key,"products":group,"baseline":before,"candidate":after,"difference":after-before,"conserved":math.isclose(before,after,abs_tol=2e-8,rel_tol=0)});print("[DEBUG] "+json.dumps(rows[-1]),flush=True)
report={"source":data["source"],"scope":"Retained actual source-bound native geometry re-evaluated by corrected independent oracle; not a current producer/publication certificate","rows":rows,"success":all(r["conserved"]for r in rows),"seconds":time.monotonic()-start};(O/"retained-union-corrected-oracle.json").write_text(json.dumps(report,indent=2),encoding="utf-8");raise SystemExit(not report["success"])
