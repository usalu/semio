from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/type-guidance.rs')
s=p.read_text(encoding='utf-8')
def replace(a,b):
 global s
 assert a in s,a[:100]
 s=s.replace(a,b)
replace('fn set_word(request:', 'fn set_word(request:')
pos=s.index('fn set_window(')
s=s[:pos]+'fn collection_word(request:&StagedFormRequest,row:usize,member:usize,real:bool)->Result<&str,ValueError>{if real{super::type_input::real_word(set_members(request,row)?.get(member).ok_or_else(invalid)?).ok_or_else(invalid)}else{set_word(request,row,member)}}\n'+s[pos:]
replace('if field.member.is_some(){(10,41+row.len()+member.len())}', 'if field.member.is_some(){if field.kind==InputKind::Real{(11,45+row.len()+member.len())}else{(10,41+row.len()+member.len())}}')
replace('if field.selection{set_word(request,field.row,member)?}', 'if field.selection{collection_word(request,field.row,member,field.kind==InputKind::Real)?}')
replace('else if field.kind==InputKind::Number{row_data(request,field.row)?.1.tag()}', 'else if field.kind==InputKind::Number{if field.selection{selected_numeric(request,field.row)?.map_or("",|(kind,_)|kind.tag())}else{row_data(request,field.row)?.1.tag()}}')
a=s.index('        let label = if let Some(remove)=field.operation')
b=s.index('else{match (field.kind,',a)
s=s[:a]+'''        let label = if let Some(remove)=field.operation{let array=field.kind==InputKind::Array;let empty=super::type_input::typed_constructor(constructor(request,field.row)?).ok_or_else(invalid)?.1.is_empty();match(remove,array,empty,request.locale){(true,true,_,Locale::En)=>"Remove real word",(true,true,_,Locale::De)=>"REAL-Wort entfernen",(true,false,_,Locale::En)=>"Remove reference",(true,false,_,Locale::De)=>"Referenz entfernen",(false,true,true,Locale::En)=>"Create ARRAY aggregate",(false,true,true,Locale::De)=>"ARRAY-Aggregat erstellen",(false,true,false,Locale::En)=>"Add real word",(false,true,false,Locale::De)=>"REAL-Wort hinzufügen",(false,false,true,Locale::En)=>"Create SET aggregate",(false,false,true,Locale::De)=>"SET-Aggregat erstellen",(false,false,false,Locale::En)=>"Add reference",(false,false,false,Locale::De)=>"Referenz hinzufügen"}}'''+s[b:]
replace('InputKind::Number|InputKind::Select if present=>2', 'InputKind::Select if present&&self.state.leaves[self.state.part]==InputKind::Number=>2+usize::from(selected_numeric(request,self.state.row)?.is_some()),InputKind::Number|InputKind::Select if present=>2')
replace('let field=self.current_field()?;let number=field.kind==InputKind::Number;', 'let field=self.current_field()?;let number=field.kind==InputKind::Number;')
replace('if field.present{let(_,kind,text)=row_data(self.page.as_ref().ok_or_else(stale)?.request.as_ref().ok_or_else(stale)?,field.row)?;if kind.tag()==if reference{"reference"}else{word}{text}else{""}}else{""}', 'if field.present{let request=self.page.as_ref().ok_or_else(stale)?.request.as_ref().ok_or_else(stale)?;let(kind,text)=if number&&field.selection{selected_numeric(request,field.row)?.unwrap_or((InputKind::Number,""))}else{let(_,kind,text)=row_data(request,field.row)?;(kind,text)};if kind.tag()==if reference{"reference"}else{word}{text}else{""}}else{""}')
a=s.index('        if let Some((name,kind))=choice{return Ok(typed_copy::Input')
b=s.index('\n    }\n    fn advance_typed_copy',a)
s=s[:a]+'''        if let Some((name,kind))=choice{let source=if field.present&&row_data(request,field.row)?.1==InputKind::Select{Some(constructor(request,field.row)?)}else{None};let kind=if kind==InputKind::Number{if source.and_then(|source|super::type_input::typed_constructor(source)).and_then(|(_,items)|items.first()).and_then(|child|child.get("kind")).and_then(DslValue::as_str)==Some("real"){InputKind::Real}else{InputKind::Integer}}else{kind};return Ok(typed_copy::Input{source,name,kind,fill:false});}
        if state.copying==field.row&&!field.present{return Ok(typed_copy::Input{source:None,name:C::select_member(domains[field.ordinal].ok_or_else(invalid)?,0).ok_or_else(invalid)?,kind:InputKind::String,fill:false});}
        let row=state.copying-usize::from(!field.present&&state.copying>field.row);let source=constructor(request,row)?;let(name,items)=super::type_input::typed_constructor(source).ok_or_else(invalid)?;let kind=if field.selection&&state.copying==field.row{field.kind}else{match items.first().and_then(|value|value.get("kind")).and_then(DslValue::as_str){Some("integer")=>InputKind::Integer,Some("real")=>InputKind::Real,Some("aggregate")=>{let main=page.fields.iter().find(|value|value.kind==InputKind::Select&&value.present&&value.row==row).ok_or_else(invalid)?;state.leaves[main.ordinal]},_=>InputKind::String}};Ok(typed_copy::Input{source:Some(source),name,kind,fill:field.selection&&state.copying==field.row&&!matches!(kind,InputKind::Set|InputKind::Array)})'''+s[b:]
