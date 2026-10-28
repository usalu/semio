from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/type-guidance.rs');s=p.read_text(encoding='utf8')
def replace(a,b):
 global s
 assert s.count(a)==1,(a[:90],s.count(a));s=s.replace(a,b)
a=s.index('fn row_text(');b=s.index('fn decimal(',a)
s=s[:a]+'''fn rows(request: &StagedFormRequest) -> Result<&[DslValue], ValueError> {
    Ok(super::type_input::request(marker(request)?.1).ok_or_else(invalid)?.1.unwrap_or(&[]))
}
fn row_text(request: &StagedFormRequest, ordinal: usize) -> Result<(usize, &str), ValueError> {
    let DslValue::Object(fields) = rows(request)?.get(ordinal).ok_or_else(invalid)? else { return Err(invalid()) };
    if fields.len() != 2 { return Err(invalid()); }
    let index = fields.iter().find(|(key, _)| key == "index").and_then(|(_, value)| value.as_str()).and_then(|value| canonical_decimal(value, false)).and_then(|value| usize::try_from(value).ok()).ok_or_else(invalid)?;
    let DslValue::Object(value) = &fields.iter().find(|(key, _)| key == "value").ok_or_else(invalid)?.1 else { return Err(invalid()) };
    if value.len() != 2 || !value.iter().any(|(key, value)| key == "kind" && value.as_str() == Some("reference")) { return Err(invalid()); }
    Ok((index, value.iter().find(|(key, _)| key == "value").and_then(|(_, value)| value.as_str()).ok_or_else(invalid)?))
}
fn row_value_mut(row: &mut DslValue) -> Result<&mut String, ValueError> {
    let DslValue::Object(fields) = row else { return Err(invalid()) };
    let DslValue::Object(fields) = &mut fields.iter_mut().find(|(key, _)| key == "value").ok_or_else(invalid)?.1 else { return Err(invalid()) };
    let DslValue::String(value) = &mut fields.iter_mut().find(|(key, _)| key == "value").ok_or_else(invalid)?.1 else { return Err(invalid()) };
    Ok(value)
}
fn row_capacity(index: &str, text: &str) -> usize { 4 * size_of::<(String, DslValue)>() + 28 + index.len() + text.len() }
fn make_row(index: &str, text: &str) -> Result<(DslValue, usize), ValueError> {
    let failure = || ValueError::literal(ValueRefusalKind::AllocationFailed, "type guide row");
    let string = |text: &str| -> Result<String, ValueError> { let mut result=String::new(); result.try_reserve_exact(text.len()).map_err(|_|failure())?; result.push_str(text); Ok(result) };
    let mut value=String::new(); value.try_reserve_exact(text.len()).map_err(|_|failure())?;
    let mut outer=Vec::new(); outer.try_reserve_exact(2).map_err(|_|failure())?;
    let mut inner=Vec::new(); inner.try_reserve_exact(2).map_err(|_|failure())?;
    inner.push((string("kind")?,DslValue::String(string("reference")?)));
    inner.push((string("value")?,DslValue::String(value)));
    outer.push((string("index")?,DslValue::String(string(index)?)));
    outer.push((string("value")?,DslValue::Object(inner)));
    let capacity=outer.capacity()*size_of::<(String,DslValue)>()+outer.iter().map(|(key,value)|key.capacity()+match value{DslValue::String(value)=>value.capacity(),DslValue::Object(fields)=>fields.capacity()*size_of::<(String,DslValue)>()+fields.iter().map(|(key,value)|key.capacity()+value.as_str().map_or(0,|text|text.len())).sum::<usize>(),_=>0}).sum::<usize>();
    let capacity=capacity+text.len();
    Ok((DslValue::Object(outer),capacity))
}
''' + s[b:]
replace('''    binding: StagedFormBinding,
    attribute: String,
    index: usize,
}''','''    fields: Vec<Field>,
}
#[derive(semio_framework_value::RetireOwned)]
struct Field {
    binding: StagedFormBinding,
    attribute: String,
    index: usize,
    row: usize,
    present: bool,
}''')
replace('''        1
    }
    fn control''','''        self.page.as_ref().map_or(0, |page| page.fields.len())
    }
    fn control''')
