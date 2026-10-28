import json,hashlib
from pathlib import Path
p=Path(__file__).parent;m=json.loads((p/'🗑️generated/guided-number-binary-control-source.json').read_text(encoding='utf-8-sig'))
rows=[dict(path=r['path'],matches=hashlib.sha256(Path(r['path']).read_bytes()).hexdigest()==r['sha256'])for r in m];assert all(r['matches']for r in rows)
(p/'🗑️generated/guided-rmb-independent-source.json').write_text(json.dumps(dict(rows=rows,nativeExecution=False),ensure_ascii=False,indent=2)+'\n',encoding='utf-8');print('[DEBUG] '+str(len(rows))+' frozen hashes match')
