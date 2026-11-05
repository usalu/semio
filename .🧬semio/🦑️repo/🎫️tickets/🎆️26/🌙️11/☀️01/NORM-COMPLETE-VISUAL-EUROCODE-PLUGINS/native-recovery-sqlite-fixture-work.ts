import fs from 'node:fs';
const p='🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🫴️receiving/🧪️tests/🦀️.rs';let s=fs.readFileSync(p,'utf8');
function replace(a:string,b:string){if(!s.includes(a))throw Error('Missing fixture source '+a.slice(0,120));s=s.replace(a,b);}
replace('struct OriginalSchemaWallet{grant:RetainedCloneGrant,progress:RetainedCloneProgress}','struct OriginalSchemaWallet{grant:RetainedCloneGrant,progress:RetainedCloneProgress,work:u64}');
replace('impl ReceivingWallet for OriginalSchemaWallet{','impl ReceivingWallet for OriginalSchemaWallet{\n fn actual_work(&self)->u64{self.work}\n fn record_work(&mut self,units:u64)->Result<()>{self.work=self.work.checked_add(units).ok_or_else(||error(ValueRefusalKind::WorkLimit,"original schema work counter overflow"))?;Ok(())}');
s=s.replaceAll('OriginalSchemaWallet{grant,progress:Default::default()}','OriginalSchemaWallet{grant,progress:Default::default(),work:0}');
s=s.replaceAll('OriginalSchemaWallet{grant:RetainedCloneGrant{maximum_items:0,..grant},progress:Default::default()}','OriginalSchemaWallet{grant:RetainedCloneGrant{maximum_items:0,..grant},progress:Default::default(),work:0}');
s=s.replaceAll('control.performed_work()','body.actual_work()');
fs.writeFileSync(p,s);
