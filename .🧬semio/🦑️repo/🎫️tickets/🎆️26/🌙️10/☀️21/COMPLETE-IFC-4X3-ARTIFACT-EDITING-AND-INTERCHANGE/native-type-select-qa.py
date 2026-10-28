"""🔐️ Record current retained SELECT sources without rewriting historical checkpoints."""
from pathlib import Path
import hashlib,json,subprocess
ticket=Path(__file__).parent
root=ticket.parents[6]
generated=ticket/'🗑️generated'
ifc=root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc'
details=ifc/'🔎️details'
editor=ifc/'🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor'
baseline=json.loads((generated/'native-type-list-source.json').read_text(encoding='utf-8'))
files=[root/row['path']for row in baseline]
files += [details/name for name in ['constituent_roster.rs','type_select_input_tests.rs','type_input_typed_copy.rs','type_input_typed_copy_tests.rs','type_select_input.test.ts','🧪️tests/type-select-input.json','🧪️tests/type-select-input.schema.json','🧪️tests/type-select-input.feature']]
files += [editor/'guided-type-select-tests.rs']
rows=[]
for index,path in enumerate(dict.fromkeys(files)):
    row={'path':str(path.relative_to(root)).replace('\\','/'),'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'utf16':len(str(path).encode('utf-16-le'))//2}
    assert row['utf16']<=256
    if path.suffix=='.rs':
        result=subprocess.run(['rustfmt','--edition','2024','--config','skip_children=true','--emit','stdout',str(path)],capture_output=True,text=True,encoding='utf-8',errors='replace')
        row['syntaxExit']=result.returncode
        (generated/f'native-select-syntax-{index}.txt').write_text('[DEBUG] Native SELECT source syntax only\n'+result.stderr,encoding='utf-8')
    rows.append(row)
assert all(row.get('syntaxExit',0)==0 for row in rows)
assert all(hashlib.sha256((root/row['path']).read_bytes()).hexdigest()==row['sha256']for row in rows)
(generated/'native-type-select-source.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG]',json.dumps({'sourceFiles':len(rows),'syntaxFiles':sum('syntaxExit'in row for row in rows),'maximumPath':max(row['utf16']for row in rows),'nativeExecuted':False}))
result=subprocess.run(['cargo','test','--manifest-path',str(editor.parent/'📦️packages/🦀️rust/Cargo.toml'),'--lib','original_select'],cwd=root,capture_output=True,text=True,encoding='utf-8',errors='replace')
(generated/'native-type-select-cargo.txt').write_text('[DEBUG] Authentic SELECT native selector\n'+result.stdout+result.stderr,encoding='utf-8')
print('[DEBUG]',json.dumps({'authenticCargoExit':result.returncode,'nativeExecuted':False}))
assert all(hashlib.sha256((root/row['path']).read_bytes()).hexdigest()==row['sha256']for row in rows)
