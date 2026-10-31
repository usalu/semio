"""🧪 Certify all actual gravel cells and native painter/Cairo output without rebuilding the model."""
import hashlib,importlib,json,math,sys,time,traceback
from pathlib import Path
import numpy as np
from shapely.geometry import Polygon,LineString
from shapely.ops import unary_union
sys.dont_write_bytecode=True;root=Path.cwd();sys.path.insert(0,str(root));ticket=Path(__file__).parent;generated=ticket/"🗑️generated"
base="🎓️teaching.🏛️architecture.🏗️bim.🎬️clip";source=root/"🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json"
def peak_rss_bytes():
    """🧠 Read the native lifetime process RSS peak through an isolated standard-library interface."""
    if sys.platform!='win32':
        import resource
        return int(resource.getrusage(resource.RUSAGE_SELF).ru_maxrss)*(1 if sys.platform=='darwin' else 1024)
    import ctypes
    from ctypes import wintypes
    class Counters(ctypes.Structure):
        """📊 Native operating-system process counters, kept private to this input harness."""
        _fields_=[('cb',wintypes.DWORD),('faults',wintypes.DWORD),('sizes',ctypes.c_size_t*8)]
    counters=Counters();counters.cb=ctypes.sizeof(counters)
    kernel=ctypes.WinDLL('kernel32',use_last_error=True);kernel.GetCurrentProcess.restype=ctypes.c_void_p
    native=ctypes.WinDLL('psapi',use_last_error=True);native.GetProcessMemoryInfo.argtypes=[ctypes.c_void_p,ctypes.POINTER(Counters),wintypes.DWORD]
    if not native.GetProcessMemoryInfo(kernel.GetCurrentProcess(),ctypes.byref(counters),ctypes.sizeof(counters)):
        raise ctypes.WinError(ctypes.get_last_error())
    return int(counters.sizes[0])

started=time.monotonic();raw=source.read_bytes();model=json.loads(raw);element=next(e for e in model["elements"]if e["tag"]=="FIN-PILOTIS-GRAVEL")
primitives=importlib.import_module(base+".🎨️visuals.🏛️savoye.🧊️axon.🧩️primitives");axon=importlib.import_module(base+".🎨️visuals.🏛️savoye.🧊️axon.🐍️");skin=primitives._surfaces
paths=[Path(x.__file__)for x in (primitives,axon,skin,skin._r,primitives._c,primitives._c._model,axon._d,axon._d._coverage)];before={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest()for p in paths}
item={**element,**element["geometry"]};info={"tag":element["tag"],"guid":element["guid"],"cls":element["class"],"kind":item["kind"],"disciplines":("architecture",),"names":frozenset((element["tag"],element["guid"],element["class"],item["kind"]))}
print("[DEBUG] full source surface start",flush=True);prims=primitives._surface_prims(item,info,3.17,6.6,0);print("[DEBUG] full source surface done",time.monotonic()-started,flush=True)
facets=[{"rings":f[1],"normal":f[2],"material":p.skin[i]}for p in prims for i,f in enumerate(p.faces)]
original=[{"rings":f[1],"normal":f[2],"material":f[3]}for part in item["parts"]for f in primitives._part_faces(part,"gravel")]
def polygons(rows,horizontal=False):
    """📐 Independently project actual upper material to GEOS, including every hole."""
    return [Polygon([p[:2]for p in f["rings"][0]],[[p[:2]for p in ring]for ring in f["rings"][1:]])for f in rows if f["normal"][2]>0 and(not horizontal or all(p[2]==-.15 for p in f["rings"][0]))]
metrics={}
for label,horizontal in (("horizontal",True),("all_upper_projection",False)):
    a,b=polygons(original,horizontal),polygons(facets,horizontal);want=unary_union(a);got=unary_union(b)
    metrics[label]={"source_faces":len(a),"output_faces":len(b),"source_area":want.area,"output_area":got.area,"area_difference":got.symmetric_difference(want).area,"boundary_difference":got.boundary.symmetric_difference(want.boundary).length,"repeated_fill_area":sum(p.area for p in b)-got.area}
    assert metrics[label]["area_difference"]<1e-10,metrics[label]
    assert abs(metrics[label]["repeated_fill_area"])<1e-10,metrics[label]
    if horizontal:assert metrics[label]["boundary_difference"]<1e-10,metrics[label]
def flux(rows):
    """🧊 Independently integrate the oriented closed-facet volume through tetrahedron determinants."""
    total=0.
    for facet in rows:
        for ri,ring in enumerate(facet["rings"]):
            a=np.asarray(ring[0]);normal=np.asarray(facet["normal"]);area=sum(np.cross(np.asarray(ring[i])-a,np.asarray(ring[i+1])-a)for i in range(1,len(ring)-1))
            sign=1 if area@normal>0 else -1
            total+=(1 if ri==0 else -1)*sign*sum(float(a@np.cross(ring[i],ring[i+1]))/6 for i in range(1,len(ring)-1))
    return total