replace('''        if index != 0 {
            return Err(invalid());
        }
        let request = page.request.as_ref().ok_or_else(stale)?;
        let value = row_text(request, page.index)?.unwrap_or("");
        let label = match (page.attribute.as_str(), request.locale)''','''        let field = page.fields.get(index).ok_or_else(invalid)?;
        let request = page.request.as_ref().ok_or_else(stale)?;
        let value = if field.present { row_text(request, field.row)?.1 } else { "" };
        let label = match (field.attribute.as_str(), request.locale)''')
replace('address: 0, controller:', 'address: index as u64, controller:')
replace('description: Some(&page.attribute)', 'description: Some(&field.attribute)')
replace('StagedBodyBinding::Action(&page.binding)', 'StagedBodyBinding::Action(&field.binding)')
replace('''    guard_values: bool,
}''','''    guard_values: bool,
    field: usize,
    copying: usize,
    offset: usize,
}''')
replace('''    placeholder: ManuallyDrop<Option<super::type_input::Input>>,''','''    placeholders: ManuallyDrop<Option<Vec<super::type_input::Input>>>,
    fields: ManuallyDrop<Option<Vec<Field>>>,
    copied_rows: ManuallyDrop<Option<Vec<DslValue>>>,
    copied_row: ManuallyDrop<Option<DslValue>>,''')
replace('    value: IfcValue,\n', '')
replace('''            placeholder: ManuallyDrop::new(None),''','''            placeholders: ManuallyDrop::new(None),
            fields: ManuallyDrop::new(None),
            copied_rows: ManuallyDrop::new(None),
            copied_row: ManuallyDrop::new(None),''')
