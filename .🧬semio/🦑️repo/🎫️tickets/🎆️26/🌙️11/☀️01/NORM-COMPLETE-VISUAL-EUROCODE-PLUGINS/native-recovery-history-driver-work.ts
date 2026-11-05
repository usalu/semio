import fs from 'node:fs';
const p='🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs';
let s=fs.readFileSync(p,'utf8');
function replace(a:string,b:string){if(!s.includes(a))throw Error('Missing exact source: '+a.slice(0,150));s=s.replace(a,b);}
replace('drop(slot.take());\n        return Ok','drop(slot.take());\n        *work=1;\n        return Ok');
for(const [name,output,extra,call] of [['step','RetainedHistorySourceStep',', history: &mut Option<HistoryLog>',', history'],['close_step','RetainedCloneStep','','']]){
 const sig='pub fn '+name+'<S: RetainedWireByteSource + ?Sized>(&mut self, source: &S, turn: &dyn RetainedHistoryTurn, grant: RetainedCloneGrant'+extra+') -> Result<'+output+', ValueError> {';
 replace(sig,'pub fn '+name+'<S: RetainedWireByteSource + ?Sized>(&mut self, source: &S, turn: &dyn RetainedHistoryTurn, grant: RetainedCloneGrant'+extra+') -> semio_framework_value::RetainedWorkOutcome<'+output+', ValueError> {self.last_work=0;let result=self.'+name+'_result(source,turn,grant'+call+');semio_framework_value::RetainedWorkOutcome{work:self.last_work,result}}\n\n    fn '+name+'_result<S: RetainedWireByteSource + ?Sized>(&mut self, source: &S, turn: &dyn RetainedHistoryTurn, grant: RetainedCloneGrant'+extra+') -> Result<'+output+', ValueError> {');
}
s=s.replaceAll('        self.last_work=1;\n        let last_work=&mut self.last_work;','        let last_work=&mut self.last_work;');
replace('match RetainedHistorySourceDecode::admit(source, range, limits, persisted_document, control, grant) {','let turn=RetainedHistorySourceDecode::admit(source, range, limits, persisted_document, control, grant);\n                *last_work=turn.work;\n                match turn.result {');
replace('decoder.take();\n                    Ok(RetainedHistorySourceStep::Closed)','decoder.take();\n                    *last_work=last_work.saturating_add(1);\n                    Ok(RetainedHistorySourceStep::Closed)');
replace('owner.take_ready_into(history, grant).map(|ready| if ready { RetainedHistorySourceStep::Ready } else { RetainedHistorySourceStep::Blocked })','let turn=owner.take_ready_into(history, grant);\n                    *last_work=turn.work;\n                    turn.result.map(|ready| if ready { RetainedHistorySourceStep::Ready } else { RetainedHistorySourceStep::Blocked })');
replace('*self.fault = Some(error);\n                self.closing = true;','*self.fault = Some(error);\n                self.last_work=self.last_work.saturating_add(1);\n                self.closing = true;');
replace('self.receipt.take();\n            self.closed = true;','self.receipt.take();\n            self.closed = true;\n            self.last_work=self.last_work.saturating_add(1);');
replace('owner.take_fault_into(fault, grant).map(|_| RetainedHistorySourceStep::Progress)','let turn=owner.take_fault_into(fault, grant);\n                    *last_work=turn.work;\n                    turn.result.map(|_| RetainedHistorySourceStep::Progress)');
replace('owner.take_partial_into(partial, grant).map(|_| RetainedHistorySourceStep::Progress)','let turn=owner.take_partial_into(partial, grant);\n                    *last_work=turn.work;\n                    turn.result.map(|_| RetainedHistorySourceStep::Progress)');
replace('Err(error) => *self.fault = Some(error),','Err(error) => { *self.fault = Some(error);self.last_work=self.last_work.saturating_add(1); },');
replace('self.decoder.take();\n                if issued.is_none()','self.decoder.take();\n                self.last_work=self.last_work.saturating_add(1);\n                if issued.is_none()');
replace('self.receipt.take();\n        self.closed = true;','self.receipt.take();\n        self.closed = true;\n        self.last_work=self.last_work.saturating_add(1);');
fs.writeFileSync(p,s);
