from pathlib import Path
P=Path('✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21')
I=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any')
changed=[]
def edit(p,pairs):
 s=p.read_text(encoding='utf-8')
 for old,new in pairs:
  assert old in s,(str(p),old)
  s=s.replace(old,new)
 p.write_text(s,encoding='utf-8');changed.append(str(p))
edit(P/'🔢️values/🦀️.rs', [('    Str(String),','    Str(String),\n    Binary(String),'),('impl Part21Value {','''/// 🧮️ Recognizes canonical STEP binary content: unused high bits followed by uppercase hexadecimal digits.
pub fn is_part21_binary(value: &str) -> bool {
    let bytes = value.as_bytes();
    let Some(&unused @ b'0'..=b'3') = bytes.first() else { return false; };
    if bytes.len() == 1 { return unused == b'0'; }
    if !bytes[1..].iter().all(|value| value.is_ascii_digit() || (b'A'..=b'F').contains(value)) { return false; }
    let first = if bytes[1].is_ascii_digit() { bytes[1] - b'0' } else { bytes[1] - b'A' + 10 };
    first < (1u8 << (4 - (unused - b'0')))
}

impl Part21Value {''')])
edit(P/'🔤️lexer-and-parse/🦀️.rs', [('    fn read_value(&mut self)', '''    fn read_binary(&mut self) -> Result<Part21Value, Part21Error> {
        let start = self.pos;
        self.pos += 1;
        let mut content = String::new();
        loop {
            match self.bump() {
                Some('"') => break,
                Some(value) if value.is_ascii_hexdigit() => content.push(value.to_ascii_uppercase()),
                Some(found) => return Err(Part21Error::UnexpectedChar { at: self.pos - 1, found, expected: "STEP binary hexadecimal digit or closing quote" }),
                None => return Err(Part21Error::UnexpectedEof { at: self.pos, expected: "closing binary quote" }),
            }
        }
        if !is_part21_binary(&content) { return Err(Part21Error::InvalidNumber { at: start, text: content }); }
        Ok(Part21Value::Binary(content))
    }

    fn read_value(&mut self)'''),("            Some('.') => self.read_enum().map(Part21Value::Enum),", "            Some('.') => self.read_enum().map(Part21Value::Enum),\n            Some('\"') => self.read_binary(),")])
edit(P/'✍️writer/🦀️.rs', [('        Part21Value::Enum(name) => {','''        Part21Value::Binary(content) => {
            out.push('"');
            out.push_str(content);
            out.push('"');
        }
        Part21Value::Enum(name) => {''')])
edit(P/'🚦️controlled/🛫️output/🦀️.rs', [('                Part21Value::Str(_) => ("str", 2),','                Part21Value::Str(_) => ("str", 2),\n                Part21Value::Binary(_) => ("binary", 2),'),('Part21Value::Str(value) | Part21Value::Enum(value)','Part21Value::Str(value) | Part21Value::Binary(value) | Part21Value::Enum(value)')])
edit(P/'🚦️controlled/🛬️input/🦀️.rs', [('"ref"|"str"|"enum"|"int"|"real"','"ref"|"str"|"binary"|"enum"|"int"|"real"'),('    "enum"=>Part21Value::Enum', '''    "binary"=>{let value=required(entries,"value",control)?;if !value.as_str().is_some_and(is_part21_binary){return Err(invalid("invalid canonical STEP binary content"));}Part21Value::Binary(self.text(value,control)?)},
    "enum"=>Part21Value::Enum''')])
edit(P/'🧾️value-and-canonical-codecs/🦀️.rs', [('            Self::Str(text) => tagged("str", vec![("value", text.to_value())]),','            Self::Str(text) => tagged("str", vec![("value", text.to_value())]),\n            Self::Binary(text) => tagged("binary", vec![("value", text.to_value())]),'),('            "enum" => Self::Enum', '''            "binary" => { let content=String::from_value(part21_member(&mut entries, &record, "value")?)?;if !is_part21_binary(&content){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"invalid canonical STEP binary content"));}Self::Binary(content) },
            "enum" => Self::Enum'''),('static PART21_KINDS: [&str; 9]', 'static PART21_KINDS: [&str; 10]'),('"typed", "unset", "derived"]','"typed", "unset", "derived", "binary"]'),('            Self::Derived => 8,','            Self::Derived => 8,\n            Self::Binary(_) => 9,'),('Self::Str(text) | Self::Enum(text)','Self::Str(text) | Self::Binary(text) | Self::Enum(text)'),('Self::Ref(_) | Self::Str(_) | Self::Enum(_)','Self::Ref(_) | Self::Str(_) | Self::Binary(_) | Self::Enum(_)')])
edit(I/'🧬️schema/📸️snapshot/🦀️.rs', [('    String(String),','    String(String),\n    Binary(String),'),('        Part21Value::Str(s) => IfcValue::String(s.clone()),','        Part21Value::Str(s) => IfcValue::String(s.clone()),\n        Part21Value::Binary(s) => IfcValue::Binary(s.clone()),'),('        IfcValue::String(s) => Part21Value::Str(s.clone()),','        IfcValue::String(s) => Part21Value::Str(s.clone()),\n        IfcValue::Binary(s) => Part21Value::Binary(s.clone()),')])
edit(I/'🚪️io/📝️text/🔺️diff/🦀️.rs', [('        IfcValue::String(s) => format!("S[{}]", enc_str(s)),','        IfcValue::String(s) => format!("S[{}]", enc_str(s)),\n        IfcValue::Binary(s) => format!("B[{}]", enc_str(s)),'),('        "S" => Ok(IfcValue::String(dec_str(inner)?)),','''        "S" => Ok(IfcValue::String(dec_str(inner)?)),
        "B" => {let content=dec_str(inner)?;if !semio_s_artifact_stdio_contract::part21::is_part21_binary(&content){return Err("invalid canonical IFC binary content".into());}Ok(IfcValue::Binary(content))},''')])
