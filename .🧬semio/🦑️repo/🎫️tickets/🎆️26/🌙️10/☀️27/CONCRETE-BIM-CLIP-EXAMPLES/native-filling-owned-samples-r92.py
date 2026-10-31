"""🪟 Export current repaired source fillings for an independent native IFC occupancy audit."""
from pathlib import Path
import hashlib,importlib,json,sys,time
from shapely.geometry import Polygon
ROOT=Path.cwd();BASE=Path(__file__).parent;OUT=BASE/'🗑️generated/native-r92';sys.path.insert(0,str(ROOT))
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
ex=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️')
paths=[Path(api.__file__),Path(ex.__file__),Path(api.__file__).with_name('🔣️.json')]
def hashes():
    """🔐 Bind source and actual geometry/codec producer inputs."""
    return {p.relative_to(ROOT).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
before=hashes();model=api.load_model();cases=json.loads((BASE/'filling-structure-owned-cases-r92.json').read_text())['cases'];samples=[];start=time.monotonic()
for tag in sorted({r['source']['tag'] for r in cases}):
    if (OUT/'filling-owned.cancel').exists():raise KeyboardInterrupt('Native filling capture cancelled')
    obj=api._find(model,tag);body=api._curtain_filling_geometry(model,obj) if obj['class']=='IfcCurtainWall' else api._filling_geometry(model,obj)
    parts=body['parts'];vertices=[v for p in parts for v in api.part_vertices(p)];volume=sum(Polygon(p['outline'],p.get('holes',[])).area*(p['z1']-p['z0']) for p in parts)
    for language in ('en','de'):
        fragment=api.step_excerpt(tag,model,language=language);text='\n'.join(fragment['lines'])+'\n';representation=next(line for line in fragment['lines'] if line.startswith(fragment['ids']['representation']+'='));representation_type=representation.split("'")[3]
        (OUT/('owned-'+tag+'-'+language+'.ifc')).write_text(text,encoding='utf8')
        samples.append({'tag':tag,'language':language,'text':text,'ids':fragment['ids'],'entity':obj['class'].upper(),'guid':obj['guid'],'name':obj['name'][language],'predefined':obj.get('predefined_type',api._find(model,obj['type']).get('predefined_type')) if 'type' in obj else obj.get('predefined_type'),'parts':len(parts),'volume':volume,'bounds':[[min(p[i] for p in vertices) for i in range(3)],[max(p[i] for p in vertices) for i in range(3)]],'representation_type':representation_type,'witnesses':[row['witness'] for row in cases if row['source']['tag']==tag]})
    print('[DEBUG] native filling capture '+tag+' '+str(len(parts)),flush=True)
after=hashes();report={'stable':before==after,'before':before,'after':after,'seconds':time.monotonic()-start,'samples':samples}
(OUT/'filling-owned-samples.json').write_text(json.dumps(report,ensure_ascii=False),encoding='utf8')
assert report['stable'],'Native capture source changed'
