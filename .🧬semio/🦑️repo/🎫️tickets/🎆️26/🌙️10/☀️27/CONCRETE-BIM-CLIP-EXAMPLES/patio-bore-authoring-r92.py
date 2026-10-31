"""🚰️ Reconcile a bounded inferred patio inlet with its actual floor and existing native opening."""
import copy,hashlib,importlib,json,math,sys,time
from collections import defaultdict
from fractions import Fraction as Q
from pathlib import Path
from shapely import constrained_delaunay_triangles,union_all
from shapely.geometry import Polygon,box
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));ticket=Path(__file__).parent;out=ticket/'🗑️generated';api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');profiles=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧊️profiles.🐍️');m=api.load_model();start=time.monotonic();case=json.loads((Path(api.__file__).parent/'🧪️tests/🚰️patio.json').read_text(encoding='utf8'));collector=api._find(m,case['collector']);opening=api._find(m,case['opening']);floor=api._find(m,case['covering']);base=api._z(m,floor['geometry']['base_level']);old=Polygon(opening['geometry']['outline']);sleeve=[p for p in collector['geometry']['parts'] if p['role']=='inlet_sleeve'];points=[v for p in sleeve for v in p['outline']];center=[(min(v[k] for v in points)+max(v[k] for v in points))/2 for k in range(2)];delta=[Q(str(case['axis'][k]))-Q(str(center[k])) for k in range(2)];moved=[]
for raw in collector['geometry']['parts']:
    p=copy.deepcopy(raw);p['outline']=[[float(Q(str(v[k]))+delta[k]) for k in range(2)] for v in p['outline']];moved.append(p)
radius=max(math.dist(v,center) for v in points);envelope=profiles.circle_envelope(radius+case['installation_fit'],1e-5,0.,256);ring=[[float(Q(str(case['axis'][k]))+Q(str(v[k]))) for k in range(2)] for v in envelope['outline']];hole=Polygon(ring);full=union_all([Polygon(p['outline']) for p in moved]).convex_hull
reference=copy.deepcopy(m);reference['elements']=[o for o in reference['elements'] if o['tag']!=case['opening']];slab=copy.deepcopy(api._find(reference,case['slab']));slab['geometry']['column_reserves']=[];slab['geometry']['finish_voids']=[];raw_slab=api._slab_geometry(reference,slab);cap=base+min(p['z0'] for p in floor['geometry']['parts']);support=union_all([Polygon(p['outline']) for p in raw_slab['parts'] if math.isclose(p['z1'],cap,abs_tol=1e-10)]);program=union_all([Polygon(p['outline']) for p in json.loads((out/'patio-original-stock-r92.json').read_text(encoding='utf8'))['record']['geometry']['parts']]);window=box(old.bounds[0]-.002,old.bounds[1]-.002,old.bounds[2]+.002,old.bounds[3]+.002);local_stock=support.intersection(program).intersection(window);groups=defaultdict(list)
for p in floor['geometry']['parts']:groups[(p['z0'],p['z1'],p['material'],p['role'])].append(Polygon(p['outline']))
result=[];rows=[]
for (z0,z1,material,role),polygons in groups.items():
    previous=union_all(polygons)
    patio=box(.3,6.081,2.96,8.9)
    restored=support.intersection(patio)
    added=restored.difference(previous)
    if previous.difference(restored).area>1e-10 or added.difference(old.buffer(1e-9)).area>1e-10:raise ValueError('Original patio source stock disagrees beyond the obsolete service bore: '+str((previous.difference(restored).area,added.difference(old.buffer(1e-9)).area)))
    region=restored.difference(hole);triangles=list(constrained_delaunay_triangles(region).geoms);expected=region.area*(z1-z0);actual=0.
    for t in triangles:
        if t.area<=0 or not region.covers(t):raise ValueError('Invalid constrained patio material cell')
        part={'outline':[list(v) for v in list(t.exterior.coords)[:-1]],'z0':z0,'z1':z1,'material':material,'role':role};native=api._polyhedron_part(api.part_vertices(part),api.part_faces(part));actual+=api._polyhedron_volume(native);result.append(part)
    if not math.isclose(actual,expected,rel_tol=1e-9,abs_tol=1e-12):raise ValueError('Independent patio material volume mismatch')
    rows.append({'role':role,'material':material,'before_area':previous.area,'restored_old_bore_area':restored.area-previous.area,'new_bore_removed_area':restored.intersection(hole).area,'expected_volume':expected,'actual_volume':actual,'parts':len(triangles)})
source_volume=sum(Polygon(p['outline']).area*(p['z1']-p['z0']) for p in collector['geometry']['parts']);new_volume=sum(Polygon(p['outline']).area*(p['z1']-p['z0']) for p in moved);assert math.isclose(source_volume,new_volume,rel_tol=1e-11,abs_tol=1e-14)
wearing=union_all([Polygon(p['outline']) for p in result if p['role']=='wearing']);assert wearing.union(hole).buffer(1e-10).covers(full),'Complete flange must bear inside actual patio finish'
report={'collector_parts':moved,'axis':case['axis'],'opening_outline':ring,'floor_parts':result,'groups':rows,'source_collector_volume':source_volume,'new_collector_volume':new_volume,'radius':radius,'fit':case['installation_fit'],'envelope':{k:v for k,v in envelope.items() if k!='outline'},'qualification':['Original patio collection function is attested, exact inlet position and dimensions are not','Only inferred current inlet placement moves into actual patio; shape and material conserved','Existing native bore is aligned and local0.1mm installation fit is inferred','Original rectangular patio programme reconstructed at source facade/partition faces(.3..2.96,6.081..8.9) and intersected with actual raw structural support; only the obsolete bore is added relative to prior occupied floor, independent GEOS difference checked'],'seconds':time.monotonic()-start};(out/'patio-bore-candidate-r92.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in report.items() if k not in ('collector_parts','floor_parts','opening_outline')}))
                                                                                     