import json,hashlib
from pathlib import Path
p=Path(__file__).parent;m=json.loads((p/'🗑️generated/type-list-input-source.json').read_text(encoding='utf-8-sig'));rows=[dict(path=r['path'],matches=hashlib.sha256(Path(r['path']).read_bytes()).hexdigest().lower()==r['sha256'].lower())for r in m];assert all(r['matches']for r in rows);(p/'🗑️generated/type-list-independent-source.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
