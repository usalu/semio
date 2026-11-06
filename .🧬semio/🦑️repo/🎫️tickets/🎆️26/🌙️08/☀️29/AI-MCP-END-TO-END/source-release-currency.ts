import{readFileSync,writeFileSync,renameSync}from"node:fs";import{dirname}from"node:path";const t=dirname(import.meta.path);
function edit(p:string,change:(s:string)=>string){let s=readFileSync(p,"utf8"),n=change(s);if(n===s)throw Error("unchanged "+p);let tmp=t+"/🗑️generated/release-currency-"+Math.random().toString(16).slice(2)+".rs";writeFileSync(tmp,n);if(readFileSync(p,"utf8")!==s)throw Error("CAS "+p);renameSync(tmp,p)}
function replace(s:string,a:string,b:string){if(!s.includes(a))throw Error("missing "+a);return s.replace(a,b)}
const base="🧰️framework/🔨️modules/🧵️job/📬️outcome/";
edit(base+"📦️builder/🪙️return-escrow/🦀️.rs",s=>{
 s=replace(s,"pub held_copy_bytes:usize,pub held_fuel:u64","pub held_copy_bytes:usize,pub held_release_bytes:usize,pub held_fuel:u64");
 s=replace(s,"&&self.work.checked_add(u128::from(self.held_fuel))","&&self.spent.released_bytes.checked_add(self.held_release_bytes).is_some_and(|bytes|bytes<=self.granted.maximum_release_bytes)&&self.work.checked_add(u128::from(self.held_fuel))");
 s=replace(s,"Ok((retained,fuel.checked_sub(self.held_fuel)","retained.maximum_release_bytes=retained.maximum_release_bytes.checked_sub(self.held_release_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,\"original return release holds exceed retained lifetime stock\"))?;Ok((retained,fuel.checked_sub(self.held_fuel)");
 s=replace(s,"ORIGINAL_RETURN_ESCROW_ROW_BYTES:usize=136","ORIGINAL_RETURN_ESCROW_ROW_BYTES:usize=160");
 s=replace(s,"pub struct OriginalReturnEscrowAmounts{pub items:u64,pub copy_bytes:u64,pub work:u64}","pub struct OriginalReturnEscrowAmounts{pub items:u64,pub copy_bytes:u64,pub work:u64,pub release_bytes:u64}");
 s=replace(s,"Self{items:0,copy_bytes:0,work:0}","Self{items:0,copy_bytes:0,work:0,release_bytes:0}");
 s=replace(s,"work:self.work.checked_add(other.work)?}","work:self.work.checked_add(other.work)?,release_bytes:self.release_bytes.checked_add(other.release_bytes)?}");
 s=replace(s,"&&self.work<=limit.work}","&&self.work<=limit.work&&self.release_bytes<=limit.release_bytes}");
 s=s.replaceAll("Some((14,","Some((17,").replaceAll("13*step","16*step").replaceAll("13-phase","16-phase").replaceAll("if phase==13","if phase==16").replaceAll("Some((slot,13))","Some((slot,16))");
 s=replace(s,"phase>6&&row.held.work!=held.work","phase>6&&row.held.work!=held.work||phase>13&&row.held.release_bytes!=held.release_bytes");
 s=replace(s,"12=>row.released.work=0,_","12=>row.released.work=0,13=>row.held.release_bytes=held.release_bytes,14=>row.redeemed.release_bytes=0,15=>row.released.release_bytes=0,_");
 s=s.replaceAll(/OriginalReturnEscrowAmounts\{([^{}]*)\}/g,(all,body)=>/release_bytes|pub /.test(body)?all:"OriginalReturnEscrowAmounts{"+body+",release_bytes:0}");
 s=s.replaceAll("held_copy_bytes:0,held_fuel:","held_copy_bytes:0,held_release_bytes:0,held_fuel:");
 return s;
});
edit(base+"🪙️budget/🦀️.rs",s=>{
 s=replace(s,"pub held_copy_bytes:usize,pub held_fuel:","pub held_copy_bytes:usize,pub held_release_bytes:usize,pub held_fuel:");
 s=s.replaceAll("held_copy_bytes:0,held_fuel:","held_copy_bytes:0,held_release_bytes:0,held_fuel:");
 s=replace(s,"held_copy_bytes:self.held_copy_bytes,held_fuel:","held_copy_bytes:self.held_copy_bytes,held_release_bytes:self.held_release_bytes,held_fuel:");
 s=s.replaceAll(/(\b(?:budget|b|self))\.held_copy_bytes!=0/g,"$1.held_copy_bytes!=0||$1.held_release_bytes!=0").replaceAll(/(\b(?:budget|b))\.held_copy_bytes==0/g,"$1.held_copy_bytes==0&&$1.held_release_bytes==0");
 s=s.replaceAll(/OriginalReturnEscrowAmounts\{([^{}]*)\}/g,(all,body)=>/release_bytes|pub /.test(body)?all:"OriginalReturnEscrowAmounts{"+body+",release_bytes:0}");
 return s;
});
edit(base+"📤️publication/🪙️escrow/🦀️.rs",s=>{
 s=replace(s,"const ROW:usize=136","const ROW:usize=160");
 s=s.replaceAll("88*rows","91*rows").replaceAll("88*32","91*32");
 for(const currency of ["held","redeemed","released"])s=replace(s,"word!(row."+currency+".work);","word!(row."+currency+".work);word!(row."+currency+".release_bytes);");
 return s;
});
console.log("release currency mounted in actual snapshot/budget/table/publication");
