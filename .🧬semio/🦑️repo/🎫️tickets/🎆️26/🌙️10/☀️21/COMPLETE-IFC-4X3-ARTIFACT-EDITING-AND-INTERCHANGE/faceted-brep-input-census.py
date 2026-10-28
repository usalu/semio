import json,hashlib
from pathlib import Path
p=Path(__file__).parent
names=['faceted-brep-geometry-vectors.json','faceted-brep-geometry-vectors.schema.json','faceted-brep-vectors-oracle.py','faceted-brep-vector-schema-oracle.ts','faceted-void-shell-evidence.py','faceted-void-shell-certificate.ts','faceted-brep-bounds-certificate.py']
r=[dict(path=str(p/n),sha256=hashlib.sha256((p/n).read_bytes()).hexdigest())for n in names]
(p/'🗑️generated/faceted-brep-vectors/input-source.json').write_text(json.dumps(r,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
