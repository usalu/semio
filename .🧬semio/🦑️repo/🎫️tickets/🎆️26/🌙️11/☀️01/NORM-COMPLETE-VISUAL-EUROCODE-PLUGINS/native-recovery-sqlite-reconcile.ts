import fs from 'node:fs';
const b='🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/';
{
 const p=b+'🏛️model/🦀️.rs';let s=fs.readFileSync(p,'utf8');
 s=s.replace('pub(crate) struct SqliteSnapshotLedger { pub(crate) performed_work:u64,','pub(crate) struct SqliteSnapshotLedger {');
 s=s.replace('    /// 🧮️ Observes defining producer operations independently of physical receipts.\n    pub fn performed_work(&self)->u64{self.ledger.performed_work}\n    /// 🧾️ Records actual selected operations in the original shared ledger, including failed prefixes.\n    pub fn record_performed_work(&mut self,units:u64)->std::result::Result<(),ValueError>{self.ledger.performed_work=self.ledger.performed_work.checked_add(units).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"SQLite producer work counter overflow"))?;Ok(())}\n','');
 fs.writeFileSync(p,s);
}
{
 const p=b+'🧩️artifact/🫴️receiving/🦀️.rs';let s=fs.readFileSync(p,'utf8');
 s=s.replaceAll('self.control.record_performed_work(1)?;','');
 s=s.replace(' pub fn take_refusal(&self,owned:usize)->Option<ValueError>{if let Some(cause)=self.cause.borrow_mut().take(){return Some(cause)}self.settle(owned).err()}\n','');
 const old='if let Some(cause)=self.cause.borrow_mut().take(){error=error.with_context_refusal(cause)}';
 if(!s.includes(old))throw Error('SQL failure fold changed; reread current owner');
 s=s.replace(old,'if let Some(cause)=self.cause.borrow_mut().take(){error=error.with_settlement_refusal(cause).expect("value-only SQLite body settles its original channel once")}');
 fs.writeFileSync(p,s);
}
