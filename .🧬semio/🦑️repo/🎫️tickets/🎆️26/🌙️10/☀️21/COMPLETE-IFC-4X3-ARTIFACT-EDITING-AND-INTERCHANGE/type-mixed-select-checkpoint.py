"""🔐️ Freeze the actual mixed SELECT consumer and its language-neutral contract."""
from pathlib import Path
import hashlib,json
ticket=Path(__file__).parent
root=ticket.parents[6]
details=root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details'
files=[details/name for name in ['type_input.ts','type_mixed_select_input.test.ts','🧪️tests/type-mixed-select-input.json','🧪️tests/type-mixed-select-input.schema.json','🧪️tests/type-mixed-select-input.feature']]
rows=[{'path':str(path.relative_to(root)).replace('\\','/'),'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'utf16':len(str(path).encode('utf-16-le'))//2}for path in files]
assert max(row['utf16']for row in rows)<=256
(ticket/'🗑️generated/type-mixed-select-source.json').write_text(json.dumps({'nativeExecuted':False,'files':rows},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG]',json.dumps({'sourceFiles':len(rows),'nativeExecuted':False}))
