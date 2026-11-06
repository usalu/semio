import{readFileSync,writeFileSync,renameSync}from"node:fs";import{dirname,join}from"node:path";const t=dirname(import.meta.path),f="🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs",s=readFileSync(f,"utf8");let n=s;
function method(body:string,name:string,text:string){const a=body.indexOf("fn "+name+"(");if(a<0){const i=body.indexOf("{")+1;return body.slice(0,i)+"\n"+text+"\n"+body.slice(i)}let start=body.lastIndexOf("\n",a)+1,i=body.indexOf("{",a),depth=1,j=i+1;for(;depth;j++){if(body[j]==="{")depth++;else if(body[j]==="}")depth--}return body.slice(0,start)+text+body.slice(j)}
function section(name:string,fn:(s:string)=>string){const re=new RegExp("impl[^\\n]*RetirementCursor for "+name+"[^\\n]*\\{"),m=re.exec(n);if(!m)throw Error("impl "+name);const a=m.index;let i=a+m[0].length,d=1;for(;d;i++){if(n[i]==="{")d++;else if(n[i]==="}")d--}n=n.slice(0,a)+fn(n.slice(a,i))+n.slice(i)}
for(const[name,type,source,metadata]of[
 ["DequeCollection<T>","T","self.0.front()","size_of::<std::collections::VecDeque<T>>()"],
 ["UnorderedSet<T>","T","self.head.as_ref()","size_of::<std::collections::hash_set::IntoIter<T>>()"],
 ["UnorderedMap<K,V>","(K,V)","self.head.as_ref()","size_of::<std::collections::hash_map::IntoIter<K,V>>()"],
 ["OrderedSet<T>","T","self.0.first()","size_of::<std::collections::BTreeSet<T>>()"],
 ["OrderedMap<K, V>","(K,V)","self.0.first_key_value()","size_of::<std::collections::BTreeMap<K,V>>()"],
]as const){section(name,body=>{
const query=name==="OrderedMap<K, V>"?'if let Some((key,value))=self.0.first_key_value(){if !key.retirement_ready()||!value.retirement_ready(){return Err(crate::ValueError::invariant("ordered original entry requires recovery"))}let mut d=retirement_cursor_birth_demand::<(K,V)>()?;d.copy_bytes=d.copy_bytes.checked_add('+metadata+').ok_or_else(||crate::ValueError::invariant("ordered map metadata extent overflow"))?;Ok(d)}else{Ok(Default::default())}':source+".map_or(Ok(Default::default()),|value|retirement_child_demand::<"+type+">(value,"+metadata+"))";
body=method(body,"next_demand"," fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{"+query+"}");
body=method(body,"next_computational_work"," fn next_computational_work(&self,body:usize)->Result<u64,crate::ValueError>{self.next_demand(body)?;Ok(if self.terminal_is_empty(){0}else{retirement_cursor_birth_work()+1})}");
body=method(body,"next_birth_bytes"," fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|d|d.capacity_bytes)}");
const close="fn close_step(&mut self,grant:RetainedCloneGrant,actual:&mut Option<usize>)->RetirementStep";let a=body.indexOf(close);if(a>=0){let p=body.indexOf("{",a)+1;body=body.slice(0,p)+'let d=match self.next_demand(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetirementStep::Failure(error.into(),0)};if !crate::retained_clone::retained_clone_grant_admits(grant,d){return RetirementStep::BudgetExhausted}'+body.slice(p)}
return body;
})}
section("Collection<T>",body=>{
body=method(body,"next_demand",' fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if self.terminal_is_empty(){return Ok(Default::default())}if self.0.is_empty(){return Ok(crate::RetirementDemand{release_bytes:self.0.capacity()*size_of::<T>(),depth:1,..Default::default()})}if self.next_birth_bytes(body).unwrap_or(0)!=0{return retirement_child_demand(self.0.last().expect("quoted original vector child"),size_of::<Vec<T>>())}Ok(crate::RetirementDemand{copy_bytes:self.next_selected_work_byte_demand(body)?,depth:1,..Default::default()})}');
body=method(body,"next_computational_work",' fn next_computational_work(&self,body:usize)->Result<u64,crate::ValueError>{let d=self.next_demand(body)?;Ok(if self.terminal_is_empty(){0}else if d.capacity_bytes!=0{retirement_cursor_birth_work()+1}else{1})}');
return body;
});
section("Optional<T>",body=>{
body=method(body,"next_demand",' fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{if let Some(value)=self.original.as_ref().and_then(Option::as_ref){retirement_child_demand(value,size_of::<Option<T>>())}else{Ok(crate::RetirementDemand{copy_bytes:if self.original.is_some(){size_of::<Option<Option<T>>>()}else{0},depth:usize::from(self.original.is_some()),..Default::default()})}}');
body=method(body,"next_computational_work",' fn next_computational_work(&self,body:usize)->Result<u64,crate::ValueError>{let d=self.next_demand(body)?;Ok(if self.original.is_none(){0}else if d.capacity_bytes!=0{retirement_cursor_birth_work()+1}else{1})}');
body=body.replace("let progress=crate::retained_clone::RetainedCloneProgress{copied_items:1,retained_capacity_bytes:birth,..Default::default()};","let demand=match self.next_demand(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetirementStep::Failure(error.into(),0)};if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return RetirementStep::BudgetExhausted}let progress=crate::retained_clone::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};");
return body;
});
section("Deferred<T>",body=>{
body=method(body,"next_demand",' fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{self.original.as_ref().map_or(Ok(Default::default()),|value|retirement_child_demand(value,size_of::<Option<T>>()))}');
body=method(body,"next_computational_work",' fn next_computational_work(&self,body:usize)->Result<u64,crate::ValueError>{self.next_demand(body)?;Ok(if self.original.is_some(){retirement_cursor_birth_work()+1}else{0})}');
return body;
});
const q=join(t,"🗑️generated/retirement-selected-quotes.rs");writeFileSync(q,n);if(readFileSync(f,"utf8")!==s)throw Error("concurrent retirement");renameSync(q,f);console.log("Selected child birth copy/work metadata quotes migrated for eight primitive families");
