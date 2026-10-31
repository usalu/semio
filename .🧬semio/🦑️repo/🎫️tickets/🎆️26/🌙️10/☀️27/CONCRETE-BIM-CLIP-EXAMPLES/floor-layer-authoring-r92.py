"""🪵 Author explicit first-storey flooring from owned structural caps and registered room outlines."""
import importlib,json,sys,copy
from pathlib import Path
sys.path.insert(0,str(Path.cwd()));a=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');an=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🐍️');m=a.load_model();sl=a._slab_geometry(m,a._find(m,'SL-1'));support=[]
from shapely.geometry import Polygon
from shapely.ops import unary_union
from shapely import constrained_delaunay_triangles
for p in sl.get('full_parts',sl['parts']):
 for ring in a.part_regions(p,p["z1"]):
  support.append((Polygon(ring),p['z1']))
support_bands={z:unary_union([p for p,h in support if h==z]) for z in sorted({h for p,h in support})};support=unary_union([p for p,h in support])
cutters=[]
for e in m['elements']:
 if e['class']=='IfcColumn':
  for p in a.column_geometry(m,e)['parts']:
   if p['z0']<3.17-1e-8 and p['z1']>3.14+1e-8:cutters.append({'outline':a.part_footprint(p),'z0':3.14,'z1':3.17})
for e in m['elements']:
 if e['class']=='IfcOpeningElement' and e['geometry'].get('slab')=='SL-1':cutters.append({'outline':a._expand(m,e['geometry']['outline']),'z0':3.14,'z1':3.17})
support=support.difference(unary_union([Polygon(p["outline"]) for p in cutters]));remaining=support;out=[]

def convex_regions(region):
 """🧩 Merge adjacent triangles only when their actual union remains convex."""
 from collections import deque
 polys={i:p for i,p in enumerate(constrained_delaunay_triangles(region).geoms) if p.area>1e-10 and region.covers(p)};edges={};queue=deque();serial=max(polys,default=-1)+1
 def keys(poly):
  ring=list(poly.exterior.coords)
  return [tuple(sorted((tuple(x),tuple(y)))) for x,y in zip(ring,ring[1:])]
 def add(i):
  for edge in keys(polys[i]):
   owners=edges.setdefault(edge,set());owners.add(i)
   if len(owners)==2:queue.append(tuple(owners))
 for i in polys:add(i)
 while queue:
  i,j=queue.popleft()
  if i not in polys or j not in polys:continue
  joined=polys[i].union(polys[j])
  if joined.geom_type!='Polygon' or joined.interiors or joined.convex_hull.difference(joined).area>1e-14:continue
  for old in (i,j):
   for edge in keys(polys[old]):edges[edge].discard(old)
   del polys[old]
  polys[serial]=joined;add(serial);serial+=1
 assert abs(sum(p.area for p in polys.values())-region.area)<1e-7,(sum(p.area for p in polys.values()),region.area)
 return list(polys.values())

def assigned(tag,rings,material,existing=False):
 global remaining
 region=remaining.intersection(unary_union([Polygon(r) for r in rings]))
 if region.is_empty or region.area<1e-10:return
 remaining=remaining.difference(region)
 zone=[{'outline':[list(q) for q in t.exterior.coords[:-1]],'z0':z,'z1':3.17} for z,band in support_bands.items() for t in convex_regions(region.intersection(band)) if t.area>1e-10]
 parts=[]
 for p in zone:
  cap=round(p['z0']-3.17,9);ring=p['outline']
  if material=='MAT-PARQUET':
   parts.extend([{'outline':ring,'z0':-.028,'z1':0.,'role':'wearing','material':material},{'outline':ring,'z0':-.03,'z1':-.028,'role':'bonding','material':'MAT-BITUMEN'}])
   if cap<-.03-1e-9:parts.append({'outline':ring,'z0':cap,'z1':-.03,'role':'bedding','material':'MAT-MORTAR-PUTTY'})
  elif material=='MAT-PAVING':
   low=-.04 if cap<=-.04 else -.015;mat=material if cap<=-.04 else 'MAT-PAVING-UNRESOLVED'
   parts.extend([{'outline':ring,'z0':cap,'z1':low,'role':'bedding','material':'MAT-MORTAR-PUTTY'},{'outline':ring,'z0':low,'z1':0.,'role':'wearing','material':mat}])
  else:parts.extend([{'outline':ring,'z0':cap,'z1':-.015,'role':'planting_substrate' if material=='MAT-TURF' else 'bedding','material':'MAT-PLANTING-SUBSTRATE' if material=='MAT-TURF' else 'MAT-MORTAR-PUTTY'},{'outline':ring,'z0':-.015,'z1':0.,'role':'wearing','material':material}])
 out.append({'tag':tag,'material':material,'parts':parts,'existing':existing,'area':sum(a._area(p['outline']) for p in zone)})
