"""🏛️ Capture current native assemblies for independent English/German IFC mesh verification."""
import hashlib,importlib,json
from pathlib import Path
import numpy as np
from scipy.spatial import ConvexHull
from shapely.geometry import Polygon
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');importlib.reload(api);api._source.cache_clear()
exchange=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️');importlib.reload(exchange)
source_path=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json')
source_before=hashlib.sha256(source_path.read_bytes()).hexdigest()
source=api.load_model();samples=[]
building=api.building_geometry(source) if any(e['class']=='IfcSlab' and e['tag'] in product_tags for e in source['elements']) else None
for tag in globals()['product_tags']:
    obj=next(e for e in source['elements'] if e['tag']==tag)
    if obj['class']=='IfcSlab':parts=next(s for s in building['slabs'] if s['tag']==tag)['parts']
    elif obj['class']=='IfcWall':parts=api.path_wall_geometry(source,obj)['parts']
    elif obj['class'] in ('IfcDoor','IfcWindow','IfcCurtainWall'):parts=api.filling_geometry(tag,source)['parts']
    else:parts=api.service_geometry(tag,source)['parts']
    vertices=[v for part in parts for v in api.part_vertices(part)]
    def occupied_volume(part):
        if 'mesh' in part:return ConvexHull(np.asarray(api.part_vertices(part))).volume
        if 'section' in part:
            section=part['section'];return Polygon(section['profile']).area*abs(section['sweep'][1]-section['sweep'][0])
        rings=[part['outline']]
        if 'top_outline' in part:rings+=[[[sum(pair)/2 for pair in zip(a,b)] for a,b in zip(part['outline'],part['top_outline'])],part['top_outline']]
        areas=[Polygon(ring,part.get('holes',[])).area for ring in rings]
        return (part['z1']-part['z0'])*(areas[0] if len(areas)==1 else (areas[0]+4*areas[1]+areas[2])/6)
    volume=sum(occupied_volume(part) for part in parts)
    for language in ('en','de'):
        fragment=exchange.step_excerpt(tag,source,language=language)
        text='\n'.join(fragment['lines'])
        representation='SolidModel' if 'mesh' in next((p for p in parts if 'mesh' in p),{}) else 'SweptSolid'
        samples.append({'tag':tag,'entity':obj['class'].upper(),'predefined':obj['predefined_type'],'representation_type':representation,'guid':obj['guid'],'name':obj['name'][language],'language':language,'text':text,'ids':fragment['ids'],'volume':volume,'parts':len(parts),'bounds':[[min(v[i] for v in vertices) for i in range(3)],[max(v[i] for v in vertices) for i in range(3)]]})
source_after=hashlib.sha256(source_path.read_bytes()).hexdigest()
assert source_before==source_after,'Source advanced during native assembly capture'
result={'source_hash':source_before,'source_hash_after':source_after,'samples':samples}
Path(__file__).parent.joinpath('🗑️generated/'+globals().get('output_name','roof-native-r54.json')).write_text(json.dumps(result,ensure_ascii=False),encoding='utf8')
print({'source_hash':result['source_hash'],'samples':len(samples),'tags':[s['tag'] for s in samples[::2]]})
