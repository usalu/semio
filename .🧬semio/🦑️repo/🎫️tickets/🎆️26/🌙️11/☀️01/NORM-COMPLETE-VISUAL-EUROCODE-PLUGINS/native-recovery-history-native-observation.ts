import fs from 'node:fs';
const p='🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs';let s=fs.readFileSync(p,'utf8');
function replace(a:string,b:string){if(!s.includes(a))throw Error('Missing source '+a.slice(0,120));s=s.replace(a,b);}
replace('pub struct RetainedHistoryDecode {\n    last_work: u64,','pub struct RetainedHistoryDecode {\n    last_work: u64,\n    last_native_work:u64,');
replace('            last_work: 0,','            last_work: 0,\n            last_native_work:0,');
replace('pub fn last_turn_work(&self) -> u64 { self.last_work }','pub fn last_turn_work(&self) -> u64 { self.last_work }\n\n    /// 🔭️ Observes the actual native counter separately from the selected producer report.\n    pub fn last_turn_native_work(&self)->u64{self.last_native_work}');
replace('    run: impl FnOnce(&mut NativeDecodeControl<\'_>) -> T,','    observed:&mut u64,\n    run: impl FnOnce(&mut NativeDecodeControl<\'_>) -> T,');
replace('    let result = control.with_work_recipient(work,','    let before=control.actual_work();\n    let result = control.with_work_recipient(work,');
replace('    *receipt = Some(control.detach()','    *observed=control.actual_work().saturating_sub(before);\n    *receipt = Some(control.detach()');
s=s.replaceAll('{self.last_work=0;let result=self.step_result','{self.last_work=0;self.last_native_work=0;let result=self.step_result');
s=s.replaceAll('{self.last_work=0;let result=self.close_step_result','{self.last_work=0;self.last_native_work=0;let result=self.close_step_result');
s=s.replaceAll('grant.maximum_capacity_bytes, false, work, |control|','grant.maximum_capacity_bytes, false, work, &mut self.last_native_work, |control|');
s=s.replaceAll('grant.maximum_capacity_bytes, true, work, |control|','grant.maximum_capacity_bytes, true, work, &mut self.last_native_work, |control|');
fs.writeFileSync(p,s);
