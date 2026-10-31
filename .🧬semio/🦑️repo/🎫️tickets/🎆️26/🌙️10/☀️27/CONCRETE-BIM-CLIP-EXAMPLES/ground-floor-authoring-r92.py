"""🪵 Author completed ground wearing assemblies with explicit structural cap ownership."""
import importlib,json,sys,copy,math
from pathlib import Path
from shapely.geometry import Polygon,box
from shapely.ops import unary_union
from shapely import constrained_delaunay_triangles
sys.path.insert(0,str(Path.cwd()));a=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=a.load_model()
def convex_regions(region):
 """🧩 Merge adjacent triangles only when their actual union remains convex."""
 from collections import deque
 polys={i:p for i,p in enumerate(constrained_delaunay_triangles(region).geoms) if p.area>1e-10};edges={};queue=deque();serial=max(polys,default=-1)+1
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

rooms={s['tag']:Polygon(a._expand(m,s['outline'])) for s in m['spaces'] if s['storey']=='ground'};garage=rooms['SP-GARAGE'];mask=unary_union([p for tag,p in rooms.items() if tag!='SP-GARAGE'])
for e in m['elements']:
 if e['class']=='IfcWall' and e['storey']=='ground':
  g=a._wall_geometry(m,e);mask=mask.union(unary_union([Polygon(p['outline']) for p in g['parts'] if abs(p['z0'])<1e-8]))
mask=mask.difference(garage)
for tag in ['COVER-HOIST','COVER-COAL']:
 mask=mask.difference(Polygon(a._expand(m,a._find(m,tag)['geometry']['outline'])))
for tag in ['FIX-SERVANT-STORAGE-1-LONG','FIX-SERVANT-BASIN-CABINET-1']:
 e=a._find(m,tag);g=a.solid_geometry(tag,m);mask=mask.difference(unary_union([Polygon(p['outline']) for p in g['parts'] if 'carcass' in p.get('role','')]).envelope)
slabs=[];supported=[];occupied=[]
for tag in ['SL-GROUND','SL-GROUND-CELLAR']:
 e=a._find(m,tag);regions=[]
 for p in a._slab_geometry(m,e)['full_parts']:
  if abs(p['z1'])<1e-8:occupied.extend(Polygon(r) for r in a.part_regions(p,p['z1']))
 for raw in e['geometry']['regions']:
  poly=Polygon(a._expand(m,raw['outline']));active=poly.intersection(mask) if abs(raw['top_offset']['value'])<1e-8 else Polygon();rest=poly.difference(active)
  for region,drop in [(active,True),(rest,False)]:
   for i,p in enumerate(convex_regions(region)):
    r=copy.deepcopy(raw);r['tag']=raw['tag']+('-FIN-' if drop else '-CEM-')+str(i+1);r['outline']=[list(q) for q in p.exterior.coords[:-1]]
    if drop:r['top_offset']={'value':-.04,'unit':'m','basis':'inferred','source':'SAB187-191 wearing assembly; cap inferred below retained finished datum'}
    regions.append(r)
  if not active.is_empty:supported.append(active)
 slabs.append({'tag':tag,'regions':regions,'top_offset':max(r['top_offset']['value'] for r in regions)})
net=unary_union(occupied).intersection(mask)
for e in m['elements']:
 if e['class']=='IfcColumn':
  for p in a.column_geometry(m,e)['parts']:
   if p['z0']<0 and p['z1']>-.04:net=net.difference(Polygon(a.part_footprint(p)))
