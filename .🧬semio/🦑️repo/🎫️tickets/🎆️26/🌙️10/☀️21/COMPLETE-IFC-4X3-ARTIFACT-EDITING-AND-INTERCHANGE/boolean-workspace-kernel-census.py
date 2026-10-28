"""🔐️ Freeze the actual entered-kernel and closed owned-source producer surface."""
from pathlib import Path
import hashlib,json
t=Path(__file__).parent
b=t.parents[6]/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🧊️boolean'
owned=[b/name for name in ['🟦️.ts','arena.ts','🔣️.json']]+[b/'🧪️tests'/name for name in ['🟦️.test.ts','workspace-kernel.test.ts','workspace-kernel-cases.json','workspace-kernel.schema.json','owned-workspace.test.ts','owned-workspace-transfer.test.ts','owned-workspace-cases.json','owned-workspace.schema.json','workspace-copy.test.ts','workspace-copy-cases.json','workspace-copy.schema.json']]
prior=json.loads((t/'🗑️generated/boolean-workspace-source.json').read_text(encoding='utf8'))
paths=set(b.rglob('*'))|{Path(row['path']) for row in prior['owned']+prior['references']}
refs=sorted((path for path in paths if path.is_file() and path not in owned),key=str)
def row(path):return {'path':str(path),'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'utf16PathUnits':len(str(path).encode('utf-16-le'))//2}
value={'owned':list(map(row,owned)),'references':list(map(row,refs)),'producerExecuted':True,'nativeExecuted':False,'physicalHeapBound':False,'registrationOwnedBy':'root','scope':'One captured numeric workspace, cooperative borrowed input leases, entered validation/operation arenas and opaque prepaid builder/output transfers. No IFC AST adapter or native producer.'}
assert len(set(owned+refs))==len(owned+refs)
assert all(row['utf16PathUnits']<=256 for row in value['owned']+value['references'])
(t/'🗑️generated/boolean-workspace-kernel-source.json').write_text(json.dumps(value,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
print('[DEBUG]',json.dumps({'owned':len(owned),'references':len(refs),'files':len(owned)+len(refs),'maxPath':max(row['utf16PathUnits']for row in value['owned']+value['references'])}))
