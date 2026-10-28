from pathlib import Path
import json
base=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts')
I=base/'🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any'
B=base/'🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base'
T=base/'📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base'
def edit(p,pairs):
 s=p.read_text(encoding='utf-8')
 for old,new in pairs:
  assert old in s,(str(p),old)
  s=s.replace(old,new)
 p.write_text(s,encoding='utf-8',newline='\n')
V='semio_s_artifact_stdio_contract::part21::is_part21_binary'
edit(T/'🧬️schema/📸️snapshot/🦀️.rs',[('    String(String),','    String(String),\n    Binary(String),'),('        Part21Value::Str(s) => StepValue::String(s.clone()),','        Part21Value::Str(s) => StepValue::String(s.clone()),\n        Part21Value::Binary(s) => StepValue::Binary(s.clone()),'),('        StepValue::String(s) => Part21Value::Str(s.clone()),','        StepValue::String(s) => Part21Value::Str(s.clone()),\n        StepValue::Binary(s) => Part21Value::Binary(s.clone()),')])
edit(T/'🧬️schema/📸️snapshot/🟦️.ts',[('  | { string: string }','  | { string: string }\n  | { binary: string }')])
edit(B/'🧬️schema/📸️snapshot/🟦️.ts',[("  | { kind: 'str'; value: string }","  | { kind: 'str'; value: string }\n  | { kind: 'binary'; value: string }")])
edit(T/'🚪️io/📝️text/🔺️diff/🦀️.rs',[('        StepValue::String(s) => format!("S[{}]", enc_str(s)),','        StepValue::String(s) => format!("S[{}]", enc_str(s)),\n        StepValue::Binary(s) => format!("B[{}]", enc_str(s)),'),('        "S" => Ok(StepValue::String(dec_str(inner)?)),',f'        "S" => Ok(StepValue::String(dec_str(inner)?)),\n        "B" => {{let content=dec_str(inner)?;if !{V}(&content){{return Err("invalid canonical STEP binary content".into());}}Ok(StepValue::Binary(content))}},')])
edit(B/'🚪️io/📝️text/🔺️diff/🦀️.rs',[('        Part21Value::Str(s) => {','        Part21Value::Binary(s) => {out.push_str("B[");hex_encode_into(s.as_bytes(), out);out.push(\']\');}\n        Part21Value::Str(s) => {'),('        "S" => Ok(Part21Value::Str(dec_str(inner)?)),',f'        "S" => Ok(Part21Value::Str(dec_str(inner)?)),\n        "B" => {{let content=dec_str(inner)?;if !{V}(&content){{return Err("invalid canonical STEP binary content".into());}}Ok(Part21Value::Binary(content))}},')])
for root,enum,variant in [(T,'StepValue','String'),(B,'Part21Value','Str')]:
 edit(root/'🚪️io/💾️binary/🔺️diff/🦀️.rs',[(f'        {enum}::{variant}(s) => {{',f'        {enum}::Binary(s) => {{out.push(9);write_str_bin(out,s);}}\n        {enum}::{variant}(s) => {{'),(f'        4 => Ok({enum}::{variant}(read_str_bin(reader)?)),',f'        4 => Ok({enum}::{variant}(read_str_bin(reader)?)),\n        9 => {{let content=read_str_bin(reader)?;if !{V}(&content){{return Err("invalid canonical STEP binary content".into());}}Ok({enum}::Binary(content))}},')])
 S=root/'🚪️io/🪶️sqlite/📸️snapshot'
 for p in [S/'🦀️.rs',S/'📏️encoding/💰️backing/🦀️.rs']:
  pairs=[(f'{enum}::{variant}(_)=>"'+('str' if root==B else 'string')+'",',f'{enum}::Binary(_)=>"binary",{enum}::{variant}(_)=>"'+('str' if root==B else 'string')+'",')]
  if p.parent==S: pairs += [(f'{enum}::{variant}(v)=>cells['+('6' if root==B else '3')+']=Cell::Text(v),',f'{enum}::{variant}(v)|{enum}::Binary(v)=>cells['+('6' if root==B else '3')+']=Cell::Text(v),')]
  else: pairs += [(f'{enum}::{variant}(value)|{enum}::Enum(value)',f'{enum}::{variant}(value)|{enum}::Binary(value)|{enum}::Enum(value)')]
  edit(p,pairs)
 edit(S/'🚦️native/🦀️.rs',[(f'{enum}::{variant}(value)=>{{node.text=Some(control.copy_text(value)?);("string",&[][..])}},',f'{enum}::{variant}(value)=>{{node.text=Some(control.copy_text(value)?);("string",&[][..])}},{enum}::Binary(value)=>{{if !{V}(value){{return Err(invalid("invalid canonical STEP binary content"));}}node.text=Some(control.copy_text(value)?);("binary",&[][..])}},'),('"string"|"enum"=>','"string"|"enum"|"binary"=>'),('if !valid||node.children.iter()',f'if (node.kind=="binary"&&!node.text.as_deref().is_some_and({V}))||!valid||node.children.iter()'),(f'"string"=>{enum}::{variant}('+('text.unwrap()' if root==B else 'node.text.take().unwrap()')+'),',f'"binary"=>{enum}::Binary('+('text.unwrap()' if root==B else 'node.text.take().unwrap()')+'),'+f'"string"=>{enum}::{variant}('+('text.unwrap()' if root==B else 'node.text.take().unwrap()')+'),')])
 edit(S/'🧮️semantic/🦀️.rs',[('"string"|"enum"=>literal.is_some()',f'"binary"=>literal.is_some_and({V})&&payloads==1&&children.is_empty(),"string"|"enum"=>literal.is_some()')])
 col=7 if root==B else 4
 if root==B: act=[('("str",7)','("str"|"binary",7)')]
 else: act=[('"string"=>Some(4)','"string"|"binary"=>Some(4)')]
 edit(S/'🛬️reconstruction/🦀️.rs',act+[(f'"'+('str' if root==B else 'string')+f'"=>{enum}::{variant}(control.copy_text(row.text({col})?)?),',f'"binary"=>{{let content=row.text({col})?;if !{V}(content){{return Err(invalid("invalid canonical STEP binary content"))}}{enum}::Binary(control.copy_text(content)?)}},"'+('str' if root==B else 'string')+f'"=>{enum}::{variant}(control.copy_text(row.text({col})?)?),')])
 for p in [S/'🗄️.sql',S/'🟦️.ts']:
  tag='str' if root==B else 'string'
  edit(p,[(f"'{tag}','enum'",f"'{tag}','binary','enum'"),(f"(kind = '{tag}') = (string_value IS NOT NULL)",f"(kind IN ('{tag}','binary')) = (string_value IS NOT NULL)")])
 validator='function binary(value:string):string{if(typeof value!=="string"||!/^(?:0[0-9A-F]*|1[0-7][0-9A-F]*|2[0-3][0-9A-F]*|3[01][0-9A-F]*)$/.test(value))throw Error("invalid canonical STEP binary content");return value;}\n'
 edit(S/'🟦️.ts',[('function unsigned(value:bigint)',validator+'function unsigned(value:bigint)')])
 if root==B:
  edit(S/'🟦️.ts',[('case"str":cells[6]=value.value;break;','case"str":cells[6]=value.value;break;case"binary":cells[6]=binary(value.value);break;'),('kind==="str"&&i===7','(kind==="str"||kind==="binary")&&i===7'),('case"str":value={kind,value:artifactSqliteText(row,7)};break;','case"str":value={kind,value:artifactSqliteText(row,7)};break;case"binary":value={kind,value:binary(artifactSqliteText(row,7))};break;')])
 else:
  edit(S/'🟦️.ts',[('else if("string" in v){cells[1]="string";cells[4]=v.string;}','else if("string" in v){cells[1]="string";cells[4]=v.string;}\n    else if("binary" in v){cells[1]="binary";cells[4]=binary(v.binary);}'),('kind==="string"?4:','(kind==="string"||kind==="binary")?4:'),('case "string":value={string:artifactSqliteText(row,4)};break;','case "string":value={string:artifactSqliteText(row,4)};break;case "binary":value={binary:binary(artifactSqliteText(row,4))};break;')])
p=base/'🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🏗️ifc/🔖️4/✳️any/🦀️.rs'
edit(p,[('Part21Value::List(_) | Part21Value::Ref(_) | Part21Value::Unset | Part21Value::Derived => None,','Part21Value::Binary(_) | Part21Value::List(_) | Part21Value::Ref(_) | Part21Value::Unset | Part21Value::Derived => None,')])
p=Path('✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21/🧪️tests/🚦️sqlite-cohort/🦀️.rs');edit(p,[('        Part21Value::Str(text) => serde_json::json!({"kind":"str","value":text}),','        Part21Value::Str(text) => serde_json::json!({"kind":"str","value":text}),\n        Part21Value::Binary(text) => serde_json::json!({"kind":"binary","value":text}),')])
print('Updated STEP and IFC2x3 exhaustive scalar persistence paths.')
