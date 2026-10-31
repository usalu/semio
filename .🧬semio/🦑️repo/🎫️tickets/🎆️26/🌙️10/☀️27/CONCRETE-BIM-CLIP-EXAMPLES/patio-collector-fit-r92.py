"""🎯 Measure candidate collector fitting against the actual patio wearing footprint."""
import importlib,json,math,sys
from pathlib import Path
from shapely import union_all
from shapely.geometry import Polygon,Point
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));ticket=Path(__file__).parent;api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=api.load_model();collector=api._find(m,'COLLECTOR-KITCHEN-PATIO-9');wearing=union_all([Polygon(p['outline']) for p in api._find(m,'FIN-KITCHEN-PATIO')['geometry']['parts'] if p['role']=='wearing']);parts=[p for p in api._service(m,collector)['parts'] if p['role']=='inlet_sleeve'];shell=union_all([Polygon(p['outline']) for p in parts]).convex_hull;rows=[]
from shapely.affinity import translate
for delta in (0,.06,.10,.16,.20):
    fit=translate(shell,yoff=delta).buffer(.0001);rows.append({'delta_y':delta,'outside_actual_patio_area':fit.difference(wearing).area,'boundary_clearance':fit.boundary.distance(wearing.boundary),'fully_supported':wearing.covers(fit)})
from shapely.ops import nearest_points
old=Polygon(api._find(m,'OP-RAIN-9-SL1')['geometry']['outline']);reference=wearing.union(old);samples=[]
for x in (2.2,2.4,2.6,2.8,2.9,3.0,3.08,3.2):
    for y in (8.7,8.9,9.1,9.3):
        fit=translate(shell,xoff=x-3.08,yoff=y-8.77).buffer(.0001)
        if reference.covers(fit):samples.append({'axis':[x,y],'clearance':fit.boundary.distance(wearing.boundary),'displacement':math.dist([x,y],[3.08,8.77])})
samples.sort(key=lambda r:r['displacement'])
from PIL import Image,ImageDraw
image=Image.new('RGB',(1400,1000),'white');draw=ImageDraw.Draw(image);scale=230
point=lambda p:(80+p[0]*scale,930-(p[1]-7.5)*scale)
for p in api._find(m,'FIN-KITCHEN-PATIO')['geometry']['parts']:
    if p['role']=='wearing':draw.polygon([point(v) for v in p['outline']],fill='#cadcce',outline='#9db9a5')
for x,y,label,color in [(3.08,8.77,'Current body','#d74b39'),(2.8,8.7,'Current bore/axis','#326db0')]:
    cx,cy=point([x,y]);r=.047*scale;draw.ellipse((cx-r,cy-r,cx+r,cy+r),fill=color);draw.text((cx+15,cy-15),label,fill=color)
for r in samples[:6]:
    cx,cy=point(r['axis']);draw.ellipse((cx-4,cy-4,cx+4,cy+4),fill='#7358a5');draw.text((cx+8,cy),str(r['axis']),fill='#7358a5')
for x in range(6):
    cx,cy=point([x,7.5]);draw.text((cx,cy+12),str(x),fill='black')
for y in (8,9,10,11):
    cx,cy=point([0,y]);draw.text((cx-30,cy),str(y),fill='black')
draw.text((40,25),'Actual first-floor patio wearing footprint / source-native plan diagnostic',fill='black');image.save(ticket/'🗑️generated/patio-collector-fit-r92.png')
report={'current_geometry_center':[(shell.bounds[k]+shell.bounds[k+2])/2 for k in range(2)],'current_axis':collector['geometry']['axis'],'floor_bounds':wearing.bounds,'nearest_floor_point':list(nearest_points(Point(3.08,8.77),wearing)[1].coords)[0],'rows':rows,'fully_supported_candidate_stations':samples};(ticket/'🗑️generated/patio-collector-fit-r92.json').write_text(json.dumps(report,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps(report))
