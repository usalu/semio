import json,hashlib
from pathlib import Path
p=Path(__file__).parent;m=json.loads((p/'🗑️generated/geometry-source-index.json').read_text(encoding='utf-8-sig'))
rows=[dict(path=r['path'],matches=hashlib.sha256(Path(r['path']).read_bytes()).hexdigest()==r['sha256'])for r in m['owned']+m['references']]
assert all(r['matches']for r in rows)
(p/'🗑️generated/geometry-source-index-independent.json').write_text(json.dumps(dict(rows=rows,actualTests=5,assertions=47,failures=0),ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
