"""🚰️ Freeze actual patio collector, restored finish and native opening for independent SDK checks."""
import hashlib,importlib,json,sys,time
from pathlib import Path
from shapely.geometry import Polygon
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));ticket=Path(__file__).parent;out=ticket/'🗑️generated';api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');path=Path(api.__file__).with_name('🔣️.json');raw=path.read_bytes();m=json.loads(raw);rows=[];start=time.monotonic();paths=[Path(api.__file__),Path(api.__file__).parent/'🔌️exchange/🐍️.py'];before={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
obj=api._find(m,'SL-GROUND');parts=api._slab_geometry_entry(m,obj)['parts'];positive=[[16.0125,13.8125,-.015],[16.0675,13.8125,-.015]];negative=[[16.0755,13.8125,-.015],[16.0125,13.8125,.001]];volume=sum(api._polyhedron_volume(p)for p in parts)
for language in ('en','de'):
    f=api.step_excerpt(obj['tag'],m,language=language);rows.append({'tag':obj['tag'],'guid':obj['guid'],'class':obj['class'].upper(),'language':language,'ids':f['ids'],'text':'\n'.join(f['lines']),'vertices':[api.part_vertices(p)for p in parts],'volume':volume,'positive_points':positive,'negative_points':negative})
assert before=={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths};report={'source':hashlib.sha256(raw).hexdigest(),'source_after':hashlib.sha256(path.read_bytes()).hexdigest(),'producers':before,'rows':rows,'seconds':time.monotonic()-start};(out/'garage-threshold-native-r92.json').write_text(json.dumps(report,ensure_ascii=False),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in report.items() if k!='rows'}))
