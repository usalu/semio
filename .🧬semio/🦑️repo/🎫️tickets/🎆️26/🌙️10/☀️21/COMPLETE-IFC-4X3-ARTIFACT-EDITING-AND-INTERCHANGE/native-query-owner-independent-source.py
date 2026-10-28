import json,hashlib
from pathlib import Path
p=Path(__file__).parent;m=json.loads((p/'🗑️generated/native-query-owner-source.json').read_text(encoding='utf-8-sig'));rows=[dict(kind=k,path=r['path'],matches=hashlib.sha256(Path(r['path']).read_bytes()).hexdigest().upper()==r['sha256'].upper())for k in ['owned','references']for r in m[k]];assert all(r['matches']for r in rows if r['kind']=='owned');(p/'🗑️generated/native-query-owner-independent-source.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
