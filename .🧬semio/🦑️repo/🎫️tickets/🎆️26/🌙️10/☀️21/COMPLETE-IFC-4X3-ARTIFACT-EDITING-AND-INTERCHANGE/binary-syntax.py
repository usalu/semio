from pathlib import Path
import json,subprocess
A=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts');T=A/'📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base';B=A/'🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base'
pattern='^(?:0[0-9A-F]*|1[0-7][0-9A-F]*|2[0-3][0-9A-F]*|3[01][0-9A-F]*)$'
def edit(p,pairs):
 s=p.read_text(encoding='utf-8')
 for old,new in pairs:
  assert old in s,(str(p),old)
  s=s.replace(old,new)
 p.write_text(s,encoding='utf-8',newline='\n')
for p in T.rglob('*.json'):
 try:d=json.loads(p.read_text(encoding='utf-8'))
 except:continue
 dirty=False
 def visit(node):
  global dirty
  if isinstance(node,dict):
   for k,v in list(node.items()):
    if k=='StepValue' and isinstance(v,dict) and 'oneOf' in v and not any('binary' in x.get('properties',{})for x in v['oneOf']):
     v['oneOf'].insert(5,{'type':'object','required':['binary'],'additionalProperties':False,'properties':{'binary':{'type':'string','pattern':pattern}}});dirty=True
    visit(v)
  elif isinstance(node,list):
   for x in node:visit(x)
 visit(d)
 if dirty:p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n',encoding='utf-8',newline='\n')
edit(T/'🧬️schema/📸️snapshot/🛰️.proto',[('    StepTypedValue typed_value = 9;','    StepTypedValue typed_value = 9;\n    string binary_value = 10;')])
edit(T/'🧬️schema/📸️snapshot/🔗️.graphql',[('  | StepValueString','  | StepValueString\n  | StepValueBinary'),('type StepValueString { string: String! }','type StepValueString { string: String! }\ntype StepValueBinary { binary: String! }')])
edit(B/'🧬️schema/📸️snapshot/🛰️.proto',[('DERIVED = 8;','DERIVED = 8; BINARY = 9;'),('string type_name = 7;','string type_name = 7; string binary_value = 8;')])
edit(B/'🧬️schema/📸️snapshot/🔗️.graphql',[('TYPED UNSET DERIVED }','TYPED UNSET DERIVED BINARY }'),('stringValue: String intValue','stringValue: String binaryValue: String intValue')])
p=B/'🧬️schema/🔣️.json';d=json.loads(p.read_text(encoding='utf-8'));v=d['$defs']['Part21Value'];v['properties']['kind']['enum'].append('binary');p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n',encoding='utf-8',newline='\n')
roots=[T,B,A/'🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any',Path('✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21'),A/'🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog']
files=[p for root in roots for p in root.rglob('*.rs')if 'Binary('in p.read_text(encoding='utf-8')]
fail=[]
for p in files:
 r=subprocess.run(['rustfmt','--emit','stdout','--config','skip_children=true','--edition','2024',str(p)],stdout=subprocess.DEVNULL,stderr=subprocess.PIPE,text=True,encoding='utf-8')
 if r.returncode:fail.append((str(p),r.stderr))
print('Rust syntax parse files:',len(files),'failures:',len(fail))
for p,error in fail:print(p,error)
