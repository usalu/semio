import {readFileSync,writeFileSync,renameSync,mkdirSync} from 'node:fs';
import {join,dirname} from 'node:path';
const generated=join(dirname(import.meta.path),'🗑️generated');mkdirSync(generated,{recursive:true});const root='🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/';
function edit(path:string,update:(s:string)=>string){const file=root+path,old=readFileSync(file,'utf8'),next=update(old);const temp=join(generated,'collections-'+Date.now()+'.rs');writeFileSync(temp,next);if(readFileSync(file,'utf8')!==old)throw Error('concurrent '+path);renameSync(temp,file);}
function rep(s:string,a:string,b:string){if(s.split(a).length!==2)throw Error('anchor '+a.slice(0,120));return s.replace(a,b);}
edit('📋️paged-list/🦀️.rs',s=>{
 s=rep(s,'body: usize) -> Result<crate::RetirementDemand, crate::ValueError>','body: usize, maximum_work_units:u64) -> Result<crate::RetirementDemand, crate::ValueError>');
 s=rep(s,'if let Some(demand)=source.binding_demand(&self.source)?{return Ok(demand);}','let binding=source.binding_demand(&self.source)?;if maximum_work_units==0{return Ok(Default::default())}if let Some(demand)=binding{return Ok(demand);}');
 s=rep(s,'capacity_bytes: self.values.next_capacity_allocation_bytes(length)?','copy_bytes:self.values.next_capacity_copy_byte_demand(length).map_err(crate::ValueError::from)?,capacity_bytes: self.values.next_capacity_allocation_bytes(length)?');
 s=rep(s,'if self.child_value.is_some() {\n                return if child.terminal_is_empty()', 'if self.child_value.is_some() {\n                if !self.child_closing{return Ok(crate::RetirementDemand{depth:1,..Default::default()})}\n                return if child.terminal_is_empty()');
 s=rep(s,')), body)?;demand.depth=', ')), body,maximum_work_units)?;demand.depth=');
 s=rep(s,'    fn advance(&mut self, source:',`    fn advance_work_demand(&self,source:RetainedCloneRef<'_,PagedList<T,N>>,body:usize,maximum_work_units:u64)->Result<u64,crate::ValueError>{
        self.advance_demands(source,body,maximum_work_units)?;if maximum_work_units==0||self.output_ready(){return Ok(0)}if self.source.is_none()||self.values.capacity()<source.get().len(){return Ok(1)}
        let Some(child)=self.child.as_ref()else{return Ok(1)};
        if self.child_value.is_some(){return if !self.child_closing{child.begin_close_work_demand()?.checked_add(1).ok_or_else(||crate::ValueError::invariant("paged child close intent work overflow"))}else if child.terminal_is_empty(){Ok(2)}else{child.close_work_demand(body)}}
        if child.output_ready(){return child.take_work_demand()?.checked_add(1).ok_or_else(||crate::ValueError::invariant("paged child transfer work overflow"))}
        child.advance_work_demand(source.project(self.index+1,|values|values.get(self.index).expect("validated paged list source ordinal")),body,maximum_work_units)
    }
    fn take_work_demand(&self)->Result<u64,crate::ValueError>{Ok(u64::from(self.output_ready()))}
    fn begin_close_work_demand(&self)->Result<u64,crate::ValueError>{Ok(u64::from(!self.closing))}
    fn begin_close_paid(&mut self)->crate::RetainedWorkOutcome<(),crate::ValueError>{let work=u64::from(self.begin_close());crate::RetainedWorkOutcome{result:Ok(()),work}}
    fn close_work_demand(&self,body:usize)->Result<u64,crate::ValueError>{
        if !self.closing||self.terminal_is_empty(){return Ok(0)}if let Some(child)=self.child.as_ref(){return if !self.child_closing{child.begin_close_work_demand()?.checked_add(1).ok_or_else(||crate::ValueError::invariant("paged child close work overflow"))}else if child.terminal_is_empty(){Ok(1)}else{child.close_work_demand(body)}}
        if !self.close.is_empty(){return self.close.next_owner_work::<T>(false,&None,body)}if self.child_value.is_some(){return self.close.next_owner_work::<T>(true,&None,body)}self.close.next_owner_work::<PagedList<T,N>>(self.output.is_some()||!self.values.terminal_is_empty(),&self.source,body)
    }
    fn advance(&mut self, source:`);
 s=rep(s,'body: usize, grant: RetainedCloneGrant','body: usize, maximum_work_units:u64, grant: RetainedCloneGrant');
 s=rep(s,'let demand = self.advance_demands(source, body)?;','let selected=self.advance_work_demand(source,body,maximum_work_units)?;if selected==0||selected>maximum_work_units{return Ok(RetainedCloneStep::Progress(Default::default()))}let demand = self.advance_demands(source, body,maximum_work_units)?;');
 s=rep(s,'copied_items: 1, retained_capacity_bytes: progress.allocated_bytes','copied_items: 1,copied_bytes:demand.copy_bytes, retained_capacity_bytes: progress.allocated_bytes');
 s=rep(s,'if state.child_value.is_some() {\n                if !child.terminal_is_empty()', 'if state.child_value.is_some() {\n                if !state.child_closing{let turn=child.begin_close_paid();work+=turn.work;turn.result?;state.child_closing=true;work+=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}\n                if !child.terminal_is_empty()');
 s=rep(s,'state.child = None;state.child_closing=false;\n                state.index += 1;', 'state.child = None;state.child_closing=false;\n                state.index += 1;work+=1;');
 s=rep(s,'state.child_value=Some(value);\n                child.begin_close();state.child_closing=true;', 'state.child_value=Some(value);work+=1;');
 s=rep(s,'?,body, nested);','?,body,maximum_work_units, nested);');
 s=rep(s,'if !state.child_closing {child.begin_close();state.child_closing=true;work+=1;', 'if !state.child_closing {let turn=child.begin_close_paid();work+=turn.work;turn.result?;state.child_closing=true;work+=1;');
 return s;
});
edit('🧺️std-collections/🦀️.rs',s=>{
 s=rep(s,'body: usize) -> Result<crate::RetirementDemand, crate::ValueError>','body: usize, maximum_work_units:u64) -> Result<crate::RetirementDemand, crate::ValueError>');
 s=rep(s,'if !source.check_binding(&self.source)? {','let bound=source.check_binding(&self.source)?;if maximum_work_units==0{return Ok(Default::default())}if !bound {');
 s=rep(s,'entry").0),body),','entry").0),body,maximum_work_units),');
 s=rep(s,'entry").1),body),','entry").1),body,maximum_work_units),');
 s=rep(s,'KEY_CLOSE if !self.key_cursor.terminal_is_empty() => return', 'KEY_CLOSE if !self.key_cursor.close_started()=>{},\n            KEY_CLOSE if !self.key_cursor.terminal_is_empty() => return');
 s=rep(s,'VALUE_CLOSE if !self.value_cursor.terminal_is_empty() => return', 'VALUE_CLOSE if !self.value_cursor.close_started()=>{},\n            VALUE_CLOSE if !self.value_cursor.terminal_is_empty() => return');
 s=rep(s,'    fn advance(&mut self, source:',`    fn advance_work_demand(&self,source:RetainedCloneRef<'_,M>,body:usize,maximum_work_units:u64)->Result<u64,crate::ValueError>{
        self.advance_demands(source,body,maximum_work_units)?;if maximum_work_units==0||self.output_ready(){return Ok(0)}if self.source.is_none(){return Ok(1)}
        match self.phase{
            KEY if self.key_cursor.output_ready()=>self.key_cursor.take_work_demand()?.checked_add(1).ok_or_else(||crate::ValueError::invariant("collection key transfer work overflow")),
            KEY=>self.key_cursor.advance_work_demand(source.project(self.index*2+1,|map|map.entry_at(self.index).expect("validated standard collection entry").0),body,maximum_work_units),
            KEY_CLOSE if !self.key_cursor.close_started()=>self.key_cursor.begin_close_work_demand(),
            KEY_CLOSE if !self.key_cursor.terminal_is_empty()=>self.key_cursor.close_work_demand(body),
            VALUE if self.value_cursor.output_ready()=>self.value_cursor.take_work_demand()?.checked_add(1).ok_or_else(||crate::ValueError::invariant("collection value transfer work overflow")),
            VALUE=>self.value_cursor.advance_work_demand(source.project(self.index*2+2,|map|map.entry_at(self.index).expect("validated standard collection entry").1),body,maximum_work_units),
            VALUE_CLOSE if !self.value_cursor.close_started()=>self.value_cursor.begin_close_work_demand(),
            VALUE_CLOSE if !self.value_cursor.terminal_is_empty()=>self.value_cursor.close_work_demand(body),
            DONE=>Ok(0),_=>Ok(1)
        }
    }
    fn take_work_demand(&self)->Result<u64,crate::ValueError>{Ok(u64::from(self.output_ready()))}
    fn begin_close_work_demand(&self)->Result<u64,crate::ValueError>{Ok(u64::from(!self.closing))}
    fn begin_close_paid(&mut self)->crate::RetainedWorkOutcome<(),crate::ValueError>{let work=u64::from(self.begin_close());crate::RetainedWorkOutcome{result:Ok(()),work}}
    fn close_work_demand(&self,body:usize)->Result<u64,crate::ValueError>{
        if !self.closing||self.terminal_is_empty(){return Ok(0)}if !self.key_cursor.terminal_is_empty(){return if !self.key_cursor.close_started(){self.key_cursor.begin_close_work_demand()}else{self.key_cursor.close_work_demand(body)}}if !self.value_cursor.terminal_is_empty(){return if !self.value_cursor.close_started(){self.value_cursor.begin_close_work_demand()}else{self.value_cursor.close_work_demand(body)}}
        if !self.close.is_empty(){return self.close.next_owner_work::<M>(false,&None,body)}if self.key.is_some(){return self.close.next_owner_work::<M::Key>(true,&None,body)}if self.value.is_some(){return self.close.next_owner_work::<M::Value>(true,&None,body)}self.close.next_owner_work::<M>(self.building.is_some()||self.output.is_some(),&self.source,body)
    }
    fn advance(&mut self, source:`);
 s=rep(s,'body: usize, grant: RetainedCloneGrant','body: usize, maximum_work_units:u64, grant: RetainedCloneGrant');
 s=rep(s,'let demand=self.advance_demands(source,body)?;', 'let selected=self.advance_work_demand(source,body,maximum_work_units)?;if selected==0||selected>maximum_work_units{return Ok(RetainedCloneStep::Progress(Default::default()))}let demand=self.advance_demands(source,body,maximum_work_units)?;');
 s=rep(s,'self.key=Some(value);\n                    self.key_cursor.begin_close();','self.key=Some(value);work+=1;');
 s=rep(s,'self.value=Some(value);\n                    self.value_cursor.begin_close();','self.value=Some(value);work+=1;');
 s=rep(s,'entry").0),body,nested);','entry").0),body,maximum_work_units,nested);');
 s=rep(s,'entry").1),body,nested);','entry").1),body,maximum_work_units,nested);');
 s=rep(s,'KEY_CLOSE => {\n                if !self.key_cursor.terminal_is_empty()', 'KEY_CLOSE => {\n                if !self.key_cursor.close_started(){let turn=self.key_cursor.begin_close_paid();work+=turn.work;turn.result?;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}\n                if !self.key_cursor.terminal_is_empty()');
 s=rep(s,'VALUE_CLOSE => {\n                if !self.value_cursor.terminal_is_empty()', 'VALUE_CLOSE => {\n                if !self.value_cursor.close_started(){let turn=self.value_cursor.begin_close_paid();work+=turn.work;turn.result?;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}\n                if !self.value_cursor.terminal_is_empty()');
 s=rep(s,'if self.key_cursor.begin_close() {work+=1;', 'if !self.key_cursor.close_started() {let turn=self.key_cursor.begin_close_paid();work+=turn.work;turn.result?;');
 s=rep(s,'if self.value_cursor.begin_close() {work+=1;', 'if !self.value_cursor.close_started() {let turn=self.value_cursor.begin_close_paid();work+=turn.work;turn.result?;');
 s=rep(s,'fn output_ready(&self)->bool{self.output.is_some()}','fn output_ready(&self)->bool{!self.closing&&!self.spent&&self.output.is_some()}\n    fn close_started(&self)->bool{self.closing}');
 return s;
});
console.log('collection work propagation applied');
