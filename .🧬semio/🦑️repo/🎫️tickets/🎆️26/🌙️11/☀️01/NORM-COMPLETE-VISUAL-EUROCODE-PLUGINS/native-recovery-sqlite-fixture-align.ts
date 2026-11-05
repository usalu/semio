import fs from 'node:fs';
const p='🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🫴️receiving/🧪️tests/🦀️.rs';let s=fs.readFileSync(p,'utf8');
s=s.replace('struct OriginalSchemaWallet{grant:RetainedCloneGrant,progress:RetainedCloneProgress,work:u64}','struct OriginalSchemaWallet{grant:RetainedCloneGrant,progress:RetainedCloneProgress}');
s=s.replace(' fn actual_work(&self)->u64{self.work}\n fn record_work(&mut self,units:u64)->Result<()>{self.work=self.work.checked_add(units).ok_or_else(||error(ValueRefusalKind::WorkLimit,"original schema work counter overflow"))?;Ok(())}\n','');
s=s.replaceAll('progress:Default::default(),work:0}','progress:Default::default()}');
s=s.replaceAll('body.actual_work()','control.performed_work()');
fs.writeFileSync(p,s);
