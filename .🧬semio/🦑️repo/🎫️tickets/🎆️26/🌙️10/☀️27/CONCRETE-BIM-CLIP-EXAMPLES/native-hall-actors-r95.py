"""🏗️ Capture each rebuilt hall wall and both complete dependent door products in both languages."""
import importlib,json,hashlib,runpy,sys
from pathlib import Path
R=Path.cwd();P=Path(__file__).parent;O=P/'🗑️generated/native-r95-hall';sys.path.insert(0,str(R));A=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');candidate=json.loads((O/'skirting-candidate.json').read_text(encoding='utf8'));cache=json.loads((O/'skirting-geometry.json').read_text(encoding='utf8'));assert candidate['source_stable'] and cache['snapshot']==candidate['before']
for name,value in candidate['after'].items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==value
m=runpy.run_path(str(P/'native-hall-bind-r95.py'))['prepare'](A,A.load_model(),candidate);g=cache['geometry'];paths={R/name for name in candidate['after']}|{Path(__file__),P/'native-hall-bind-r95.py',O/'skirting-candidate.json',O/'skirting-geometry.json'}
def hashes():
    """🔐 Bind native exports to the complete current source, producers, helpers and exact stock."""
    return {str(path.relative_to(R)):hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(paths)}
before=hashes();samples=[]
for tag in ('W-G-SERVICE','W-G-ROOM-10','W-G-ROOM-13','D-SERVANT-1','D-SERVANT-2'):
    obj=A._find(m,tag);body=next(item for family in ('walls','screens','fillings') for item in g[family] if item['tag']==tag);parts=body['parts'];vertices=[v for part in parts for v in A.part_vertices(part)];volume=sum(A._polyhedron_volume(part) for part in parts);assert volume>0
    for language in ('en','de'):
        print('[DEBUG] Native coordinated hall actor '+tag+'/'+language,flush=True);fragment=A.step_excerpt(tag,m,language=language);text='\n'.join(fragment['lines'])+'\n';representation=next(line for line in fragment['lines'] if line.startswith(fragment['ids']['representation']+'='));kind=obj.get('predefined_type')
        if kind is None and obj.get('type'):kind=A._find(m,obj['type']).get('predefined_type')
        samples.append({'tag':tag,'language':language,'text':text,'ids':fragment['ids'],'guid':obj['guid'],'name':obj['name'][language],'entity':obj['class'].upper(),'predefined':kind,'representation_type':representation.split("'")[3],'parts':len(parts),'volume':volume,'bounds':[[min(v[i] for v in vertices) for i in range(3)],[max(v[i] for v in vertices) for i in range(3)]]})
        (O/('actor-'+tag+'-'+language+'.ifc')).write_text(text,encoding='utf8')
after=hashes();report={'stable':before==after,'unpublished_candidate':True,'before':before,'after':after,'samples':samples};(O/'actor-samples.json').write_text(json.dumps(report,ensure_ascii=False),encoding='utf8');assert report['stable'];print('[DEBUG] Complete five-actor bilingual capture10fragments',flush=True)