for tag,material in [('FIN-TERRACE-PAVING','MAT-PAVING'),('FIN-TERRACE-GRASS','MAT-TURF'),('FIN-KITCHEN-PATIO','MAT-PAVING-UNRESOLVED')]:
 raw=a._find(m,tag);g=a.solid_geometry(tag,m);assigned(tag,[p['outline'] for p in g['parts']],material,True)
guest=a._expand(m,a._find(m,'SP-GUEST')['outline']);assigned('FIN-GUEST-TILE',[[[2.54,10.219],[3.46,10.219],[3.46,11.588],[2.54,11.588]]],'MAT-GRES-OCHRE',True);assigned('FIN-GUEST-PARQUET',[guest],'MAT-PARQUET',True)
for s in m['spaces']:
 if s['storey']!='first' or s['tag']=='SP-GUEST':continue
 material={'SP-SALON':'MAT-GRES-OCHRE','SP-KITCHEN':'MAT-GRES-OCHRE','SP-PANTRY':'MAT-GRES-UNRESOLVED','SP-FIRST-HALL':'MAT-GRES-WHITE','SP-SHARED-BATH':'MAT-GRES-OCHRE','SP-WC-FIRST-WEST':'MAT-GRES-OCHRE','SP-WC-FIRST-EAST':'MAT-GRES-OCHRE','SP-PARENT-BATH':'MAT-GRES-WHITE','SP-SON':'MAT-PARQUET','SP-PARENTS':'MAT-PARQUET','SP-BOUDOIR':'MAT-PARQUET'}.get(s['tag'],'MAT-FLOOR-FINISH')
 assigned('FIN-'+s['tag'][3:],[a._expand(m,s['outline'])],material)
if not remaining.is_empty:
 remaining=[{'outline':[list(q) for q in t.exterior.coords[:-1]],'z0':z,'z1':3.17} for z,band in support_bands.items() for t in convex_regions(remaining.intersection(band)) if t.area>1e-10]
 out.append({'tag':'FIN-FIRST-THRESHOLDS','material':'MAT-FLOOR-FINISH','existing':False,'area':sum(a._area(p['outline']) for p in remaining),'parts':[{'outline':p['outline'],'z0':round(p['z0']-3.17,9) if role=='bedding' else lo,'z1':hi,'role':role,'material':mat} for lo,hi,role,mat in [(-.03,-.015,'bedding','MAT-MORTAR-PUTTY'),(-.015,0.,'wearing','MAT-FLOOR-FINISH')] for p in remaining]})
import importlib.util
recipe=Path(__file__).with_name("floor-raised-beam-authoring-r92.py");spec=importlib.util.spec_from_file_location("floor_raised_beam_author",recipe);raised=importlib.util.module_from_spec(spec);spec.loader.exec_module(raised);trimmed={r["tag"]:r for r in raised.trim_floor_layers(m,out)}
for row in out:row["parts"]=trimmed[row["tag"]]["parts"];row["area"]=sum(Polygon(p["outline"]).area for p in row["parts"] if p["role"]=="wearing")
for tag in json.loads((Path(a.__file__).parent/"🧪️tests/🪵️raised-beams.json").read_text())["beam_tags"]:
 for part in a._solid_geometry(m,a._find(m,tag))["parts"]:
  if 3.08-1e-8<=part["z1"]<=3.17+1e-8:support_bands[part["z1"]]=support_bands.get(part["z1"],Polygon()).union(unary_union([Polygon(r) for r in a.part_regions(part,part["z1"])]).intersection(support))
result={'source_products':len(m['elements']),'support_area':support.area,'floors':out,'limits':['First-floor finish material assignment outside retained guest/terrace evidence is explicitly inferred.','Threshold and residual structural-cap bands have unmeasured wearing specification; geometric coverage does not claim a historical tile pattern.']}
for row in out:
 for part in row['parts']:
  if part['role'] in ('bedding','bonding') and not(part['role']=='bonding' and Polygon(part['outline']).difference(support_bands.get(round(3.17+part['z0'],9),Polygon())).area>1e-7):
   z=round(3.17+part['z0'],9);below=unary_union([Polygon(p["outline"]) for r in out for p in r["parts"] if abs(p["z1"]-part["z0"])<1e-9]);contact=support_bands.get(z,Polygon()).union(below);outside=Polygon(part['outline']).difference(contact).area
   assert outside<1e-7,(row['tag'],outside,part['outline'])
p=Path(__file__).parent/'🗑️generated/floor-layer-authoring-r92.json';p.write_text(json.dumps(result),encoding='utf8');print('[DEBUG] '+json.dumps({'support_area':result['support_area'],'floors':[{'tag':r['tag'],'area':r['area'],'parts':len(r['parts'])} for r in out]}),flush=True)
