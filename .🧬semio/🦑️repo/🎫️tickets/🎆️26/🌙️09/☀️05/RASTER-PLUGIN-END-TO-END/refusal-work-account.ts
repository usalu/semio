import {readFileSync,writeFileSync} from "node:fs";
const path=process.argv[2]!;let source=readFileSync(path,"utf8");
function edit(before:string,after:string){if(!source.includes(before))throw new Error(`Missing anchored input: ${before}`);source=source.replaceAll(before,after);}
edit('struct TrackedChildRetirement{','const REFUSAL_WORK:u64=94;\nstruct TrackedChildRetirement{');
edit('Ok(u64::from(self.operation.is_some()))','Ok(if self.refuse_once{REFUSAL_WORK}else{u64::from(self.operation.is_some())})');
edit('self.refuse_once=false;work=1;return Err(','self.refuse_once=false;work=REFUSAL_WORK;return Err(');
edit('fn retire_owned_work_demand(&self,_:&TrackedChildOperation)->Result<u64,ValueError>{Ok(1)}','fn retire_owned_work_demand(&self,_:&TrackedChildOperation)->Result<u64,ValueError>{Ok(if self.first.load(std::sync::atomic::Ordering::SeqCst){REFUSAL_WORK}else{1})}');
edit('result:Err((failure,operation)),work:1','result:Err((failure,operation)),work:REFUSAL_WORK');
writeFileSync(path,source);
