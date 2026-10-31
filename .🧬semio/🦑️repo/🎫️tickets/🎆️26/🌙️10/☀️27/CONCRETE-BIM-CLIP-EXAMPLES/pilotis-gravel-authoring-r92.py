"""🌿️ Rebuild a bounded exterior wearing surface from the sourced local terrain anchor and actual reservations."""
import importlib,json,math,sys,time
from collections import deque
from pathlib import Path
from shapely import constrained_delaunay_triangles,union_all
from shapely.geometry import Polygon
from shapely.geometry.polygon import orient
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));ticket=Path(__file__).parent;out=ticket/'🗑️generated';api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');profiles=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧊️profiles.🐍️');m=api.load_model();case=json.loads((Path(api.__file__).parent/'🧪️tests/🌿️gravel.json').read_text(encoding='utf8'));top=case['reference_elevation'];bottom=top-case['depth'];started=time.monotonic();masks=[];rows=[]
for obj in m['elements']:
    if obj['class']=='IfcSlab' and obj['storey']=='ground' and obj['tag'].startswith('SL-GROUND'):
        masks.append(Polygon(api._expand(m,obj['geometry']['outline'])));rows.append({'tag':obj['tag'],'scope':'Enclosed ground-floor programme; exterior wearing is not asserted below indoor floors'})
    elif obj['class']=='IfcWall' and obj['storey']=='ground':
        path=api._wall_path(m,obj);half=api._wall_thickness(m,obj)/2
        for a,b in zip(path,path[1:]):
            length=math.dist(a,b);normal=[-(b[1]-a[1])*half/length,(b[0]-a[0])*half/length];masks.append(Polygon([[p[k]+side*normal[k] for k in (0,1)] for p,side in ((a,-1),(b,-1),(b,1),(a,1))]))
    elif obj['class']=='IfcColumn':
        for p in api.column_geometry(m,obj)['parts']:
            if min(p['z1'],top)<=max(p['z0'],bottom):continue
            if p.get('profile',{}).get('shape')=='circle':
                radius=p['profile']['diameter']['value']/2;center=[(min(v[k] for v in p['outline'])+max(v[k] for v in p['outline']))/2 for k in range(2)];envelope=profiles.circle_envelope(radius,case['maximum_circle_reservation'],2e-9,96);ring=[[center[k]+v[k] for k in (0,1)] for v in envelope['outline']];masks.append(Polygon(ring));rows.append({'tag':obj['tag'],'scope':'Actual terrain-band analytic circle with inferred bounded conservative reserve','radius':radius,'sides':envelope['sides'],'maximum_deviation':envelope['maximum_deviation']})
            else:masks.extend(Polygon(ring) for ring in api.part_band_footprints(p,bottom,top))
    elif obj['class']=='IfcFooting':
        for p in api._solid_geometry(m,obj)['parts']:
            if min(p['z1'],top)>max(p['z0'],bottom):masks.extend(Polygon(ring) for ring in api.part_band_footprints(p,bottom,top))
stock=Polygon(case['stock']);region=stock.difference(union_all(masks));assert region.is_valid and region.area>0;triangles=list(constrained_delaunay_triangles(region).geoms);polys={};edges={};queue=deque()
for i,p in enumerate(triangles):
    if p.area<=0 or not region.covers(p):raise ValueError('A terrain cell is not positive material inside the true reserved domain')
    polys[i]=p

def keys(poly):
    """🧵 Retain exact common boundary edges for conservative convex unions."""
    ring=list(poly.exterior.coords)
    return [tuple(sorted((tuple(a),tuple(b)))) for a,b in zip(ring,ring[1:])]

def add(index):
    """🔗 Queue only actual shared original triangulation edges."""
    for edge in keys(polys[index]):
        owners=edges.setdefault(edge,set());owners.add(index)
        if len(owners)==2:queue.append(tuple(owners))
for index in polys:add(index)
serial=len(polys)
while queue:
    a,b=queue.popleft()
    if a not in polys or b not in polys:continue
    joined=polys[a].union(polys[b])
    if joined.geom_type!='Polygon' or joined.interiors or not joined.equals(joined.convex_hull):continue
    for old in (a,b):
        for edge in keys(polys[old]):edges[edge].discard(old)
        del polys[old]
    polys[serial]=joined;add(serial);serial+=1
parts=[];volumes=[]
for polygon in polys.values():
    ring=[list(v) for v in list(orient(polygon,1).exterior.coords)[:-1]];p={'outline':ring,'z0':-case['depth'],'z1':0.,'material':'MAT-GRAVEL','role':'wearing'};vertices=api.part_vertices(p);faces=api.part_faces(p)
    for f in faces:
        for turn in range(len(f)):
            a,b,c=[vertices[f[(turn+i)%len(f)]] for i in range(3)];u=[b[k]-a[k] for k in range(3)];v=[c[k]-a[k] for k in range(3)];normal=[u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]]
            if any(normal):f[:]=f[turn:]+f[:turn];break
    native=api._polyhedron_part(vertices,faces);value=api._polyhedron_volume(native)
    if value<=0:raise ValueError('A terrain cell is not native positive material')
    volumes.append(value);parts.append(p)
expected=region.area*case['depth'];actual=math.fsum(volumes);assert math.isclose(expected,actual,rel_tol=1e-10,abs_tol=1e-10);assert math.isclose(math.fsum(Polygon(p['outline']).area for p in parts),region.area,rel_tol=1e-10,abs_tol=1e-9)
report={'parts':parts,'base_level':top,'area':region.area,'volume':actual,'minimum_cell_volume':min(volumes),'initial_triangles':len(triangles),'reservations':rows,'seconds':time.monotonic()-started,'qualification':['Annex6 two transverse section terrain markers anchor local exterior height−0.15; uniform19x21.5m extension is inferred','40mm depth, perimeter, grading and grain/subgrade/drainage unmeasured','Ground-floor slab and wall programmes exclude exterior wearing under enclosed interiors','All column/footing material reservations use actual new vertical band; analytic circles use a conservative maximum0.1mm outward construction reserve','No positive cell discarded; only geometrically identical convex unions merge shared triangulation edges']};(out/'pilotis-gravel-candidate-r92.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in report.items() if k not in ('parts','reservations')}))
