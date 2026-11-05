import fs from 'node:fs';
const b='🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/';
function replace(s:string,a:string,c:string){if(!s.includes(a))throw Error('Missing source '+a.slice(0,110));return s.replace(a,c);}
{
 const p=b+'🏛️model/🦀️.rs';let s=fs.readFileSync(p,'utf8');
 s=replace(s,'pub(crate) struct SqliteSnapshotLedger {','pub(crate) struct SqliteSnapshotLedger { pub(crate) performed_work:u64,');
 s=replace(s,"impl<'a> SqliteSnapshotControl<'a> {","impl<'a> SqliteSnapshotControl<'a> {\n    /// 🧮️ Observes defining producer operations independently of physical receipts.\n    pub fn performed_work(&self)->u64{self.ledger.performed_work}\n    /// 🧾️ Records actual selected operations in the original shared ledger, including failed prefixes.\n    pub fn record_performed_work(&mut self,units:u64)->std::result::Result<(),ValueError>{self.ledger.performed_work=self.ledger.performed_work.checked_add(units).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,\"SQLite producer work counter overflow\"))?;Ok(())}\n");
 fs.writeFileSync(p,s);
}
{
 const p=b+'🧩️artifact/🫴️receiving/🦀️.rs';let s=fs.readFileSync(p,'utf8');
 s=replace(s,'self.body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})}','let result=self.body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()});self.control.record_performed_work(1)?;result}');
 s=replace(s,'};self.receipt(before)?;result}','};self.control.record_performed_work(1)?;self.receipt(before)?;result}');
 s=replace(s,'*output=Some(Box::new(construct()));let recorded=','*output=Some(Box::new(construct()));self.control.record_performed_work(1)?;let recorded=');
 s=replace(s,'};self.receipt(before)?;result.map_err(|error|cause.unwrap_or(error))','};self.control.record_performed_work(1)?;self.receipt(before)?;result.map_err(|error|cause.unwrap_or(error))');
 fs.writeFileSync(p,s);
}
