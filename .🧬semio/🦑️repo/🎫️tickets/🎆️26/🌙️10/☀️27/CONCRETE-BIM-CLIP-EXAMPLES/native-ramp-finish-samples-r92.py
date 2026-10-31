"""🛝️ Capture original ramp finish construction for sparse bilingual native proof."""
from pathlib import Path
import importlib,json,hashlib,sys,time
from scipy.spatial import ConvexHull
ROOT=Path.cwd();BASE=Path(__file__).parent;OUT=BASE/'🗑️generated/native-r92';sys.path.insert(0,str(ROOT))
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');ex=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️')
paths=[p for p in Path(api.__file__).parent.rglob('*.py') if not any('🧪' in part or part=='__pycache__' for part in p.relative_to(Path(api.__file__).parent).parts)]+[Path(api.__file__).with_name('🔣️.json'),Path(api.__file__).parent/'🧬️schema/🔣️.schema.json']
def hashes():
    """🔐 Bind actual source/API/codec bytes."""
    return {p.relative_to(ROOT).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
before=hashes();m=api.load_model();samples=[]
controls=[]
for tag in ('FIN-RAMP-INTERIOR','FIN-RAMP-EXTERIOR'):
    obj=api._find(m,tag);body=api.solid_geometry(tag,m);parts=body['parts'];vertices=[v for p in parts for v in api.part_vertices(p)];volume=sum(ConvexHull(api.part_vertices(p)).volume for p in parts)
    for language in ('en','de'):
        fragment=api.step_excerpt(tag,m,language=language);text='\n'.join(fragment['lines'])+'\n';representation=next(l for l in fragment['lines'] if l.startswith(fragment['ids']['representation']+'='));kind=obj.get('predefined_type')
        if kind is None and obj.get('type'):kind=api._find(m,obj['type']).get('predefined_type')
        samples.append({'tag':tag,'language':language,'text':text,'ids':fragment['ids'],'guid':obj['guid'],'name':obj['name'][language],'entity':obj['class'].upper(),'predefined':kind,'representation_type':representation.split("'")[3],'parts':len(parts),'volume':volume,'bounds':[[min(p[i] for p in vertices) for i in range(3)],[max(p[i] for p in vertices) for i in range(3)]],'controls':[{'point':p['point'],'inside':p[tag]} for p in controls]})
        (OUT/('ramp-finish-'+tag+'-'+language+'.ifc')).write_text(text,encoding='utf8')
after=hashes();report={'stable':before==after,'before':before,'after':after,'samples':samples};(OUT/'ramp-finish-samples.json').write_text(json.dumps(report,ensure_ascii=False),encoding='utf8');assert report['stable'];print('[DEBUG] ramp finish sparse native samples',[(s['tag'],s['parts']) for s in samples],flush=True)
