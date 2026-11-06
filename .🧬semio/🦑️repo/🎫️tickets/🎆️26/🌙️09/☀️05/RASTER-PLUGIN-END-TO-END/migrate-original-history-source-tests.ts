import { readFileSync, writeFileSync } from "node:fs";
const path="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🔗️history/🧪️tests/🦀️.rs";
let text=readFileSync(path,"utf8");
for(const [a,b] of [
 ["let step=source.advance_root(&mut ledger,denied).unwrap();", "let turn=source.advance_root(&mut ledger,denied);assert_eq!(turn.work,0);let step=turn.result.unwrap();"],
 ["let step=source.advance_root(&mut ledger,g).unwrap();", "let turn=source.advance_root(&mut ledger,g);let _work=turn.work;let step=turn.result.unwrap();"],
 ["let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||source.advance(&mut ledger,&mut output,index+1,|rows|&rows[index],denied).unwrap());", "let(turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||source.advance(&mut ledger,&mut output,index+1,|rows|&rows[index],denied));assert_eq!(turn.work,0);let step=turn.result.unwrap();"],
 ["let step=source.advance(&mut ledger,&mut output,index+1,|rows|&rows[index],g).unwrap();", "let turn=source.advance(&mut ledger,&mut output,index+1,|rows|&rows[index],g);let _work=turn.work;let step=turn.result.unwrap();"],
 ["let step=source.advance(&mut ledger,&mut output,1,|ops|&ops[0],g).unwrap();", "let turn=source.advance(&mut ledger,&mut output,1,|ops|&ops[0],g);let _work=turn.work;let step=turn.result.unwrap();"],
 ["let step=source.close_step(&mut ledger,g).unwrap();", "let turn=source.close_step(&mut ledger,g);let _work=turn.work;let step=turn.result.unwrap();"],
 ["let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||source.close_step(&mut ledger,g));match result{", "let(turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||source.close_step(&mut ledger,g));let _work=turn.work;match turn.result{"]
]) {if(!text.includes(a))throw new Error(`missing expected source test: ${a}`);text=text.replaceAll(a,b)}
writeFileSync(path,text);
