"""🔗 Capture all current analysis-link values without mutating authored source."""
import hashlib,importlib,json,sys,time
from pathlib import Path
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));ticket=Path(__file__).parent
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');path=Path(api.__file__).with_name('🔣️.json');before=hashlib.sha256(path.read_bytes()).hexdigest();producer_paths=[p for p in Path(api.__file__).parent.rglob('*.py') if '🧪️tests' not in str(p)]+[Path(api.__file__).parent/'🧬️schema/🔣️.schema.json'];producers_before={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in producer_paths};model=api.load_model();started=time.monotonic();rows=[]
for link in model['links']:
    begin=time.monotonic();values=api._link_values(model,link['kind']);rows.append({'id':link['id'],'kind':link['kind'],'source_revision':model['revision'],'previous':link['used'],'used':values,'seconds':time.monotonic()-begin});print('[DEBUG] captured '+link['id'],flush=True)
after=hashlib.sha256(path.read_bytes()).hexdigest();out={'source_before':before,'source_after':after,'producers_before':producers_before,'producers_after':{str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest()for p in producer_paths},'stable':before==after and producers_before=={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest()for p in producer_paths},'api_sha256':hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest(),'analysis_sha256':hashlib.sha256(Path(api.__file__).parent.joinpath('🧮️analysis/🐍️.py').read_bytes()).hexdigest(),'rows':rows,'seconds':time.monotonic()-started}
(ticket/'🗑️generated'/'analysis-links-capture-r92.json').write_text(json.dumps(out,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in out.items()if k!='rows'}),flush=True)
