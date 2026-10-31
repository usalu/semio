"""⏱️ Compare exact constant-section and original general kernels on retained actual circle/gravel candidates."""
import importlib.util,json,math,sys,time
from pathlib import Path
sys.dont_write_bytecode=True;ticket=Path(__file__).parent;root=Path.cwd();sys.path.insert(0,str(root))
spec=importlib.util.spec_from_file_location("expanded_profile_r95",ticket/"whole-expanded-interference-r92.py");m=importlib.util.module_from_spec(spec);sys.modules[spec.name]=m;spec.loader.exec_module(m)
data=json.loads((ticket/"🗑️generated/whole-diagnostic-r95/whole-expanded-owned-capture-r92.json").read_text(encoding="utf8"));circles=[r for r in data["records"]if r["geometry"].get("profile",{}).get("shape")=="circle"];gravel=[r for r in data["records"]if r["tag"]=="FIN-PILOTIS-GRAVEL"]
prepared=[];proof_started=time.perf_counter()
for r in gravel:
    p=r["geometry"];mesh=p["mesh"]
    if not m.constant_prism_proof(mesh["vertices"],mesh["faces"]):continue
    vertices=mesh["vertices"];bounds=(min(p[0]for p in vertices),min(p[1]for p in vertices),max(p[0]for p in vertices),max(p[1]for p in vertices));prepared.append((r,bounds))
proof_seconds=time.perf_counter()-proof_started;pairs=[]
for circle in circles:
    a=m.Solid(circle)
    for target,b in prepared:
        if a.hi<=target["geometry"]["z0"]or a.lo>=target["geometry"]["z1"]:continue
        c=a.bounds
        if c[2]<=b[0]or b[2]<=c[0]or c[3]<=b[1]or b[3]<=c[1]:continue
        pairs.append((circle,target))
        if len(pairs)>=48:break
    if len(pairs)>=48:break
rows=[];original=m.constant_prism_proof;old_total=new_total=0.;construction=[0.,0.]
for index,(source,target)in enumerate(pairs):
    now=time.perf_counter()
    try:m.constant_prism_proof=lambda *args:False;a,b=m.Solid(source),m.Solid(target)
    finally:m.constant_prism_proof=original
    construction[0]+=time.perf_counter()-now;m.Solid.section.cache_clear();now=time.perf_counter();old=m.intersection(a,b);old_seconds=time.perf_counter()-now;old_total+=old_seconds
    now=time.perf_counter();a,b=m.Solid(source),m.Solid(target);construction[1]+=time.perf_counter()-now;m.Solid.section.cache_clear();now=time.perf_counter();new=m.intersection(a,b);new_seconds=time.perf_counter()-now;new_total+=new_seconds
    assert abs(old["volume"]-new["volume"])<=max(old["error"],new["error"],1e-14),(source["tag"],target["index"],old,new)
    rows.append({"source":source["tag"],"target":target["tag"],"target_index":target["index"],"proof":b.prism_proof,"old":old,"new":new,"old_seconds":old_seconds,"new_seconds":new_seconds})
    if index%8==0:print("[DEBUG] measured",index,len(pairs),old_total,new_total,flush=True)
out={"scope":"Performance/equivalence of retained diagnostic actual inputs; not a final current-model interference certificate","capture_source":data["source"],"capture_stable":data["capture_stable"],"proven_gravel_cells":len(prepared),"all_gravel_cells":len(gravel),"proof_seconds":proof_seconds,"pairs":len(rows),"old_query_seconds":old_total,"new_query_seconds":new_total,"query_speedup":old_total/new_total if new_total else None,"old_construction_seconds":construction[0],"new_construction_seconds":construction[1],"results":rows}
(ticket/"🗑️generated/whole-expanded-prism-profile-r95.json").write_text(json.dumps(out,indent=2),encoding="utf8");print({k:v for k,v in out.items()if k!="results"},flush=True)
