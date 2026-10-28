"""🔐️ Freeze only the owned SELECT consumer paths for independent review."""
from pathlib import Path
import hashlib,json
ticket=Path(__file__).parent
root=ticket.parents[6]
details=root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details'
paths=[details/'type_input.ts',details/'type_select_input.test.ts',*(details/'🧪️tests'/name for name in ['type-select-input.json','type-select-input.schema.json','type-select-input.feature'])]
rows=[{'path':str(path.relative_to(root)).replace('\\','/'),'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'absoluteLength':len(str(path).encode('utf-16-le'))//2}for path in paths]
(ticket/'🗑️generated/type-select-input-source.json').write_text(json.dumps({'nativeExecuted':False,'files':rows},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG]',json.dumps({'files':len(rows),'maxPath':max(row['absoluteLength']for row in rows)}))
