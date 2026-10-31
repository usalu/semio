"""🔍 Validate exact diagnostic board geometry at declared nine-decimal native publication precision without quantization."""
from pathlib import Path
import importlib,json,sys,time,hashlib
BASE=Path(__file__).parent;OUT=BASE/'🗑️generated/native-r92';sys.path.insert(0,str(Path.cwd()))
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');before=hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest();start=time.monotonic()
report=json.loads((OUT/'skirting-candidate.json').read_text(encoding='utf8'));fronts=json.loads((OUT/'cupboard-skirting-candidate.json').read_text(encoding='utf8'));issues=[];count=0;exchange=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️')
for obj in {obj['tag']:obj for obj in report['elements']+fronts['elements']}.values():
    for index,part in enumerate(obj['geometry']['parts']):
        if (OUT/'skirting.cancel').exists():raise KeyboardInterrupt('Board precision audit cancelled')
        count+=1;points=[[float(exchange._real(v)) for v in p] for p in part['mesh']['vertices']]
        try:api._polyhedron_part(points,part['mesh']['faces'])
        except ValueError as error:issues.append({'tag':obj['tag'],'index':index,'error':str(error),'part':part,'rounded_vertices':points})
    print('[DEBUG] precision '+obj['tag'],flush=True)
after=hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest();result={'source_candidate_stable':report['stable'],'producer_before':before,'producer_after':after,'producer_stable':before==after,'parts':count,'failures':len(issues),'issues':issues,'elapsed':time.monotonic()-start}
(OUT/'skirting-precision.json').write_text(json.dumps(result,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] board precision '+json.dumps({k:v for k,v in result.items() if k!='issues'}));sys.exit(bool(issues))
