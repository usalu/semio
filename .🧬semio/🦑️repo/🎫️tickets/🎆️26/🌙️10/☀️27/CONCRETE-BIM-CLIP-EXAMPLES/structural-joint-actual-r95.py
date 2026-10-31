"""🏛️ Prove actual unpublished structural owner declarations with true native geometry and independent unions."""
import hashlib,importlib,importlib.util,json,math,os,sys,time,traceback,threading,_thread,faulthandler
from copy import deepcopy
from pathlib import Path
from fractions import Fraction as Q
import numpy as np
from scipy.spatial import ConvexHull
from jsonschema import Draft202012Validator
sys.path.insert(0,str(Path.cwd()));B=Path(__file__).parent;O=B/"🗑️generated/native-r92/structural";O.mkdir(parents=True,exist_ok=True);START=time.monotonic();STOP=threading.Event();PHASE={"value":"starting"}
A=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
T=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🏗️structure.🔗️joints.🧪️tests.🐍️")
spec=importlib.util.spec_from_file_location("structural_source",B/"structural-joint-source-r95.py");S=importlib.util.module_from_spec(spec);spec.loader.exec_module(S)
def memory():
    """🌡️ Read only the current process through native OS counters."""
    if os.name=="nt":
        import ctypes
        from ctypes import wintypes
        class Counters(ctypes.Structure):
            _fields_=[("cb",wintypes.DWORD),("faults",wintypes.DWORD)]+[(n,ctypes.c_size_t) for n in ("peak","rss","peak_paged","paged","peak_nonpaged","nonpaged","pagefile","peak_pagefile","private")]
        data=Counters();data.cb=ctypes.sizeof(data);k=ctypes.WinDLL("kernel32",use_last_error=True);k.GetCurrentProcess.restype=wintypes.HANDLE;p=ctypes.WinDLL("psapi",use_last_error=True);p.GetProcessMemoryInfo.argtypes=[wintypes.HANDLE,ctypes.c_void_p,wintypes.DWORD]
        if not p.GetProcessMemoryInfo(k.GetCurrentProcess(),ctypes.byref(data),data.cb):raise OSError(ctypes.get_last_error())
        return {"rss":data.rss,"peak":data.peak}
    import resource
    peak=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss*(1 if sys.platform=="darwin" else 1024)
    return {"rss":peak,"peak":peak}
def guard():
    """⏳ Bound this proof without changing admission or controlling peer processes."""
    if time.monotonic()-START>1500:raise TimeoutError("Own1500second structural proof bound")
    if memory()["rss"]>5905580032:raise MemoryError("Own5.5GiB structural proof bound")
    if (O/"cancel").exists():raise InterruptedError("Own structural proof cancellation")
def observe():
    """🧵 Preserve safe Python snapshots and cooperative bounds without a native timed watchdog."""
    main=threading.main_thread().ident;sample=START
    while not STOP.wait(1):
        if time.monotonic()-sample>60:
            frame=sys._current_frames().get(main)
            if frame:
                with (O/"stack.log").open("a",encoding="utf-8") as out:out.write("[DEBUG] "+PHASE["value"]+chr(10));out.writelines(traceback.format_stack(frame))
            sample=time.monotonic()
        try:guard()
        except BaseException:
            (O/"bound.json").write_text(json.dumps({"error":traceback.format_exc(),"phase":PHASE["value"],"seconds":time.monotonic()-START,**memory()}),encoding="utf-8");_thread.interrupt_main();return
def event(phase):
    """🧾 Flush actual product/validation milestones."""
    PHASE["value"]=phase;print("[DEBUG] "+json.dumps({"phase":phase,"seconds":time.monotonic()-START,**memory()}),flush=True);guard()
def shape(m,tag):
    """🧱 Read actual native physical cells without whole-building construction."""
    obj=A._find(m,tag)
    if obj["class"]=="IfcSlab":return A._slab_geometry_entry(m,obj)
    if obj["class"]=="IfcStair":return A._stair_geometry(m,obj)
    if obj["class"]=="IfcColumn":return {**A.column_geometry(m,obj),"full_parts":A.column_geometry(m,obj)["parts"]}
    if obj["class"]in ("IfcRamp","IfcRampFlight"):return A._ramp_geometry(m,obj)
    return A._solid_geometry(m,obj)
