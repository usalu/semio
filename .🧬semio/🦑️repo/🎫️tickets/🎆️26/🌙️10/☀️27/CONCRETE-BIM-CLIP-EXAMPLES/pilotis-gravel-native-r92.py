"""🚰️ Freeze actual patio collector, restored finish and native opening for independent SDK checks."""
import hashlib,importlib,json,sys,time
from pathlib import Path
from shapely.geometry import Polygon
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));ticket=Path(__file__).parent;out=ticket/'🗑️generated';api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');path=Path(api.__file__).with_name('🔣️.json');raw=path.read_bytes();m=json.loads(raw);rows=[];start=time.monotonic();paths=[Path(api.__file__),Path(api.__file__).parent/'🔌️exchange/🐍️.py'];before={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
obj=api._find(m,'FIN-PILOTIS-GRAVEL');parts=api._solid_geometry(m,obj)['parts'];largest=max(parts,key=lambda p:Polygon(p['outline']).area);w=Polygon(largest['outline']).representative_point();positive=[[w.x,w.y,-.17]];negative=[[w.x,w.y,-.10]]
for column in api._by_class(m,'IfcColumn'):
    for p in api.column_geometry(m,column)['parts']:
        if p.get('profile',{}).get('shape')!='circle' or min(p['z1'],-.15)<=max(p['z0'],-.19):continue
        r=p['profile']['diameter']['value']/2;cx=(min(v[0]for v in p['outline'])+max(v[0]for v in p['outline']))/2;cy=(min(v[1]for v in p['outline'])+max(v[1]for v in p['outline']))/2;negative.append([cx+r*.999,cy,-.17])
volume=sum(api._polyhedron_volume(p) for p in parts)
for language in ('en','de'):
    f=api.step_excerpt(obj['tag'],m,language=language);rows.append({'tag':obj['tag'],'guid':obj['guid'],'class':obj['class'].upper(),'language':language,'ids':f['ids'],'text':'\n'.join(f['lines']),'vertices':[api.part_vertices(p)for p in parts],'volume':volume,'positive_points':positive,'negative_points':negative})
assert before=={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths};report={'source':hashlib.sha256(raw).hexdigest(),'source_after':hashlib.sha256(path.read_bytes()).hexdigest(),'producers':before,'rows':rows,'seconds':time.monotonic()-start};(out/'pilotis-gravel-native-r92.json').write_text(json.dumps(report,ensure_ascii=False),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in report.items() if k!='rows'}))
