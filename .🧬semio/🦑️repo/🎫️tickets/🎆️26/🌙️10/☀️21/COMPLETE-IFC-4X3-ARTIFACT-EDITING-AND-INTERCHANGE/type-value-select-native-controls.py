from pathlib import Path
T=Path(__file__).parent
D=T.parents[6]/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details'
def replace(s,a,b,n=1):
 assert s.count(a)==n,(a,s.count(a))
 return s.replace(a,b)
p=D/'type-guidance.rs';s=p.read_text(encoding='utf-8')
s=s.replace('InputKind::Set|InputKind::Array','InputKind::Set|InputKind::Array|InputKind::List')
s=replace(s,'InputKind::String|InputKind::Set|InputKind::Array|InputKind::List=>true','InputKind::String|InputKind::Enum|InputKind::Binary|InputKind::Set|InputKind::Array|InputKind::List=>true')
s=replace(s,'Some("real")=>InputKind::Real,Some("aggregate")', 'Some("real")=>InputKind::Real,Some("enum")=>InputKind::Enum,Some("binary")=>InputKind::Binary,Some("aggregate")')
s=replace(s,'kind:if leaf==InputKind::Array{InputKind::Real}else{InputKind::Reference}', 'kind:if leaf==InputKind::Array{InputKind::Real}else if leaf==InputKind::List{InputKind::Integer}else{InputKind::Reference}')
s=replace(s,'fn collection_word(request:&StagedFormRequest,row:usize,member:usize,real:bool)->Result<&str,ValueError>{if real{super::type_input::real_word(set_members(request,row)?.get(member).ok_or_else(invalid)?).ok_or_else(invalid)}else{set_word(request,row,member)}}', 'fn collection_word(request:&StagedFormRequest,row:usize,member:usize,kind:InputKind)->Result<&str,ValueError>{let value=set_members(request,row)?.get(member).ok_or_else(invalid)?;match kind{InputKind::Real=>super::type_input::real_word(value),InputKind::Integer=>super::type_input::integer_word(value),InputKind::Reference=>super::type_input::reference_text(value),_=>None}.ok_or_else(invalid)}')
s=replace(s,'collection_word(request,field.row,member,field.kind==InputKind::Real)', 'collection_word(request,field.row,member,field.kind)')
s=replace(s,'collection_word(request,row,member,true)', 'collection_word(request,row,member,InputKind::Real)')
s=replace(s,'let array=self.state.leaves[self.state.part-1]==InputKind::Array;let value=if array{','let array=self.state.leaves[self.state.part-1]==InputKind::Array;let integer=self.state.leaves[self.state.part-1]==InputKind::List;let value=if integer{signed_decimal(collection_word(request,row,member,InputKind::Integer)?).map(IfcValue::Integer)}else if array{')
s=replace(s,'count:usize)->Result<(usize,usize),ValueError>{let mut offset=0;', 'count:usize,leaves:usize)->Result<(usize,usize),ValueError>{let mut offset=0;')
s=replace(s,'if offset%SET_PAGE!=0||offset>=count&&offset!=0', 'if leaves>32&&offset%31==0&&offset<leaves{offset=0;}else if offset%SET_PAGE!=0||offset>=count&&offset!=0')
s=replace(s,'super::type_input::set_members(constructor(request,self.state.row)?).map_or(0,|items|items.len()))?.1', 'super::type_input::set_members(constructor(request,self.state.row)?).map_or(0,|items|items.len()),self.state.leaf_counts[self.state.part])?.1')
s=replace(s,'set_window(request,self.state.position,index,set_count)?', 'set_window(request,self.state.position,index,set_count,self.state.leaf_counts[self.state.part])?')
s=replace(s,'field.selection&&field.kind==InputKind::Number{if selected_numeric(self.page.as_ref().ok_or_else(stale)?.request.as_ref().ok_or_else(stale)?,field.row)?.is_some(){7}else{6}}', 'field.selection&&matches!(field.kind,InputKind::Number|InputKind::Enum){let request=self.page.as_ref().ok_or_else(stale)?.request.as_ref().ok_or_else(stale)?;if super::type_input::typed_constructor(constructor(request,field.row)?).ok_or_else(invalid)?.1.is_empty(){6}else{7}}')
s=replace(s,'!(field.selection&&field.kind==InputKind::Number)', '!(field.selection&&matches!(field.kind,InputKind::Number|InputKind::Enum))',2)
s=replace(s,'self.current_field()?.selection&&self.current_field()?.kind==InputKind::Number&&self.choice_path_len()?==6', 'self.current_field()?.selection&&matches!(self.current_field()?.kind,InputKind::Number|InputKind::Enum)&&self.choice_path_len()?==6')
s=replace(s,'let domain = self.domains[field.ordinal].ok_or_else(invalid)?;', 'let domain = if field.selection{self.leaf_domains[field.ordinal]}else{self.domains[field.ordinal]}.ok_or_else(invalid)?;')
s=replace(s,'InputKindCursor::member::<C>(self.domains[field.ordinal].ok_or_else(invalid)?, index)', 'InputKindCursor::member::<C>(if field.selection{self.leaf_domains[field.ordinal]}else{self.domains[field.ordinal]}.ok_or_else(invalid)?, index)')
a='let paging=if matches!(field.kind,InputKind::Set|InputKind::Array|InputKind::List)&&field.operation==Some(false){Some((super::type_input::set_members(constructor(request,field.row)?).map_or(0,|items|items.len()),SET_PAGE,SET_PAGE))}else if let Some(domain)=self.domains[field.ordinal].filter(|_|self.state.kinds[field.ordinal]==InputKind::Enum){Some((InputKindCursor::count::<C>(domain).ok_or_else(invalid)?,31,32))}else{None};'
b='let paging=if field.kind==InputKind::Select{Some((next.leaf_counts[field.ordinal],31,32))}else if matches!(field.kind,InputKind::Set|InputKind::Array|InputKind::List)&&field.operation==Some(false)&&next.leaf_counts[field.ordinal]<=32{Some((super::type_input::set_members(constructor(request,field.row)?).map_or(0,|items|items.len()),SET_PAGE,SET_PAGE))}else if let Some(domain)=self.domains[field.ordinal].filter(|_|self.state.kinds[field.ordinal]==InputKind::Enum){Some((InputKindCursor::count::<C>(domain).ok_or_else(invalid)?,31,32))}else{None};'
s=replace(s,a,b)
a='''                                if next.page_offset >= count || next.page_offset%step!=0 {
                                    return Err(invalid().into());
                                }
                                next.active_found = true;'''
