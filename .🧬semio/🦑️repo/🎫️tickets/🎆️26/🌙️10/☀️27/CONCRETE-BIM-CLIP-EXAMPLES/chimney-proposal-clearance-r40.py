import importlib,json
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
m=api.load_model();frozen_geometry=api.building_geometry(m)
base=Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES')
from shapely.geometry import Point,Polygon
from shapely.ops import unary_union
adj={};candidates={'A':[6.668116676570763,10.303608242295041],'B':[6.642597380018394,11.188393677573586],'proposal':[6.64,11.5]}
for tag in ['W-G-ROOM-10','W-G-SERVICE','W-PARENT-BATH-S','W-SUITE-ENTRY','GUARD-ST-GF','GUARD-ST-FR']:
 item=next(e for cat in ['walls','railings'] for e in frozen_geometry[cat] if e['tag']==tag)
 parts=item['parts']
 if tag.startswith('W-'):
  z=1.5 if tag.startswith('W-G') else 4.5;parts=[p for p in parts if p['z0']<=z<=p['z1']]
 footprints=[Polygon(p['outline']) for p in parts]
 adj[tag]={label:{'gap':min(Point(*xy).buffer(.2,quad_segs=96).distance(foot) for foot in footprints),'overlap_area':sum(Point(*xy).buffer(.2,quad_segs=96).intersection(foot).area for foot in footprints)} for label,xy in candidates.items()}
(base/'🗑️generated/chimney-proposal-clearance-r40.json').write_text(json.dumps(adj,indent=2));print(json.dumps(adj,indent=2));print('native_chimney_volume',api.service_geometry('BOILER-FLUE-VISIBLE',m)['volume'])
