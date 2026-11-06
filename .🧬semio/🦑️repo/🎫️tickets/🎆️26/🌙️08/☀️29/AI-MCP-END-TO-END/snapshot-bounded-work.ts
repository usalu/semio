import{readFileSync,writeFileSync,renameSync,mkdirSync,existsSync}from'node:fs';import{dirname,join}from'node:path';
const root='🧰️framework/🔨️modules/🌱️value/📽️projection/',generated=join(dirname(import.meta.path),'🗑️generated');
function rep(s:string,a:string,b:string){if(s.split(a).length!==2)throw Error('anchor '+a.slice(0,100));return s.replace(a,b)}
function edit(path:string,f:(s:string)=>string){const old=readFileSync(root+path,'utf8'),next=f(old),tmp=join(generated,'snapshot-work-'+Date.now()+'.rs');writeFileSync(tmp,next);if(readFileSync(root+path,'utf8')!==old)throw Error('concurrent '+path);renameSync(tmp,root+path)}
const law={version:1,limits:[0,1,2,4,32],cancellationTurns:[0,1,2,3,4,8,16],record:{text:'actual 源 🌱',values:['alpha','βeta','日','']},invariants:['zero work preserves original and every partial owner','quote never exceeds actual selected maximum','child receives remaining original work','cancellation closes real staged record fields']};
const dir=root+'🎟️work/🧪️tests/bounded/';mkdirSync(dir,{recursive:true});if(existsSync(dir+'🔣️.json'))throw Error('law exists');writeFileSync(dir+'🔣️.json',JSON.stringify(law,null,2)+'\n');writeFileSync(dir+'🧬️.json',JSON.stringify({$schema:'https://json-schema.org/draft/2020-12/schema',type:'object',required:['version','limits','cancellationTurns','record','invariants'],additionalProperties:false,properties:{version:{const:1},limits:{type:'array',items:{type:'integer',minimum:0}},cancellationTurns:{type:'array',items:{type:'integer',minimum:0}},record:{type:'object',required:['text','values'],additionalProperties:false,properties:{text:{type:'string'},values:{type:'array',items:{type:'string'}}}},invariants:{type:'array',items:{type:'string'}}}},null,2)+'\n');
edit('🦀️.rs',s=>{
 s=s.replaceAll("original:RetainedCloneRef<'_,S>,body:usize)","original:RetainedCloneRef<'_,S>,body:usize,maximum_work_units:u64)").replaceAll("original:RetainedCloneRef<'_,S>,body:usize,grant:","original:RetainedCloneRef<'_,S>,body:usize,maximum_work_units:u64,grant:");
 return s;
});
for(const p of ['📦️owned/🦀️.rs','📋️field/🦀️.rs'])edit(p,s=>{
 s=s.replaceAll("original:RetainedCloneRef<'_,T>,body:usize)","original:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)").replaceAll("original:RetainedCloneRef<'_,T>,body:usize,grant:","original:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64,grant:");
 for(const method of ['advance_demands','advance_work_demand','demands'])s=s.replaceAll(method+'(original,body)',method+'(original,body,maximum_work_units)');
 s=s.replaceAll('advance(original,body,grant)','advance(original,body,maximum_work_units,grant)').replaceAll('advance(original,body,child)','advance(original,body,maximum_work_units,child)');
 const a=s.indexOf('fn advance_work_demand('),b=s.indexOf('{',a);let depth=1,e=b+1;for(;e<s.length&&depth;e++){if(s[e]==='{')depth++;else if(s[e]==='}')depth--}if(a<0||depth)throw Error('quote '+p);const body=s.slice(b+1,e-1);s=s.slice(0,b+1)+'if maximum_work_units==0{return Ok(0)}let selected=(||->Result<u64,ValueError>{'+body+'})();selected.map(|work|if work<=maximum_work_units{work}else{0})'+s.slice(e-1);
 const needle='let d=self.demands(original,body,maximum_work_units)?;';s=rep(s,needle,'let selected=self.advance_work_demand(original,body,maximum_work_units)?;if selected==0{return Ok(RetainedCloneStep::Progress(Default::default()))}'+needle);
 if(p.startsWith('📋️'))s=rep(s,'if self.closing||self.ready{return Err(invalid());}','if self.closing||self.ready{return Err(invalid());}if maximum_work_units==0{return Ok(Default::default())}');
 return s;
});
edit('👁️post/🦀️.rs',s=>{
 s=rep(s,'source:ManuallyDrop<Option<RetainedCloneSource<S,S>>>}','source:ManuallyDrop<Option<RetainedCloneSource<S,S>>>,actual:Option<usize>}');
 s=rep(s,'source:ManuallyDrop::new(None)}}}','source:ManuallyDrop::new(None),actual:None}}}');
 s=rep(s,'  let mut demand=match(mode,','  if self.actual.is_some(){return Ok(Some(RetirementDemand{copy_bytes:size_of::<Option<usize>>(),depth:1,..Default::default()}))}\n  let mut demand=match(mode,');
 s=rep(s,'if self.demand(post,mode,body)?.is_none(){return Ok(0)}match mode','if self.demand(post,mode,body)?.is_none(){return Ok(0)}if self.actual.is_some(){return Ok(1)}match mode');
 s=rep(s,'  let child=RetainedCloneGrant{','  if self.actual.is_some(){self.actual=None;work=1;return Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<usize>>(),..Default::default()}))}\n  let child=RetainedCloneGrant{');
 s=rep(s,'admission.advance(child)','admission.advance(child,&mut self.actual)');
 s=rep(s,'self.source.is_none()&&self.admission.is_none()','self.source.is_none()&&self.admission.is_none()&&self.actual.is_none()');
 return s;
});
console.log('Snapshot Projection contract forwards actual bounded work; post admission retains actual allocation marker until separately funded clear');
