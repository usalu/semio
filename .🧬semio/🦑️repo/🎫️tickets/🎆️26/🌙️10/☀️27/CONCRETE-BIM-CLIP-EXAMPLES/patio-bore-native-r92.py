"""🚰️ Freeze actual patio collector, restored finish and native opening for independent SDK checks."""
import hashlib,importlib,json,sys,time
from pathlib import Path
from shapely.geometry import Polygon
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));ticket=Path(__file__).parent;out=ticket/'🗑️generated';api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');path=Path(api.__file__).with_name('🔣️.json');raw=path.read_bytes();m=json.loads(raw);rows=[];start=time.monotonic();paths=[Path(api.__file__),Path(api.__file__).parent/'🔌️exchange/🐍️.py'];before={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
for tag in ('COLLECTOR-KITCHEN-PATIO-9','FIN-KITCHEN-PATIO','OP-RAIN-9-SL1'):
    obj=api._find(m,tag)
    if obj['class']=='IfcWasteTerminal':
        parts=api._service(m,obj)['parts'];p=next(p for p in parts if p['role']=='inlet_sleeve');w=Polygon(p['outline']).representative_point();positive=[[w.x,w.y,(p['z0']+p['z1'])/2]];negative=[[2.8,8.6,3.025],[3.0434474784198784,8.793411991282804,3.025]]
    elif obj['class']=='IfcCovering':parts=api._solid_geometry(m,obj)['parts'];positive=[[2.8,8.7,3.1625],[2.88,8.6,3.1625]];negative=[[2.8,8.6,3.1625]]
    else:parts=api._slab_opening_geometry(m,obj)['parts'];positive=[[2.8,8.6,3.025]];negative=[[2.8,8.7,3.025]]
    volume=sum(api._polyhedron_volume(p) for p in parts)
    for language in ('en','de'):
        f=api.step_excerpt(tag,m,language=language);rows.append({'tag':tag,'guid':obj['guid'],'class':obj['class'].upper(),'language':language,'ids':f['ids'],'text':'\n'.join(f['lines']),'vertices':[api.part_vertices(p) for p in parts],'volume':volume,'positive_points':positive,'negative_points':negative})
assert before=={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths};report={'source':hashlib.sha256(raw).hexdigest(),'source_after':hashlib.sha256(path.read_bytes()).hexdigest(),'producers':before,'rows':rows,'seconds':time.monotonic()-start};(out/'patio-bore-native-r92.json').write_text(json.dumps(report,ensure_ascii=False),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in report.items() if k!='rows'}))
