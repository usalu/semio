"""🧱 Capture isolated indoor coal-bay floor candidate without source publication."""
from pathlib import Path
import importlib,json,hashlib,sys,time,runpy
from shapely.geometry import Polygon
import numpy as np
from scipy.spatial import ConvexHull
ROOT=Path.cwd();BASE=Path(__file__).parent;OUT=BASE/'🗑️generated/native-r92';sys.path.insert(0,str(ROOT))
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');ex=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️')
paths=[p for p in Path(api.__file__).parent.rglob('*.py') if not any('🧪' in part or part=='__pycache__' for part in p.relative_to(Path(api.__file__).parent).parts)]+[Path(api.__file__).with_name('🔣️.json'),Path(api.__file__).parent/'🧬️schema/🔣️.schema.json']
def hashes():
    """🔐 Bind actual source/API/codec bytes."""
    return {p.relative_to(ROOT).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
paths.extend([Path(__file__),BASE/'coal-bay-floor-author-r94.py',OUT/'coal-bay-floor-candidate-r94.json'])
before=hashes();m=api.load_model();candidate=json.loads((OUT/'coal-bay-floor-candidate-r94.json').read_text());assert candidate['source_sha256']==hashlib.sha256(Path(api.__file__).with_name('🔣️.json').read_bytes()).hexdigest();m=runpy.run_path(str(BASE/'coal-bay-floor-author-r94.py'))['apply'](m,candidate);samples=[]
controls=[]
for tag in sorted(candidate['additions']):
    print('[DEBUG] native indoor floor capture '+tag,flush=True)
    obj=api._find(m,tag);body=api.solid_geometry(tag,m);parts=body['parts'];vertices=[v for p in parts for v in api.part_vertices(p)];volume=0.
    for p in parts:
        if not any(key in p for key in ('mesh','section','top_outline')):volume+=Polygon(p['outline'],p.get('holes',[])).area*(p['z1']-p['z0'])
        else:
            points=np.asarray(api.part_vertices(p),float);span=np.ptp(points,axis=0);volume+=ConvexHull((points-points.min(axis=0))/span).volume*float(np.prod(span))
    first=parts[0]
    if not any(key in first for key in ('mesh','section','top_outline')):
        inside=Polygon(first['outline'],first.get('holes',[])).representative_point();positive=[inside.x,inside.y,(first['z0']+first['z1'])/2]
    else:positive=np.mean(np.asarray(api.part_vertices(first),float),axis=0).tolist()
    for language in ('en','de'):
        fragment=api.step_excerpt(tag,m,language=language);text='\n'.join(fragment['lines'])+'\n';representation=next(l for l in fragment['lines'] if l.startswith(fragment['ids']['representation']+'='));kind=obj.get('predefined_type')
        if kind is None and obj.get('type'):kind=api._find(m,obj['type']).get('predefined_type')
        samples.append({'tag':tag,'language':language,'text':text,'ids':fragment['ids'],'guid':obj['guid'],'name':obj['name'][language],'entity':obj['class'].upper(),'predefined':kind,'representation_type':representation.split("'")[3],'parts':len(parts),'volume':volume,'bounds':[[min(p[i] for p in vertices) for i in range(3)],[max(p[i] for p in vertices) for i in range(3)]],'controls':[{'point':positive,'inside':True}]})
        (OUT/('coalfloor-'+tag+'-'+language+'.ifc')).write_text(text,encoding='utf8')
after=hashes();report={'unpublished_candidate':True,'candidate_sha256':hashlib.sha256((OUT/'coal-bay-floor-candidate-r94.json').read_bytes()).hexdigest(),'stable':before==after,'before':before,'after':after,'samples':samples};(OUT/'coal-bay-floor-samples-r94.json').write_text(json.dumps(report,ensure_ascii=False),encoding='utf8');assert report['stable'];print('[DEBUG] isolated indoor coal floor samples',[(s['tag'],s['parts']) for s in samples],flush=True)
