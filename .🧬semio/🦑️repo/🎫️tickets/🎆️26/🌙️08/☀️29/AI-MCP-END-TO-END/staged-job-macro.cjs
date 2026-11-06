const fs=require('node:fs');const file='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📣️macros/⚛️wp4-reactor/🦀️.rs';
const before=fs.readFileSync(file,'utf8');const pattern=/        pub unsafe extern "C" fn semio_owned_start_job_v1\([^]*?(?=        #\[cfg\(all\(target_arch = "wasm32", target_env = "p2"\)\)\])/g;
if([...before.matchAll(pattern)].length!==1)throw Error('start macro changed');
const fields='operation,generation,epoch,clock_binding,absolute_deadline_us,total_fuel,items,copy,capacity,release,depth,policy_items,policy_copy,policy_capacity,policy_release,policy_depth,fuel,deadline_us,cancelled'.split(',');
const args=fields.map(field=>`${field}:u64`).join(',');
const replacement=`        pub extern "C" fn semio_owned_start_job_v1(job:u64,${args})->u64{\n            match $crate::owned_abi::original_job_budget([${fields.join(',')}]){Ok(budget)=>$crate::reactor::original_actor_context::exports::start_job_owned(job,budget),Err(kind)=>$crate::operation_authority::owned_result::encode_refusal(kind)}\n        }\n`;
const temp='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/🗑️generated/staged-job-macro.tmp';fs.writeFileSync(temp,before.replace(pattern,replacement));if(fs.readFileSync(file,'utf8')!==before)throw Error('shared macro changed');fs.renameSync(temp,file);
