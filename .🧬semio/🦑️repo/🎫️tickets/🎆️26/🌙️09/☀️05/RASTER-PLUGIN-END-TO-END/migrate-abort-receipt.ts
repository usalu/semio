import {readFileSync,writeFileSync} from "node:fs";
const path="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🏁️replay/abort.rs";
const before="let p=cx.with_native_receiving(|control|self.metadata.as_mut().unwrap().advance(store,self.result.as_ref().unwrap(),control,grant))?;if let Some(issued)=cx.receiving_issued_grant(){cx.consume_receiving_retained(issued,p)?;}else{cx.consume_retained(p)?;}cx.consume_fuel(p.copied_items as u64);";
const after="let mut work=0u64;let result=cx.with_native_receiving(|control|{let turn=self.metadata.as_mut().unwrap().advance(store,self.result.as_ref().unwrap(),control,grant);work=turn.work;turn.result.map(RetainedCloneStep::Progress)});semio_framework_job::receive_original_receiving_work(cx,semio_framework_value::RetainedWorkOutcome{result,work})?;";
const text=readFileSync(path,"utf8");if(!text.includes(before))throw new Error("missing abort metadata receiving call");writeFileSync(path,text.replace(before,after));
