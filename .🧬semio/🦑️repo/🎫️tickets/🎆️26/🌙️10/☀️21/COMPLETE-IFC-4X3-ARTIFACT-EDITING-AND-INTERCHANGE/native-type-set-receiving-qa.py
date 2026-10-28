"""🧺️ Record the partial SET receiver/copier checkpoint without claiming native execution."""
from pathlib import Path
import hashlib,json,subprocess
ticket=Path(__file__).parent
root=ticket.parents[6]
generated=ticket/'🗑️generated'
ifc=root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc'
details=ifc/'🔎️details'
editor=ifc/'🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor'
files=[details/name for name in ['type_input_kind.rs','type_input.rs','type_input_typed_copy.rs','type_input_typed_copy_tests.rs','type_input_set_copy.rs','type_input_set_copy_tests.rs','type_set_select_input_tests.rs']]+[editor/'guided-type-set-select-tests.rs']
rows=[]
for index,path in enumerate(files):
    result=subprocess.run(['rustfmt','--edition','2024','--config','skip_children=true','--emit','stdout',str(path)],capture_output=True,text=True,encoding='utf-8',errors='replace')
    row={'path':str(path.relative_to(root)).replace('\\','/'),'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'utf16':len(str(path).encode('utf-16-le'))//2,'syntaxExit':result.returncode}
    assert row['utf16']<=256
    (generated/f'native-set-receiving-syntax-{index}.txt').write_text('[DEBUG] Partial SET syntax only\n'+result.stderr,encoding='utf-8')
    rows.append(row)
assert all(row['syntaxExit']==0 for row in rows)
(generated/'native-type-set-receiving-source.json').write_text(json.dumps({'completeProducer':False,'nativeExecuted':False,'files':rows},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
result=subprocess.run(['cargo','test','--manifest-path',str(editor.parent/'📦️packages/🦀️rust/Cargo.toml'),'--lib','original_set_select'],cwd=root,capture_output=True,text=True,encoding='utf-8',errors='replace')
(generated/'native-type-set-receiving-cargo.txt').write_text('[DEBUG] Authentic partial SET native selector\n'+result.stdout+result.stderr,encoding='utf-8')
assert all(hashlib.sha256((root/row['path']).read_bytes()).hexdigest()==row['sha256']for row in rows)
print('[DEBUG]',json.dumps({'files':len(rows),'syntaxExit':0,'authenticCargoExit':result.returncode,'completeProducer':False,'nativeExecuted':False}))
