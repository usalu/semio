from pathlib import Path
import hashlib,json
ticket=Path(__file__).parent
root=ticket.parents[6]
details=root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details'
files=[details/name for name in ['type_input.ts','type_list_input.test.ts','🧪️tests/type-list-input.json','🧪️tests/type-list-input.schema.json','🧪️tests/type-list-input.feature']]
rows=[{'path':str(path.relative_to(root)).replace('\\','/'),'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'utf16':len(str(path).encode('utf-16-le'))//2}for path in files]
assert max(row['utf16']for row in rows)<=256
(ticket/'🗑️generated/type-list-input-source.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG]',json.dumps({'sourceFiles':len(rows),'maximumPath':max(row['utf16']for row in rows),'nativeExecuted':False}))
