"""🧪 Independently certify full native-cell material shadows with exact third-party rational geometry."""
import hashlib,json,math,sys,time
from pathlib import Path
from fractions import Fraction as F
from rene.exact import Point,Contour,Polygon,Empty
import importlib.metadata
pins=Path(__file__).with_name("material-skin-oracle-requirements-r95.txt").read_text(encoding="utf8").splitlines()
for pin in pins:
    name,version=pin.split("==");assert importlib.metadata.version(name)==version,(name,version)
sys.dont_write_bytecode=True;root=Path.cwd();ticket=Path(__file__).parent;source=root/"🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json";raw=source.read_bytes();model=json.loads(raw);e=next(e for e in model["elements"]if e["tag"]=="FIN-PILOTIS-GRAVEL");started=time.monotonic();last=[-1];output_name=sys.argv[1]if len(sys.argv)>1 else "material-skin-exact-oracle-r95.json"
physical=lambda e:{k:e.get(k)for k in ("guid","class","storey","material","geometry")}
physical_hash=hashlib.sha256(json.dumps(physical(e),sort_keys=True,separators=(",",":")).encode()).hexdigest()
def progress(message):
    """📣 Publish progress without discarding exact arithmetic or admitting through a light path."""
    elapsed=int(time.monotonic()-started)
    if elapsed//10!=last[0]:last[0]=elapsed//10;print("[DEBUG]",elapsed,message,flush=True)
def simplify(ring):
    """📐 Remove only exact redundant collinear continuation vertices."""
    out=[]
    for p in ring:
        if not out or out[-1]!=p:out.append(p)
    if len(out)>1 and out[0]==out[-1]:out.pop()
    changed=True
    while changed and len(out)>3:
        changed=False
        for i,p in enumerate(out):
            a,b=out[i-1],out[(i+1)%len(out)]
            if (p[0]-a[0])*(b[1]-p[1])-(p[1]-a[1])*(b[0]-p[0])==0:
                out.pop(i);changed=True;break
    return out
def polygon(ring):
    """🧩 Build an exact oracle polygon, ignoring only projections with exactly zero area."""
    ring=simplify(ring);area=sum(a[0]*b[1]-a[1]*b[0]for a,b in zip(ring,ring[1:]+ring[:1]))
    if len(ring)<3 or not area:return None
    if area<0:ring.reverse()
    return Polygon(Contour([Point(x,y)for x,y in ring]),[])
def fold(values,label):
    """🪢 Independently overlay exact third-party sets by a bounded balanced reduction."""
    while len(values)>1:
        progress((label,len(values)))
        values=[values[i]|values[i+1]if i+1<len(values)else values[i]for i in range(0,len(values),2)]
    return values[0]if values else Empty()
def extract(contour):
    """📍 Transfer oracle rationals without intermediate binary64 arithmetic."""
    return simplify([(F(int(p.x.numerator),int(p.x.denominator)),F(int(p.y.numerator),int(p.y.denominator)))for p in contour.vertices])
def canonical(ring):
    """🪪 Canonicalize exact complete boundary cycles independently of orientation or start index."""
    ring=simplify(ring);forward=tuple(ring);reverse=tuple(reversed(ring))
    return min(tuple(seq[i:]+seq[:i])for seq in (forward,reverse)for i,p in enumerate(seq)if p==min(seq))
report={"scope":"Single-product actual source geometry oracle; never a whole-model freshness certificate","physical_input_sha256":physical_hash,"oracle":"rene.exact complete-cell face projections; no snapping, hull, positive-area cutoff or runtime dependency","version":importlib.metadata.version("rene"),"pins":pins,"source":hashlib.sha256(raw).hexdigest(),"cells":len(e["geometry"]["parts"]),"views":{}}
for name,turns in {"sw":0,"se":3,"ne":2,"nw":1}.items():
    c,s=round(math.cos(math.pi/2*turns)),round(math.sin(math.pi/2*turns));matrix=((c-s,c+s),(-s-c,c-s),(0,2))
    shadows=[]
    for index,part in enumerate(e["geometry"]["parts"]):
        mesh=part["mesh"];vertices=[tuple(sum(F(p[i])*matrix[i][j]for i in range(3))for j in range(2))for p in mesh["vertices"]];faces=[polygon([vertices[i]for i in face])for face in mesh["faces"]];shadow=fold([f for f in faces if f is not None],name+" cell")
        shadows.append((sum(float(p[0])for p in vertices)/len(vertices),shadow));progress((name,"cells",index))
    result=fold([shape for x,shape in sorted(shadows,key=lambda r:r[0])],name+" global");polys=[result]if isinstance(result,Polygon)else list(result.polygons)
    exact=[{"outer":extract(p.border),"holes":[extract(h)for h in p.holes]}for p in polys]
    signature=repr(sorted((canonical(r["outer"]),tuple(sorted(canonical(h)for h in r["holes"])))for r in exact))
    report["views"][name]={"regions":[{"rings":[[[float(v)for v in p]for p in ring]for ring in [r["outer"],*r["holes"]]]}for r in exact],"components":len(exact),"holes":sum(len(r["holes"])for r in exact),"exact_boundary_sha256":hashlib.sha256(signature.encode()).hexdigest()}
    (ticket/"🗑️generated"/output_name).write_text(json.dumps(report,indent=2),encoding="utf8");print("[DEBUG] certified",name,report["views"][name]["components"],report["views"][name]["holes"],flush=True)
report["seconds"]=time.monotonic()-started;report["source_after"]=hashlib.sha256(source.read_bytes()).hexdigest();current=json.loads(source.read_bytes());current_element=next(obj for obj in current["elements"]if obj["tag"]==e["tag"]);report["physical_input_after_sha256"]=hashlib.sha256(json.dumps(physical(current_element),sort_keys=True,separators=(",",":")).encode()).hexdigest();assert report["physical_input_sha256"]==report["physical_input_after_sha256"]
(ticket/"🗑️generated"/output_name).write_text(json.dumps(report,indent=2),encoding="utf8");print("[DEBUG] complete",report["seconds"],flush=True)
