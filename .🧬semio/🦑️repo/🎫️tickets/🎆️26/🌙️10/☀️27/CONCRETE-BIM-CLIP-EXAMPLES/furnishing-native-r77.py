"""🪑 Capture actual native furnishing and synthetic mixed solids for independent IFC decoding."""
import hashlib,importlib,json
from copy import deepcopy
from pathlib import Path
from scipy.spatial import ConvexHull
prefix='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.';api=importlib.import_module(prefix+'🧱️model.🐍️');module=importlib.import_module(prefix+'🧱️model.🔌️exchange.🧪️tests.🪑furnishings');source=api.load_model();case=json.loads(Path(module.__file__).with_suffix('.json').read_text());ticket=Path(__file__).parent;path=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json');before=hashlib.sha256(path.read_bytes()).hexdigest();samples=[]
synthetic=case['synthetic'];model=deepcopy(source);obj={'tag':synthetic['tag'],'guid':api.global_id(synthetic['tag']),'class':'IfcFurnishingElement','name':{'en':'Synthetic owned furnishing','de':'Synthetische Ausstattung'},'storey':'ground','material':synthetic['fallback_material'],'geometry':deepcopy(synthetic['geometry']),'attributes':{},'psets':{},'metadata':{'author':'portable fixture','revision':1,'domains':['architecture']}};model['elements'].append(obj)
inputs=[(source,c['tag']) for c in case['source_cases']]+[(model,obj['tag'])]
no_fallback=deepcopy(model);last=no_fallback['elements'][-1]
for part in last['geometry']['parts']:part.setdefault('material',last['material'])
last.pop('material');inputs.append((no_fallback,obj['tag']))
for index,(m,tag) in enumerate(inputs):
    item=next(e for e in m['elements'] if e['tag']==tag);parts=api.solid_geometry(tag,m)['parts'];vertices=[v for part in parts for v in api.part_vertices(part)];volume=sum(ConvexHull(api.part_vertices(part)).volume for part in parts)
    for language in ('en','de'):
        fragment=api.step_excerpt(tag,m,language=language);material_tags=list(dict.fromkeys(p.get('material',item.get('material')) for p in parts));material_names=[api.element(t,m)['name'][language] for t in material_tags]
        samples.append({'case':index,'tag':tag,'entity':'IFCFURNISHINGELEMENT','representation_type':'SweptSolid','guid':item['guid'],'name':item['name'][language],'language':language,'text':'\n'.join(fragment['lines']),'ids':fragment['ids'],'parts':len(parts),'volume':volume,'net_volume':api.quantities(tag,m)['volume'],'storey_guid':api.element(item['storey'],m)['guid'],'material_names':material_names,'bounds':[[min(v[k] for v in vertices) for k in range(3)],[max(v[k] for v in vertices) for k in range(3)]]})
record={'source_before':before,'source_after':hashlib.sha256(path.read_bytes()).hexdigest(),'samples':samples};assert record['source_before']==record['source_after'];(ticket/'🗑️generated/furnishing-native-r77.json').write_text(json.dumps(record,ensure_ascii=False));print({'samples':len(samples),'source':before})
