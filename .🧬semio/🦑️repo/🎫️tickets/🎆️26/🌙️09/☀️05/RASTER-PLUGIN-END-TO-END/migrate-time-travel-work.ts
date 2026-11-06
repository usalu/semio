import {readFileSync,writeFileSync} from "node:fs";
const path="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🗄️store-state/🦀️.rs";
let text=readFileSync(path,"utf8");
for(const [before,after] of [
 ["let result=cx.with_native_receiving(|native|self.step_preview(store,deadline_us,clock,grant,native).map_err(TimeTravelReceivingFailure::Semantic));let progress=", "let result=cx.with_native_receiving(|native|self.step_preview(store,deadline_us,clock,grant,native).map_err(TimeTravelReceivingFailure::Semantic));if let Some(work)=self.preview_job.as_mut().and_then(|job|job.take_preparation_work()){cx.consume_fuel(work)}let progress="],
 ["let result=cx.with_native_receiving(|native|self.step_replay(store,deadline_us,clock,grant,native).map_err(TimeTravelReceivingFailure::Semantic));let progress=", "let result=cx.with_native_receiving(|native|self.step_replay(store,deadline_us,clock,grant,native).map_err(TimeTravelReceivingFailure::Semantic));if let Some(work)=self.replay.as_mut().and_then(|replay|replay.take_preparation_work()){cx.consume_fuel(work)}let progress="]
]){if(!text.includes(before))throw new Error("missing time-travel receiving call");text=text.replace(before,after)}writeFileSync(path,text);
