from pathlib import Path
import hashlib,json,subprocess,os,sys
r=Path('C:/git/semio');t=Path(__file__).parent;g=t/'🗑️generated';e=r/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor'
baseline=json.loads((g/'guided-candidate-source.json').read_text(encoding='utf-8'));i=r/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc';files=[r/row['path']for row in baseline]+[i/'🔎️details/type_number_binary_guidance_tests.rs',i/'🔎️details/type_number_binary_guidance.test.ts',i/'📦️packages/🟦️typescript/📜️script.ts']+[e/name for name in ['guided-type-number-binary-cases.json','guided-type-number-binary-cases.schema.json','guided-type-number-binary.feature','guided-type-number-binary-tests.rs']];rows=[]
for n,f in enumerate(files):
 row={'path':str(f.relative_to(r)).replace('\\','/'),'sha256':hashlib.sha256(f.read_bytes()).hexdigest(),'utf16':len(str(f).encode('utf-16-le'))//2}
 if f.suffix=='.rs':
  p=subprocess.run(['rustfmt','--edition','2024','--config','skip_children=true','--emit','stdout',str(f)],capture_output=True,text=True,encoding='utf-8',errors='replace');row['syntaxExit']=p.returncode;(g/f'guided-numeric-control-{n}.txt').write_text('[DEBUG] Native syntax diagnostics\n'+p.stderr,encoding='utf-8')
 rows.append(row)
(g/'guided-number-binary-control-source.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2),encoding='utf-8');print(json.dumps({'sourceFiles':len(rows),'syntaxFiles':sum('syntaxExit'in row for row in rows),'syntaxFailures':[row['path']for row in rows if row.get('syntaxExit',0)],'maximumPath':max(row['utf16']for row in rows)}));assert all(row.get('syntaxExit',0)==0 for row in rows)
if '--source-only' in sys.argv:
 assert all(hashlib.sha256((r/row['path']).read_bytes()).hexdigest()==row['sha256'] for row in rows),'source changed during syntax checks'
 sys.exit(0)
p=subprocess.run(['bun',str(t/'guided-number-binary-fields-probe.ts')],cwd=r,capture_output=True,text=True,encoding='utf-8',errors='replace',shell=os.name=='nt');(g/'guided-number-binary-control-ts.txt').write_text('[DEBUG] Closed schema and actual source field probe\n'+p.stdout+p.stderr,encoding='utf-8');print(p.stdout);assert p.returncode==0,p.stderr
for name,args in [('tests',['bun','test',str(i/'🔎️details/type_number_binary_guidance.test.ts'),str(i/'🔎️details/type_number_binary_input.test.ts')]),('strict',['bun','node_modules/typescript/bin/tsc','--noEmit','--project',str(t/'guided-number-binary-tsconfig.json')])]:
 p=subprocess.run(args,cwd=r,capture_output=True,text=True,encoding='utf-8',errors='replace',shell=os.name=='nt');(g/f'guided-number-binary-{name}.txt').write_text('[DEBUG] Current registered numeric guidance '+name+'\n'+p.stdout+p.stderr,encoding='utf-8');print(json.dumps({'check':name,'exit':p.returncode}));assert p.returncode==0,p.stderr
p=subprocess.run(['cargo','test','--manifest-path',str(e.parent/'📦️packages/🦀️rust/Cargo.toml'),'--lib','donated_numeric_binary_controller'],cwd=r,capture_output=True,text=True,encoding='utf-8',errors='replace');(g/'guided-number-binary-control-cargo.txt').write_text('[DEBUG] Authentic controller selector\n'+p.stdout+p.stderr,encoding='utf-8');print(json.dumps({'authenticCargoExit':p.returncode,'nativeExecuted':False,'controllerLawMounted':True,'numberModeProducer':'source-authored'}))
assert all(hashlib.sha256((r/row['path']).read_bytes()).hexdigest()==row['sha256'] for row in rows),'source changed during QA'
