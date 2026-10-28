import json,hashlib
from pathlib import Path
p=Path(__file__).parent;m=json.loads((p/'🗑️generated/cooperative-solid-validation-source.json').read_text(encoding='utf-8-sig'));rows=[dict(kind=kind,path=r['path'],matches=hashlib.sha256(Path(r['path']).read_bytes()).hexdigest().lower()==r['sha256'].lower())for kind in ['owned','references']for r in m[kind]];assert all(r['matches']for r in rows if r['kind']=='owned');(p/'🗑️generated/cooperative-solid-independent-source.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
