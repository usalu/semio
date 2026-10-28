from pathlib import Path
import hashlib,json
T=Path(__file__).parent
R=T.parents[6]
G=T/'🗑️generated'
D=R/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details'
paths=[D/n for n in ['type_input.ts','type_value_select_input.test.ts','🧪️tests/type-value-select-input.json','🧪️tests/type-value-select-input.schema.json','🧪️tests/type-value-select-input.feature','type_value_select_input_tests.rs']]
rows=[{'path':str(p.relative_to(R)).replace('\\','/'),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'utf16':len(str(p).encode('utf-16-le'))//2}for p in paths]
closed=json.loads((G/'native-type-measure-select-source.json').read_text(encoding='utf-8'))['files']
owned=[row for row in closed if row['scope']=='owned-native-measure']
assert len(owned)==11
assert all(hashlib.sha256((R/row['path']).read_bytes()).hexdigest()==row['sha256']for row in owned)
ts=json.loads((G/'type-measure-select-source.json').read_text(encoding='utf-8'))['files']
base=next(row for row in ts if row['path'].endswith('/type_input.ts'))
assert rows[0]['sha256']==base['sha256']
assert max(row['utf16']for row in rows)<=256
(G/'type-value-select-expanded-red-source.json').write_text(json.dumps({'productionExtended':False,'registered':False,'nativeMounted':False,'nativeExecuted':False,'closedMeasureNativeOwnedHashes':len(owned),'files':rows},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG]',json.dumps({'redFiles':len(rows),'closedMeasureNativeOwnedHashes':len(owned),'productionUnchanged':True,'maximumPath':max(row['utf16']for row in rows)}))
