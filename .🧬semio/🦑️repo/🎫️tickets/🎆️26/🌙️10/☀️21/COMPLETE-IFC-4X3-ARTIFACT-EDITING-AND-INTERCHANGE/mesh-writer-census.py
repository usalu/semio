"""🔐️ Pin direct source constructors separately from parent-owned controller and publication work."""
from pathlib import Path
import hashlib,json
t=Path(__file__).parent
i=t.parents[6]/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc'
a=i/'🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io'
owned=[a/'geometry'/name for name in ['mesh-writer.ts','csg.ts','convex-solid.ts','circular-solid.ts','circle.ts','rounded.ts','rectangle.ts']]+[a/'🧪️tests'/name for name in ['mesh-writer-fixtures.ts','mesh-writer.test.ts','mesh-writer-cases.json','mesh-writer.schema.json','mesh-writer-authority.test.ts','mesh-writer-authority-cases.json','mesh-writer-authority.schema.json']]
prior=json.loads((t/'🗑️generated/rounded-profile-source.json').read_text(encoding='utf8'))
paths={Path(row['path'])for row in prior['owned']+prior['references']}|set(owned)|{a/'geometry/boolean.ts',i/'📐️geometry/🧊️boolean/arena.ts',i/'📐️geometry/🧊️boolean/workspace.ts'}
config=json.loads((t/'mesh-writer-final-tsconfig.json').read_text(encoding='utf8'))
paths|={(t/path).resolve()for path in config['files']}
refs=sorted(paths-set(owned),key=str)
def row(path):return{'path':str(path),'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'utf16PathUnits':len(str(path).encode('utf-16-le'))//2}
value={'owned':list(map(row,owned)),'references':list(map(row,refs)),'producerExecuted':True,'nativeExecuted':False,'schema':'IFC4X3_ADD2','scope':'One source algorithm per shape emits directly to actual pre-reserved raw or opaque owned endpoint. Original graph/source roles and strict REAL consumed domains; no raw mesh copy or added world reversal.'}
assert len(set(owned+refs))==len(owned+refs)
assert all(row['utf16PathUnits']<=256 for row in value['owned']+value['references'])
(t/'🗑️generated/mesh-writer-source.json').write_text(json.dumps(value,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
print('[DEBUG]',json.dumps({'owned':len(owned),'references':len(refs),'files':len(owned)+len(refs),'maxPath':max(row['utf16PathUnits']for row in value['owned']+value['references'])}))
