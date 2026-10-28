from pathlib import Path
home=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details')
p=home/'type_input_kind.rs';s=p.read_text(encoding='utf8').replace('Clone,Copy,Default,Debug,PartialEq,Eq','Clone,Copy,Default,Debug,PartialEq,Eq,semio_framework_value::RetireOwned');p.open('r+b').write(s.encode())
p=home/'type-guidance.rs';s=p.read_text(encoding='utf8')
s=s.replace('use super::*;','use super::*;\nuse super::type_input_kind::{Kind as InputKind,Cursor as InputKindCursor};',1)
a=s.index('fn row_text(');b=s.index('fn row_value_mut(',a)
s=s[:a]+'''fn row_data(request:&StagedFormRequest,ordinal:usize)->Result<(usize,InputKind,&str),ValueError>{
    let DslValue::Object(fields)=rows(request)?.get(ordinal).ok_or_else(invalid)? else{return Err(invalid())};if fields.len()!=2{return Err(invalid());}
    let index=fields.iter().find(|(key,_)|key=="index").and_then(|(_,value)|value.as_str()).and_then(|text|canonical_decimal(text,false)).and_then(|index|usize::try_from(index).ok()).ok_or_else(invalid)?;
    let DslValue::Object(value)=&fields.iter().find(|(key,_)|key=="value").ok_or_else(invalid)?.1 else{return Err(invalid())};if value.len()!=2{return Err(invalid());}
    let kind=match value.iter().find(|(key,_)|key=="kind").and_then(|(_,value)|value.as_str()){Some("reference")=>InputKind::Reference,Some("string")=>InputKind::String,Some("integer")=>InputKind::Integer,_=>return Err(invalid())};
    let text=value.iter().find(|(key,_)|key=="value").and_then(|(_,value)|value.as_str()).ok_or_else(invalid)?;Ok((index,kind,text))
}
fn row_text(request:&StagedFormRequest,ordinal:usize)->Result<(usize,&str),ValueError>{let(index,_,text)=row_data(request,ordinal)?;Ok((index,text))}
fn signed_decimal(text:&str)->Option<i64>{let digits=text.strip_prefix('-').unwrap_or(text);if text.len()>20||digits.is_empty()||!digits.bytes().all(|byte|byte.is_ascii_digit())||digits.len()>1&&digits.starts_with('0')||text=="-0"{return None;}text.parse().ok()}
''' + s[b:]
s=s.replace('    present: bool,\n}', '    present: bool,\n    kind: InputKind,\n}')
s=s.replace('let label = match (field.attribute.as_str(), request.locale) {','let label = match (field.kind,field.attribute.as_str(), request.locale) {').replace('("Profile", Locale::En)', '(InputKind::Reference,"Profile", Locale::En)').replace('("Profile", Locale::De)', '(InputKind::Reference,"Profile", Locale::De)').replace('(_, Locale::En) => "Required reference",','(InputKind::Reference,_, Locale::En) => "Required reference",').replace('(_, Locale::De) => "Erforderliche Referenz",','(InputKind::Reference,_, Locale::De) => "Erforderliche Referenz",\n            (InputKind::String,_,Locale::En)=>"Required text",(InputKind::String,_,Locale::De)=>"Erforderlicher Text",(InputKind::Integer,_,Locale::En)=>"Required integer",(InputKind::Integer,_,Locale::De)=>"Erforderliche Ganzzahl",')
s=s.replace('    row_part: usize,\n}', '    row_part: usize,\n    kinds: [InputKind;16],\n}')
s=s.replace('    row_parts: ManuallyDrop<Option<RowParts>>,', '    row_parts: ManuallyDrop<Option<RowParts>>,\n    scalar: ManuallyDrop<Option<String>>,\n    kind_cursor: InputKindCursor,')
s=s.replace('            row_parts: ManuallyDrop::new(None),', '            row_parts: ManuallyDrop::new(None),\n            scalar: ManuallyDrop::new(None),\n            kind_cursor:Default::default(),')
s=s.replace('''            4 => demand.copy_bytes += size_of::<Field>() + size_of::<super::type_input::Input>(),''','''            4 => demand.copy_bytes += size_of::<Field>() + size_of::<super::type_input::Input>(),
            30=>{demand.copy_bytes+=size_of::<String>();demand.capacity_bytes=row_data(self.request.as_ref().ok_or_else(stale)?,self.fields.as_ref().and_then(|fields|fields.last()).ok_or_else(invalid)?.row)?.2.len();},
            31=>demand.copy_bytes+=192+size_of::<String>(),
            32=>demand.copy_bytes+=2*size_of::<String>()+size_of::<IfcValue>(),''')
s=s.replace('let (index, text) = self.copying_row()?;', 'let (index, _, text) = self.copying_row()?;')
s=s.replace('let (_, text) = self.copying_row()?;', 'let (_, _, text) = self.copying_row()?;')
s=s.replace('fn copying_row(&self) -> Result<(usize, &str), ValueError>', 'fn copying_row(&self) -> Result<(usize, InputKind, &str), ValueError>')
s=s.replace('return Ok((field.index, ""));','return Ok((field.index, field.kind, ""));')
s=s.replace('''        row_text(self.page.as_ref().ok_or_else(stale)?.request.as_ref().ok_or_else(stale)?, ordinal)''','''        row_data(self.page.as_ref().ok_or_else(stale)?.request.as_ref().ok_or_else(stale)?, ordinal)''')
s=s.replace('let (index, text) = self.copying_row()?;', 'let (index, kind, text) = self.copying_row()?;')
# The earlier projection replacement also touches row_part_text.
s=s.replace('''    fn row_part_text<'a>(&'a self, buffer: &'a mut [u8; 20]) -> Result<&'a str, ValueError> {
        let (index, _, text) = self.copying_row()?;''','''    fn row_part_text<'a>(&'a self, buffer: &'a mut [u8; 20]) -> Result<&'a str, ValueError> {
        let (index, kind, text) = self.copying_row()?;''')
