"""🧾 Capture six unpublished coordinated fixtures through actual bilingual STEP export."""
import hashlib,importlib,importlib.util,json,sys,time
from pathlib import Path
import numpy as np
from scipy.spatial import ConvexHull
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));ticket=Path(__file__).parent;out=ticket/'🗑️generated/native-r95-fixtures';out.mkdir(parents=True,exist_ok=True);prefix='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.';api=importlib.import_module(prefix+'🧱️model.🐍️');model=Path(api.__file__).parent;paths=[p for p in model.rglob('*.py')if '🧪️tests'not in str(p)]+[model/'🔣️.json',model/'🧬️schema/🔣️.schema.json',model/'🧪️tests/🔧️fixture-fits.py',model/'🧪️tests/🔧️fixture-fits.json',model/'🧪️tests/🔧️fixture-fits.schema.json',Path(__file__),ticket/'fixture-fits-author-r95.py']
def hashes():
    """🔐 Bind actual source, all physical producers and the precise candidate authoring inputs."""
    return {str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest()for p in paths}
before=hashes();spec=importlib.util.spec_from_file_location('fixture_fit_author_sdk_r95',ticket/'fixture-fits-author-r95.py');author=importlib.util.module_from_spec(spec);spec.loader.exec_module(author);source=author.apply(api.load_model());case=author.fits.case();tags=[case['rain']['tag'],case['lamp']['tag']]+[r['tag']for r in case['taps']]+[case['storage']['tag']];rows=[];start=time.monotonic()
for tag in tags:
    obj=api._find(source,tag);body=api.solid_geometry(tag,source);parts=body['parts'];vertices=[api.part_vertices(p)for p in parts];partvol=[ConvexHull(np.asarray(v)-np.mean(v,axis=0)).volume for v in vertices];positive=[np.mean(v,axis=0).tolist()for v in vertices];negative=[]
    if tag==case['rain']['tag']:negative=[[(a[k]+b[k])/2 for k in range(3)]for a,b in zip(case['rain']['spine'],case['rain']['spine'][1:])]
    for row in case['taps']:
        if row['tag']==tag:negative=[[row['axis'][0],row['axis'][1],3.17+.98],[row['axis'][0],row['axis'][1]+.16,3.17+1.04]]
    for language in('en','de'):
        fragment=api.step_excerpt(tag,source,language=language);rows.append({'tag':tag,'guid':obj['guid'],'class':obj['class'].upper(),'name':obj['name'][language],'predefined':obj.get('predefined_type')if any('PredefinedType'in level['adds']for level in api.class_chain(obj['class'],source))else None,'language':language,'ids':fragment['ids'],'text':'\n'.join(fragment['lines']),'vertices':vertices,'part_volumes':partvol,'volume':float(sum(partvol)),'positive_points':positive,'negative_points':negative})
    print('[DEBUG] captured fixture '+tag+' cells='+str(len(parts)),flush=True)
after=hashes();report={'source':hashlib.sha256((model/'🔣️.json').read_bytes()).hexdigest(),'producers':before,'after':after,'stable':before==after,'rows':rows,'seconds':time.monotonic()-start,'scope':'Unpublished six-asset source-qualified fixture fitting; all cells, identities, metadata and EN/DE SDK decoding required before source publication.'};(out/'fixture-fits-native-r95.json').write_text(json.dumps(report,ensure_ascii=True),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in report.items()if k not in('rows','producers','after')}),flush=True);raise SystemExit(not report['stable'])