remaining=net;out=[]
def finish(tag,zone,material,wood=False,angle=None):
 global remaining
 region=remaining.intersection(zone)
 if region.is_empty or region.area<1e-8:return
 remaining=remaining.difference(region);cells=convex_regions(region);parts=[]
 for p in cells:
  ring=[list(q) for q in p.exterior.coords[:-1]]
  parts.append({'outline':ring,'z0':-.025 if wood else -.015,'z1':0.,'role':'wearing','material':material})
  if not wood:parts.append({'outline':ring,'z0':-.04,'z1':-.015,'role':'bedding','material':'MAT-MORTAR-PUTTY'})
 if wood:
  x0,y0,x1,y1=region.bounds;axis=0 if x1-x0>y1-y0 else 1;lo,hi=(x0,x1) if axis==0 else (y0,y1)
  for i in range(math.ceil((hi-lo)/.4)+1):
   q=lo+i*.4;batten=box(q,y0,q+.04,y1) if axis==0 else box(x0,q,x1,q+.04)
   for p in convex_regions(region.intersection(batten)):
    ring=[list(v) for v in p.exterior.coords[:-1]];parts.extend([{'outline':ring,'z0':-.04,'z1':-.039,'role':'inferred_bitumen_batten_bed','material':'MAT-BITUMEN'},{'outline':ring,'z0':-.039,'z1':-.025,'role':'inferred_batten','material':'MAT-OAK'}])
 out.append({'tag':tag,'material':material,'parts':parts,'wood':wood,'angle':angle,'area':region.area})
patches=[box(3.225,14.88529411764706,3.939705882352941,15.723529411764707),box(3.9676470588235295,15.723529411764707,4.722058823529411,16.533823529411766)]
for tag in ['SP-SERVANTS-1','SP-SERVANTS-2']:finish('FIN-'+tag[3:]+'-WASH',unary_union(patches).intersection(rooms[tag]),'MAT-GRES-OCHRE',angle=0)
finish('FIN-LAUNDRY',rooms['SP-LAUNDRY'],'MAT-GRES-OCHRE',angle=0)
for tag in ['SP-HALL','SP-CHAUFFEUR','SP-CHAUFFEUR-BATH','SP-WC-GROUND-SERVICE','SP-WC-CHAUFFEUR']:finish('FIN-'+tag[3:],rooms[tag],'MAT-GRES-WHITE',angle=45 if tag in ['SP-HALL','SP-CHAUFFEUR'] else 0)
for tag in ['SP-SERVANTS-1','SP-SERVANTS-2','SP-CHAUFFEUR-BEDROOM']:finish('FIN-'+tag[3:],rooms[tag],'MAT-OAK',True)
finish('FIN-GROUND-THRESHOLDS',remaining,'MAT-GRES-WHITE')
probe=copy.deepcopy(m)
for delta in slabs:
 e=a._find(probe,delta['tag']);e['geometry']['regions']=delta['regions'];e['geometry']['top_offset']['value']=delta['top_offset'];a._slab_geometry(probe,e)
for row in out:
 e={'tag':row['tag'],'guid':a.global_id(row['tag']),'class':'IfcCovering','predefined_type':'FLOORING','storey':'ground','name':{'en':row['tag'],'de':row['tag']},'metadata':{'domains':['architecture']},'geometry':{'kind':'flooring','base_level':'ground','parts':row['parts']}}
 g=a._solid_geometry(probe,e);assert all(p['z1']<=1e-9 for p in g['parts'])
result={'native_preflight':True,'slabs':slabs,'floors':out,'net_area':net.area,'limits':['Structural thickness retained; cap offset below finished datum inferred.','Oak25mm/60mm strip invoice source-attested; batten14mm/bitumen1mm/spacing400mm inferred.','Tile15mm and bedding25mm depths inferred; original tile150mm and laying directions source-attested.','Wash patch extents bounded reconstruction inferences, not measured historical outlines.']};(Path(__file__).parent/'🗑️generated/ground-floor-authoring-r92.json').write_text(json.dumps(result),encoding='utf8');print('[DEBUG] '+json.dumps({'net_area':net.area,'floors':[{'tag':r['tag'],'area':r['area'],'parts':len(r['parts'])} for r in out]}),flush=True)
                            