import{readFileSync,writeFileSync,renameSync}from'node:fs';import{dirname,join}from'node:path';
const file='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🦀️.rs',old=readFileSync(file,'utf8');let s=old;
function rep(a:string,b:string){if(s.split(a).length!==2)throw Error('anchor '+a);s=s.replace(a,b)}
rep('fn charge(cx: &mut StepContext<\'_>, retiring: bool) -> Result<bool, OriginalTurnFailure> {','fn charge_metadata(cx:&mut StepContext<\'_>,retiring:bool,bytes:usize)->Result<bool,OriginalTurnFailure>{');
rep('    let bytes = std::mem::size_of::<Phase>() + std::mem::size_of::<Option<OriginalSourceBinding>>() + std::mem::size_of::<Option<u64>>() + std::mem::size_of::<Option<usize>>();\n','');
const anchor='fn charge_capture<T: RetireOwned>';
rep(anchor,'fn control_bytes()->usize{std::mem::size_of::<Phase>()+std::mem::size_of::<Option<OriginalSourceBinding>>()+std::mem::size_of::<Option<u64>>()+std::mem::size_of::<Option<usize>>()}\nfn charge(cx:&mut StepContext<\'_>,retiring:bool)->Result<bool,OriginalTurnFailure>{charge_metadata(cx,retiring,control_bytes())}\n'+anchor);
rep('        if !charge(cx, retirement)? { return Ok(false); }','        let bytes=control_bytes()+if self.origin_binding.is_none(){std::mem::size_of_val(&self.origin_binding)+std::mem::size_of_val(&self.origin_address)}else{0};if !charge_metadata(cx,retirement,bytes)?{return Ok(false)}');
rep('self.origin_address.get_or_insert(self as*const Self as usize);','let address=self as*const Self as usize;self.origin_address.get_or_insert(address);');
const tmp=join(dirname(import.meta.path),'🗑️generated/poll-origin-paid-'+Date.now()+'.rs');writeFileSync(tmp,s);if(readFileSync(file,'utf8')!==old)throw Error('concurrent');renameSync(tmp,file);console.log('First actual Source origin capture is funded exactly; later rounds retain that immutable identity');