edit(I/'🚪️io/💾️binary/🔺️diff/🦀️.rs', [('        IfcValue::String(s) => {','        IfcValue::Binary(s) => { out.push(9); write_str_bin(out, s); }\n        IfcValue::String(s) => {'),('        4 => Ok(IfcValue::String(read_str_bin(reader)?)),','''        4 => Ok(IfcValue::String(read_str_bin(reader)?)),
        9 => {let content=read_str_bin(reader)?;if !semio_s_artifact_stdio_contract::part21::is_part21_binary(&content){return Err("invalid canonical IFC binary content".into());}Ok(IfcValue::Binary(content))},''')])
S=I/'🚪️io/🪶️sqlite/📸️snapshot'
for p in [S/'🦀️.rs',S/'📏️encoding/💰️backing/🦀️.rs']:
 pairs=[('IfcValue::String(_)=>"string",','IfcValue::String(_)=>"string",IfcValue::Binary(_)=>"binary",')]
 if p.name=='🦀️.rs' and p.parent==S: pairs += [('IfcValue::String(v)=>cells[3]=Cell::Text(v),','IfcValue::String(v)|IfcValue::Binary(v)=>cells[3]=Cell::Text(v),')]
 else: pairs += [('IfcValue::String(value)|IfcValue::Enum(value)','IfcValue::String(value)|IfcValue::Binary(value)|IfcValue::Enum(value)')]
 edit(p,pairs)
edit(S/'🚦️native/🦀️.rs',[('IfcValue::String(value)=>{node.text=Some(control.copy_text(value)?);("string",&[][..])},','IfcValue::String(value)=>{node.text=Some(control.copy_text(value)?);("string",&[][..])},IfcValue::Binary(value)=>{if !semio_s_artifact_stdio_contract::part21::is_part21_binary(value){return Err(invalid("invalid canonical IFC binary content"));}node.text=Some(control.copy_text(value)?);("binary",&[][..])},'),('"string"|"enum"=>node.text.is_some()','"binary"=>node.text.as_deref().is_some_and(semio_s_artifact_stdio_contract::part21::is_part21_binary)&&payloads==1&&node.children.is_empty(),"string"|"enum"=>node.text.is_some()'),('"string"=>IfcValue::String(node.text.take().unwrap()),','"string"=>IfcValue::String(node.text.take().unwrap()),"binary"=>IfcValue::Binary(node.text.take().unwrap()),')])
edit(S/'🧮️semantic/🦀️.rs',[('"string"|"enum"=>literal.is_some()', '"binary"=>literal.is_some_and(semio_s_artifact_stdio_contract::part21::is_part21_binary)&&payloads==1&&children.is_empty(),"string"|"enum"=>literal.is_some()')])
edit(S/'🛬️reconstruction/🦀️.rs',[('"string"=>Some(4),','"string"|"binary"=>Some(4),'),('"string"=>IfcValue::String(control.copy_text(row.text(4)?)?),','"string"=>IfcValue::String(control.copy_text(row.text(4)?)?),"binary"=>{let content=row.text(4)?;if !semio_s_artifact_stdio_contract::part21::is_part21_binary(content){return Err(invalid("invalid canonical IFC binary content"))}IfcValue::Binary(control.copy_text(content)?)},')])
for p in [S/'🗄️.sql',S/'🟦️.ts']:
 edit(p,[("'real','string','enum'","'real','string','binary','enum'"),("(kind = 'string') = (string_value IS NOT NULL)","(kind IN ('string','binary')) = (string_value IS NOT NULL)")])
edit(S/'🟦️.ts', [('function signed(value:bigint)', '''function binary(value:string):string{if(typeof value!=="string"||!/^(?:0[0-9A-F]*|1[0-7][0-9A-F]*|2[0-3][0-9A-F]*|3[01][0-9A-F]*)$/.test(value))throw Error("invalid canonical IFC binary content");return value;}
function signed(value:bigint)'''),('case"string":cells[4]=value.value;break;', 'case"string":cells[4]=value.value;break;case"binary":cells[4]=binary(value.value);break;'),('kind==="string"&&i===4','(kind==="string"||kind==="binary")&&i===4'),('case"string":value={kind,value:artifactSqliteText(row,4)};break;','case"string":value={kind,value:artifactSqliteText(row,4)};break;case"binary":value={kind,value:binary(artifactSqliteText(row,4))};break;')])
C=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🦀️.rs')
edit(C,[('        ("BOOLEAN", IfcValue::Enum(value))','        ("BINARY", IfcValue::Binary(value)) => semio_s_artifact_stdio_contract::part21::is_part21_binary(value),\n        ("BOOLEAN", IfcValue::Enum(value))')])
print('\n'.join(changed))
