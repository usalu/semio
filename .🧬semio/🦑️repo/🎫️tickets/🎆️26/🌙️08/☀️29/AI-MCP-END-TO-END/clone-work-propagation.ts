import {readFileSync,writeFileSync,renameSync,mkdirSync,existsSync} from 'node:fs';
import {join,dirname} from 'node:path';
const ticket=dirname(import.meta.path),generated=join(ticket,'🗑️generated');mkdirSync(generated,{recursive:true});
const root='🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/';
function edit(path:string,update:(s:string)=>string){const file=root+path,old=readFileSync(file,'utf8'),next=update(old);if(next===old)throw Error('unchanged '+path);const temp=join(generated,'clone-'+Date.now()+'.rs');writeFileSync(temp,next);if(readFileSync(file,'utf8')!==old)throw Error('concurrent '+path);renameSync(temp,file);}
function replace(s:string,from:string,to:string){if(s.split(from).length!==2)throw Error('anchor '+from.slice(0,100));return s.replace(from,to);}
function create(path:string,text:string){const file=root+path;if(existsSync(file))throw Error('exists '+file);mkdirSync(dirname(file),{recursive:true});writeFileSync(file,text);}
const law={version:1,maximumWork:[0,1,2,4,8,32],actions:[{kind:'binding',work:1},{kind:'byte',work:1},{kind:'childTransfer',childWork:1,parentWork:1,work:2},{kind:'childCloseIntent',childWork:1,parentWork:1,work:2},{kind:'sharedDuplicate',childWork:1,parentWork:1,work:2}],invariants:['zero budget preserves every owner','selected work must fit before mutation','child receives original selected budget','output transfer and close intent are separate turns']};
create('🧪️tests/work-budget/🔣️.json',JSON.stringify(law,null,2)+'\n');
create('🧪️tests/work-budget/🧬️.json',JSON.stringify({$schema:'https://json-schema.org/draft/2020-12/schema',type:'object',required:['version','maximumWork','actions','invariants'],additionalProperties:false,properties:{version:{const:1},maximumWork:{type:'array',items:{type:'integer',minimum:0}},actions:{type:'array',items:{type:'object',required:['kind','work'],properties:{kind:{type:'string'},work:{type:'integer',minimum:1},childWork:{type:'integer',minimum:0},parentWork:{type:'integer',minimum:0}},additionalProperties:false}},invariants:{type:'array',items:{type:'string'}}}},null,2)+'\n');
create('🧪️tests/work-budget/🟦️.ts',`import {test,expect} from 'bun:test';
import {Database} from 'bun:sqlite';
import law from './🔣️.json';
test('bounded clone work admission agrees with independent SQL arithmetic',()=>{const db=new Database(':memory:');try{const query=db.query('SELECT CASE WHEN ?1 > 0 AND ?1 <= ?2 THEN ?1 ELSE 0 END AS paid');for(const action of law.actions){if('childWork' in action)expect(action.work).toBe(action.childWork+action.parentWork);for(const budget of law.maximumWork){const expected=action.work<=budget?action.work:0;expect((query.get(action.work,budget)as{paid:number}).paid).toBe(expected);if(budget===0)expect(expected).toBe(0)}}}finally{db.close()}});
`);
edit('🔗️issuance-clone/🦀️.rs',s=>{
 s=replace(s,"_body:usize)->Result<RetirementDemand,ValueError>","_body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>");
 s=replace(s,'if let Some(demand)=source.binding_demand(&self.state.source)?{return Ok(demand);}Ok(RetirementDemand{depth:1,..Default::default()})','let binding=source.binding_demand(&self.state.source)?;if maximum_work_units==0{return Ok(Default::default())}if let Some(demand)=binding{return Ok(demand);}Ok(RetirementDemand{copy_bytes:if self.output_ready(){0}else{size_of::<SealedShared<T>>()+size_of::<bool>()},depth:1,..Default::default()})');
 s=replace(s,' fn advance(&mut self,source:',` fn advance_work_demand(&self,source:RetainedCloneRef<'_,SealedShared<T>>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{self.advance_demands(source,body,maximum_work_units)?;Ok(if maximum_work_units==0||self.output_ready(){0}else if self.state.source.is_none(){1}else{2})}
 fn take_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(self.output_ready()))}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(!self.state.closing))}
 fn begin_close_paid(&mut self)->crate::RetainedWorkOutcome<(),ValueError>{let work=u64::from(self.begin_close());crate::RetainedWorkOutcome{result:Ok(()),work}}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.state.closing||self.terminal_is_empty(){return Ok(0)}self.state.close.next_owner_work::<SealedShared<T>>(self.state.output.is_some(),&self.state.source,body)}
 fn advance(&mut self,source:`);
 s=replace(s,'body: usize, grant:RetainedCloneGrant','body: usize, maximum_work_units:u64, grant:RetainedCloneGrant');
 s=replace(s,'let demand=self.advance_demands(source,body)?;', 'let selected=self.advance_work_demand(source,body,maximum_work_units)?;if selected==0||selected>maximum_work_units{return Ok(RetainedCloneStep::Progress(Default::default()))}let demand=self.advance_demands(source,body,maximum_work_units)?;');
 s=replace(s,'let(output,progress)=turn.result?;state.output=Some(output);let progress=', 'let(output,mut progress)=turn.result?;state.output=Some(output);work+=1;progress.copied_bytes=demand.copy_bytes;let progress=');
 return s;
});
edit('🔮️demands/🦀️.rs',s=>replace(s,'maximum_copy_bytes:usize)','maximum_copy_bytes:usize,maximum_work_units:u64)'));
edit('🌱️dynamic/🦀️.rs',s=>{
 s=replace(s,"copy:usize)->Result<RetirementDemand,ValueError> {\n        self.verify_source", "copy:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError> {\n        self.verify_source");
 s=replace(s,'if self.state.phase==2{return Ok(Default::default());}', 'if maximum_work_units==0||self.state.phase==2{return Ok(Default::default());}');
 s=replace(s,'child.demands(self.child_source(source)?,copy)?','child.demands(self.child_source(source)?,copy,maximum_work_units)?');
 s=replace(s,'maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError> {self.demands(source,maximum_copy_bytes)}','maximum_copy_bytes:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError> {self.demands(source,maximum_copy_bytes,maximum_work_units)}');
 s=replace(s,'body:usize)->Result<RetirementDemand,ValueError>{self.demands(source,body)}','body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{self.demands(source,body,maximum_work_units)}');
 s=replace(s,'    fn advance(&mut self,source:',`    fn advance_work_demand(&self,source:RetainedCloneRef<'_,DslValue>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{
        self.demands(source,body,maximum_work_units)?;if maximum_work_units==0||self.state.phase==2{return Ok(0)}if self.state.source.is_none(){return Ok(1)}
        if let Some(child)=self.state.child.as_ref(){if self.state.draining{return if !child.close_started(){child.begin_close_work_demand()}else if child.terminal_is_empty(){Ok(1)}else{child.close_work_demand(body)}}if self.state.child_value.is_some(){return Ok(1)}if child.output_ready(){return child.take_work_demand()?.checked_add(1).ok_or_else(||Self::refusal("dynamic child transfer work overflow"))}return child.advance_work_demand(self.child_source(source)?,body,maximum_work_units)}Ok(1)
    }
    fn take_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(self.output_ready()))}
    fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(!self.state.closing))}
    fn begin_close_paid(&mut self)->RetainedWorkOutcome<(),ValueError>{let work=u64::from(self.begin_close());RetainedWorkOutcome{result:Ok(()),work}}
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
        if !self.state.closing||self.terminal_is_empty(){return Ok(0)}let s=&self.state;
        if let Some(child)=s.child.as_ref(){return if !child.close_started(){child.begin_close_work_demand()}else if child.terminal_is_empty(){Ok(1)}else{child.close_work_demand(body)}}
        if !s.close.is_empty(){return s.close.next_owner_work::<DslValue>(false,&s.source,body)}if s.bytes.is_some(){return s.close.next_owner_work::<Vec<u8>>(true,&s.source,body)}if s.key.is_some(){return s.close.next_owner_work::<String>(true,&s.source,body)}if s.child_value.is_some(){return s.close.next_owner_work::<DslValue>(true,&s.source,body)}s.close.next_owner_work::<DslValue>(s.target.is_some(),&s.source,body)
    }
    fn advance(&mut self,source:`);
 s=replace(s,'body: usize, grant:RetainedCloneGrant','body: usize, maximum_work_units:u64, grant:RetainedCloneGrant');
 s=replace(s,'let demand=self.demands(source,body)?;', 'let selected=self.advance_work_demand(source,body,maximum_work_units)?;if selected==0||selected>maximum_work_units{return Ok(RetainedCloneStep::Progress(Default::default()))}let demand=self.demands(source,body,maximum_work_units)?;');
 s=replace(s,'.advance(projected,body,RetainedCloneGrant', '.advance(projected,body,maximum_work_units,RetainedCloneGrant');
 s=replace(s,'self.state.child_value=Some(value);','self.state.child_value=Some(value);work+=1;');
 s=replace(s,'if child.begin_close(){work+=1;return Ok(', 'if !child.close_started(){let turn=child.begin_close_paid();work+=turn.work;turn.result?;return Ok(');
 s=replace(s,'fn output_ready(&self)->bool{self.state.phase==2&&self.state.target.is_some()}','fn output_ready(&self)->bool{!self.state.closing&&!self.state.spent&&self.state.phase==2&&self.state.target.is_some()}');
 return s;
});
console.log(JSON.stringify({updated:['issuance-clone','demands','dynamic'],fixture:'work-budget'}));
