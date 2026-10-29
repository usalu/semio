"""🪜 Validate current native stair bodies against independent convex material measures."""
import importlib,json,hashlib,time,math
from pathlib import Path
from scipy.spatial import ConvexHull
from shapely.geometry import Polygon
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');importlib.reload(api);api._source.cache_clear()
exchange=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️');importlib.reload(exchange)
base=Path(__file__).parent/'🗑️generated';model=api.load_model();raw=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json').read_bytes();samples=[];measures=[]
for tag in ('ST-CG','ST-GF','ST-FR'):
    started=time.monotonic();(base/'stair-native-r39-progress.json').write_text(json.dumps({'tag':tag,'phase':'geometry'}),encoding='utf8')
    obj=next(e for e in model['elements'] if e['tag']==tag);physical=api.stair_geometry(tag,model);parts=physical['parts'];vertices=[v for p in parts for v in api.part_vertices(p)]
    volume=sum(ConvexHull(api.part_vertices(p)).volume if 'mesh' in p else Polygon(p['outline']).area*(p['z1']-p['z0']) for p in parts)
    (base/'stair-native-r39-progress.json').write_text(json.dumps({'tag':tag,'phase':'quantity','independent_material_sum':volume}),encoding='utf8')
    quantity=api.quantities(tag,model)['volume'];assert math.isclose(quantity,volume,abs_tol=1e-8),(tag,quantity,volume)
    for language in ('en','de'):
        fragment=exchange.step_excerpt(tag,model,language=language)
        samples.append({'tag':tag,'guid':obj['guid'],'name':obj['name'][language],'language':language,'text':'\n'.join(fragment['lines']),'ids':fragment['ids'],'volume':volume,'parts':len(parts),'bounds':[[min(v[i] for v in vertices) for i in range(3)],[max(v[i] for v in vertices) for i in range(3)]]})
    measures.append({'tag':tag,'quantity':quantity,'independent_material_sum':volume,'parts':len(parts),'seconds':time.monotonic()-started});(base/'stair-native-r39-progress.json').write_text(json.dumps({'tag':tag,'phase':'complete','measure':measures[-1]}),encoding='utf8')
(base/'stair-native-r39.json').write_text(json.dumps({'source_hash':hashlib.sha256(raw).hexdigest(),'samples':samples,'measures':measures},ensure_ascii=False),encoding='utf8')
print({'source_hash':hashlib.sha256(raw).hexdigest(),'measures':measures,'samples':len(samples)})
