"""🔐️ Record the held reference SELECT source and authored native boundaries."""
from pathlib import Path
import hashlib,json,subprocess
ticket=Path(__file__).parent
root=ticket.parents[6]
generated=ticket/'🗑️generated'
ifc=root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc'
details=ifc/'🔎️details'
editor=ifc/'🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor'
ts=[details/name for name in ['type_input.ts','type_reference_select_input.test.ts','🧪️tests/type-reference-select-input.json','🧪️tests/type-reference-select-input.schema.json','🧪️tests/type-reference-select-input.feature']]
def row(path):
    result={'path':str(path.relative_to(root)).replace('\\','/'),'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'utf16':len(str(path).encode('utf-16-le'))//2}
    assert result['utf16']<=256
    return result
(generated/'type-reference-select-source.json').write_text(json.dumps({'nativeExecuted':False,'files':[row(path)for path in ts]},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
baseline=json.loads((generated/'native-type-select-source.json').read_text(encoding='utf-8'))
files=[root/item['path']for item in baseline]+ts[1:]+[details/'type_reference_select_input_tests.rs',editor/'guided-type-reference-select-tests.rs']
rows=[]
for index,path in enumerate(dict.fromkeys(files)):
    item=row(path)
    if path.suffix=='.rs':
        result=subprocess.run(['rustfmt','--edition','2024','--config','skip_children=true','--emit','stdout',str(path)],capture_output=True,text=True,encoding='utf-8',errors='replace')
        item['syntaxExit']=result.returncode
        (generated/f'native-reference-select-syntax-{index}.txt').write_text('[DEBUG] Reference SELECT syntax only\n'+result.stderr,encoding='utf-8')
    rows.append(item)
assert all(item.get('syntaxExit',0)==0 for item in rows)
assert all(row(root/item['path'])['sha256']==item['sha256']for item in rows)
(generated/'native-type-reference-select-source.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG]',json.dumps({'sourceFiles':len(rows),'syntaxFiles':sum('syntaxExit'in item for item in rows),'maximumPath':max(item['utf16']for item in rows),'nativeExecuted':False}))
result=subprocess.run(['cargo','test','--manifest-path',str(editor.parent/'📦️packages/🦀️rust/Cargo.toml'),'--lib','original_reference_select'],cwd=root,capture_output=True,text=True,encoding='utf-8',errors='replace')
(generated/'native-type-reference-select-cargo.txt').write_text('[DEBUG] Authentic Reference SELECT native selector\n'+result.stdout+result.stderr,encoding='utf-8')
print('[DEBUG]',json.dumps({'authenticCargoExit':result.returncode,'nativeExecuted':False}))
assert all(row(root/item['path'])['sha256']==item['sha256']for item in rows)
