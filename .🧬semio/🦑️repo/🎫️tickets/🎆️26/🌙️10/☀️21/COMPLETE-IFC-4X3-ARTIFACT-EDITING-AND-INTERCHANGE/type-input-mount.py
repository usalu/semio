from pathlib import Path
home=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details')
def update(name, changes):
    path=home/name
    data=path.read_text(encoding='utf-8')
    for old,new in changes:
        assert data.count(old)==1,(name,old[:100],data.count(old))
        data=data.replace(old,new,1)
    result=data.encode('utf-8');original=path.stat().st_size
    with path.open('r+b') as output:
        output.write(result)
        if len(result)<original:output.write(b' '*(original-len(result)))
update('🦀️.rs',[
('#[path="type_remap.rs"]mod type_remap;','#[path="type_remap.rs"]mod type_remap;\n#[path="type_input.rs"]mod type_input;'),
('''    let DslValue::Object(fields) = value else { return None };
    let [(key, DslValue::String(name))] = fields.as_slice() else { return None };
    (key == "$ifcType").then_some(name.as_str())''','''    type_input::request(value).map(|(name,_)|name)''')])
update('work_tests.rs',[('#[path="primary_remap_tests.rs"]mod primary_remap_tests;','#[path="primary_remap_tests.rs"]mod primary_remap_tests;\n#[path="type_input_tests.rs"]mod type_input_tests;')])
update('type_remap.rs',[
('    reuse: usize,','    reuse: usize,\n    supplied: usize,'),
('values: &[IfcValue], count: usize)', 'values: &[IfcValue], count: usize, supplied: Option<&[super::type_input::Input]>)'),
('''                if self.target == count {
                    return Ok((true, 0));
                }''','''                if supplied.and_then(|values|values.get(self.supplied)).is_some_and(|value|value.index<self.target)||self.target==count&&supplied.is_some_and(|values|values.len()!=self.supplied){return Err(invalid("IFC type input cannot override a compatible source slot"));}
                if self.target == count {
                    return Ok((true, 0));
                }'''),
('''                    if !self.target_attribute.ok_or_else(|| invalid("IFC retype target metadata missing"))?.optional {
                        return Err(invalid("IFC retype required slot needs an explicit value"));
                    }
                    self.finish_slot(usize::MAX)?;''','''                    if let Some(value)=supplied.and_then(|values|values.get(self.supplied)).filter(|value|value.index==self.target){
                        if self.target_attribute.ok_or_else(||invalid("IFC retype target metadata missing"))?.optional||value.value.is_none(){return Err(invalid("IFC supplied type value must fill a required target slot"));}
                        self.finish_slot(super::type_input::INPUT|self.supplied)?;self.supplied+=1;
                    }else{
                        if !self.target_attribute.ok_or_else(|| invalid("IFC retype target metadata missing"))?.optional {return Err(invalid("IFC retype required slot needs an explicit value"));}
                        self.finish_slot(usize::MAX)?;
                    }'''),
('pub(super) fn metadata(&self) -> [usize; 17]', 'pub(super) fn metadata(&self) -> [usize; 18]'),
('''            self.reuse,
''','''            self.reuse,
            self.supplied,
''')])
update('constituent_roster.rs',[
('    root_slots: Option<Vec<usize>>,','    root_slots: Option<Vec<usize>>,\n    inputs: super::type_input::Inputs,'),
('''        let DslValue::Object(fields) = value else {
            return None;
        };
        let [(key, DslValue::String(name))] = fields.as_mut_slice() else {
            return None;
        };
        (key.as_str() == "$ifcType").then_some(name)''','''        super::type_input::request_name_mut(value)'''),
('''            15 => {
                demand.copy_bytes''','''            22 => {
                return Err(invalid("IFC type input demand requires original event"));
            }
            15 => {
                demand.copy_bytes'''),
('''        let demand = self.demands(Some(source), cursor, value_cursor, snapshot, body)?;''','''        let demand = if self.state.phase==22 { self.input_demands(event,body)? } else { self.demands(Some(source), cursor, value_cursor, snapshot, body)? };'''),
('''        if self.state.phase == 19 || self.state.phase == 20 && !value_cursor.terminal_is_empty() {''','''        if self.state.phase==22 {
            cx.consume_retained(RetainedCloneProgress{copied_bytes:Self::base().copy_bytes,..Default::default()})?;
            let mut bytes=[0;256];let name=Self::type_name(event,address).and_then(|name|super::domain_cursor::canonical_name(name,&mut bytes)).ok_or_else(||invalid("IFC type input target changed"))?;
            if self.inputs.advance::<C>(event,name,self.state.target_arity,cx)?{self.state.phase=15;}
            return Ok(false);
        }
        if self.state.phase == 19 || self.state.phase == 20 && !value_cursor.terminal_is_empty() {'''),
('''                    let mapping = complete && (!self.state.primary || !entity.complex.is_empty()) && old != next && old != Some(0) && next != Some(0);''','''                    let mapping = complete && (!self.state.primary || !entity.complex.is_empty()) && old != next && old != Some(0) && next != Some(0);
                    let supplied=super::type_input::rows(event).is_some();
                    if supplied&&!(self.state.primary&&mapping){return Err(invalid("IFC type inputs require complete complex-primary differing-arity remap").into());}'''),
('''                    self.state.phase = if mapping {
                        15''','''                    self.state.phase = if mapping {
                        if supplied {22}else{15}'''),
('self.remap.advance::<C>(previous, target, args, self.state.target_arity)?','self.remap.advance::<C>(previous, target, args, self.state.target_arity, self.inputs.values())?'),
('''                            if previous == usize::MAX { Some(&IfcValue::Unset) } else { entity.args.get(previous) }''','''                            if previous == usize::MAX { Some(&IfcValue::Unset) } else if let Some(ordinal)=super::type_input::input_ordinal(previous){self.inputs.argument(ordinal)}else { entity.args.get(previous) }'''),
('''                    } else {
                        if value.is_some() || !value_cursor.terminal_is_empty() {''','''                    } else if let Some(ordinal)=self.remap.map.as_ref().and_then(|map|map.get(index)).and_then(|value|super::type_input::input_ordinal(*value)) {
                        if value.is_some(){return Err(invalid("IFC supplied type value receiver occupied").into());}
                        *value=Some(self.inputs.take_argument(ordinal).ok_or_else(||invalid("IFC supplied type argument missing"))?);self.state.phase=21;
                    } else {
                        if value.is_some() || !value_cursor.terminal_is_empty() {'''),
('''        self.state.closing = true;
''','''        self.state.closing = true;
        self.inputs.begin_close();
'''),
('&& self.root_slots.is_none() && self.retirement.is_empty()', '&& self.root_slots.is_none() && self.inputs.terminal_is_empty() && self.retirement.is_empty()'),
('''        if !self.retirement.is_empty() {
            self.retirement.next_owner_demands''','''        if !self.inputs.terminal_is_empty(){return self.inputs.close_demands(body);}
        if !self.retirement.is_empty() {
            self.retirement.next_owner_demands'''),
('''        if !self.retirement.is_empty() {
            return self.retirement.step_granted(grant);''','''        if !self.inputs.terminal_is_empty(){return self.inputs.close_step(grant);}
        if !self.retirement.is_empty() {
            return self.retirement.step_granted(grant);'''),
('[usize; 17], usize, usize)', '[usize; 18], usize, usize, (super::type_input::State,usize,usize,bool,bool))'),
('''            self.root_slots.as_ref().map_or(0, Vec::capacity),
''','''            self.root_slots.as_ref().map_or(0, Vec::capacity),
            self.inputs.metadata(),
''')])
