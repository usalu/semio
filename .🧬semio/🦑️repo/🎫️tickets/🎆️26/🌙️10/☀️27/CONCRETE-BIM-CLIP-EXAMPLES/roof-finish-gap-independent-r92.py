"""🏠 Classify exact roof finish coverage residuals without changing source."""
import hashlib,importlib,json,sys,time
from pathlib import Path
from shapely.geometry import Polygon,mapping,shape
from shapely.ops import unary_union
from shapely.strtree import STRtree
from shapely import set_precision
T=Path(__file__).parent;root=next(p for p in T.parents if (p/'AGENTS.md').is_file() and (p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');path=Path(api.__file__).with_name('🔣️.json');raw=path.read_bytes();m=json.loads(raw);source=hashlib.sha256(raw).hexdigest();producer=hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest();case=json.loads(Path(api.__file__).parent.joinpath('🧪️tests/🪵️floors.json').read_text(encoding='utf8'))['remaining_storeys'];started=time.monotonic();z=case['roof_mid'];slabs=[]
for tag in case['roof_slabs']:
 print('[DEBUG] resolving narrow roof '+tag,flush=True);slabs.append(api._slab_geometry(m,api._find(m,tag)))
def regions(parts,height):return [Polygon(r) for p in parts if p['z0']<height<p['z1'] for r in api.part_regions(p,height)]
weather_shapes=[p for s in slabs for p in regions(s['finish_parts'],z)];weather=unary_union(weather_shapes);owners={r['owner'] for tag in case['roof_slabs'] for r in api._find(m,tag)['geometry'].get('finish_voids',[])};owned=[]
for owner in sorted(owners):
 g=api._solid_geometry(m,api._find(m,owner));owned.extend({'owner':owner,'polygon':q} for q in regions(g['parts'],z))
covered_shapes=weather_shapes+[r['polygon'] for r in owned];covered=weather.union(unary_union([r['polygon'] for r in owned]));space=Polygon(api._expand(m,api._find(m,case['roof_room'])['outline']));supports=[Polygon(r) for s in slabs for p in s['full_parts'] for r in api.part_regions(p,p['z1'])];domain=space.intersection(unary_union(supports));missing=domain.difference(covered);precision=[]
for grid in (1e-9,1e-10,1e-11,1e-12):
 dd=unary_union([set_precision(p,grid) for p in supports]).intersection(set_precision(space,grid));cc=unary_union([set_precision(p,grid) for p in covered_shapes]);mm=dd.difference(cc);precision.append({'grid':grid,'missing_area':mm.area})
components=list(missing.geoms) if hasattr(missing,'geoms') else [missing];columns=[]
for obj in m['elements']:
 if obj['class']=='IfcColumn':
  cg=api.column_geometry(m,obj);foot=unary_union([Polygon(p['outline'],p.get('holes',[])) for p in cg['parts']]);columns.append({'tag':obj['tag'],'polygon':foot,'z':[cg['base'],cg['top']]})
rows=[]
for q in components:
 if q.is_empty:continue
 pt=q.representative_point();hits=[{'column':c['tag'],'intersection_area':q.intersection(c['polygon']).area,'column_z':c['z']} for c in columns if q.intersection(c['polygon']).area>1e-15];rows.append({'area':q.area,'bounds':q.bounds,'witness':[pt.x,pt.y,z],'geometry':mapping(q),'column_footprint_hits':hits})
tree=STRtree(covered_shapes);local=[]
for support in supports:
 q=support.intersection(space)
 if q.is_empty or q.area==0:continue
 candidates=[covered_shapes[int(i)] for i in tree.query(q)];rem=q.difference(unary_union(candidates)) if candidates else q
 if rem.area>0:local.append(rem)
local_missing=unary_union(local);after=hashlib.sha256(path.read_bytes()).hexdigest();out={'source':source,'source_after':after,'producer':producer,'producer_after':hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest(),'height':z,'raw_missing_area':missing.area,'raw_missing_geometry':mapping(missing),'components':rows,'precision_diagnostics':precision,'localized_missing_area':local_missing.area,'localized_missing_geometry':mapping(local_missing),'weather_area':weather.area,'covered_area':covered.area,'supported_domain_area':domain.area,'owners':sorted(owners),'seconds':time.monotonic()-started};(T/'🗑️generated/roof-finish-gap-independent-r92.json').write_text(json.dumps(out,ensure_ascii=False,indent=2),encoding='utf8');print(json.dumps({k:v for k,v in out.items() if k not in ('raw_missing_geometry','components','localized_missing_geometry')}),flush=True)