replace('            value: IfcValue::Unset,\n','')
a=s.index('        match self.state.phase {',s.index('pub fn demands'));b=s.index('        Ok(demand)',a)
s=s[:a]+'''        match self.state.phase {
            23 => demand.capacity_bytes=self.state.target*(size_of::<super::type_input::Input>()+size_of::<Field>()),
            3 => { demand.copy_bytes += super::type_remap::Plan::copy_bytes()+size_of::<super::type_input::Input>(); demand.capacity_bytes=self.plan.capacity_bytes(self.state.target)?; }
            5 => { demand.copy_bytes += 4*size_of::<DomainCursor>(); demand.capacity_bytes=256; }
            7 if self.error.is_some() || !self.retirement.is_empty() => return self.retirement.next_owner_demands::<String>(self.error.is_some(), &None,4096),
            6 => demand.copy_bytes += size_of::<Page>()+2*size_of::<StagedFormRequest>()+size_of::<Vec<Field>>(),
            8 | 24 => { let page=self.page.as_ref().ok_or_else(stale)?;return Ok(text(token_part(&page.request.as_ref().ok_or_else(stale)?.token,self.state.part))); }
            9 | 10 => return Ok(text("0")),
            11 => { let page=self.page.as_ref().ok_or_else(stale)?;let (name,_)=super::type_input::request(marker(page.request.as_ref().ok_or_else(stale)?)?.1).ok_or_else(invalid)?;let mut bytes=[0;256];let name=super::domain_cursor::canonical_name(name,&mut bytes).ok_or_else(invalid)?;return Ok(text(C::normalization_attribute(name,page.fields[self.state.field].index,true).map_err(|_|invalid())?.ok_or_else(invalid)?.name)); }
            12 => { demand.capacity_bytes=if self.current_field()?.present{1}else{2}*size_of::<StagedFormBindingEdit>(); }
            13 => { demand.capacity_bytes=(rows(self.page.as_ref().ok_or_else(stale)?.request.as_ref().ok_or_else(stale)?)?.len()+1)*size_of::<DslValue>(); }
            14 => { let (index,text)=self.copying_row()?;let mut buffer=[0;20];demand.capacity_bytes=row_capacity(decimal(index,&mut buffer),text);demand.copy_bytes+=demand.capacity_bytes-text.len()+size_of::<DslValue>(); }
            20 => { demand.copy_bytes+=64+size_of::<String>(); }
            21 => demand.copy_bytes+=2*size_of::<DslValue>(),
            26 => { demand.capacity_bytes=2*size_of::<String>()+15;demand.copy_bytes+=demand.capacity_bytes+size_of::<StagedFormBindingEdit>()+size_of::<Vec<DslValue>>(); }
            15 => { let mut buffer=[0;20];demand.capacity_bytes=5*size_of::<String>()+25+decimal(self.current_field()?.row,&mut buffer).len();demand.copy_bytes+=demand.capacity_bytes+size_of::<StagedFormBindingEdit>(); }
            16 | 27 => demand.copy_bytes+=size_of::<StagedFormBindingEdit>(),
            17 if !self.retirement.is_empty() => return self.retirement.next_owner_demands::<DslValue>(false,&None,4096),
            17 if self.plan.map.is_some() => return self.retirement.next_owner_demands::<Vec<usize>>(true,&None,4096),
            17 if self.placeholders.is_some() => return self.retirement.next_owner_demands::<Vec<super::type_input::Input>>(true,&None,4096),
            _=>{}
        }
''' + s[b:]
a=s.index('            3 => {',s.index('fn advance_inner'));b=s.index('            22 => {',a)
s=s[:a]+'''            23 => {
                let capacity=self.state.target*(size_of::<super::type_input::Input>()+size_of::<Field>());
                let physical=OriginalConstructionDemand{items:1,copy_bytes:base,capacity_bytes:capacity,release_bytes:0,depth:1};
                let mut output=None;
                if admit_original_construction(cx,&mut output,physical,|_|{
                    *self.placeholders=Some(Vec::new());*self.fields=Some(Vec::new());
                    self.placeholders.as_mut().unwrap().try_reserve_exact(self.state.target).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"type required roster"))?;
                    self.fields.as_mut().unwrap().try_reserve_exact(self.state.target).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"type required fields"))?;
                    Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:base,retained_capacity_bytes:self.placeholders.as_ref().unwrap().capacity()*size_of::<super::type_input::Input>()+self.fields.as_ref().unwrap().capacity()*size_of::<Field>(),..Default::default()}))
                })? {self.state.phase=3;}
            }
            3 => {
                let entity=&source.entities[self.state.position];let (_,value)=marker(self.request.as_ref().ok_or_else(stale)?)?;let(name,_)=super::type_input::request(value).ok_or_else(invalid)?;let mut bytes=[0;256];let name=super::domain_cursor::canonical_name(name,&mut bytes).ok_or_else(invalid)?;
                if let Some(index)=self.plan.required_frontier(entity.args.len()).filter(|index|self.placeholders.as_ref().unwrap().last().is_none_or(|input|input.index!=*index)){
                    if self.placeholders.as_ref().unwrap().len()>=STAGED_FORM_BODY_LIMITS.content{return Err(invalid().into());}
                    let attribute=C::normalization_attribute(name,index,true).map_err(|_|invalid())?.ok_or_else(invalid)?;let domain=attribute.type_expr.projection();
                    if domain.kind!="named"||domain.name.is_none_or(|name|C::direct_attribute_count(name).is_none()){return Err(invalid().into());}
                    metadata(cx,base+size_of::<super::type_input::Input>(),||self.placeholders.as_mut().unwrap().push(super::type_input::Input{index,value:Some(IfcValue::Unset)}))?;
                }else{
                    let mut previous=[0;256];let previous=super::domain_cursor::canonical_name(&entity.name,&mut previous).ok_or_else(invalid)?;let demand=RetirementDemand{copy_bytes:base+super::type_remap::Plan::copy_bytes(),capacity_bytes:self.plan.capacity_bytes(self.state.target)?,depth:1,..Default::default()};
                    if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),demand){return Ok(false);}
                    let(complete,capacity)=self.plan.advance::<C>(previous,name,&entity.args,self.state.target,self.placeholders.as_deref())?;
                    cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:capacity,..Default::default()})?;cx.consume_fuel(1);
                    if complete{self.state.phase=4;self.state.row=0;self.state.part=0;}
                }
            }
            4 => {
                let request=self.request.as_ref().ok_or_else(stale)?;let values=self.placeholders.as_ref().ok_or_else(invalid)?;
                if self.state.part==values.len(){if values.is_empty()||self.state.row!=rows(request)?.len(){return Err(invalid().into());}metadata(cx,base,||self.state.phase=22)?;return Ok(false);}
                let index=values[self.state.part].index;let current=if self.state.row<rows(request)?.len(){Some(row_text(request,self.state.row)?)}else{None};if current.is_some_and(|(ordinal,_)|ordinal<index){return Err(invalid().into());}
                let present=current.is_some_and(|(ordinal,_)|ordinal==index);let id=current.filter(|_|present).and_then(|(_,text)|canonical_decimal(text,true));
                metadata(cx,base+size_of::<Field>()+size_of::<super::type_input::Input>(),||{
                    self.placeholders.as_mut().unwrap()[self.state.part].value=Some(id.map_or(IfcValue::Unset,IfcValue::Reference));self.state.ready&=id.is_some();
                    self.fields.as_mut().unwrap().push(Field{binding:StagedFormBinding{token:empty_token(),layout:if present{StagedFormLayout::Preserve}else{StagedFormLayout::Replace},edits:Vec::new()},attribute:String::new(),index,row:self.state.row,present});self.state.row+=usize::from(present);self.state.part+=1;
                })?;
            }
''' + s[b:]
replace('''                    self.state.phase = 3;
                })?;
            }
            23''','''                    self.state.phase = 23;
                })?;
            }
            23''')
