"""🔬 Isolate actual published footprint seams against independent GEOS boundaries."""
import importlib,json,sys,time,math
from pathlib import Path
from shapely.geometry import Polygon,LineString,Point
from shapely.ops import unary_union
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));ticket=Path(__file__).parent
U=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🧩️regions.🐍️')
source=json.loads((ticket/'🗑️generated/ramp-plan-union-green-r93.json').read_text(encoding='utf8'));rows=[]
for row in source['actual']:
    if row['flight'] not in ('RF-1B','RF-2B'):continue
    polygons=row['footprints'];expected=unary_union([Polygon(p) for p in polygons]);regions=U.polygon_union(polygons);items=[]
    for index,region in enumerate(regions):
        body=Polygon(region['outer'],region['holes']);ring=LineString(region['outer']+[region['outer'][0]])
        items.append({'index':index,'area':body.area,'length':body.length,'valid':body.is_valid,'outer':region['outer'],'holes':region['holes'],'outside_expected_length':ring.difference(expected.boundary).length,'hausdorff':ring.hausdorff_distance(expected.boundary),'distance':ring.distance(expected.boundary)})
    rows.append({'flight':row['flight'],'expected_type':expected.geom_type,'expected_components':len(expected.geoms) if hasattr(expected,'geoms')else 1,'expected_holes':sum(len(p.interiors) for p in expected.geoms) if hasattr(expected,'geoms')else len(expected.interiors),'expected_length':expected.length,'actual_length':sum(i['length'] for i in items),'items':items})
    print('[DEBUG] '+row['flight']+' '+json.dumps({k:v for k,v in rows[-1].items() if k!='items'}),flush=True)
(ticket/'🗑️generated/ramp-union-edge-diagnostic-r93.json').write_text(json.dumps({'capture_identity':source['before'],'rows':rows},indent=2),encoding='utf8')
