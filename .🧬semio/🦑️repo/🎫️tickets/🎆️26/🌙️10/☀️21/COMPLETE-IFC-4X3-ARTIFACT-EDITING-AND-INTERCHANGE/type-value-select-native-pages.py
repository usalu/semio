from pathlib import Path
T=Path(__file__).parent
D=T.parents[6]/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details'
def replace(s,a,b,n=1):
 assert s.count(a)==n,(a,s.count(a))
 return s.replace(a,b)
p=D/'type_input_kind.rs';s=p.read_text(encoding='utf-8')
s=replace(s,'    select_scalar: bool,','    select_scalar: bool,\n    first_leaf:Option<&\'static str>,')
s=replace(s,'    pub(super) fn leaf_count(',"    pub(super) fn first_leaf(&self)->Option<&'static str>{self.first_leaf}\n    pub(super) fn leaf_count(")
s=replace(s,'if index!=self.select_next{return Err(invalid());}if C::direct_attribute_count(name)','if index!=self.select_next{return Err(invalid());}if index==0{self.first_leaf=Some(name);}if C::direct_attribute_count(name)')
p.write_text(s,encoding='utf-8',newline='\n')
p=D/'type-guidance.rs';s=p.read_text(encoding='utf-8')
s=replace(s,'    selected: [usize; 16],',"    selected: [usize; 16],\n    selected_leaves:[usize;16],\n    leaf_counts:[usize;16],\n    first_leaves:[Option<&'static str>;16],\n    leaf_name:Option<&'static str>,")
s=replace(s,'    kind_cursor: InputKindCursor,','    kind_cursor: InputKindCursor,\n    leaf_cursor:super::type_input_select::Cursor,')
s=replace(s,'            kind_cursor: Default::default(),','            kind_cursor: Default::default(),\n            leaf_cursor:Default::default(),')
s=replace(s,'    domains: [Option<&\'static str>; 16],',"    domains: [Option<&'static str>; 16],\n    leaf_domains:[Option<&'static str>;16],")
s=replace(s,'            domains: Default::default(),','            domains: Default::default(),\n            leaf_domains:Default::default(),')
s=replace(s,'let base = 4 * size_of::<State>() + 2 * 256 + size_of::<IfcValue>();','let base = 4 * size_of::<State>() + 2 * 256 + size_of::<IfcValue>() + 2*size_of::<super::type_input_select::Cursor>();',2)
s=replace(s,'                            self.state.references[index_in_roster] = cursor.reference_member();','                            self.state.references[index_in_roster] = cursor.reference_member();\n                            self.state.leaf_counts[index_in_roster]=cursor.leaf_count().unwrap_or(0);\n                            self.state.first_leaves[index_in_roster]=cursor.first_leaf();')
s=replace(s,'self.state.member=0;self.state.offset=0;self.state.phase=90;','self.state.member=0;self.state.offset=0;self.state.leaf_name=None;self.leaf_cursor.clear();self.state.phase=90;')
a=s.index('            90=>{');b=s.index('            91=>{',a)
s=s[:a]+'''            90=>{
                let request=self.request.as_ref().ok_or_else(stale)?;let(name,_)=super::type_input::typed_constructor(constructor(request,self.state.row)?).ok_or_else(invalid)?;let mut next=self.state;let mut cursor=self.leaf_cursor;
                if let Some(member)=next.leaf_name{if name.len()!=member.len(){next.leaf_name=None;next.offset=0;}else if next.offset==member.len(){next.selected_leaves[next.part]=next.member+1;next.leaf_name=None;cursor.clear();next.phase=91;}else{let end=(next.offset+64).min(member.len());if name.as_bytes()[next.offset..end]==member.as_bytes()[next.offset..end]{next.offset=end;}else{next.leaf_name=None;next.offset=0;}}}else{match cursor.advance::<C>(self.domains[next.part].ok_or_else(invalid)?)?{super::type_input_select::Step::Pending=>{},super::type_input_select::Step::Complete{..}=>return Err(invalid().into()),super::type_input_select::Step::Leaf{index,name}=>{next.member=index;next.leaf_name=Some(name);}}}metadata(cx,base,||{self.state=next;self.leaf_cursor=cursor;})?;
            }
'''+s[b:]
s=replace(s,'InputKind::Number|InputKind::Set|InputKind::Array)||items.first()', 'InputKind::Number|InputKind::Enum|InputKind::Binary|InputKind::List|InputKind::Set|InputKind::Array)||items.first()')
s=replace(s,'self.state.leaves[self.state.part]=kind;self.kind_cursor.clear();', 'self.state.leaves[self.state.part]=kind;self.leaf_domains[self.state.part]=cursor.domain();self.kind_cursor.clear();')
s=replace(s,'if field.kind==InputKind::Select{return Ok((0,C::select_count(self.domains[field.ordinal].ok_or_else(invalid)?).ok_or_else(invalid)?,None));}', '''if field.kind==InputKind::Select{let count=self.state.leaf_counts[field.ordinal];if count==0{return Err(invalid());}let start=if count>32&&self.state.active_target==Some(field.index)&&self.state.page_offset%31==0{self.state.page_offset}else{0};if start>=count{return Err(invalid());}let length=if count>32{(count-start).min(31)}else{count};let pin=self.state.selected_leaves[field.ordinal].checked_sub(1).or(field.selected_choice).filter(|index|*index<start||*index>=start+length);return Ok((start,length,pin));}''')
s=replace(s,'if field.kind==InputKind::Select{return C::select_member(self.domains[field.ordinal].ok_or_else(invalid)?,index).ok_or_else(invalid);}', 'if field.kind==InputKind::Select{return self.state.leaf_name.ok_or_else(invalid);}')
s=replace(s,'name:C::select_member(domains[field.ordinal].ok_or_else(invalid)?,0).ok_or_else(invalid)?', 'name:state.first_leaves[field.ordinal].ok_or_else(invalid)?')
a=s.index('            93=>{');b=s.index('            83=>{',a)
s=s[:a]+'''            93=>{
                if self.state.leaf_name.is_none(){let field=self.current_field()?;let(start,length,pin)=self.choice_shape()?;let target=if self.state.option<length{start+self.state.option}else{pin.ok_or_else(invalid)?};let mut cursor=self.leaf_cursor;let answer=cursor.advance::<C>(self.domains[field.ordinal].ok_or_else(invalid)?)?;if matches!(answer,super::type_input_select::Step::Complete{..}){return Err(invalid().into());}metadata(cx,base,||{self.leaf_cursor=cursor;if let super::type_input_select::Step::Leaf{index,name}=answer{if index==target{self.state.leaf_name=Some(name);self.leaf_cursor.clear();}}})?;return Ok(false);}
                let name=self.choice_word()?;if C::direct_attribute_count(name).is_some(){metadata(cx,base+2*size_of::<InputKindCursor>(),||{self.state.choice_kind=InputKind::Reference;self.kind_cursor.clear();self.state.phase=38;})?;}else{let expression=C::defined_type_expression(name).ok_or_else(invalid)?;let mut cursor=self.kind_cursor;let answer=cursor.advance_selected::<C>(expression)?;if answer.is_some_and(|kind|!matches!(kind,InputKind::String|InputKind::Integer|InputKind::Enum|InputKind::Real|InputKind::Number|InputKind::Binary|InputKind::List|InputKind::Set|InputKind::Array)){return Err(invalid().into());}metadata(cx,base+2*size_of::<InputKindCursor>(),||{self.kind_cursor=cursor;if let Some(kind)=answer{self.state.choice_kind=kind;self.kind_cursor.clear();self.state.phase=38;}})?;}
            }
'''+s[b:]
s=replace(s,'self.state.option += 1;', 'self.state.option += 1;\n                    self.state.leaf_name=None;\n                    self.leaf_cursor.clear();')
s=replace(s,'self.state.option = 0;', 'self.state.option = 0;\n                    self.state.leaf_name=None;\n                    self.leaf_cursor.clear();')
p.write_text(s,encoding='utf-8',newline='\n')
print('[DEBUG] native recursive leaf page ownership and ordinal discovery written')
