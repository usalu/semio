"""🪑 Independently capture every authored generic furnishing and exact occupied union for native verification."""
import hashlib,importlib,json,math,time
from pathlib import Path
from shapely.geometry import Polygon
import numpy as np
prefix='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.';api=importlib.import_module(prefix+'🧱️model.🐍️');overlay=importlib.import_module(prefix+'🧱️model.🧪️tests.🚗️overlay');ticket=Path(__file__).parent;generated=ticket/'🗑️generated';path=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json');before=hashlib.sha256(path.read_bytes()).hexdigest();source=json.loads(path.read_text());samples=[];measures=[];started=time.monotonic()
def exact_volume(parts):
    levels=sorted({p[k] for p in parts for k in ('z0','z1')});cases=[];thickness=[]
    for i,(lo,hi) in enumerate(zip(levels,levels[1:])):
        active=[p['outline'] for p in parts if p['z0']<(lo+hi)/2<p['z1']];cases.append({'id':str(i),'actual':[[ring+[ring[0]]] for ring in active],'expected':[]});thickness.append(hi-lo)
    areas=json.loads(overlay.overlay_json(json.dumps({'cases':cases})))['cases'];return math.fsum(item['xor_area']*height for item,height in zip(areas,thickness))
objects=[obj for obj in source['elements'] if obj['class']=='IfcFurnishingElement'];assert len(objects)==25
for index,obj in enumerate(objects):
    assert 'predefined_type' not in obj;g=obj['geometry'];assert set(g)=={'kind','parts','base_level'}
    base=g['base_level'] if isinstance(g['base_level'],(int,float)) else next(s['elevation']['value'] for s in source['storeys'] if s['tag']==g['base_level']);parts=[]
    for raw in g['parts']:
        assert 'section' not in raw and 'mesh' not in raw and 'top_outline' not in raw and not raw.get('holes')
        shape=Polygon(raw['outline']);assert shape.is_valid
        xy=np.asarray(raw['outline']);assert abs(shape.area-abs(np.dot(xy[:,0],np.roll(xy[:,1],1))-np.dot(xy[:,1],np.roll(xy[:,0],1)))/2)<1e-9
        parts.append({**raw,'z0':base+raw['z0'],'z1':base+raw['z1']})
    union=exact_volume(parts);component=sum(Polygon(p['outline']).area*(p['z1']-p['z0']) for p in parts);q=api.quantities(obj['tag'],source);assert abs(q['volume']-union)<1e-8,(obj['tag'],q['volume'],union)
    expected_bounds=[[min(p['outline'][j][axis] for p in parts for j in range(len(p['outline']))) for axis in (0,1)]+[min(p['z0'] for p in parts)],[max(p['outline'][j][axis] for p in parts for j in range(len(p['outline']))) for axis in (0,1)]+[max(p['z1'] for p in parts)]]
    for language in ('en','de'):
        fragment=api.step_excerpt(obj['tag'],source,language=language);material_tags=list(dict.fromkeys(p.get('material',obj.get('material')) for p in parts))
        samples.append({'tag':obj['tag'],'entity':'IFCFURNISHINGELEMENT','representation_type':'SweptSolid','guid':obj['guid'],'name':obj['name'][language],'language':language,'text':'\n'.join(fragment['lines']),'ids':fragment['ids'],'parts':len(parts),'volume':component,'net_volume':union,'storey_guid':next(s['guid'] for s in source['storeys'] if s['tag']==obj['storey']),'material_names':[next(m for m in source['materials'] if m['tag']==tag)['name'][language] for tag in material_tags],'bounds':expected_bounds})
    measures.append({'tag':obj['tag'],'parts':len(parts),'component_sum':component,'occupied_union':union,'api_volume':q['volume']});(generated/'furnishing-all-r77-progress.json').write_text(json.dumps({'debug':'[DEBUG] all25native source capture','completed':index+1,'planned':25,'tag':obj['tag'],'seconds':time.monotonic()-started}))
record={'source_before':before,'source_after':hashlib.sha256(path.read_bytes()).hexdigest(),'samples':samples,'measures':measures,'seconds':time.monotonic()-started};assert record['source_before']==record['source_after'];(generated/'furnishing-all-r77.json').write_text(json.dumps(record,ensure_ascii=False));print({'source':before,'products':len(objects),'samples':len(samples),'seconds':record['seconds']})