replace('items.first().is_some_and(|value|value.get("kind").and_then(DslValue::as_str)!=Some(kind.tag()))', 'items.first().is_some_and(|value|{let tag=value.get("kind").and_then(DslValue::as_str);if kind==InputKind::Number{!matches!(tag,Some("integer"|"real"))}else{tag!=Some(kind.tag())}})')
replace('matches!(self.state.leaves[self.state.part],InputKind::String|InputKind::Set)||text.and_then(|text|canonical_decimal(text,false)).is_some_and(|bits|(bits>>52)&2047!=2047)', 'match self.state.leaves[self.state.part]{InputKind::String|InputKind::Set|InputKind::Array=>true,InputKind::Integer=>text.and_then(signed_decimal).is_some(),InputKind::Number=>selected_numeric(request,self.state.row)?.is_some_and(|(kind,text)|if kind==InputKind::Integer{signed_decimal(text).is_some()}else{canonical_decimal(text,false).is_some_and(|bits|(bits>>52)&2047!=2047)}),_=>text.and_then(|text|canonical_decimal(text,false)).is_some_and(|bits|(bits>>52)&2047!=2047)}')
# append an actual numeric text field after the selected representation field
needle='if set{for member in set_window.0..set_window.0+set_window.1'
replace(needle, 'if leaf==InputKind::Number{if let Some((kind,_))=selected_numeric(request,self.state.row).expect("closed original numeric child"){self.fields.as_mut().unwrap().push(Field{binding:StagedFormBinding{token:empty_token(),layout:StagedFormLayout::Preserve,edits:Vec::new()},attribute:String::new(),index,ordinal:self.state.part,member:None,operation:None,selection:true,selected_choice:None,row:self.state.row,present:true,kind,choices:None});}}'+needle)
replace('present:true,kind:InputKind::Reference,choices:None});}}}}', 'present:true,kind:if leaf==InputKind::Array{InputKind::Real}else{InputKind::Reference},choices:None});}}}}')
replace('let value=canonical_decimal(set_word(request,row,member)?,true).map(IfcValue::Reference);', 'let array=self.state.leaves[self.state.part-1]==InputKind::Array;let value=if array{canonical_decimal(collection_word(request,row,member,true)?,false).filter(|bits|(bits>>52)&2047!=2047).map(|bits|IfcValue::Real(f64::from_bits(bits)))}else{canonical_decimal(set_word(request,row,member)?,true).map(IfcValue::Reference)};')
replace('set_controls::text_path(row,member)', 'set_controls::text_path(row,member,real)')
# exact selected NUMBER choice path authority
pos=s.index('    fn choice_real(')
s=s[:pos]+'''    fn choice_path_len(&self)->Result<usize,ValueError>{let field=self.current_field()?;Ok(if field.selection&&field.kind==InputKind::Number{if selected_numeric(self.page.as_ref().ok_or_else(stale)?.request.as_ref().ok_or_else(stale)?,field.row)?.is_some(){7}else{6}}else{4})}
    fn choice_path_word<'a>(&self,part:usize,buffer:&'a mut[u8;20])->Result<&'a str,ValueError>{Ok(match part{0=>"value",1=>"$ifcInputs",2=>decimal(self.current_field()?.row,buffer),3|4=>"value",5=>"items",6=>"0",_=>return Err(invalid())})}
'''+s[pos:]
replace('demand.capacity_bytes = 4 * size_of::<String>();','demand.capacity_bytes = self.choice_path_len()? * size_of::<String>();')
a=s.index('                    let word = match self.state.choice_part {')
b=s.index(';\n                    let mut demand = text(word)',a)
s=s[:a]+'                    let word = self.choice_path_word(self.state.choice_part,&mut buffer)?'+s[b:]
replace('            33 => {\n                metadata', '            33 => {\n                let append=self.current_field()?.selection&&self.current_field()?.kind==InputKind::Number&&self.choice_path_len()?==6;\n                metadata')
replace('Some(StagedFormBindingEdit::Choice { kind: StagedFormChoiceEditKind::Set, path:', 'Some(StagedFormBindingEdit::Choice { kind: if append{StagedFormChoiceEditKind::Append}else{StagedFormChoiceEditKind::Set}, path:')
replace('capacity_bytes: if self.state.phase == 34 { 4 * size_of::<String>() }', 'capacity_bytes: if self.state.phase == 34 { self.choice_path_len()? * size_of::<String>() }')
replace('                let StagedFormBindingEdit::Choice { path, options, .. } = self.edit.as_mut()', '                let path_count=self.choice_path_len()?;\n                let StagedFormBindingEdit::Choice { path, options, .. } = self.edit.as_mut()')
replace('path.try_reserve_exact(4)', 'path.try_reserve_exact(path_count)')
replace('if self.state.choice_part == 4 {\n                    let select=', 'if self.state.choice_part == self.choice_path_len()? {\n                    let select=')
# compute borrowed text before mutably borrowing edit
replace('                    let row = self.current_field()?.row;\n                    let StagedFormBindingEdit::Choice { path, .. }', '                    let mut buffer=[0;20];let text=self.choice_path_word(self.state.choice_part,&mut buffer)?;\n                    let StagedFormBindingEdit::Choice { path, .. }')
a=s.index('                        let mut buffer = [0; 20];\n                        let text = match self.state.choice_part',s.index('            37 => {',s.index('pub fn advance') if 'pub fn advance' in s else 10000))
b=s.index(';\n                        if copy_text_extra',a)
s=s[:a]+s[b+2:]
p.write_text(s,encoding='utf-8',newline='\n')
