"""🧭️ Census the original recursive Value source without substituting parser evidence for native proof."""
from pathlib import Path
import hashlib,json,subprocess
T=Path(__file__).parent
R=T.parents[6]
G=T/'🗑️generated'
I=R/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc'
D=I/'🔎️details'
E=I/'🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor'
baseline=json.loads((G/'native-type-measure-select-source.json').read_text(encoding='utf-8'))['files']
owned=[D/n for n in ['🦀️.rs','type_input_select.rs','type_input_kind.rs','type_input.rs','type_input_list_copy.rs','type_input_list_copy_tests.rs','type_input_typed_copy.rs','type_input_value_copy_tests.rs','type_input_set_controls.rs','type-guidance.rs','type_value_select_input_tests.rs','work_tests.rs']]+[E/'guided-type-value-select-tests.rs',E/'render-work-tests.rs']
files=[R/row['path']for row in baseline]+owned+[D/n for n in ['type_value_select_input.test.ts','🧪️tests/type-value-select-input.json','🧪️tests/type-value-select-input.schema.json','🧪️tests/type-value-select-input.feature']]
rows=[]
for index,path in enumerate(dict.fromkeys(files)):
 row={'path':str(path.relative_to(R)).replace('\\','/'),'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'utf16':len(str(path).encode('utf-16-le'))//2,'scope':'owned-native-value'if path in owned else'reference-or-closed-ts-law'}
 assert row['utf16']<=256
 if path.suffix=='.rs':
  answer=subprocess.run(['rustfmt','--edition','2024','--config','skip_children=true','--emit','stdout',str(path)],capture_output=True,text=True,encoding='utf-8',errors='replace')
  row['syntaxExit']=answer.returncode
  (G/f'native-value-select-syntax-{index}.txt').write_text('[DEBUG] Syntax parse only\n'+answer.stderr,encoding='utf-8')
 rows.append(row)
assert all(row.get('syntaxExit',0)==0 for row in rows)
answer=subprocess.run(['cargo','test','--manifest-path',str(E.parent/'📦️packages/🦀️rust/Cargo.toml'),'--lib','recursive'],cwd=R,capture_output=True,text=True,encoding='utf-8',errors='replace')
(G/'native-type-value-select-cargo.txt').write_text('[DEBUG] Authentic native recursive Value selector\n'+answer.stdout+answer.stderr,encoding='utf-8')
assert all(hashlib.sha256((R/row['path']).read_bytes()).hexdigest()==row['sha256']for row in rows)
data={'nativeExecuted':False,'nativeTypechecked':False,'physicalAccountingProved':False,'files':rows,'authenticCargoExit':answer.returncode}
(G/'native-type-value-select-source.json').write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG]',json.dumps({'files':len(rows),'owned':len(owned),'references':len(rows)-len(owned),'syntaxFiles':sum('syntaxExit'in row for row in rows),'syntaxExit':0,'maximumPath':max(row['utf16']for row in rows),'authenticCargoExit':answer.returncode,'nativeExecuted':False,'nativeTypechecked':False}))
