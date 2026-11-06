import{readFileSync,writeFileSync,renameSync}from"node:fs";import{dirname}from"node:path";const t=dirname(import.meta.path),b="🧰️framework/🔨️modules/🧵️job/📬️outcome/";function edit(p:string,f:(s:string)=>string){const s=readFileSync(p,"utf8"),n=f(s),tmp=t+"/🗑️generated/capacity-currency-"+Math.random().toString(16).slice(2);if(n===s)throw Error("unchanged "+p);writeFileSync(tmp,n);if(readFileSync(p,"utf8")!==s)throw Error("CAS");renameSync(tmp,p)}function rep(s:string,a:string,c:string){if(!s.includes(a))throw Error("missing "+a);return s.replace(a,c)}
edit(b+"📦️builder/🪙️return-escrow/🦀️.rs",s=>{
 s=rep(s,"pub held_release_bytes:usize,pub held_fuel:","pub held_release_bytes:usize,pub held_capacity_bytes:usize,pub held_fuel:");
 s=rep(s,"&&self.work.checked_add(u128::from(self.held_fuel))","&&self.spent.retained_capacity_bytes.checked_add(self.held_capacity_bytes).is_some_and(|bytes|bytes<=self.granted.maximum_capacity_bytes)&&self.work.checked_add(u128::from(self.held_fuel))");
 s=rep(s,"Ok((retained,fuel.checked_sub(self.held_fuel)","retained.maximum_capacity_bytes=retained.maximum_capacity_bytes.checked_sub(self.held_capacity_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,\"original return capacity holds exceed retained lifetime stock\"))?;Ok((retained,fuel.checked_sub(self.held_fuel)");
 s=rep(s,"ORIGINAL_RETURN_ESCROW_ROW_BYTES:usize=160","ORIGINAL_RETURN_ESCROW_ROW_BYTES:usize=184");
 s=rep(s,"pub work:u64,pub release_bytes:u64}","pub work:u64,pub release_bytes:u64,pub capacity_bytes:u64}");
 s=rep(s,"Self{items:0,copy_bytes:0,work:0,release_bytes:0}","Self{items:0,copy_bytes:0,work:0,release_bytes:0,capacity_bytes:0}");
 s=rep(s,"release_bytes:self.release_bytes.checked_add(other.release_bytes)?}","release_bytes:self.release_bytes.checked_add(other.release_bytes)?,capacity_bytes:self.capacity_bytes.checked_add(other.capacity_bytes)?}");
 s=rep(s,"&&self.release_bytes<=limit.release_bytes}","&&self.release_bytes<=limit.release_bytes&&self.capacity_bytes<=limit.capacity_bytes}");
 s=s.replaceAll("Some((17,","Some((20,").replaceAll("16*step","19*step").replaceAll("16-phase","19-phase").replaceAll("if phase==16","if phase==19").replaceAll("Some((slot,16))","Some((slot,19))");
 s=rep(s,"phase>13&&row.held.release_bytes!=held.release_bytes","phase>13&&row.held.release_bytes!=held.release_bytes||phase>16&&row.held.capacity_bytes!=held.capacity_bytes");
 s=rep(s,"15=>row.released.release_bytes=0,_","15=>row.released.release_bytes=0,16=>row.held.capacity_bytes=held.capacity_bytes,17=>row.redeemed.capacity_bytes=0,18=>row.released.capacity_bytes=0,_");
 return s;
});
edit(b+"🪙️budget/🦀️.rs",s=>{
 s=rep(s,"pub held_release_bytes:usize,pub held_fuel:","pub held_release_bytes:usize,pub held_capacity_bytes:usize,pub held_fuel:");
 s=s.replaceAll("held_release_bytes:0,held_fuel:","held_release_bytes:0,held_capacity_bytes:0,held_fuel:");
 s=rep(s,"held_release_bytes:self.held_release_bytes,held_fuel:","held_release_bytes:self.held_release_bytes,held_capacity_bytes:self.held_capacity_bytes,held_fuel:");
 return s.replaceAll(/(\b(?:budget|b|self))\.held_release_bytes!=0/g,"$1.held_release_bytes!=0||$1.held_capacity_bytes!=0").replaceAll(/(\b(?:budget|b))\.held_release_bytes==0/g,"$1.held_release_bytes==0&&$1.held_capacity_bytes==0");
});
edit(b+"📤️publication/🪙️escrow/🦀️.rs",s=>{s=rep(s,"const ROW:usize=160","const ROW:usize=184");s=s.replaceAll("91*rows","94*rows").replaceAll("91*32","94*32");for(const c of ["held","redeemed","released"])s=rep(s,"word!(row."+c+".release_bytes);","word!(row."+c+".release_bytes);word!(row."+c+".capacity_bytes);");return s});
const files=JSON.parse(readFileSync(t+"/🗑️generated/release-currency-files.json","utf8"))as string[];
for(const p of files)if(p.endsWith(".rs")){const s=readFileSync(p,"utf8"),n=s.replaceAll(/OriginalReturnEscrowAmounts\{([^{}]*)\}/g,(all,body)=>/capacity_bytes|pub |\.\./.test(body)?all:"OriginalReturnEscrowAmounts{"+body+",capacity_bytes:0}").replaceAll(/held_release_bytes:([^,{}]*),held_fuel:/g,(all,v)=>"held_release_bytes:"+v+",held_capacity_bytes:0,held_fuel:");if(n!==s)edit(p,()=>n)}
edit(b+"📦️builder/🪙️return-escrow/🛡️guard/🦀️.rs",s=>rep(s,"1+16*(size_of::<u64>()","1+19*(size_of::<u64>()"));
for(const f of["case.json","case.schema.json"]){const p=b+"📦️builder/🪙️return-escrow/🛡️guard/"+f;edit(p,s=>{const v=JSON.parse(s);const guard=f==="case.json"?v.guard:v.properties.guard.const,cases=f==="case.json"?v.cases:v.properties.cases.const;guard.copyBytes=51;cases[0].observed[1]=563;cases[0].available[1]=3533;return JSON.stringify(v,null,2)+"\n"})}
console.log("full physical currency Source foundation installed");
