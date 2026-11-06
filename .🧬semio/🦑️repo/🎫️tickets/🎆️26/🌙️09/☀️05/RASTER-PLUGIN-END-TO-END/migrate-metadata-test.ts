import {readFileSync,writeFileSync} from "node:fs";
const path="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/🏁️publication/📚️metadata/🧪️tests/🦀️.rs";
const before="let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(grant).unwrap());assert!(step.progress().fits(grant));";
const after="let(turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(grant));let _work=turn.work;let step=turn.result.unwrap();assert!(step.progress().fits(grant));";
const text=readFileSync(path,"utf8");if(!text.includes(before))throw new Error("missing metadata close test");writeFileSync(path,text.replace(before,after));