def normalize_volume(part):
    """🔬 Condition true Float64 vertices in an exact local affine frame before independent SciPy."""
    values=[[Q(v) for v in p] for p in A.part_vertices(part)];origin=values[0]
    edges=[([values[j][k]-values[i][k] for k in range(3)]) for face in A.part_faces(part) for i,j in zip(face,face[1:]+face[:1])]
    u=max(edges,key=lambda v:sum(x*x for x in v));v=max(edges,key=lambda q:sum((u[(k+1)%3]*q[(k+2)%3]-u[(k+2)%3]*q[(k+1)%3])**2 for k in range(3)))
    normal=[u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]];side=[normal[1]*u[2]-normal[2]*u[1],normal[2]*u[0]-normal[0]*u[2],normal[0]*u[1]-normal[1]*u[0]];basis=[u,side,normal]
    det=abs(u[0]*(side[1]*normal[2]-side[2]*normal[1])-u[1]*(side[0]*normal[2]-side[2]*normal[0])+u[2]*(side[0]*normal[1]-side[1]*normal[0]));assert det>0
    coords=[[sum((point[k]-origin[k])*axis[k] for k in range(3)) for axis in basis] for point in values];lo=[min(p[k] for p in coords) for k in range(3)];span=[max(p[k] for p in coords)-lo[k] for k in range(3)];assert min(span)>0
    unit=np.asarray([[float((p[k]-lo[k])/span[k]) for k in range(3)] for p in coords],float)
    return ConvexHull(unit).volume*float(span[0]*span[1]*span[2]/det)
