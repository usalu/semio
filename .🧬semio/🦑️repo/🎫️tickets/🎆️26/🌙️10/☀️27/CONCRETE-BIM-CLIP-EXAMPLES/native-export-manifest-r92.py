import importlib,json,sys,hashlib
from pathlib import Path
r=next(p for p in Path(__file__).resolve().parents if (p/'package.json').exists());sys.path.insert(0,str(r));a=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');e=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️');m=a.load_model();out={'source':hashlib.sha256(Path(a.__file__).with_name('🔣️.json').read_bytes()).hexdigest(),'supported_classes':list(e.BODIES),'unsupported':[]}
for o in m['elements']:
 if o['class'] not in e.BODIES:out['unsupported'].append({'tag':o['tag'],'class':o['class'],'kind':o.get('geometry',{}).get('kind')})
Path(__file__).with_name('🗑️generated').joinpath('native-export-manifest-r92.json').write_text(json.dumps(out,ensure_ascii=False),encoding='utf8');print(json.dumps(out,ensure_ascii=False))
