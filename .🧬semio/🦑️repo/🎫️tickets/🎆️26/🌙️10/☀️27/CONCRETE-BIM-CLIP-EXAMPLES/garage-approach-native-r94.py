"""🚗 Capture the independently accepted unpublished approach through actual EN/DE IFC export."""
import hashlib,importlib,importlib.util,json,sys,time
from pathlib import Path
from shapely.geometry import Polygon
root=Path.cwd();B=Path(__file__).parent;O=B/'🗑️generated/native-r92';sys.path.insert(0,str(root));started=time.perf_counter()
spec=importlib.util.spec_from_file_location('garage_author',B/'garage-approach-authoring-r94.py');H=importlib.util.module_from_spec(spec);spec.loader.exec_module(H);A=H.API
candidate_path=O/'garage-approach-candidate-r94.json';candidate=json.loads(candidate_path.read_text());physical=json.loads((O/'garage-approach-physical-green-r94.json').read_text())
assert physical['success'] and physical['candidate_sha256']==hashlib.sha256(candidate_path.read_bytes()).hexdigest()
paths=[Path(A.__file__),Path(A.__file__).parent/'🔌️exchange/🐍️.py',B/'garage-approach-authoring-r94.py',candidate_path]
before={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
raw=A._source();m=json.loads(raw);obj=H.apply_candidate(m,candidate);parts=A._solid_geometry(m,obj)['parts']
patches,_=H.construction();planes={p['name']:p['plane'] for p in patches};planes['flat']=(H.q(0),H.q(0),H.q(-.15))
positive=[];negative=[];seen=set();partvol=[];vertices=[]
for i,(p,cell) in enumerate(zip(parts,candidate['cells'])):
    vertices.append(A.part_vertices(p));partvol.append(A._polyhedron_volume(p))
    family=cell['patch'].split('-')[0]
    if family not in seen and cell['area']>1e-5:
        seen.add(family);point=Polygon(p['outline']).representative_point();ax,by,c=planes[cell['patch']];top=float(ax*H.q(point.x)+by*H.q(point.y)+c)
        positive.append([point.x,point.y,top-.02]);negative.extend([[point.x,point.y,top+.005],[point.x,point.y,top-.045]])
rows=[]
for language in ('en','de'):
    f=A.step_excerpt(obj['tag'],m,language=language)
    rows.append({'tag':obj['tag'],'guid':obj['guid'],'class':obj['class'].upper(),'language':language,'ids':f['ids'],'text':'\n'.join(f['lines']),'vertices':vertices,'part_volumes':partvol,'volume':physical['scipy_volume'],'positive_points':positive,'negative_points':negative})
assert before=={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
report={'source':hashlib.sha256(raw.encode('utf-8')).hexdigest(),'source_after':hashlib.sha256(A._source().encode('utf-8')).hexdigest(),'candidate_sha256':physical['candidate_sha256'],'producers':before,'rows':rows,'seconds':time.perf_counter()-started,'scope':'unpublished inferred approach only; all cells strict SDK required'}
(O/'garage-approach-native-r94.json').write_text(json.dumps(report,ensure_ascii=True),encoding='utf-8');print('[DEBUG] '+json.dumps({k:v for k,v in report.items() if k!='rows'}),flush=True)