metrics["closed_volume"]={"source":flux(original),"output":flux(facets)};assert abs(metrics["closed_volume"]["source"]-metrics["closed_volume"]["output"])<1e-10,metrics["closed_volume"]
records=[];original_multi=axon._multi;original_region=axon._d.region;original_segments=axon._d.segments
def record_multi(rings,fill,opacity,stroke,width,stroke_opacity):
    """📝 Record the actual native painter fill contract before creating its native object."""
    records.append({"kind":"fill","rings":len(rings),"stroke":stroke,"width":width});return original_multi(rings,fill,opacity,stroke,width,stroke_opacity)
def record_region(rings,fill,opacity,stroke,width,stroke_opacity):
    """📝 Preserve the compound-region fill and hole contract through the real renderer."""
    records.append({"kind":"fill","rings":len(rings),"stroke":stroke,"width":width});return original_region(rings,fill,opacity,stroke,width,stroke_opacity)
def record_segments(pairs,stroke,width,opacity):
    """📝 Record actual native material-boundary paths before creating their native object."""
    records.append({"kind":"ink","segments":len(pairs),"width":width});return original_segments(pairs,stroke,width,opacity)
axon._multi=record_multi;axon._d.region=record_region;axon._d.segments=record_segments
import manim
from PIL import Image
oracle=json.loads((generated/"material-skin-exact-oracle-r95.json").read_text(encoding="utf8"));requested=sys.argv[1:]or list(axon.VIEWS)
assert all(view in oracle["views"]for view in requested),"Exact third-party views are still pending"
for view_name in requested:
    turns=axon.VIEWS[view_name]
    view=axon.AxonView(scale=.42,centre=(9.5,10.75,-.15),turns=turns);painter=axon._Painter(view,axon._Selection(),False)
    records.clear();actors=manim.VGroup(*(painter.render(p,"architecture",0)for p in prims));visible=[f for f in facets if view.facing(f["normal"])>1e-6]
    regions=prims[0].outlines[(view.turns,skin.producer_token())]
    segments=[(a,b)for region in regions for ring in region["rings"]for a,b in zip(ring,ring[1:]+ring[:1])]
    assert all(r["stroke"]is None and r["width"]==0 for r in records if r["kind"]=="fill"),records
    assert sum(r["segments"]for r in records if r["kind"]=="ink")==len(segments)
    c,s=round(math.cos(math.pi/2*turns)),round(math.sin(math.pi/2*turns));matrix=((c-s,c+s),(-s-c,c-s),(0,2))
    from fractions import Fraction as F
    project=lambda p:tuple(float(sum(F(p[i])*matrix[i][j]for i in range(3)))for j in range(2))
    expected=unary_union([Polygon(region["rings"][0],region["rings"][1:])for region in oracle["views"][view_name]["regions"]])
    actual=unary_union([Polygon(region["rings"][0],region["rings"][1:])for region in regions]);ink=unary_union([LineString(pair)for pair in segments])
    area_error=actual.symmetric_difference(expected).area;boundary_error=ink.symmetric_difference(expected.boundary).length
    assert area_error==0,(view_name,area_error)
    assert boundary_error==0,(view_name,boundary_error)
    metrics[view_name]={"visible_faces":len(visible),"ink_segments":len(segments),"native_family":len(actors.get_family()),"fill_buckets":len([r for r in records if r["kind"]=="fill"]),"projected_area_difference":area_error,"projected_boundary_difference":boundary_error}
    if view_name=="sw":
        actors.scale_to_fit_width(12);actors.move_to(manim.ORIGIN);camera=manim.Camera(pixel_width=1920,pixel_height=1080,background_color=axon._d.BACKGROUND)
        camera.capture_mobjects([actors]);image=camera.pixel_array
        metrics[view_name]["pixel_colors"]=len(np.unique(image[:,:,:3].reshape(-1,3),axis=0));assert metrics[view_name]["pixel_colors"]>10
        Image.fromarray(image).save(generated/"material-skin-gravel-native-sw-r95.png")
    print("[DEBUG] view",view_name,metrics[view_name],flush=True)
axon._multi=original_multi;axon._d.region=original_region;axon._d.segments=original_segments
receipt={"pass":True,"source":hashlib.sha256(raw).hexdigest(),"source_after":hashlib.sha256(source.read_bytes()).hexdigest(),"before":before,"after":{str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest()for p in paths},"source_cells":len(item["parts"]),"source_facets":len(original),"output_facets":len(facets),"primitives":len(prims),"peak_rss_bytes":peak_rss_bytes(),"accepted_views":requested,"independent_projected_oracle":oracle["oracle"],"independent_projected_oracle_version":oracle["version"],"metrics":metrics,"seconds":time.monotonic()-started}
assert receipt["source"]==receipt["source_after"]and receipt["before"]==receipt["after"]
(generated/("material-skin-actual-native-r95-"+("-".join(requested))+".json")).write_text(json.dumps(receipt,indent=2),encoding="utf8");print("[DEBUG] PASS",json.dumps(receipt),flush=True)