s=s.replace('            3 => "reference",','            3 => kind.tag(),')
a=s.index('                    let domain = attribute.type_expr.projection();',s.index('fn advance_inner'));b=s.index('                } else {',a)
s=s[:a]+'''                    let mut cursor=self.kind_cursor;let result=cursor.advance::<C>(attribute.type_expr);let kind=result.as_ref().ok().copied().flatten();
                    if !metadata(cx,base+size_of::<super::type_input::Input>()+2*size_of::<InputKindCursor>(),||{self.kind_cursor=cursor;if let Some(kind)=kind{let index_in_roster=self.placeholders.as_ref().unwrap().len();self.state.kinds[index_in_roster]=kind;self.placeholders.as_mut().unwrap().push(super::type_input::Input{index,value:Some(IfcValue::Unset)});self.kind_cursor.clear();}})?{return Ok(false);}result?;
''' + s[b:]
# discovery demand includes its actual new cursor pair.
s=s.replace('demand.copy_bytes += super::type_remap::Plan::copy_bytes() + size_of::<super::type_input::Input>();','demand.copy_bytes += super::type_remap::Plan::copy_bytes() + size_of::<super::type_input::Input>()+2*size_of::<InputKindCursor>();')
a=s.index('                let current = if self.state.row < rows(request)?.len()');b=s.index('            22 => {',a)
s=s[:a]+'''                let current=if self.state.row<rows(request)?.len(){Some(row_data(request,self.state.row)?)}else{None};if current.is_some_and(|(ordinal,_,_)|ordinal<index){return Err(invalid().into());}
                let present=current.is_some_and(|(ordinal,_,_)|ordinal==index);let kind=self.state.kinds[self.state.part];if current.is_some_and(|(ordinal,actual,_)|ordinal==index&&kind!=actual){return Err(invalid().into());}
                let text=current.filter(|_|present).map(|(_,_,text)|text);let value=match kind{InputKind::Reference=>text.and_then(|text|canonical_decimal(text,true)).map(IfcValue::Reference),InputKind::Integer=>text.and_then(signed_decimal).map(IfcValue::Integer),InputKind::String=>None};let valid=if kind==InputKind::String{present}else{value.is_some()};
                metadata(cx,base+size_of::<Field>()+size_of::<super::type_input::Input>(),||{self.placeholders.as_mut().unwrap()[self.state.part].value=Some(value.unwrap_or(IfcValue::Unset));self.state.ready&=valid;self.fields.as_mut().unwrap().push(Field{binding:StagedFormBinding{token:empty_token(),layout:if present{StagedFormLayout::Preserve}else{StagedFormLayout::Replace},edits:Vec::new()},attribute:String::new(),index,row:self.state.row,present,kind});self.state.row+=usize::from(present);self.state.part+=1;if kind==InputKind::String&&present{self.state.phase=30;}})?;
            }
            30=>{
                let row=self.fields.as_ref().and_then(|fields|fields.last()).ok_or_else(invalid)?.row;let reserve=row_data(self.request.as_ref().ok_or_else(stale)?,row)?.2.len();let physical=OriginalConstructionDemand{items:1,copy_bytes:base+size_of::<String>(),capacity_bytes:reserve,release_bytes:0,depth:1};
                if construct_row_string(&mut self.scalar,reserve,"",physical,cx)?{self.state.offset=0;self.state.phase=31;}
            }
            31=>{
                let row=self.fields.as_ref().and_then(|fields|fields.last()).ok_or_else(invalid)?.row;let text=row_data(self.request.as_ref().ok_or_else(stale)?,row)?.2;let mut end=(self.state.offset+64).min(text.len());while !text.is_char_boundary(end){end-=1;}let mut buffer=[0u8;64];let chunk=&text[self.state.offset..end];buffer[..chunk.len()].copy_from_slice(chunk);let length=chunk.len();let complete=end==text.len();
                metadata(cx,base+192+size_of::<String>(),||{self.scalar.as_mut().unwrap().push_str(std::str::from_utf8(&buffer[..length]).expect("UTF8 boundary"));self.state.offset=end;if complete{self.state.phase=32;}})?;
            }
            32=>{metadata(cx,base+2*size_of::<String>()+size_of::<IfcValue>(),||{self.placeholders.as_mut().unwrap()[self.state.part-1].value=Some(IfcValue::String(self.scalar.take().unwrap()));self.state.phase=4;})?;}
''' + s[b:]
s=s.replace('            && self.row_parts.is_none()','            && self.row_parts.is_none()\n            && self.scalar.is_none()')
s=s.replace('        owner!(&self.row_parts, RowParts);','        owner!(&self.scalar, String);\n        owner!(&self.row_parts, RowParts);').replace('        owner!(&mut self.row_parts);','        owner!(&mut self.scalar);\n        owner!(&mut self.row_parts);')
data=s.encode();old=p.stat().st_size
with p.open('r+b')as f:f.write(data);f.write(b' '*max(0,old-len(data)))