b='''                                let members=if field.kind==InputKind::Select&&field.present&&matches!(next.leaves[field.ordinal],InputKind::Set|InputKind::Array|InputKind::List){super::type_input::set_members(constructor(request,field.row)?).map_or(0,|items|items.len())}else{0};
                                if field.kind==InputKind::Select&&next.page_offset>0&&next.page_offset%SET_PAGE==0&&next.page_offset<members{next.active_found=true;next.navigation[0]=Some((field.index,if next.page_offset==SET_PAGE{(count-1)/step*step}else{next.page_offset-SET_PAGE}));if next.page_offset+SET_PAGE<members{next.navigation[1]=Some((field.index,next.page_offset+SET_PAGE));}next.scan+=1;metadata(cx,base,||self.state=next)?;return Ok(false);}
                                if next.page_offset >= count || next.page_offset%step!=0 {return Err(invalid().into());}
                                next.active_found = true;
                                if field.kind==InputKind::Select&&next.page_offset+step>=count&&members>SET_PAGE{next.navigation[1]=Some((field.index,SET_PAGE));}'''
s=replace(s,a,b)
p.write_text(s,encoding='utf-8',newline='\n')
p=D/'type_input_set_controls.rs';s=p.read_text(encoding='utf-8')
s=replace(s,'else if field.kind==InputKind::Array{"real"}else{"reference"}', 'else if field.kind==InputKind::Array{"real"}else if field.kind==InputKind::List{"integer"}else{"reference"}')
p.write_text(s,encoding='utf-8',newline='\n')
print('[DEBUG] native recursive child controls and combined root/member navigation written')
