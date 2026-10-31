"""🏗️ Author the single attested ramp stringer with exact ordered native material ownership."""
import copy,hashlib,importlib,json,math,sys,time
from fractions import Fraction as Q
from pathlib import Path
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));ticket=Path(__file__).parent;out=ticket/'🗑️generated'
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');F=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🎨️finishes.🐍️')
source=api.load_model();capture=json.loads((out/'whole-owned-capture-r92.json').read_text(encoding='utf8'));started=time.monotonic();case=json.loads((root/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🛝️stringer.json').read_text(encoding='utf8'))
def check():
    """🛑 Respect cancellation between bounded source geometry operations."""
    if(out/'ramp-stringer-authoring.cancel').exists():raise RuntimeError('Stringer authoring cancellation requested')

def bounds(part):
    """📦 Preserve exact-coordinate bounds for conservative candidate exclusion."""
    exact=getattr(part,'_exact_mesh',None)
    v=exact['vertices'] if exact is not None else part['mesh']['vertices'] if 'mesh' in part else api.part_vertices(part)
    return [min(p[k] for p in v) for k in range(3)]+[max(p[k] for p in v) for k in range(3)]

def intersects(a,b):
    """🔍 Exclude only separated bounding intervals, without a geometry tolerance."""
    return all(a[k]<=b[k+3] and b[k]<=a[k+3] for k in range(3))

def cross(a,b):
    """📐 Retain exact source-plane coefficients."""
    return [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]

def planes(part):
    """🧭 Derive native outward half-spaces from three original face vertices."""
    exact=getattr(part,'_exact_mesh',None)
    v=[[Q(c) for c in p] for p in (exact['vertices'] if exact is not None else api.part_vertices(part))];result=[]
    for face in (exact['faces'] if exact is not None else api.part_faces(part)):
        a=v[face[0]]
        for i in range(1,len(face)-1):
            b,c=v[face[i]],v[face[i+1]];n=cross([b[k]-a[k] for k in range(3)],[c[k]-a[k] for k in range(3)])
            if any(n):break
        else:raise ValueError('A source cutter has no positive face')
        result.append({'normal':n,'offset':sum(n[k]*a[k] for k in range(3)),'points':[a,b,c]})
    return result

def convex_cells(record):
    """🕳️ Preserve holes and nonconvex outlines without replacing the source body by its hull."""
    from shapely import constrained_delaunay_triangles
    from shapely.geometry import Polygon
    p=record['geometry']
    if any(k in p for k in ('mesh','section','top_outline')):return [p]
    if record['class']=='IfcColumn' and p.get('profile',{}).get('shape')=='circle':
        axis=api._find(source,record['tag'])['geometry']['axis'];profile=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧊️profiles.🐍️').circle_envelope(p['radius'],1e-5,2e-9,256)
        ring=[[axis[0]+v[0],axis[1]+v[1]] for v in profile['outline']]
        return [{'outline':ring,'z0':p['z0'],'z1':p['z1']}]
    polygon=Polygon(p['outline'],p.get('holes',()))
    if not polygon.is_valid:raise ValueError('Source cutter polygon is invalid: '+record['tag'])
    if not p.get('holes') and polygon.equals(polygon.convex_hull):return [p]
    original={}
    for ring in [p['outline'],*p.get('holes',())]:
        for point in ring:
            key=tuple(float(v) for v in point)
            if key in original and original[key]!=point:raise ValueError('Distinct source cutter vertices collapse before triangulation')
            original[key]=point
    return [{'outline':[original[tuple(v)] for v in list(t.exterior.coords)[:-1]],'z0':p['z0'],'z1':p['z1']} for t in constrained_delaunay_triangles(polygon).geoms]

def subtract(states,cutter):
    """✂️ Keep exact outside branches and discard only the actual convex cutter interior."""
    result=[];ps=planes(cutter);cb=bounds(cutter)
    for state in states:
        if not intersects(bounds(state),cb):result.append(state);continue
        if F.stock_separated(state,ps):result.append(state);continue
        remaining=state
        for plane in ps:
            if remaining is None:break
            inverse={**plane,'normal':[-v for v in plane['normal']],'offset':-plane['offset']}
            outside=F.clip_stock(remaining,[inverse])
            if outside is not None:result.append(outside)
            remaining=F.clip_stock(remaining,[plane])
    return result

if __name__=='__main__':
    flight=api._ramp_geometry(source,api._find(source,'RF-1A'));vertices=[v for p in flight['parts'] for v in api.part_vertices(p)];y0=case['alignment']['y0'];y1=case['alignment']['y1'];x0=max(v[0] for v in vertices);x1=Q(x0)+Q(str(case['section']['width']));z0=max(v[2] for v in vertices if v[1]==y0);z1=max(v[2] for v in vertices if v[1]==y1);depth=Q(str(case['section']['vertical_depth']))
    base=[[Q(x0),Q(y0),Q(z0)-depth],[x1,Q(y0),Q(z0)-depth],[x1,Q(y1),Q(z1)-depth],[Q(x0),Q(y1),Q(z1)-depth]];top=[[x,y,z+depth] for x,y,z in base]
    faces=[[3,2,1,0],[4,5,6,7]]+[[i,(i+1)%4,4+(i+1)%4,4+i] for i in range(4)]
    raw={'mesh':{'vertices':base+top,'faces':faces},'material':'MAT-RC','role':'inclined_stringer_12x26'};states=[F.clip_stock(raw,[],exact=True)];raw_bounds=bounds(raw);processed=[]
    for record in capture['records']:
        check()
        if record['group']!='structure' or record['class'] not in ('IfcColumn','IfcBeam','IfcSlab','IfcFooting'):continue
        if not intersects(raw_bounds,bounds(record['geometry'])):continue
        for cell in convex_cells(record):states=subtract(states,cell)
        processed.append({'tag':record['tag'],'index':record['index']});print('[DEBUG] stringer cutter '+record['tag']+' parts='+str(len(states)),flush=True)
    parts=[];independent=0.
    from scipy.spatial import ConvexHull
    for state in states:
        check();published=F.publish_stock(state);p={'mesh':published['mesh'],'material':'MAT-RC','role':'inclined_stringer_12x26'}
        native=api._polyhedron_part(p['mesh']['vertices'],p['mesh']['faces']);hull=ConvexHull(p['mesh']['vertices']);volume=api._polyhedron_volume(native)
        if volume<=0 or abs(volume-hull.volume)>1e-9:raise ValueError('Stringer cell is not an independent positive convex native solid')
        independent+=hull.volume;parts.append(p)
    report={'tag':case['tag'],'parts':parts,'processed_cutters':processed,'raw_volume':float((Q(y1)-Q(y0))*Q(str(case['section']['width']))*depth),'volume':independent,'source':hashlib.sha256(Path(api.__file__).with_name('🔣️.json').read_bytes()).hexdigest(),'source_capture':capture['source'],'producers':{str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in [Path(api.__file__),Path(F.__file__)]},'seconds':time.monotonic()-started,'qualification':['Initial captured columns/slabs/beams require final current-source reconciliation','Circular shaft reservation uses adaptive circumscribed domain with maximum radial deviation below declared IFC10micrometre precision; original analytic shaft is unchanged','Nominal12x26 section attested; vertical-depth convention and alignment inferred','RF1A structural plate is separate and terminates at the same current X face; no duplicate top6cm']}
    (out/'ramp-stringer-candidate-r92.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in report.items() if k not in ('parts','processed_cutters','qualification','producers')}),flush=True)
