from pathlib import Path
f=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/type-guidance.rs');s=f.read_text(encoding='utf8')
a='''            30 => {
                let row = self.fields'''
b='''            60=>{
                let field=self.fields.as_ref().and_then(|fields|fields.last()).ok_or_else(invalid)?;
                let text=row_data(self.request.as_ref().ok_or_else(stale)?,field.row)?.2;
                let domain=self.state.domains[self.state.part-1].ok_or_else(invalid)?;
                let word=InputKindCursor::member::<C>(domain,self.state.member);
                let mut next=self.state;
                match word{
                    None=>{next.ready=false;next.phase=4;}
                    Some(word)if word.len()!=text.len()=>{next.member+=1;next.offset=0;}
                    Some(word)if next.offset==word.len()=>{next.selected[next.part-1]=Some(next.member);next.phase=30;}
                    Some(word)=>{let end=word.len().min(next.offset+64);if word.as_bytes()[next.offset..end]==text.as_bytes()[next.offset..end]{next.offset=end;}else{next.member+=1;next.offset=0;}}
                }
                metadata(cx,base,||self.state=next)?;
            }
            61=>{
                let mut next=self.state;
                let request=self.request.as_ref().ok_or_else(stale)?;
                if next.scan==0{
                    if let Some(choice)=request.page.as_ref().and_then(|page|page.choice.as_ref()){
                        if choice.path.len()!=4||choice.path[0]!="entities"||canonical_decimal(&choice.path[1],false).and_then(|value|usize::try_from(value).ok())!=Some(next.position)||choice.path[2]!="args"{return Err(invalid().into());}
                        next.active_target=Some(canonical_decimal(&choice.path[3],false).and_then(|value|usize::try_from(value).ok()).ok_or_else(invalid)?);
                        next.page_offset=canonical_decimal(&choice.offset,false).and_then(|value|usize::try_from(value).ok()).ok_or_else(invalid)?;
                        if next.page_offset%31!=0{return Err(invalid().into());}
                    }
                }
                if let Some(field)=self.fields.as_ref().and_then(|fields|fields.get(next.scan)){
                    if let Some(domain)=next.domains[next.scan]{
                        let count=InputKindCursor::count::<C>(domain).ok_or_else(invalid)?;
                        if count>32{
                            let target=*next.active_target.get_or_insert(field.index);
                            if field.index<target{next.navigation[0]=Some((field.index,(count-1)/31*31));}
                            else if field.index==target{
                                if next.page_offset>=count{return Err(invalid().into());}
                                next.active_found=true;
                                if next.page_offset>0{next.navigation[0]=Some((field.index,next.page_offset-31));}
                                if next.page_offset+31<count{next.navigation[1]=Some((field.index,next.page_offset+31));}
                            }else if next.navigation[1].is_none(){next.navigation[1]=Some((field.index,0));}
                        }
                    }
                    next.scan+=1;
                }else{
                    if next.active_target.is_some()&&!next.active_found{return Err(invalid().into());}
                    next.phase=62;next.nav=0;
                }
                metadata(cx,base,||self.state=next)?;
            }
            62=>{
                let count=self.state.navigation.iter().flatten().count();let physical=OriginalConstructionDemand{items:1,copy_bytes:base,capacity_bytes:count*size_of::<Navigation>(),release_bytes:0,depth:1};
                if admit_original_construction(cx,&mut self.navigation,physical,|_|{let mut rows=Vec::new();rows.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"type choice pages"))?;let capacity=rows.capacity()*size_of::<Navigation>();Ok((rows,RetainedCloneProgress{copied_items:1,copied_bytes:base,retained_capacity_bytes:capacity,..Default::default()}))})?{self.state.phase=63;}
            }
            63=>{
                if self.nav_spec().is_none(){metadata(cx,base+2*size_of::<Navigation>(),||self.state.phase=22)?;}
                else{metadata(cx,base+2*size_of::<Navigation>(),||{*self.nav_item=Some(Navigation{previous:self.state.navigation[0]==self.nav_spec(),binding:StagedFormPageBinding{token:empty_token(),page:StagedFormPage{field_offset:String::new(),diagnostic_offset:String::new(),choice:Some(StagedFormChoicePage{path:Vec::new(),offset:String::new()})}}});self.state.part=0;self.state.phase=64;})?;}
            }
            64=>{
                let token=&self.request.as_ref().ok_or_else(stale)?.token;
                if copy_text(token_part_mut(&mut self.nav_item.as_mut().ok_or_else(invalid)?.binding.token,self.state.part),token_part(token,self.state.part),cx)?{self.state.part+=1;if self.state.part==4{self.state.phase=65;}}
            }
            65|66=>{
                let page=&mut self.nav_item.as_mut().ok_or_else(invalid)?.binding.page;
                let target=if self.state.phase==65{&mut page.diagnostic_offset}else{&mut page.field_offset};
                if copy_text(target,"0",cx)?{self.state.phase+=1;}
            }
            67=>{
                let mut buffer=[0;20];let text=decimal(self.nav_spec().ok_or_else(invalid)?.1,&mut buffer);
                if copy_text_extra(&mut self.nav_item.as_mut().ok_or_else(invalid)?.binding.page.choice.as_mut().ok_or_else(invalid)?.offset,text,40,cx)?{self.state.phase=68;}
            }
            68=>{
                let path=&mut self.nav_item.as_mut().ok_or_else(invalid)?.binding.page.choice.as_mut().ok_or_else(invalid)?.path;
                let mut output=None;let physical=OriginalConstructionDemand{items:1,copy_bytes:base,capacity_bytes:4*size_of::<String>(),release_bytes:0,depth:1};
                if admit_original_construction(cx,&mut output,physical,|_|{path.try_reserve_exact(4).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"type page address"))?;Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:base,retained_capacity_bytes:path.capacity()*size_of::<String>(),..Default::default()}))})?{self.state.part=0;self.state.phase=69;}
            }
            69=>{
                if self.state.part==4{metadata(cx,base+size_of::<String>(),||self.state.phase=70)?;}
                else if self.nav_item.as_ref().unwrap().binding.page.choice.as_ref().unwrap().path.len()==self.state.part{metadata(cx,base+size_of::<String>(),||self.nav_item.as_mut().unwrap().binding.page.choice.as_mut().unwrap().path.push(String::new()))?;}
                else{
                    let mut buffer=[0;20];let text=self.page_path_text(self.state.part,self.nav_spec().ok_or_else(invalid)?.0,&mut buffer)?;
                    if copy_text_extra(&mut self.nav_item.as_mut().unwrap().binding.page.choice.as_mut().unwrap().path[self.state.part],text,40,cx)?{self.state.part+=1;}
                }
            }
            70=>{metadata(cx,base+2*size_of::<Navigation>(),||{self.navigation.as_mut().unwrap().push(self.nav_item.take().unwrap());self.state.nav+=1;self.state.phase=63;})?;}
'''+a
assert a in s;s=s.replace(a,b)
a='''            15 if self.current_field()?.kind==InputKind::Enum =>'''
b='''            33=>{
                metadata(cx,base+size_of::<StagedFormBindingEdit>()+size_of::<ChoiceParts>(),||{*self.edit=Some(StagedFormBindingEdit::Choice{kind:StagedFormChoiceEditKind::Set,path:Vec::new(),options:Vec::new()});*self.choice_parts=Some(ChoiceParts{strings:Default::default(),payload:None});self.state.phase=34;})?;
            }
            34|35=>{
                let(_,length,pin)=self.choice_shape()?;let count=length+usize::from(pin.is_some());
                let physical=OriginalConstructionDemand{items:1,copy_bytes:base+if self.state.phase==34{size_of::<Vec<String>>()}else{0},capacity_bytes:if self.state.phase==34{4*size_of::<String>()}else{count*size_of::<StagedFormBindingOption>()},release_bytes:0,depth:1};
                let StagedFormBindingEdit::Choice{path,options,..}=self.edit.as_mut().ok_or_else(invalid)? else{return Err(invalid().into())};
                let mut output=None;
                if admit_original_construction(cx,&mut output,physical,|_|{let capacity=if self.state.phase==34{path.try_reserve_exact(4).map_err(|_|invalid())?;path.capacity()*size_of::<String>()}else{options.try_reserve_exact(count).map_err(|_|invalid())?;options.capacity()*size_of::<StagedFormBindingOption>()};Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:physical.copy_bytes,retained_capacity_bytes:capacity,..Default::default()}))})?{self.state.phase+=1;}
            }
            36=>{
                let(_,length,pin)=self.choice_shape()?;let count=length+usize::from(pin.is_some());let physical=OriginalConstructionDemand{items:1,copy_bytes:base+size_of::<Choices>(),capacity_bytes:count*size_of::<StagedScalarChoice>(),release_bytes:0,depth:1};
                if admit_original_construction(cx,&mut self.page.as_mut().unwrap().fields[self.state.field].choices,physical,|_|{let mut rows=Vec::new();rows.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"type declared choices"))?;let capacity=rows.capacity()*size_of::<StagedScalarChoice>();Ok((Choices::new(rows),RetainedCloneProgress{copied_items:1,copied_bytes:physical.copy_bytes,retained_capacity_bytes:capacity,..Default::default()}))})?{self.state.choice_part=0;self.state.phase=37;}
            }
            37=>{
                if self.state.choice_part==4{metadata(cx,base+size_of::<String>(),||{self.state.choice_part=0;self.state.phase=38;})?;}
                else{
                    let row=self.current_field()?.row;
                    let StagedFormBindingEdit::Choice{path,..}=self.edit.as_mut().ok_or_else(invalid)? else{return Err(invalid().into())};
                    if path.len()==self.state.choice_part{metadata(cx,base+size_of::<String>(),||path.push(String::new()))?;}
                    else{let mut buffer=[0;20];let text=match self.state.choice_part{0=>"value",1=>"$ifcInputs",2=>decimal(row,&mut buffer),_=>"value"};if copy_text_extra(&mut path[self.state.choice_part],text,40,cx)?{self.state.choice_part+=1;}}
                }
            }
            38 if self.state.choice_part<5=>{
                let word=self.choice_text()?;let physical=OriginalConstructionDemand{items:1,copy_bytes:base+size_of::<String>()+word.len(),capacity_bytes:word.len(),release_bytes:0,depth:1};
                if construct_row_string(&mut self.choice_parts.as_mut().unwrap().strings[self.state.choice_part],word.len(),word,physical,cx)?{self.state.choice_part+=1;}
            }
            38=>{metadata(cx,base,||self.state.phase=39)?;}
            39=>{
                let physical=OriginalConstructionDemand{items:1,copy_bytes:base+size_of::<Vec<(String,DslValue)>>(),capacity_bytes:2*size_of::<(String,DslValue)>(),release_bytes:0,depth:1};
                if admit_original_construction(cx,&mut self.choice_parts.as_mut().unwrap().payload,physical,|_|{let mut fields=Vec::new();fields.try_reserve_exact(2).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"type enum payload"))?;let capacity=fields.capacity()*size_of::<(String,DslValue)>();Ok((fields,RetainedCloneProgress{copied_items:1,copied_bytes:physical.copy_bytes,retained_capacity_bytes:capacity,..Default::default()}))})?{self.state.phase=40;}
            }
            40=>{
                let word=self.choice_word()?;let locale=self.page.as_ref().unwrap().request.as_ref().unwrap().locale;
                metadata(cx,base+4*size_of::<ChoiceParts>()+2*size_of::<StagedFormBindingOption>()+size_of::<StagedScalarChoice>(),||{
                    let parts=self.choice_parts.as_mut().unwrap();let mut payload=parts.payload.take().unwrap();payload.push((parts.strings[1].take().unwrap(),DslValue::String(parts.strings[2].take().unwrap())));payload.push((parts.strings[3].take().unwrap(),DslValue::String(parts.strings[4].take().unwrap())));
                    let StagedFormBindingEdit::Choice{options,..}=self.edit.as_mut().unwrap()else{unreachable!()};options.push(StagedFormBindingOption{key:parts.strings[0].take().unwrap(),value:DslValue::Object(payload)});
                    self.page.as_mut().unwrap().fields[self.state.field].choices.as_mut().unwrap().push(word,Self::choice_label(word,locale));
                    self.state.option+=1;self.state.choice_part=0;
                    let(_,length,pin)=self.choice_shape().expect("completed declaration window");self.state.phase=if self.state.option==length+usize::from(pin.is_some()){16}else{38};
                })?;
            }
'''+a
assert a in s;s=s.replace(a,b)
b=s.encode();old=f.stat().st_size
with f.open('r+b')as o:o.write(b);o.write(b' '*max(0,old-len(b)))
