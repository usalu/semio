import {readFileSync,writeFileSync} from "node:fs";
const path="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🔗️history/📜️report/🧪️tests/🦀️.rs";
let text=readFileSync(path,"utf8");
for(const [before,after] of [
 ["let p=cx.with_native_receiving(|control|metadata.advance(&mut store,&finished,control,g))?;let issued=cx.receiving_issued_grant().unwrap();cx.consume_receiving_retained(issued,p)?;cx.consume_fuel(p.copied_items as u64);", "let mut work=0u64;let result=cx.with_native_receiving(|control|{let turn=metadata.advance(&mut store,&finished,control,g);work=turn.work;turn.result.map(semio_framework_value::RetainedCloneStep::Progress)});semio_framework_job::receive_original_receiving_work(cx,semio_framework_value::RetainedWorkOutcome{result,work})?;"],
 ["let step=cx.with_native_retirement(|control|metadata.close_step(&mut store,control,g))?;cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64);", "let mut work=0u64;let result=cx.with_native_retirement(|control|{let turn=metadata.close_step(&mut store,control,g);work=turn.work;turn.result});semio_framework_job::receive_original_retained_work(cx,semio_framework_value::RetainedWorkOutcome{result,work})?;"]
]){if(!text.includes(before))throw new Error("missing report test call");text=text.replaceAll(before,after)}writeFileSync(path,text);
