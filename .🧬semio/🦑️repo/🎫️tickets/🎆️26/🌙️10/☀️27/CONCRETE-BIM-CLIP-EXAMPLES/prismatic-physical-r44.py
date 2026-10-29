import importlib,json,time,copy
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');api._source.cache_clear();m=api.load_model()
base=Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES')
patch=json.loads((base/'🗑️generated/prismatic-patch-r44.json').read_text())
if not any(e['tag']==patch['elements'][0]['tag'] for e in m['elements']):
 m['elements']+=patch['elements'];m['relations']+=patch['relations'];next(e for e in m['elements'] if e['tag']=='SL-R')['geometry']['finish_voids']=patch['finish_voids'];m['building']['box']['height']=patch['height']
 next(r for r in m['relations'] if r['tag']=='REL-CON-roof')['related'] += [e['tag'] for e in patch['elements']]
 for pm in m['models']:
  pm['element_tags'] += [e['tag'] for e in patch['elements'] if pm['id'] in e['metadata']['domains']]
progress=base/'🗑️generated/prismatic-progress-r44.json';progress.write_text(json.dumps({'phase':'selected native slab and service','state':'running'}))
started=time.monotonic();native_slr=api._slab_geometry(m,api.element('SL-R',m));native_chimneys=[api.service_geometry(c['tag'],m) for c in patch['elements'] if c['class']=='IfcChimney']
(base/'🗑️generated/prismatic-native-r44.json').write_text(json.dumps({'slab':native_slr,'chimneys':native_chimneys,'quantities':{x['tag']:api.quantities(x['tag'],m) for x in native_chimneys}},ensure_ascii=False))
progress.write_text(json.dumps({'phase':'selected native slab and service','state':'PASS','seconds':time.monotonic()-started}));print('Selected native support+composite data exported',time.monotonic()-started)

pr=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚬️prisms');saved_load=api.load_model;api.load_model=lambda:copy.deepcopy(m)
try:
 for name in [n for n in dir(pr) if n.startswith('test_')]:
  getattr(pr,name)();print(name,'PASS')
 for case in pr.CASE['terminals']:
  for mode in ['axis','footprint','plan','section','elevation','3d','body']:
   representation=api.represent(case['tag'],mode,m);assert representation['primitives'],(case['tag'],mode)
 print('All native composite representation modes PASS')
finally:api.load_model=saved_load

progress.write_text(json.dumps({'phase':'full candidate geometry','state':'running','products':len(m['elements'])}));started=time.monotonic();candidate_g=api.building_geometry(m)
progress.write_text(json.dumps({'phase':'full candidate geometry','state':'PASS','products':len(m['elements']),'seconds':time.monotonic()-started}));print('Full candidate building_geometry PASS',len(m['elements']),time.monotonic()-started)

import traceback
from shapely.geometry import Polygon,box
from shapely.ops import unary_union
collision=[]
for case in pr.CASE['terminals']:
 data=api.service_geometry(case['tag'],m);shape=unary_union([Polygon(p['outline']) for p in data['parts']]);lo=min(p['z0'] for p in data['parts']);hi=max(p['z1'] for p in data['parts']);bounds=shape.bounds
 for category in ['slabs','walls','screens','pilotis','beams','foundations','stairs','railings','furnishings','details','fillings','services']:
  for item in candidate_g.get(category,[]):
   if item['tag']==case['tag']:continue
   for part in item.get('parts',[])+item.get('finish_parts',[]):
    if min(hi,part['z1'])<=max(lo,part['z0'])+1e-10:continue
    ring=api.part_footprint(part)
    if not ring:continue
    xs,ys=zip(*ring)
    if max(xs)<bounds[0] or min(xs)>bounds[2] or max(ys)<bounds[1] or min(ys)>bounds[3]:continue
    area=shape.intersection(Polygon(ring)).area
    assert area<1e-8,(case['tag'],category,item['tag'],area)
 for col in candidate_g['pilotis']:
  for part in col['parts']:
   if min(hi,part['z1'])<=max(lo,part['z0'])+1e-10:continue
   assert shape.distance(Polygon(api.part_footprint(part)))>=.25-1e-9,(case['tag'],col['tag'])
 collision.append({'tag':case['tag'],'state':'PASS','all_occupied_categories':True,'column_clearance':.25})
print('All occupied3D/Z-band conservative clash and generic column clearance PASS')
saved_load=api.load_model;saved_bg=api.building_geometry;api.load_model=lambda:copy.deepcopy(m);api.building_geometry=lambda model=None:copy.deepcopy(candidate_g) if model is None or model==m else saved_bg(model)
checks=[]
try:
 pr=importlib.reload(pr)
 for name in [n for n in dir(pr) if n.startswith('test_')]:
  start=time.monotonic();getattr(pr,name)();checks.append({'name':name,'status':'PASS','seconds':time.monotonic()-start})
 print('4 source/synthetic/material/representation/production prism contracts PASS')
 for suffix in ['🚬️chimneys','🏠ground','🪜️stairs']:
  mod=importlib.reload(importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.'+suffix))
  for name in [n for n in dir(mod) if n.startswith('test_')]:
   start=time.monotonic();getattr(mod,name)();checks.append({'name':suffix+name,'status':'PASS','seconds':time.monotonic()-start})
 print('Existing boiler,7ground and current mandatory-U stair contracts PASS')
finally:api.load_model=saved_load;api.building_geometry=saved_bg
(base/'🗑️generated/prismatic-checks-r44.json').write_text(json.dumps({'collision':collision,'tests':checks},indent=2))
ex=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️')
for case in pr.CASE['terminals']:
 for language in ['en','de']:
  native=ex.step_excerpt(case['tag'],m,language=language);print(case['tag'],language,native.get('representation_type'),native.keys())
