"""💡 Freeze four actual native bodies for independent shared-point confirmation."""
import hashlib,importlib,json,sys,time
from pathlib import Path
T=Path(__file__).parent;root=next(p for p in T.parents if (p/'AGENTS.md').is_file() and (p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');path=Path(api.__file__).with_name('🔣️.json');raw=path.read_bytes();m=json.loads(raw);tags=['LIGHT-EXTERIOR-11','11','LIGHT-EXTERIOR-6','6'];samples=[];started=time.monotonic()
def selected(model):return {tag:hashlib.sha256(json.dumps(api._find(model,tag),sort_keys=True,ensure_ascii=False).encode()).hexdigest() for tag in tags}
before=selected(m);producer_paths=[Path(api.__file__),Path(api.__file__).parent/'🔌️exchange/🐍️.py'];producers={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in producer_paths}
for tag in tags:
 for language in ('en','de'):
  f=api.step_excerpt(tag,m,language=language);samples.append({'tag':tag,'language':language,'ids':f['ids'],'text':'\n'.join(f['lines']),'witness':[.1225413951052298,6.000877906449073,2.1667538846729677] if tag in ('11','LIGHT-EXTERIOR-11') else [18.87745860489477,6.000877906449073,2.1667538846729677]})
after=path.read_bytes();out={'source':hashlib.sha256(raw).hexdigest(),'source_after':hashlib.sha256(after).hexdigest(),'selected_before':before,'selected_after':selected(json.loads(after)),'producers_before':producers,'producers_after':{k:hashlib.sha256((root/k).read_bytes()).hexdigest() for k in producers},'native_fragments':samples,'seconds':time.monotonic()-started};assert out['selected_before']==out['selected_after'];assert out['producers_before']==out['producers_after'];(T/'🗑️generated/light-post-native-r92.json').write_text(json.dumps(out,ensure_ascii=False),encoding='utf8');print(json.dumps({k:v for k,v in out.items() if k!='native_fragments'}),flush=True)
