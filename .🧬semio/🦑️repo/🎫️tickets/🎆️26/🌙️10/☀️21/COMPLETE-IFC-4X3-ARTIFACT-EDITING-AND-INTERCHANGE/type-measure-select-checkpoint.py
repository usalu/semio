"""🔐️ Record the coherent MeasureValue TS consumer without native or Part21 certification."""
from pathlib import Path
import hashlib,json
ticket=Path(__file__).parent
root=ticket.parents[6]
details=root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details'
files=[details/name for name in ['type_input.ts','type_measure_select_input.test.ts','🧪️tests/type-measure-select-input.json','🧪️tests/type-measure-select-input.schema.json','🧪️tests/type-measure-select-input.feature']]
rows=[{'path':str(path.relative_to(root)).replace('\\','/'),'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'utf16':len(str(path).encode('utf-16-le'))//2}for path in files]
assert all(row['utf16']<=256 for row in rows)
(ticket/'🗑️generated/type-measure-select-source.json').write_text(json.dumps({'nativeExecuted':False,'part21ConformanceCertified':False,'files':rows},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG]',json.dumps({'files':len(rows),'maximumPath':max(row['utf16']for row in rows),'nativeExecuted':False,'part21ConformanceCertified':False}))
