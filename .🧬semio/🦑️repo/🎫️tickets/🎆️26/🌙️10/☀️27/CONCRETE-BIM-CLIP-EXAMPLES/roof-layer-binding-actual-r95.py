"""🏛️ Verify actual unpublished roof recipe metadata and true changed material ownership."""
import hashlib,importlib,importlib.util,json,math,sys,time,traceback
from pathlib import Path
from copy import deepcopy
from fractions import Fraction as Q
import numpy as np
from scipy.spatial import ConvexHull
from shapely.geometry import Polygon
from shapely.ops import unary_union
from jsonschema import Draft202012Validator
sys.path.insert(0,str(Path.cwd()));B=Path(__file__).parent;O=B/"🗑️generated/native-r92";START=time.perf_counter()
A=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
spec=importlib.util.spec_from_file_location("roof_binding_source",B/"roof-layer-binding-source-r95.py");S=importlib.util.module_from_spec(spec);spec.loader.exec_module(S)
tags=["FIN-ROOF-PAVING","FIN-ROOF-GRAVEL","FIN-ROOF-STAIR"]
digest=lambda value:hashlib.sha256(json.dumps(value,sort_keys=True,separators=(",",":"),ensure_ascii=False).encode()).hexdigest()
def guard():
    """⏳ Bound only this recipe proof and honor its own cooperative cancellation marker."""
    if time.perf_counter()-START>900:raise TimeoutError("Own900second roof recipe proof bound")
    if (O/"roof-layer-binding-cancel").exists():raise InterruptedError("Own roof recipe proof cancellation")
def scipy_volume(part):
    """🔬 Condition true binary64 vertices in an exact local frame before independent native hull evaluation."""
    vertices=[[Q(v) for v in p] for p in A.part_vertices(part)];ring=[[Q(v) for v in p] for p in part["outline"]]
    u=max(([b[0]-a[0],b[1]-a[1]] for a,b in zip(ring,ring[1:]+ring[:1])),key=lambda v:v[0]*v[0]+v[1]*v[1]);det=u[0]*u[0]+u[1]*u[1];origin=vertices[0]
    coords=[[(x-origin[0])*u[0]+(y-origin[1])*u[1],-(x-origin[0])*u[1]+(y-origin[1])*u[0],z-origin[2]] for x,y,z in vertices]
    low=[min(v[k] for v in coords) for k in range(3)];span=[max(v[k] for v in coords)-low[k] for k in range(3)]
    assert min(span)>0,"Published native cell loses a positive dimension"
    unit=np.asarray([[float((p[k]-low[k])/span[k]) for k in range(3)] for p in coords],float)
    return ConvexHull(unit).volume*float(span[0]*span[1]*span[2]/det)
def main():
    """🛡️ Require exact original native baseline and independently occupied new roof material."""
    raw=A._source();m=json.loads(raw);source=hashlib.sha256(raw.encode()).hexdigest();before=deepcopy(m)
    original={tag:digest(A._solid_geometry(m,A._find(m,tag))["parts"]) for tag in tags}
    S.apply(m)
    baseline={tag:digest(A._solid_geometry(m,A._find(m,tag))["parts"]) for tag in tags}
    assert original==baseline
    stripped=deepcopy(m)
    for tag in tags:A._find(stripped,tag)["geometry"].pop("layer_binding")
    assert stripped==before
    schema=json.loads((Path(A.__file__).parent/"🧬️schema/🔣️.schema.json").read_text(encoding="utf-8"))
    errors=list(Draft202012Validator(schema).iter_errors(m));assert not errors,[str(e)[:200] for e in errors[:5]]
    print("[DEBUG] actual roof baseline exact/schema GREEN",flush=True)
    rows=[]
    for name in ("roof_paving","roof_paving_alt"):
        guard();changed=A.apply_event(A.demo_event(name,m),m);parts_by={};started=time.perf_counter()
        changed_tags=sorted(o["tag"] for o in changed["elements"] if o.get("metadata")!=A._find(m,o["tag"]).get("metadata"))
        assert changed_tags==sorted(tags+["SL-R","SL-KIOSK"])
        for tag in tags:
            guard();progress=[]
            def observe(event):
                guard();progress.append(event)
                if event["completed"]%250==0 or event["completed"]==event["total"]:print("[DEBUG] "+name+" "+tag+" "+json.dumps(event),flush=True)
            parts=A.solid_geometry(tag,changed,progress=observe)["parts"];parts_by[tag]=parts
            native=math.fsum(A._polyhedron_volume(p) for p in parts)
            independent=math.fsum(scipy_volume(p) for p in parts)
            for index,part in enumerate(parts):
                try:
                    native_part=A._polyhedron_part(A.part_vertices(part),A.part_faces(part))
                    assert A._polyhedron_volume(native_part)>0
                except BaseException:
                    (O/"roof-layer-rejected-part-r95.json").write_text(json.dumps({"tag":tag,"index":index,"part":part,"trace":traceback.format_exc()},indent=2),encoding="utf-8");raise
            levels=sorted({p[k] for p in parts for k in ("z0","z1")})
            occupied=math.fsum((hi-lo)*unary_union([Polygon(p["outline"]) for p in parts if p["z0"]<(lo+hi)/2<p["z1"]]).area for lo,hi in zip(levels,levels[1:]))
            assert math.isclose(native,independent,abs_tol=2e-8) and math.isclose(native,occupied,abs_tol=2e-8),(tag,native,independent,occupied)
            for part in parts:
                assert part["z1"]>part["z0"] and Polygon(part["outline"]).is_valid
            quantities=A.quantities(tag,changed);assert math.isclose(quantities["volume"],occupied,abs_tol=2e-8)
            rows.append({"event":name,"tag":tag,"parts":len(parts),"native_volume":native,"scipy_volume":independent,"geos_volume":occupied,"public_volume":quantities["volume"],"top":max(p["z1"] for p in parts)})
        for tag in ("SL-R","SL-KIOSK"):
            guard();slab=A._slab_geometry_entry(changed,A._find(changed,tag));assert slab["shared_finish_parts"]
            print("[DEBUG] actual occupied reserve "+name+" "+tag+" GREEN",flush=True)
        print("[DEBUG] actual roof event "+name+" GREEN "+str(time.perf_counter()-started),flush=True)
    assert source==hashlib.sha256(A._source().encode()).hexdigest()
    report={"success":True,"source_sha256":source,"baseline_native_hashes":baseline,"baseline_source_parts_unchanged":True,"schema_errors":0,"rows":rows,"seconds":time.perf_counter()-START,"api_sha256":hashlib.sha256(Path(A.__file__).read_bytes()).hexdigest(),"recipe_sha256":hashlib.sha256(Path(A._flooring_layer_backend().__file__).read_bytes()).hexdigest(),"publication":"UNPUBLISHED binding metadata candidate; no source geometry or identity write"}
    (O/"roof-layer-binding-actual-r95.json").write_text(json.dumps(report,indent=2),encoding="utf-8");print("[DEBUG] "+json.dumps(report),flush=True)
try:main()
except BaseException:
    (O/"roof-layer-binding-actual-failure-r95.json").write_text(json.dumps({"error":traceback.format_exc(),"seconds":time.perf_counter()-START},indent=2),encoding="utf-8");raise
