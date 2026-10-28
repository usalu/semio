from pathlib import Path
import json
A=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts');I=A/'🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any';T=A/'📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base';B=A/'🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base'
p=I/'🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts';s=p.read_text(encoding='utf-8');s+='''

import BinaryAjv from "ajv";
test("IFC binary transport schema independently enforces canonical STEP padding",async()=>{
 const schema=await Bun.file(new URL("../../../../🧬️schema/📸️snapshot/🔣️.json",import.meta.url)).json();const validate=new BinaryAjv({strict:false}).compile({$defs:schema.$defs,$ref:"#/$defs/IfcValue"});
 for(const value of binaryCases.canonical)expect(validate({kind:"binary",value})).toBe(true);
 for(const value of binaryCases.invalid)expect(validate({kind:"binary",value})).toBe(false);
});
''';p.write_text(s,encoding='utf-8',newline='\n')
p=Path('✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21/🚦️controlled/🧪️tests/🧫️fixtures/🔣️.json');d=json.loads(p.read_text(encoding='utf-8'));d['cases'].append({'name':'canonical-binary-padding','kind':'value','value':{'kind':'binary','value':'17'}});p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n',encoding='utf-8',newline='\n')
p=B/'🧬️schema/🔣️.json';d=json.loads(p.read_text(encoding='utf-8'));d['$defs']['Part21Value']['properties']['kind']['x-semio-ui']['options']['binary']={'en':'Binary','de':'Binär'};p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n',encoding='utf-8',newline='\n')
for p in [I/'🚪️io/💾️binary/🔺️diff/🦀️.rs',T/'🚪️io/💾️binary/🔺️diff/🦀️.rs']:
 s=p.read_text(encoding='utf-8').replace('same 0-8','same 0-9');p.write_text(s,encoding='utf-8',newline='\n')
p=I/'🧬️schema/🧬️mutations/🦀️.rs';s=p.read_text(encoding='utf-8').replace('7 of its 9 variants','8 of its 10 variants');p.write_text(s,encoding='utf-8',newline='\n')