replace('''                        } else if super::type_input::input_ordinal(ordinal).is_some() {
                            Some(&self.value)''','''                        } else if let Some(input) = super::type_input::input_ordinal(ordinal) {
                            self.placeholders.as_ref()?.get(input)?.value.as_ref()''')
replace('''                if super::type_input::input_ordinal(ordinal).is_some() && matches!(self.value, IfcValue::Unset) {''','''                if super::type_input::input_ordinal(ordinal).is_some_and(|input|matches!(self.placeholders.as_ref().and_then(|values|values.get(input)).and_then(|value|value.value.as_ref()),Some(IfcValue::Unset))) {''')
replace('''                } else if super::type_input::input_ordinal(ordinal).is_some() {
                    &self.value''','''                } else if let Some(input) = super::type_input::input_ordinal(ordinal) {
                    self.placeholders.as_ref().and_then(|values|values.get(input)).and_then(|value|value.value.as_ref()).ok_or_else(invalid)?''')
a=s.index('            6 => {',s.index('fn advance_inner'));b=s.index('            18 => return Ok(true),',a)
s=s[:a]+'''            6 => {
                metadata(cx,base+size_of::<Page>()+2*size_of::<StagedFormRequest>()+size_of::<Vec<Field>>(),||{
                    *self.page=Some(Page{request:self.request.take(),status:StagedFormStatus{token:empty_token(),ready:self.state.ready,diagnostics:Vec::new(),diagnostic_offset:String::new(),diagnostic_total:String::new()},fields:self.fields.take().unwrap()});self.state.part=0;self.state.field=0;self.state.phase=8;
                })?;
            }
            8 | 24 => {
                let page=self.page.as_mut().ok_or_else(stale)?;
                if self.state.phase==24 && self.state.field==page.fields.len(){metadata(cx,base,||self.state.phase=17)?;return Ok(false);}
                let token=&page.request.as_ref().ok_or_else(stale)?.token;
                let target=if self.state.phase==8{token_part_mut(&mut page.status.token,self.state.part)}else{token_part_mut(&mut page.fields[self.state.field].binding.token,self.state.part)};
                if copy_text(target,token_part(token,self.state.part),cx)?{self.state.part+=1;if self.state.part==4{self.state.part=0;self.state.phase=if self.state.phase==8{9}else{11};}}
            }
            9 | 10 => {
                let page=self.page.as_mut().ok_or_else(stale)?;let target=if self.state.phase==9{&mut page.status.diagnostic_offset}else{&mut page.status.diagnostic_total};
                if copy_text(target,"0",cx)?{self.state.phase=if self.state.phase==9{10}else{24};}
            }
            11 => {
                let page=self.page.as_mut().ok_or_else(stale)?;let(name,_)=super::type_input::request(marker(page.request.as_ref().ok_or_else(stale)?)?.1).ok_or_else(invalid)?;let mut bytes=[0;256];let name=super::domain_cursor::canonical_name(name,&mut bytes).ok_or_else(invalid)?;let field=&mut page.fields[self.state.field];let attribute=C::normalization_attribute(name,field.index,true).map_err(|_|invalid())?.ok_or_else(invalid)?;
                if copy_text(&mut field.attribute,attribute.name,cx)?{self.state.phase=12;}
            }
            12 => {
                let page=self.page.as_mut().ok_or_else(stale)?;let field=&mut page.fields[self.state.field];let count=if field.present{1}else{2};let mut output=None;let physical=OriginalConstructionDemand{items:1,copy_bytes:base,capacity_bytes:count*size_of::<StagedFormBindingEdit>(),release_bytes:0,depth:1};
                if admit_original_construction(cx,&mut output,physical,|_|{field.binding.edits.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"type guide bindings"))?;Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:base,retained_capacity_bytes:field.binding.edits.capacity()*size_of::<StagedFormBindingEdit>(),..Default::default()}))})?{self.state.phase=if field.present{15}else{13};}
            }
            13 => {
                let count=rows(self.page.as_ref().ok_or_else(stale)?.request.as_ref().ok_or_else(stale)?)?.len()+1;let physical=OriginalConstructionDemand{items:1,copy_bytes:base,capacity_bytes:count*size_of::<DslValue>(),release_bytes:0,depth:1};
                if admit_original_construction(cx,&mut self.copied_rows,physical,|_|{let mut rows=Vec::new();rows.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"type guide sorted rows"))?;let capacity=rows.capacity()*size_of::<DslValue>();Ok((rows,RetainedCloneProgress{copied_items:1,copied_bytes:base,retained_capacity_bytes:capacity,..Default::default()}))})?{self.state.copying=0;self.state.offset=0;self.state.phase=14;}
            }
            14 => {
                let(index,text)=self.copying_row()?;let mut buffer=[0;20];let index=decimal(index,&mut buffer);let capacity=row_capacity(index,text);let physical=OriginalConstructionDemand{items:1,copy_bytes:base+capacity-text.len()+size_of::<DslValue>(),capacity_bytes:capacity,release_bytes:0,depth:1};
                let mut row=None;
                if admit_original_construction(cx,&mut row,physical,|_|{let(row,capacity)=make_row(index,text)?;Ok((row,RetainedCloneProgress{copied_items:1,copied_bytes:physical.copy_bytes,retained_capacity_bytes:capacity,..Default::default()}))})?{*self.copied_row=row;self.state.offset=0;self.state.phase=20;}
            }
            20 => {
                let(_,text)=self.copying_row()?;let mut end=(self.state.offset+64).min(text.len());while !text.is_char_boundary(end){end-=1;}let chunk=&text[self.state.offset..end];let mut chunk_buffer=[0u8;64];chunk_buffer[..chunk.len()].copy_from_slice(chunk);let length=chunk.len();let complete=end==text.len();
                metadata(cx,base+64+size_of::<String>(),||{row_value_mut(self.copied_row.as_mut().unwrap()).expect("closed row").push_str(std::str::from_utf8(&chunk_buffer[..length]).expect("UTF8 boundary"));self.state.offset=end;if complete{self.state.phase=21;}})?;
            }
            21 => {
                let count=rows(self.page.as_ref().ok_or_else(stale)?.request.as_ref().ok_or_else(stale)?)?.len()+1;
                metadata(cx,base+2*size_of::<DslValue>(),||{self.copied_rows.as_mut().unwrap().push(self.copied_row.take().unwrap());self.state.copying+=1;self.state.phase=if self.state.copying==count{26}else{14};})?;
            }
            26 => {
                let capacity=2*size_of::<String>()+15;let physical=OriginalConstructionDemand{items:1,copy_bytes:base+capacity+size_of::<StagedFormBindingEdit>()+size_of::<Vec<DslValue>>(),capacity_bytes:capacity,release_bytes:0,depth:1};
                if admit_original_construction(cx,&mut self.edit,physical,|_|{Ok((StagedFormBindingEdit::Fixed{edit:StagedFormEdit::Set{path:vec!["value".into(),"$ifcInputs".into()],value:DslValue::Array(self.copied_rows.take().unwrap())}},RetainedCloneProgress{copied_items:1,copied_bytes:physical.copy_bytes,retained_capacity_bytes:capacity,..Default::default()}))})?{self.state.phase=27;}
            }
            27 => {
                metadata(cx,base+size_of::<StagedFormBindingEdit>(),||{self.page.as_mut().unwrap().fields[self.state.field].binding.edits.push(self.edit.take().unwrap());self.state.phase=15;})?;
            }
            15 => {
                let mut buffer=[0;20];let row=decimal(self.current_field()?.row,&mut buffer);let capacity=5*size_of::<String>()+25+row.len();let physical=OriginalConstructionDemand{items:1,copy_bytes:base+capacity+size_of::<StagedFormBindingEdit>(),capacity_bytes:capacity,release_bytes:0,depth:1};
                if admit_original_construction(cx,&mut self.edit,physical,|_|{Ok((StagedFormBindingEdit::Text{path:vec!["value".into(),"$ifcInputs".into(),row.into(),"value".into(),"value".into()]},RetainedCloneProgress{copied_items:1,copied_bytes:physical.copy_bytes,retained_capacity_bytes:capacity,..Default::default()}))})?{self.state.phase=16;}
            }
            16 => {
                metadata(cx,base+size_of::<StagedFormBindingEdit>(),||{self.page.as_mut().unwrap().fields[self.state.field].binding.edits.push(self.edit.take().unwrap());self.state.field+=1;self.state.part=0;self.state.phase=24;})?;
            }
            17 => {
                if self.plan.map.is_some() || self.placeholders.is_some() || !self.retirement.is_empty(){
                    let demand=self.demands()?;
                    semio_framework_job::close_original_retained_step(cx,demand,|grant|if !self.retirement.is_empty(){self.retirement.step_granted(grant)}else if self.plan.map.is_some(){self.retirement.begin_granted(&mut self.plan.map,grant)?.ok_or_else(invalid)}else{self.retirement.begin_granted(&mut self.placeholders,grant)?.ok_or_else(invalid)})?;
                }else{metadata(cx,base,||{self.plan.clear_metadata();self.state.phase=18;})?;}
            }
''' + s[b:]
replace('''    pub fn original_request_received(&self) -> bool {''','''    fn current_field(&self) -> Result<&Field, ValueError> { self.page.as_ref().and_then(|page|page.fields.get(self.state.field)).ok_or_else(stale) }
    fn copying_row(&self) -> Result<(usize, &str), ValueError> {
        let field=self.current_field()?;if self.state.copying==field.row{return Ok((field.index,""));}
        let ordinal=self.state.copying-usize::from(self.state.copying>field.row);
        row_text(self.page.as_ref().ok_or_else(stale)?.request.as_ref().ok_or_else(stale)?,ordinal)
    }
    pub fn original_request_received(&self) -> bool {''')
replace('self.placeholder.is_none()', 'self.placeholders.is_none() && self.fields.is_none() && self.copied_rows.is_none() && self.copied_row.is_none()')
replace('''        owner!(&self.page, Page);''','''        owner!(&self.copied_row, DslValue);
        owner!(&self.copied_rows, Vec<DslValue>);
        owner!(&self.fields, Vec<Field>);
        owner!(&self.page, Page);''')
replace('owner!(&self.placeholder, super::type_input::Input);','owner!(&self.placeholders, Vec<super::type_input::Input>);')
replace('''        owner!(&mut self.page);''','''        owner!(&mut self.copied_row);
        owner!(&mut self.copied_rows);
        owner!(&mut self.fields);
        owner!(&mut self.page);''')
replace('owner!(&mut self.placeholder);','owner!(&mut self.placeholders);')
data=s.encode();old=p.stat().st_size
with p.open('r+b')as output:output.write(data);output.write(b' '*max(0,old-len(data)))
