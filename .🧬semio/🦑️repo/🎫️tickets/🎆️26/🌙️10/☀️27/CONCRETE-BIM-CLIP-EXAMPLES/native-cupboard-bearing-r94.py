"""🪵 Diagnose actual cupboard front and foot support against current native floor/wall material."""
import json
from pathlib import Path
from shapely.geometry import Polygon,Point
B=Path(__file__).parent;O=B/'🗑️generated/native-r92';g=json.loads((O/'skirting-geometry.json').read_text())['geometry'];r=json.loads((O/'skirting-candidate.json').read_text());tag='FIX-SERVANT-BASIN-CABINET-1';board=next(x for x in r['elements'] if x['tag']=='SKIRT-'+tag+'-front');host=next(x for x in g['details']+g['furnishings']+g['services'] if x['tag']==tag)
verts=board['geometry']['parts'][0]['mesh']['vertices'];foot=Polygon([(x,y) for x,y,z in verts if z==0]).convex_hull
rows=[]
for group in ('details','slabs','walls','screens','beams','pilotis'):
 for obj in g[group]:
  for index,p in enumerate(obj['parts']):
   if 'outline' not in p:continue
   shape=Polygon(p['outline'],p.get('holes',[]));area=shape.intersection(foot).area
   if area>0 or shape.distance(foot)<.25:rows.append({'group':group,'tag':obj['tag'],'part':index,'z0':p['z0'],'z1':p['z1'],'role':p.get('role'),'intersection_area':area,'distance':shape.distance(foot)})
feet=[]
for p in host['parts']:
 if p['z0']!=0:continue
 shape=Polygon(p['outline']);supports=[]
 for obj in g['details']+g['slabs']:
  for index,q in enumerate(obj['parts']):
   if 'outline' not in q or q['z1']!=0:continue
   area=shape.intersection(Polygon(q['outline'],q.get('holes',[]))).area
   if area>0:supports.append({'tag':obj['tag'],'part':index,'area':area})
 feet.append({'outline':p['outline'],'supports':supports})
report={'board':board['tag'],'floor_footprint':list(foot.exterior.coords),'nearby_actual_parts':rows,'host_feet':feet};(O/'cupboard-bearing-r94.json').write_text(json.dumps(report,indent=2),encoding='utf8');print('[DEBUG] cupboard bearing '+json.dumps(report),flush=True)
