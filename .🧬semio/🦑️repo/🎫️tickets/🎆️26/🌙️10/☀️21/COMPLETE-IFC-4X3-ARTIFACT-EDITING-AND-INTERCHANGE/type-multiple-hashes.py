from pathlib import Path
import hashlib,json
root=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc');ticket=Path(__file__).parent
names=['🔎️details/type-guidance.rs','🔎️details/type-guidance-tests.rs','🔎️details/type_input.ts','🔎️details/type_input.test.ts','🔎️details/type_multiple_input.test.ts','🔎️details/🧪️tests/type-multiple-input.json','🔎️details/🧪️tests/type-multiple-input.schema.json','🔎️details/🧪️tests/type-multiple-input.feature','🔎️details/type_multiple_input_tests.rs','🔎️details/type_input_tests.rs','🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/guided-type-multiple-tests.rs']
rows=[]
for name in names:
 p=root/name;assert len(str(p.resolve()))<=256;rows.append({'path':str(p).replace('\\','/'),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()})
(ticket/'🗑️generated/type-multiple-source.json').write_text(json.dumps({'nativeExecution':False,'files':rows},ensure_ascii=False,indent=2)+'\n',encoding='utf8');print(json.dumps({'files':len(rows),'nativeExecution':False}))
