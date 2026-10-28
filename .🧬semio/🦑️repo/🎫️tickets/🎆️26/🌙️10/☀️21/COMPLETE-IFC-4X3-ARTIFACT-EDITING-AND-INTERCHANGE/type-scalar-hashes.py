from pathlib import Path
import hashlib,json
root=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc');ticket=Path(__file__).parent
names=['🔎️details/type_input.ts','🔎️details/type_input.test.ts','🔎️details/type_multiple_input.test.ts','🔎️details/type_scalar_input.test.ts','🔎️details/🧪️tests/type-scalar-input.json','🔎️details/🧪️tests/type-scalar-input.schema.json','🔎️details/🧪️tests/type-scalar-input.feature','🔎️details/type_input.rs','🔎️details/type_input_kind.rs','🔎️details/type-guidance.rs','🔎️details/type_scalar_input_tests.rs','🔎️details/type_input_tests.rs','🔎️details/🦀️.rs','🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/details.rs','🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/guided-type-scalar-tests.rs']
rows=[]
for name in names:
 p=root/name;assert len(str(p.resolve()))<=256;rows.append({'path':str(p).replace('\\','/'),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()})
(ticket/'🗑️generated/type-scalar-source.json').write_text(json.dumps({'nativeExecution':False,'files':rows},ensure_ascii=False,indent=2)+'\n',encoding='utf8');print(json.dumps({'files':len(rows),'nativeExecution':False}))
