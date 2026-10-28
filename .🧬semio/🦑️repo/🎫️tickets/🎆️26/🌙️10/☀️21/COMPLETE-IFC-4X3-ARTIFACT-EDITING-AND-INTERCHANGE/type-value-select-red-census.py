from pathlib import Path
import hashlib,json
T=Path(__file__).parent
R=T.parents[6]
D=R/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details'
paths=[D/n for n in ['type_input.ts','type_value_select_input.test.ts','🧪️tests/type-value-select-input.json','🧪️tests/type-value-select-input.schema.json','🧪️tests/type-value-select-input.feature']]
rows=[{'path':str(p.relative_to(R)).replace('\\','/'),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'utf16':len(str(p).encode('utf-16-le'))//2}for p in paths]
assert max(row['utf16']for row in rows)<=256
(T/'🗑️generated/type-value-select-red-source.json').write_text(json.dumps({'productionExtended':False,'registered':False,'nativeExecuted':False,'files':rows},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG]',json.dumps({'files':len(rows),'maximumPath':max(row['utf16']for row in rows),'productionExtended':False,'registered':False}))
