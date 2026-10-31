"""⚖️ Audit authored first-floor finish weights independently of analytical allowances."""
import hashlib,importlib,json,sys,time
from pathlib import Path
from shapely.geometry import Polygon
from shapely.ops import unary_union
root=next(p for p in Path(__file__).resolve().parents if (p/'AGENTS.md').is_file() and (p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
prefix='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.'
api=importlib.import_module(prefix+'🧱️model.🐍️');analysis=importlib.import_module(prefix+'🧱️model.🧮️analysis.🐍️');source=Path(api.__file__).with_name('🔣️.json');raw=source.read_bytes();m=json.loads(raw);ticket=Path(__file__).parent;started=time.monotonic();materials={v['tag']:v for v in m['materials']};rows=[];parts=[];producers=[Path(api.__file__),Path(analysis.__file__)];before={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in producers}
def ringarea(r):return abs(sum(a[0]*b[1]-b[0]*a[1] for a,b in zip(r,r[1:]+r[:1])))/2
for obj in m['elements']:
 if obj['class']!='IfcCovering' or obj.get('predefined_type')!='FLOORING' or obj.get('storey')!='first':continue
 component={};footprints=[]
 for i,p in enumerate(obj['geometry']['parts']):
  assert 'mesh' not in p and 'section' not in p and 'top_outline' not in p
  poly=Polygon(p['outline'],p.get('holes',[]));assert poly.is_valid and poly.area>0
  independent=ringarea(p['outline'])-sum(ringarea(h) for h in p.get('holes',[]));assert abs(poly.area-independent)<1e-8
  h=p['z1']-p['z0'];assert h>0
  mat=p.get('material',obj.get('material'));d=materials[mat]['density'];assert d['unit']=='kN/m³'
  volume=poly.area*h;weight=volume*d['value'];key=mat
  c=component.setdefault(key,{'material':mat,'density':d,'volume_m3':0.,'weight_kN':0.});c['volume_m3']+=volume;c['weight_kN']+=weight
  footprints.append(poly);parts.append({'tag':obj['tag'],'polygon':poly,'weight_per_area':h*d['value'],'volume':volume})
 footprint=unary_union(footprints);weight=sum(c['weight_kN'] for c in component.values());rows.append({'tag':obj['tag'],'guid':obj['guid'],'parts':len(footprints),'footprint_area_m2':footprint.area,'volume_m3':sum(c['volume_m3'] for c in component.values()),'weight_kN':weight,'mean_on_own_footprint_kN_m2':weight/footprint.area,'materials':list(component.values())})
print('[DEBUG] independent17coveringweights complete',flush=True)
slab=api._slab_geometry(m,api.element('SL-1',m));slabparts=slab.get('full_parts',slab['parts']);struct=unary_union([Polygon(p['outline'],p.get('holes',[])) for p in slabparts]);covered=unary_union([p['polygon'] for p in parts]);total=sum(r['weight_kN'] for r in rows);inside=sum(p['polygon'].intersection(struct).area*p['weight_per_area'] for p in parts);outside=total-inside
geometry={'roles':{'floor':'SL-1'},'slabs':[slab]};zones=analysis._zones(api,m,geometry,'first');runtime=analysis._mean_breakdown([r for z in zones for r in z['regions']]);policy=m['loads']['finishes']['first']
gap=struct.difference(covered);column_rows=[];column_shapes=[]
for obj in m['elements']:
 if obj['class']!='IfcColumn':continue
 rings=[]
 for part in api.column_geometry(m,obj)['parts']:
  vs=api.part_vertices(part)
  if max(v[2] for v in vs)<=3.140000001 or min(v[2] for v in vs)>=3.169999999:continue
  assert 'outline' in part,obj['tag']+' requires actual occupied outline'
  rings.append(Polygon(part['outline'],part.get('holes',[])))
 if rings:
  shape=unary_union(rings);hit=shape.intersection(gap)
  if hit.area>1e-10:column_rows.append({'tag':obj['tag'],'gap_overlap_area_m2':hit.area,'column_projected_area_m2':shape.area});column_shapes.append(shape)
column_union=unary_union(column_shapes);remaining_gap=gap.difference(column_union)
record={'gap_classification':{'column_overlaps':column_rows,'column_owned_gap_area_m2':gap.intersection(column_union).area,'remaining_area_m2':remaining_gap.area,'remaining_components':[{'area_m2':g.area,'centroid':list(g.centroid.coords)[0],'bounds':list(g.bounds),'outline':list(g.exterior.coords)} for g in getattr(remaining_gap,'geoms',[remaining_gap]) if g.area>1e-9]},'scope':'Exact nominal authored geometry times assumed material unitweight; not measured historical mass','source_sha256':hashlib.sha256(raw).hexdigest(),'rows':rows,'totals':{'coverings':len(rows),'volume_m3':sum(r['volume_m3'] for r in rows),'weight_kN':total,'union_covered_area_m2':covered.area,'sum_individual_footprint_area_m2':sum(r['footprint_area_m2'] for r in rows),'mean_on_covered_union_kN_m2':total/covered.area,'analytical_structural_union_area_m2':struct.area,'weight_inside_structural_projection_kN':inside,'weight_outside_structural_projection_kN':outside,'covered_area_outside_structural_projection_m2':covered.difference(struct).area,'structural_area_without_physical_floor_m2':struct.difference(covered).area,'mean_over_analytical_structural_area_kN_m2':inside/struct.area,'uniform_policy_weight_kN':policy['value']*struct.area,'policy_minus_physical_inside_kN':policy['value']*struct.area-inside},'uncovered_components':[{'area_m2':g.area,'centroid':list(g.centroid.coords)[0],'bounds':list(g.bounds)} for g in getattr(struct.difference(covered),'geoms',[struct.difference(covered)]) if g.area>1e-9],'current_policy':policy,'runtime_breakdown':runtime,'runtime_regions':[{'tag':r['tag'],'area':r['area'],'thickness':r['thickness'],'breakdown':r['breakdown']} for z in zones for r in z['regions']],'source_after_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'producers_before':before,'producers_after':{str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in producers},'seconds':time.monotonic()-started}
assert record['source_sha256']==record['source_after_sha256'];assert record['producers_before']==record['producers_after'];(ticket/'🗑️generated/floor-load-independent-r92.json').write_text(json.dumps(record,ensure_ascii=False,indent=2),encoding='utf8');print(json.dumps(record['totals']),flush=True);print(json.dumps({'runtime_breakdown':runtime,'source':record['source_sha256'],'seconds':record['seconds']}),flush=True)