def hashes():
    """🔐 Record all actually relevant first-party physical source producers."""
    root=Path.cwd();paths=[Path(A.__file__),Path(A._structural_joint_backend().__file__),Path(A._flooring_layer_backend().__file__),Path(A.__file__).parent/"🧬️schema/🔣️.schema.json",Path(__file__),B/"structural-joint-source-r95.py",B/"structural-joint-source-cases-r95.json"]
    return {str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
def main():
    """🛡️ Require complete physical coverage, strict native cells and union conservation before any source write."""
    raw=A._source();m=json.loads(raw);source=hashlib.sha256(raw.encode()).hexdigest();case=json.loads((B/"structural-joint-source-cases-r95.json").read_text(encoding="utf-8"));before_hashes=hashes()
    selected=set(sys.argv[1:]) or set(case["group_ids"]);assert selected<=set(case["group_ids"])
    groups=[group for key,group in zip(case["group_ids"],case["groups"]) if key in selected]
    tags=list(dict.fromkeys(tag for group in groups for tag in group));baseline={}
    (O/"started.json").write_text(json.dumps({"source":source,"before_producers":before_hashes,"selected_groups":sorted(selected),"whole_manifest":len(groups)==len(case["groups"]),"own_bounds":{"seconds":1500,"rss":5905580032}},indent=2),encoding="utf-8")
    for tag in tags:event("baseline:"+tag);baseline[tag]=shape(m,tag)
    original=deepcopy(m);S.apply(m,ramps=True)
    stripped=deepcopy(m)
    for row in case["relationships"]:A._find(stripped,row["donor"])["geometry"].pop("structural_joints",None)
    for row in case.get("column_reservations",()):
        target=A._find(stripped,row["donor"])["geometry"];prior=A._find(original,row["donor"])["geometry"]
        if "column_reserves" in prior:target["column_reserves"]=deepcopy(prior["column_reserves"])
        else:target.pop("column_reserves",None)
    for row in case.get("ramp_interfaces",()):
        target=A._find(stripped,row["donor"])["geometry"];prior=A._find(original,row["donor"])["geometry"]
        if "interface_owners" in prior:target["interface_owners"]=deepcopy(prior["interface_owners"])
        else:target.pop("interface_owners",None)
    for row in case.get("receiving_interfaces",()):
        target=A._find(stripped,row["donor"])["geometry"];prior=A._find(original,row["donor"])["geometry"]
        if "interface_owners" in prior:target["interface_owners"]=deepcopy(prior["interface_owners"])
        else:target.pop("interface_owners",None)
    for row in case["relationships"]+case.get("column_reservations",[])+case.get("receiving_interfaces",[])+case.get("ramp_interfaces",[]):
        target=A._find(stripped,row["donor"]);prior=A._find(original,row["donor"])
        if "psets" in prior:target["psets"]=deepcopy(prior["psets"])
        else:target.pop("psets",None)
    assert stripped==original
    schema=json.loads((Path(A.__file__).parent/"🧬️schema/🔣️.schema.json").read_text(encoding="utf-8"));event("candidate-schema");errors=list(Draft202012Validator(schema).iter_errors(m));assert not errors,[e.message for e in errors[:5]]
    candidate={};references={};issues=[]
    for tag in tags:
        event("candidate:"+tag);candidate[tag]=shape(m,tag)
        if A._find(m,tag)["class"]not in ("IfcSlab","IfcColumn"):assert T.digest(candidate[tag]["full_parts"])==T.digest(baseline[tag]["full_parts"]),tag
        references[tag]=T.digest(candidate[tag]["full_parts"]);issues.extend({"product":tag,**i} for i in candidate[tag].get("construction_issues",[]))
        for index,p in enumerate(candidate[tag]["parts"]):
            guard()
            try:
                native=A._polyhedron_part(A.part_vertices(p),A.part_faces(p));volume=A._polyhedron_volume(native);independent=normalize_volume(native);assert volume>0 and independent>0
                assert math.isclose(volume,independent,abs_tol=2e-10,rel_tol=2e-10),(tag,index,volume,independent)
            except BaseException:
                (O/"rejected-part.json").write_text(json.dumps({"tag":tag,"index":index,"part":p,"error":traceback.format_exc()},indent=2),encoding="utf-8");raise
    (O/"actual-parts.json").write_text(json.dumps({"source":source,"baseline":baseline,"candidate":candidate}),encoding="utf-8")
    rows=[]
    for group in groups:
        event("union:"+"/".join(group));old=T.volume([p for tag in group for p in baseline[tag]["parts"]]);new=T.volume([p for tag in group for p in candidate[tag]["parts"]]);assert math.isclose(old,new,abs_tol=2e-8,rel_tol=0),(group,old,new)
        rows.append({"products":group,"baseline_occupied_union":old,"candidate_occupied_union":new})
    for row in case["relationships"]:
        if row["donor"] not in candidate or row["owner"] not in candidate:continue
        event("exclusive:"+row["donor"]+"/"+row["owner"]);a=candidate[row["donor"]]["parts"];b=candidate[row["owner"]]["parts"]
        for material in sorted({p.get("material",A._find(m,row["donor"]).get("material")) for p in a}):
            first=[p for p in a if p.get("material",A._find(m,row["donor"]).get("material"))==material];second=[p for p in b if p.get("material",A._find(m,row["owner"]).get("material"))==material]
            if not first or not second:continue
            overlap=T.volume(first)+T.volume(second)-T.volume(first+second);assert abs(overlap)<2e-8,(row,material,overlap)
            rows.append({"donor":row["donor"],"owner":row["owner"],"material":material,"candidate_common_volume":overlap})
    assert source==hashlib.sha256(A._source().encode()).hexdigest();after_hashes=hashes()
    out={"success":True,"source":source,"selected_groups":sorted(selected),"whole_manifest":len(groups)==len(case["groups"]),"scope":"Unpublished explicit structural joints, all selected native cells positive/closed and independently measured; foreign-material conflicts retained/reported","stable_producers":before_hashes==after_hashes,"before_producers":before_hashes,"after_producers":after_hashes,"groups":rows,"reference_hashes":references,"construction_issues":issues,"ready_for_publication":not issues and before_hashes==after_hashes,"seconds":time.monotonic()-START,**memory()}
    (O/"actual-result.json").write_text(json.dumps(out,indent=2),encoding="utf-8");(O/"actual-parts.json").write_text(json.dumps({"source":source,"baseline":baseline,"candidate":candidate}),encoding="utf-8");print("[DEBUG] "+json.dumps(out),flush=True)
STACK=(O/"fatal.log").open("w",encoding="utf-8");faulthandler.enable(file=STACK,all_threads=True);watcher=threading.Thread(target=observe,daemon=True);watcher.start()
try:main()
except BaseException:
    (O/"failure.json").write_text(json.dumps({"error":traceback.format_exc(),"phase":PHASE["value"],"seconds":time.monotonic()-START,**memory()},indent=2),encoding="utf-8");raise
finally:STOP.set();watcher.join(timeout=2)
