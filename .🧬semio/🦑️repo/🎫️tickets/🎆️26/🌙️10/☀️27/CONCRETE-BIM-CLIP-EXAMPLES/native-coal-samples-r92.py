"""⛏️ Capture exact coal/footing ownership for sparse bilingual native proof."""
from pathlib import Path
import importlib,json,hashlib,sys,time
from scipy.spatial import ConvexHull
ROOT=Path.cwd();BASE=Path(__file__).parent;OUT=BASE/'🗑️generated/native-r92';sys.path.insert(0,str(ROOT))
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');ex=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️')
paths=[Path(api.__file__),Path(ex.__file__),Path(api.__file__).with_name('🔣️.json')]
def hashes():
    """🔐 Bind actual source/API/codec bytes."""
    return {p.relative_to(ROOT).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
before=hashes();m=api.load_model();samples=[]
controls=[{'point':[4.488717447916667,15.9459185,-2.46],'FT-CELLAR':False,'CELLAR-COAL-CHUTE':True},{'point':[4.488717447916667,15.9459185,-2.35],'FT-CELLAR':False,'CELLAR-COAL-CHUTE':False},{'point':[4.7,15.9459185,-2.46],'FT-CELLAR':True,'CELLAR-COAL-CHUTE':False},{'point':[4.488717447916667,15.9459185,-2.65],'FT-CELLAR':True,'CELLAR-COAL-CHUTE':False}]
for tag in ('FT-CELLAR','CELLAR-COAL-CHUTE'):
    obj=api._find(m,tag);body=api.solid_geometry(tag,m);parts=body['parts'];vertices=[v for p in parts for v in api.part_vertices(p)];volume=sum(ConvexHull(api.part_vertices(p)).volume for p in parts)
    for language in ('en','de'):
        fragment=api.step_excerpt(tag,m,language=language);text='\n'.join(fragment['lines'])+'\n';representation=next(l for l in fragment['lines'] if l.startswith(fragment['ids']['representation']+'='));kind=obj.get('predefined_type')
        if kind is None and obj.get('type'):kind=api._find(m,obj['type']).get('predefined_type')
        samples.append({'tag':tag,'language':language,'text':text,'ids':fragment['ids'],'guid':obj['guid'],'name':obj['name'][language],'entity':obj['class'].upper(),'predefined':kind,'representation_type':representation.split("'")[3],'parts':len(parts),'volume':volume,'bounds':[[min(p[i] for p in vertices) for i in range(3)],[max(p[i] for p in vertices) for i in range(3)]],'controls':[{'point':p['point'],'inside':p[tag]} for p in controls]})
        (OUT/('coal-'+tag+'-'+language+'.ifc')).write_text(text,encoding='utf8')
after=hashes();report={'stable':before==after,'before':before,'after':after,'samples':samples};(OUT/'coal-samples.json').write_text(json.dumps(report,ensure_ascii=False),encoding='utf8');assert report['stable'];print('[DEBUG] coal sparse native samples',[(s['tag'],s['parts']) for s in samples],flush=True)
