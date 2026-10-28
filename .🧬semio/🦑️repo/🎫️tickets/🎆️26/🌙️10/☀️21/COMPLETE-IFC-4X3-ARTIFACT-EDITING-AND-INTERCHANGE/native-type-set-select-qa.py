"""🧺️ Freeze the full original SET producer source roster without native execution claims."""
from pathlib import Path
import hashlib,json,subprocess
ticket=Path(__file__).parent
root=ticket.parents[6]
generated=ticket/'🗑️generated'
ifc=root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc'
details=ifc/'🔎️details'
editor=ifc/'🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor'
baseline=json.loads((generated/'native-type-mixed-select-source.json').read_text(encoding='utf-8'))
owned=[details/name for name in ['type_input_kind.rs','type_input.rs','type_input_typed_copy.rs','type_input_typed_copy_tests.rs','type_input_set_copy.rs','type_input_set_copy_tests.rs','type_input_set_controls.rs','type-guidance.rs','type_set_select_input_tests.rs','work_tests.rs']]+[editor/'guided-type-set-select-tests.rs',editor/'render-work-tests.rs']
files=[root/row['path']for row in baseline]+owned+[details/name for name in ['type_set_select_input.test.ts','🧪️tests/type-set-select-input.json','🧪️tests/type-set-select-input.schema.json','🧪️tests/type-set-select-input.feature']]
rows=[]
for index,path in enumerate(dict.fromkeys(files)):
    row={'path':str(path.relative_to(root)).replace('\\','/'),'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'utf16':len(str(path).encode('utf-16-le'))//2,'scope':'owned-set-production-or-native-law'if path in owned else'reference-or-closed-ts-law'}
    assert row['utf16']<=256
    if path.suffix=='.rs':
        result=subprocess.run(['rustfmt','--edition','2024','--config','skip_children=true','--emit','stdout',str(path)],capture_output=True,text=True,encoding='utf-8',errors='replace')
        row['syntaxExit']=result.returncode
        (generated/f'native-set-select-syntax-{index}.txt').write_text('[DEBUG] Native SET syntax only\n'+result.stderr,encoding='utf-8')
    rows.append(row)
assert all(row.get('syntaxExit',0)==0 for row in rows)
manifest={'nativeExecuted':False,'nativeTypechecked':False,'files':rows}
(generated/'native-type-set-select-source.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
result=subprocess.run(['cargo','test','--manifest-path',str(editor.parent/'📦️packages/🦀️rust/Cargo.toml'),'--lib','original_set_select'],cwd=root,capture_output=True,text=True,encoding='utf-8',errors='replace')
(generated/'native-type-set-select-cargo.txt').write_text('[DEBUG] Authentic full SET native selector\n'+result.stdout+result.stderr,encoding='utf-8')
assert all(hashlib.sha256((root/row['path']).read_bytes()).hexdigest()==row['sha256']for row in rows)
print('[DEBUG]',json.dumps({'sourceFiles':len(rows),'ownedFiles':len(owned),'syntaxFiles':sum('syntaxExit'in row for row in rows),'syntaxExit':0,'maximumPath':max(row['utf16']for row in rows),'authenticCargoExit':result.returncode,'nativeExecuted':False,'nativeTypechecked':False}))
