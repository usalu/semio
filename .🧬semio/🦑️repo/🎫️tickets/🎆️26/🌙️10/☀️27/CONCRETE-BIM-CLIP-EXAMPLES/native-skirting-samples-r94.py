"""🪵 Capture every published original timber board for bilingual native physical proof."""
from pathlib import Path
import importlib,json,hashlib,sys,time,runpy
from scipy.spatial import ConvexHull
ROOT=Path.cwd();BASE=Path(__file__).parent;OUT=BASE/'🗑️generated'/sys.argv[sys.argv.index('--output-subfolder')+1] if '--output-subfolder' in sys.argv else BASE/'🗑️generated/native-r92';sys.path.insert(0,str(ROOT))
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');ex=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️')
paths=[p for p in Path(api.__file__).parent.rglob('*.py') if not any('🧪' in part or part=='__pycache__' for part in p.relative_to(Path(api.__file__).parent).parts)]+[Path(api.__file__).with_name('🔣️.json'),Path(api.__file__).parent/'🧬️schema/🔣️.schema.json']
def hashes():
    """🔐 Bind actual source/API/codec bytes."""
    return {p.relative_to(ROOT).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
hall='--unpublished-hall' in sys.argv
diagnostic='--candidate' in sys.argv;diagnosed='--diagnosed' in sys.argv
if hall:paths.extend([OUT/'skirting-candidate.json',BASE/'native-hall-bind-r95.py',BASE/'native-hall-source-r95.py',BASE/'roof-layer-binding-source-r95.py'])
if diagnosed:paths.append(OUT/'skirting-candidate-native-r94.json')
if diagnostic:paths.extend([OUT/'skirting-candidate.json',BASE/'native-skirting-source-r92.py'])
paths.append(Path(__file__))
before=hashes();m=api.load_model();samples=[]
if hall:
    candidate=json.loads((OUT/'skirting-candidate.json').read_text(encoding='utf8'))
    for name,value in candidate['after'].items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==value
    m=runpy.run_path(str(BASE/'native-hall-bind-r95.py'))['prepare'](api,m,candidate)
if diagnostic:m=runpy.run_path(str(BASE/'native-skirting-source-r92.py'))['with_skirting'](api,m,json.loads((OUT/'skirting-candidate.json').read_text())['elements'])
controls=[]
tags=[obj['tag'] for obj in m['elements'] if obj['class']=='IfcCovering' and obj.get('predefined_type')=='SKIRTINGBOARD']
assert tags,'No published timber products'
if diagnosed:
    failed={row['tag'] for row in json.loads((OUT/'skirting-candidate-native-r94.json').read_text())['failures']};tags=[tag for tag in tags if tag in failed]
for tag in tags:
    if (OUT/'skirting.cancel').exists():raise KeyboardInterrupt('Native board capture cancelled')
    print('[DEBUG] native board capture '+tag,flush=True)
    obj=api._find(m,tag);body=api.solid_geometry(tag,m);parts=body['parts'];vertices=[v for p in parts for v in api.part_vertices(p)];volume=0.
    for p in parts:
        import numpy as np
        points=np.asarray(api.part_vertices(p),float);span=np.ptp(points,axis=0);volume+=ConvexHull((points-np.min(points,axis=0))/span).volume*float(np.prod(span))
    for language in ('en','de'):
        fragment=api.step_excerpt(tag,m,language=language);text='\n'.join(fragment['lines'])+'\n';representation=next(l for l in fragment['lines'] if l.startswith(fragment['ids']['representation']+'='));kind=obj.get('predefined_type')
        if kind is None and obj.get('type'):kind=api._find(m,obj['type']).get('predefined_type')
        samples.append({'tag':tag,'language':language,'text':text,'ids':fragment['ids'],'guid':obj['guid'],'name':obj['name'][language],'entity':obj['class'].upper(),'predefined':kind,'representation_type':representation.split("'")[3],'parts':len(parts),'volume':volume,'bounds':[[min(p[i] for p in vertices) for i in range(3)],[max(p[i] for p in vertices) for i in range(3)]],'controls':[{'point':p['point'],'inside':p[tag]} for p in controls]})
        (OUT/(('boardcand-' if diagnostic else 'board-')+tag+'-'+language+'.ifc')).write_text(text,encoding='utf8')
after=hashes();report={'partial_diagnosed':diagnosed,'unpublished_candidate':diagnostic or hall,'stable':before==after,'before':before,'after':after,'samples':samples};(OUT/('skirting-candidate-samples-r94.json' if diagnostic else 'skirting-samples-r94.json')).write_text(json.dumps(report,ensure_ascii=False),encoding='utf8');assert report['stable'];print('[DEBUG] published timber native samples',[(s['tag'],s['parts']) for s in samples],flush=True)
