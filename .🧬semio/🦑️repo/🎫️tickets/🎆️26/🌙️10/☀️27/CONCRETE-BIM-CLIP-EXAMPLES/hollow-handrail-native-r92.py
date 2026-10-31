"""🛝️ Freeze the actual guard STEP bodies and binary64 vertex expectations for an independent SDK."""
import hashlib,importlib,json,sys,time
from pathlib import Path
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));ticket=Path(__file__).parent;out=ticket/'🗑️generated';api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');path=Path(api.__file__).with_name('🔣️.json');raw=path.read_bytes();m=json.loads(raw);tags=('TUBE-ST-GF','TUBE-ST-FR','TUBE-RM-2');start=time.monotonic();rows=[];case=json.loads((Path(api.__file__).parent/'🧪️tests/🪈️tubes.json').read_text(encoding='utf8'));producers=[Path(api.__file__),Path(api.__file__).parent/'🔌️exchange/🐍️.py'];before={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in producers}
for tag in tags:
    print('[DEBUG] Native handrail capture '+tag,flush=True);obj=api._find(m,tag);parts=api._solid_geometry(m,obj)['parts'];vertices=[api.part_vertices(p) for p in parts];volume=sum(api._polyhedron_volume(p) for p in parts)
    for language in ('en','de'):
        f=api.step_excerpt(tag,m,language=language);rows.append({'tag':tag,'guid':obj['guid'],'language':language,'ids':f['ids'],'text':'\n'.join(f['lines']),'vertices':vertices,'volume':volume,'air_controls':case['source_air_controls'][tag],'bearing_controls':[r for r in case['source_bearing_controls']if r['product']==tag]})
after={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in producers};assert before==after
report={'source':hashlib.sha256(raw).hexdigest(),'source_after':hashlib.sha256(path.read_bytes()).hexdigest(),'producers':before,'rows':rows,'seconds':time.monotonic()-start};(out/'hollow-handrail-native-r92.json').write_text(json.dumps(report,ensure_ascii=False),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in report.items() if k!='rows'}))
