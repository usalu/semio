"""🚶 Audit proposed side-access WC routes against actual native occupied material."""
import hashlib,importlib,json,sys,time
from pathlib import Path
from shapely.geometry import Polygon,LineString,box
root=next(p for p in Path(__file__).resolve().parents if (p/'AGENTS.md').is_file() and (p/'package.json').is_file())
sys.path.insert(0,str(root));sys.dont_write_bytecode=True
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
source_path=Path(api.__file__).with_name('🔣️.json');before=hashlib.sha256(source_path.read_bytes()).hexdigest();m=api.load_model();started=time.monotonic()
print('[DEBUG] Capturing actual native occupied geometry for side WC routes',flush=True)
g=api.building_geometry(m);items=[]
for category in ['slabs','walls','pilotis','screens','fillings','beams','details','furnishings','foundations','railings','services','stairs']:
    items.extend((item['tag'],part) for item in g[category] for part in item.get('parts',[]))
if g.get('curtain_wall'):items.extend((g['curtain_wall']['tag'],part) for part in g['curtain_wall']['parts'])
for item in g['ramp']['ramps']:items.extend((item['tag'],part) for part in item['parts']+item['child_parts'])
routes=[{'id':'WEST','points':[[4.910661764705882,11.2],[4.03,11.2]],'portal_x':4.414705882352941},{'id':'EAST','points':[[5.81875,11.2],[6.65,11.2]],'portal_x':6.286764705882353}]
results=[]
for route in routes:
    body=LineString(route['points']).buffer(.3001,quad_segs=64);hits=[];near=[];tested=0
    for tag,part in items:
        if part['z1']<=3.20 or part['z0']>=4.97:continue
        vertices=api.part_vertices(part);xx=[v[0] for v in vertices];yy=[v[1] for v in vertices]
        if not box(min(xx),min(yy),max(xx),max(yy)).intersects(body.buffer(.5)):continue
        for ring in api.part_band_footprints(part,max(3.20,part['z0']),min(4.97,part['z1'])):
            poly=Polygon(ring);tested+=1;area=poly.intersection(body).area;distance=poly.distance(body)
            if area>0:hits.append({'tag':tag,'area_m2':area})
            if distance<.3:near.append({'tag':tag,'distance_m':distance})
    route['body_radius_m']=.3001;route['vertical_band_m']=[3.20,4.97];route['actual_obstacle_intersections']=hits;route['nearest']=sorted(near,key=lambda r:r['distance_m'])[:12];route['tested_occupied_rings']=tested;route['proposed_portal_side_margin_m']=.32-.3001;route['status']='PASS' if not hits else 'FAIL';results.append(route)
after=hashlib.sha256(source_path.read_bytes()).hexdigest();result={'source_sha256':before,'source_stable_during_run':before==after,'after_source_sha256':after,'source_products':len(m['elements']),'routes':results,'seconds':time.monotonic()-started,'limits':['Actual pre-install source obstacles; proposed WC walls/leaves/bowls are not falsely claimed to exist.','Clear side portal widths are independent candidate inputs; closed door travel is not claimed.','Native occupied-band contours checked with independent GEOS capsule intersection; no area exemption.']}
(Path(__file__).parent/'🗑️generated/wc-side-route-audit-r92.json').write_text(json.dumps(result,indent=2),encoding='utf8');print(json.dumps(result),flush=True)
