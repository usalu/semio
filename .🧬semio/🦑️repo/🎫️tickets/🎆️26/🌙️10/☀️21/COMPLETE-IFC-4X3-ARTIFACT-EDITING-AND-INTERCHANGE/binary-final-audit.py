from pathlib import Path
import subprocess,json
root=Path.cwd();ticket=root/'.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️21/COMPLETE-IFC-4X3-ARTIFACT-EDITING-AND-INTERCHANGE'
r=subprocess.run(['rg','-l','Part21Value::Derived|IfcValue::Derived|StepValue::Derived','✏️s/🔌️plugins/🗄️stdio','-g','*.rs'],capture_output=True,text=True,encoding='utf-8')
flags=[]
for name in r.stdout.splitlines():
 p=Path(name);s=p.read_text(encoding='utf-8')
 if '🧪️tests'in name or 'tests.rs'in name:continue
 for e,v in [('Part21Value','Str'),('IfcValue','String'),('StepValue','String')]:
  if f'{e}::{v}'in s and f'{e}::Derived'in s and f'{e}::Binary'not in s:flags.append((e,p))
print('Files requiring manual wildcard review:',len(flags))
for e,p in flags:print(e,str(p))
P=Path('✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21');A=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts');I=A/'🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any';B=A/'🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base';T=A/'📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base'
files=[P/x for x in ['🔢️values/🦀️.rs','🔤️lexer-and-parse/🦀️.rs','✍️writer/🦀️.rs','🚦️controlled/🛬️input/🦀️.rs','🚦️controlled/🛫️output/🦀️.rs','🧾️value-and-canonical-codecs/🦀️.rs','🧪️tests/🔬️unit/🦀️.rs','🧪️tests/🚦️sqlite-cohort/🦀️.rs','🚦️controlled/🧪️tests/🧫️fixtures/🔣️.json']]
for base in [I,B,T]:
 for suffix in ['🦀️.rs','📏️encoding/💰️backing/🦀️.rs','🚦️native/🦀️.rs','🧮️semantic/🦀️.rs','🛬️reconstruction/🦀️.rs','🟦️.ts','🗄️.sql','🧪️tests/🟦️.ts','🧪️tests/🦀️.rs','🧪️tests/🧫️fixtures/🎛️semantic.json']:
  files.append(base/'🚪️io/🪶️sqlite/📸️snapshot'/suffix)
 for suffix in ['🦀️.rs','🟦️.ts','🛰️.proto','🔗️.graphql','🔣️.json']:
  if base==B and suffix in ['🦀️.rs','🔣️.json']:continue
  if base==I and suffix=='🟦️.ts':continue
  files.append(base/'🧬️schema/📸️snapshot'/suffix)
 for kind in ['📝️text','💾️binary']:files.append(base/'🚪️io'/kind/'🔺️diff/🦀️.rs')
 for p in base.rglob('*.json'):
  try:d=json.loads(p.read_text(encoding='utf-8'))
  except:continue
  enum='IfcValue'if base==I else 'StepValue'if base==T else 'Part21Value'
  if enum in d.get('$defs',{}) and any('binary'in str(x)for x in d['$defs'][enum].get('oneOf',[])):files.append(p)
files+=[I/'🧬️schema/🧬️mutations/🦀️.rs',I/'🧬️schema/🔺️diff/🦀️.rs',B/'🧬️schema/🔣️.json',A/'🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🦀️.rs',A/'🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🏗️ifc/🔖️4/✳️any/🦀️.rs']
files +=[I/'🚪️io/🪶️sqlite/📸️snapshot/🧪️tests'/name for name in ['binary-cases.json','binary.feature']]
files +=[base/'🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/binary.test.ts' for base in [B,T]]
files=sorted(set(files),key=str)
assert all(p.exists()for p in files)
assert all(len(str(root/p))<=256 for p in files)
(ticket/'binary-files.md').write_text('# Binary Primitive Files\n\nThese paths contain this worker\'s edits; shared files may also contain concurrently owned changes.\n\n'+''.join('- `'+p.as_posix()+'`\n'for p in files),encoding='utf-8',newline='\n')
print('Recorded source/fixture paths:',len(files),'max characters:',max(len(str(root/p))for p in files))
