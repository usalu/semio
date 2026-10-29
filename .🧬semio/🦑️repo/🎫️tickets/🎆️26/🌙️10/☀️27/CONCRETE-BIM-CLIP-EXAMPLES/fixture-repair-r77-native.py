"""🏛️ Capture affected fixture/door bodies for independent pinned native IFC decoding."""
import hashlib,importlib,json
from pathlib import Path
from shapely.geometry import Polygon
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');exchange=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️')
source=api.load_model();path=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json');before=hashlib.sha256(path.read_bytes()).hexdigest();samples=[]
for tag in ('TAP-LAUNDRY-1','TAP-LAUNDRY-2','TAP-GUEST','D-GUEST','W-PRIVATE'):
    obj=next(e for e in source['elements'] if e['tag']==tag)
    if obj['class']=='IfcWall':parts=api._wall_geometry(source,obj)['parts']
    elif obj['class']=='IfcDoor':parts=api.filling_geometry(tag,source)['parts']
    elif obj['class']=='IfcOpeningElement':parts=api.opening_geometry(tag,source)['parts']
    else:parts=api.solid_geometry(tag,source)['parts']
    vertices=[v for p in parts for v in api.part_vertices(p)];component_volume=sum(Polygon(p['outline'],p.get('holes',[])).area*(p['z1']-p['z0']) for p in parts)
    for language in ('en','de'):
        fragment=exchange.step_excerpt(tag,source,language=language)
        samples.append({'tag':tag,'entity':obj['class'].upper(),'predefined':obj['predefined_type'],'representation_type':'SweptSolid','guid':obj['guid'],'name':obj['name'][language],'language':language,'text':'\n'.join(fragment['lines']),'ids':fragment['ids'],'volume':component_volume,'volume_scope':'sum of emitted component solids; per-product union inventory separately verified','parts':len(parts),'bounds':[[min(v[i] for v in vertices) for i in range(3)],[max(v[i] for v in vertices) for i in range(3)]]})
result={'source_before':before,'source_after':hashlib.sha256(path.read_bytes()).hexdigest(),'samples':samples};assert result['source_before']==result['source_after'];Path(__file__).parent.joinpath('🗑️generated/fixture-repair-r77-native.json').write_text(json.dumps(result,ensure_ascii=False));print({'samples':len(samples),'source':before})
