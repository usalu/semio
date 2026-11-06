import{readFileSync,writeFileSync,renameSync}from'node:fs';import{dirname,join}from'node:path';
const ticket=dirname(import.meta.path),generated=join(ticket,'🗑️generated'),root='🧰️framework/🔨️modules/🌱️value/🛬️decode/';
function rep(s:string,a:string,b:string){if(s.split(a).length!==2)throw Error('anchor '+a.slice(0,100));return s.replace(a,b)}
function edit(file:string,f:(s:string)=>string){const old=readFileSync(file,'utf8'),next=f(old);if(next===old)return;const tmp=join(generated,'work-view-'+Date.now()+'.rs');writeFileSync(tmp,next);if(readFileSync(file,'utf8')!==old)throw Error('concurrent '+file);renameSync(tmp,file)}
edit(root+'🎟️work/🦀️.rs',s=>{
 s=rep(s,'pub trait NativeDecodeWorkRecipient:Send{fn admits','pub trait NativeDecodeWorkRecipient:Send{fn available_work(&mut self,observed_since_scope:u64)->Result<u64,ValueError>;fn admits');
 const anchor=' pub fn actual_work(&self)->u64{self.work.actual}';
 return rep(s,anchor,` /// 🔭️ Borrows the actual scoped CPU ceiling without issuing, reserving or spending another action.
 pub fn available_work_units(&mut self)->Result<u64,ValueError>{if self.work.reserved_issuer.is_some(){if !self.work.same_issuer()||self.work.quote_pending{return Err(invalid())}return Ok(self.work.reserved)}let observed=self.work.actual.checked_sub(self.work.baseline).ok_or_else(invalid)?;let loan=self.work.recipient.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native work selection requires its actual caller loan"))?;loan.recipient.available_work(observed)}
`+anchor);
});
edit(root+'♻️retirement/🦀️.rs',s=>{
 s=rep(s,'pub trait NativePreparedDecodeControl{','pub trait NativePreparedDecodeControl{\n fn available_work_units(&mut self)->Result<u64,ValueError>;');
 return rep(s,"impl NativePreparedDecodeControl for NativeDecodeControl<'_>{","impl NativePreparedDecodeControl for NativeDecodeControl<'_>{\n fn available_work_units(&mut self)->Result<u64,ValueError>{NativeDecodeControl::available_work_units(self)}");
});
const rows=JSON.parse(readFileSync(join(generated,'native-work-view-census.json'),'utf8'))as{file:string,body:string}[];
function closeParen(s:string,a:number){let n=1;for(let i=a+1;i<s.length;i++){if(s[i]==='(')n++;else if(s[i]===')'&&--n===0)return i;}return-1}
const unresolved:{file:string,body:string}[]=[],changed:string[]=[];
for(const row of rows){let current=readFileSync(row.file,'utf8');if(!current.includes(row.body)){if(current.includes('fn available_work('))continue;unresolved.push(row);continue}if(row.body.includes('fn available_work('))continue;
 const body=row.body,open=body.indexOf('is_some_and('),a=open+'is_some_and'.length,b=open<0?-1:closeParen(body,a),lambda=b<0?'':body.slice(a+1,b),limit=lambda.includes('<=')?lambda.slice(lambda.indexOf('<=')+2).trim():'';
 if(!limit||limit.includes('cx.')||limit.includes(';')||body.includes('self.turn.with')||body.includes('try_lock')){unresolved.push(row);continue}
 const err=body.match(/fn admits[\s\S]*?->\s*(?:[A-Za-z_:]+::)?Result\s*<\s*bool\s*,\s*([^>]+)>/)?.[1].trim();if(!err){unresolved.push(row);continue}
 const method=`fn available_work(&mut self,observed:u64)->Result<u64,${err}>{let limit:u64=${limit};limit.checked_sub(observed).ok_or_else(||${err}::invariant("native observed work exceeds its actual finite scope"))}`;
 const prefix=body.slice(0,body.indexOf('{')+1),next=body.replace(prefix,prefix+method);
 edit(row.file,s=>rep(s,body,next));changed.push(row.file);
}
writeFileSync(join(generated,'native-work-view-unresolved.json'),JSON.stringify(unresolved));
console.log(JSON.stringify({changed:changed.length,unresolved:unresolved.map(r=>({file:r.file,body:r.body.slice(0,900)}))}));
