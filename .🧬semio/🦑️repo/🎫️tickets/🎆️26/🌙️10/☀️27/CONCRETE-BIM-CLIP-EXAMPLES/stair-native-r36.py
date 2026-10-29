"""🪜 Author native structural-stair samples for an independent IFC engine."""
import importlib,json
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');importlib.reload(api);api._source.cache_clear()
exchange=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️');importlib.reload(exchange)
test=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪜️stair-parts');importlib.reload(test)
case,model=test.example();samples=[]
for language in ('en','de'):
    tag=case['stair'];obj=next(e for e in model['elements'] if e['tag']==tag);obj['predefined_type']='HALF_TURN_STAIR'
    physical=api.stair_geometry(tag,model);vertices=[v for p in physical['parts'] for v in api.part_vertices(p)];fragment=exchange.step_excerpt(tag,model,language=language)
    samples.append({'tag':tag,'guid':obj['guid'],'name':obj['name'][language],'language':language,'text':'\n'.join(fragment['lines']),'ids':fragment['ids'],'volume':case['expected']['volume'],'parts':len(physical['parts']),'bounds':[[min(v[i] for v in vertices) for i in range(3)],[max(v[i] for v in vertices) for i in range(3)]]})
(Path(__file__).parent/'🗑️generated/stair-native-r36.json').write_text(json.dumps(samples,ensure_ascii=False),encoding='utf8')
print({'samples':len(samples),'parts':samples[0]['parts'],'volume':samples[0]['volume']})
